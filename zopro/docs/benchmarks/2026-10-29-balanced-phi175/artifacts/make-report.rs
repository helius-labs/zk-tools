use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-phi175";
fn rep(s: &mut String, a: &str, b: &str) {
    assert!(s.contains(a), "missing{a}");
    *s = s.replace(a, b);
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("report.rs").exists());
    let mut s =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175/report.rs")
            .unwrap();
    let a = s.find("    let mut b=String::from(").unwrap();
    s.truncate(a);
    rep(&mut s, "inverse-direct-sdk175", "balanced-phi175");
    rep(&mut s, "18154", "18670");
    let a = s.find("            text.strip_suffix(").unwrap();
    let b = s[a..]
        .find("        } else {\n            bytes.clone()")
        .unwrap()
        + a;
    s.replace_range(a..b,r#"            text.replacen(after,audit["before"].as_str().unwrap(),1).into_bytes()
        } else if p == audit["second_parent"].as_str().unwrap() {
            std::str::from_utf8(&bytes).unwrap().strip_suffix(audit["second_parent_suffix"].as_str().unwrap()).unwrap().as_bytes().to_vec()
"#);
    rep(&mut s, "exact, 174", "exact, 173");
    rep(
        &mut s,
        "[(\"native\", 13), (\"runs\", 165)]",
        "[(\"native\", 10), (\"runs\", 165)]",
    );
    rep(&mut s, "receipts.len(), 178", "receipts.len(), 175");
    rep(&mut s, "streams, 712", "streams, 700");
    rep(
        &mut s,
        "zopro_bn254::inverse::inverse::<",
        "zopro_core::msm::g2_borrowed::apply::<",
    );
    rep(
        &mut s,
        "defining_inverse_period_fraction",
        "defining_g2_apply_period_fraction",
    );
    let a = s.find("    for f in [\n        \"analysis.json\"").unwrap();
    let b = s[a..].find("    ] {").unwrap() + a;
    s.replace_range(a..b,r#"    for f in [
        "analysis.json", "finish.log", "design.md", "history-audit.md", "balanced.rs", "bootstrap.rs", "setup.rs", "native.rs",
        "audit.rs", "audit.log", "code-audit.json", "source-audit.json", "native-flags.txt", "candidate-flags.txt",
        "build-pins.json", "build-plan.json", "frozen-control.json", "frozen-candidate.json", "frozen-control-cargo.jsonl", "frozen-candidate-cargo.jsonl",
        "native/control/artifacts.json", "native/candidate/artifacts.json", "identities.json", "make-capture.rs", "capture-setup.rs", "capture.rs",
        "pins.json", "run-plan.json", "jobs.json", "cases.json", "preflight-cases.json", "prior-proof-sha256.json", "proof-inventory.json",
        "pmu-feasibility.log", "prepare.log", "host-before.txt", "host-after.txt", "make-report.rs", "report.rs", "report-body.rs",
"#);
    rep(
        &mut s,
        "\"native_workflow_steps\":13",
        "\"native_workflow_steps\":10",
    );
    rep(&mut s, "\"receipts\":178", "\"receipts\":175");
    rep(
        &mut s,
        "\"original_streams_byte_exact\":712",
        "\"original_streams_byte_exact\":700",
    );
    rep(&mut s, "\"originals_exact\":174", "\"originals_exact\":173");
    rep(
        &mut s,
        "\"old_primitive_schedule_not_repeated\":true",
        "\"original_uniform_width_partition_trial_not_repeated\":true",
    );
    s.push_str("    write_readme(&a,&manifest,&profiles,&d);\n    seal();\n    println!(\"Archived balancedphi175;7902SDK78Gnark ledger18670;no adoption\");\n}\ninclude!(\"report-body.rs\");\nfn main(){match std::env::args().nth(1).as_deref(){Some(\"archive\")=>archive(),Some(\"seal\")=>seal(),_=>panic!(\"archive|seal\")}}\n");
    fs::write(r.join("report.rs"), s).unwrap();
}
