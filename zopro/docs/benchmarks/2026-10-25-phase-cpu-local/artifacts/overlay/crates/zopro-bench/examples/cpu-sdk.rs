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
    let phase_enabled = std::env::var("ZOPRO_BENCH_PHASE_CPU").as_deref() == Ok("1");
    let mut phase_snapshots = Vec::with_capacity(rounds);
    let mut last = None;
    if let Some(control) = profile.as_mut() {
        control.enable()?;
    }
    let before = perf::Usage::now()?;
    let loop_started = Instant::now();
    for _ in 0..rounds {
        let capture = zopro_prover::zopro_core::phase_cpu::Capture::start(phase_enabled)?;
        let started = Instant::now();
        zopro_prover::zopro_core::phase_cpu::mark("sdk_input");
        let proof = sdk.prove(&request)?;
        zopro_prover::zopro_core::phase_cpu::mark("done");
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        phase_snapshots.push(capture.finish()?);
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
    for (round, snapshot) in phase_snapshots.into_iter().enumerate() {
        let stamps = snapshot.stamps[..snapshot.len].iter().map(|stamp| {
            let stamp = stamp.unwrap();
            serde_json::json!({"phase":stamp.phase,"clock_before_ns":stamp.clock_before_ns,
                "clock_after_ns":stamp.clock_after_ns,"user_us":stamp.user_us,
                "system_us":stamp.system_us,"minor_faults":stamp.minor_faults,
                "major_faults":stamp.major_faults,"voluntary_switches":stamp.voluntary_switches,
                "involuntary_switches":stamp.involuntary_switches,"tid":stamp.tid})
        }).collect::<Vec<_>>();
        println!("PHASE_CPU {}", serde_json::json!({"round":round,"enabled":phase_enabled,"stamps":stamps}));
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
