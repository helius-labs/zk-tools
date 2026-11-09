use super::*;
use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};
use ark_ff::{BigInt,PrimeField};
use std::{collections::BTreeSet,ffi::OsString,sync::{Mutex,atomic::Ordering}};
struct Env(Vec<(&'static str,Option<OsString>)>);
impl Env{fn new(values:&[(&'static str,&str)])->Self{let old=values.iter().map(|(k,v)|{let old=std::env::var_os(k);std::env::set_var(k,v);(*k,old)}).collect();Self(old)}}
impl Drop for Env{fn drop(&mut self){for(k,v)in self.0.iter().rev(){if let Some(v)=v{std::env::set_var(k,v)}else{std::env::remove_var(k)}}}}
fn fixture(n:usize)->(Vec<G2Affine>,Vec<Fr>){let mut point=G2Projective::IDENTITY;let mut projectives=Vec::with_capacity(n);let mut weights=Vec::with_capacity(n);for i in 0..n{point+=G2Affine::GENERATOR;let (p,w)=match i%13{0=>(G2Projective::IDENTITY,Fr::ZERO),1=>(G2Affine::GENERATOR.to_projective(),Fr::ONE),2=>(-G2Affine::GENERATOR.to_projective(),-Fr::ONE),3=>(G2Affine::GENERATOR.to_projective(),Fr::ONE),4=>(-point,-Fr::from(i as u64+1)),_=>(point,Fr::from(i as u64+1))};projectives.push(p);weights.push(w);} (G2Projective::batch_to_affine(&projectives),weights)}
fn values(n:usize,kind:usize)->Vec<Fr>{let mut state=Fr::from(813491u64).pow(&[119]);(0..n).map(|i|{state=state*Fr::from(6364136223846793005u64)+Fr::from(i as u64+71);match kind{0=>state,1=>if i%5==0{Fr::ZERO}else{state},2=>Fr::ONE,3=>{let bit=[0,11,12,63,64,125,126,127,128,253][i%10];let p=Fr::from(2).pow(&[bit]);if i%2==0{p-Fr::ONE}else{-p-Fr::ONE}},4=>Fr::from((i%2048+1)as u64),5=>Fr::from((i%23+1)as u64),6=>Fr::ZERO,7=>Fr::from((i%1024+1)as u64),_=>unreachable!()}}).collect()}
fn ark_fr(value:Fr)->ark_bn254::Fr{ark_bn254::Fr::from_bigint(BigInt(value.to_canonical())).unwrap()}
fn ark_fq(value:Fq)->ark_bn254::Fq{ark_bn254::Fq::from_bigint(BigInt(value.to_canonical())).unwrap()}
fn expected(weights:&[Fr],scalars:&[Fr])->ark_bn254::G2Affine{let sum=weights.iter().zip(scalars).fold(ark_bn254::Fr::from(0u64),|sum,(w,s)|sum+ark_fr(*w)*ark_fr(*s));(ark_bn254::G2Projective::generator()*sum).into_affine()}
fn compare(actual:G2Affine,want:&ark_bn254::G2Affine){if want.is_zero(){assert!(actual.is_identity());}else{assert!(!actual.is_identity());assert_eq!(ark_fq(actual.x.c0),want.x.c0);assert_eq!(ark_fq(actual.x.c1),want.x.c1);assert_eq!(ark_fq(actual.y.c0),want.y.c0);assert_eq!(ark_fq(actual.y.c1),want.y.c1);}}

fn mode(paired: bool, bundled: bool) -> Env {
    Env::new(&[("ZOPRO_CPU_G2_PAIRED_TOP", if paired {"1"} else {"0"}),
        ("ZOPRO_CPU_INDEXED_TILE_BUNDLES", if bundled {"1"} else {"0"})])
}
fn query_check(points: &[G2Affine], weights: &[Fr], kind: usize) {
    let scalars = values(points.len(), kind);
    let want = expected(weights, &scalars);
    let mut words = scalars.iter().map(Fr::to_canonical).collect::<Vec<_>>();
    let before = points.to_vec();
    let got = crate::msm_affine::g2_query_exclusive(points, &mut words).to_affine();
    compare(got, &want);
    assert_eq!(points, before);
}
#[test]
fn all_four_modes_raw_boundaries_match_arkworks() {
    let _env = Env::new(&[("ZOPRO_CPU_DEDUP","0"),("ZOPRO_CPU_PACKED","1"),
        ("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_G2_PREFIX_RECORDS","1")]);
    let mut cases = 0;
    for n in [0,7,1023,1024,1025,4097,30089,65535,65536] {
        let (points, weights) = fixture(n);
        for paired in [false,true] { for bundled in [false,true] {
            let _mode = mode(paired,bundled);
            for kind in [0,3,6] {
                let before = ADMITTED.load(Ordering::Relaxed);
                let before_parts = PAIRED_PARTS.load(Ordering::Relaxed);
                query_check(&points,&weights,kind);
                let admitted = paired && (1024..65536).contains(&n);
                assert_eq!(ADMITTED.load(Ordering::Relaxed)-before,usize::from(admitted));
                assert_eq!(PAIRED_PARTS.load(Ordering::Relaxed)-before_parts,2*usize::from(admitted));
                cases += 1;
            }
        }}
    }
    eprintln!("PAIRED_TOP_RAW_ARK_CASES {cases}");
}
#[test]
fn grouped_queries_all_modes_match_arkworks() {
    let _env = Env::new(&[("ZOPRO_CPU_DEDUP","1"),("ZOPRO_CPU_PACKED","1"),
        ("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_G2_PREFIX_RECORDS","1")]);
    let mut cases = 0;
    for n in [4096,8192] {
        let (points,weights) = fixture(n);
        for paired in [false,true] { for bundled in [false,true] {
            let _mode = mode(paired,bundled);
            for kind in [0,4,5,6,7] {
                let words = values(n,kind).iter().map(Fr::to_canonical).collect::<Vec<_>>();
                let groups = crate::msm_affine::scalar_groups(&words);
                let admitted = paired && groups.as_ref().map_or(true,|g|(1024..65536).contains(&g.values.len()));
                drop(groups);
                let before = ADMITTED.load(Ordering::Relaxed);
                query_check(&points,&weights,kind);
                assert_eq!(ADMITTED.load(Ordering::Relaxed)-before,usize::from(admitted));
                cases += 1;
            }
        }}
    }
    eprintln!("PAIRED_TOP_GROUPED_ARK_CASES {cases}");
}
#[test]
fn checked_parent_forecast_denial_and_length_leave_words_unchanged() {
    let cost = crate::msm_affine::workspace::paired_top_extra;
    for n in [0,7,1023,65536,usize::MAX] {assert!(cost(n,2).is_none());}
    for parts in [0,1,3,usize::MAX] {assert!(cost(1024,parts).is_none());}
    let mut last = 0;
    for n in 1024..65536 {
        let bytes = cost(n,2).unwrap();
        assert!(bytes>=last);
        last=bytes;
    }
    eprintln!("PAIRED_TOP_PARENT_FORECAST min={} primary={} max={} bytes; not actual allocations/RSS",
        cost(1024,2).unwrap(),cost(30089,2).unwrap(),cost(65535,2).unwrap());
    let _env = Env::new(&[("ZOPRO_CPU_PREPARE_MAX_BYTES","0"),("ZOPRO_CPU_DEDUP","0"),
        ("ZOPRO_CPU_G2_PAIRED_TOP","1"),("ZOPRO_CPU_INDEXED_TILE_BUNDLES","1")]);
    let(points,weights)=fixture(1025);
    let mut words=values(points.len(),0).iter().map(Fr::to_canonical).collect::<Vec<_>>();
    let before=words.clone();
    assert!(try_raw(&points,&mut words,12,2).is_none());
    assert_eq!(words,before);
    let count=ADMITTED.load(Ordering::Relaxed);
    query_check(&points,&weights,0);
    assert_eq!(ADMITTED.load(Ordering::Relaxed),count);
    let mut wrong=vec![[0u64;4];2];let saved=wrong.clone();
    assert!(std::panic::catch_unwind(std::panic::AssertUnwindSafe(||try_raw(&points,&mut wrong,12,2))).is_err());
    assert_eq!(wrong,saved);
}
#[test]
fn every_worker_runs_nested_four_mode_queries_on_small_stacks() {
    let _env = Env::new(&[("ZOPRO_CPU_DEDUP","1"),("ZOPRO_CPU_PACKED","1"),
        ("ZOPRO_CPU_DIRECT","1"),("ZOPRO_CPU_G2_PREFIX_RECORDS","1")]);
    let (raw,raw_weights)=fixture(1027);let(grouped,group_weights)=fixture(4096);
    let raw_words=values(raw.len(),0).iter().map(Fr::to_canonical).collect::<Vec<_>>();
    let grouped_words=values(grouped.len(),4).iter().map(Fr::to_canonical).collect::<Vec<_>>();
    let raw_want=expected(&raw_weights,&values(raw.len(),0));
    let group_want=expected(&group_weights,&values(grouped.len(),4));
    let mut workers_total=0;let mut queries=0;
    for workers in [1,4,16,32] {
        let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
        for paired in [false,true] {for bundled in [false,true] {
            let _mode=mode(paired,bundled);let seen=Mutex::new(BTreeSet::new());
            pool.broadcast(|ctx| {
                seen.lock().unwrap().insert(ctx.index());
                for (points,words,want) in [(&raw,&raw_words,&raw_want),(&grouped,&grouped_words,&group_want)] {
                    let mut words=words.clone();
                    compare(crate::msm_affine::g2_query_exclusive(points,&mut words).to_affine(),want);
                }
            });
            assert_eq!(seen.lock().unwrap().len(),workers);queries+=2*workers;
        }}
        workers_total+=workers;
    }
    assert_eq!(workers_total,53);assert_eq!(queries,424);
    eprintln!("PAIRED_TOP_NESTED_WORKERS {workers_total} COMPLETE_QUERIES {queries}");
}
