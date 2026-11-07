//! The two scaled APIs must be checked independently of point cancellation.
use crate::{FpConfig,FqConfig,FrConfig};
use ark_ff::{Field,PrimeField};
use core::{arch::x86_64::{__m256i,__m512i},marker::PhantomData};
fn bytes(x:[u64;4])->[u8;32]{let mut b=[0;32];for i in 0..4{b[i*8..i*8+8].copy_from_slice(&x[i].to_le_bytes());}b}
fn split(x:[u64;4])->[u64;5]{let mask=(1u64<<52)-1;[x[0]&mask,((x[0]>>52)|(x[1]<<12))&mask,((x[1]>>40)|(x[2]<<24))&mask,((x[2]>>28)|(x[3]<<36))&mask,x[3]>>16]}
fn random(seed:&mut u64,twop:[u64;4])->[u64;4]{let mut x=[0;4];for v in &mut x{*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;*v=*seed;}x[3]&=(1u64<<63)-1;if !crate::portable::less(x,twop){x=crate::portable::sub4(x,twop).0;}x}
macro_rules! checks {($name:ident,$width:literal,$module:ident,$kind:ident)=>{
 fn $name<C:FpConfig,A:PrimeField>(){let p=C::P;let twop=crate::portable::add4(p,p).0;let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,crate::portable::add4(p,[1,0,0,0]).0,crate::portable::sub4(twop,[1,0,0,0]).0,twop];let hinv=A::from(2).pow([256u64]).inverse().unwrap();let h=A::from(16).inverse().unwrap();let mut seed=0x3bc251e9a0847df6u64;let mut chain=[[0;4];$width];let mask=(1u64<<52)-1;let mut different=0;
 fn pack<C:FpConfig>(v:[[u64;4];$width])->crate::$module::$kind<C,true>{let d=v.map(split);let vectors=core::array::from_fn(|j|{let words:[u64;$width]=core::array::from_fn(|i|d[i][j]);
 // SAFETY: test-only reinterpretation of equal-size initialized lane words.
 unsafe{core::mem::transmute::<[u64;$width],$kind>(words)}});crate::$module::$kind(vectors,PhantomData)}
 for round in 0..4096{let raw:[[[u64;4];$width];4]=core::array::from_fn(|operand|core::array::from_fn(|lane|{if round<2401{let mut ix=(round+lane)%2401;for _ in 0..operand{ix/=7;}edges[ix%7]}else if round<2443{let bits=[1usize,51,52,53,63,64,103,104,155,156,207,208,253,254][(round-2401)/3];let mut value=[0;4];value[bits/64]=1u64<<(bits%64);let mode=(round-2401)%3;if mode==1{value=crate::portable::add4(value,[1,0,0,0]).0;}if mode==2{value=crate::portable::sub4(value,[1,0,0,0]).0;}if operand%2==1{crate::portable::sub4(twop,value).0}else{value}}else if operand==0&&round%2==0{chain[lane]}else{random(&mut seed,twop)}}));
 let [a,b,c,d]=raw.map(pack::<C>);let values=[a.sum2_div16(&b,&c,&d),a.difference2_div16(&b,&c,&d)];for (negative,v)in values.into_iter().enumerate(){let words:[[u64;$width];5]=v.0.map(|x|{
 // SAFETY: each SIMD word has exactly width initialized u64 lanes.
 unsafe{core::mem::transmute::<$kind,[u64;$width]>(x)}});let exported=v.get();for lane in 0..$width{let limb:[u64;5]=core::array::from_fn(|j|words[j][lane]);assert!(limb.iter().all(|x|*x<=mask));assert!(limb[4]<(1u64<<48));let out=[limb[0]|(limb[1]<<52),(limb[1]>>12)|(limb[2]<<40),(limb[2]>>24)|(limb[3]<<28),(limb[3]>>36)|(limb[4]<<16)];assert!(crate::portable::less(out,twop));let input:[A;4]=core::array::from_fn(|j|A::from_le_bytes_mod_order(&bytes(raw[j][lane]))*hinv);let unscaled=if negative==0{input[0]*input[1]+input[2]*input[3]}else{input[0]*input[1]-input[2]*input[3]};let want=unscaled*h;let got=A::from_le_bytes_mod_order(&bytes(out))*hinv;assert_eq!(got,want);assert_eq!(A::from_le_bytes_mod_order(&bytes(exported[lane].0))*hinv,want);if got!=unscaled{different+=1;}if negative==round%2{chain[lane]=out;}}}}
 assert!(different>1000);}
};}
// The macro's vector type and field wrapper have different names.
type Ifma=__m256i;
type Ifma8=__m512i;
checks!(check4,4,ifma,Ifma);
checks!(check8,8,ifma8,Ifma8);
#[test]fn scaled_fused_div16_four_lanes(){check4::<FqConfig,ark_ref::Fq>();check4::<FrConfig,ark_ref::Fr>();}
#[test]fn scaled_fused_div16_eight_lanes(){check8::<FqConfig,ark_ref::Fq>();check8::<FrConfig,ark_ref::Fr>();}
