use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";const OLD:&str="/home/ubuntu/src/zopro-perf-work/inverse-hint-alias176";
fn main(){
 let mut s=fs::read_to_string(format!("{OLD}/review.rs")).unwrap().replace(OLD,R);
 s=s.replace("\"qualification-pins.json\",\"qualification-v2-pins.json\",\"build-pins.json\",\"pins.json\"","\"qualification-pins.json\",\"build-pins.json\",\"pins.json\"");
 s=s.replace("(\"qualification-v2\",2),(\"build\",4)","(\"qualification\",8),(\"build\",6)");
 let lo=s.find(" for(p,n)in[(\"qualification-v2/core/tests.json\"").unwrap();let hi=s[lo..].find(" let a=read(r(\"analysis.json\"))").unwrap()+lo;
 s.replace_range(lo..hi,r#" for arm in["OFF","ON"]{for(name,n)in[("field",39),("core",125),("prover",2),("cubic",6)]{let t=read(r(format!("qualification/{arm}-{name}/tests.json")));assert_eq!(t["unfiltered"],true);assert_eq!(t["passed_tests"].as_array().unwrap().len(),n);}}
 let source=read(r("source-review.json"));assert_eq!(source["originals_exact"],173);assert_eq!(source["originals_recovered"],176);
 for arm in["OFF","ON"]{let frozen=read(r(format!("frozen-{arm}.json")));assert_eq!(frozen["rustflags"].as_str().unwrap().contains("--cfg zopro_ifma_logjumps"),arm=="ON");}
"#);
 let lo=s.find("  if j[\"variant\"]!=\"accepted\"").unwrap();let hi=s[lo..].find("  let dir=Path::new").unwrap()+lo;s.replace_range(lo..hi,"");
 s=s.replace("29051","29255").replace("\"initial_failed_core_preserved\":true,\"source_v2_only_child_layout_repair\":true,","\"all_native_suites_first_pass\":true,\"source_recovery\":source,").replace(";failedsource preserved",";allqualificationfirstpass");
 fs::write(format!("{R}/review.rs"),s).unwrap();println!("Prepared separate rawsample/gate/proof/ledger checker; no campaign analysis imports");
}
