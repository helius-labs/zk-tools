use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/stage-uprobes175";
fn rep(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"expected once {a}");*s=s.replace(a,b);}
fn main(){let r=Path::new(R);if std::env::args().nth(1).as_deref()==Some("pins"){let mut pins:Value=serde_json::from_slice(&fs::read(r.join("pins.json")).unwrap()).unwrap();for f in["prepare-summary.rs","prepare-summary","summary-generator.rs","summary-generator"]{let p=r.join(f);let h=format!("{:x}",Sha256::digest(fs::read(&p).unwrap()));pins[p.display().to_string()]=json!(h);}fs::write(r.join("pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();return;}
    let mut s=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-current-cpu-summary/generate.rs").unwrap();
    rep(&mut s,"    let root = Path::new(Z);","    let root = Path::new(Z);\n    let stage_trace=read(root.join(\"docs/benchmarks/2026-10-29-stage-uprobes175/artifacts/analysis.json\"));\n    assert_eq!(stage_trace[\"independent_proof_count\"],22189);");
    rep(&mut s,"        \"2026-10-29-interleaved-complex175\",","        \"2026-10-29-interleaved-complex175\",\n        \"2026-10-29-stage-uprobes175\",");
    rep(&mut s,"    fs::write(dest.join(\"README.md\"),format!(\"{}\\n\",md.trim_end())).unwrap();",r#"
    md+="\n## Frozen accepted SDK stage diagnostic\n\nEntry/return probes observe nine function boundaries in the exact accepted executable, without a source rebuild. Seven blocks compare plain and traced primary requests at1/16/32workers. These are instrumented stage intervals, not replacement latency results; nested/parallel spans overlap and K/H feed is partial. [Full diagnostic](../2026-10-29-stage-uprobes175/README.md).\n\n|Workers|Plain ms|Traced ms|Paired ratio|Bounds|Within factor1.03|\n|---|---:|---:|---:|---|---|\n";
    for w in[1,16,32]{let p=&stage_trace["overhead"][w.to_string()];writeln!(md,"|{w}|{:.6}|{:.6}|{:.9}|[{:.9}, {:.9}]|{}|",p["plain_median_ms"].as_f64().unwrap(),p["traced_median_ms"].as_f64().unwrap(),p["paired_ratio"].as_f64().unwrap(),p["low"].as_f64().unwrap(),p["high"].as_f64().unwrap(),p["within_factor1_03"]).unwrap();}
    md+="\nAll21recorded processes have31completeSDKintervals, balanced entry/return stacks, one PID and zero lost records. The trial adds1440SDKrequests and45fresh independent Gnark exports; the ledger is now22,189. All probes in the owned group were removed. Production and all accepted local/AWS/M5 tables remain unchanged. No source adoption or new performance qualification follows.\n";
    fs::write(dest.join("README.md"),format!("{}\n",md.trim_end())).unwrap();"#);
    rep(&mut s,"    fs::write(\n        dest.join(\"data.json\"),",r#"    data["source_commit"]=json!("e323fe88b81cc510ab6477ae865a6a7f90a96dd9");
    data["historical_distinct_Gnark_ledger"]=json!(22189);
    data["latest_stage_diagnostic"]=json!({"report":"../2026-10-29-stage-uprobes175/README.md","diagnostic_only":true,"overhead":stage_trace["overhead"],"stages":stage_trace["stages"],"SDK_requests":1440,"new_independent_exports":45,"independent_proof_count":22189,"source_rebuilds":0,"production_changed":false});
    fs::write(
        dest.join("data.json"),"#);
    fs::write(r.join("summary-generator.rs"),s).unwrap();println!("Prepared summary source; no capture or new data");
}
