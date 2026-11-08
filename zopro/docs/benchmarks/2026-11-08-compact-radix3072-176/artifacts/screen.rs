use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::{BTreeMap,BTreeSet},fs,path::{Path,PathBuf}};
use zopro_bn254::Fr;
mod split { include!("/home/ubuntu/src/zopro-perf-work/partial-windows175/split.rs"); }
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/compact-radix3072-176";
const Z:&str="/home/ubuntu/src/zk-tools/zopro";
const INPUT:&str="/home/ubuntu/src/zopro-perf-work/g2-queue-scan175/inputs";
const NATIVE:&str="/home/ubuntu/src/zopro-perf-work/g2-operation-census176/analysis.json";
const FIELD:&str="/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate/libzopro_bn254-1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713.rlib";
const POW3:[u64;11]=[1,3,9,27,81,243,729,2187,6561,19683,59049];
type Words=[u64;4];
fn read(p:impl AsRef<Path>)->Value {serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn hash(p:impl AsRef<Path>)->String {format!("{:x}",Sha256::digest(fs::read(p).unwrap()))}
fn save(p:impl AsRef<Path>,v:&Value){assert!(!p.as_ref().exists());fs::write(p,serde_json::to_vec_pretty(v).unwrap()).unwrap();}
fn path(s:&str)->PathBuf {Path::new(ROOT).join(s)}
fn pack(x:i128)->u128 {
    let mut m=x.unsigned_abs();assert!(m<1u128<<126);
    let mut low=0u128;let mut h=0u64;
    for i in 0..11 {let a=(m%3072)as u64;m/=3072;low|=((a&1023)as u128)<<(10*i);h+=(a>>10)*POW3[i];if i==10{assert!(a<=1136);}}
    assert_eq!(m,0);assert!(h<118098);
    low|((h as u128)<<110)|((u128::from(x<0))<<127)
}
fn unsigned_digit(code:u128,i:usize)->i32 {
    let h=((code>>110)&0x1ffff)as u64;
    let q=(h*((1u64<<40).div_ceil(POW3[i])))>>40;
    (((code>>(10*i))&1023)as u64+1024*(q%3))as i32
}
fn digit(code:u128,i:usize)->i16 {
    let a=unsigned_digit(code,i);
    let incoming=if i==0 {0}else{i32::from(unsigned_digit(code,i-1)>=1536)};
    let d=a-3072*i32::from(a>=1536)+incoming;
    let d=if code>>127!=0{-d}else{d};i16::try_from(d).unwrap()
}
fn independent_digits(code:u128)->[i16;11] {
    let mut h=((code>>110)&0x1ffff)as u64;
    let mut incoming=0i32;
    std::array::from_fn(|i|{let a=((code>>(i*10))&1023)as i32+1024*(h%3)as i32;h/=3;let next=i32::from(a>=1536);let d=a-3072*next+incoming;incoming=next;if code>>127!=0{(-d)as i16}else{d as i16}})
}
fn reconstruct(d:&[i16;11],base:i128)->i128 {d.iter().rev().fold(0i128,|a,&d|a.checked_mul(base).unwrap().checked_add(d as i128).unwrap())}
fn components(s:Words)->[i128;2] {std::array::from_fn(|j|{let m=s[2*j]as u128|(((s[2*j+1]&0x7fff_ffff_ffff_ffff)as u128)<<64);assert!(m<1u128<<126);if s[2*j+1]>>63!=0{-(m as i128)}else{m as i128}})}
fn signed(x:i128)->Fr {let a=x.unsigned_abs();let f=Fr::from_canonical([a as u64,(a>>64)as u64,0,0]).unwrap();if x<0{-f}else{f}}
fn checked(x:i128)->u128 {let p=pack(x);assert_eq!(p>>127,u128::from(x<0));let d=std::array::from_fn(|i|digit(p,i));assert!(d.iter().all(|d|d.unsigned_abs()<=1536));assert_eq!(d,independent_digits(p));assert_eq!(reconstruct(&d,3072),x);p}
fn codec_checks()->Value {
    assert_eq!(std::mem::size_of::<[u128;2]>(),32);assert_eq!(std::mem::size_of::<Words>(),32);
    let mut checks=0u64;
    for h in 0u64..118098 {for i in 0..11 {let q=(h*((1u64<<40).div_ceil(POW3[i])))>>40;assert_eq!(q,h/POW3[i]);assert!(h*((1u64<<40).div_ceil(POW3[i]))<1u64<<57);checks+=1;}}
    for a in 0i32..3072 {for previous in 0i32..3072 {let carry=i32::from(previous>=1536);let next=i32::from(a>=1536);let d=a-3072*next+carry;assert!(d.abs()<=1536);assert_eq!(d+3072*next,a+carry);}}
    let mut v=vec![0,1,-1,(1i128<<126)-1,-((1i128<<126)-1)];
    let mut power=1i128;
    for i in 0..11 {for a in [1,1023,1024,1535,1536,1537,2048,3071] {for delta in -1..=1 {if let Some(x)=power.checked_mul(a).and_then(|v|v.checked_add(delta)){if x>=0&&x<(1i128<<126){v.extend([x,-x]);}}}}if i!=10{power=power.checked_mul(3072).unwrap();}}
    let mut seed=0x93cab2d7531842a1u128;
    for _ in 0..16384 {seed=seed.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);let x=(seed&((1u128<<126)-1))as i128;v.push(if seed>>127!=0{-x}else{x});}
    for &x in &v {checked(x);}
    json!({"signed_boundary_and_deterministic_inputs":v.len(),"reciprocal_quotient_checks":checks,"adjacent_digit_cases":3072u64*3072,"bytes_per_scalar_row":32,"top_unsigned_digit_max":1136,"small_reciprocal_constants":11})
}
fn grouping(words:&[Words])->Option<Vec<Words>> {
    if words.len()<256{return None;}let n=words.len();let sample=n.min(2048);let stride=n.div_ceil(sample);let observed=n.div_ceil(stride).min(sample);
    let unique=words.iter().step_by(stride).take(sample).collect::<BTreeSet<_>>();if unique.len()*200>observed*199{return None;}
    let percent=if(32768..65536).contains(&n){95}else{80};let zeros=words.iter().step_by(stride).take(sample).filter(|s|**s==[0;4]).count();let nonzero=observed-zeros;
    if unique.len()-usize::from(zeros!=0)==nonzero&&nonzero*100>observed*percent{return None;}
    let mut seen=BTreeSet::new();let values=words.iter().copied().filter(|&w|w!=[0;4]&&seen.insert(w)).collect::<Vec<_>>();if values.len()*100>n*percent{None}else{Some(values)}
}
#[derive(Default,Debug)]
struct Counts(BTreeMap<String,u64>);
impl Counts {
    fn add(&mut self,k:&str,n:u64){*self.0.entry(k.into()).or_default()+=n;}
    fn get(&self,k:&str)->u64 {*self.0.get(k).unwrap_or(&0)}
    fn pairs(&mut self,prefix:&str,n:usize) {self.add(&format!("{prefix}_calls"),1);self.add(&format!("{prefix}_pairs"),n as u64);self.add(&format!("{prefix}_vector_pairs"),(n/8*8)as u64);self.add(&format!("{prefix}_scalar_tail_pairs"),(n%8)as u64);if n==0{self.add(&format!("{prefix}_zero_calls"),1);}}
    fn join(&mut self,other:&Counts){for(k,&v)in&other.0{self.add(k,v);}}
    fn json(&self)->Value{json!(self.0)}
}
struct Window {occupied:Vec<bool>,projective:Vec<bool>,busy:Vec<bool>,batch:Vec<usize>,queue:Vec<usize>,counts:Counts}
impl Window {
    fn new(cap:usize)->Self {Self{occupied:vec![false;cap],projective:vec![false;cap],busy:vec![false;cap],batch:Vec::new(),queue:Vec::new(),counts:Counts::default()}}
    fn add(&mut self,b:usize){if !self.occupied[b]{self.occupied[b]=true;self.counts.add("first_writes",1);}else{assert!(!self.busy[b]);self.busy[b]=true;self.batch.push(b);}}
    fn execute(&mut self){if self.batch.is_empty(){return;}let n=self.batch.len();self.counts.pairs("stream_batch",n);if n==200{self.counts.add("full_batches",1);}for b in self.batch.drain(..){assert!(self.busy[b]);self.busy[b]=false;}}
    fn flush(&mut self,overflow:bool){if self.queue.is_empty(){return;}self.counts.add("queue_flushes",1);if overflow{self.counts.add("overflow_flushes",1);}for b in self.queue.drain(..){self.counts.add("flushed_points",1);if self.projective[b]{self.counts.add("projective_accumulator_adds",1);}else{self.projective[b]=true;self.counts.add("projective_initializations",1);}}}
    fn drain(&mut self){while let Some(&b)=self.queue.last(){self.counts.add("drain_inspections",1);if self.busy[b]{break;}self.queue.pop();self.counts.add("drained",1);self.add(b);}}
    fn feed(&mut self,digits:impl Iterator<Item=i16>){for d in digits{if d==0{continue;}self.counts.add("terms",1);let b=d.unsigned_abs()as usize-1;if self.busy[b]{self.queue.push(b);self.counts.add("queued",1);if self.queue.len()==199{self.flush(true);}continue;}self.add(b);if self.batch.len()==200{self.execute();self.drain();}}self.execute();self.flush(false);}
    fn transform(&mut self){self.counts.add("high_affine_transform_Fq_products",2*self.occupied.iter().filter(|&&b|b).count()as u64);self.counts.add("high_projective_transform_Fq_products",2*self.projective.iter().filter(|&&b|b).count()as u64);}
    fn finish(mut self)->(Vec<bool>,Counts){let n=self.projective.iter().filter(|&&b|b).count()as u64;self.counts.add("normalized_projective_points",n);self.counts.add("final_projective_affine_combines",n);if n!=0{self.counts.add("nonempty_normalization_calls",1);}assert_eq!(self.counts.get("terms"),self.counts.get("first_writes")+self.counts.get("stream_batch_pairs")+self.counts.get("flushed_points"));assert_eq!(self.counts.get("queued"),self.counts.get("drained")+self.counts.get("flushed_points"));assert_eq!(self.counts.get("flushed_points"),self.counts.get("projective_initializations")+self.counts.get("projective_accumulator_adds"));assert_eq!(self.counts.get("normalized_projective_points"),self.counts.get("projective_initializations"));(self.occupied,self.counts)}
}
fn merge(a:&mut[bool],b:&[bool],c:&mut Counts){for(a,b)in a.chunks_mut(512).zip(b.chunks(512)){let n=a.iter().zip(b).filter(|(a,b)|**a&&**b).count();if n!=0{c.pairs("merge_batch",n);}for(a,b)in a.iter_mut().zip(b){*a|=*b;}}}
fn layer(lists:&mut[Vec<bool>],c:&mut Counts){let mut n=0;for list in lists {n+=list.chunks_exact(2).filter(|p|p[0]&&p[1]).count();*list=list.chunks(2).map(|p|p.iter().any(|&b|b)).collect();}c.pairs("reduction_layer",n);}
fn weighted(buckets:&[bool],c:&mut Counts){c.add("weighted_calls",1);c.add("weighted_slots",buckets.len()as u64);c.add("weighted_nonidentity",buckets.iter().filter(|&&v|v).count()as u64);for b in buckets.chunks(256){assert!(b.len().is_power_of_two());let mut lists=vec![b.to_vec()];while lists[0].len()>1{let copied=lists[0].iter().step_by(2).copied().collect();layer(&mut lists,c);lists.push(copied);}c.add("weighted_local_projective_double_slots",b.len().ilog2()as u64);c.add("weighted_local_projective_subtract_slots",b.len().ilog2()as u64);}c.add("weighted_combine_projective_add_slots",3*buckets.len().div_ceil(256)as u64+1);c.add("weighted_combine_projective_double_slots",8);}
fn indexed_component(rows:&[[i16;11]],w:usize,cap:usize,c:&mut Counts)->Vec<bool>{
    let mut counts=vec![0usize;2*cap];for d in rows.iter().map(|d|d[w]){if d!=0{counts[d.unsigned_abs()as usize-1+usize::from(d<0)*cap]+=1;}}
    c.add("indexed_calls",1);c.add("indexed_inputs",counts.iter().sum::<usize>()as u64);c.add("indexed_sign_bins",counts.len()as u64);
    let mut partials=Vec::new();for &n in&counts{for start in(0..n).step_by(1024){let mut lists=vec![vec![true;(n-start).min(1024)]];while lists[0].len()>1{layer(&mut lists,c);}}partials.push(vec![true;n.div_ceil(1024)]);}
    for lists in partials.chunks_mut(256){while lists.iter().any(|l|l.len()>1){layer(lists,c);}}
    let mut positive=counts[..cap].iter().map(|&n|n!=0).collect::<Vec<_>>();let negative=counts[cap..].iter().map(|&n|n!=0).collect::<Vec<_>>();merge(&mut positive,&negative,c);positive
}
fn query(all:&[[[i16;11];2]],candidate:bool)->Value {
    let chunk=all.len().div_ceil(2);let mut total=Counts::default();let mut ordinary=Counts::default();let mut top=Counts::default();let mut windows=Vec::new();let mut recordrows=Vec::new();
    for w in 0..11 {let cap=if candidate{1536}else if w==10{64}else{2048};let mut part_buckets=Vec::new();let mut wc=Counts::default();
        for (part,rows)in all.chunks(chunk).enumerate(){let mut c=Counts::default();let buckets=if candidate||w!=10 {let mut s=Window::new(cap);s.feed(rows.iter().map(|r|r[1][w]));s.transform();s.feed(rows.iter().map(|r|r[0][w]));let(b,n)=s.finish();c.join(&n);b}else{let high=rows.iter().map(|r|r[1]).collect::<Vec<_>>();let low=rows.iter().map(|r|r[0]).collect::<Vec<_>>();let mut high=indexed_component(&high,w,cap,&mut c);c.add("high_affine_transform_Fq_products",2*high.iter().filter(|&&b|b).count()as u64);let low=indexed_component(&low,w,cap,&mut c);merge(&mut high,&low,&mut c);high};
            recordrows.push(json!({"window":w,"part":part,"counts":c.json()}));wc.join(&c);part_buckets.push(buckets);
        }
        let mut combined=part_buckets.remove(0);for b in part_buckets {merge(&mut combined,&b,&mut wc);}weighted(&combined,&mut wc);
        if w==10{top.join(&wc);}else{ordinary.join(&wc);}total.join(&wc);windows.push(json!({"window":w,"capacity_per_partition":cap,"counts":wc.json()}));
    }
    let joins=total.get("stream_batch_pairs")+total.get("flushed_points")+total.get("merge_batch_pairs")+total.get("reduction_layer_pairs");
    json!({"total":total.json(),"ordinary":ordinary.json(),"top":top.json(),"windows":windows,"part_records":recordrows,"generic_mathematical_joins":joins,"interwindow_horner_doubles":if candidate{110}else{120},"interwindow_horner_extra_projective_adds":if candidate{10}else{0}})
}
fn prepare(){
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");let m=read(&manifest);let sources=m["source_files_sha256"].as_object().unwrap();assert_eq!(sources.len(),176);let mut pins=BTreeMap::new();
    for(p,h)in sources{let p=Path::new(Z).join(p);assert_eq!(hash(&p),h.as_str().unwrap());pins.insert(p.display().to_string(),hash(p));}
    for p in[path("screen.rs"),path("design.md"),path("screen"),manifest,PathBuf::from(FIELD),PathBuf::from(NATIVE),PathBuf::from("/home/ubuntu/src/zopro-perf-work/partial-windows175/split.rs"),Path::new(INPUT).join("manifest.json")]{pins.insert(p.display().to_string(),hash(p));}
    for f in read(Path::new(INPUT).join("manifest.json"))["fixtures"].as_array().unwrap(){let p=f["file"].as_str().unwrap();assert_eq!(hash(p),f["sha256"].as_str().unwrap());pins.insert(p.into(),hash(p));}
    save(path("pins.json"),&json!(pins));save(path("plan.json"),&json!({"version":1,"source_pins":path("pins.json"),"steps":[{"kind":"command","name":"codec-and-execution-type-screen","argv":[path("screen"),PathBuf::from("run")],"timeout_seconds":300}]}));println!("Prepared {} source/input pins, no recordings",pins.len());
}
fn run(){
    for(p,h)in read(path("pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
    let codec=codec_checks();let native=read(NATIVE);let mut rows=Vec::new();let lambda=split::phi_lambda();
    for f in read(Path::new(INPUT).join("manifest.json"))["fixtures"].as_array().unwrap(){let bytes=fs::read(f["file"].as_str().unwrap()).unwrap();let n=u64::from_le_bytes(bytes[8..16].try_into().unwrap())as usize;assert_eq!(n,f["points"].as_u64().unwrap()as usize);assert_eq!(bytes.len(),16+160*n);assert!(bytes[16..16+128*n].chunks_exact(128).all(|p|p.iter().any(|&b|b!=0)));
        let raw=bytes[16+128*n..].chunks_exact(32).map(|s|std::array::from_fn(|i|u64::from_le_bytes(s[8*i..8*i+8].try_into().unwrap()))).collect::<Vec<Words>>();let grouped=grouping(&raw);let words=grouped.as_deref().unwrap_or(&raw);let shape=f["shape"].as_str().unwrap();assert_eq!(words.len(),if shape=="1x2"{30089}else{28852});assert_eq!(grouped.is_some(),shape=="2x2");
        let mut b=Vec::new();let mut c=Vec::new();for &s in words{let phi=split::phi_split(s);let components=components(phi);assert_eq!(signed(components[0])+lambda*signed(components[1]),Fr::from_canonical(s).unwrap());let mut bd=[[0;11];2];let mut cd=[[0;11];2];let mut packed=[0u64;4];for j in 0..2{let code=checked(components[j]);packed[2*j]=code as u64;packed[2*j+1]=(code>>64)as u64;for w in 0..11{bd[j][w]=split::digit_of(&phi,j!=0,w,12);cd[j][w]=digit(code,w);}assert_eq!(reconstruct(&bd[j],4096),components[j]);let reread=packed[2*j]as u128|((packed[2*j+1]as u128)<<64);assert_eq!(reread,code);}b.push(bd);c.push(cd);}
        let baseline=query(&b,false);let candidate=query(&c,true);
        // This previously recorded native census is reused, not rerun.
        if shape=="1x2"{let nc=&native["cases"]["1x2-OFF-w1"];let mut sums=Counts::default();for r in nc["rows"].as_array().unwrap(){for c in r["components"].as_array().unwrap(){for (from,to)in[("terms","terms"),("first_writes","first_writes"),("affine_updates","stream_batch_pairs"),("batches","stream_batch_calls"),("full_batches","full_batches"),("scalar_tail_pairs","stream_batch_scalar_tail_pairs"),("vector_pairs","stream_batch_vector_pairs"),("queue_pushes","queued"),("drained","drained"),("flush_points","flushed_points"),("overflow_flushes","overflow_flushes"),("projective_adds","projective_accumulator_adds"),("projective_initializations","projective_initializations")]{sums.add(to,c[from].as_u64().unwrap());}}}
            for(k,&v)in&sums.0{assert_eq!(baseline["ordinary"][k].as_u64().unwrap_or(0),v,"native ordinary {k}");}
            for(from,to)in[("reduction_generic_pairs","reduction_layer_pairs"),("reduction_vector_pairs","reduction_layer_vector_pairs"),("reduction_scalar_tail_pairs","reduction_layer_scalar_tail_pairs"),("reduction_layers","reduction_layer_calls"),("indexed_calls","indexed_calls"),("indexed_inputs","indexed_inputs"),("indexed_sign_bins","indexed_sign_bins"),("weighted_calls","weighted_calls"),("weighted_nonidentity","weighted_nonidentity"),("weighted_slots","weighted_slots")]{assert_eq!(baseline["total"][to],nc["global"][from],"native global {from}");}
            assert_eq!(baseline["ordinary"]["normalized_projective_points"],nc["normalized_projective_points"]);
        }
        let all=candidate["generic_mathematical_joins"].as_u64().unwrap()as f64/baseline["generic_mathematical_joins"].as_u64().unwrap()as f64;
        rows.push(json!({"shape":shape,"raw_rows":n,"modeled_rows":words.len(),"grouping_admitted":grouped.is_some(),"grouped_point_identities_unknown":grouped.is_some(),"component_checks":2*words.len(),"generic_join_ratio":all,"baseline":baseline,"candidate":candidate}));
    }
    save(path("analysis.json"),&json!({"schema":1,"codec_checks":codec,"fixture_rows":rows,"baseline_primary_native_work_census_matches":true,"generic_noncancelling_assumption":true,"new_proofs":0,"new_timings":0,"production_changed":false,"proof_ledger_count":37023,"goal_achieved":false}));
    println!("32-byte codec checks and native primary work-census parity PASS; execution-type counts saved, no timings or proofs");
}
fn main(){match std::env::args().nth(1).as_deref(){Some("prepare")=>prepare(),Some("run")=>run(),_=>panic!("prepare | run")}}
