use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-descriptor-chunks176";
const O:&str="/home/ubuntu/src/zopro-perf-work/g2-two-chain-stream176";
fn main(){
 assert!(!Path::new(R).join("campaign.rs").exists());
 for name in["reference-audit.rs","capture-prepare.rs","campaign.rs"]{
  let mut s=fs::read_to_string(format!("{O}/{name}")).unwrap().replace(O,R).replace("two_chain_stream","descriptor_chunks").replace("two-chain-apply.rs","descriptor-apply.rs").replace("test result: ok. 128 passed;","test result: ok. 127 passed;");
  if name!="reference-audit.rs"{s=s.replace("\"freeze.rs\",\"reference-audit.rs\"","\"freeze.rs\",\"codegen.rs\",\"codegen.json\",\"linked-code/OFF-G2.txt\",\"linked-code/ON-G2.txt\",\"reference-audit.rs\"");}
  fs::write(format!("{R}/{name}"),s).unwrap();
 }
 let mut protocol=fs::read_to_string(format!("{O}/protocol.md")).unwrap();
 for(a,b)in[("Prospective two-chain kernel screen","Prospective typed-descriptor G2 kernel screen"),("19a37c35830e0300a3040fc9d82a3b16d384ed0560a218c779dd375aed399c91","740708e5ab03491c26b5db4e843b0af95c5a6bdf2c9634652b514d6471dfd99c"),("Reused native128tests/three new tests are not rerun.","Completed native127tests include two original oracle populations reapplied to this changed function; they are not new mathematical populations and are not rerun for capture."),("parameter/chain-count sweep","layout/attribute/compiler rescue sweep")]{assert!(protocol.contains(a));protocol=protocol.replace(a,b);}
 protocol.push_str("\nSource opportunity is typed fixed-size descriptor groups plus reverse group/record zip, with original single prefix chain, unchanged scalar tail/arithmetic and destination bounds. Linked OFF/ON static bytes51830/50614, instruction sites8590/8391, bounds-panic call sites36/19, conditional jumps96/69, IFMA1450/1450. These are static opportunities, not a predicted CPU or proof speedup. The two-chain candidate stays closed. The fixed original native/reference/latency/profiling gates above are declared before this candidate's first timing.\n");
 fs::write(format!("{R}/protocol.md"),protocol).unwrap();
 println!("Created reference helper, capture controller and prospective475-step protocol; no reference run/capture plan/timing yet");
}
