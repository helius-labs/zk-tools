use crate::{config::Category, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

pub fn median(values: &[f64]) -> f64 {
    let mut values = values.to_vec();
    values.sort_by(f64::total_cmp);
    let n = values.len();
    if n.is_multiple_of(2) {
        (values[n / 2 - 1] + values[n / 2]) / 2.
    } else {
        values[n / 2]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Sdk {
    pub samples_ms: Vec<f64>,
    pub median_ms: f64,
    pub summary: Value,
}

pub fn sdk_output(text: &str, rounds: usize) -> Result<Sdk> {
    let mut samples = Vec::new();
    let mut summaries = Vec::new();
    for line in text.lines() {
        if let Some(sample) = line.strip_prefix("SAMPLE ") {
            samples.push(serde_json::from_str::<Value>(sample)?);
        }
        if let Some(summary) = line.strip_prefix("SUMMARY ") {
            summaries.push(serde_json::from_str::<Value>(summary)?);
        }
    }
    if rounds == 0 || samples.len() != rounds || summaries.len() != 1 {
        return Err("incomplete SDK output".into());
    }
    let mut times = Vec::with_capacity(rounds);
    for (round, sample) in samples.iter().enumerate() {
        let time = sample
            .get("verified_ms")
            .or_else(|| sample.get("ms"))
            .and_then(Value::as_f64)
            .ok_or("missing SDK time")?;
        if !time.is_finite() || time <= 0. || sample["round"].as_u64() != Some(round as u64) {
            return Err("invalid or repeated SDK round".into());
        }
        times.push(time);
    }
    let summary = summaries.pop().unwrap();
    if summary["rounds"].as_u64() != Some(rounds as u64) || summary["priming"].as_u64() != Some(3) {
        return Err("unexpected SDK measurement scope".into());
    }
    Ok(Sdk {
        median_ms: median(&times),
        samples_ms: times,
        summary,
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Profile {
    pub samples: usize,
    pub period: u64,
    pub lost_records: usize,
    pub category_periods: BTreeMap<String, u64>,
    pub category_percent: BTreeMap<String, f64>,
    pub symbol_periods: BTreeMap<String, u64>,
    pub sampled_pids: BTreeSet<u32>,
    pub sampled_tids: usize,
}

pub fn perf_samples(text: &str, categories: &[Category]) -> Result<Profile> {
    let mut symbols = BTreeMap::<String, u64>::new();
    let mut counts = BTreeMap::<String, u64>::new();
    let mut samples = 0;
    let mut pids = BTreeSet::new();
    let mut tids = BTreeSet::new();
    for line in text
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
    {
        if line.contains("PERF_RECORD_LOST") {
            return Err(format!("lost perf samples: {line}").into());
        }
        let mut fields = line.split_whitespace();
        let (pid, tid) = fields
            .next()
            .and_then(|s| s.split_once('/'))
            .ok_or("missing perf process/thread IDs")?;
        pids.insert(pid.parse::<u32>()?);
        tids.insert(tid.parse::<u32>()?);
        let period: u64 = fields.next().ok_or("missing perf period")?.parse()?;
        let ip = fields.next().ok_or("missing perf IP")?;
        if !ip.bytes().all(|b| b.is_ascii_hexdigit()) {
            return Err("invalid perf IP".into());
        }
        let raw = fields.collect::<Vec<_>>().join(" ");
        let symbol = rustc_demangle::try_demangle(&raw)
            .map(|s| format!("{s:#}"))
            .unwrap_or(raw);
        if period == 0 || symbol.is_empty() {
            return Err("empty perf sample".into());
        }
        let weight = symbols.entry(symbol.clone()).or_default();
        *weight = weight.checked_add(period).ok_or("sample period overflow")?;
        let category = categories
            .iter()
            .find(|category| {
                category
                    .contains
                    .iter()
                    .any(|pattern| symbol.contains(pattern))
            })
            .map_or("other", |c| &c.name);
        let weight = counts.entry(category.into()).or_default();
        *weight = weight
            .checked_add(period)
            .ok_or("category period overflow")?;
        samples += 1;
    }
    let period = symbols
        .values()
        .try_fold(0u64, |sum, value| sum.checked_add(*value))
        .ok_or("total sample period overflow")?;
    if samples == 0 {
        return Err("empty profile".into());
    }
    if pids.len() != 1 {
        return Err("profile includes more than one sampled process".into());
    }
    assert_eq!(counts.values().sum::<u64>(), period);
    let category_percent = counts
        .iter()
        .map(|(k, v)| (k.clone(), 100. * (*v as f64) / (period as f64)))
        .collect();
    Ok(Profile {
        samples,
        period,
        lost_records: 0,
        category_periods: counts,
        category_percent,
        symbol_periods: symbols,
        sampled_pids: pids,
        sampled_tids: tids.len(),
    })
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Row {
    pub case: String,
    pub variant: String,
    pub phase: String,
    pub block: usize,
    #[serde(flatten)]
    pub sdk: Sdk,
    pub profile: Option<Profile>,
}

pub fn paired_summary(rows: &[Row], baseline: &str) -> Result<Value> {
    let mut cells = BTreeMap::<String, BTreeMap<String, Vec<&Row>>>::new();
    for row in rows.iter().filter(|row| row.phase == "latency") {
        cells
            .entry(row.case.clone())
            .or_default()
            .entry(row.variant.clone())
            .or_default()
            .push(row);
    }
    let mut result = serde_json::Map::new();
    for (case, variants) in cells {
        let mut row = serde_json::Map::new();
        for (variant, group) in &variants {
            let mut ratios = Vec::new();
            let mut savings_us = Vec::new();
            for sample in group {
                let control = variants
                    .get(baseline)
                    .and_then(|v| v.iter().find(|r| r.block == sample.block))
                    .ok_or("missing paired control")?;
                ratios.push(sample.sdk.median_ms / control.sdk.median_ms);
                savings_us.push((control.sdk.median_ms - sample.sdk.median_ms) * 1000.);
            }
            row.insert(variant.clone(),serde_json::json!({"median_ms":median(&group.iter().map(|r|r.sdk.median_ms).collect::<Vec<_>>()),
                "median_ratio":median(&ratios),"geometric_ratio":(ratios.iter().map(|v|v.ln()).sum::<f64>()/ratios.len() as f64).exp(),
                "paired_median_saving_us":median(&savings_us),"winning_blocks":ratios.iter().filter(|&&r|r<1.).count(),"paired_ratios":ratios}));
        }
        result.insert(case, Value::Object(row));
    }
    Ok(Value::Object(result))
}

pub fn profile_summary(rows: &[Row], baseline: &str) -> Result<Value> {
    let mut cells = BTreeMap::<String, BTreeMap<String, Vec<&Row>>>::new();
    for row in rows.iter().filter(|row| row.phase == "profile") {
        cells
            .entry(row.case.clone())
            .or_default()
            .entry(row.variant.clone())
            .or_default()
            .push(row);
    }
    let mut result = serde_json::Map::new();
    for (case, variants) in cells {
        let mut values = BTreeMap::<String, Value>::new();
        for (variant, group) in &variants {
            let mut total_cpu = Vec::new();
            let mut system_cpu = Vec::new();
            let mut wall = Vec::new();
            let mut occupied = Vec::new();
            let mut rss = 0.;
            let mut categories = BTreeMap::<String, u64>::new();
            let mut total_period = 0u64;
            let mut samples = 0usize;
            for row in group {
                let number = |key: &str| -> Result<f64> {
                    row.sdk.summary[key]
                        .as_f64()
                        .filter(|v| v.is_finite() && *v >= 0.)
                        .ok_or_else(|| {
                            format!("missing/nonfinite SDK accounting field: {key}").into()
                        })
                };
                let rounds = row.sdk.samples_ms.len() as f64;
                let cpu = number("user_cpu_ms")? + number("system_cpu_ms")?;
                if cpu == 0. {
                    return Err("empty SDK CPU accounting interval; increase profile rounds".into());
                }
                let elapsed = number("loop_wall_ms")?;
                if elapsed == 0. {
                    return Err("empty SDK accounting interval".into());
                }
                total_cpu.push(cpu / rounds);
                system_cpu.push(number("system_cpu_ms")? / rounds);
                wall.push(elapsed / rounds);
                occupied.push(cpu / elapsed);
                rss = f64::max(rss, number("whole_process_maxrss_kib")? / 1024.);
                let profile = row.profile.as_ref().ok_or("missing parsed profile")?;
                total_period = total_period
                    .checked_add(profile.period)
                    .ok_or("aggregate period overflow")?;
                samples += profile.samples;
                for (name, period) in &profile.category_periods {
                    let value = categories.entry(name.clone()).or_default();
                    *value = value
                        .checked_add(*period)
                        .ok_or("aggregate category overflow")?;
                }
            }
            let shares: BTreeMap<_, _> = categories
                .iter()
                .map(|(k, v)| (k.clone(), 100. * (*v as f64) / (total_period as f64)))
                .collect();
            values.insert(variant.clone(),serde_json::json!({"cpu_ms_per_request":median(&total_cpu),"system_cpu_ms_per_request":median(&system_cpu),
                "wall_ms_per_request":median(&wall),"mean_occupied_logical_cpus":median(&occupied),"max_process_rss_mib":rss,
                "samples":samples,"period":total_period,"category_percent":shares,"captures":group.len()}));
        }
        let control = values
            .get(baseline)
            .ok_or("missing profile control")?
            .clone();
        for value in values.values_mut() {
            let mut differences = serde_json::Map::new();
            let names: std::collections::BTreeSet<_> = value["category_percent"]
                .as_object()
                .unwrap()
                .keys()
                .chain(control["category_percent"].as_object().unwrap().keys())
                .cloned()
                .collect();
            for name in names {
                differences.insert(
                    name.clone(),
                    serde_json::json!(
                        value["category_percent"][&name].as_f64().unwrap_or(0.)
                            - control["category_percent"][&name].as_f64().unwrap_or(0.)
                    ),
                );
            }
            value["cpu_ratio_to_baseline"] = serde_json::json!(
                value["cpu_ms_per_request"].as_f64().unwrap()
                    / control["cpu_ms_per_request"].as_f64().unwrap()
            );
            value["category_delta_percentage_points"] = Value::Object(differences);
        }
        result.insert(case, serde_json::to_value(values)?);
    }
    Ok(Value::Object(result))
}

pub fn markdown(result: &Value) -> String {
    let mut out=String::from("# CPU experiment results\n\nLatency runs and warm cycle captures run in separate processes. Profiles exclude key loading and three priming proofs. These diagnostics do not authorize adoption or establish minimum RAM.\n\n| Case | Variant | SDK median ms | Paired latency ratio | Profile CPU ms/request | Profile wall ms/request | Peak RSS MiB |\n| --- | --- | ---: | ---: | ---: | ---: | ---: |\n");
    if let Some(cases) = result["latency"].as_object() {
        for (case, variants) in cases {
            for (variant, cell) in variants.as_object().unwrap() {
                let profile = &result["profile_comparison"][case][variant];
                out.push_str(&format!(
                    "| {case} | {variant} | {:.6} | {:.6} | {} | {} | {} |\n",
                    cell["median_ms"].as_f64().unwrap(),
                    cell["median_ratio"].as_f64().unwrap(),
                    profile["cpu_ms_per_request"]
                        .as_f64()
                        .map(|n| format!("{n:.3}"))
                        .unwrap_or_else(|| "—".into()),
                    profile["wall_ms_per_request"]
                        .as_f64()
                        .map(|n| format!("{n:.6}"))
                        .unwrap_or_else(|| "—".into()),
                    profile["max_process_rss_mib"]
                        .as_f64()
                        .map(|n| format!("{n:.2}"))
                        .unwrap_or_else(|| "—".into())
                ));
            }
        }
    }
    out.push_str("\nCategory shares use sample-period weights and explicit ordered substring rules. Inlined work can appear under enclosing symbols. Shares are not wall-time fractions or removable latency. Unknown and unclassified symbols remain in the total.\n\n| Case | Variant | Category | Cycle share % | Change vs baseline, percentage points |\n| --- | --- | --- | ---: | ---: |\n");
    if let Some(cases) = result["profile_comparison"].as_object() {
        for (case, variants) in cases {
            for (variant, cell) in variants.as_object().unwrap() {
                for (category, share) in cell["category_percent"].as_object().unwrap() {
                    out.push_str(&format!(
                        "| {case} | {variant} | {category} | {:.3} | {:.3} |\n",
                        share.as_f64().unwrap(),
                        cell["category_delta_percentage_points"][category]
                            .as_f64()
                            .unwrap()
                    ));
                }
            }
        }
    }
    out.push_str("\nAll final exports passed the configured independent verifier, public-input binding and proof-freshness checks. Raw commands, samples, profiles, checkpoints and artifact pins are retained beside this report.\n");
    if let Some(references) = result["references"]
        .as_array()
        .filter(|rows| !rows.is_empty())
    {
        out.push_str("\n## Supplied reference comparison\n\nCPU and Metal times below are supplied historical references, not new Apple measurements.\n\n| Case | Variant | SDK ms | Reference CPU ms | CPU speedup | Reference Metal ms | Metal speedup | CPU reaches 10× |\n| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |\n");
        for row in references {
            out.push_str(&format!(
                "| {} | {} | {:.6} | {:.3} | {:.3}× | {:.3} | {:.3}× | {} |\n",
                row["case"].as_str().unwrap(),
                row["variant"].as_str().unwrap(),
                row["ms"].as_f64().unwrap(),
                row["cpu_ms"].as_f64().unwrap(),
                row["cpu_speedup"].as_f64().unwrap(),
                row["metal_ms"].as_f64().unwrap(),
                row["metal_speedup"].as_f64().unwrap(),
                row["cpu_meets_10x"].as_bool().unwrap()
            ));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn all_rounds_and_scope_are_required() {
        let text="SAMPLE {\"round\":0,\"verified_ms\":2}\nSAMPLE {\"round\":1,\"verified_ms\":4}\nSUMMARY {\"rounds\":2,\"priming\":3}\n";
        assert_eq!(sdk_output(text, 2).unwrap().median_ms, 3.);
        assert!(sdk_output(text, 3).is_err());
        assert!(sdk_output(&text.replace("\"round\":1", "\"round\":0"), 2).is_err());
        assert!(sdk_output(&text.replace("\"priming\":3", "\"priming\":0"), 2).is_err());
        assert!(sdk_output(&text.replace("\"verified_ms\":2", "\"verified_ms\":0"), 2).is_err());
    }
    #[test]
    fn weighted_profiles_preserve_unknown_symbols_and_reject_loss() {
        let categories = [Category {
            name: "curve".into(),
            contains: vec!["msm".into()],
        }];
        let profile = perf_samples(
            "12/12 10 00a1 msm::fill\n12/13 30 00a2 [unknown]\n12/12 10 00a1 msm::fill\n",
            &categories,
        )
        .unwrap();
        assert_eq!(profile.samples, 3);
        assert_eq!(profile.period, 50);
        assert_eq!(profile.category_percent["curve"], 40.);
        assert_eq!(profile.category_percent["other"], 60.);
        assert_eq!(profile.sampled_tids, 2);
        assert!(perf_samples("12/12 10 00a1 symbol\nPERF_RECORD_LOST 4\n", &categories).is_err());
        assert!(perf_samples("", &categories).is_err());
        assert!(perf_samples("not a sample", &categories).is_err());
    }
    #[test]
    fn mixed_processes_and_period_overflow_fail_and_rust_symbols_demangle() {
        assert!(perf_samples("12/12 1 0f symbol\n13/13 1 0f symbol\n", &[]).is_err());
        assert!(perf_samples(
            "12/12 18446744073709551615 0f symbol\n12/12 1 0f symbol\n",
            &[]
        )
        .is_err());
        let profile = perf_samples("12/12 10 0f _ZN3foo3barE\n", &[]).unwrap();
        assert_eq!(profile.symbol_periods["foo::bar"], 10);
    }
    #[test]
    fn profile_totals_use_period_weights_and_per_request_cpu() {
        let mut rows = Vec::new();
        for (name, cpu, period) in [("base", 40., 10u64), ("candidate", 20., 30u64)] {
            rows.push(Row{case:"one".into(),variant:name.into(),phase:"profile".into(),block:0,
                sdk:Sdk{samples_ms:vec![1.,1.],median_ms:1.,summary:serde_json::json!({"user_cpu_ms":cpu,"system_cpu_ms":2.,"loop_wall_ms":4.,"whole_process_maxrss_kib":1024})},
                profile:Some(perf_samples(&format!("12/12 {period} 0f symbol\n"),&[]).unwrap())});
        }
        let result = profile_summary(&rows, "base").unwrap();
        assert_eq!(result["one"]["base"]["cpu_ms_per_request"], 21.);
        assert_eq!(
            result["one"]["candidate"]["cpu_ratio_to_baseline"],
            22. / 42.
        );
        assert_eq!(
            result["one"]["candidate"]["category_percent"]["other"],
            100.
        );
        assert_eq!(
            result["one"]["candidate"]["category_delta_percentage_points"]["other"],
            0.
        );
    }
    #[test]
    fn paired_ratios_use_matching_blocks_instead_of_pooled_times() {
        let mut rows = Vec::new();
        for (block, base, value) in [(0, 10., 9.), (1, 100., 110.), (2, 10., 9.)] {
            for (name, time) in [("base", base), ("candidate", value)] {
                rows.push(Row {
                    case: "one".into(),
                    variant: name.into(),
                    phase: "latency".into(),
                    block,
                    sdk: Sdk {
                        samples_ms: vec![time],
                        median_ms: time,
                        summary: Value::Null,
                    },
                    profile: None,
                });
            }
        }
        let result = paired_summary(&rows, "base").unwrap();
        assert_eq!(result["one"]["candidate"]["median_ratio"], 0.9);
        assert_eq!(result["one"]["candidate"]["winning_blocks"], 2);
        assert!(
            (result["one"]["candidate"]["geometric_ratio"]
                .as_f64()
                .unwrap()
                - 0.891f64.cbrt())
            .abs()
                < 1e-12
        );
        assert_eq!(result["one"]["candidate"]["paired_median_saving_us"], 1000.);
    }
}
