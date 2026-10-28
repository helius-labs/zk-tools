use std::{fs, path::PathBuf, sync::Arc};
#[global_allocator]
static A: zopro_prover::zopro_core::alloc::Allocator = zopro_prover::zopro_core::alloc::Allocator;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 5);
    let f = fs::File::open(&a[1])?;
    // SAFETY: the host pins this immutable trusted image for the whole process.
    let image = unsafe { memmap2::Mmap::map(&f)? };
    drop(f);
    let prover = zopro_prover::Prover::load_image(Arc::new(image))?;
    let mut sdk = zopro_zolana::Prover::new(prover)?;
    let request = fs::read_to_string(&a[2])?;
    for _ in 0..3 {
        std::hint::black_box(sdk.prove(&request)?);
    }
    zopro_prover::zopro_core::euler_capture::begin(PathBuf::from(&a[4]));
    let proof = sdk.prove(&request)?;
    assert!(zopro_prover::zopro_core::euler_capture::captured());
    let out = PathBuf::from(&a[3]);
    fs::create_dir_all(&out)?;
    fs::write(out.join("0.proof"), &proof.proof)?;
    fs::write(out.join("0.public"), &proof.public_inputs)?;
    println!(
        "SUMMARY {}",
        serde_json::json!({"SDK_requests":4,"workers":rayon::current_num_threads(),"internally_verified":true,"timing_measured":false,"representation_captured":true})
    );
    Ok(())
}
