use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn median(x: &[f64]) -> f64 {
    assert!(!x.is_empty() && x.iter().all(|v| v.is_finite() && *v > 0.));
    let mut x = x.to_vec();
    x.sort_by(f64::total_cmp);
    if x.len() % 2 == 1 {
        x[x.len() / 2]
    } else {
        (x[x.len() / 2 - 1] + x[x.len() / 2]) / 2.
    }
}
fn geo(x: &[f64]) -> f64 {
    assert!(!x.is_empty() && x.iter().all(|v| v.is_finite() && *v > 0.));
    (x.iter().map(|v| v.ln()).sum::<f64>() / x.len() as f64).exp()
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32");
    assert_eq!(read(root.join("timing/result.json"))["status"], "complete");
    let order = read(root.join("timing-order.json"));
    let order = order.as_array().unwrap();
    assert_eq!(order.len(), 279);
    let mut values = BTreeMap::new();
    for row in order {
        let got = read(
            root.join("timing")
                .join(row["name"].as_str().unwrap())
                .join("stdout.log"),
        );
        assert_eq!(got["q"], row["q"]);
        assert_eq!(got["column"], row["column"]);
        assert_eq!(got["mode"], row["mode"]);
        assert_eq!(got["iterations"], 100000);
        assert_eq!(got["elements_per_tile"], 256);
        let cost = got["ns_per_element"].as_f64().unwrap();
        assert!(cost > 0. && cost.is_finite());
        assert!((cost - got["elapsed_ns"].as_f64().unwrap() / 25600000.).abs() < 1e-10);
        let output = got["canonical_words"].as_array().unwrap();
        assert_eq!(output.len(), 256);
        assert!(output.iter().all(|v| v
            .as_array()
            .is_some_and(|w| w.len() == 4 && w.iter().all(Value::is_u64))));
        assert!(values
            .insert(
                (
                    row["q"].as_u64().unwrap(),
                    row["block"].as_u64().unwrap(),
                    row["variant"].as_str().unwrap().to_owned()
                ),
                got
            )
            .is_none());
    }
    let mut all = BTreeMap::new();
    let mut failures = Vec::new();
    let mut block_ratios = BTreeMap::from([
        ("accepted", vec![Vec::new(); 31]),
        ("fresh", vec![Vec::new(); 31]),
    ]);
    let mut comparisons = 0;
    for q in [1, 256, 16384] {
        let mut variants = BTreeMap::new();
        for variant in ["accepted", "fresh", "deferred"] {
            let times = (0..31)
                .map(|b| {
                    values[&(q, b, variant.into())]["ns_per_element"]
                        .as_f64()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            variants.insert(
                variant,
                json!({"median_ns_per_element":median(&times),"samples":times}),
            );
        }
        let mut checks = Vec::new();
        for control in ["accepted", "fresh"] {
            let mut ratios = Vec::new();
            for b in 0..31 {
                let a = &values[&(q, b, control.into())];
                let c = &values[&(q, b, "deferred".into())];
                assert_eq!(
                    a["canonical_words"], c["canonical_words"],
                    "q{q} b{b} control{control}"
                );
                comparisons += 1024;
                let r =
                    c["ns_per_element"].as_f64().unwrap() / a["ns_per_element"].as_f64().unwrap();
                ratios.push(r);
                block_ratios.get_mut(control).unwrap()[b as usize].push(r);
            }
            let ratio = median(&ratios);
            let wins = ratios.iter().filter(|v| **v < 1.).count();
            let nonregression = ratio <= 1.03;
            let primary = q != 256 || (ratio < 1. && wins >= 21);
            if !nonregression {
                failures.push(format!("q{q} nonregression vs{control}"));
            }
            if !primary {
                failures.push(format!("primary gain/21wins vs{control}"));
            }
            checks.push(json!({"control":control,"paired_median_ratio":ratio,"wins":wins,"ratios":ratios,"nonregression_pass":nonregression,"primary_pass":primary}));
        }
        let cal = (0..31)
            .map(|b| {
                let a = values[&(q, b, "accepted".into())]["ns_per_element"]
                    .as_f64()
                    .unwrap();
                let f = values[&(q, b, "fresh".into())]["ns_per_element"]
                    .as_f64()
                    .unwrap();
                f / a
            })
            .collect::<Vec<_>>();
        let cal = median(&cal);
        let cal_factor = cal.max(1. / cal);
        if cal_factor > 1.03 {
            failures.push(format!("q{q} fresh calibration"));
        }
        all.insert(q.to_string(),json!({"variants":variants,"checks":checks,"calibration_ratio":cal,"calibration_factor":cal_factor,"calibration_pass":cal_factor<=1.03}));
    }
    let mut aggregate = BTreeMap::new();
    for control in ["accepted", "fresh"] {
        let case_ratios = [1, 256, 16384].map(|q| {
            all[&q.to_string()]["checks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["control"] == control)
                .unwrap()["paired_median_ratio"]
                .as_f64()
                .unwrap()
        });
        let ratio = geo(&case_ratios);
        let blocks = block_ratios[control]
            .iter()
            .map(|v| geo(v))
            .collect::<Vec<_>>();
        let wins = blocks.iter().filter(|v| **v < 1.).count();
        let pass = ratio < 1. && wins >= 21;
        if !pass {
            failures.push(format!("aggregate gain/21wins vs{control}"));
        }
        aggregate.insert(
            control,
            json!({"ratio":ratio,"wins":wins,"block_ratios":blocks,"pass":pass}),
        );
    }
    assert_eq!(comparisons, 190464);
    let result = json!({"status":"complete","passed":failures.is_empty(),"adopted":false,"cases":all,"aggregate":aggregate,"failures":failures,"measured_processes":279,"measured_tile_iterations":27900000u64,"prime_tile_iterations":1674000,"raw_word_comparisons_vs_two_controls":comparisons,"all_output_words_equal":true,"new_sdk_proofs":0,"scope":"Full five-stage DIF arithmetic tile only; initial packing/roots/host conversion excluded. A component pass does not establish complete FFT or SDK improvement."});
    fs::write(
        root.join("component-result.json"),
        format!("{}\n", serde_json::to_string_pretty(&result).unwrap()),
    )
    .unwrap();
    println!(
        "Component gate{} accepted{:.9} fresh{:.9}; all{comparisons} outputwords agree",
        result["passed"],
        result["aggregate"]["accepted"]["ratio"].as_f64().unwrap(),
        result["aggregate"]["fresh"]["ratio"].as_f64().unwrap()
    );
}
