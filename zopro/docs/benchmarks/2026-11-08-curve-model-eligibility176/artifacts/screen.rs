use num_bigint::BigUint;
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeMap,fs,path::{Path,PathBuf}};
use zopro_bn254::{CurveConfig,Field,FpConfig,Fq,Fq2,FqConfig,G1Config,G2Config};
const R:&str="/home/ubuntu/src/zopro-perf-work/curve-model-eligibility176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const FIELD:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
const BIG:&str="/home/ubuntu/src/zopro-perf-work/triple-cancel175/target/release/deps/libnum_bigint-c6cab6b530f85d82.rlib";
fn path(s:&str)->PathBuf{Path::new(R).join(s)}
fn read(p:impl AsRef<Path>)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String{format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:impl AsRef<Path>,v:&Value){assert!(!p.as_ref().exists());fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn from_words(w:&[u64])->BigUint{BigUint::from_bytes_le(&w.iter().flat_map(|w|w.to_le_bytes()).collect::<Vec<_>>())}
#[derive(Clone,Debug,Eq,PartialEq)]struct I(BigUint,BigUint);
fn mul(a:&I,b:&I,p:&BigUint)->I{I((&a.0*&b.0+p*p-&a.1*&b.1)%p,(&a.0*&b.1+&a.1*&b.0)%p)}
fn power(a:&I,e:&BigUint,p:&BigUint)->I{let mut result=I(1u32.into(),0u32.into());let mut base=a.clone();for byte in e.to_bytes_le(){for bit in 0..8{if byte>>bit&1!=0{result=mul(&result,&base,p);}base=mul(&base,&base,p);}}result}
fn integer(a:Fq2)->I{I(from_words(&a.c0.to_canonical()),from_words(&a.c1.to_canonical()))}
fn enc(a:&I)->Value{json!([a.0.to_str_radix(10),a.1.to_str_radix(10)])}
// Tiny-field group oracle uses direct affine formulas and Fermat inversion.
#[derive(Clone,Copy,Debug,Eq,PartialEq)]struct T(u32,u32);
fn plus(a:T,b:T,p:u32)->T{T((a.0+b.0)%p,(a.1+b.1)%p)}
fn minus(a:T,b:T,p:u32)->T{T((a.0+p-b.0)%p,(a.1+p-b.1)%p)}
fn times(a:T,b:T,p:u32)->T{T((a.0*b.0+p*p-a.1*b.1)%p,(a.0*b.1+a.1*b.0)%p)}
fn tp(mut a:T,mut n:u32,p:u32)->T{let mut r=T(1,0);while n!=0{if n&1!=0{r=times(r,a,p);}a=times(a,a,p);n>>=1;}r}
fn divide(a:T,b:T,p:u32,q:u32)->T{assert_ne!(b,T(0,0));let inverse=tp(b,q-2,p);assert_eq!(times(b,inverse,p),T(1,0));times(a,inverse,p)}
type Point=Option<(T,T)>;
fn add(a:Point,b:Point,p:u32,q:u32)->Point{let(Some((x,y)),Some((u,v)))=(a,b)else{return a.or(b);};let slope=if x==u{if plus(y,v,p)==T(0,0){return None;}assert_eq!(y,v);divide(times(T(3,0),times(x,x,p),p),plus(y,y,p),p,q)}else{divide(minus(v,y,p),minus(u,x,p),p,q)};let rx=minus(minus(times(slope,slope,p),x,p),u,p);Some((rx,minus(times(slope,minus(x,rx,p),p),y,p)))}
fn toy()->Value{
    let mut curves=0;let mut points=0;let mut certificate_cases=0;let mut no2_certificates=0;let mut no3_certificates=0;
    for extension in[false,true]{for p in[7u32,19,31,43]{if extension&&p>19{continue;}let q=if extension{p*p}else{p};let elements=(0..q).map(|v|T(v%p,v/p)).collect::<Vec<_>>();let bs=if extension{vec![T(1,0),T(0,1),T(1,1),T(2,1),T(1,2),T(3,2)]}else{(1..p).map(|v|T(v,0)).collect::<Vec<_>>()};
        for b in bs{curves+=1;let mut n2=0;let mut n3=0;for &x in&elements{let rhs=plus(times(times(x,x,p),x,p),b,p);for &y in&elements{if times(y,y,p)!=rhs{continue;}let point=Some((x,y));let double=add(point,point,p,q);let triple=add(double,point,p,q);let two=y==T(0,0);let psi=times(T(3,0),times(x,plus(times(times(x,x,p),x,p),times(T(4,0),b,p),p),p),p);let three=psi==T(0,0);assert_eq!(two,double.is_none());assert_eq!(three,triple.is_none());n2+=u32::from(two);n3+=u32::from(three);points+=1;}}
            let neg=minus(T(0,0),b,p);let cubic=tp(neg,(q-1)/3,p);let legendre=tp(b,(q-1)/2,p);let other=tp(times(T(p-3,0),b,p),(q-1)/2,p);if cubic!=T(1,0){assert_eq!(n2,0);no2_certificates+=1;}if legendre==T(p-1,0)&&other==T(p-1,0){assert_eq!(n3,0);no3_certificates+=1;}certificate_cases+=1;
        }
    }}
    assert_eq!(curves,108);json!({"prime_field_curves":96,"extension_field_curves":12,"finite_points_checked":points,"character_certificate_cases":certificate_cases,"no_two_torsion_certificates":no2_certificates,"no_three_torsion_certificates":no3_certificates})
}
fn prepare(){let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");let m=read(&manifest);let sources=m["source_files_sha256"].as_object().unwrap();assert_eq!(sources.len(),176);let mut pins=BTreeMap::new();for(p,h)in sources{let p=Path::new(Z).join(p);assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p.display().to_string(),hash(p));}for p in[path("screen.rs"),path("screen"),path("design.md"),manifest,FIELD.into(),BIG.into()]{pins.insert(p.display().to_string(),hash(p));}save(path("pins.json"),&json!(pins));save(path("plan.json"),&json!({"version":1,"source_pins":path("pins.json"),"steps":[{"kind":"command","name":"actual-field-and-small-curve-eligibility","argv":[path("screen"),PathBuf::from("run")],"timeout_seconds":300}]}));println!("Prepared {} pins",pins.len());}
fn run(){
    for(p,h)in read(path("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}let p=from_words(&FqConfig::P);let one=BigUint::from(1u32);let two=BigUint::from(2u32);let three=BigUint::from(3u32);assert_eq!(&p%&three,one);let minus_one=I(&p-&one,0u32.into());let unit=I(one.clone(),0u32.into());let mut rows=Vec::new();
    // Independently derive the twist coefficient, rather than substituting an
    // untwisted Fp curve into the extension-field check.
    let inv82=BigUint::from(82u32).modpow(&(&p-&two),&p);let expected_b=I((27u32*&inv82)%&p,(&p-(3u32*&inv82)%&p)%&p);assert_eq!(integer(G2Config::B),expected_b);
    for extension in[false,true]{let q=if extension{&p*&p}else{p.clone()};let quadratic=(&q-&one)/&two;let cubic=(&q-&one)/&three;let b=if extension{G2Config::B}else{Fq2::new(G1Config::B,Fq::ZERO)};let negative=-b;let minus3=Fq2::new(-Fq::from(3u64),Fq::ZERO)*b;let mut characters=Vec::new();
        for(label,v,e)in[("B_quadratic",b,&quadratic),("minus3B_quadratic",minus3,&quadratic),("minusB_cubic",negative,&cubic)]{let native=if extension{v.pow(&e.to_u64_digits())}else{Fq2::new(v.c0.pow(&e.to_u64_digits()),Fq::ZERO)};let independent=power(&integer(v),e,&p);assert_eq!(integer(native),independent);if label.ends_with("quadratic"){assert_eq!(independent,minus_one);if extension{assert!(v.sqrt().is_none());}else{assert!(v.c0.sqrt().is_none());}}else{assert_ne!(independent,unit);assert_eq!(power(&independent,&three,&p),unit);}characters.push(json!({"label":label,"input":enc(&integer(v)),"exponent":e.to_str_radix(10),"character":enc(&independent)}));}
        rows.push(json!({"curve":if extension{"G2 actual sextic twist over Fp2"}else{"G1 over Fp"},"field_cardinality":q.to_str_radix(10),"B":enc(&integer(b)),"characters":characters,"no_rational_two_torsion":true,"no_rational_three_torsion":true,"same_field_montgomery_model_eligible":false,"same_field_twisted_edwards_model_eligible":false,"same_field_twisted_hessian_model_eligible":false}));
    }
    let toy=toy();save(path("analysis.json"),&json!({"schema":1,"p":p.to_str_radix(10),"twist_coefficient_independently_reconstructed":true,"curves":rows,"toy_checks":toy,"native_integer_character_agreements":6,"new_proofs":0,"new_timings":0,"production_changed":false,"native_optimization_selected":false,"proof_ledger_count":37023,"goal_achieved":false}));println!("All six native/integer characters and108 small-curve torsion checks pass; same-field Hessian/Montgomery/Edwards routes unavailable for both BN254 groups");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("run")=>run(),_=>panic!("prepare | run")}}
