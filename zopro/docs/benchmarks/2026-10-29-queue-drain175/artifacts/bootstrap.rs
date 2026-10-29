use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/queue-drain175";
const P:&str="/home/ubuntu/src/zopro-perf-work/component-partition175";
fn change(s:&mut String,a:&str,b:&str){assert!(s.contains(a),"missing {a}");*s=s.replace(a,b);}
fn main(){let r=Path::new(R);assert!(!r.join("capture.rs").exists());
    for name in["sampling.rs","counter.rs"]{fs::copy(Path::new(P).join(name),r.join(name)).unwrap();}
    let mut capture=fs::read_to_string(Path::new(P).join("capture.rs")).unwrap();change(&mut capture,P,R);change(&mut capture,"/home/ubuntu/src/zopro-perf-work/sample-accounting175/proof-sha256-after.json","/home/ubuntu/src/zopro-perf-work/component-partition175/proof-sha256-after.json");change(&mut capture,"22234","22366");change(&mut capture,"passed.len(),128","passed.len(),127");
    let a=capture.find("    let required=").unwrap();let b=capture[a..].find("    let mut ids=").unwrap()+a;
    capture.replace_range(a..b,"    let required=[\"msm_affine::queue_drain::tests::simd_width_pending_and_signed_destination_guards\",\"msm_affine::borrowed_pending::queue_drain_tests::drain_and_fallback_match_independent_buckets\",\"msm_affine::g2_borrowed::queue_drain_tests::drain_and_fallback_match_independent_buckets\"];\n");
    fs::write(r.join("capture.rs"),capture).unwrap();
    let mut analysis=fs::read_to_string(Path::new(P).join("analysis.rs")).unwrap();change(&mut analysis,"22366","22498");change(&mut analysis,"22234","22366");change(&mut analysis,"symbol.contains(\"::g2_phi::partition::\")","symbol.contains(\"drain_queued_batches\")");change(&mut analysis,"new_partition_context_period","new_queue_drain_context_period");change(&mut analysis,"\"native_core_tests_each\":128","\"native_core_tests_each\":127");
    let before="}else{stats[\"high\"].as_f64().unwrap()<1.&&stats[\"wins\"].as_u64().unwrap()>=10}";
    let after="}else if c[\"workers\"]==16{stats[\"high\"].as_f64().unwrap()<=1.02}else{stats[\"high\"].as_f64().unwrap()<1.&&stats[\"wins\"].as_u64().unwrap()>=10}";
    change(&mut analysis,before,after);fs::write(r.join("analysis.rs"),analysis).unwrap();
    let old=fs::read_to_string(Path::new(P).join("publish.rs")).unwrap();fs::write(r.join("publish.rs"),old.replace(P,R).replace("io::{Read,Write}","io::Read")).unwrap();
    println!("Prepared source-derived SDK capture/analysis/archive helpers; new prospective primary32 gain/physical16 nonregression gate");
}
