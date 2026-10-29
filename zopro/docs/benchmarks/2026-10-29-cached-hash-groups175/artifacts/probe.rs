use serde_json::{json,Value};
use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet},fs,hash::{BuildHasher,Hasher},hint::black_box,time::Instant};
use zopro_bn254::Fr;
use zopro_prover::zopro_core::artifact::Artifact;
mod index_map;
mod builders;
mod perf;
use builders::{Groups,original};
mod parent;
use parent::candidate as representative;
mod deferred;
mod forecast;
use deferred::candidate;
#[global_allocator]static ALLOCATOR:zopro_prover::zopro_core::alloc::Allocator=zopro_prover::zopro_core::alloc::Allocator;
struct Input { name:String, words:Vec<[u64;4]>, capacity:usize, percent:usize, expected:Option<Groups> }
fn oracle(words:&[[u64;4]],percent:usize)->Option<Groups>{
    let mut lookup=BTreeMap::<[u64;4],Vec<u32>>::new();let mut first=Vec::new();
    for (i,&v) in words.iter().enumerate(){if v==[0;4]{continue;}if !lookup.contains_key(&v){first.push(v);}lookup.entry(v).or_default().push(i as u32);}
    if first.len()*100>words.len()*percent{return None;}
    let mut out=Groups{len:words.len(),values:first,order:vec![],starts:vec![0],counts:vec![]};
    for v in &out.values {let rows=&lookup[v];out.counts.push(rows.len());out.order.extend(rows);out.starts.push(out.order.len());}Some(out)
}
fn sample(words:&[[u64;4]])->Option<(usize,usize)>{
    let n=words.len();if !(256..u32::MAX as usize).contains(&n){return None;}
    let sample=n.min(2048);let stride=n.div_ceil(sample);
    let unique:HashSet<&[u64;4],ahash::RandomState>=words.iter().step_by(stride).take(sample).collect();
    let observed=n.div_ceil(stride).min(sample);if unique.len()*200>observed*199{return None;}
    let percent=if (32768..65536).contains(&n){95}else{80};
    let zeros=words.iter().step_by(stride).take(sample).filter(|s|**s==[0;4]).count();
    let nonzero=observed-zeros;let distinct=unique.len()-usize::from(zeros!=0);
    if distinct==nonzero && nonzero*100>observed*percent{return None;}
    Some(((unique.len()*16).min(n*percent/100).min(32768),percent))
}
fn pin_cpu(){unsafe{let mut set:libc::cpu_set_t=std::mem::zeroed();libc::CPU_ZERO(&mut set);libc::CPU_SET(2,&mut set);assert_eq!(libc::sched_setaffinity(0,std::mem::size_of_val(&set),&set),0);let mut actual:libc::cpu_set_t=std::mem::zeroed();assert_eq!(libc::sched_getaffinity(0,std::mem::size_of_val(&actual),&mut actual),0);for i in 0..libc::CPU_SETSIZE as usize {assert_eq!(libc::CPU_ISSET(i,&actual),i==2);}}}
fn inputs(path:&str)->Vec<Input>{
    let cases:Value=serde_json::from_slice(&fs::read(path).unwrap()).unwrap();let mut seen=BTreeSet::new();let mut out=Vec::new();
    let pool=rayon::ThreadPoolBuilder::new().num_threads(32).build().unwrap();
    pool.install(||{for c in cases.as_array().unwrap(){let shape=c["shape"].as_str().unwrap();if !seen.insert(shape.to_string()){continue;}
        let bytes=fs::read(c["key"].as_str().unwrap()).unwrap();let request=fs::read_to_string(c["request"].as_str().unwrap()).unwrap();
        let sdk=zopro_zolana::Prover::load(&bytes).unwrap();let witness=sdk.witness(&request).unwrap();let image=sdk.solver_image().unwrap();drop(sdk);
        let artifact=Artifact::parse(&bytes).unwrap();let key=artifact.key(artifact.points(),Some(&image)).unwrap();let raw=artifact.key(artifact.points(),None).unwrap();
        let witness:Vec<_>=witness.iter().flat_map(|v|v.0.iter().flat_map(|w|w.to_le_bytes())).collect();
        let values=key.msm_scalars(&witness).unwrap();let raw_values=raw.msm_scalars(&witness).unwrap();assert_eq!(values,raw_values);drop(raw);drop(key);
        let canonical=values.iter().map(Vec::len).max().unwrap()<65536;
        for (query,i) in [("A",0),("B",1)]{let words:Vec<_>=values[i].iter().map(|v|if canonical {v.to_canonical()}else{v.0}).collect();
            let policy=sample(&words);let (capacity,percent)=policy.unwrap_or((0,if (32768..65536).contains(&words.len()){95}else{80}));
            // Retain policy-declined data as explicit diagnostic contexts, rather than silently dropping a shape.
            let expected=oracle(&words,percent);println!("INPUT {}",json!({"name":format!("{shape}/{query}"),"n":words.len(),"representation":if canonical{"canonical"}else{"Montgomery"},"sample_admits":policy.is_some(),"capacity":capacity,"percent":percent,"builder_admits":expected.is_some()}));
            out.push(Input{name:format!("{shape}/{query}"),words,capacity,percent,expected});
        }
    }});drop(pool);assert_eq!(out.len(),18);out
}
fn state(block:usize,round:usize)->ahash::RandomState{let n=(block as u64).wrapping_mul(0x9e3779b97f4a7c15)^(round as u64);ahash::RandomState::with_seeds(n, n.rotate_left(13)^12345,n.wrapping_mul(1597),!n)}
fn build(input:&Input,mode:usize,block:usize,round:usize)->Option<Groups>{match mode{0=>original(&input.words,input.capacity,input.percent,state(block,round)),1=>representative(&input.words,input.capacity,input.percent,state(block,round)),2=>candidate(&input.words,input.capacity,input.percent,state(block,round)),_=>panic!()}}
fn main(){let args:Vec<_>=std::env::args().collect();let data=inputs(&args[2]);pin_cpu();let selected:Vec<_>=if args[1]=="profile"{data.iter().filter(|i|i.name=="1x2/B").collect()}else{data.iter().collect()};for input in&selected{for mode in 0..3{for round in 0..3{assert_eq!(build(input,mode,32,round),input.expected);}}}
if args[1]=="profile"{let mode:usize=args[3].parse().unwrap();let i=selected[0];let mut ctl=perf::Control::from_env().unwrap().unwrap();ctl.enable().unwrap();for r in 0..4096{drop(black_box(build(i,mode,33,r)));}ctl.disable().unwrap();assert_eq!(build(i,mode,34,0),i.expected);println!("PROFILE {}",json!({"mode":mode,"calls":4096}));return}assert_eq!(args[1],"latency");for b in 0..31{for offset in 0..18{let idx=if b%2==0{(offset+b)%18}else{(17-offset+b)%18};let i=&data[idx];let mut modes=[0,1,2];modes.rotate_left((b+idx)%3);if(b+idx)%2!=0{modes.reverse()}for mode in modes{let mut ns=0;let mut usage=vec![];for round in 0..8{let before=perf::Usage::now().unwrap();let start=Instant::now();let out=black_box(build(i,mode,b,round));ns+=start.elapsed().as_nanos();let after=perf::Usage::now().unwrap();usage.push(after.difference(&before));assert_eq!(out,i.expected);drop(out);}println!("ROW {}",json!({"block":b,"context":i.name,"mode":mode,"calls":8,"mean_ms":ns as f64/8e6,"usage":usage,"all_outputs_oracle_checked":true}));}}}println!("COMPLETE {}",json!({"contexts":18,"blocks":31,"timed_builders":13392,"priming_builders":162,"new_proofs":0}));}

