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
        return Err("usage: cpu-stream IMAGE SEQUENCE_JSON ROUNDS [PROOF_DIRECTORY]".into());
    }
    let rounds: usize = args[3].to_str().ok_or("rounds must be UTF-8")?.parse()?;
    if rounds == 0 {
        return Err("rounds must be positive".into());
    }
    let mut profile = perf::Control::from_env()?;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let frontend = placement::configure()?;
    let file = fs::File::open(&args[1])?;
    // SAFETY: the caller keeps this trusted key image unchanged for the run.
    let image = unsafe { memmap2::Mmap::map(&file)? };
    drop(file);
    let prover = zopro_prover::Prover::load_image(Arc::new(image))?;
    #[cfg(all(target_os = "linux", target_arch = "x86_64"))]
    let prover = {
        let mut prover = prover;
        prover.set_frontend_pool(frontend);
        match std::env::var("ZOPRO_BENCH_PUBLIC_OVERLAP").as_deref() {
            Ok("early") => prover.set_overlap_verification(true),
            Ok("off") => prover.set_overlap_verification(false),
            Err(std::env::VarError::NotPresent) => {}
            _ => return Err("public verification overlap must be early or off".into()),
        }
        prover
    };
    let mut prover = prover;
    if let Some(path) = std::env::var_os("ZOPRO_BENCH_WIRE_FILE") {
        let file = fs::File::open(path)?;
        // The benchmark owns the immutable input during preparation.
        let bytes = unsafe { memmap2::Mmap::map(&file)? };
        let start = Instant::now();
        let enabled = prover.prepare_public_wire_offsets(&bytes)?;
        println!("WIRE_FILE {} {}", enabled, start.elapsed().as_nanos());
    }
    if let Some(file) = std::env::var_os("ZOPRO_TRIAL_GROUPS") {
        let data=fs::read(file)?; let start=Instant::now();
        let enabled=prover.prepare_group_trial(&data)?;
        println!("GROUP_FILE {} {}",enabled,start.elapsed().as_nanos());
    }
    if let Some(dir)=std::env::var_os("ZOPRO_LINEAR_GROUPS"){
        let dir=std::path::Path::new(&dir);
        let a=fs::read(dir.join("a.bin"))?;let k=fs::read(dir.join("k.bin"))?;
        let start=Instant::now();let ready=prover.prepare_linear_trials(&a,&k)?;
        println!("LINEAR_FILE {} {}",ready,start.elapsed().as_nanos());
    }
    let mut sdk = zopro_zolana::Prover::new(prover)?;
    let paths: Vec<String> = serde_json::from_reader(fs::File::open(&args[2])?)?;
    if paths.len() != 3 + rounds {
        return Err("sequence must contain three warmups and one path per measured round".into());
    }
    let directory = Path::new(args.get(4).ok_or("proof directory is required")?);
    fs::create_dir_all(directory)?;
    let mut requests = std::collections::BTreeMap::new();
    for path in &paths {
        if !requests.contains_key(path) {
            requests.insert(path.clone(), fs::read_to_string(path)?);
        }
    }
    let sequence: Vec<&str> = paths.iter().map(|path| requests[path].as_str()).collect();
    for request in &sequence[..3] {
        std::hint::black_box(sdk.prove(request)?);
    }
    let mut samples = Vec::with_capacity(rounds);
    if let Some(control) = profile.as_mut() {
        control.enable()?;
    }
    let before = perf::Usage::now()?;
    let loop_started = Instant::now();
    for request in &sequence[3..] {
        let start = Instant::now();
        let proof = sdk.prove(request)?;
        let elapsed = start.elapsed();
        samples.push((elapsed, proof));
    }
    let loop_wall_ms = loop_started.elapsed().as_secs_f64() * 1000.;
    let after = perf::Usage::now()?;
    if let Some(control) = profile.as_mut() {
        control.disable()?;
    }
    for (round, (elapsed, proof)) in samples.into_iter().enumerate() {
        fs::write(directory.join(format!("{round}.proof")), &proof.proof)?;
        fs::write(directory.join(format!("{round}.public")), &proof.public_inputs)?;
        let ms = elapsed.as_secs_f64() * 1000.;
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
