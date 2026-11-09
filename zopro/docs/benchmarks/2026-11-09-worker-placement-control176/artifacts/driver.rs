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
    let placement_before=placement_setup()?;
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
    let loop_wall_ms = loop_started.elapsed().as_secs_f64() * 1000.;
    let after = perf::Usage::now()?;
    if let Some(control) = profile.as_mut() {
        control.disable()?;
    }
    let placement_after=placement_snapshot(placement_before["mode"].as_str().unwrap(), &placement_before["main_allowed"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize).collect::<Vec<_>>(),placement_before["workers"].as_u64().unwrap()as usize)?;
    assert_eq!(placement_before,placement_after);
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
    summary["placement_before"]=placement_before;
    summary["placement_after"]=placement_after;
    println!("SUMMARY {summary}");
    Ok(())
}
fn placement_setup()->Result<serde_json::Value,Box<dyn std::error::Error>>{
    let mode=std::env::var("ZOPRO_BENCH_WORKER_PLACEMENT")?;
    if mode!="unbound"&&mode!="fixed"{return Err("placement must be unbound or fixed".into());}
    let workers:usize=std::env::var("RAYON_NUM_THREADS")?.parse()?;
    let allowed=placement_affinity()?;
    if workers!=allowed.len(){return Err("one worker per inherited logical CPU is required".into());}
    let start_allowed=allowed.clone();let fixed=mode=="fixed";
    rayon::ThreadPoolBuilder::new().num_threads(workers).start_handler(move|i|{
        if fixed {placement_pin(start_allowed[i]).expect("worker affinity");}
    }).build_global()?;
    placement_snapshot(&mode,&allowed,workers)
}
fn placement_affinity()->Result<Vec<usize>,std::io::Error>{
    let mut set=std::mem::MaybeUninit::<libc::cpu_set_t>::zeroed();
    // SAFETY: getaffinity initializes a cpu_set_t-sized local set on success.
    if unsafe{libc::sched_getaffinity(0,std::mem::size_of::<libc::cpu_set_t>(),set.as_mut_ptr())}!=0{return Err(std::io::Error::last_os_error());}
    let set=unsafe{set.assume_init()};Ok((0..libc::CPU_SETSIZE as usize).filter(|&i|unsafe{libc::CPU_ISSET(i,&set)}).collect())
}
fn placement_pin(cpu:usize)->Result<(),std::io::Error>{
    let mut set=unsafe{std::mem::zeroed::<libc::cpu_set_t>()};
    // SAFETY: cpu came from getaffinity, inside CPU_SETSIZE; target is this thread only.
    unsafe{libc::CPU_ZERO(&mut set);libc::CPU_SET(cpu,&mut set);}
    if unsafe{libc::sched_setaffinity(0,std::mem::size_of::<libc::cpu_set_t>(),&set)}!=0{return Err(std::io::Error::last_os_error());}Ok(())
}
fn placement_snapshot(mode:&str,allowed:&[usize],workers:usize)->Result<serde_json::Value,Box<dyn std::error::Error>>{
    use std::collections::BTreeSet;
    let mut rows=rayon::broadcast(|ctx|{let worker=ctx.index();let tid=unsafe{libc::syscall(libc::SYS_gettid)};let mask=placement_affinity().expect("worker read affinity");assert_eq!(rayon::current_thread_index(),Some(worker));assert_eq!(mask,if mode=="fixed"{vec![allowed[worker]]}else{allowed.to_vec()});serde_json::json!({"index":worker,"tid":tid,"allowed":mask})});
    rows.sort_by_key(|v|v["index"].as_u64().unwrap());assert_eq!(rows.len(),workers);let tids=rows.iter().map(|r|r["tid"].as_u64().unwrap()).collect::<BTreeSet<_>>();assert_eq!(tids.len(),workers);let tasks=std::fs::read_dir("/proc/self/task")?.count();assert_eq!(tasks,workers+1);assert_eq!(placement_affinity()?,allowed);
    Ok(serde_json::json!({"mode":mode,"workers":workers,"main_allowed":allowed,"worker_rows":rows,"tasks":tasks}))
}
