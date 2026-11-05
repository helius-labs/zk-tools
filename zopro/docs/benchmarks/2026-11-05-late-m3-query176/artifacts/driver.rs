use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,sync::{Arc,Mutex},time::Instant};
use zopro_bn254::{Fq,Fq2,Fr,G1Affine,G2Affine};
#[path="perf_support.rs"]mod perf;
#[global_allocator]static ALLOCATOR:zopro_core::alloc::Allocator=zopro_core::alloc::Allocator;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn affinity(cpu:usize){unsafe{let mut mask:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut mask);libc::CPU_SET(cpu,&mut mask);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask),0);assert_eq!(libc::sched_getcpu(),cpu as i32);}}
fn point_hash(fields:impl IntoIterator<Item=Fq>)->String{let mut h=Sha256::new();for f in fields{for w in f.to_canonical(){h.update(w.to_le_bytes())}}format!("{:x}",h.finalize())}
fn main(){
 let a=std::env::args().collect::<Vec<_>>();assert_eq!(a.len(),6,"driver QUERY FIXTURE ORACLE WORKERS ROUNDS");let query=&a[1];assert!(["A","B"].contains(&query.as_str()));let workers=a[4].parse::<usize>().unwrap();let rounds=a[5].parse::<usize>().unwrap();assert!([1,16,32].contains(&workers)&&rounds>0);
 let bytes=fs::read(&a[2]).unwrap();let oracle:Value=serde_json::from_slice(&fs::read(&a[3]).unwrap()).unwrap();assert_eq!(hash(&bytes),oracle["fixture_sha256"]);assert_eq!(oracle["independent_Arkworks_complete_MSM"],true);let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(n,oracle["points"]);
 let mut g1=vec![];let mut g2=vec![];let point_size=if query=="A"{64}else{128};assert_eq!(bytes.len(),16+(point_size+32)*n);
 if query=="A"{assert_eq!(&bytes[..8],b"ACOLMSM1");for p in bytes[16..16+64*n].chunks_exact(64){let point=if p.iter().all(|x|*x==0){G1Affine::IDENTITY}else{G1Affine::new(Fq::from_be_bytes(p[..32].try_into().unwrap()).unwrap(),Fq::from_be_bytes(p[32..].try_into().unwrap()).unwrap())};assert!(point.is_on_curve());g1.push(point)}}
 else{assert_eq!(&bytes[..8],b"B2SCAN01");for p in bytes[16..16+128*n].chunks_exact(128){let f=p.chunks_exact(32).map(|b|Fq::from_be_bytes(b.try_into().unwrap()).unwrap()).collect::<Vec<_>>();let point=G2Affine::new(Fq2::new(f[1],f[0]),Fq2::new(f[3],f[2]));assert!(point.is_on_curve());g2.push(point)}}
 let original=bytes[16+point_size*n..].chunks_exact(32).map(|v|{let w=std::array::from_fn(|i|u64::from_le_bytes(v[i*8..i*8+8].try_into().unwrap()));assert!(Fr::from_canonical(w).is_some());w}).collect::<Vec<[u64;4]>>();drop(bytes);
 let inputs_digest=||{let mut h=Sha256::new();for p in &g1{for f in[p.x,p.y]{for w in f.0{h.update(w.to_le_bytes())}}}for p in &g2{for f in[p.x.c0,p.x.c1,p.y.c0,p.y.c1]{for w in f.0{h.update(w.to_le_bytes())}}}format!("{:x}",h.finalize())};let before_points=inputs_digest();
 let observed=Arc::new(Mutex::new(vec![]));let observed_start=observed.clone();let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).start_handler(move|i|{let cpu=if workers==1{2}else{i};affinity(cpu);observed_start.lock().unwrap().push(json!({"worker":i,"cpu":cpu,"tid":unsafe{libc::syscall(libc::SYS_gettid)}}));}).build().unwrap();pool.broadcast(|_|());let mut worker_rows=observed.lock().unwrap().clone();worker_rows.sort_by_key(|v|v["worker"].as_u64().unwrap());assert_eq!(worker_rows.len(),workers);
 let mut words=original.clone();let mut expected_metadata=None;
 let mut run=||{
  if query=="A"{let(p,m)=pool.install(||zopro_core::coalesce_query_probe::a(std::hint::black_box(&g1),std::hint::black_box(&original)));let p=p.to_affine();([p.x,p.y,Fq::ZERO,Fq::ZERO],m)}
  else{words.copy_from_slice(std::hint::black_box(&original));let(p,m)=pool.install(||zopro_core::coalesce_query_probe::b(std::hint::black_box(&g2),std::hint::black_box(&mut words)));let p=p.to_affine();([p.x.c0,p.x.c1,p.y.c0,p.y.c1],m)}
 };
 let check=|fields:[Fq;4]|{let fields=if query=="A"{&fields[..2]}else{&fields[..]};assert_eq!(point_hash(fields.iter().copied()),oracle["expected_canonical_result_sha256"]);};
 for _ in 0..3{let(p,m)=run();check(p);if let Some(old)=expected_metadata{assert_eq!(old,m)}else{expected_metadata=Some(m)}}
 let mut outputs=Vec::with_capacity(rounds);let mut ns=Vec::with_capacity(rounds);let mut control=perf::Control::from_env().unwrap();if let Some(c)=control.as_mut(){c.enable().unwrap()}
 let before=perf::Usage::now().unwrap();let start=Instant::now();for _ in 0..rounds{let clock=Instant::now();let out=std::hint::black_box(run());ns.push(clock.elapsed().as_nanos()as u64);outputs.push(out);}let elapsed=start.elapsed().as_nanos()as u64;let after=perf::Usage::now().unwrap();if let Some(c)=control.as_mut(){c.disable().unwrap()}
 for(p,m)in outputs{check(p);assert_eq!(expected_metadata.unwrap(),m);}let(p,m)=run();check(p);assert_eq!(expected_metadata.unwrap(),m);pool.broadcast(|ctx|assert_eq!(unsafe{libc::sched_getcpu()},if workers==1{2}else{ctx.index()}as i32));assert_eq!(before_points,inputs_digest());
 let meta=expected_metadata.unwrap();let receipt=json!({"query":query,"input_rows":n,"workers":workers,"pid":std::process::id(),"rounds":rounds,"priming_passes":3,"postcheck_passes":1,"all_timed_outputs_checked":true,"every_output_matches_independent_full_MSM":true,"point_inputs_unchanged":true,"worker_rows":worker_rows,"metadata":{"route":meta.route,"classes":meta.classes,"skips":meta.skips,"retained_payload_bytes":meta.payload},"elapsed_ns":elapsed,"ns_per_query":elapsed as f64/rounds as f64,"samples_ns":ns,"B_word_reset_inside_timer":query=="B","normalization_inside_timer":true,"usage":after.difference(&before),"new_proofs":0});println!("CALLER {receipt}");
}
