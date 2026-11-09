use super::*;
use zopro_bn254::{Fq,G2Affine,G2Projective};
use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};
use ark_ff::{BigInt,PrimeField};
use std::{collections::BTreeSet,ffi::OsString,sync::{Arc,Mutex,atomic::{Ordering,AtomicUsize}}};
struct Env(Vec<(&'static str,Option<OsString>)>);
impl Env{fn new(values:&[(&'static str,&str)])->Self{let old=values.iter().map(|(k,v)|{let old=std::env::var_os(k);std::env::set_var(k,v);(*k,old)}).collect();Self(old)}}
impl Drop for Env{fn drop(&mut self){for(k,v)in self.0.iter().rev(){if let Some(v)=v{std::env::set_var(k,v)}else{std::env::remove_var(k)}}}}
fn fixture(n:usize)->(Vec<G2Affine>,Vec<Fr>){let mut point=G2Projective::IDENTITY;let mut projectives=Vec::with_capacity(n);let mut weights=Vec::with_capacity(n);for i in 0..n{point+=G2Affine::GENERATOR;let (p,w)=match i%13{0=>(G2Projective::IDENTITY,Fr::ZERO),1=>(G2Affine::GENERATOR.to_projective(),Fr::ONE),2=>(-G2Affine::GENERATOR.to_projective(),-Fr::ONE),3=>(G2Affine::GENERATOR.to_projective(),Fr::ONE),4=>(-point,-Fr::from(i as u64+1)),_=>(point,Fr::from(i as u64+1))};projectives.push(p);weights.push(w);} (G2Projective::batch_to_affine(&projectives),weights)}
fn values(n:usize,kind:usize)->Vec<Fr>{let mut state=Fr::from(813491u64).pow(&[119]);(0..n).map(|i|{state=state*Fr::from(6364136223846793005u64)+Fr::from(i as u64+71);match kind{0=>state,1=>if i%5==0{Fr::ZERO}else{state},2=>Fr::ONE,3=>{let bit=[0,11,12,63,64,125,126,127,128,253][i%10];let p=Fr::from(2).pow(&[bit]);if i%2==0{p-Fr::ONE}else{-p-Fr::ONE}},4=>Fr::from((i%2048+1)as u64),5=>Fr::from((i%23+1)as u64),6=>Fr::ZERO,7=>Fr::from((i%1024+1)as u64),_=>unreachable!()}}).collect()}
fn ark_fr(value:Fr)->ark_bn254::Fr{ark_bn254::Fr::from_bigint(BigInt(value.to_canonical())).unwrap()}
fn ark_fq(value:Fq)->ark_bn254::Fq{ark_bn254::Fq::from_bigint(BigInt(value.to_canonical())).unwrap()}
fn expected(weights:&[Fr],scalars:&[Fr])->ark_bn254::G2Affine{let sum=weights.iter().zip(scalars).fold(ark_bn254::Fr::from(0u64),|sum,(w,s)|sum+ark_fr(*w)*ark_fr(*s));(ark_bn254::G2Projective::generator()*sum).into_affine()}
fn compare(actual:G2Affine,want:&ark_bn254::G2Affine){if want.is_zero(){assert!(actual.is_identity());}else{assert!(!actual.is_identity());assert_eq!(ark_fq(actual.x.c0),want.x.c0);assert_eq!(ark_fq(actual.x.c1),want.x.c1);assert_eq!(ark_fq(actual.y.c0),want.y.c0);assert_eq!(ark_fq(actual.y.c1),want.y.c1);}}

static COUNTS:[AtomicUsize;5]=[const {AtomicUsize::new(0)};5];
pub(crate) fn record(i:usize){COUNTS[i].fetch_add(1,Ordering::Relaxed);}
fn reset(){for c in &COUNTS{c.store(0,Ordering::Relaxed);}}
fn counts()->[usize;5]{core::array::from_fn(|i|COUNTS[i].load(Ordering::Relaxed))}
fn run(points:&[G2Affine],weights:&[Fr],kind:usize)->[usize;5]{
 let scalars=values(points.len(),kind);let want=expected(weights,&scalars);
 let before=points.to_vec();let mut words=scalars.iter().map(Fr::to_canonical).collect::<Vec<_>>();
 reset();let output=g2_query_exclusive(points,&mut words).to_affine();compare(output,&want);assert_eq!(points,before);counts()
}
#[test] fn raw_and_owned_composition(){
 let _env=Env::new(&[("ZOPRO_CPU_DEDUP","0"),("ZOPRO_CPU_PACKED","1"),("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_G2_PREFIX_RECORDS","1"),("ZOPRO_CPU_CANONICAL_BATCH","1"),("ZOPRO_CPU_VECTOR_INVERSE","1")]);
 let mut checked=0;let mut total=[0;5];
 for n in[0,7,1023,1024,1025,4096,30089,65535,65536]{let(p,w)=fixture(n);for kind in 0..4{let c=run(&p,&w,kind);for i in 0..5{total[i]+=c[i];}if (1024..65536).contains(&n){assert!(c[0]>0&&c[2]>0,"missing top/borrowed n={n}kind={kind} {c:?}");if kind==0{assert!(c[3]>0&&c[4]>0,"missing uniform/linear {c:?}");}}checked+=1;}}
 {let _owned=Env::new(&[("ZOPRO_CPU_G2_PREFIX_RECORDS","0")]);for n in[1024,4097]{let(p,w)=fixture(n);for kind in[0,2]{let c=run(&p,&w,kind);assert!(c[0]>0&&c[1]>0);assert_eq!(c[2],0);checked+=1;}}}
 let(p,_)=fixture(3);let mut words=vec![[0;4];2];let before=words.clone();assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||g2_query_exclusive(&p,&mut words))).is_err());assert_eq!(words,before);
 eprintln!("PREFIX_PHI_RAW_ARK_CASES {checked} ROUTES {total:?}");
}
#[test] fn grouped_and_denied_composition(){
 let _env=Env::new(&[("ZOPRO_CPU_DEDUP","1"),("ZOPRO_CPU_PACKED","1"),("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_G2_PREFIX_RECORDS","1"),("ZOPRO_CPU_CANONICAL_BATCH","1"),("ZOPRO_CPU_VECTOR_INVERSE","1")]);
 let mut checked=0;for n in[1024,4096,8192,65536]{let(p,w)=fixture(n);for kind in[0,4,5,6,7]{let words=values(n,kind).iter().map(Fr::to_canonical).collect::<Vec<_>>();let grouped=scalar_groups(&words);let admitted=(1024..65536).contains(&n)&&grouped.as_ref().map_or(true,|g|(1024..65536).contains(&g.values.len()));if n==8192&&[4,7].contains(&kind){assert!(grouped.is_some()&&admitted);}drop(grouped);let c=run(&p,&w,kind);assert_eq!(c[0]>0,admitted);if admitted&&kind==0{assert!(c[2]>0&&c[3]>0&&c[4]>0);}checked+=1;}}
 {let _deny=Env::new(&[("ZOPRO_CPU_PREPARE_MAX_BYTES","0")]);let(p,w)=fixture(8192);let words=values(8192,4).iter().map(Fr::to_canonical).collect::<Vec<_>>();assert!(scalar_groups(&words).is_none());let c=run(&p,&w,4);assert!(c[0]>0&&c[2]>0&&c[3]>0&&c[4]>0);checked+=1;}
 eprintln!("PREFIX_PHI_GROUPED_ADMISSION_ARK_CASES {checked}");
}
#[test] fn every_worker_nested_composition(){
 let _env=Env::new(&[("ZOPRO_CPU_DEDUP","1"),("ZOPRO_CPU_PACKED","1"),("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_G2_PREFIX_RECORDS","1"),("ZOPRO_CPU_CANONICAL_BATCH","1"),("ZOPRO_CPU_VECTOR_INVERSE","1")]);
 let(raw,rw)=fixture(1027);let(grouped,gw)=fixture(4096);let raw=Arc::new(raw);let grouped=Arc::new(grouped);
 let rs=values(raw.len(),0);let gs=values(grouped.len(),4);let rwords=rs.iter().map(Fr::to_canonical).collect::<Vec<_>>();let gwords=gs.iter().map(Fr::to_canonical).collect::<Vec<_>>();let re=expected(&rw,&rs);let ge=expected(&gw,&gs);let mut total=0;
 for workers in[1,4,16,32]{reset();let seen=Arc::new(Mutex::new(BTreeSet::new()));let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();pool.broadcast(|ctx|{seen.lock().unwrap().insert(ctx.index());rayon::join(||{let mut input=rwords.clone();compare(g2_query_exclusive(&raw,&mut input).to_affine(),&re);},||{let mut input=gwords.clone();compare(g2_query_exclusive(&grouped,&mut input).to_affine(),&ge);});});assert_eq!(seen.lock().unwrap().len(),workers);let c=counts();assert!(c[0]>0&&c[2]>0&&c[3]>0&&c[4]>0);assert_eq!(raw.as_ref(),&fixture(1027).0);assert_eq!(grouped.as_ref(),&fixture(4096).0);total+=workers;}
 assert_eq!(total,53);eprintln!("PREFIX_PHI_SMALL_STACK_WORKERS {total} NESTED_COMPLETE_QUERIES {}",total*2);
}
