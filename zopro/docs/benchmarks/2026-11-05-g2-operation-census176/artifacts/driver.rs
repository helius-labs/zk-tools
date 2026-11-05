use serde_json::{json,Value};use sha2::{Digest,Sha256};use std::{fs,sync::{Arc,Mutex},time::Instant};
use zopro_bn254::{Fq,Fq2,Fr,G1Affine,G2Affine};
#[path="perf_support.rs"]mod perf;
#[global_allocator]static ALLOCATOR:zopro_core::alloc::Allocator=zopro_core::alloc::Allocator;
fn hash(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
fn affinity(cpu:usize){unsafe{let mut mask:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut mask);libc::CPU_SET(cpu,&mut mask);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&mask),&mask),0);assert_eq!(libc::sched_getcpu(),cpu as i32);}}
fn point_hash(fields:impl IntoIterator<Item=Fq>)->String{let mut h=Sha256::new();for f in fields{for w in f.to_canonical(){h.update(w.to_le_bytes())}}format!("{:x}",h.finalize())}
fn main(){
 let a=std::env::args().collect::<Vec<_>>();assert_eq!(a.len(),6,"driver QUERY FIXTURE ORACLE WORKERS ROUNDS");let query=&a[1];assert_eq!(query,"B");let workers=a[4].parse::<usize>().unwrap();let rounds=a[5].parse::<usize>().unwrap();assert!([1,32].contains(&workers));assert_eq!(rounds,1);
 let bytes=fs::read(&a[2]).unwrap();let oracle:Value=serde_json::from_slice(&fs::read(&a[3]).unwrap()).unwrap();assert_eq!(hash(&bytes),oracle["fixture_sha256"]);assert_eq!(oracle["independent_Arkworks_complete_MSM"],true);let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(n,oracle["points"]);
 let mut g1=vec![];let mut g2=vec![];let point_size=if query=="A"{64}else{128};assert_eq!(bytes.len(),16+(point_size+32)*n);
 if query=="A"{assert_eq!(&bytes[..8],b"ACOLMSM1");for p in bytes[16..16+64*n].chunks_exact(64){let point=if p.iter().all(|x|*x==0){G1Affine::IDENTITY}else{G1Affine::new(Fq::from_be_bytes(p[..32].try_into().unwrap()).unwrap(),Fq::from_be_bytes(p[32..].try_into().unwrap()).unwrap())};assert!(point.is_on_curve());g1.push(point)}}
 else{assert_eq!(&bytes[..8],b"B2SCAN01");for p in bytes[16..16+128*n].chunks_exact(128){let f=p.chunks_exact(32).map(|b|Fq::from_be_bytes(b.try_into().unwrap()).unwrap()).collect::<Vec<_>>();let point=G2Affine::new(Fq2::new(f[1],f[0]),Fq2::new(f[3],f[2]));assert!(point.is_on_curve());g2.push(point)}}
 let original=bytes[16+point_size*n..].chunks_exact(32).map(|v|{let w=std::array::from_fn(|i|u64::from_le_bytes(v[i*8..i*8+8].try_into().unwrap()));assert!(Fr::from_canonical(w).is_some());w}).collect::<Vec<[u64;4]>>();drop(bytes);
 let inputs_digest=||{let mut h=Sha256::new();for p in &g1{for f in[p.x,p.y]{for w in f.0{h.update(w.to_le_bytes())}}}for p in &g2{for f in[p.x.c0,p.x.c1,p.y.c0,p.y.c1]{for w in f.0{h.update(w.to_le_bytes())}}}format!("{:x}",h.finalize())};let before_points=inputs_digest();
 let observed=Arc::new(Mutex::new(vec![]));let observed_start=observed.clone();let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).start_handler(move|i|{let cpu=if workers==1{2}else{i};affinity(cpu);observed_start.lock().unwrap().push(json!({"worker":i,"cpu":cpu,"tid":unsafe{libc::syscall(libc::SYS_gettid)}}));}).build().unwrap();pool.broadcast(|_|());let mut worker_rows=observed.lock().unwrap().clone();worker_rows.sort_by_key(|v|v["worker"].as_u64().unwrap());assert_eq!(worker_rows.len(),workers);
 let mut words=original.clone();
 zopro_core::g2_operation_census::begin();
 let (result,meta)=pool.install(||zopro_core::coalesce_query_probe::b(&g2,&mut words));
 let point=result.to_affine();let actual=point_hash([point.x.c0,point.x.c1,point.y.c0,point.y.c1]);
 assert_eq!(actual,oracle["expected_canonical_result_sha256"]);
 assert_eq!(before_points,inputs_digest());
 pool.broadcast(|ctx|assert_eq!(unsafe{libc::sched_getcpu()},if workers==1{2}else{ctx.index()}as i32));
 let (mut rows,global)=zopro_core::g2_operation_census::finish();
 let on=std::env::var("ZOPRO_CPU_LATE_M3").as_deref()==Ok("1")&&n==30089;
 assert_eq!(meta.route,if on{313}else{0});assert_eq!(rows.len(),if on{36}else{20});
 rows.sort_by_key(|row|(row.column,row.multiplier,row.points));
 let named=|c:&[u64]|zopro_core::g2_operation_census::LABELS.iter().zip(c).map(|(k,v)|((*k).to_string(),json!(v))).collect::<serde_json::Map<_,_>>();
 let records=rows.iter().map(|row|{row.validate();json!({"column":row.column,"multiplier":row.multiplier,"points":row.points,"half":row.half,"batch_size":row.size,"normalized_projective_points":row.normalized,"components":row.counts.iter().map(|c|named(c)).collect::<Vec<_>>(),"batch_histograms":row.histogram.iter().map(|v|v.iter().enumerate().filter(|(_,n)|**n!=0).map(|(i,n)|(i.to_string(),json!(n))).collect::<serde_json::Map<_,_>>()).collect::<Vec<_>>()})}).collect::<Vec<_>>();
 let mut sums=[0u64;21];for row in&rows{for c in&row.counts{for(i,v)in c.iter().enumerate(){if i==18||i==19{sums[i]=sums[i].max(*v);}else{sums[i]+=v;}}}}
 let globals=zopro_core::g2_operation_census::GLOBAL_LABELS.iter().zip(global).map(|(k,v)|((*k).to_string(),json!(v))).collect::<serde_json::Map<_,_>>();
 println!("CENSUS {}",json!({"query":"B","fixture_sha256":hash(&fs::read(&a[2]).unwrap()),"input_rows":n,"workers":workers,"worker_rows":worker_rows,"candidate_used":on,"metadata_route":meta.route,"point_inputs_unchanged":true,"independent_complete_MSM_hash_matches":true,"result_sha256":actual,"ordinary_windows":rows.len(),"totals":named(&sums),"normalized_projective_points":rows.iter().map(|r|r.normalized).sum::<usize>(),"global":globals,"rows":records,"new_proofs":0,"no_latency_measurement":true}));
}
