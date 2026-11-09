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
    let diagnostic=diagnostic_new()?;
    for prime in 0..3 {
        std::hint::black_box(diagnostic_prove(&mut sdk,&request,&diagnostic,prime==0)?);
    }
    let diagnostic_priming=diagnostic_snapshot(&diagnostic)?;
    diagnostic_reset(&diagnostic)?;
    let mut samples = Vec::with_capacity(rounds);
    let mut last = None;
    if let Some(control) = profile.as_mut() {
        control.enable()?;
    }
    let before = perf::Usage::now()?;
    let loop_started = Instant::now();
    for _ in 0..rounds {
        let started = Instant::now();
        let proof = diagnostic_prove(&mut sdk,&request,&diagnostic,false)?;
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
    summary["query_diagnostic"] = diagnostic_snapshot(&diagnostic)?;
    summary["query_diagnostic_priming"] = diagnostic_priming;
    println!("SUMMARY {summary}");
    Ok(())
}

#[cfg(zopro_query_neutralization)]
type Diagnostic = Option<(Arc<zopro_prover::zopro_core::query_neutralization::Controller>,u8)>;
#[cfg(not(zopro_query_neutralization))]
type Diagnostic = ();
fn diagnostic_new() -> Result<Diagnostic,Box<dyn std::error::Error>> {
    #[cfg(zopro_query_neutralization)] {
        let mask=std::env::var("ZOPRO_QUERY_NEUTRALIZATION_MASK").ok().map(|v|v.parse::<u8>()).transpose()?;
        if mask.is_some_and(|v|v>7){return Err("diagnostic mask must be 0..7".into());}
        Ok(mask.map(|v|(zopro_prover::zopro_core::query_neutralization::Controller::new(),v)))
    }
    #[cfg(not(zopro_query_neutralization))] {
        if std::env::var_os("ZOPRO_QUERY_NEUTRALIZATION_MASK").is_some(){return Err("diagnostic unavailable in OFF".into());}Ok(())
    }
}
fn diagnostic_prove(sdk:&mut zopro_zolana::Prover,request:&str,diagnostic:&Diagnostic,learn:bool)->Result<zopro_zolana::ProofResult,Box<dyn std::error::Error>> {
    #[cfg(zopro_query_neutralization)] let _scope=diagnostic.as_ref().map(|(c,m)|c.enter(if learn {zopro_prover::zopro_core::query_neutralization::Mode::Learn}else{zopro_prover::zopro_core::query_neutralization::Mode::Replay(*m)})).transpose()?;
    #[cfg(not(zopro_query_neutralization))] let _=(diagnostic,learn);
    Ok(sdk.prove(request)?)
}
fn diagnostic_snapshot(diagnostic:&Diagnostic)->Result<serde_json::Value,Box<dyn std::error::Error>>{
    #[cfg(zopro_query_neutralization)] if let Some((c,mask))=diagnostic {
        let s=c.snapshot()?;
        if !s.cache_ready||s.faults!=0||s.declines!=0||s.misses!=[0;5]{return Err(format!("diagnostic coverage invalid: {s:?}").into());}
        return Ok(serde_json::json!({"diagnostic_only":true,"mask":mask,"requests":s.requests,"declines":s.declines,"faults":s.faults,"entry_hops":s.entry_hops,"calls":s.calls,"hits":s.hits,"computed":s.computed,"misses":s.misses,"ready":s.cache_ready,"topology":s.learned_topology,"path":s.learned_path_kind,"coordinate_bytes":zopro_prover::zopro_core::query_neutralization::Controller::coordinate_payload_bytes(),"controller_bytes":zopro_prover::zopro_core::query_neutralization::Controller::payload_bytes()}));
    }
    #[cfg(not(zopro_query_neutralization))] let _=diagnostic;
    Ok(serde_json::Value::Null)
}
fn diagnostic_reset(diagnostic:&Diagnostic)->Result<(),Box<dyn std::error::Error>>{
    #[cfg(zopro_query_neutralization)] if let Some((c,_))=diagnostic{c.reset_counts()?;}
    #[cfg(not(zopro_query_neutralization))] let _=diagnostic;
    Ok(())
}
