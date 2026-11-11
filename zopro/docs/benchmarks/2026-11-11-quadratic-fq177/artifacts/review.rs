use num_bigint::BigInt as I;
use num_traits::One;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/quadratic-fq177";
fn read(p:&str)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:&str)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn integer(v:&Value)->I{I::parse_bytes(v.as_str().unwrap().as_bytes(),10).unwrap()}
fn main(){
 let v=read(&format!("{R}/result.json"));assert_eq!(v["passed"],true);let p=integer(&v["p"]);let a=integer(&v["a"]);let u=integer(&v["u"]);let r=integer(&v["R"]);let h=integer(&v["H"]);let t=integer(&v["root"]);
 assert_eq!(r,I::one()<<131usize);assert_eq!(h,I::one()<<127usize);assert_eq!(a,6*&u*&u+3*&u+1);assert_eq!(p,&a*&a+3*&u*&u);assert_eq!((&a+&u*&t)%&p,I::from(0));assert_eq!((&t*&t+3)%&p,I::from(0));
 assert!(&a+3*&u<h);let real:I=16*&h*&h+&r*(&a+3*&u);let imag:I=8*&h*&h+&r*(&a+&u);let limit:I=2*&r*&h;assert!(real<limit&&imag<limit);assert_eq!(real,integer(&v["bounds"]["fused_real_numerator"]));assert_eq!(imag,integer(&v["bounds"]["fused_imag_numerator"]));assert_eq!(limit,integer(&v["bounds"]["strict_limit"]));
 let n=10*10+4096;assert_eq!(v["checks"],json!({"input_pairs":n,"ordinary":n+16,"fused":2*n,"codec_and_linear":3*n,"fused_corner_cases":32,"changing_chain":8192,"Fq2_components":3072}));
 // Count low/high partial products by their bit positions and maximum widths.
 let sites=|left:&[usize],right:&[usize],limit:usize|{let mut count=0;for(i,&x)in left.iter().enumerate(){for(j,&y)in right.iter().enumerate(){let start=(i+j)*52;if start<limit{count+=1;}if start+52<limit&&x+y>52{count+=1;}}}count};
 let raw=3*sites(&[52,52,25],&[52,52,25],312);let low=3*sites(&[52,52,28],&[52,52,28],131);let fixed=2*sites(&[52,52,28],&[52,52,23],312)+sites(&[52,52,28],&[52,11],312);assert_eq!((raw,low,fixed),(51,27,45));assert_eq!(v["IFMA_recipe"]["ordinary"],raw+low+fixed);assert_eq!(v["IFMA_recipe"]["fused2"],2*raw+low+fixed);
 for(name,expected)in[("run",1),("fixed-run",0)]{let plan=read(&format!("{R}/{}",if name=="run"{"plan.json"}else{"fixed-plan.json"}));for step in plan["steps"].as_array().unwrap(){let s=step["name"].as_str().unwrap();if name=="run"&&s=="qualify"{continue}let dir=format!("{R}/{name}/{s}");let exit=read(&format!("{dir}/exit.json"));assert_eq!(exit["returncode"],expected);let command=read(&format!("{dir}/command.json"));assert_eq!(&command["argv"].as_array().unwrap()[4..],step["argv"].as_array().unwrap());for stream in["stdout","stderr"]{assert_eq!(hash(&format!("{dir}/{stream}.log")),exit[format!("{stream}_sha256")]);}}}
 for(path,h)in read(&format!("{R}/fixed-pins.json")).as_object().unwrap(){assert_eq!(hash(path),h.as_str().unwrap());}
 let ledger="/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf-profile177/proof-sha256-after-profile.json";assert_eq!(read(ledger).as_array().unwrap().len(),53917);assert_eq!(hash(ledger),"a033132ee87cf5d5260c396acf1894bbb1afc95c07d8ba22c55ed9c8e5a85873");
 let out=json!({"passed":true,"scope":"same-agent separate algebra/bounds/site/receipt reader; numerical model not replayed","checked_comparisons":36488,"recipe_sites":[raw+low+fixed,2*raw+low+fixed],"workflow_commands":3,"successful_commands":2,"failed_compile_commands":1,"proof_ledger_unchanged":53917,"new_proofs":0,"native_selected":false});fs::write(format!("{R}/review.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("Independent analytic bounds and partial-product ledger match; source/command pins and unchanged ledger pass.");
}
