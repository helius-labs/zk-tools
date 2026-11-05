//! Diagnostic source ordinals only; no point data, addresses, clocks or arithmetic changes.
use std::{cell::RefCell,sync::Mutex};
use zopro_bn254::{Affine,CurveConfig,Fq2,G2Affine,G2Projective};
#[derive(Debug)]
pub struct Row { pub window:usize,pub root_points:usize,pub start:usize,pub end:usize,pub batches:[Vec<Vec<u32>>;2] }
struct Active { origin:usize,row:Row,phase:usize }
thread_local! { static ACTIVE:RefCell<Option<Active>>=const{RefCell::new(None)}; }
static ROWS:Mutex<Vec<Row>>=Mutex::new(Vec::new());
struct Restore { previous:Option<Active> }
impl Drop for Restore { fn drop(&mut self){ACTIVE.with(|s|{*s.borrow_mut()=self.previous.take();});} }
pub(crate) fn with_range<T>(bases:&[G2Affine],window:usize,start:usize,end:usize,f:impl FnOnce()->T)->T {
    assert!(start<=end&&end<=bases.len()&&bases.len()<=u32::MAX as usize);
    let next=Active{origin:bases.as_ptr() as usize,row:Row{window,root_points:bases.len(),start,end,batches:std::array::from_fn(|_|Vec::new())},phase:0};
    let guard=Restore{previous:ACTIVE.with(|s|s.replace(Some(next)))};
    let value=f();
    let active=ACTIVE.with(|s|s.borrow_mut().take().unwrap());
    drop(guard);
    for batches in &active.row.batches { assert!(batches.iter().map(Vec::len).sum::<usize>()<=end-start); }
    ROWS.lock().unwrap().push(active.row);
    value
}
pub(crate) fn phase(component:usize){assert!(component<2);ACTIVE.with(|s|{if let Some(a)=s.borrow_mut().as_mut(){a.phase=component;}});}
pub(crate) fn observe<C:CurveConfig<Base=Fq2>>(batch:&[(u32,&Affine<C>)]) {
    ACTIVE.with(|s|{if let Some(a)=s.borrow_mut().as_mut(){
        let stride=std::mem::size_of::<Affine<C>>();assert_eq!(stride,std::mem::size_of::<G2Affine>());
        let indices=batch.iter().map(|(_,p)|{
            let offset=(*p as *const Affine<C> as usize).checked_sub(a.origin).expect("source before root");
            assert_eq!(offset%stride,0);let i=offset/stride;
            assert!(a.row.start<=i&&i<a.row.end&&i<a.row.root_points,"source outside original part");i as u32
        }).collect();
        a.row.batches[a.phase].push(indices);
    }});
}
pub fn begin(){ACTIVE.with(|s|assert!(s.borrow().is_none()));ROWS.lock().unwrap().clear();}
pub fn finish()->Vec<Row>{let mut rows=std::mem::take(&mut *ROWS.lock().unwrap());rows.sort_by_key(|r|(r.window,r.start,r.end));rows}
pub fn query(bases:&[G2Affine],words:&mut [[u64;4]])->G2Projective { crate::msm_affine::g2_query_exclusive(bases,words) }
#[cfg(test)]mod tests {
    use super::*;use rayon::prelude::*;
    #[test]fn source_ordinals_components_and_nested_scope_restoration(){
        begin();let p=vec![G2Affine::GENERATOR;16];
        with_range(&p,7,0,16,||{
            phase(0);observe(&(0..8).map(|i|(i as u32,&p[i])).collect::<Vec<_>>());
            with_range(&p,9,8,16,||{phase(1);observe(&(8..16).rev().map(|i|(i as u32,&p[i])).collect::<Vec<_>>());});
            phase(1);observe(&(0..3).map(|i|(i as u32,&p[i])).collect::<Vec<_>>());
        });
        let rows=finish();assert_eq!(rows.len(),2);assert_eq!(rows[0].window,7);assert_eq!(rows[0].batches[0],vec![(0..8).collect::<Vec<_>>()]);assert_eq!(rows[0].batches[1],vec![vec![0,1,2]]);assert!(rows[1].batches[0].is_empty());assert_eq!(rows[1].batches[1],vec![(8..16).rev().collect::<Vec<_>>()]);
    }
    #[test]fn threaded_scopes_panic_cleanup_and_reuse(){
        let p=vec![G2Affine::GENERATOR;16];
        for workers in[1,4,16,32]{
            begin();let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
            pool.install(||(0..64usize).into_par_iter().for_each(|w|{
                if w%7==0 {let failed=std::panic::catch_unwind(||with_range(&p,1000+w,0,16,||panic!("intentional observer cleanup check")));assert!(failed.is_err());}
                with_range(&p,w,0,16,||{phase(w%2);observe(&(0..8).map(|i|(i as u32,&p[i])).collect::<Vec<_>>());});
                ACTIVE.with(|s|assert!(s.borrow().is_none()));
            }));
            let rows=finish();assert_eq!(rows.len(),64);for(w,row)in rows.iter().enumerate(){assert_eq!(row.window,w);assert_eq!(row.batches[w%2],vec![(0..8).collect::<Vec<_>>()]);assert!(row.batches[1-w%2].is_empty());}
        }
    }
}
