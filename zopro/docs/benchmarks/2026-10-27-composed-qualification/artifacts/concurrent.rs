use std::{fs,path::Path,sync::{Arc,Barrier}};
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
fn tids()->Vec<u32>{let mut v:Vec<_>=fs::read_dir("/proc/self/task").unwrap().map(|e|e.unwrap().file_name().to_str().unwrap().parse().unwrap()).collect();v.sort_unstable();v}
fn main(){
 let a:Vec<_>=std::env::args().collect();let shape=&a[1];let callers:usize=a[2].parse().unwrap();let rounds:usize=a[3].parse().unwrap();let out=Path::new(&a[4]);fs::create_dir_all(out).unwrap();let n:usize=std::env::var("RAYON_NUM_THREADS").unwrap().parse().unwrap();
 let stem=if let Some(v)=shape.strip_prefix("merge-"){format!("merge_{}",v.replace('x',"_"))}else{format!("transfer_confidential_{}",shape.replace('x',"_"))};let fixture=if shape.starts_with("merge-"){shape.to_owned()}else{format!("transfer-{shape}")};
 let f=fs::File::open(format!("{}/{stem}.image",std::env::var("ZOPRO_BASIS_KEY_DIR").unwrap())).unwrap();let map=Arc::new(unsafe{memmap2::Mmap::map(&f)}.unwrap());drop(f);let request=fs::read_to_string(format!("/home/ubuntu/src/zk-tools/zopro/tools/zolana/fixtures/{fixture}.json")).unwrap();
 let loaded=Barrier::new(callers+1);let go=Barrier::new(callers+1);
 std::thread::scope(|scope|{
  let mut jobs=Vec::new();
  for caller in 0..callers{
   let image=map.clone();let request=&request;let stem=&stem;let loaded=&loaded;let go=&go;
   jobs.push(scope.spawn(move||{
    let mut sdk=zopro_zolana::Prover::new(zopro_prover::Prover::load_image(image).unwrap()).unwrap();loaded.wait();go.wait();
    for round in 0..rounds{let p=sdk.prove(request).unwrap();assert_eq!(p.proof.len(),164);assert_eq!(p.public_inputs.len(),44);let dir=out.join(stem);fs::create_dir_all(&dir).unwrap();fs::write(dir.join(format!("t{caller}-r{round}.proof")),p.proof).unwrap();fs::write(dir.join(format!("t{caller}-r{round}.public")),p.public_inputs).unwrap();}
   }));
  }
  loaded.wait();let active=tids();assert_eq!(active.len(),n+callers+1,"one existing global pool plus actual callers");println!("THREADS {}",serde_json::json!({"phase":"loaded","workers":n,"callers":callers,"tids":active}));go.wait();
  for job in jobs{job.join().unwrap();}
 });
 let idle=tids();assert_eq!(idle.len(),n+1);println!("COMPLETE {}",serde_json::json!({"workers":n,"callers":callers,"rounds":rounds,"proofs":callers*rounds,"idle_tids":idle}));
}
