use std::{arch::x86_64::*, hint::black_box};
const MASK: u64 = (1 << 52) - 1;
macro_rules! width {
($check:ident,$n:literal,$v:ty,$zero:ident,$lo:ident,$hi:ident,$and:ident,$splat:ident,$raw:ident,$masked:ident,$feed:ident,$feedraw:ident,$mask48:ident)=>{
#[no_mangle]#[inline(never)]pub fn $raw(a:$v,b:$v,p:$v)->$v{unsafe{$lo($zero(),a,b)}}
#[no_mangle]#[inline(never)]pub fn $masked(a:$v,b:$v,p:$v)->$v{unsafe{$and($lo($zero(),a,b),$splat(MASK as i64))}}
#[no_mangle]#[inline(never)]pub fn $feed(a:$v,b:$v,p:$v)->$v{unsafe{let m=$and($lo($zero(),a,b),$splat(MASK as i64));$hi($zero(),m,p)}}
#[no_mangle]#[inline(never)]pub fn $feedraw(a:$v,b:$v,p:$v)->$v{unsafe{$hi($zero(),$lo($zero(),a,b),p)}}
#[no_mangle]#[inline(never)]pub fn $mask48(a:$v,b:$v,p:$v)->$v{unsafe{$and($lo($zero(),a,b),$splat(((1u64<<48)-1)as i64))}}
fn $check(){
 let edges=[0,1,(1u64<<48)-1,1<<48,(1u64<<48)+1,MASK-1,MASK,1<<52,(1u64<<52)+1,(1u64<<63)-1,1<<63,u64::MAX];let mut state=0x6d61736b5f353264u64;let mut checked=0u64;
 for row in 0..edges.len()*edges.len()+8192{
 let a:[u64;$n]=core::array::from_fn(|i|if row<edges.len()*edges.len(){edges[(row/edges.len()+i)%edges.len()]}else{state^=state<<13;state^=state>>7;state^=state<<17;state});
 let b:[u64;$n]=core::array::from_fn(|i|if row<edges.len()*edges.len(){edges[(row%edges.len()+2*i)%edges.len()]}else{state^=state<<13;state^=state>>7;state^=state<<17;state});let p:[u64;$n]=core::array::from_fn(|i|a[i]^b[i].rotate_left(29));
 // Exact-size SIMD conversions; CPU support is checked before entry.
 let(av,bv,pv):($v,$v,$v)=unsafe{(std::mem::transmute(black_box(a)),std::mem::transmute(black_box(b)),std::mem::transmute(black_box(p)))};
 let raw:[u64;$n]=unsafe{std::mem::transmute($raw(av,bv,pv))};let masked:[u64;$n]=unsafe{std::mem::transmute($masked(av,bv,pv))};let feed:[u64;$n]=unsafe{std::mem::transmute($feed(av,bv,pv))};let feedraw:[u64;$n]=unsafe{std::mem::transmute($feedraw(av,bv,pv))};
 for i in 0..$n{let exact=(((a[i]&MASK)as u128*(b[i]&MASK)as u128)&MASK as u128)as u64;let high=((exact as u128*(p[i]&MASK)as u128)>>52)as u64;assert_eq!(raw[i],exact);assert_eq!(masked[i],exact);assert_eq!(feed[i],high);assert_eq!(feedraw[i],high);assert_eq!(raw[i]>>52,0);checked+=1;}
 }
 let(a,b,p):($v,$v,$v)=unsafe{(std::mem::transmute([(1u64<<48)+1;$n]),std::mem::transmute([1u64;$n]),std::mem::transmute([1u64;$n]))};let raw:[u64;$n]=unsafe{std::mem::transmute($raw(a,b,p))};let short:[u64;$n]=unsafe{std::mem::transmute($mask48(a,b,p))};assert!(raw.iter().zip(short).all(|(x,y)|*x!=y&&y==1));
 println!("ISA lanes={} raw_reference_checks={} feed_reference_checks={} necessary48_counterexamples={}",$n,checked,checked*2,$n);
}
}}
width!(
    four,
    4,
    __m256i,
    _mm256_setzero_si256,
    _mm256_madd52lo_epu64,
    _mm256_madd52hi_epu64,
    _mm256_and_si256,
    _mm256_set1_epi64x,
    zero_raw4,
    zero_mask52_4,
    zero_feed52_4,
    zero_feedraw4,
    zero_mask48_4
);
width!(
    eight,
    8,
    __m512i,
    _mm512_setzero_si512,
    _mm512_madd52lo_epu64,
    _mm512_madd52hi_epu64,
    _mm512_and_si512,
    _mm512_set1_epi64,
    zero_raw8,
    zero_mask52_8,
    zero_feed52_8,
    zero_feedraw8,
    zero_mask48_8
);
fn main() {
    assert!(is_x86_feature_detected!("avx512ifma") && is_x86_feature_detected!("avx512vl"));
    four();
    eight();
}
