use std::{fs,path::Path};
fn main(){let r="/home/ubuntu/src/zopro-perf-work/g2-descriptor-chunks176";assert!(!Path::new(r).join("review.rs").exists());
 let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-two-chain-stream176/review.rs").unwrap();
 for(a,b)in[("/home/ubuntu/src/zopro-perf-work/g2-two-chain-stream176",r),("two-chain-apply.rs","descriptor-apply.rs"),("two_chain_stream","descriptor_chunks"),("test result: ok. 128 passed;","test result: ok. 127 passed;"),("l.contains(\"two_chain_tests::\")","l.contains(\"msm::descriptor_chunks::\")"),("assert_eq!(selected.len(),3);","assert_eq!(selected.len(),2);")]{assert!(s.contains(a),"{a}");s=s.replace(a,b);}
 fs::write(format!("{r}/review.rs"),s).unwrap();println!("Prepared separate source/inverse/receipt/raw-statistics/profile reviewer; no successful stage repeated");}
