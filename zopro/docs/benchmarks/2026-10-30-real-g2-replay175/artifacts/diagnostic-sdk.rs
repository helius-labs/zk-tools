//! Measure verified SDK requests against an existing CPU key image.
use std::{fs, path::Path, sync::Arc, time::Instant};
#[path = "support/perf.rs"]
mod perf;

#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator =
    zopro_prover::zopro_core::alloc::Allocator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<_> = std::env::args_os().collect();
    if !(4..=5).contains(&args.len()) {
        return Err("usage: cpu-sdk IMAGE REQUEST_JSON ROUNDS [PROOF_DIRECTORY]".into());
    }
    let rounds: usize = args[3].to_str().ok_or("rounds must be UTF-8")?.parse()?;
    if rounds == 0 {
        return Err("rounds must be positive".into());
    }
    let mut profile = perf::Control::from_env()?;
    let file = fs::File::open(&args[1])?;
    // SAFETY: the caller keeps this trusted key image unchanged for the run.
    let image = unsafe { memmap2::Mmap::map(&file)? };
    drop(file);
    let prover = zopro_prover::Prover::load_image(Arc::new(image))?;
    let mut sdk = zopro_zolana::Prover::new(prover)?;
    let request = fs::read_to_string(&args[2])?;
    for _ in 0..3 {
        std::hint::black_box(sdk.prove(&request)?);
    }
    let mut samples = Vec::with_capacity(rounds);
    let mut last = None;
    if let Some(control) = profile.as_mut() {
        control.enable()?;
    }
    let before = perf::Usage::now()?;
    let loop_started = Instant::now();
    for _ in 0..rounds {
        let started = Instant::now();
        let proof = sdk.prove(&request)?;
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        last = Some(proof);
    }
    zopro_prover::zopro_core::g2_replay::finish_capture();
    let loop_wall_ms = loop_started.elapsed().as_secs_f64() * 1000.;
    let after = perf::Usage::now()?;
    if let Some(control) = profile.as_mut() {
        control.disable()?;
    }
    if let Some(directory) = args.get(4) {
        let directory = Path::new(directory);
        fs::create_dir_all(directory)?;
        let proof = last.as_ref().unwrap();
        fs::write(directory.join("0.proof"), &proof.proof)?;
        fs::write(directory.join("0.public"), &proof.public_inputs)?;
    }
    for (round, ms) in samples.into_iter().enumerate() {
        println!("SAMPLE {}", serde_json::json!({"round": round, "ms": ms}));
    }
    let mut summary = after.difference(&before);
    summary["rounds"] = rounds.into();
    summary["priming"] = 3.into();
    summary["workers"] = rayon::current_num_threads().into();
    summary["loop_wall_ms"] = loop_wall_ms.into();
    println!("SUMMARY {summary}");
    Ok(())
}
