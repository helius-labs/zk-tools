//! Diagnostic counters. No timing or speed estimate is derived from this build.
use std::{collections::BTreeMap,mem::size_of,sync::{atomic::{AtomicBool,Ordering},Mutex}};
use zopro_bn254::{CurveConfig,Fq};
static ACTIVE:AtomicBool=AtomicBool::new(false);
static TOTAL:Mutex<BTreeMap<String,u64>>=Mutex::new(BTreeMap::new());
pub fn begin() {
    assert!(!ACTIVE.load(Ordering::Acquire));
    TOTAL.lock().unwrap().clear();
    ACTIVE.store(true,Ordering::Release);
}
pub fn end()->BTreeMap<String,u64> {
    assert!(ACTIVE.swap(false,Ordering::AcqRel));
    TOTAL.lock().unwrap().clone()
}
fn curve<C:CurveConfig>()->&'static str { if size_of::<C::Base>()==size_of::<Fq>() {"g1"} else {"g2"} }
fn add(map:&mut BTreeMap<String,u64>,key:impl Into<String>,n:u64) { *map.entry(key.into()).or_default()+=n; }
#[derive(Debug,PartialEq,Eq)]
struct Queue { matched_destinations:usize,matched_entries:usize,free_entries:usize,extra_per_matched:usize }
fn classify(pending:impl Iterator<Item=u32>,queued:impl Iterator<Item=u32>)->Queue {
    let pending:std::collections::BTreeSet<_>=pending.map(|i|i&0x7fffffff).collect();
    let mut counts=BTreeMap::<u32,usize>::new();
    for i in queued { *counts.entry(i&0x7fffffff).or_default()+=1; }
    let mut q=Queue {matched_destinations:0,matched_entries:0,free_entries:0,extra_per_matched:0};
    for (i,n) in counts {
        if pending.contains(&i) { q.matched_destinations+=1;q.matched_entries+=n;q.extra_per_matched+=n-1; }
        else { q.free_entries+=n; }
    }
    q
}
pub(crate) struct Stream {active:bool,prefix:String,local:BTreeMap<String,u64>}
impl Stream {
    pub(crate) fn new<C:CurveConfig>(path:&str,half:usize,batch:usize)->Self {
        let active=ACTIVE.load(Ordering::Acquire);
        let mut s=Self {active,prefix:format!("stream/{}/{path}/buckets={half}/batch={batch}/",curve::<C>()),local:BTreeMap::new()};
        if active { add(&mut s.local,"windows",1); }
        s
    }
    pub(crate) fn pushed(&mut self) {if self.active {add(&mut self.local,"pending_pushes",1);}}
    pub(crate) fn queued(&mut self) {if self.active {add(&mut self.local,"queued_arrivals",1);}}
    pub(crate) fn empty(&mut self) {if self.active {add(&mut self.local,"empty_bucket_assignments",1);}}
    pub(crate) fn projective(&mut self) {if self.active {add(&mut self.local,"projective_updates",1);}}
    pub(crate) fn blocked(&mut self) {if self.active {add(&mut self.local,"queue_take_blocked",1);}}
    pub(crate) fn execute<P>(&mut self,pending:&[(u32,P)],queued:&[(u32,P)]) {
        if !self.active {return;}
        let q=classify(pending.iter().map(|p|p.0),queued.iter().map(|p|p.0));
        add(&mut self.local,"execute_calls",1);
        add(&mut self.local,"executed_entries",pending.len()as u64);
        add(&mut self.local,"matched_destinations_over_flushes",q.matched_destinations as u64);
        add(&mut self.local,"matched_entries_over_flushes",q.matched_entries as u64);
        add(&mut self.local,"free_entries_over_flushes",q.free_entries as u64);
        add(&mut self.local,format!("flush/len={}/queued={}/matched={}/entries={}/free={}/extra={}",pending.len(),queued.len(),q.matched_destinations,q.matched_entries,q.free_entries,q.extra_per_matched),1);
    }
    pub(crate) fn flush<P>(&mut self,pending:&[(u32,P)],queued:&[(u32,P)]) {
        if !self.active || queued.is_empty() {return;}
        let q=classify(pending.iter().map(|p|p.0),queued.iter().map(|p|p.0));
        add(&mut self.local,"projective_queue_flushes",1);
        add(&mut self.local,"projective_queue_entries",queued.len()as u64);
        add(&mut self.local,format!("projective_flush/pending={}/queued={}/matched={}/entries={}/free={}",pending.len(),queued.len(),q.matched_destinations,q.matched_entries,q.free_entries),1);
    }
}
impl Drop for Stream {
    fn drop(&mut self) {
        if !self.active {return;}
        let mut total=TOTAL.lock().unwrap();
        for (key,n) in &self.local { add(&mut total,format!("{}{key}",self.prefix),*n); }
    }
}
fn census(counts:&[usize],ordinary_pairs:usize)->String {
    format!("groups={}/points={}/binary_slots={}/ordinary_pairs={ordinary_pairs}/triple_slots={}/max={}/groups_three={}",counts.len(),counts.iter().sum::<usize>(),counts.iter().map(|n|n/2).sum::<usize>(),counts.iter().map(|n|n/3).sum::<usize>(),counts.iter().max().copied().unwrap_or(0),counts.iter().filter(|&&n|n==3).count())
}
pub(crate) fn tree<C:CurveConfig>(counts:&[usize],ordinary_pairs:usize) {
    if !ACTIVE.load(Ordering::Acquire) {return;}
    add(&mut TOTAL.lock().unwrap(),format!("tree/{}/{}",curve::<C>(),census(counts,ordinary_pairs)),1);
}
#[cfg(test)]mod tests {
    use super::*;
    #[test]fn signed_queue_classification_counts_distinct_and_repeats() {
        assert_eq!(classify([1,2,3].into_iter(),[1,0x80000001,2,5].into_iter()),Queue{matched_destinations:2,matched_entries:3,free_entries:1,extra_per_matched:1});
        assert_eq!(classify([].into_iter(),[1,1].into_iter()),Queue{matched_destinations:0,matched_entries:0,free_entries:2,extra_per_matched:0});
        assert_eq!(classify([1].into_iter(),[].into_iter()),Queue{matched_destinations:0,matched_entries:0,free_entries:0,extra_per_matched:0});
    }
    #[test]fn tree_census_includes_tails_and_real_ordinary_count() {
        assert_eq!(census(&[0,1,2,3,4,5,9],7),"groups=7/points=24/binary_slots=10/ordinary_pairs=7/triple_slots=6/max=9/groups_three=1");
        assert_eq!(census(&[],0),"groups=0/points=0/binary_slots=0/ordinary_pairs=0/triple_slots=0/max=0/groups_three=0");
    }
    #[test]fn gated_parallel_publish_and_reset_exclude_warmup() {
        use zopro_bn254::G1Config;
        let mut warm=Stream::new::<G1Config>("test",8,2);warm.pushed();drop(warm);
        begin();
        std::thread::scope(|s| {for _ in 0..8 {s.spawn(|| {let mut g=Stream::new::<G1Config>("test",8,2);g.pushed();g.execute(&[(1,()),(2,())],&[(1,()),(1,())]);g.flush(&[(1,())],&[(1,())]);g.projective();g.blocked();});}});
        let v=end();let p="stream/g1/test/buckets=8/batch=2/";
        assert_eq!(v[&format!("{p}windows")],8);assert_eq!(v[&format!("{p}pending_pushes")],8);
        assert_eq!(v[&format!("{p}execute_calls")],8);assert_eq!(v[&format!("{p}matched_destinations_over_flushes")],8);
        assert_eq!(v[&format!("{p}matched_entries_over_flushes")],16);
        begin();assert!(end().is_empty());
    }
}
