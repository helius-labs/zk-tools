use serde_json::json;
use sha2::{Digest,Sha256};
use std::{alloc::{GlobalAlloc,Layout},sync::atomic::{AtomicUsize,Ordering::Relaxed},time::Instant,hint::black_box};
use zopro_core::MixedCaller;
static ALLOCS:AtomicUsize=AtomicUsize::new(0);
struct Counted;
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self,l:Layout)->*mut u8 {ALLOCS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.alloc(l)}}
    unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8 {ALLOCS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.alloc_zeroed(l)}}
    unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8 {ALLOCS.fetch_add(1,Relaxed);unsafe{zopro_core::alloc::Allocator.realloc(p,l,n)}}
    unsafe fn dealloc(&self,p:*mut u8,l:Layout) {unsafe{zopro_core::alloc::Allocator.dealloc(p,l)}}
}
#[global_allocator] static A:Counted=Counted;
fn main() {
    let args:Vec<_>=std::env::args().collect();assert_eq!(args.len(),5);let size:usize=args[1].parse().unwrap();let collisions:usize=args[2].parse().unwrap();let n:usize=args[3].parse().unwrap();assert!(n>0);
    let selected=match args[4].as_str(){"mixed"=>true,"binary"=>false,_=>panic!("variant")};rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    let mut f=MixedCaller::new(size,collisions);
    f.with_borrowed(|f| {
        for _ in 0..3 {f.reset();if selected{black_box(&mut *f).run(true,8);}else{black_box(&mut *f).run_preselected(8);}f.check(8);}
        f.reset();let before=ALLOCS.load(Relaxed);let start=Instant::now();if selected{black_box(&mut *f).run(true,n);}else{black_box(&mut *f).run_preselected(n);}let elapsed=start.elapsed();black_box(&f);let allocations=ALLOCS.load(Relaxed)-before;
        assert_eq!(allocations,0);f.check(n);let coordinates=f.coordinates();let (mixed_bytes,control_bytes)=f.scratch_bytes();let(queue_entries,selected_entries,leftovers)=f.queue_sizes();assert_eq!(selected_entries,collisions);
        println!("RESULT {}",json!({"size":size,"collisions":collisions,"variant":args[4],"iterations":n,"prime_calls":3,"prime_iterations":8,"elapsed_ns":elapsed.as_nanos(),"ns_per_round":elapsed.as_nanos()as f64/n as f64,"timed_allocations":allocations,"mixed_scratch_capacity_bytes":mixed_bytes,"control_scratch_capacity_bytes":control_bytes,"queue_entries":queue_entries,"selected_entries":selected_entries,"leftover_entries":leftovers,"output_sha256":format!("{:x}",Sha256::digest(serde_json::to_vec(&coordinates).unwrap())),"internally_verified":true,"SDK_requests":0}));
    });
}
#[cfg(test)] mod tests {
    use super::*;
    use ark_bn254::{Fr,G1Projective};use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};use ark_ff::{AdditiveGroup,Field,PrimeField,BigInt};
    fn word(seed:&mut u64)->[u64;4] {let mut out=[0;4];for x in &mut out {*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;*x=*seed;}out[3]&=0x1fffffffffffffff;out}
    fn input(n:usize,seed:&mut u64)->Vec<[[u64;4];3]> {(0..n).map(|_|[word(seed),word(seed),word(seed)]).collect()}
    fn scalar(w:[u64;4])->Fr {Fr::from_bigint(BigInt(w)).unwrap()}
    fn check_case(w:&[[[u64;4];3]],m:usize,rounds:&[usize]) {
        let mut fixture=MixedCaller::from_words(w,m);let(initial,pending,queue)=fixture.inputs();
        let mut selected=vec![false;queue.len()];let mut seen=std::collections::BTreeSet::new();
        for (j,e) in queue.iter().enumerate().rev() {if pending.iter().any(|p|p.0==e.0)&&seen.insert(e.0){selected[j]=true;}}
        assert_eq!(seen.len(),m);
        let mut delta=vec![Fr::ZERO;initial.len()];for (i,s) in pending {delta[i as usize]+=scalar(s);}for (j,(i,s)) in queue.iter().enumerate() {if selected[j]{delta[*i as usize]+=scalar(*s);}}
        let expected:Vec<_>=rounds.iter().map(|&n| initial.iter().zip(&delta).map(|(a,d)| {
            let p=(G1Projective::generator()*(scalar(*a)+Fr::from(n as u64)* *d)).into_affine();match p.xy(){Some((x,y))=>[x.into_bigint().0,y.into_bigint().0],None=>[[0;4];2]}
        }).collect::<Vec<_>>()).collect();
        fixture.with_borrowed(|f| {for variant in [false,true]{for (j,&n) in rounds.iter().enumerate(){f.reset();f.run(variant,n);f.check(n);assert_eq!(f.coordinates(),expected[j]);let(_,count,_)=f.queue_sizes();assert_eq!(count,m);}}});
    }
    #[test]fn full_width_mixed_signed_tails_reuse_and_queue_order_match_arkworks() {
        let mut seed=0x01c998f7a1ba004d;
        for n in [0usize,1,7,8,9,16,31,200,352] {
            let mut counts=vec![0,n.min(5),n.min(13),n.min(64),n];counts.sort();counts.dedup();
            for m in counts {check_case(&input(n,&mut seed),m,&[1,3]);}
        }
    }
    #[test]fn complete_guards_after_prepared_ordinary_rows_match_arkworks() {
        let edges=[Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(2u64),-Fr::from(2u64),Fr::from(3u64)];let mut seed=0x8409d98b8ba115de;
        for a in edges {for b in edges {for c in edges {
            let mut w=input(24,&mut seed);let target=(11*37+19)%24;w[target]=[a.into_bigint().0,b.into_bigint().0,c.into_bigint().0];check_case(&w,13,&[1]);
        }}}
    }
    #[test]fn optimistic_preselected_control_matches_qualified_endpoints() {
        let mut seed=0x2e511407cd54817;
        for size in [200,352] {for matches in [0,5,8,13,16,32] {
            let words=input(size,&mut seed);let mut fixture=MixedCaller::from_words(&words,matches);
            fixture.with_borrowed(|f| {f.reset();f.run(false,3);f.check(3);let expected=f.coordinates();
                f.reset();f.run_preselected(3);f.check(3);assert_eq!(f.coordinates(),expected);
                f.reset();f.run(true,3);f.check(3);assert_eq!(f.coordinates(),expected);
            });
        }}
    }
}
