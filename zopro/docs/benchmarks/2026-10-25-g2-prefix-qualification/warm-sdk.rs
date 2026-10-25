#[cfg(zopro_worker_groups)]
type RequestExecutor=zopro_prover::zopro_core::cpu_executor::CpuExecutor;
#[cfg(not(zopro_worker_groups))]
struct RequestExecutor;
fn request_proof(executor:Option<&RequestExecutor>,sdk:&mut zopro_zolana::Prover,req:&str)->zopro_zolana::ProofResult {
 #[cfg(zopro_worker_groups)]if let Some(e)=executor{return e.run(||sdk.prove(req).unwrap());}
 sdk.prove(req).unwrap()
}
use std::{fs::{self,OpenOptions},path::Path,sync::Arc,io::{Write,BufRead,BufReader},time::Instant};
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn control(command:&str,out:&mut fs::File,input:&mut BufReader<fs::File>){out.write_all(command.as_bytes()).unwrap();out.write_all(b"\n").unwrap();out.flush().unwrap();let mut reply=String::new();input.read_line(&mut reply).unwrap();assert_eq!(reply.trim_matches(|c:char|c.is_whitespace()||c==char::from(0)),"ack");}
fn thread_ids()->Vec<u32> {
 let mut ids:Vec<_>=fs::read_dir("/proc/self/task").unwrap().map(|e|e.unwrap().file_name().to_str().unwrap().parse().unwrap()).collect();ids.sort_unstable();ids
}
fn prove_main(executor:Option<&RequestExecutor>){
 let args:Vec<_>=std::env::args().collect();let shape=&args[1];let rounds:usize=args[2].parse().unwrap();let output=&args[3];
 let perf=std::env::var_os("CROSS_PERF_CTL").is_some();
 let mut ctl=OpenOptions::new().read(true).write(true).open(&args[4]).unwrap();let mut ack=BufReader::new(OpenOptions::new().read(true).write(true).open(&args[5]).unwrap());
 let stem=if let Some(rest)=shape.strip_prefix("merge-"){format!("merge_{}",rest.replace('x',"_"))}else{format!("transfer_confidential_{}",shape.replace('x',"_"))};let fixture=if shape.starts_with("merge-"){shape.to_owned()}else{format!("transfer-{shape}")};
 let file=fs::File::open(format!("{}/{stem}.image",std::env::var("ZOPRO_BASIS_KEY_DIR").unwrap())).unwrap();let map=unsafe{memmap2::Mmap::map(&file)}.unwrap();drop(file);let load=||zopro_zolana::Prover::new(zopro_prover::Prover::load_image(Arc::new(map)).unwrap()).unwrap();
 #[cfg(zopro_worker_groups)]let mut sdk=match executor{Some(e)=>e.run(load),None=>load()};
 #[cfg(not(zopro_worker_groups))]let mut sdk=load();let request=fs::read_to_string(format!("/home/ubuntu/src/zk-tools/zopro/tools/zolana/fixtures/{fixture}.json")).unwrap();
 for _ in 0..3{std::hint::black_box(request_proof(executor,&mut sdk,&request));}
 let pause=std::env::var_os("ZOPRO_PRESSURE_PAUSE");
 if let Some(path)=pause {
  let path=Path::new(&path);fs::write(path.join("ready"),b"ready").unwrap();
  let started=Instant::now();while !path.join("go").exists(){assert!(started.elapsed().as_secs()<40);std::thread::sleep(std::time::Duration::from_millis(2));}
 }
 let before_tids=thread_ids();
 if perf{control("enable",&mut ctl,&mut ack);}
 let mut before=unsafe{std::mem::zeroed::<libc::rusage>()};let mut after=unsafe{std::mem::zeroed::<libc::rusage>()};assert_eq!(unsafe{libc::getrusage(libc::RUSAGE_SELF,&mut before)},0);
 let loop_started=Instant::now();let mut samples=Vec::with_capacity(rounds);let mut last=None;
 for _ in 0..rounds{let started=Instant::now();let p=request_proof(executor,&mut sdk,&request);let ms=started.elapsed().as_secs_f64()*1e3;assert_eq!(p.proof.len(),164);assert_eq!(p.public_inputs.len(),44);samples.push(ms);last=Some(std::hint::black_box(p));}
 let wall=loop_started.elapsed().as_secs_f64()*1e3;assert_eq!(unsafe{libc::getrusage(libc::RUSAGE_SELF,&mut after)},0);if perf{control("disable",&mut ctl,&mut ack);}
 let after_tids=thread_ids();assert_eq!(before_tids,after_tids);let expected:usize=std::env::var("RAYON_NUM_THREADS").unwrap().parse().unwrap();assert_eq!(after_tids.len(),expected+1,"existing pool only");println!("THREADS {}",serde_json::json!({"before":before_tids,"after":after_tids,"workers":expected,"total":after_tids.len()}));
 let p=last.unwrap();let out=Path::new(output).join(&stem);fs::create_dir_all(&out).unwrap();fs::write(out.join("0.proof"),p.proof).unwrap();fs::write(out.join("0.public"),p.public_inputs).unwrap();for (round,ms) in samples.iter().enumerate(){println!("SAMPLE {}",serde_json::json!({"round":round,"verified_ms":ms}));}samples.sort_by(f64::total_cmp);let tv=|v:libc::timeval|v.tv_sec as f64*1e3+v.tv_usec as f64/1e3;
 println!("SUMMARY {}",serde_json::json!({"mode":"zopro","shape":shape,"rounds":rounds,"priming":3,"verified_ms_median":samples[rounds/2],"loop_wall_ms":wall,"user_cpu_ms":tv(after.ru_utime)-tv(before.ru_utime),"system_cpu_ms":tv(after.ru_stime)-tv(before.ru_stime),"voluntary_switches":after.ru_nvcsw-before.ru_nvcsw,"involuntary_switches":after.ru_nivcsw-before.ru_nivcsw,"minor_faults":after.ru_minflt-before.ru_minflt,"whole_process_maxrss_kib":after.ru_maxrss}));
}

