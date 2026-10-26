use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};

fn read(path: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(path).unwrap()).unwrap()
}
fn median(mut values: Vec<f64>) -> f64 {
    assert!(!values.is_empty() && values.iter().all(|n| n.is_finite()));
    values.sort_by(f64::total_cmp);
    let mid = values.len() / 2;
    if values.len().is_multiple_of(2) {
        (values[mid - 1] + values[mid]) / 2.
    } else {
        values[mid]
    }
}
fn geomean(values: &[f64]) -> f64 {
    assert!(!values.is_empty() && values.iter().all(|n| n.is_finite() && *n > 0.));
    (values.iter().map(|n| n.ln()).sum::<f64>() / values.len() as f64).exp()
}
fn value(row: &Value, key: &str) -> f64 {
    row[key].as_f64().unwrap()
}
fn main() {
    let args: Vec<_> = std::env::args().skip(1).collect();
    assert_eq!(args.len(), 2);
    let root = Path::new(&args[0]);
    let raw = read(root.join("perf/rows.json"));
    let result = read(root.join("perf/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["distinct_verified_exports"], 93);
    assert_eq!(result["profiles"], 9);
    let rows = raw.as_array().unwrap();
    assert_eq!(rows.len(), 93);
    let sdk: usize = rows
        .iter()
        .map(|r| r["samples_ms"].as_array().unwrap().len() + 3)
        .sum();
    assert_eq!(sdk, 3516);
    let latency: Vec<_> = rows.iter().filter(|r| r["phase"] == "latency").collect();
    let profiles: Vec<_> = rows.iter().filter(|r| r["phase"] == "profile").collect();
    assert_eq!(latency.len(), 84);
    assert_eq!(profiles.len(), 9);
    assert!(profiles.iter().all(|r| r["profile"]["lost_records"] == 0
        && r["profile"]["sampled_pids"].as_array().unwrap().len() == 1));
    let cases = ["1x2-w32", "2x2-w32", "merge-54x1-w32", "1x2-w16"];
    let lookup = |case: &str, variant: &str, block: usize| -> &Value {
        let found: Vec<_> = latency
            .iter()
            .filter(|r| r["case"] == case && r["variant"] == variant && r["block"] == block)
            .collect();
        assert_eq!(found.len(), 1);
        found[0]
    };
    let mut failed = Vec::new();
    let mut by_case = BTreeMap::new();
    for case in cases {
        let mut metrics = BTreeMap::new();
        for variant in ["accepted", "off", "on"] {
            let picked: Vec<_> = (0..7).map(|b| lookup(case, variant, b)).collect();
            for row in &picked {
                assert_eq!(row["samples_ms"].as_array().unwrap().len(), 31);
                assert_eq!(
                    row["summary"]["workers"],
                    if case.ends_with("w16") { 16 } else { 32 }
                );
            }
            let summaries: Vec<_> = picked.iter().map(|r| &r["summary"]).collect();
            metrics.insert(variant.to_owned(), json!({
                "median_ms": median(picked.iter().map(|r| value(r,"median_ms")).collect()),
                "cpu_ms_per_proof": median(summaries.iter().map(|s| (value(s,"user_cpu_ms")+value(s,"system_cpu_ms"))/31.).collect()),
                "voluntary_switches_per_proof": median(summaries.iter().map(|s| value(s,"voluntary_switches")/31.).collect()),
                "maxrss_mib": summaries.iter().map(|s| value(s,"whole_process_maxrss_kib")/1024.).max_by(f64::total_cmp).unwrap()
            }));
        }
        let ratios = |variant: &str, control: &str| -> Vec<f64> {
            (0..7)
                .map(|b| {
                    value(lookup(case, variant, b), "median_ms")
                        / value(lookup(case, control, b), "median_ms")
                })
                .collect()
        };
        let calibration = median(ratios("off", "accepted"));
        let factor = calibration.max(1. / calibration);
        if factor > 1.03 {
            failed.push(format!("{case}: OFF calibration factor {factor}"));
        }
        let mut comparisons = BTreeMap::new();
        for control in ["accepted", "off"] {
            let paired = ratios("on", control);
            let ratio = median(paired.clone());
            let wins = paired.iter().filter(|r| **r < 1.).count();
            if ratio > 1.02 {
                failed.push(format!("{case}/{control}: nonregression ratio {ratio}"));
            }
            for (b, r) in paired.iter().enumerate() {
                if *r > 1.05 {
                    failed.push(format!("{case}/{control}/block{b}: ratio {r}"));
                }
            }
            if case == "1x2-w32" && (ratio >= 1. || wins < 4) {
                failed.push(format!(
                    "{case}/{control}: primary ratio {ratio}, wins {wins}/7"
                ));
            }
            comparisons.insert(
                control.to_owned(),
                json!({"paired_median":ratio,"wins":wins,"paired_ratios":paired}),
            );
        }
        by_case.insert(case,json!({"metrics":metrics,"on_over_control":comparisons,"off_calibration_factor":factor}));
    }
    let mut aggregate = BTreeMap::new();
    for control in ["accepted", "off"] {
        let ratios: Vec<_> = (0..7)
            .map(|b| {
                geomean(
                    &cases
                        .iter()
                        .map(|c| {
                            value(lookup(c, "on", b), "median_ms")
                                / value(lookup(c, control, b), "median_ms")
                        })
                        .collect::<Vec<_>>(),
                )
            })
            .collect();
        let ratio = median(ratios.clone());
        if ratio >= 1. {
            failed.push(format!(
                "four-case/{control}: geometric paired ratio {ratio}"
            ));
        }
        aggregate.insert(
            control,
            json!({"paired_median":ratio,"block_geomeans":ratios}),
        );
    }
    let profile_samples: u64 = profiles
        .iter()
        .map(|r| r["profile"]["samples"].as_u64().unwrap())
        .sum();
    let output = json!({"status":if failed.is_empty(){"pilot passed, further qualification required"}else{"pilot rejected"},
        "adopted":false,"goal_achieved":false,"performance_gate_passed":failed.is_empty(),"failed_checks":failed,
        "cases":by_case,"four_case_aggregate":aggregate,"sdk_perf":sdk,"profiles":9,"profile_samples":profile_samples,
        "lost_records":0,"distinct_perf_exports":93,
        "limitations":"Local SMT CPU only. Profiles are separate and sampled CPU shares are not removable wall time. RSS includes loading/priming and does not establish minimum RAM. No pressure, portable or large qualification. C7a target stage is declared separately and remains pending."});
    fs::write(
        &args[1],
        format!("{}\n", serde_json::to_string_pretty(&output).unwrap()),
    )
    .unwrap();
    println!("{}", output["status"]);
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn medians_and_positive_geometric_aggregation_use_every_block() {
        assert_eq!(median(vec![9., 1., 2., 7., 5., 6., 3.]), 5.);
        assert_eq!(median(vec![2., 4.]), 3.);
        assert!((geomean(&[0.5, 2., 1., 1.]) - 1.).abs() < 1e-14);
        assert!(std::panic::catch_unwind(|| median(vec![f64::NAN])).is_err());
        assert!(std::panic::catch_unwind(|| geomean(&[0.])).is_err());
    }
}
