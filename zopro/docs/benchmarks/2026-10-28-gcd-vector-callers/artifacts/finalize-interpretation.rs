use serde_json::{json, Value};
use std::fs;
fn main() {
    let a: Value = serde_json::from_slice(&fs::read("analysis.json").unwrap()).unwrap();
    assert_eq!(a["gates"]["component_gate_passed"], false);
    let failed: Vec<_> = a["pairs"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, v)| k.contains("2-over-") && v["high"].as_f64().unwrap() > 1.02)
        .map(|(k, _)| k.clone())
        .collect();
    fs::write("result-interpretation.json",serde_json::to_vec_pretty(&json!({"logical32_on_off":a["logical32_aggregate"]["on-over-off"],"logical32_on_frozen":a["logical32_aggregate"]["on-over-frozen"],"failed_nonregression_bounds":failed,"all_twelve_on_off_paired_medians_above_one":a["pairs"].as_object().unwrap().iter().filter(|(k,_)|k.ends_with("2-over-1")).all(|(_,v)|v["median"].as_f64().unwrap()>1.),"prototype_gain_reversed_in_actual_point_callers":true,"SMT_alone_explains_reversal":false,"exclusive_cause_proven":false,"SDK_trial_permitted":false})).unwrap()).unwrap();
    let mut s = fs::read_to_string("close-body.rs").unwrap();
    let needle = "    report.push_str(\"\\nThe primary aggregate";
    assert_eq!(s.matches(needle).count(), 1);
    let note = r#"    report.push_str("\nThe native real-stream aggregate is 0.590% slower than fresh OFF and 0.857% slower than frozen, with both marginal intervals above parity. Every one of the twelve ON/OFF paired medians is above one. The owned/200 physical16 estimate is about 2.52% slower than OFF and fails the nonregression bound; other failed bounds are listed in result-interpretation.json. A small roughly 0.277% lower borrowed/352 solo estimate against frozen remains included, but it crosses parity and fresh OFF is already faster than frozen there.\n\nThis reversal also appears without SMT: borrowed/200 solo ON/OFF is about 0.619% slower, and G2/200 solo about 0.730% slower, with bounds above parity. SMT alone therefore cannot explain the integration result. The standalone prototype and native caller differ in surrounding computation, input distributions and compiled context; exact helper source does not establish equivalent whole-routine execution cost. The exclusive cause is unresolved. The positive inverse-only result is retained within its measured scope, while this native integration is closed without an SDK trial.\n");
"#;
    s = s.replace(needle, &format!("{note}{needle}"));
    fs::write("close-body.rs", s).unwrap();
}
