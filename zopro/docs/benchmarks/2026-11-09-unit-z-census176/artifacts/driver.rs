use std::{fs,path::Path,sync::Arc};
#[global_allocator]
static ALLOCATOR: zopro_prover::zopro_core::alloc::Allocator = zopro_prover::zopro_core::alloc::Allocator;
fn main()->Result<(),Box<dyn std::error::Error>> {
 let args=std::env::args_os().collect::<Vec<_>>();assert_eq!(args.len(),5,"census-sdk IMAGE REQUEST_JSON 1 PROOF_DIRECTORY");assert_eq!(args[3],"1");
 let before=placement_setup()?;
 let file=fs::File::open(&args[1])?;
 // SAFETY: the pinned trusted key image remains immutable throughout this process.
 let image=unsafe{memmap2::Mmap::map(&file)?};drop(file);
 let prover=zopro_prover::Prover::load_image(Arc::new(image))?;let mut sdk=zopro_zolana::Prover::new(prover)?;let request=fs::read_to_string(&args[2])?;
 for _ in 0..3{std::hint::black_box(sdk.prove(&request)?);}
 zopro_bn254::unit_z_census::begin();let proof=sdk.prove(&request)?;let counts=zopro_bn254::unit_z_census::finish();
 for row in counts{assert_eq!(row[0],row[1..].iter().sum::<u64>());}
 let allowed=before["main_allowed"].as_array().unwrap().iter().map(|v|v.as_u64().unwrap()as usize).collect::<Vec<_>>();
 let after=placement_snapshot("fixed",&allowed,before["workers"].as_u64().unwrap()as usize)?;assert_eq!(before,after);
 let dir=Path::new(&args[4]);fs::create_dir_all(dir)?;assert_eq!(proof.proof.len(),164);fs::write(dir.join("0.proof"),&proof.proof)?;fs::write(dir.join("0.public"),&proof.public_inputs)?;
 println!("CENSUS {}",serde_json::json!({"labels":zopro_bn254::unit_z_census::LABELS,"counts":counts,"priming":3,"counted_requests":1,"SDK_requests":4,"workers":rayon::current_num_threads(),"placement_before":before,"placement_after":after,"diagnostic_only":true,"latency_measured":false}));Ok(())
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
