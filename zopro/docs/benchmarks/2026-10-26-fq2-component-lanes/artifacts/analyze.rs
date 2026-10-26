use serde_json::{json, Value};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn median(v: &[f64]) -> f64 {
    assert!(!v.is_empty() && v.iter().all(|x| x.is_finite() && *x > 0.0));
    let mut v = v.to_vec();
    v.sort_by(f64::total_cmp);
    if v.len() % 2 == 1 {
        v[v.len() / 2]
    } else {
        (v[v.len() / 2 - 1] + v[v.len() / 2]) * 0.5
    }
}
fn geo(v: &[f64]) -> f64 {
    assert!(!v.is_empty() && v.iter().all(|x| x.is_finite() && *x > 0.0));
    (v.iter().map(|v| v.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn main() {
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    assert_eq!(read(root.join("timing/result.json"))["status"], "complete");
    let order = read(root.join("timing-order.json"));
    let order = order.as_array().unwrap();
    assert_eq!(order.len(), 248);
    let mut rows = BTreeMap::new();
    for row in order {
        let name = row["name"].as_str().unwrap();
        let got = read(root.join("timing").join(name).join("stdout.log"));
        let pattern = row["pattern"].as_str().unwrap();
        let mode = row["mode"].as_str().unwrap();
        let block = row["block"].as_u64().unwrap();
        assert_eq!(got["pattern"], pattern);
        assert_eq!(got["mode"], mode);
        assert_eq!(got["iterations"], 1000000);
        let expected = if pattern == "four-streams" { 32 } else { 8 };
        assert_eq!(got["elements_per_iteration"], expected);
        let ns = got["elapsed_ns"].as_f64().unwrap();
        let cost = got["ns_per_element"].as_f64().unwrap();
        assert!(ns > 0.0 && cost.is_finite() && cost > 0.0);
        assert!((cost - ns / (1000000.0 * expected as f64)).abs() < 1e-10);
        let words = got["canonical_words"].as_array().unwrap();
        let count = match pattern {
            "four-streams" => 64,
            "scaled-product" => 24,
            "affine-expression" => 32,
            _ => 16,
        };
        assert_eq!(words.len(), count);
        assert!(words.iter().all(|v| v
            .as_array()
            .is_some_and(|v| v.len() == 4 && v.iter().all(Value::is_u64))));
        assert!(rows
            .insert((pattern.to_owned(), block, mode.to_owned()), got)
            .is_none());
    }
    let patterns = [
        "chain",
        "four-streams",
        "scaled-product",
        "affine-expression",
    ];
    let mut summaries = BTreeMap::new();
    let mut block_ratios = vec![Vec::new(); 31];
    let mut passed = true;
    let mut failures = Vec::new();
    let mut complex_words = 0;
    for p in patterns {
        let mut ratios = Vec::new();
        let mut reference = Vec::new();
        let mut candidate = Vec::new();
        for block in 0..31 {
            let a = &rows[&(p.into(), block, "reference".into())];
            let b = &rows[&(p.into(), block, "interleaved".into())];
            assert_eq!(
                a["canonical_words"], b["canonical_words"],
                "all output words {p} {block}"
            );
            complex_words += a["canonical_words"].as_array().unwrap().len() * 4;
            let aa = a["ns_per_element"].as_f64().unwrap();
            let bb = b["ns_per_element"].as_f64().unwrap();
            reference.push(aa);
            candidate.push(bb);
            let ratio = bb / aa;
            ratios.push(ratio);
            block_ratios[block as usize].push(ratio);
        }
        let ratio = median(&ratios);
        let wins = ratios.iter().filter(|x| **x < 1.0).count();
        let nonreg = ratio <= 1.03;
        let target = p != "affine-expression" || (ratio < 1.0 && wins >= 21);
        passed &= nonreg && target;
        if !nonreg {
            failures.push(format!("{p} exceeds3%paired median"));
        }
        if !target {
            failures.push("affine expression improvement/21wins".into());
        }
        summaries.insert(p,json!({"reference_ns_per_element":median(&reference),"interleaved_ns_per_element":median(&candidate),"paired_median_ratio":ratio,"wins":wins,"nonregression_pass":nonreg,"target_pass":target,"ratios":ratios}));
    }
    let per_pattern = patterns.map(|p| summaries[p]["paired_median_ratio"].as_f64().unwrap());
    let aggregate = geo(&per_pattern);
    let aggregate_blocks = block_ratios.iter().map(|v| geo(v)).collect::<Vec<_>>();
    let wins = aggregate_blocks.iter().filter(|v| **v < 1.0).count();
    let aggregate_pass = aggregate < 1.0 && wins >= 21;
    passed &= aggregate_pass;
    if !aggregate_pass {
        failures.push("four-pattern aggregate improvement/21wins".into());
    }
    let result = json!({"status":"complete","passed":passed,"adopted":false,"patterns":summaries,"aggregate_ratio":aggregate,"aggregate_wins":wins,"aggregate_block_ratios":aggregate_blocks,"failures":failures,"measured_processes":248,"measured_iterations":248000000u64,"priming_iterations":1488000,"all_output_word_comparisons":complex_words,"all_output_words_equal":true,"scope":"Native arithmetic component with initial packing and final conversion outside timing; no complete inversion stream, SDK proof, pressure, mobile, cloud or production speed claim."});
    fs::write(
        root.join("component-result.json"),
        format!("{}\n", serde_json::to_string_pretty(&result).unwrap()),
    )
    .unwrap();
    println!("Component gate {} aggregate {:.9} wins{wins}/31; all{complex_words} output word comparisons equal",if passed{"PASS"}else{"FAIL"},aggregate);
}
