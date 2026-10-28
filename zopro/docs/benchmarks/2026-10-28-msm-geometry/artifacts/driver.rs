//! One observed proof after three unobserved primes; no latency measurement.
use std::{fs,sync::Arc};
#[global_allocator]
static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn main()->Result<(),Box<dyn std::error::Error>> {
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),4);
    let file=fs::File::open(&args[1])?;
    // SAFETY: the experiment pins this trusted immutable key image.
    let image=unsafe{memmap2::Mmap::map(&file)?};drop(file);
    let prover=zopro_prover::Prover::load_image(Arc::new(image))?;
    let mut sdk=zopro_zolana::Prover::new(prover)?;
    let request=fs::read_to_string(&args[2])?;
    for _ in 0..3 {std::hint::black_box(sdk.prove(&request)?);}
    zopro_prover::zopro_core::msm_geometry::begin();
    let proof=sdk.prove(&request)?;
    let counts=zopro_prover::zopro_core::msm_geometry::end();
    assert!(!counts.is_empty());
    let out=std::path::Path::new(&args[3]);fs::create_dir_all(out)?;
    fs::write(out.join("0.proof"),&proof.proof)?;fs::write(out.join("0.public"),&proof.public_inputs)?;
    println!("GEOMETRY {}",serde_json::to_string(&counts)?);
    println!("SUMMARY {}",serde_json::json!({"observed_proofs":1,"unobserved_primes":3,"SDK_requests":4,"workers":rayon::current_num_threads(),"internally_verified":true,"latency_measured":false}));
    Ok(())
}
