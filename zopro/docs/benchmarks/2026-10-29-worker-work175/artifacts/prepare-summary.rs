use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/worker-work175";
fn rep(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"expected once {a}");*s=s.replace(a,b);}
fn main(){let mut s=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-current-cpu-summary/generate.rs").unwrap();
    rep(&mut s,"    let root = Path::new(Z);","    let root = Path::new(Z);\n    let worker_work=read(root.join(\"docs/benchmarks/2026-10-29-worker-work175/artifacts/analysis.json\"));\n    assert_eq!(worker_work[\"independent_proof_count\"],22198);");
    rep(&mut s,"        \"2026-10-29-stage-uprobes175\",","        \"2026-10-29-stage-uprobes175\",\n        \"2026-10-29-worker-work175\",");
    rep(&mut s,"    fs::write(dest.join(\"README.md\"),format!(\"{}\\n\",md.trim_end())).unwrap();",r#"
    md+="\n## Retired work across worker counts\n\nThree fixed PMU blocks compare the same frozen SDK at1/16/32workers with no uprobes or source rebuild. Counts include worker-policy differences as well as runtime work; CPU time is not an instruction count. No new latency qualification or adoption follows. [Full diagnostic](../2026-10-29-worker-work175/README.md).\n\n|Workers|Instructions/request|Cycles/request|IPC|CPU ms/request|All panels qualified|\n|---|---:|---:|---:|---:|---|\n";
    for w in[1,16,32]{let p=&worker_work["topologies"][w.to_string()];let num=|v:&Value,n:usize|v.as_f64().map(|v|format!("{v:.n$}")).unwrap_or_else(||"—".into());writeln!(md,"|{w}|{}|{}|{}|{}|{}|",num(&p["counters"]["instructions:u"]["median"],0),num(&p["counters"]["cycles:u"]["median"],0),num(&p["median_IPC"],3),num(&p["median_CPU_ms_per_request"],3),p["all_panels_qualified"]).unwrap();}
    md+="\nAll nine new exports independently verify; the trial adds306SDKrequests and9newGnark proofs. Three existing topology preflights are reused. The ledger is now22,198; production, all accepted local/AWS/M5 tables and the prior failed parallel-trace overhead checks remain unchanged.\n";
    fs::write(dest.join("README.md"),format!("{}\n",md.trim_end())).unwrap();"#);
    rep(&mut s,"    fs::write(\n        dest.join(\"data.json\"),",r#"    data["source_commit"]=json!("a4ac1d8ae88ebb020fb83f5c51e036741d07cb2d");
    data["historical_distinct_Gnark_ledger"]=json!(22198);
    data["latest_worker_work_diagnostic"]=json!({"report":"../2026-10-29-worker-work175/README.md","diagnostic_only":true,"topologies":worker_work["topologies"],"all_counter_panels_qualified":worker_work["all_counter_panels_qualified"],"SDK_requests":306,"new_independent_exports":9,"independent_proof_count":22198,"source_rebuilds":0,"production_changed":false});
    fs::write(
        dest.join("data.json"),"#);
    fs::write(Path::new(R).join("summary-generator.rs"),s).unwrap();println!("Prepared worker-counter summary source before capture");
}
