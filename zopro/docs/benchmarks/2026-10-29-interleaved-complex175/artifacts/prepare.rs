use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};
const R:&str="/home/ubuntu/src/zopro-perf-work/interleaved-complex175";const Z:&str="/home/ubuntu/src/zk-tools/zopro";
fn r(p:impl AsRef<Path>)->PathBuf{Path::new(R).join(p)}fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}fn save(p:impl AsRef<Path>,v:&Value){fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap()}
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"missing unique {a}");*s=s.replace(a,b);}
fn main(){
    assert!(!r("source").exists());assert_eq!(read(r("model.json"))["status"],"passed");assert_eq!(read(r("model.json"))["exact_complex_CIOS_cases"],60248);
    let mp=Path::new(Z).join("docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");let m=read(&mp);let mut pins=BTreeMap::new();pins.insert(mp.display().to_string(),hash(&mp));
    let ms="\n#[cfg(zopro_gauss_caller_probe)]\nmod caller_probe;\n#[cfg(zopro_gauss_caller_probe)]\npub use caller_probe::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    let ls="\n#[cfg(zopro_gauss_caller_probe)]\npub use msm::{Caller,CallerG1,CallerG2,BorrowedG1,BorrowedG2};\n";
    let frozenroot=Path::new("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/source-frozen");
    for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=Path::new(Z).join(f);assert_eq!(hash(&p),h.as_str().unwrap());let q=r("source").join(f);fs::create_dir_all(q.parent().unwrap()).unwrap();fs::copy(&p,&q).unwrap();pins.insert(p.display().to_string(),hash(&p));let old=frozenroot.join(f);let mut b=fs::read(&old).unwrap();for(file,suffix)in[("crates/zopro-core/src/msm.rs",ms),("crates/zopro-core/src/lib.rs",ls)]{if f==file{assert!(b.ends_with(suffix.as_bytes()));b.truncate(b.len()-suffix.len());}}assert_eq!(format!("{:x}",Sha256::digest(b)),h.as_str().unwrap());pins.insert(old.display().to_string(),hash(old));}
    let parent="/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/source/crates/zopro-bn254/src/nonnegative_gauss.rs";let old=fs::read_to_string(parent).unwrap();fs::write(r("parent-unsigned-cios.rs"),&old).unwrap();let start=old.find("    #[cfg(zopro_gauss_positive)]").unwrap();let mut kernel=format!("impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{{\n{}",&old[start..]);
    replace(&mut kernel,"#[cfg(zopro_gauss_positive)]","#[cfg(zopro_interleaved_complex)]");replace(&mut kernel,"pub fn complex_product_gauss(","pub fn complex_interleaved<const CONJ:bool>(");
    let a=kernel.find("            let mut s = [zero; 5];").unwrap();let b=kernel.find("            let mut x = [zero; 6];").unwrap();kernel.replace_range(a..b,"            let mut borrow = zero;\n");
    let a=kernel.find("                    let tv = ").unwrap();let b=kernel[a..].find("                    macro_rules! cancel {").unwrap()+a;
    kernel.replace_range(a..b,r#"                    let v = _mm512_add_epi64(_mm512_sub_epi64(two_p[$i],b1.0[$i]),borrow);
                    borrow = _mm512_srai_epi64::<52>(v);
                    let negative = _mm512_and_si512(v,mask);
                    let real_b1 = if CONJ {b1.0[$i]}else{negative};
                    let imag_b1 = if CONJ {negative}else{b1.0[$i]};
                    macro_rules! product {
                        ($j:literal,$k:literal)=>{{
                            x[$j]=_mm512_madd52lo_epu64(x[$j],a0.0[$j],b0.0[$i]);
                            x[$k]=_mm512_madd52hi_epu64(x[$k],a0.0[$j],b0.0[$i]);
                            x[$j]=_mm512_madd52lo_epu64(x[$j],a1.0[$j],real_b1);
                            x[$k]=_mm512_madd52hi_epu64(x[$k],a1.0[$j],real_b1);
                            y[$j]=_mm512_madd52lo_epu64(y[$j],a1.0[$j],b0.0[$i]);
                            y[$k]=_mm512_madd52hi_epu64(y[$k],a1.0[$j],b0.0[$i]);
                            y[$j]=_mm512_madd52lo_epu64(y[$j],a0.0[$j],imag_b1);
                            y[$k]=_mm512_madd52hi_epu64(y[$k],a0.0[$j],imag_b1);
                        }};
                    }
                    product!(0,1);product!(1,2);product!(2,3);product!(3,4);product!(4,5);
"#);
    kernel.push_str(&fs::read_to_string(r("field-tests.rs")).unwrap());fs::write(r("kernel.rs"),&kernel).unwrap();fs::write(r("source/crates/zopro-bn254/src/interleaved_complex.rs"),&kernel).unwrap();
    let fsuffix="\n#[cfg(zopro_interleaved_complex)] include!(\"interleaved_complex.rs\");\n";let p=r("source/crates/zopro-bn254/src/ifma8.rs");let mut s=fs::read_to_string(&p).unwrap();s.push_str(fsuffix);fs::write(p,s).unwrap();
    let mut edits=vec![];let p=r("source/crates/zopro-core/src/msm.rs");let mut s=fs::read_to_string(&p).unwrap();
    let a="        let value = [\n            dy[0].sum2(&dx[0], &dy[1], &dx[1]),\n            dy[1].difference2(&dx[0], &dy[0], &dx[1]),\n        ];";
    let b="        let value = {\n            #[cfg(zopro_interleaved_complex)]{FqBatch::complex_interleaved::<true>(&dy[0],&dy[1],&dx[0],&dx[1])}\n            #[cfg(not(zopro_interleaved_complex))]{[\n                dy[0].sum2(&dx[0], &dy[1], &dx[1]),\n                dy[1].difference2(&dx[0], &dy[0], &dx[1]),\n            ]}\n        };";
    replace(&mut s,a,b);edits.push((a.to_string(),b.to_string()));
    let a="            [\n                a[0].difference2(&b[0], &a[1], &b[1]),\n                a[0].sum2(&b[1], &a[1], &b[0]),\n            ]";
    let b="            #[cfg(zopro_interleaved_complex)]{FqBatch::complex_interleaved::<false>(&a[0],&a[1],&b[0],&b[1])}\n            #[cfg(not(zopro_interleaved_complex))]{[\n                a[0].difference2(&b[0], &a[1], &b[1]),\n                a[0].sum2(&b[1], &a[1], &b[0]),\n            ]}";
    replace(&mut s,a,b);edits.push((a.to_string(),b.to_string()));s.push_str(ms);fs::write(p,s).unwrap();
    let p=r("source/crates/zopro-core/src/lib.rs");let mut s=fs::read_to_string(&p).unwrap();s.push_str(ls);fs::write(p,s).unwrap();fs::copy(r("caller_probe.rs"),r("source/crates/zopro-core/src/msm/caller_probe.rs")).unwrap();
    let mut exact=0;for(f,h)in m["source_files_sha256"].as_object().unwrap(){let p=r("source").join(f);if hash(&p)==h.as_str().unwrap(){exact+=1}else{let mut s=fs::read_to_string(&p).unwrap();let suffix=match f.as_str(){"crates/zopro-core/src/msm.rs"=>ms,"crates/zopro-core/src/lib.rs"=>ls,"crates/zopro-bn254/src/ifma8.rs"=>fsuffix,_=>panic!("unexpected change {f}")};assert!(s.ends_with(suffix));s.truncate(s.len()-suffix.len());if f=="crates/zopro-core/src/msm.rs"{for(a,b)in edits.iter().rev(){replace(&mut s,b,a);}}assert_eq!(format!("{:x}",Sha256::digest(s)),h.as_str().unwrap());}pins.insert(p.display().to_string(),hash(p));}assert_eq!(exact,172);
    for f in["crates/zopro-bn254/src/interleaved_complex.rs","crates/zopro-core/src/msm/caller_probe.rs"]{pins.insert(r("source").join(f).display().to_string(),hash(r("source").join(f)));}
    let flags=format!("{} --cfg zopro_gauss_caller_probe",m["rustflags"].as_str().unwrap());fs::write(r("native-flags.txt"),&flags).unwrap();
    let frozen=read("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/artifacts-frozen.json");assert_eq!(frozen["rustflags"],flags);for(p,h)in frozen["input_pins"].as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());pins.insert(p.into(),h.as_str().unwrap().into());}save(r("artifacts-frozen.json"),&frozen);
    save(r("source-audit.json"),&json!({"originals":175,"originals_exact":exact,"reverse175_exact":true,"snapshot_files":177,"msm_edits":edits,"msm_suffix":ms,"lib_suffix":ls,"field_suffix":fsuffix,"frozen_inverse175_verified":true,"production_changed":false}));
    let mut steps=vec![];
    for v in["off","on"]{let mut env=m["environment"].clone();env["RUSTFLAGS"]=json!(if v=="on"{format!("{flags} --cfg zopro_interleaved_complex")}else{flags.clone()});env["CARGO_TARGET_DIR"]=json!(r(format!("target-{v}")));env["CARGO_BUILD_JOBS"]=json!("16");env["RAYON_NUM_THREADS"]=json!("4");
        let suites=if v=="on"{vec![("zopro-bn254","field"),("zopro-core","core"),("zopro-prover","prover")]}else{vec![("zopro-core","core"),("zopro-prover","prover")]};
        for(pkg,suite)in suites {let mut args=vec!["cargo".to_string(),"test".into(),"--manifest-path".into(),r("source/Cargo.toml").display().to_string(),"--locked".into(),"--offline".into(),"--release".into(),"-p".into(),pkg.into(),"--lib".into()];if suite=="prover"{args.extend(["--test".into(),"cubic".into()]);}args.extend(["--".into(),"--test-threads=1".into()]);steps.push(json!({"kind":"command","name":format!("{suite}-{v}"),"argv":args,"environment":env,"unfiltered":true,"timeout_seconds":1800}));}
        steps.push(json!({"kind":"command","name":format!("libraries-{v}"),"argv":["cargo","build","--manifest-path",r("source/Cargo.toml"),"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{v}"),"argv":[r("native-helper"),"freeze",v],"environment":env,"timeout_seconds":180}));steps.push(json!({"kind":"command","name":format!("link-{v}"),"argv":[r("native-helper"),"link",v],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"link-frozen","argv":[r("native-helper"),"link","frozen"],"timeout_seconds":600}));assert_eq!(steps.len(),12);
    for f in["prepare.rs","native.rs","caller_probe.rs","probe.rs","perf.rs","design.md","kernel.rs","field-tests.rs","parent-unsigned-cios.rs","source-audit.json","native-flags.txt","model.rs","model.json","model.log","model.stderr.log","model-build.log","model-build-initial.log","model-initial.stderr.log","Cargo.toml","Cargo.lock"]{pins.insert(r(f).display().to_string(),hash(r(f)));}
    save(r("build-pins.json"),&json!(pins));save(r("build-plan.json"),&json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}));println!("Prepared12native steps;172of175exact/inverse175; joint ordinary-basis outputs with streamed complement");
}
