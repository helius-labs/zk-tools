//! Measure verified SDK requests against an existing CPU key image.
use std::{fs, path::Path, sync::Arc, time::Instant};
#[path = "support/perf.rs"]
mod perf;
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
#[path = "support/placement.rs"]
mod placement;

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
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let exclusive_g2 = match std::env::var("ZOPRO_BENCH_EXCLUSIVE_G2_SCHEDULE").as_deref() {
        Ok("early") => Some(true),
        Ok("late") => Some(false),
        Err(std::env::VarError::NotPresent) => None,
        _ => return Err("exclusive G2 schedule must be early or late".into()),
    };
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let frontend = placement::configure(exclusive_g2.is_some())?;
    let file = fs::File::open(&args[1])?;
    // SAFETY: the caller keeps this trusted key image unchanged for the run.
    let image = unsafe { memmap2::Mmap::map(&file)? };
    drop(file);
    let prover = zopro_prover::Prover::load_image(Arc::new(image))?;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let prover = {
        let mut prover = prover;
        if let Some(overlap) = exclusive_g2 {
            prover.set_exclusive_g2_pool(frontend.clone(), overlap);
        }
        prover.set_frontend_pool(frontend);
        prover.set_solver_pool(placement::configure_solver()?);
        if std::env::var("ZOPRO_BENCH_GROUPED_POINT_TILING").as_deref() == Ok("1") {
            prover.set_grouped_point_tiling(true);
        }
        match std::env::var("ZOPRO_BENCH_GROUPED_G2_FRONTEND").as_deref() {
            Ok("1") => prover.set_grouped_g2_on_frontend(true),
            Ok("0") | Err(std::env::VarError::NotPresent) => {}
            _ => return Err("grouped G2 frontend must be 0 or 1".into()),
        }
        match std::env::var("ZOPRO_BENCH_RAW_G1_A_FRONTEND").as_deref() {
            Ok("1") => prover.set_raw_g1_a_on_frontend(true),
            Ok("0") | Err(std::env::VarError::NotPresent) => {}
            _ => return Err("raw G1 A frontend must be 0 or 1".into()),
        }
        match std::env::var("ZOPRO_BENCH_G1_K_FRONTEND").as_deref() {
            Ok("1") => prover.set_g1_k_on_frontend(true),
            Ok("0") | Err(std::env::VarError::NotPresent) => {}
            _ => return Err("G1 K frontend must be 0 or 1".into()),
        }
        match std::env::var("ZOPRO_BENCH_GROUPED_B2_SERIAL").as_deref() {
            Ok("1") => prover.set_grouped_b2_serial_preparation(true),
            Ok("0") | Err(std::env::VarError::NotPresent) => {}
            _ => return Err("grouped B2 serial preparation must be 0 or 1".into()),
        }
        match std::env::var("ZOPRO_BENCH_GROUPED_B2_ORIGINS").as_deref() {
            Ok("1") => prover.set_grouped_b2_poseidon_origins(true),
            Ok("0") | Err(std::env::VarError::NotPresent) => {}
            _ => return Err("grouped B2 Poseidon origins must be 0 or 1".into()),
        }
        match std::env::var("ZOPRO_BENCH_EXCLUSIVE_G2_GROUPING").as_deref() {
            Ok("0") => prover.set_exclusive_g2_grouping(false),
            Ok("1") | Err(std::env::VarError::NotPresent) => {}
            _ => return Err("exclusive G2 grouping must be 0 or 1".into()),
        }
        match std::env::var("ZOPRO_BENCH_PUBLIC_OVERLAP").as_deref() {
            Ok("early") => prover.set_overlap_verification(true),
            Ok("off") => prover.set_overlap_verification(false),
            Err(std::env::VarError::NotPresent) => {}
            _ => return Err("public verification overlap must be early or off".into()),
        }
        prover
    };
    let mut prover = prover;
    match std::env::var("ZOPRO_BENCH_EULER_FFT_LEAF_FUSION").as_deref() {
        Ok("1") => prover.set_euler_fft_leaf_fusion(true),
        Ok("0") | Err(std::env::VarError::NotPresent) => {}
        _ => return Err("Euler FFT leaf fusion must be 0 or 1".into()),
    }
    match std::env::var("ZOPRO_BENCH_PACKED_FINAL_EXPONENTIATION").as_deref() {
        Ok("1") => prover.set_packed_final_exponentiation(true),
        Ok("0") | Err(std::env::VarError::NotPresent) => {}
        _ => return Err("packed final exponentiation must be 0 or 1".into()),
    }
    if let Some(path) = std::env::var_os("ZOPRO_BENCH_WIRE_FILE") {
        let file = fs::File::open(path)?;
        // The benchmark owns the immutable input during preparation.
        let bytes = unsafe { memmap2::Mmap::map(&file)? };
        let start = Instant::now();
        let enabled = prover.prepare_public_wire_offsets(&bytes)?;
        println!("WIRE_FILE {} {}", enabled, start.elapsed().as_nanos());
    }
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
