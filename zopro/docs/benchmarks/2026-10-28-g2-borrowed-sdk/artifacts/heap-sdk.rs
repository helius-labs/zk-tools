//! Measure verified SDK requests against an existing CPU key image.
use std::{fs, path::Path, sync::Arc, time::Instant};
#[path = "support/perf.rs"]
mod perf;

use std::alloc::{GlobalAlloc,Layout};
struct Alloc;
#[cfg(zopro_queue_heap_probe)]
mod heap {
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    pub static LIVE: AtomicUsize = AtomicUsize::new(0);
    pub static PEAK: AtomicUsize = AtomicUsize::new(0);
    pub static CALLS: AtomicUsize = AtomicUsize::new(0);
    pub static BYTES: AtomicUsize = AtomicUsize::new(0);
    pub fn add(n: usize) {
        CALLS.fetch_add(1, Relaxed);
        BYTES.fetch_add(n, Relaxed);
        let v = LIVE.fetch_add(n, Relaxed) + n;
        PEAK.fetch_max(v, Relaxed);
    }
    pub fn remove(n: usize) {
        LIVE.fetch_sub(n, Relaxed);
    }
    pub fn reset() -> usize {
        let v = LIVE.load(Relaxed);
        PEAK.store(v, Relaxed);
        CALLS.store(0, Relaxed);
        BYTES.store(0, Relaxed);
        v
    }
    pub fn report(before: usize) -> serde_json::Value {
        let after = LIVE.load(Relaxed);
        let peak = PEAK.load(Relaxed);
        let calls = CALLS.load(Relaxed);
        let bytes = BYTES.load(Relaxed);
        serde_json::json!({"live_before":before,"live_after":after,"peak_live":peak,"peak_extra":peak.saturating_sub(before),"allocation_requests":calls,"requested_bytes":bytes})
    }
}
unsafe impl GlobalAlloc for Alloc {
    unsafe fn alloc(&self, l: Layout) -> *mut u8 {
        let p = unsafe { zopro_core::alloc::Allocator.alloc(l) };
        #[cfg(zopro_queue_heap_probe)]
        if !p.is_null() {
            heap::add(l.size())
        }
        p
    }
    unsafe fn alloc_zeroed(&self, l: Layout) -> *mut u8 {
        let p = unsafe { zopro_core::alloc::Allocator.alloc_zeroed(l) };
        #[cfg(zopro_queue_heap_probe)]
        if !p.is_null() {
            heap::add(l.size())
        }
        p
    }
    unsafe fn dealloc(&self, p: *mut u8, l: Layout) {
        #[cfg(zopro_queue_heap_probe)]
        heap::remove(l.size());
        unsafe { zopro_core::alloc::Allocator.dealloc(p, l) }
    }
    unsafe fn realloc(&self, p: *mut u8, l: Layout, n: usize) -> *mut u8 {
        let q = unsafe { zopro_core::alloc::Allocator.realloc(p, l, n) };
        #[cfg(zopro_queue_heap_probe)]
        if !q.is_null() {
            heap::remove(l.size());
            heap::add(n)
        }
        q
    }
}
#[global_allocator]
static ALLOC: Alloc = Alloc;


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
    let heap_before=heap::reset();
    let loop_started = Instant::now();
    for _ in 0..rounds {
        let started = Instant::now();
        let proof = sdk.prove(&request)?;
        samples.push(started.elapsed().as_secs_f64() * 1000.0);
        last = Some(proof);
    }
    let loop_wall_ms = loop_started.elapsed().as_secs_f64() * 1000.;
    let memory=heap::report(heap_before);
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
    summary["heap"]=memory;
    summary["heap_instrumented"]=true.into();
    println!("SUMMARY {summary}");
    Ok(())
}
