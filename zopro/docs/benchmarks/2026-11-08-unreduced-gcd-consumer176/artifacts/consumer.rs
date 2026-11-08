use std::{fs::{self,OpenOptions,File},io::{Read,Write},hint::black_box,sync::Barrier,time::Instant};
use zopro_bn254::{Fp,FpConfig,Fq,FqConfig,Fr,FrConfig};
mod native;
unsafe extern "C"{fn sched_setaffinity(pid:i32,size:usize,mask:*const u64)->i32;fn sched_getcpu()->i32;fn gettid()->i32;}
fn affinity(cpu:usize){let mut mask=[0u64;16];mask[cpu/64]|=1u64<<(cpu%64);unsafe{assert_eq!(sched_setaffinity(0,std::mem::size_of_val(&mask),mask.as_ptr()),0);assert_eq!(sched_getcpu(),cpu as i32);}}
struct Control{command:File,ack:File}
impl Control{
 fn open()->Option<Self>{let command=std::env::var_os("ZOPRO_BENCH_PERF_CTL");let ack=std::env::var_os("ZOPRO_BENCH_PERF_ACK");match(command,ack){(None,None)=>None,(Some(c),Some(a))=>{use std::os::unix::fs::FileTypeExt;for p in[&c,&a]{assert!(fs::metadata(p).unwrap().file_type().is_fifo());}Some(Self{command:OpenOptions::new().read(true).write(true).open(c).unwrap(),ack:OpenOptions::new().read(true).write(true).open(a).unwrap()})},_=>panic!("both control FIFOs required")}}
 fn send(&mut self,command:&[u8]){self.command.write_all(command).unwrap();self.command.flush().unwrap();let mut reply=Vec::new();loop{let mut c=[0u8;1];self.ack.read_exact(&mut c).unwrap();reply.push(c[0]);assert!(reply.len()<32);if c[0]==b'\n'{break;}}assert_eq!(reply,b"ack\n");}
}
#[no_mangle]#[inline(never)]pub fn complete_fq_original(x:&Fq)->Fq{x.inverse().unwrap()}
#[no_mangle]#[inline(never)]pub fn complete_fr_original(x:&Fr)->Fr{x.inverse().unwrap()}
#[no_mangle]#[inline(never)]pub fn complete_fq_candidate(x:&Fq)->Fq{native::inverse(x).unwrap()}
#[no_mangle]#[inline(never)]pub fn complete_fr_candidate(x:&Fr)->Fr{native::inverse(x).unwrap()}
fn load<C:FpConfig>(path:&str)->(Vec<Fp<C>>,Vec<Fp<C>>){let data=fs::read(path).unwrap();assert_eq!(&data[..8],b"UCGI1761");let n=u64::from_le_bytes(data[8..16].try_into().unwrap())as usize;assert_eq!(n,4096);assert_eq!(data.len(),16+64*n);let(mut input,mut expected)=(Vec::new(),Vec::new());for row in data[16..].chunks_exact(64){let a=Fp::<C>::from_montgomery(std::array::from_fn(|i|u64::from_le_bytes(row[i*8..i*8+8].try_into().unwrap())));let b=Fp::<C>::from_montgomery(std::array::from_fn(|i|u64::from_le_bytes(row[32+i*8..40+i*8].try_into().unwrap())));assert!(a.is_reduced()&&b.is_reduced()&&!a.is_zero());input.push(a);expected.push(b);}(input,expected)}
fn field<C:FpConfig>(name:&str,path:&str,arm:&str,workers:usize,passes:usize,qualify:bool,op:fn(&Fp<C>)->Fp<C>){
 let(input,expected)=load::<C>(path);let check=||{for(x,y)in input.iter().zip(&expected){assert_eq!(op(x),*y);}};
 if qualify{assert_eq!(native::inverse(&Fp::<C>::ZERO),None);check();println!("QUALIFIED {{\"field\":\"{name}\",\"arm\":\"{arm}\",\"checks\":4096,\"raw_zero_none\":true}}");return;}
 assert!([1,16,32].contains(&workers)&&passes>0);let ready=Barrier::new(workers+1);let gate=Barrier::new(workers+1);let done=Barrier::new(workers+1);let post=Barrier::new(workers+1);
 let mut expected_sum=[0u64;4];for p in &expected{for j in 0..4{expected_sum[j]=expected_sum[j].wrapping_add(p.0[j]);}}for w in &mut expected_sum{*w=w.wrapping_mul(passes as u64);}
 let mut control=Control::open();
 let(wall,rows)=std::thread::scope(|scope|{
  let mut handles=Vec::new();for worker in 0..workers{let cpu=if workers==1{2}else{worker};let input=&input;let expected=&expected;let ready=&ready;let gate=&gate;let done=&done;let post=&post;
   handles.push(std::thread::Builder::new().stack_size(512*1024).spawn_scoped(scope,move||{
    affinity(cpu);let tid=unsafe{gettid()};for(x,y)in input.iter().zip(expected){assert_eq!(op(x),*y);}for _ in 0..3{for(x,y)in input[..256].iter().zip(&expected[..256]){assert_eq!(op(black_box(x)),*y);}}
    ready.wait();gate.wait();let start=Instant::now();let mut checksum=[0u64;4];for _ in 0..passes{for i in 0..input.len(){let index=(i+worker*137)%input.len();let out=black_box(op(black_box(&input[index])));for j in 0..4{checksum[j]=checksum[j].wrapping_add(out.0[j]);}}}let ns=start.elapsed().as_nanos()as u64;done.wait();post.wait();
    assert_eq!(checksum,expected_sum);for(x,y)in input.iter().zip(expected){assert_eq!(op(x),*y);}assert_eq!(unsafe{sched_getcpu()},cpu as i32);
    format!("{{\"worker\":{worker},\"cpu\":{cpu},\"tid\":{tid},\"elapsed_ns\":{ns},\"checksum\":{checksum:?}}}")
   }).unwrap());
  }
  ready.wait();if let Some(c)=control.as_mut(){c.send(b"enable\n");}let start=Instant::now();gate.wait();done.wait();let wall=start.elapsed().as_nanos()as u64;if let Some(c)=control.as_mut(){c.send(b"disable\n");}post.wait();
  let rows=handles.into_iter().map(|h|h.join().unwrap()).collect::<Vec<_>>();(wall,rows)
 });
 let operations=passes*4096;println!("INVERSE {{\"field\":\"{name}\",\"arm\":\"{arm}\",\"pid\":{},\"workers\":{workers},\"passes\":{passes},\"operations_per_worker\":{operations},\"wall_ns\":{wall},\"wall_ns_per_inverse_per_worker\":{},\"checksum\":{expected_sum:?},\"all_pre_and_post_outputs_checked\":true,\"worker_rows\":[{}]}}",std::process::id(),wall as f64/operations as f64,rows.join(","));
}
fn main(){let a=std::env::args().collect::<Vec<_>>();assert_eq!(a.len(),7,"consumer qualify|bench Fq|Fr FIXTURE OFF|ON WORKERS PASSES");let qualify=match a[1].as_str(){"qualify"=>true,"bench"=>false,_=>panic!("mode")};let workers=a[5].parse().unwrap();let passes=a[6].parse().unwrap();match(a[2].as_str(),a[4].as_str()){
 ("Fq","OFF")=>field::<FqConfig>("Fq",&a[3],"OFF",workers,passes,qualify,complete_fq_original),("Fq","ON")=>field::<FqConfig>("Fq",&a[3],"ON",workers,passes,qualify,complete_fq_candidate),
 ("Fr","OFF")=>field::<FrConfig>("Fr",&a[3],"OFF",workers,passes,qualify,complete_fr_original),("Fr","ON")=>field::<FrConfig>("Fr",&a[3],"ON",workers,passes,qualify,complete_fr_candidate),_=>panic!("field/arm")}}
