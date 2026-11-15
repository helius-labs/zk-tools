use std::fs;
fn main(){
 let s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/msm-full180/preflight.rs").unwrap();
 let start=s.find("fn inspect(").unwrap();let end=s.find("\nfn main()").unwrap();
 let mut f=s[start..end].to_string();
 for (a,b) in [
 ("fn inspect(name:&str)->Value{","pub fn inspect(name:&str)->Value{"),
 ("preflight-jobs.json","jobs.json"),("preflight-run/","run/"),
 ("assert_eq!(samples.len(),1);","let rounds=j[\"rounds\"].as_u64().unwrap() as usize;let workers=j[\"workers\"].clone();assert_eq!(samples.len(),rounds);for (i,s) in samples.iter().enumerate(){assert_eq!(s[\"round\"],i);let ms=if j[\"engine\"]==\"zopro\"{s[\"ms\"].as_f64().unwrap()}else{s[\"verified_ms\"].as_f64().unwrap()};assert!(ms.is_finite()&&ms>0.0);}"),
 ("assert_eq!(summaries[0][\"rounds\"],1)","assert_eq!(summaries[0][\"rounds\"],rounds)"),
 ("assert_eq!(summaries[0][\"workers\"],4)","assert_eq!(summaries[0][\"workers\"],workers)"),
 ("assert_eq!(summaries[0][\"threads\"],4)","assert_eq!(summaries[0][\"threads\"],workers)"),
 ("if on{4}else{0}","if on{rounds+3}else{0}"),
 ("assert_eq!(phase[\"workers\"],4)","assert_eq!(phase[\"workers\"],workers)"),
 ("\"preflight_time_not_a_performance_result\":true","\"samples\":samples,\"job\":j")
 ]{assert!(f.contains(a),"missing {a}");f=f.replace(a,b);}
 fs::write("/home/ubuntu/src/zopro-perf-work/msm-local180/validate.rs",format!("// Adapted from the qualified full-request preflight reader; transformations retained.\nuse super::*;\n{f}\n")).unwrap();
}