#[cfg(test)]mod tests{
use super::*;
#[derive(Clone,Default)]struct Constant;
struct ConstantHasher;
impl Hasher for ConstantHasher{fn write(&mut self,_:&[u8]){}fn finish(&self)->u64{0xffff_ffff_ffff_ffff}}
impl BuildHasher for Constant{type Hasher=ConstantHasher;fn build_hasher(&self)->ConstantHasher{ConstantHasher}}
#[test]fn collision_growth_and_all_word_equality(){for capacity in [0,1,3,17,128]{let mut table=index_map::IndexMap::new(capacity,Constant);let mut values=Vec::new();for i in 0..1025u64{let v=[7,i/3,i%3,9];assert_eq!(table.insert(&v,&mut values),(i as u32,true));assert_eq!(table.insert(&v,&mut values),(i as u32,false));}for i in (0..1025u64).rev(){assert_eq!(table.insert(&[7,i/3,i%3,9],&mut values),(i as u32,false));}assert_eq!(values.len(),1025);}}
#[test]fn stable_full_outputs_match_ordered_oracle(){for n in [0,1,3,255,256,257,1025,4097]{for pattern in 0..7{let words:Vec<_>=(0..n).map(|i:usize|match pattern{0=>[0;4],1=>[7,8,9,10],2=>if i%7==0{[0;4]}else{[(i%17)as u64,4,3,2]},3=>[i as u64,1,2,3],4=>[3,2,1,i as u64],5=>[(i%(n*80/100+1).max(1))as u64,3,4,5],_=>[i as u64*1597, i as u64*65537,!(i as u64),i.rotate_left(7)as u64]}).collect();for percent in [80,95]{let want=oracle(&words,percent);for capacity in [0,1,17,32768]{assert_eq!(original(&words,capacity,percent,state(n,pattern)),want);assert_eq!(candidate(&words,capacity,percent,state(n,pattern)),want);}}}}}
#[test]fn dictionary_payload_fits_existing_map_forecast(){for n in [1usize,3,4,7,17,256,32768,65536,1<<20,u32::MAX as usize-1]{let b=(n*8).div_ceil(7).next_power_of_two().max(4);let old=b*41+80+(b/2)*41+80;let new=b*5+16+(b/2)*5+16;assert!(new<=old);assert_eq!(std::mem::size_of::<u64>(),8);}}
}
