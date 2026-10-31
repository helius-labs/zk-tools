use serde_json::{json,Value};use std::{collections::BTreeMap,fs,path::Path,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";
fn main(){let mut rows=vec![];fs::create_dir_all(format!("{R}/codegen-asm")).unwrap();
 for arm in["accepted","OFF","ON"]{let binary=if arm=="accepted"{"/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk".to_string()}else{format!("{R}/build/sdk-{arm}-sdk")};
  let out=Command::new("nm").args(["-S","--defined-only"]).arg(&binary).output().unwrap();assert!(out.status.success());let text=String::from_utf8(out.stdout).unwrap();let mut n=0;
  for line in text.lines(){let f=line.split_whitespace().collect::<Vec<_>>();if f.len()!=4{continue;}let sym=f[3];let name=rustc_demangle::try_demangle(sym).map(|x|format!("{x:#}")).unwrap_or(sym.into());if !(name.contains("msm::stream_g1_impl")||name.contains("msm::g2_prefix::apply")||name.contains("msm::g2_borrowed::apply")){continue;}
   let size=u64::from_str_radix(f[1],16).unwrap();let out=Command::new("objdump").args(["-d","--no-show-raw-insn"]).arg(format!("--disassemble={sym}")).arg(&binary).output().unwrap();assert!(out.status.success());let asm=String::from_utf8(out.stdout).unwrap();let path=format!("{R}/codegen-asm/{arm}-{n:02}.asm");fs::write(&path,&asm).unwrap();
   let instructions=asm.lines().filter_map(|l|l.split_once(':')).filter_map(|(_,tail)|tail.split_whitespace().next()).collect::<Vec<_>>();let ifma=instructions.iter().filter(|s|s.starts_with("vpmadd52")).count();let stack_mem=asm.lines().filter(|l|l.contains("(%rsp)")||l.contains("(%rbp)")).count();rows.push(json!({"arm":arm,"symbol":name,"mangled":sym,"bytes":size,"instruction_lines":instructions.len(),"IFMA_sites":ifma,"stack_base_operand_lines":stack_mem,"asm":Path::new(&path).strip_prefix(R).unwrap()}));n+=1;
  }assert_eq!(n,9);
 }
 fs::write(format!("{R}/codegen.json"),serde_json::to_vec_pretty(&json!({"complete":true,"contexts":rows,"dynamic_spill_or_critical_path_claim":false})).unwrap()).unwrap();println!("Codegen27contexts retained; static lengths/IFMA/stack-base operands are descriptive, not dynamic costs");}
