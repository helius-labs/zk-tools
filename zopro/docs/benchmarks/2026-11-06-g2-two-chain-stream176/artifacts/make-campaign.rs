use std::{fs,path::Path};
fn main(){let root=Path::new("/home/ubuntu/src/zopro-perf-work/g2-two-chain-stream176");assert!(!root.join("campaign.rs").exists());let old=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-denominator-tree176/campaign.rs").unwrap();let a=old.find("fn prepare(){").unwrap();let b=old.find("fn own(data:").unwrap();let mut s=old[..a].to_owned()+&fs::read_to_string(root.join("capture-prepare.rs")).unwrap()+&old[b..];for(from,to)in[
("const R:&str=\"/home/ubuntu/src/zopro-perf-work/g2-denominator-tree176\";","const R:&str=\"/home/ubuntu/src/zopro-perf-work/g2-two-chain-stream176\";"),
("let qualified=count>=400&&unknown_share<=0.05;","let qualified=count>=1000&&unknown_share<=0.05;"),
("\"proof_ledger\":28349","\"proof_ledger\":30846"),
("Closed474 commands/444checkedcallers","Closed475 commands/444checkedcallers"),
("if [\"preflight-\",\"latency-\",\"stat-\",\"cycles-\"].iter()","if n!=\"preflight-check\"&&[\"preflight-\",\"latency-\",\"stat-\",\"cycles-\"].iter()"),
("let mut count=0u64;let mut unknown=0;","let assigned=v[\"worker_rows\"].as_array().unwrap().iter().map(|v|(v[\"tid\"].as_u64().unwrap(),v[\"cpu\"].as_u64().unwrap())).collect::<BTreeMap<_,_>>();let mut count=0u64;let mut unknown=0;"),
("assert!(cpu<32);assert_eq!(f[3].parse::<u64>().unwrap(),16000000);","assert!(cpu<32);let tid=t.parse::<u64>().unwrap();if tid!=pid{assert_eq!(assigned[&tid],cpu);}assert_eq!(f[3].parse::<u64>().unwrap(),16000000);"),
("Some(\"finish\")=>finish(),","Some(\"finish\")=>finish(),Some(\"preflight\")=>preflight(),")
]{assert_eq!(s.matches(from).count(),1,"{from}");s=s.replacen(from,to,1);}fs::write(root.join("campaign.rs"),s).unwrap();println!("Generated475stepcontroller;preflighthelper excluded from CALLERs;complete perfenv andworkerCPUchecks;no capture.");}
