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
    let front = frontend_width();
    let cpus = placement_affinity()?;
    let frontend = if front > 0 {
        let ids = cpus.clone();
        let pool = Arc::new(rayon::ThreadPoolBuilder::new().num_threads(front)
            .start_handler(move |i| placement_pin(ids[i]).expect("frontend worker affinity")).build()?);
        let pins = pool.broadcast(|ctx| {
            let allowed = placement_affinity().unwrap();
            assert_eq!(allowed, vec![cpus[ctx.index()]]);
            (ctx.index(), allowed[0])
        });
        println!("FRONT {}", serde_json::to_string(&pins)?);
        Some(pool)
    } else { None };
    let placement_before=placement_setup()?;
    let file = fs::File::open(&args[1])?;
    let image = Arc::new(unsafe { memmap2::Mmap::map(&file)? });
    let request = fs::read_to_string(&args[2])?;
    let directory = Path::new(&args[4]);
    fs::create_dir_all(directory)?;
    let mut clients = Vec::new();
    for _ in 0..2 {
        let mut prover = zopro_prover::Prover::load_image(image.clone())?;
        prover.set_frontend_pool(frontend.clone());
        let mut sdk = zopro_zolana::Prover::new(prover)?;
        for _ in 0..3 { std::hint::black_box(sdk.prove(&request)?); }
        clients.push(sdk);
    }
    let barrier = std::sync::Barrier::new(3);
    let finished = std::sync::Barrier::new(3);
    std::thread::scope(|scope| {
        let mut handles = Vec::new();
        for (i, mut sdk) in clients.into_iter().enumerate() {
            let (start,end,req) = (&barrier,&finished,&request);
            handles.push(scope.spawn(move || {
                start.wait();
                let t = Instant::now();
                let mut samples=Vec::new(); let mut last=None;
                for _ in 0..rounds {
                    let begin = Instant::now();
                    last=Some(sdk.prove(req).unwrap());
                    samples.push(begin.elapsed().as_secs_f64()*1000.0);
                }
                let elapsed=t.elapsed().as_secs_f64()*1000.0;
                end.wait();
                let proof=last.unwrap();
                fs::write(directory.join(format!("{i}.proof")),proof.proof).unwrap();
                fs::write(directory.join(format!("{i}.public")),proof.public_inputs).unwrap();
                (samples, elapsed)
            }));
        }
        let before=perf::Usage::now().unwrap();
        let t=Instant::now();
        barrier.wait();
        finished.wait();
        let wall=t.elapsed().as_secs_f64()*1000.0;
        let usage=perf::Usage::now().unwrap().difference(&before);
        let data:Vec<_>=handles.into_iter().map(|h|h.join().unwrap()).collect();
        println!("CLIENTS {}",serde_json::json!({"wall_ms":wall,"clients":data,"usage":usage,"front_workers":front,"rounds":rounds}));
    });
    let after=placement_snapshot(placement_before["mode"].as_str().unwrap(),&cpus,cpus.len())?;
    assert_eq!(placement_before,after);
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
    rows.sort_by_key(|v|v["index"].as_u64().unwrap());assert_eq!(rows.len(),workers);let tids=rows.iter().map(|r|r["tid"].as_u64().unwrap()).collect::<BTreeSet<_>>();assert_eq!(tids.len(),workers);let tasks=std::fs::read_dir("/proc/self/task")?.count();assert_eq!(tasks,workers+frontend_width()+1);assert_eq!(placement_affinity()?,allowed);
    Ok(serde_json::json!({"mode":mode,"workers":workers,"main_allowed":allowed,"worker_rows":rows,"tasks":tasks}))
}

fn frontend_width() -> usize {
    let main: usize = std::env::var("RAYON_NUM_THREADS").unwrap().parse().unwrap();
    let front: usize = std::env::var("ZOPRO_BENCH_SHARED_FRONTEND").unwrap_or_default().parse().unwrap_or(0);
    if front < main { front } else { 0 }
}
