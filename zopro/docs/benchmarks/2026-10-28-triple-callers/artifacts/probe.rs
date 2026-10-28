use serde_json::json;
use sha2::{Digest,Sha256};
use std::{alloc::{GlobalAlloc,Layout},sync::atomic::{AtomicUsize,Ordering::Relaxed},time::Instant,hint::black_box};
use zopro_core::{TripleG1,TripleG2};
static ALLOCS:AtomicUsize=AtomicUsize::new(0);
struct Counted;
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self,l:Layout)->*mut u8 { ALLOCS.fetch_add(1,Relaxed); unsafe{zopro_core::alloc::Allocator.alloc(l)} }
    unsafe fn alloc_zeroed(&self,l:Layout)->*mut u8 { ALLOCS.fetch_add(1,Relaxed); unsafe{zopro_core::alloc::Allocator.alloc_zeroed(l)} }
    unsafe fn realloc(&self,p:*mut u8,l:Layout,n:usize)->*mut u8 { ALLOCS.fetch_add(1,Relaxed); unsafe{zopro_core::alloc::Allocator.realloc(p,l,n)} }
    unsafe fn dealloc(&self,p:*mut u8,l:Layout) { unsafe{zopro_core::alloc::Allocator.dealloc(p,l)} }
}
#[global_allocator] static A:Counted=Counted;
enum Fixture { G1(TripleG1),G2(TripleG2) }
impl Fixture {
    fn reset(&mut self) { match self { Self::G1(f)=>f.reset(),Self::G2(f)=>f.reset() } }
    fn run(&mut self,s:bool,n:usize) { match self { Self::G1(f)=>f.run(s,n),Self::G2(f)=>f.run(s,n) } }
    fn check(&self,n:usize) { match self { Self::G1(f)=>f.check(n),Self::G2(f)=>f.check(n) } }
    fn coordinates(&self)->Vec<[[u64;4];4]> { match self { Self::G1(f)=>f.coordinates(),Self::G2(f)=>f.coordinates() } }
    fn scratch(&self)->(usize,usize,usize) { match self { Self::G1(f)=>f.scratch(),Self::G2(f)=>f.scratch() } }
}
fn main() {
    let args:Vec<_>=std::env::args().collect(); assert_eq!(args.len(),5);
    let kind=&args[1]; let size:usize=args[2].parse().unwrap(); let n:usize=args[3].parse().unwrap();
    assert!(n>0); let selected=match args[4].as_str(){"direct"=>true,"binary"=>false,_=>panic!("variant")};
    rayon::ThreadPoolBuilder::new().num_threads(1).build_global().unwrap();
    let mut f=match kind.as_str(){"g1"=>Fixture::G1(TripleG1::new(size)),"g2"=>Fixture::G2(TripleG2::new(size)),_=>panic!("kind")};
    for _ in 0..3 { f.reset(); black_box(&mut f).run(selected,8); f.check(8); }
    f.reset(); let before=ALLOCS.load(Relaxed); let start=Instant::now(); black_box(&mut f).run(selected,n);
    let elapsed=start.elapsed(); black_box(&f); let allocations=ALLOCS.load(Relaxed)-before;
    assert_eq!(allocations,0); f.check(n);
    let bytes=serde_json::to_vec(&f.coordinates()).unwrap(); let (direct_bytes,binary_bytes,record_bytes)=f.scratch();
    println!("RESULT {}",json!({"kind":kind,"size":size,"variant":args[4],"iterations":n,"prime_calls":3,"prime_iterations":8,"elapsed_ns":elapsed.as_nanos(),"ns_per_round":elapsed.as_nanos()as f64/n as f64,"timed_allocations":allocations,"direct_scratch_capacity_bytes":direct_bytes,"binary_scratch_capacity_bytes":binary_bytes,"direct_record_bytes":record_bytes,"output_sha256":format!("{:x}",Sha256::digest(bytes)),"internally_verified":true,"SDK_requests":0}));
}

#[cfg(test)] mod tests {
    use super::*;
    use ark_bn254::{Fr,G1Projective,G2Projective};
    use ark_ec::{AffineRepr,CurveGroup,PrimeGroup};
    use ark_ff::{AdditiveGroup,Field,PrimeField,BigInt};
    fn words(seed:&mut u64)->[u64;4] {
        let mut out=[0;4]; for v in &mut out { *seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;*v=*seed; }
        out[3]&=0x1fffffffffffffff; out
    }
    fn input(n:usize,seed:&mut u64)->Vec<[[u64;4];3]> { (0..n).map(|_|[words(seed),words(seed),words(seed)]).collect() }
    fn s(w:[u64;4])->Fr { Fr::from_bigint(BigInt(w)).unwrap() }
    fn expected_g1(w:&[[[u64;4];3]],n:usize)->Vec<[[u64;4];4]> {
        w.iter().map(|v| { let p=(G1Projective::generator()*(s(v[0])+Fr::from(n as u64)*(s(v[1])+s(v[2])))).into_affine();
            match p.xy(){Some((x,y))=>[x.into_bigint().0,y.into_bigint().0,[0;4],[0;4]],None=>[[0;4];4]} }).collect()
    }
    fn expected_g2(w:&[[[u64;4];3]],n:usize)->Vec<[[u64;4];4]> {
        w.iter().map(|v| { let p=(G2Projective::generator()*(s(v[0])+Fr::from(n as u64)*(s(v[1])+s(v[2])))).into_affine();
            match p.xy(){Some((x,y))=>[x.c0.into_bigint().0,x.c1.into_bigint().0,y.c0.into_bigint().0,y.c1.into_bigint().0],None=>[[0;4];4]} }).collect()
    }
    macro_rules! check { ($name:ident,$ty:ident,$oracle:ident) => {
        #[test] fn $name() {
            let mut seed=0xf70985eabf42cd33;
            for n in [0,1,7,8,9,16,32,64,128] {
                let w=input(n,&mut seed); let mut f=$ty::from_scalars(&w);
                for selected in [false,true] { for rounds in [1,3,17] { f.reset(); f.run(selected,rounds); f.check(rounds); assert_eq!(f.coordinates(),$oracle(&w,rounds)); } }
            }
            let edges=[Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(2u64),-Fr::from(2u64),Fr::from(3u64)];
            // Put the edge in the second vector group, after a regular group has prepared records.
            for a in edges { for b in edges { for c in edges {
                let mut w=input(16,&mut seed); w[15]=[a.into_bigint().0,b.into_bigint().0,c.into_bigint().0];
                let mut f=$ty::from_scalars(&w);
                for selected in [false,true] { f.reset(); f.run(selected,1); f.check(1); assert_eq!(f.coordinates(),$oracle(&w,1)); }
            } } }
            println!("checked full-width arrays, repeated chains,216 exceptional/boundary patterns against Arkworks");
        }
    }; }
    check!(g1_three_point_native_and_complete_fallback,TripleG1,expected_g1);
    check!(g2_three_point_native_and_complete_fallback,TripleG2,expected_g2);
}
