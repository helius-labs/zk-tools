use serde_json::{json, Value};
use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let t: Value =
        serde_json::from_slice(&fs::read(r.join("all-nine-template-v2.json")).unwrap()).unwrap();
    let mut records = Vec::new();
    for mode in ["off", "on"] {
        for c in t["cases"].as_array().unwrap() {
            let shape = c["shape"].as_str().unwrap();
            let p = r
                .join("preflight-v2")
                .join(format!("{mode}-w4-{shape}-prove"));
            let log = fs::read_to_string(p.join("stderr.log")).unwrap();
            let n = log
                .lines()
                .filter(|l| l.starts_with("FUSED_PRODUCT_OVERLAP "))
                .count();
            let expected = if mode == "on" && shape == "1x2" { 4 } else { 0 };
            assert_eq!(n, expected, "{mode}/{shape}");
            records.push(json!({"mode":mode,"shape":shape,"trace_calls":n,"expected":expected}));
        }
    }
    let result = json!({"status":"passed","observations":records,"primary_on_calls":4,"off_and_ineligible_calls":0,"scope":"Complete native SDK preflight confirms actual branch entry; timing/profile trace remains disabled. No pressure fallback activation claim yet."});
    fs::write(
        r.join("activation-audit-v2.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Actual ON primary overlap4calls;OFFandallineligiblenone; activationPASS");
}
