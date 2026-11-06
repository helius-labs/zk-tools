use serde_json::{json,Value};use sha2::{Digest,Sha256};
use std::{alloc::{GlobalAlloc,Layout,System},fs,io::{Read,Write},sync::{Barrier,atomic::{AtomicBool,AtomicU64,Ordering}},time::Instant};
use zopro_bn254::{Fq,FqConfig,ifma8::Ifma8};
#[cfg(aos_gather)]use zopro_bn254::aos_gather::Gather;
static ACTIVE:AtomicBool=AtomicBool::new(false);static ALLOCS:AtomicU64=AtomicU64::new(0);
struct Alloc;unsafe impl GlobalAlloc for Alloc{unsafe fn alloc(&self,l:Layout)->*mut u8{if ACTIVE.load(Ordering::Relaxed){ALLOCS.fetch_add(1,Ordering::Relaxed);}unsafe{System.alloc(l)}}unsafe fn dealloc(&self,p:*mut u8,l:Layout){unsafe{System.dealloc(p,l)}}unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8{if ACTIVE.load(Ordering::Relaxed){ALLOCS.fetch_add(1,Ordering::Relaxed);}unsafe{System.realloc(p,l,n)}}unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8{if ACTIVE.load(Ordering::Relaxed){ALLOCS.fetch_add(1,Ordering::Relaxed);}unsafe{System.alloc_zeroed(l)}}}
#[global_allocator]static ALLOCATOR:Alloc=Alloc;
type V=Ifma8<FqConfig,true>;
#[cfg(aos_gather)]type Input<'a>=Gather<'a>;
#[cfg(not(aos_gather))]type Input<'a>=&'a[[u64;16]];
fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn affinity(cpu:usize){unsafe{let mut mask:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut mask);libc::CPU_SET(cpu,&mut mask);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask),0);assert_eq!(libc::sched_getcpu(),cpu as i32);}}
#[cfg(not(aos_gather))]#[inline(always)]fn pack<const FIELD:usize>(input:&Input<'_>,ids:[usize;8])->[V;2]{std::array::from_fn(|part|V::new(&std::array::from_fn(|lane|{let p=&input[ids[lane]];let offset=(FIELD+part)*4;Fq::from_montgomery(p[offset..offset+4].try_into().unwrap())})))}
#[cfg(aos_gather)]#[inline(always)]fn pack<const FIELD:usize>(input:&Input<'_>,ids:[usize;8])->[V;2]{input.pack::<FIELD>(ids)}
#[inline(never)]fn cycle(input:&Input<'_>,groups:&[[usize;8]],start:usize)->V{
    let mut acc=V::zero();
    for part in[&groups[start..],&groups[..start]]{for &ids in part{
        let x=pack::<0>(input,ids);let y=pack::<2>(input,ids);
        let nx=x[0].sum2(&x[0],&x[1],&x[1]);let ny=y[0].sum2(&y[0],&y[1],&y[1]);
        acc=acc.add(&nx).add(&ny);
    }}acc
}
struct Perf{ctl:fs::File,ack:fs::File}
impl Perf{fn open()->Option<Self>{let p=std::env::var("ZOPRO_BENCH_PERF_CTL").ok()?;Some(Self{ctl:fs::OpenOptions::new().write(true).open(p).unwrap(),ack:fs::File::open(std::env::var("ZOPRO_BENCH_PERF_ACK").unwrap()).unwrap()})}fn send(&mut self,b:&[u8]){self.ctl.write_all(b).unwrap();let mut byte=[0u8;1];loop{self.ack.read_exact(&mut byte).unwrap();if byte[0]==b'\n'{break;}}}}
fn main(){let args=std::env::args().collect::<Vec<_>>();assert_eq!(args.len(),6,"driver ROOT TRACE WORKERS ROUNDS ARM");let workers=args[3].parse::<usize>().unwrap();assert!([1,16,32].contains(&workers));let rounds=args[4].parse::<usize>().unwrap();assert!((1..=4096).contains(&rounds));let arm=&args[5];assert_eq!(cfg!(aos_gather),arm=="on");assert!(["frozen","off","on"].contains(&arm.as_str()));
let raw=fs::read(&args[1]).unwrap();assert_eq!(&raw[..8],b"B2ROOT01");let n=u64::from_le_bytes(raw[8..16].try_into().unwrap())as usize;assert_eq!(raw.len(),16+128*n);let meta:Value=serde_json::from_slice(&fs::read(format!("{}.json",args[1])).unwrap()).unwrap();assert_eq!(sha(&raw),meta["file_sha256"]);assert_eq!(meta["complete_MSM_matches_pinned_independent_Arkworks_oracle"],true);
let raw_trace=fs::read(&args[2]).unwrap();assert_eq!(sha(&raw_trace),meta["trace_sha256"]);let trace:Value=serde_json::from_slice(&raw_trace).unwrap();let mut groups=Vec::new();for row in trace["rows"].as_array().unwrap(){assert_eq!(row["root_points"],n);let lo=row["start"].as_u64().unwrap()as usize;let hi=row["end"].as_u64().unwrap()as usize;for component in row["batches"].as_array().unwrap(){for batch in component.as_array().unwrap(){for group in batch.as_array().unwrap().chunks_exact(8){let ids=std::array::from_fn(|lane|group[lane].as_u64().unwrap()as usize);assert!(ids.iter().all(|&i|lo<=i&&i<hi&&i<n));groups.push(ids);}}}}assert_eq!(groups.len(),meta["full_groups"]);let group_count=groups.len();drop(trace);drop(raw_trace);
let mut points=raw[16..].chunks_exact(128).map(|p|std::array::from_fn(|j|u64::from_le_bytes(p[j*8..j*8+8].try_into().unwrap()))).collect::<Vec<[u64;16]>>();for p in&points{for f in p.chunks_exact(4){assert!(Fq::from_canonical(f.try_into().unwrap()).is_some());}}drop(raw);let address=points.as_ptr();let capacity=points.capacity();
#[cfg(aos_gather)]let input=Gather::checked(&points).unwrap();
#[cfg(not(aos_gather))]let input=&points[..];
let expected:[Fq;8]=std::array::from_fn(|i|{let w:[u64;4]=serde_json::from_value(meta["scalar_norm_sum_raw_words"][i].clone()).unwrap();Fq::from_montgomery(w)});let expected_rounds=expected.map(|v|v*Fq::from(rounds as u64));let expected_digest=sha(&expected_rounds.iter().flat_map(|f|f.to_canonical().into_iter().flat_map(u64::to_le_bytes)).collect::<Vec<_>>());
let ready=Barrier::new(workers+1);let go=Barrier::new(workers+1);let done=Barrier::new(workers+1);let release=Barrier::new(workers+1);let mut perf=Perf::open();
let(wall_ns,rows)=std::thread::scope(|scope|{
 let mut handles=Vec::with_capacity(workers);for worker in 0..workers{let input=&input;let groups=&groups;let ready=&ready;let go=&go;let done=&done;let release=&release;handles.push(std::thread::Builder::new().stack_size(512*1024).spawn_scoped(scope,move||{
    let cpu=if workers==1{2}else{worker};affinity(cpu);let tid=unsafe{libc::syscall(libc::SYS_gettid)};
    for prime in 0..3{let got=cycle(input,groups,(worker*7919+prime*997)%groups.len()).get();assert_eq!(got,expected);}
    ready.wait();go.wait();let start=Instant::now();let mut acc=V::zero();for round in 0..rounds{let result=cycle(input,groups,(worker*7919+round*997)%groups.len());acc=acc.add(&std::hint::black_box(result));}let ns=start.elapsed().as_nanos()as u64;std::hint::black_box(&acc);done.wait();release.wait();assert_eq!(acc.get(),expected_rounds);assert_eq!(unsafe{libc::sched_getcpu()},cpu as i32);(worker,cpu,tid,ns)
 }).unwrap());}
 ready.wait();assert_eq!(ALLOCS.load(Ordering::Relaxed),0);if let Some(p)=perf.as_mut(){p.send(b"enable\n");}ACTIVE.store(true,Ordering::SeqCst);let start=Instant::now();go.wait();done.wait();let wall_ns=start.elapsed().as_nanos()as u64;ACTIVE.store(false,Ordering::SeqCst);if let Some(p)=perf.as_mut(){p.send(b"disable\n");}release.wait();let rows=handles.into_iter().map(|h|h.join().unwrap()).collect::<Vec<_>>();(wall_ns,rows)
});assert_eq!(ALLOCS.load(Ordering::Relaxed),0);drop(input);assert_eq!(points.as_ptr(),address);assert_eq!(points.capacity(),capacity);
let rows=rows.into_iter().map(|(worker,cpu,tid,ns)|json!({"worker":worker,"cpu":cpu,"tid":tid,"elapsed_ns":ns,"digest":expected_digest})).collect::<Vec<_>>();println!("RESULT {}",json!({"kind":"full-distribution-coordinate-codec-with-norm-sum","arm":arm,"direct_AoS_gather":cfg!(aos_gather),"workers":workers,"worker_rows":rows,"rounds":rounds,"prime_cycles_per_worker":3,"full_groups_per_cycle":group_count,"root_points":n,"root_sha256":meta["file_sha256"],"trace_sha256":meta["trace_sha256"],"wall_ns":wall_ns,"output_sha256":expected_digest,"independent_scalar_norm_sum_matches":true,"timed_allocations":0,"root_allocation_payload_bytes":points.len()*128,"root_allocation_capacity_bytes":capacity*128,"shared_group_indices_bytes":groups.len()*std::mem::size_of::<[usize;8]>(),"shared_group_indices_capacity_bytes":groups.capacity()*std::mem::size_of::<[usize;8]>(),"conversion_and_validation_before_timing":true,"new_proofs":0,"SDK_requests":0}));}
