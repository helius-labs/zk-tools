//! Collect finite reduction categories from one verified SDK request.
use std::{fs,path::Path,sync::Arc};
#[global_allocator]
static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn main()->Result<(),Box<dyn std::error::Error>> {
    let a:Vec<_>=std::env::args_os().collect();assert_eq!(a.len(),5);
    let f=fs::File::open(&a[1])?;
    // SAFETY: the pinned trusted key image is unchanged throughout this process.
    let image=unsafe{memmap2::Mmap::map(&f)?};drop(f);
    let p=zopro_prover::Prover::load_image(Arc::new(image))?;
    let mut sdk=zopro_zolana::Prover::new(p)?;let request=fs::read_to_string(&a[2])?;
    assert_eq!(a[3],"1");
    for _ in 0..3{std::hint::black_box(sdk.prove(&request)?);}
    zopro_prover::zopro_core::reduction_census::begin();
    let proof=sdk.prove(&request)?;
    let counters=zopro_prover::zopro_core::reduction_census::end();
    let directory=Path::new(&a[4]);fs::create_dir_all(directory)?;
    fs::write(directory.join("0.proof"),&proof.proof)?;fs::write(directory.join("0.public"),&proof.public_inputs)?;
    println!("CENSUS {}",serde_json::json!({"cells":counters,"priming":3,"observed":1,"workers":rayon::current_num_threads(),"timing_claim":false}));
    Ok(())
}
