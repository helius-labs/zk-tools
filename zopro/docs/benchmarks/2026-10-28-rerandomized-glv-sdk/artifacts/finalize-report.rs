use std::fs;
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let mut s = fs::read_to_string("close.rs").unwrap();
    fs::write("close-before-report-review.rs", &s).unwrap();
    once(&mut s,"table[\"latency\"].as_str().unwrap(),table[\"pairs\"].as_str().unwrap()","format!(\"| Workers | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\\n|---|---|---:|---:|---:|---:|\\n{}\",table[\"latency\"].as_str().unwrap()),format!(\"| Workers | Ratio | Paired median | Marginal interval | Wins | Check |\\n|---|---|---:|---|---:|---|\\n{}\",table[\"pairs\"].as_str().unwrap())");
    once(&mut s,"No owned build/helper/hash/archive or second benchmark overlaps capture; host snapshots do not establish exclusive-host use.","No owned build/helper/hash/archive or second benchmark overlaps capture. The post-capture host snapshot contains unrelated CPU workloads; there is no per-process capture-time trace establishing their contribution. It does not establish exclusive-host use or an exclusive cause of calibration drift.");
    once(&mut s,"report.push_str(table[\"assembly_profiles\"].as_str().unwrap());","report.push_str(\"| Workers | Variant | Samples | Lost | Matched groth16 symbol period |\\n|---|---|---:|---:|---:|\\n\");\n    report.push_str(table[\"assembly_profiles\"].as_str().unwrap());\n    report.push_str(\"\\nThis narrow symbol-name matcher misses inlined and generic curve arithmetic. A near-zero matched share does not mean assembly was eliminated.\\n\");");
    once(&mut s,"The component gain is not transferred to proof latency.","The component gain is not transferred to proof latency. In this run the 32-worker GLV/fresh estimate improves 1.156%, with bounds showing a 0.290% to 2.547% reduction, and 12/15 wins. The 16-worker GLV/fresh estimate improves 3.293%, bounds 1.043% to 5.082%, also 12/15 wins. Both positive observations are retained. Against accepted code the 32-worker point estimate is 0.936% slower with an interval spanning parity, while the 16-worker estimate is 0.200% faster with a broad interval. Fresh/accepted calibration fails at both worker counts. Thus the study supports the measured comparisons against fresh code but does not demonstrate an improvement over accepted production. It does not prove that all observed changes arise solely from GLV; host variation and compiled context remain unresolved.");
    fs::write("close.rs", s).unwrap();
    println!("Updatedreportinterpretation/headers/hostandmatcherscope;measurement/gatesuntouched");
}
