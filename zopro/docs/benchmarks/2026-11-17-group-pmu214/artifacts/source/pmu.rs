//! Diagnostic per-worker user-space cycles/instructions; Linux x86-64 ABI.
use std::{cell::OnceCell,os::fd::{AsRawFd,FromRawFd,OwnedFd},sync::{OnceLock,atomic::{AtomicU64,Ordering}}};
static ACTIVE:OnceLock<bool>=OnceLock::new();static RECORD:[AtomicU64;8]=[const{AtomicU64::new(0)};8];
thread_local!{static GROUP:OnceCell<Group>=const{OnceCell::new()};}
fn active()->bool{*ACTIVE.get_or_init(||std::env::var("ZOPRO_DIAGNOSTIC_PMU").as_deref()==Ok("1"))}
struct Group{leader:OwnedFd,_member:OwnedFd,ids:[u64;2]}
impl Group{
 fn open()->Self{
  fn event(config:u64,group:i32)->OwnedFd{
   // Linux perf_event_attr: type/size at0,config8,read_format32,flags40.
   // All128bytes are initialized; exclude_kernel/hv, disabled only leader.
   let mut attr=[0u64;16];attr[0]=128u64<<32;attr[1]=config;attr[4]=15;attr[5]=(1<<5)|(1<<6)|u64::from(group<0);
   let fd=unsafe{libc::syscall(libc::SYS_perf_event_open,attr.as_ptr(),0i32,-1i32,group,8u32)};
   assert!(fd>=0,"perf_event_open: {}",std::io::Error::last_os_error());unsafe{OwnedFd::from_raw_fd(fd.try_into().unwrap())}
  }
  let leader=event(0,-1);let member=event(1,leader.as_raw_fd());let mut ids=[0u64;2];
  for(fd,id)in[(leader.as_raw_fd(),&mut ids[0])]{assert_eq!(unsafe{libc::ioctl(fd,0x80082407u64,id)},0);}
  assert_eq!(unsafe{libc::ioctl(member.as_raw_fd(),0x80082407u64,&mut ids[1])},0);assert_ne!(ids[0],ids[1]);
  assert_eq!(unsafe{libc::ioctl(leader.as_raw_fd(),0x2400u64,1u32)},0);
  Self{leader,_member:member,ids}
 }
 fn read(&self)->[u64;4]{let mut raw=[0u64;7];let n=unsafe{libc::read(self.leader.as_raw_fd(),raw.as_mut_ptr().cast(),std::mem::size_of_val(&raw))};assert_eq!(n,std::mem::size_of_val(&raw)as isize,"perfgroupread: {}",std::io::Error::last_os_error());assert_eq!(raw[0],2);let mut counts=[0u64;2];let mut found=[false;2];for pair in raw[3..].chunks_exact(2){let i=self.ids.iter().position(|&id|id==pair[1]).unwrap();assert!(!found[i]);found[i]=true;counts[i]=pair[0];}assert_eq!(found,[true;2]);[counts[0],counts[1],raw[1],raw[2]]}
}
pub(super)fn initialize()->usize{if !active(){return 0;}rayon::broadcast(|_|GROUP.with(|g|{g.get_or_init(Group::open);})).len()}
pub(super)fn reset(){for r in&RECORD{r.store(0,Ordering::Relaxed);}}
fn record(end:bool){if !active(){return;}let values=GROUP.with(|g|g.get().expect("PMUworker not initialized before primes").read());for(i,&v)in values.iter().enumerate(){RECORD[2*i+usize::from(end)].store(v,Ordering::Relaxed);}}
pub(super)fn first(){record(false)}pub(super)fn last(){record(true)}
pub(super)fn snapshot()->[u64;8]{std::array::from_fn(|i|RECORD[i].load(Ordering::Relaxed))}
