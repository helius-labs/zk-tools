//! Full raw-G2 caller component, with prepared fixture inputs and allocation accounting.
use std::{alloc::{GlobalAlloc,Layout},fs,sync::{Arc,atomic::{AtomicUsize,Ordering::Relaxed}},time::Instant};
use serde_json::{json,Value};
use zopro_bn254::{Fr,G2Affine};
use zopro_prover::{Prover,zopro_core::{self,artifact::Artifact}};
#[path="support/perf.rs"]mod perf;
static LIVE:AtomicUsize=AtomicUsize::new(0);
static PEAK:AtomicUsize=AtomicUsize::new(0);
static ALLOCS:AtomicUsize=AtomicUsize::new(0);
struct Tracked;
unsafe impl GlobalAlloc for Tracked{
    unsafe fn alloc(&self,l:Layout)->*mut u8{let p=unsafe{zopro_core::alloc::Allocator.alloc(l)};if !p.is_null(){let n=LIVE.fetch_add(l.size(),Relaxed)+l.size();PEAK.fetch_max(n,Relaxed);ALLOCS.fetch_add(1,Relaxed);}p}
    unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{let p=unsafe{zopro_core::alloc::Allocator.alloc_zeroed(l)};if !p.is_null(){let n=LIVE.fetch_add(l.size(),Relaxed)+l.size();PEAK.fetch_max(n,Relaxed);ALLOCS.fetch_add(1,Relaxed);}p}
    unsafe fn dealloc(&self,p:*mut u8,l:Layout){LIVE.fetch_sub(l.size(),Relaxed);unsafe{zopro_core::alloc::Allocator.dealloc(p,l)}}
    unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{let out=unsafe{zopro_core::alloc::Allocator.realloc(p,l,n)};if !out.is_null(){let total=if n>=l.size(){LIVE.fetch_add(n-l.size(),Relaxed)+n-l.size()}else{LIVE.fetch_sub(l.size()-n,Relaxed)-(l.size()-n)};PEAK.fetch_max(total,Relaxed);ALLOCS.fetch_add(1,Relaxed);}out}
}
#[global_allocator]static ALLOCATOR:Tracked=Tracked;
fn u64_at(b:&[u8],at:usize)->usize{u64::from_le_bytes(b[at..at+8].try_into().unwrap()).try_into().unwrap()}
fn inputs(c:&Value)->(Vec<G2Affine>,Vec<[u64;4]>){
    let bytes=fs::read(c["key"].as_str().unwrap()).unwrap();
    let request=fs::read_to_string(c["request"].as_str().unwrap()).unwrap();
    let sdk=zopro_zolana::Prover::load(&bytes).unwrap();
    let witness=sdk.witness(&request).unwrap();let image=sdk.solver_image().unwrap();drop(sdk);
    let artifact=Artifact::parse(&bytes).unwrap();
    let key=artifact.key(artifact.points(),Some(&image)).unwrap();
    let witness:Vec<u8>=witness.iter().flat_map(|v|v.0.iter().flat_map(|w|w.to_le_bytes())).collect();
    let values=key.msm_scalars(&witness).unwrap();drop(key);
    let words=values[1].iter().map(Fr::to_canonical).collect::<Vec<_>>();drop(values);
    let file=fs::File::open(c["image"].as_str().unwrap()).unwrap();
    let mapped=Arc::new(unsafe{memmap2::Mmap::map(&file).unwrap()});
    let loaded=Prover::load_image(mapped.clone()).unwrap();drop(loaded);
    let raw:&[u8]=&mapped;
    assert_eq!(&raw[..16],b"zopro-euler-v2\0\0");
    let head=u64_at(raw,16);let (header,end)=Artifact::head(&raw[24..24+head]).unwrap();assert_eq!(end,head);
    assert_eq!(header.points,artifact.points);
    let at=(24+head).div_ceil(64)*64;
    let slots=u64_at(raw,at);let [a,b,k,z,b2]=header.points.map(|n|n as usize);
    let start=(at+128).checked_add((a+b+k+slots+z+1).checked_mul(64).unwrap()).unwrap();
    let points=zopro_core::decode_g2(&raw[start..start+b2*128]);
    assert_eq!(points.len(),words.len());assert!((1024..65536).contains(&points.len()));
    for (p,encoded) in points.iter().zip(artifact.b2.chunks_exact(64)){
        assert!(p.is_on_curve());assert_eq!(zopro_core::verify::compress_g2(p),encoded);
    }
    (points,words)
}
fn query(bases:&[G2Affine],words:&mut[[u64;4]],mode:&str)->Result<(zopro_bn254::G2Projective,usize),String>{
    match mode{"stream"=>zopro_core::g2_level_tree_probe(bases,words,false),"tree"=>zopro_core::g2_level_tree_probe(bases,words,true),"prefix"=>zopro_core::g2_prefix_tree_probe(bases,words),_=>Err("mode".into())}
}
fn main()->Result<(),Box<dyn std::error::Error>>{
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),6);
    let cases:Value=serde_json::from_slice(&fs::read(&args[1])?)?;
    let case=cases.as_array().unwrap().iter().find(|c|c["shape"]==args[2]).unwrap();
    let mode=args[3].as_str();assert!(["stream","tree","prefix"].contains(&mode));
    let workers:usize=args[4].parse()?;let rounds:usize=args[5].parse()?;assert!(rounds>0);
    let mut control=perf::Control::from_env()?;
    let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).build()?;
    pool.install(||->Result<(),String>{
        assert_eq!(rayon::current_num_threads(),workers);
        let (bases,words)=inputs(case);
        let expected=zopro_core::g2_level_tree_probe(&bases,&mut words.clone(),false)?.0.to_affine();
        for _ in 0..2{assert_eq!(query(&bases,&mut words.clone(),mode)?.0.to_affine(),expected);}
        let mut samples=Vec::with_capacity(rounds);let mut peak_deltas=Vec::with_capacity(rounds);let mut allocs=Vec::with_capacity(rounds);let mut forecast=0;
        if let Some(c)=control.as_mut(){c.enable().map_err(|e|e.to_string())?;}
        let usage0=perf::Usage::now().map_err(|e|e.to_string())?;
        for _ in 0..rounds{
            let before=LIVE.load(Relaxed);PEAK.store(before,Relaxed);let a0=ALLOCS.load(Relaxed);
            let start=Instant::now();
            let (out,bytes)={let mut scalars=words.clone();query(&bases,&mut scalars,mode)?};
            forecast=bytes;assert_eq!(out.to_affine(),expected);
            samples.push(start.elapsed().as_secs_f64()*1000.);
            peak_deltas.push(PEAK.load(Relaxed).saturating_sub(before));allocs.push(ALLOCS.load(Relaxed)-a0);
        }
        let usage1=perf::Usage::now().map_err(|e|e.to_string())?;
        if let Some(c)=control.as_mut(){c.disable().map_err(|e|e.to_string())?;}
        for(i,ms)in samples.iter().enumerate(){println!("SAMPLE {}",json!({"round":i,"ms":ms,"peak_live_requested_above_inputs_bytes":peak_deltas[i],"allocation_reallocation_calls":allocs[i]}));}
        let mut summary=usage1.difference(&usage0);summary["shape"]=case["shape"].clone();summary["mode"]=json!(args[3]);summary["rounds"]=json!(rounds);summary["priming"]=json!(2);summary["reference_setup_calls"]=json!(1);summary["workers"]=json!(workers);summary["points"]=json!(bases.len());summary["forecast_requested_bytes"]=json!(forecast);summary["output_sha_hint"]=json!(format!("{:x?}",zopro_core::verify::compress_g2(&expected)));println!("SUMMARY {summary}");Ok(())
    }).map_err(std::io::Error::other)?;
    Ok(())
}
