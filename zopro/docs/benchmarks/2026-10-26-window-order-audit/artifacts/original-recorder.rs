//! Private coarse request spans; no execution decision or arithmetic changes.
use std::{cell::RefCell, marker::PhantomData, rc::Rc,
    sync::{OnceLock, atomic::{AtomicBool,AtomicU64,Ordering}}, time::Instant};
pub type Event=(u64,u64,u8,usize,usize,u64,u64,usize);
const LIMIT:usize=4096;
static ENABLED:AtomicBool=AtomicBool::new(false);
static CLOCK:OnceLock<Instant>=OnceLock::new();
static ORIGIN:AtomicU64=AtomicU64::new(0);
static API:AtomicU64=AtomicU64::new(0);
static CORE:AtomicU64=AtomicU64::new(0);
static JOIN:AtomicU64=AtomicU64::new(0);
static QUERIES:[AtomicU64;3]=[AtomicU64::new(0),AtomicU64::new(0),AtomicU64::new(0)];
thread_local!{static EVENTS:RefCell<Vec<Event>>=const{RefCell::new(Vec::new())};}
fn now()->u64{CLOCK.get().expect("reset before recording").elapsed().as_nanos()as u64-ORIGIN.load(Ordering::Relaxed)}
pub struct Guard{index:Option<usize>,id:u64,_local:PhantomData<Rc<()>>}
impl Guard{pub fn id(&self)->u64{self.id}}
impl Drop for Guard{fn drop(&mut self){if let Some(i)=self.index{
    let end=now();EVENTS.with(|all|{let mut all=all.borrow_mut();assert_eq!(all[i].6,0);all[i].6=end;});
}}}
#[inline]pub fn span(kind:u8,parent:u64,a:usize,b:usize)->Guard{
    if !ENABLED.load(Ordering::Relaxed){return Guard{index:None,id:0,_local:PhantomData};}
    let worker=rayon::current_thread_index().map_or(0,|i|i+1);
    let start=now();
    let (index,id)=EVENTS.with(|all|{let mut all=all.borrow_mut();
        assert!(all.len()<LIMIT,"diagnostic capacity exceeded");assert!(all.capacity()>=LIMIT);
        let index=all.len();let id=((worker as u64)<<32)|(index as u64+1);
        all.push((id,parent,kind,a,b,start,0,worker));(index,id)
    });Guard{index:Some(index),id,_local:PhantomData}
}
pub fn api()->Guard{let g=span(0,0,0,0);API.store(g.id(),Ordering::Relaxed);g}
pub(crate)fn core(a:usize,b:usize)->Guard{let g=span(1,API.load(Ordering::Relaxed),a,b);CORE.store(g.id(),Ordering::Relaxed);g}
pub(crate)fn phase(kind:u8,a:usize,b:usize)->Guard{span(kind,CORE.load(Ordering::Relaxed),a,b)}
pub(crate)fn queries()->Guard{let g=phase(9,0,0);JOIN.store(g.id(),Ordering::Relaxed);g}
pub(crate)fn query(index:usize,a:usize,b:usize)->Guard{let g=span(10+index as u8,JOIN.load(Ordering::Relaxed),a,b);QUERIES[index].store(g.id(),Ordering::Relaxed);g}
pub(crate)fn query_id(index:usize)->u64{QUERIES[index].load(Ordering::Relaxed)}
fn clear(){EVENTS.with(|all|{let mut all=all.borrow_mut();all.clear();if all.capacity()<LIMIT{all.reserve_exact(LIMIT);}});}
fn take()->Vec<Event>{EVENTS.with(|all|std::mem::take(&mut *all.borrow_mut()))}
pub fn reset(enabled:bool){
    ENABLED.store(false,Ordering::SeqCst);CLOCK.get_or_init(Instant::now);
    rayon::broadcast(|_|clear());clear();
    API.store(0,Ordering::Relaxed);CORE.store(0,Ordering::Relaxed);JOIN.store(0,Ordering::Relaxed);
    for q in &QUERIES{q.store(0,Ordering::Relaxed);}
    ORIGIN.store(CLOCK.get().unwrap().elapsed().as_nanos()as u64,Ordering::Relaxed);
    ENABLED.store(enabled,Ordering::SeqCst);
}
pub fn snapshot()->Vec<Vec<Event>>{
    ENABLED.store(false,Ordering::SeqCst);let mut all=rayon::broadcast(|_|take());all.push(take());all
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn explicit_parents_worker_ids_disabled_and_unwind(){
        reset(true);
        {let _api=api();let _core=core(7,8);let _join=queries();let query=query(0,9,10);let parent=query.id();
            rayon::broadcast(|_|{let _child=span(17,parent,11,12);});
            let outcome=std::panic::catch_unwind(||{let _child=span(18,parent,13,14);panic!("closed by unwind");});assert!(outcome.is_err());
        }
        let all=snapshot().into_iter().flatten().collect::<Vec<_>>();
        assert_eq!(all.len(),5+rayon::current_num_threads());
        let ids=all.iter().map(|e|e.0).collect::<std::collections::HashSet<_>>();assert_eq!(ids.len(),all.len());
        for e in &all{assert!(e.6>=e.5);assert_eq!(e.0>>32,e.7 as u64);
            if e.1!=0{let parent=all.iter().find(|p|p.0==e.1).unwrap();assert!(parent.5<=e.5&&e.6<=parent.6);}
        }
        reset(false);{let _a=api();rayon::broadcast(|_|{let _b=span(17,0,1,2);});}
        assert!(snapshot().into_iter().flatten().next().is_none());
    }
}
