use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/fft-prefix-pruning176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const D:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-07-fft-prefix-pruning176";
const GEOMETRY:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-24-derivative-component/fused-reference/rows.json";
fn r(p:&str)->PathBuf{Path::new(R).join(p)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:impl AsRef<Path>,v:&Value){assert!(!p.as_ref().exists());fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
#[derive(Default,Debug,PartialEq,Eq)]
struct Counts{butterflies:u64,products:u64,outputs:u64}
fn graph(n:usize,c:usize,retain:bool)->(Counts,Vec<Vec<bool>>){
 assert!(n.is_power_of_two()&&c<=n);let mut live=vec![false;n];live[..c].fill(true);let mut counts=Counts::default();let mut masks=vec![];
 for log in (0..n.trailing_zeros()).rev(){let gap=1usize<<log;let mut previous=vec![false;n];if retain{masks.push(live.clone());}
  for base in (0..n).step_by(2*gap){for j in 0..gap{let lo=base+j;let hi=lo+gap;if live[lo]||live[hi]{counts.butterflies+=1;counts.products+=u64::from(j!=0);counts.outputs+=u64::from(live[lo])+u64::from(live[hi]);previous[lo]=true;previous[hi]=true;}}}
  live=previous;
 }
 masks.reverse();(counts,masks)
}
const P:u64=65537;
fn pow(mut a:u64,mut n:usize)->u64{let mut r=1;while n!=0{if n&1!=0{r=r*a%P;}a=a*a%P;n>>=1;}r}
fn direct(input:&[u64],n:usize,k:usize,omega:u64)->u64{let x=pow(omega,k);input.iter().rev().fold(0,|s,&a|(s*x+a)%P)}
fn pruned(input:&[u64],c:usize,omega:u64)->Vec<u64>{
 let n=input.len();let(_,masks)=graph(n,c,true);let log=n.trailing_zeros();
 let mut a=(0..n).map(|i|{let k=if log==0{0}else{i.reverse_bits()>>(usize::BITS-log)};input[k]}).collect::<Vec<_>>();
 for(level,mask)in masks.iter().enumerate(){let gap=1<<level;let root=pow(omega,n/(2*gap));
  for base in (0..n).step_by(2*gap){let mut w=1;for j in 0..gap{let lo=base+j;let hi=lo+gap;if mask[lo]||mask[hi]{let u=a[lo];let v=a[hi]*w%P;if mask[lo]{a[lo]=(u+v)%P;}if mask[hi]{a[hi]=(u+P-v)%P;}}w=w*root%P;}}
 }
 a.truncate(c);a
}
fn check()->Value{
 let mut prefix_checks=0;let mut oracle_values=0;
 for n in [2usize,4,8,16,32,64,128,256]{let full=graph(n,n,false).0;assert_eq!(full.butterflies,(n*n.trailing_zeros()as usize/2)as u64);assert_eq!(full.products,full.butterflies-(n-1)as u64);
  for c in 0..=n{let counts=graph(n,c,false).0;assert!(counts.products<=full.products&&counts.butterflies<=full.butterflies);if c>=n/2{assert_eq!(counts.products,full.products);assert_eq!(counts.butterflies,full.butterflies);assert_eq!(full.outputs-counts.outputs,(n-c)as u64);}prefix_checks+=1;}
  if n<=64{let root=pow(3,65536/n);assert_eq!(pow(root,n),1);assert_ne!(pow(root,n/2),1);for population in 0..3{let input=(0..n).map(|i|match population{0=>((i as u64+13)*(i as u64+971)*193)%P,1=>u64::from(i==n-1),_=>P-1}).collect::<Vec<_>>();let dft=(0..n).map(|k|direct(&input,n,k,root)).collect::<Vec<_>>();for c in 0..=n{let out=pruned(&input,c,root);assert_eq!(out,dft[..c]);oracle_values+=c;}}}
 }
 assert!(graph(256,1,false).0.products<graph(256,256,false).0.products);
 json!({"all_prefix_graph_checks":prefix_checks,"direct_DFT_output_comparisons":oracle_values,"oracle_field":65537,"dense_last_coefficient_and_near_modulus_inputs":true,"small_prefix_positive_control":true})
}
fn prepare(){
 let mut pins=BTreeMap::new();for p in [r("model.rs"),r("design.md"),r("model"),PathBuf::from(GEOMETRY),Path::new(Z).join("crates/zopro-core/src/fft.rs"),Path::new(Z).join("crates/zopro-core/src/fft/pair.rs")]{pins.insert(p.display().to_string(),hash(p));}
 let accepted=read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));for s in ["crates/zopro-core/src/fft.rs","crates/zopro-core/src/fft/pair.rs"]{assert_eq!(hash(Path::new(Z).join(s)),accepted["source_files_sha256"][s].as_str().unwrap());}
 save(r("pins.json"),&json!(pins));save(r("plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":[{"kind":"command","name":"prefix-dependency-and-DFT-screen","argv":[r("model"),PathBuf::from("run")],"timeout_seconds":300}]}));println!("Prepared exact FFT sources and existing geometry receipt pins");
}
fn run(){
 for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let checks=check();let mut geometries=BTreeMap::new();for row in read(GEOMETRY).as_array().unwrap(){let shape=row["shape"].as_str().unwrap().to_string();let n=row["preparation"]["domain"].as_u64().unwrap()as usize;let c=row["preparation"]["rows"].as_u64().unwrap()as usize;if let Some(old)=geometries.insert(shape,(n,c)){assert_eq!(old,(n,c));}}
 assert_eq!(geometries.len(),9);let mut rows=vec![];
 for(shape,(n,c))in geometries{assert!(c>n/2&&c<=n);let full=graph(n,n,false).0;let prefix=graph(n,c,false).0;assert_eq!(full.products,prefix.products);assert_eq!(full.butterflies,prefix.butterflies);assert_eq!(full.outputs-prefix.outputs,(n-c)as u64);
  rows.push(json!({"shape":shape,"domain":n,"requested_outputs":c,"discarded_outputs":n-c,"discarded_fraction":(n-c)as f64/n as f64,"full_butterflies":full.butterflies,"prefix_butterflies":prefix.butterflies,"full_nonunit_products":full.products,"prefix_nonunit_products":prefix.products,"full_add_sub_nodes":full.outputs,"prefix_add_sub_nodes":prefix.outputs,"removed_add_sub_nodes":full.outputs-prefix.outputs,"add_sub_fraction_removed":(full.outputs-prefix.outputs)as f64/full.outputs as f64}));
 }
 save(r("analysis.json"),&json!({"checks":checks,"rows":rows,"mathematics_passed":true,"actual_geometry_product_opportunity":false,"runtime_candidate_selected":false,"new_timings":0,"new_proofs":0,"production_changed":false,"proof_ledger_count":31387}));println!("All nine geometries retain every nonunit twiddle product; correctness checks pass");
}
fn publish(){
 let a=read(r("analysis.json"));assert_eq!(a["mathematics_passed"],true);assert_eq!(a["actual_geometry_product_opportunity"],false);for(p,h)in read(r("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
 let receipt=r("runs/prefix-dependency-and-DFT-screen");let exit=read(receipt.join("exit.json"));assert_eq!(exit["success"],true);for(s,k)in[("stdout.log","stdout_sha256"),("stderr.log","stderr_sha256")]{assert_eq!(hash(receipt.join(s)),exit[k].as_str().unwrap());}
 let dest=Path::new(D);assert!(!dest.exists());fs::create_dir_all(dest.join("artifacts")).unwrap();let mut body=String::from("# Euler FFT output-prefix dependency screen\n\n**Simple natural-output prefix pruning removes no twiddle products on any of the nine supplied geometries.** It can remove final add/sub outputs, but not the multiplication work in the existing radix2 DIT dependency graph. No native kernel or new performance campaign is selected. This does not rule out other transform factorizations or output orders.\n\nThe current Euler path performs a full forward transform before consuming the first constraint-count values. Saved geometry receipts show every requested prefix exceeds half the padded domain. At the final butterfly level, that prefix includes every lower output. Each such output needs both previous-stage inputs and the shared twiddle product, so every preceding stage remains live. This explains why discarding a large output tail is not a proportional multiplication saving.\n\n## Model results\n\nCounts are per single scalar transform, not the compiled paired SIMD kernel. Unit-root products are excluded consistently; actual kernels may execute them. No measured CPU work, latency ceiling, memory saving or AWS comparison is inferred. Geometry counts are reused from the prior derivative study; no key/solver/FFT capture was rerun.\n\n| Shape | Domain | Required outputs | Discarded | Nonunit products: full / prefix | Removed add/sub nodes | Share of full add/sub nodes |\n|---|---:|---:|---:|---:|---:|---:|\n");
 for row in a["rows"].as_array().unwrap(){body.push_str(&format!("| {} | {} | {} | {} | {} / {} | {} | {:.3}% |\n",row["shape"].as_str().unwrap(),row["domain"],row["requested_outputs"],row["discarded_outputs"],row["full_nonunit_products"],row["prefix_nonunit_products"],row["removed_add_sub_nodes"],100.*row["add_sub_fraction_removed"].as_f64().unwrap()));}
 body.push_str(&format!("\nThe graph checks all {} prefixes across domains 2–256. A separate forward evaluator checks {} requested values against direct polynomial DFTs over F65537, with checked root orders, full-width finite-field inputs, a last-coefficient basis input and near-modulus inputs. A one-output positive control does prune products, so the model is not hardcoded to report a full transform. These checks qualify a finite-field dependency model, not a BN254 runtime implementation.\n",a["checks"]["all_prefix_graph_checks"],a["checks"]["direct_DFT_output_comparisons"]));
 body.push_str("\nBoth inspected FFT sources match the accepted manifest. One compile, prepare, xtask model command and publication succeed once. There is no separate independent parser/reviewer. The actual prefix-product opportunity check fails without a percentage threshold; saved add/sub nodes are retained in the table. A transformed-output ordering, sparse-value specialization or alternative FFT algorithm would need a distinct algebra/cost/consumer study.\n\nAccepted production176/SDK6d83, the 31,387-proof ledger and [all eight supplied M5 CPU and Metal comparisons](../2026-11-07-nonbinary-radix176/README.md#accepted-proof-baseline-unchanged) remain unchanged. Local accepted 1×2 is 7.915095 ms and merge54×1 is 161.718956 ms. The all-eight CPU10× goal is active and unmet (0/8). No new proof, native curve/field test, profile, device/pressure/minimum-RAM or throughput measurement is claimed.\n\n[Design](artifacts/design.md), [model](artifacts/model.rs), [analysis](artifacts/analysis.json), [pins](artifacts/pins.json), [workflow](artifacts/plan.json), [command](artifacts/command.json), [exit receipt](artifacts/exit.json) and [checksums](SHA256SUMS).\n");
 fs::write(dest.join("README.md"),body).unwrap();let mut members=vec![PathBuf::from("README.md")];
 for(s,t)in[(r("model.rs"),"model.rs"),(r("design.md"),"design.md"),(r("analysis.json"),"analysis.json"),(r("pins.json"),"pins.json"),(r("plan.json"),"plan.json"),(r("runs/result.json"),"result.json"),(receipt.join("command.json"),"command.json"),(receipt.join("exit.json"),"exit.json"),(receipt.join("stdout.log"),"stdout.log"),(receipt.join("stderr.log"),"stderr.log")]{let rel=Path::new("artifacts").join(t);fs::copy(&s,dest.join(&rel)).unwrap();assert_eq!(hash(&s),hash(dest.join(&rel)));members.push(rel);}
 members.sort();fs::write(dest.join("SHA256SUMS"),members.iter().map(|p|format!("{}  {}\n",hash(dest.join(p)),p.display())).collect::<String>()).unwrap();println!("Published source, aggregate graph results and receipts; no native trial selected");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("run")=>run(),Some("publish")=>publish(),_=>panic!("prepare | run | publish")}}
