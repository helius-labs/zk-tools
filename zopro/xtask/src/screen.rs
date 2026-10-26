//! Short, verified SDK comparisons; these never replace final qualification.
use crate::{config::Config, Result};
use serde_json::{json, Value};
use std::collections::BTreeSet;

pub fn prepare(mut config: Config, names: &[String]) -> Result<Config> {
    config.validate()?;
    let requested: BTreeSet<_> = names.iter().collect();
    if names.is_empty()
        || requested.len() != names.len()
        || names
            .iter()
            .any(|name| !config.cases.iter().any(|c| &c.name == name))
    {
        return Err(
            "screen cases must be an explicit, nonempty, unique subset of the full plan".into(),
        );
    }
    // Preserve every variant, pin, check, verifier and prior proof identity.
    config.cases.retain(|case| requested.contains(&case.name));
    config.blocks = 3;
    for case in &mut config.cases {
        case.rounds = 17;
    }
    // Kept valid for the ordinary configuration validator; screen execution
    // has a separate receipt and executes only the latency phase.
    config.profile_cases = Some(config.cases.iter().map(|c| c.name.clone()).collect());
    config.validate()?;
    Ok(config)
}

pub fn impact(
    proof_ms: f64,
    before_us: f64,
    after_us: f64,
    target_ms: Option<f64>,
) -> Result<Value> {
    if !proof_ms.is_finite()
        || proof_ms <= 0.
        || [before_us, after_us]
            .iter()
            .any(|v| !v.is_finite() || *v < 0.)
        || before_us / 1000. > proof_ms
        || target_ms.is_some_and(|v| !v.is_finite() || v <= 0.)
    {
        return Err("use positive finite proof/target milliseconds and nonnegative component microseconds within the original proof duration".into());
    }
    let saved_ms = (before_us - after_us) / 1000.;
    let estimated_ms = proof_ms - saved_ms;
    Ok(json!({
        "proof_ms":proof_ms,"component_before_us":before_us,"component_after_us":after_us,
        "estimated_saving_us":before_us-after_us,"estimated_saving_ms":saved_ms,
        "estimated_saving_percent":100.*saved_ms/proof_ms,"estimated_proof_ms":estimated_ms,
        "target_ms":target_ms,"estimated_remaining_ms":target_ms.map(|t|(estimated_ms-t).max(0.)),
        "measurement":false,"qualification":false,"minimum_gain_cutoff":null,
        "assumptions":"Same host, build, inputs and timing boundary; the component is wholly on the SDK critical path and other work stays unchanged. This is a projection, not proof of an SDK gain. Positive small gains remain eligible for measurement."
    }))
}