#[cfg(zopro_worker_groups)]
fn executor_receipt(e:&zopro_prover::zopro_core::cpu_executor::CpuExecutor,phase:&str){
    let workers=e.worker_receipts();let sizes=e.worker_sizes();
    assert_eq!(workers.len(),sizes.iter().sum::<usize>());
    assert_eq!(workers.iter().map(|x|&x.2).collect::<std::collections::HashSet<_>>().len(),workers.len());
    let named=std::fs::read_dir("/proc/self/task").unwrap().filter_map(|x|x.ok()).filter_map(|x|std::fs::read_to_string(x.path().join("comm")).ok()).filter(|x|x.starts_with("zopro-wg-")).count();
    assert_eq!(named,workers.len());
    let total=std::fs::read_dir("/proc/self/task").unwrap().count();
    assert_eq!(total,workers.len()+1,"no additional global or hidden worker pool");
    let mut u=unsafe{std::mem::zeroed::<libc::rusage>()};assert_eq!(unsafe{libc::getrusage(libc::RUSAGE_SELF,&mut u)},0);
    println!("EXECUTOR {}",serde_json::json!({"phase":phase,"sizes":sizes,"workers":workers,"named_workers":named,"total_process_threads":total,"whole_process_maxrss_kib":u.ru_maxrss}));
}

fn main(){
 #[cfg(zopro_worker_groups)]if let Ok(mode)=std::env::var("ZOPRO_CPU_WORKER_GROUPS") {
  if mode!="outside" {
   let n:usize=std::env::var("RAYON_NUM_THREADS").unwrap().parse().unwrap();
   let kind=mode.strip_suffix("-request").unwrap_or(&mode);
   let e=match kind{"single"=>RequestExecutor::single(n),"balanced"=>RequestExecutor::balanced(n),_=>panic!("unsupported executor mode")}.unwrap();
   executor_receipt(&e,"before_key");
   if mode.ends_with("-request"){prove_main(Some(&e));}else{e.run(||prove_main(None));}
   executor_receipt(&e,"after_requests");return;
  }
 }
 prove_main(None);
}
