use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/interleaved-complex175";
fn rep(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"expected unique {a}");*s=s.replace(a,b);}
fn main(){
    let root=Path::new(R);let mut s=fs::read_to_string(root.join("capture.rs")).unwrap();
    rep(&mut s,"for(v,core)in[(\"off\",128),(\"on\",129)]","for(v,core)in[(\"off\",128),(\"on\",128)]");
    rep(&mut s,"\nlet mut code",r#"
let field=read(r("native/field-on/tests.json"));assert_eq!(field["unfiltered"],true);assert_eq!(field["passed_tests"].as_array().unwrap().len(),35);
for f in["tests.json","command.json","exit.json","stdout.log","stderr.log"]{let p=r(format!("native/field-on/{f}"));pins.insert(p.display().to_string(),hash(p));}
let mut code"#);
    rep(&mut s,"let signature=json!([out[\"output_sha256\"],out[\"fixture_reference_bytes_per_worker\"]]);","let signature=json!([out[\"output_sha256\"],out[\"capacity_bytes_per_worker\"],out[\"fixture_reference_bytes_per_worker\"]]);");
    rep(&mut s,"\"output_reference_signatures_equal\":true","\"output_and_capacity_signatures_equal\":true");
    rep(&mut s,"\"native_field_tests_inherited\":33","\"native_field_tests\":{\"off_inherited\":33,\"on\":35}");
    rep(&mut s,"\"native_core_tests\":{\"off\":128,\"on\":129}","\"native_core_tests\":{\"off\":128,\"on\":128}");
    rep(&mut s,"for f in[\"capture.rs\",\"capture-helper\",\"native/result.json\",\"codegen.json\",\"jobs.json\",\"contexts.json\"]", "for f in[\"capture.rs\",\"capture-helper\",\"native/result.json\",\"codegen.json\",\"jobs.json\",\"contexts.json\",\"publish.rs\",\"publication.rs\",\"publish\",\"summary-generator.rs\",\"summary-generator\",\"prepare-tools.rs\",\"prepare-tools\",\"prepare\",\"native-helper\",\"target/release/model\"]");
    fs::write(root.join("capture.rs"),s).unwrap();
    let old=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-cached-x175/publish.rs").unwrap();let prefix=old.split("include!(\"publication.rs\");").next().unwrap().replace("g2-cached-x175","interleaved-complex175");
    fs::write(root.join("publish.rs"),format!("{prefix}\ninclude!(\"publication.rs\");\nfn main(){{publish()}}\n")).unwrap();
    let mut s=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-current-cpu-summary/generate.rs").unwrap();
    rep(&mut s,"    let root = Path::new(Z);", "    let root = Path::new(Z);\n    let interleaved=read(root.join(\"docs/benchmarks/2026-10-29-interleaved-complex175/artifacts/analysis.json\"));\n    assert_eq!(interleaved[\"complete\"],true);");
    rep(&mut s,"        \"2026-10-29-golden-basis175\",","        \"2026-10-29-golden-basis175\",\n        \"2026-10-29-interleaved-complex175\",");
    rep(&mut s,"    fs::write(dest.join(\"README.md\"), md).unwrap();",r#"
    md+="\n## Latest joint ordinary-basis complex caller\n\n";
    writeln!(md,"The separately declared caller gate **{}**. Both ordinary-basis complex outputs share a row-CIOS schedule, with the complemented second input streamed one digit at a time. The arithmetic count stays300 IFMA sites; field basis, square, norm, scratch and point formulas stay unchanged. Fifteen blocks compare all18 owned/borrowed contexts with exact frozen and fresh OFF controls, including1/16/32worker topologies. All output/capacity checks and zero timed allocations pass. [Full results](../2026-10-29-interleaved-complex175/README.md).\n\n|Aggregate|Ratio|Bounds|Wins / 15|\n|---|---:|---|---:|",if interleaved["component_gate"]==true{"passes"}else{"fails"}).unwrap();
    for(k,p)in interleaved["aggregates"].as_object().unwrap(){writeln!(md,"|{k}|{:.9}|[{:.9}, {:.9}]|{}|",p["median"].as_f64().unwrap(),p["low"].as_f64().unwrap(),p["high"].as_f64().unwrap(),p["wins"]).unwrap();}
    writeln!(md,"\nFailed checks: `{}`. No SDK proofs, new Gnark exports or adoption. The ledger remains22,144; all earlier failed gates and positive contexts remain. These are complete caller waves, not proof timings.\n",interleaved["failed_checks"]).unwrap();
    fs::write(dest.join("README.md"),format!("{}\n",md.trim_end())).unwrap();"#);
    rep(&mut s,"    fs::write(\n        dest.join(\"data.json\"),",r#"    data["source_commit"]=json!("f5aa862c646a887388e3f5ed40650280a5399555");
    data["previous_cached_X_G2_caller"]=data["latest_actual_G2_caller"].clone();
    data["latest_actual_G2_caller"]=json!({"report":"../2026-10-29-interleaved-complex175/README.md","gate_passed":interleaved["component_gate"],"aggregates":interleaved["aggregates"],"pairs":select_records(&interleaved["pairs"], &["median","low","high","wins"]),"failed_checks":interleaved["failed_checks"],"native_field_tests":interleaved["native_field_tests"],"native_core_tests":interleaved["native_core_tests"],"native_prover_tests":interleaved["native_prover_tests"],"SDK_requests":0,"new_independent_exports":0,"production_changed":false});
    fs::write(
        dest.join("data.json"),"#);
    fs::write(root.join("summary-generator.rs"),s).unwrap();println!("Prepared caller analyzer, publisher utility and summary source; no capture");
}
