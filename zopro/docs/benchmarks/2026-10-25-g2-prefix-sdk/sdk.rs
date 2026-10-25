fn current_affinity()->Vec<usize> {
    let mut mask:libc::cpu_set_t=unsafe{std::mem::zeroed()};
    assert_eq!(unsafe{libc::sched_getaffinity(0,std::mem::size_of_val(&mask),&mut mask)},0);
    (0..libc::CPU_SETSIZE as usize).filter(|&i|unsafe{libc::CPU_ISSET(i,&mask)}).collect()
}
fn worker_record(index:usize,pin:bool)->serde_json::Value {
    let allowed=current_affinity();
    let cpu=unsafe{libc::sched_getcpu()};assert!(cpu>=0);
    if pin {assert_eq!(allowed,vec![index]);assert_eq!(cpu as usize,index);}
    serde_json::json!({"index":index,"tid":unsafe{libc::syscall(libc::SYS_gettid)},"cpus":allowed,"observed_cpu":cpu})
}
fn init_workers() {
    if std::env::var("RAYON_NUM_THREADS").as_deref()!=Ok("16") {return;}
    let mut allowed:libc::cpu_set_t=unsafe{std::mem::zeroed()};
    unsafe{libc::CPU_ZERO(&mut allowed);for cpu in 0..16 {libc::CPU_SET(cpu,&mut allowed);}}
    assert!(current_affinity().iter().filter(|&&c|c<16).count()==16);
    assert_eq!(unsafe{libc::sched_setaffinity(0,std::mem::size_of_val(&allowed),&allowed)},0);

    let pin=true;
    let main=current_affinity();
    assert!(main==Vec::from_iter(0..16)||main==Vec::from_iter(0..32));
    let ready=Arc::new(std::sync::Barrier::new(16));
    let records=Arc::new(std::sync::Mutex::new(Vec::new()));
    let shared=records.clone();
    rayon::ThreadPoolBuilder::new().num_threads(16).start_handler(move|i| {
        if pin {
            assert!(current_affinity().contains(&i));
            let mut mask:libc::cpu_set_t=unsafe{std::mem::zeroed()};
            unsafe{libc::CPU_ZERO(&mut mask);libc::CPU_SET(i,&mut mask);}
            assert_eq!(unsafe{libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask)},0);
        }
        shared.lock().unwrap().push(worker_record(i,pin));
        ready.wait();
    }).build_global().unwrap();
    let mut receipt=rayon::broadcast(|context|worker_record(context.index(),pin));
    receipt.sort_by_key(|x|x["index"].as_u64().unwrap());
    let mut initial=records.lock().unwrap().clone();initial.sort_by_key(|x|x["index"].as_u64().unwrap());
    assert_eq!(initial.len(),16);assert_eq!(receipt.len(),16);
    for (a,b)in initial.iter().zip(&receipt){assert_eq!(a["index"],b["index"]);assert_eq!(a["tid"],b["tid"]);assert_eq!(a["cpus"],b["cpus"]);}
    println!("WORKERS {}",serde_json::json!({"phase":"before_key","pin":pin,"main_cpus":main,"workers":receipt}));
}
fn finish_workers() {
    if std::env::var("RAYON_NUM_THREADS").as_deref()!=Ok("16") {return;}

    let pin=true;
    let mut receipt=rayon::broadcast(|context|worker_record(context.index(),pin));
    receipt.sort_by_key(|x|x["index"].as_u64().unwrap());
    println!("WORKERS {}",serde_json::json!({"phase":"after_capture","pin":pin,"main_cpus":current_affinity(),"workers":receipt}));
}

use std::{fs,path::Path,sync::Arc,time::Instant};
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn main(){
 let args:Vec<_>=std::env::args().collect();let shape=&args[1];let rounds:usize=args[2].parse().unwrap();let blocks:usize=args[3].parse().unwrap();let modes:Vec<usize>=args[4].split(',').map(|x|x.parse().unwrap()).collect();let out=Path::new(&args[5]);assert_eq!(modes,vec![4]);
 let stem=if let Some(s)=shape.strip_prefix("merge-"){format!("merge_{}",s.replace('x',"_"))}else{format!("transfer_confidential_{}",shape.replace('x',"_"))};let fixture=if shape.starts_with("merge-"){shape.clone()}else{format!("transfer-{shape}")};
 init_workers();
 let f=fs::File::open(format!("{}/{stem}.image",std::env::var("ZOPRO_BASIS_KEY_DIR").unwrap_or_else(|_|"/tmp/zopro-keys".into()))).unwrap();let map=unsafe{memmap2::Mmap::map(&f)}.unwrap();drop(f);let mut sdk=zopro_zolana::Prover::new(zopro_prover::Prover::load_image(Arc::new(map)).unwrap()).unwrap();let request=fs::read_to_string(format!("/home/ubuntu/src/zk-tools/zopro/tools/zolana/fixtures/{fixture}.json")).unwrap();
 let mut samples=Vec::with_capacity(blocks*rounds*modes.len());let mut exports=Vec::with_capacity(blocks*modes.len());
 for block in 0..blocks{
  for _ in 0..3{for &mode in &modes{std::hint::black_box(sdk.prove(&request).unwrap());}}
  let mut last:Vec<Option<zopro_zolana::ProofResult>>=modes.iter().map(|_|None).collect();
  for rep in 0..rounds{
   let mut order:Vec<_>=(0..modes.len()).collect();order.rotate_left((rep+block)%modes.len());if(rep/modes.len()+block)%2==1{order.reverse();}
   for index in order{let mode=modes[index];let t=Instant::now();let p=sdk.prove(&request).unwrap();let ms=t.elapsed().as_secs_f64()*1000.;assert_eq!(p.proof.len(),164);assert_eq!(p.public_inputs.len(),44);samples.push((block,rep,mode,ms));last[index]=Some(std::hint::black_box(p));}
  }
  for (index,p)in last.into_iter().enumerate(){exports.push((block,modes[index],p.unwrap()));}
 }
 finish_workers();
 let mut usage=unsafe{std::mem::zeroed::<libc::rusage>()};assert_eq!(unsafe{libc::getrusage(libc::RUSAGE_SELF,&mut usage)},0);
 for(block,mode,p)in exports{let dest=out.join(format!("b{block}-m{mode}")).join(&stem);fs::create_dir_all(&dest).unwrap();fs::write(dest.join("0.proof"),p.proof).unwrap();fs::write(dest.join("0.public"),p.public_inputs).unwrap();}
 for(block,rep,mode,ms)in samples{println!("SAMPLE {}",serde_json::json!({"block":block,"rep":rep,"mode":mode,"ms":ms}));}
 println!("SUMMARY {}",serde_json::json!({"shape":shape,"rounds":rounds,"blocks":blocks,"prime_per_mode_block":3,"modes":modes,"process_peak_rss_kib":usage.ru_maxrss}));
}
