use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn median(v: &[f64]) -> f64 {
    assert!(!v.is_empty() && v.iter().all(|x| x.is_finite() && *x > 0.));
    let mut a = v.to_vec();
    a.sort_by(f64::total_cmp);
    a[a.len() / 2]
}
fn geo(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn main() {
    let r = Path::new("/tmp/zopro-streamed-complex-b");
    assert_eq!(read(r.join("timing/result.json"))["status"], "complete");
    let order = read(r.join("timing-order.json"));
    assert_eq!(order.as_array().unwrap().len(), 496);
    let mut values = BTreeMap::new();
    for row in order.as_array().unwrap() {
        let name = row["name"].as_str().unwrap();
        let s = fs::read_to_string(r.join("timing").join(name).join("stdout.log")).unwrap();
        let lines = s
            .lines()
            .filter_map(|l| l.strip_prefix("ROW "))
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        let got: Value = serde_json::from_str(lines[0]).unwrap();
        assert_eq!(got["pattern"], row["pattern"]);
        assert_eq!(got["strategy"], row["strategy"]);
        assert_eq!(got["iterations"], 100000);
        assert_eq!(got["primes"], 3);
        let streams = if got["pattern"] == "streams4" { 4. } else { 1. };
        let cost = got["ns_per_lane"].as_f64().unwrap();
        assert!(cost.is_finite() && cost > 0.);
        assert!(
            (cost - got["elapsed_ns"].as_f64().unwrap() / (100000. * 8. * streams)).abs() < 1e-10
        );
        let words = got["output_words"].as_array().unwrap();
        assert_eq!(words.len(), 64 * streams as usize);
        assert!(words.iter().all(Value::is_u64));
        let key = (
            row["pattern"].as_str().unwrap().to_owned(),
            row["block"].as_u64().unwrap(),
            row["variant"].as_str().unwrap().to_owned(),
        );
        assert!(values.insert(key, got).is_none());
    }
    let patterns = ["chain", "streams4", "scaled", "affine-expression"];
    let controls = ["accepted", "fresh", "shared"];
    let mut all = BTreeMap::new();
    let mut failure = Vec::new();
    let mut block_ratios = BTreeMap::from(controls.map(|c| (c, vec![Vec::<f64>::new(); 31])));
    let mut compared = 0;
    for pattern in patterns {
        let mut variants = BTreeMap::new();
        for v in ["accepted", "fresh", "shared", "streamed"] {
            let times = (0..31)
                .map(|b| {
                    values[&(pattern.into(), b, v.into())]["ns_per_lane"]
                        .as_f64()
                        .unwrap()
                })
                .collect::<Vec<_>>();
            variants.insert(
                v,
                json!({"median_ns_per_lane":median(&times),"samples":times}),
            );
        }
        let mut checks = Vec::new();
        for control in controls {
            let mut ratios = Vec::new();
            for b in 0..31 {
                let a = &values[&(pattern.into(), b, control.into())];
                let c = &values[&(pattern.into(), b, "streamed".into())];
                assert_eq!(
                    a["output_words"], c["output_words"],
                    "{pattern} block{b} control{control}"
                );
                compared += a["output_words"].as_array().unwrap().len();
                let ratio = c["ns_per_lane"].as_f64().unwrap() / a["ns_per_lane"].as_f64().unwrap();
                ratios.push(ratio);
                block_ratios.get_mut(control).unwrap()[b as usize].push(ratio);
            }
            let ratio = median(&ratios);
            let pass = ratio <= 1.03;
            if !pass {
                failure.push(format!(
                    "{pattern} median ratio {ratio} vs {control} exceeds1.03"
                ));
            }
            checks.push(json!({"control":control,"paired_median_ratio":ratio,"wins":ratios.iter().filter(|x|**x<1.).count(),"ratios":ratios,"nonregression_pass":pass}));
        }
        let cal = (0..31)
            .map(|b| {
                values[&(pattern.into(), b, "fresh".into())]["ns_per_lane"]
                    .as_f64()
                    .unwrap()
                    / values[&(pattern.into(), b, "accepted".into())]["ns_per_lane"]
                        .as_f64()
                        .unwrap()
            })
            .collect::<Vec<_>>();
        let cal = median(&cal);
        let factor = cal.max(1. / cal);
        if factor > 1.03 {
            failure.push(format!("{pattern} fresh calibration {factor} exceeds1.03"));
        }
        all.insert(pattern,json!({"variants":variants,"checks":checks,"calibration_ratio":cal,"calibration_factor":factor,"calibration_pass":factor<=1.03}));
    }
    let mut aggregate = BTreeMap::new();
    for control in controls {
        let ratios = patterns.map(|p| {
            all[p]["checks"]
                .as_array()
                .unwrap()
                .iter()
                .find(|v| v["control"] == control)
                .unwrap()["paired_median_ratio"]
                .as_f64()
                .unwrap()
        });
        let ratio = geo(&ratios);
        let blocks = block_ratios[control]
            .iter()
            .map(|v| geo(v))
            .collect::<Vec<_>>();
        let wins = blocks.iter().filter(|x| **x < 1.).count();
        let pass = ratio < 1. && wins >= 21;
        if !pass {
            failure.push(format!(
                "aggregate vs{control}: ratio{ratio},wins{wins}, requires<1and>=21/31"
            ));
        }
        aggregate.insert(
            control,
            json!({"ratio":ratio,"wins":wins,"block_ratios":blocks,"pass":pass}),
        );
    }
    assert_eq!(compared, 41664);
    let result = json!({"status":"complete","passed":failure.is_empty(),"adopted":false,"cases":all,"aggregate":aggregate,"failures":failure,"measured_processes":496,"measured_iterations":49600000u64,"prime_iterations":2976000,"raw_word_comparisons_vs_three_controls":compared,"all_output_words_equal":true,"new_sdk_proofs":0,"scope":"Native field arithmetic only; not complete curve operations or proof latency. No adoption. Actual G2 callers and all-nine verified SDK trial required before promotion."});
    fs::write(
        r.join("component-result.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!(
        "Fixed component gate{}; accepted{:.9},fresh{:.9},shared{:.9}; all{compared}wordsagree",
        result["passed"],
        result["aggregate"]["accepted"]["ratio"].as_f64().unwrap(),
        result["aggregate"]["fresh"]["ratio"].as_f64().unwrap(),
        result["aggregate"]["shared"]["ratio"].as_f64().unwrap()
    );
}
