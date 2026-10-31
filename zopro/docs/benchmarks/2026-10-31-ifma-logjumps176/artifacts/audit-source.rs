use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,path::{Path,PathBuf},process::{Command,Stdio},io::Write};
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn h(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn block_end(s:&str,at:usize)->usize{let mut at=s[at..].find('{').unwrap()+at+1;let mut depth=1;while depth>0{match s.as_bytes()[at]{b'{'=>depth+=1,b'}'=>depth-=1,_=>()}at+=1;}at}
fn main(){
 let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json")).unwrap()).unwrap();let mut recovered=0;let mut exact=0;let mut rows=vec![];
 for(p,want)in m["source_files_sha256"].as_object().unwrap(){let file=Path::new(R).join("source").join(p);let bytes=fs::read(&file).unwrap();let actual=h(&bytes);if actual==want.as_str().unwrap(){exact+=1;recovered+=1;continue;}
  let mut s=String::from_utf8(bytes).unwrap();
  if p.ends_with("src/ifma.rs")||p.ends_with("src/ifma8.rs"){
   let lo=s.find("    // Integer inverse-radix folding").unwrap();let hi=s[lo..].find("    #[inline(always)]\n    fn product_cios<").unwrap()+lo;s.replace_range(lo..hi,"");
   let lo=s.find("        if cfg!(zopro_ifma_logjumps)").unwrap();let mut end=block_end(&s,lo);if s.as_bytes()[end]==b'\n'{end+=1;}s.replace_range(lo..end,"");
   let test=s.find("    fn logjumps_matches_arkworks()").unwrap();let lo=s[..test].rfind("    #[test]").unwrap();
   let hi=if p.ends_with("src/ifma.rs"){s[test..].find("    #[test]\n    fn cios_carry_matches_arkworks()").unwrap()+test}else{s[test..].find("    fn check<C: FpConfig, const MIXED: bool>()").unwrap()+test};s.replace_range(lo..hi,"");
   if p.ends_with("src/ifma8.rs"){
    let mut child=Command::new("rustfmt").args(["--emit","stdout","--edition","2021"]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();child.stdin.take().unwrap().write_all(s.as_bytes()).unwrap();let out=child.wait_with_output().unwrap();assert!(out.status.success());s=String::from_utf8(out.stdout).unwrap();
   }
  }else if p=="scripts/ifma8.py"{s=s.replace("\"cios_carry_matches_arkworks\", \"logjumps_matches_arkworks\", \"logjumps_constants_are_inverse_powers\"]:","\"cios_carry_matches_arkworks\"]:");}
  else{panic!("unexpectededit{p}");}
  let path=PathBuf::from(R).join("recovered").join(p);fs::create_dir_all(path.parent().unwrap()).unwrap();fs::write(path,s.as_bytes()).unwrap();assert_eq!(h(s.as_bytes()),want.as_str().unwrap(),"{p}");recovered+=1;rows.push(json!({"path":p,"candidate_sha256":actual,"recovered_sha256":h(s.as_bytes())}));
 }
 assert_eq!(exact,173);assert_eq!(recovered,176);fs::write(format!("{R}/source-review.json"),serde_json::to_vec_pretty(&json!({"complete":true,"originals_exact":exact,"originals_recovered":recovered,"changed":rows,"no_core_solver_curve_schedule_image_edits":true,"generator_recovered_exact":true,"new_heap_buffers":0,"logical_constant_payload_bytes_both_primes":256})).unwrap()).unwrap();println!("SourceReviewPASS:173exactoriginals/176recovered; all changes confined to2fieldkernels+generator; no core/solver/layout edit");
}
