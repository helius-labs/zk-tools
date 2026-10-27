use serde_json::json;
use std::{fs, path::Path};
const R: &str = "/tmp/zopro-composed-cpu";
fn main() {
    let r = Path::new(R);
    fs::copy("/tmp/zopro-g1-reverse-sdk/stats.rs", r.join("stats.rs")).unwrap();
    let mut p = fs::read_to_string("/tmp/zopro-g1-reverse-sdk/prepare.rs")
        .unwrap()
        .replace("/tmp/zopro-g1-reverse-sdk", R);
    p=p.replace("read(\"/tmp/zopro-g1-reverse-overlap/native/core/tests.json\")","read(r.join(\"native/core/tests.json\"))").replace("        111\n","        116\n").replace("native111checked","native116checked")
        .replace("[\"exact_component_stream_body\"]","[\"all170accepted_bytes_recovered_by_reversing_composition\"]")
        .replace("        \"/tmp/zopro-tree-kernel-split/proof-inventory.json\",","        \"/tmp/zopro-tree-kernel-split/proof-inventory.json\",\n        \"/tmp/zopro-g1-reverse-sdk/proof-inventory.json\",")
        .replace("8310","8595").replace("8319","8604").replace("p[\"blocks\"] = json!(9)","p[\"blocks\"] = json!(15)")
        .replace("native/overlap/artifacts.json","native/composed/artifacts.json").replace("native/overlap-sdk","native/composed-sdk").replace("\"overlap\"","\"composed\"").replace("-w4-overlap","-w4-composed").replace("9block3variant","15block3variant");
    assert!(!p.contains("/tmp/zopro-g1-reverse-overlap"));
    fs::write(r.join("prepare.rs"), p).unwrap();
    let mut a = fs::read_to_string("/tmp/zopro-g1-reverse-sdk/analyze.rs")
        .unwrap()
        .replace("/tmp/zopro-g1-reverse-sdk", R);
    a = a
        .replace("\"overlap\"", "\"composed\"")
        .replace("overlap/", "composed/")
        .replace("assert_eq!(rs.len(), 9)", "assert_eq!(rs.len(), 15)")
        .replace("for block in 0..9", "for block in 0..15")
        .replace("(0..9)", "(0..15)")
        .replace("(1..=9)", "(1..=15)")
        .replace("c[\"lower\"], 2.", "c[\"lower\"], 4.")
        .replace("c[\"upper\"], 8.", "c[\"upper\"], 12.")
        .replace("0.9609375", "0.96484375")
        .replace("Exactnine-block", "Exactfifteen-block")
        .replace(
            "assert_eq!(p[\"blocks\"], 9)",
            "assert_eq!(p[\"blocks\"], 15)",
        )
        .replace(
            "result[\"distinct_verified_exports\"], 276",
            "result[\"distinct_verified_exports\"], 456",
        )
        .replace("13944", "22224")
        .replace("570", "930")
        .replace("285", "465")
        .replace("8595", "9060")
        .replace("8319", "8604")
        .replace(
            "\"component_native_core_tests_reused\":111",
            "\"native_core_tests\":116",
        )
        .replace(
            "\"reverse_overlap_groups\":2",
            "\"composed_source_changes\":3",
        )
        .replace("Overlapfullpass", "Composedfullpass")
        .replace("13980SDK465Gnark", "22260SDK465Gnark");
    fs::write(r.join("analyze.rs"), a).unwrap();
    fs::write(r.join("harness-adaptation.json"),serde_json::to_vec_pretty(&json!({"parent_harness":"/tmp/zopro-g1-reverse-sdk","variants":["accepted","fresh","composed"],"prior":8595,"preflight_next":8604,"blocks":15,"latency_exports":450,"profile_exports":6,"total_exports":465,"proof_members":930,"next_prior":9060,"SDK_requests":22260,"profiles":6,"native_core_tests":116,"primary32_gain_upper_below_one":true,"individual_median_nominal_coverage":0.96484375,"all10_nonreg_upper_limit":1.02,"all10_calibration_factor":1.03})).unwrap()).unwrap();
    println!("Adapted15block3variantstandardSDKstudy;22260SDK465exports;prior8595->9060.");
}
