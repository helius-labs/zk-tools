use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,collections::BTreeMap,path::Path,process::Command};
const R:&str="/home/ubuntu/src/zopro-perf-work/ifma-logjumps176";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn main(){
 let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");let m:Value=serde_json::from_slice(&fs::read(&manifest).unwrap()).unwrap();let original=m["source_files_sha256"].as_object().unwrap();assert_eq!(original.len(),176);assert!(!Path::new(&format!("{R}/source")).exists());
 for(p,h)in original{let from=Path::new(Z).join(p);assert_eq!(hash(&from),h.as_str().unwrap());let to=Path::new(R).join("source").join(p);fs::create_dir_all(to.parent().unwrap()).unwrap();fs::copy(from,to).unwrap();}
 let parent=format!("{R}/source/crates/zopro-bn254/src/ifma.rs");let mut source=fs::read_to_string(&parent).unwrap();
 let marker="    #[inline(always)]\n    fn product_cios<const FUSED: bool>";assert_eq!(source.matches(marker).count(),1);
 let mut kernel=fs::read_to_string(format!("{R}/kernel.rs.in")).unwrap();let mut terms=String::new();for diagonal in 0..=8{for i in 0..5{for j in 0..5{if i+j==diagonal{terms.push_str(&format!("            term!({i},{j},{},{});\n",i+j,i+j+1));}}}}kernel=kernel.replace("            // PRODUCT_TERMS",&terms);
 source=source.replace(marker,&format!("{kernel}\n{marker}"));
 let dispatch="    fn product<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {";assert_eq!(source.matches(dispatch).count(),1);source=source.replace(dispatch,&format!("{dispatch}\n        if cfg!(zopro_ifma_logjumps) && LANES == 8 && MIXED {{return self.product_logjumps::<FUSED>(b,c,d);}}"));
 let start=source.find("    #[test]\n    fn cios_carry_matches_arkworks()").unwrap();let body=source[start..].find('{').unwrap()+start;let mut end=body+1;let mut depth=1;while depth>0{match source.as_bytes()[end]{b'{'=>depth+=1,b'}'=>depth-=1,_=>()}end+=1;}
 let mut test=source[start..end].replace("cios_carry_matches_arkworks","logjumps_matches_arkworks").replace(".product_cios::<",".product_logjumps::<").replace("CIOS_CARRY_FIELD_LANES","LOGJUMPS_FIELD_LANES");
 test=test.replace("check::<crate::FqConfig,ark_ref::Fq,false>();","").replace("check::<crate::FrConfig,ark_ref::Fr,false>();","");
 let constants=r#"    #[test]
    fn logjumps_constants_are_inverse_powers() {
        fn check<C:FpConfig,A:ark_ff::PrimeField>() {
            use ark_ff::Field;
            for bits in[48,52,100,104] {
                let words=Ifma::<C,true>::logjumps_rho(bits);
                assert!(crate::portable::less(words,C::P));
                let bytes=words.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>();
                let value=A::from_le_bytes_mod_order(&bytes);
                assert_eq!(value*A::from(2u64).pow([bits as u64]),A::ONE);
            }
        }
        check::<crate::FqConfig,ark_ref::Fq>();check::<crate::FrConfig,ark_ref::Fr>();
    }
"#;
 source.insert_str(start,&format!("{test}\n{constants}\n"));fs::write(&parent,source).unwrap();
 let generator=format!("{R}/source/scripts/ifma8.py");let mut s=fs::read_to_string(&generator).unwrap();let find="\"cios_carry_matches_arkworks\"]:";assert_eq!(s.matches(find).count(),1);s=s.replace(find,"\"cios_carry_matches_arkworks\", \"logjumps_matches_arkworks\", \"logjumps_constants_are_inverse_powers\"]:");fs::write(&generator,s).unwrap();
 let generated=Command::new("python3").arg(&generator).output().unwrap();fs::write(format!("{R}/generator.stdout.log"),generated.stdout).unwrap();fs::write(format!("{R}/generator.stderr.log"),generated.stderr).unwrap();assert!(generated.status.success());
 assert!(Command::new("python3").args([&generator,"--check"]).status().unwrap().success());
 let mut pins=BTreeMap::new();let mut changed=BTreeMap::new();for(p,h)in original{let p=Path::new(R).join("source").join(p);let actual=hash(&p);if actual!=h.as_str().unwrap(){changed.insert(p.display().to_string(),actual.clone());}pins.insert(p.display().to_string(),actual);}
 assert_eq!(changed.len(),3);
 fs::write(format!("{R}/source-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();
 fs::write(format!("{R}/source-audit.json"),serde_json::to_vec_pretty(&json!({"files":176,"originals_exact":173,"changed":changed,"production_changed":false,"kernel":"parallel104+100+52 LogJumps","new_heap_buffers":0,"constant_payload_per_prime_bytes":128,"generator_consistent":true})).unwrap()).unwrap();
 let mut steps=vec![];
 for label in["OFF","ON"]{
  let flags=format!("{}{}",m["rustflags"].as_str().unwrap(),if label=="ON"{" --cfg zopro_ifma_logjumps"}else{""});fs::write(format!("{R}/{label}-flags.txt"),&flags).unwrap();
  let mut env:Value=serde_json::from_slice(&fs::read("/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/environment.json").unwrap()).unwrap();env["RUSTFLAGS"]=json!(flags);env["CARGO_TARGET_DIR"]=json!(format!("{R}/target-{label}"));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");let mf=format!("{R}/source/Cargo.toml");
  for(name,package,kind)in[("field","zopro-bn254","--lib"),("core","zopro-core","--lib"),("prover","zopro-prover","--lib"),("cubic","zopro-prover","cubic")]{let mut argv=vec!["cargo".to_string(),"test".into(),"--locked".into(),"--offline".into(),"--release".into(),"--manifest-path".into(),mf.clone(),"-p".into(),package.into()];if kind=="cubic"{argv.extend(["--test".into(),"cubic".into()]);}else{argv.push(kind.into());}argv.extend(["--".into(),"--test-threads=1".into()]);let mut step=json!({"kind":"command","name":format!("{label}-{name}"),"argv":argv,"environment":env,"unfiltered":true,"timeout_seconds":1800});if name=="field"{step["required_tests"]=json!(["ifma::tests::logjumps_matches_arkworks","ifma::tests::logjumps_constants_are_inverse_powers","ifma8::tests::logjumps_matches_arkworks","ifma8::tests::logjumps_constants_are_inverse_powers"]);}steps.push(step);}
 }
 for f in["setup.rs","setup","design.md","kernel.rs.in","model.rs","model","model.json","model.log","model-initial.rs","model-type-repair-1.rs","source-audit.json","OFF-flags.txt","ON-flags.txt"]{let p=format!("{R}/{f}");pins.insert(p.clone(),hash(p));}
 let tool="/home/ubuntu/src/zopro-perf-work/sdk-link-flags176/tool-target/release/xtask";pins.insert(tool.into(),hash(tool));
 fs::write(format!("{R}/qualification-pins.json"),serde_json::to_vec_pretty(&pins).unwrap()).unwrap();fs::write(format!("{R}/qualification-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":format!("{R}/qualification-pins.json"),"steps":steps})).unwrap()).unwrap();
 println!("Prepared176privatefiles/173exactoriginals, generatorconsistent, OFF/ON field/core/prover/cubic8qualificationsteps; no timings");
}