pub fn markdown(result: &Value) -> String {
    let mut out=String::from("# Verified SDK screen\n\nThree rotated/reversed fresh-process blocks, 17 timed complete SDK requests after three primes. Every final export passes the configured independent verifier, public-input binding and freshness checks. No cycle profiles are collected in this short screen. It does not qualify adoption or replace the full circuit matrix, resource checks and profiles. There is no minimum gain cutoff.\n\n|Case|Variant|Control|SDK median ms|Paired geometric ratio|Paired median saving µs|Winning blocks|\n|---|---|---|---:|---:|---:|---:|\n");
    if let Some(controls) = result["latency_by_control"].as_object() {
        for (control, cases) in controls {
            for (case, variants) in cases.as_object().unwrap() {
                for (variant, cell) in variants.as_object().unwrap() {
                    if variant == control {
                        continue;
                    }
                    out.push_str(&format!(
                        "|{case}|{variant}|{control}|{:.6}|{:.9}|{:.3}|{}|\n",
                        cell["median_ms"].as_f64().unwrap(),
                        cell["geometric_ratio"].as_f64().unwrap(),
                        cell["paired_median_saving_us"].as_f64().unwrap(),
                        cell["winning_blocks"]
                    ));
                }
            }
        }
    }
    out.push_str("\nSavings use matched blocks; positive means faster. Independent medians need not reproduce paired ratios. Raw samples, resource summaries, proof exports and command/hash receipts are retained. Investigate unstable controls before interpreting a small difference; this report makes no automatic adoption decision.\n");
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    fn full() -> Config {
        let h = "0".repeat(64);
        serde_json::from_value(json!({"version":1,"blocks":7,"profile_blocks":1,"profile_cases":["one"],
            "command":["{binary}","{image}","{request}","{rounds}","{proof_dir}"],
            "variants":[{"name":"accepted","binary":"/tmp/accepted"},{"name":"off","binary":"/tmp/fresh","environment":{"ZOPRO_OPTION":"0"}},{"name":"on","binary":"/tmp/fresh","environment":{"ZOPRO_OPTION":"1"}}],
            "cases":[{"name":"one","shape":"1x2","workers":32,"cpu_affinity":"0-31","rounds":31,"profile_rounds":251,"public_sha256":h,"image":"/tmp/image","request":"/tmp/request","reference":{"cpu_ms":37.,"metal_ms":24.}},
                {"name":"two","shape":"2x2","workers":16,"rounds":151,"profile_rounds":251,"public_sha256":h}],
            "pins":{"/tmp/accepted":h,"/tmp/fresh":h,"/tmp/verifier":h,"/tmp/image":h,"/tmp/request":h},
            "prior_proof_sha256":[h],"checks":[["/tmp/check"]],"verifier":["/tmp/verifier","{proof_dir}"],
            "environment":{"MALLOC_ARENA_MAX":"32"},"profile_environment":{"ZOPRO_BENCH_PERF_CTL":"{control}"}})).unwrap()
    }
    #[test]
    fn selection_preserves_verification_inputs_and_every_control() {
        let before = serde_json::to_value(full()).unwrap();
        let screen = prepare(full(), &["two".into()]).unwrap();
        let after = serde_json::to_value(&screen).unwrap();
        for key in [
            "pins",
            "variants",
            "verifier",
            "checks",
            "prior_proof_sha256",
            "environment",
            "command",
        ] {
            assert_eq!(before[key], after[key], "{key}");
        }
        assert_eq!(screen.blocks, 3);
        assert_eq!(screen.cases.len(), 1);
        assert_eq!(screen.cases[0].rounds, 17);
        assert_eq!(screen.cases[0].workers, 16);
        assert_eq!(screen.cases[0].public_sha256, "0".repeat(64));
        for names in [
            vec![],
            vec!["missing".into()],
            vec!["one".into(), "one".into()],
        ] {
            assert!(prepare(full(), &names).is_err());
        }
        let mut invalid = full();
        invalid.pins.remove("/tmp/verifier");
        assert!(prepare(invalid, &["one".into()]).is_err());
    }
    #[test]
    fn projection_keeps_microseconds_distinct_from_milliseconds_and_small_gains() {
        let v = impact(8.56, 164.591, 132.079, Some(3.7)).unwrap();
        assert!((v["estimated_saving_ms"].as_f64().unwrap() - 0.032512).abs() < 1e-12);
        assert!((v["estimated_proof_ms"].as_f64().unwrap() - 8.527488).abs() < 1e-12);
        assert!(
            (v["estimated_saving_percent"].as_f64().unwrap() - 0.3798130841121495).abs() < 1e-12
        );
        assert_eq!(v["measurement"], false);
        assert!(v["minimum_gain_cutoff"].is_null());
        assert!(
            impact(8.56, 132., 164., None).unwrap()["estimated_saving_ms"]
                .as_f64()
                .unwrap()
                < 0.
        );
        for (p, b, a, t) in [
            (0., 1., 0., None),
            (8., 9000., 1., None),
            (8., -1., 0., None),
            (8., 1., f64::NAN, None),
            (8., 1., 0., Some(0.)),
        ] {
            assert!(impact(p, b, a, t).is_err());
        }
    }
}
