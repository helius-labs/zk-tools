//! Proof-only PGO training, no performance claim.
use std::{fs, hint::black_box, path::Path, sync::Arc};
#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =
    zopro_prover::zopro_core::alloc::Allocator;
extern "C" {
    fn __llvm_profile_reset_counters();
    fn __llvm_profile_dump() -> std::ffi::c_int;
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    assert_eq!(args.len(), 4);
    let out = Path::new(&args[3]);
    assert!(!out.exists());
    let file = fs::File::open(&args[1])?;
    // SAFETY: pinned trusted image remains immutable for this process.
    let image = unsafe { memmap2::Mmap::map(&file)? };
    drop(file);
    let prover = zopro_prover::Prover::load_image(Arc::new(image))?;
    let mut sdk = zopro_zolana::Prover::new(prover)?;
    let request = fs::read_to_string(&args[2])?;
    for _ in 0..3 {
        black_box(sdk.prove(&request)?);
    }
    let mut proofs = Vec::with_capacity(11);
    // SAFETY: profile runtime only; all synchronous priming proofs have joined.
    // SDK loading/proving counters are audited independently from saved profiles.
    unsafe {
        __llvm_profile_reset_counters();
    }
    for _ in 0..11 {
        proofs.push(black_box(sdk.prove(&request)?));
    }
    // SAFETY: use matching LLVM runtime; dump once before exports/log formatting.
    // The runtime marks dumped, preventing an atexit replacement of this snapshot.
    assert_eq!(unsafe { __llvm_profile_dump() }, 0);
    fs::create_dir_all(out)?;
    for (i, p) in proofs.into_iter().enumerate() {
        assert_eq!(p.proof.len(), 164);
        fs::write(out.join(format!("{i}.proof")), p.proof)?;
        fs::write(out.join(format!("{i}.public")), p.public_inputs)?;
    }
    println!(
        "TRAINING {}",
        serde_json::json!({"SDK_requests":14,"priming":3,"proofs_after_reset":11,"exported":11,"dump_before_exports":true,"workers":rayon::current_num_threads()})
    );
    Ok(())
}
