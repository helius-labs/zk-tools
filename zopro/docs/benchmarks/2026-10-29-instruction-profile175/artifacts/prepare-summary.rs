use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/instruction-profile175";
fn rep(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"expected once {a}");*s=s.replace(a,b);}
fn main(){let mut s=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-current-cpu-summary/generate.rs").unwrap();rep(&mut s,"    let root = Path::new(Z);","    let root = Path::new(Z);\n    let instruction_profiles=read(root.join(\"docs/benchmarks/2026-10-29-instruction-profile175/artifacts/analysis.json\"));\n    assert_eq!(instruction_profiles[\"independent_proof_count\"],22210);");
    rep(&mut s,"        \"2026-10-29-worker-work175\",","        \"2026-10-29-worker-work175\",\n        \"2026-10-29-instruction-profile175\",");
    rep(&mut s,"    fs::write(dest.join(\"README.md\"),format!(\"{}\\n\",md.trim_end())).unwrap();",r#"
    md+="\n## Frozen SDK instruction and cycle attribution\n\nTwelve FIFO-gated profiles compare separate instructions:u and cycles:u sampling at16/32workers, three blocks each, without rebuilding or changing the accepted executable. Flat period shares are diagnostic, not exclusive costs or measured per-IP CPI. [Full profiles](../2026-10-29-instruction-profile175/README.md).\n\n|Topology/event|Samples|All attribution panels qualified|\n|---|---:|---|\n";
    for(k,p)in instruction_profiles["aggregates"].as_object().unwrap(){writeln!(md,"|{k}|{}|{}|",p["samples"],p["all_attribution_qualified"]).unwrap();}
    md+="\nThe trial adds1,848SDKrequests and12fresh independent Gnark exports; the ledger is now22,210. No source adoption or new latency qualification follows. All accepted local/AWS/M5 tables and earlier failed gates remain unchanged.\n";
    fs::write(dest.join("README.md"),format!("{}\n",md.trim_end())).unwrap();"#);
    rep(&mut s,"    fs::write(\n        dest.join(\"data.json\"),",r#"    data["source_commit"]=json!("7faaeca0cb882ce636f290795c4d1d593fad39bd");
    data["historical_distinct_Gnark_ledger"]=json!(22210);
    data["latest_instruction_profile_diagnostic"]=json!({"report":"../2026-10-29-instruction-profile175/README.md","diagnostic_only":true,"aggregates":instruction_profiles["aggregates"],"all_attribution_qualified":instruction_profiles["all_attribution_qualified"],"SDK_requests":1848,"new_independent_exports":12,"independent_proof_count":22210,"source_rebuilds":0,"production_changed":false});
    fs::write(
        dest.join("data.json"),"#);
    fs::write(Path::new(R).join("summary-generator.rs"),s).unwrap();println!("Prepared instruction/cycle summary source before capture");}
