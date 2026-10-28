//! Diagnostic-only actual G1 scalar census; never a production dispatch input.
use std::{collections::BTreeMap,sync::{atomic::{AtomicBool,Ordering},Mutex}};
use zopro_bn254::{Affine,CurveConfig,Fq};
static ACTIVE:AtomicBool=AtomicBool::new(false);
static TOTAL:Mutex<BTreeMap<String,u64>>=Mutex::new(BTreeMap::new());
const R:[u64;4]=[0x43e1f593f0000001,0x2833e84879b97091,0xb85045b68181585d,0x30644e72e131a029];
pub fn begin(){assert!(!ACTIVE.load(Ordering::Acquire));TOTAL.lock().unwrap().clear();ACTIVE.store(true,Ordering::Release);}
pub fn end()->BTreeMap<String,u64>{assert!(ACTIVE.swap(false,Ordering::AcqRel));TOTAL.lock().unwrap().clone()}
fn less(a:&[u64;4],b:&[u64;4])->bool{a.iter().rev().cmp(b.iter().rev()).is_lt()}
fn center(k:[u64;4])->([u64;4],bool){
    assert!(less(&k,&R));let mut c=[0;4];let mut borrow=false;
    for i in 0..4{let(v,b1)=R[i].overflowing_sub(k[i]);let(v,b2)=v.overflowing_sub(u64::from(borrow));c[i]=v;borrow=b1||b2;}
    assert!(!borrow);if less(&c,&k){(c,true)}else{(k,false)}
}
fn booth_digit(words:&[u64;4],w:usize,width:usize)->i16 {
    if w==0 {let value=words[0]&((1u64<<width)-1);return (value as i64-((value>>(width-1))<<width)as i64)as i16;}
    let offset=w*width-1;let(word,shift)=(offset/64,offset%64);
    let mut value=if word<4 {words[word]>>shift}else{0};
    if shift+width+1>64&&word<3 {value|=words[word+1]<<(64-shift);}
    value&=(1u64<<(width+1))-1;
    (((value+1)>>1)as i64-((value>>width)<<width)as i64)as i16
}
fn add(m:&mut BTreeMap<String,u64>,k:impl Into<String>,v:u64){*m.entry(k.into()).or_default()+=v;}
fn small(k:&[u64;4])->bool{k[1..]==[0,0,0]&&k[0]>0&&k[0]<=65535}
fn compute<'a,C:CurveConfig<Base=Fq>+'a>(segments:&[(&'a [Affine<C>],&'a [[u64;4]])],width:usize)->BTreeMap<String,u64>{
    assert!((4..=15).contains(&width));let windows=255usize.div_ceil(width);let mut out=BTreeMap::new();
    for &(bases,words) in segments{assert_eq!(bases.len(),words.len());for(base,&k)in bases.iter().zip(words){
        add(&mut out,"inputs",1);let(c,negative)=center(k);
        if base.is_identity(){add(&mut out,"identity_bases",1);continue;}
        add(&mut out,"active_inputs",1);add(&mut out,"centered_signs",u64::from(negative));
        add(&mut out,"small_original",u64::from(small(&k)));add(&mut out,"small_centered",u64::from(small(&c)));
        add(&mut out,"zero_scalars",u64::from(k==[0;4]));
    }}
    for w in 0..windows{
        let mut hist=[vec![0u64;(1<<(width-1))+1],vec![0u64;(1<<(width-1))+1]];
        let mut negative=[0u64;2];
        for &(bases,words) in segments{for(base,&k)in bases.iter().zip(words){if base.is_identity(){continue;}
            let(c,sign)=center(k);let original=booth_digit(&k,w,width);let d=booth_digit(&c,w,width);let centered=if sign{-d}else{d};
            for (i,d)in [original,centered].into_iter().enumerate(){if d!=0{hist[i][d.unsigned_abs()as usize]+=1;negative[i]+=u64::from(d<0);}}
        }}
        for (i,name)in ["original","centered"].into_iter().enumerate(){let terms:u64=hist[i].iter().sum();let occupied=hist[i].iter().filter(|&&n|n!=0).count()as u64;
            let p=format!("w={w}/{name}");out.insert(format!("{p}/terms"),terms);out.insert(format!("{p}/negative"),negative[i]);
            out.insert(format!("{p}/occupied"),occupied);out.insert(format!("{p}/ideal_insert"),terms-occupied);
            out.insert(format!("{p}/max_index"),hist[i].iter().rposition(|&n|n!=0).unwrap_or(0)as u64);
        }
    }
    out
}
fn publish<'a,C:CurveConfig<Base=Fq>+'a>(label:&str,segments:&[(&'a [Affine<C>],&'a [[u64;4]])],width:usize){
    let local=compute(segments,width);let prefix=format!("{label}/width={width}/");let mut total=TOTAL.lock().unwrap();
    add(&mut total,format!("{prefix}calls"),1);for(k,v)in local{add(&mut total,format!("{prefix}{k}"),v);}
}
pub(crate)fn ordinary<C:CurveConfig<Base=Fq>>(bases:&[Affine<C>],words:&[[u64;4]],parts:usize,width:usize){
    if !ACTIVE.load(Ordering::Acquire){return;}assert!((1..=2).contains(&parts));assert_eq!(bases.len(),words.len());
    let size=bases.len().div_ceil(parts);for p in 0..parts{let range=p*size..((p+1)*size).min(bases.len());publish(&format!("ordinary/part={p}"),&[(&bases[range.clone()],&words[range])],width);}
}
pub(crate)fn fused<C:CurveConfig<Base=Fq>>(k:&[Affine<C>],sk:&[[u64;4]],h:&[Affine<C>],sh:&[[u64;4]],width:usize){
    if !ACTIVE.load(Ordering::Acquire){return;}
    publish("fused-k",&[(k,sk)],width);publish("fused-h",&[(h,sh)],width);publish("fused-combined",&[(k,sk),(h,sh)],width);
}
#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::{Field,Fr,G1Affine};
    fn bit(k:&[u64;4],i:usize)->i64{if i<256{((k[i/64]>>(i%64))&1)as i64}else{0}}
    fn reference(k:&[u64;4],w:usize,width:usize)->i16{let offset=w*width;let mut d=if offset==0{0}else{bit(k,offset-1)};
        for j in 0..width-1{d+=bit(k,offset+j)<<j;}d-=bit(k,offset+width-1)<<(width-1);d as i16}
    #[test]fn centered_digits_match_field_and_bit_reference(){
        let mut cases=vec![[0;4],[1,0,0,0],[65535,0,0,0]];let mut minus=R;minus[0]-=1;cases.push(minus);
        let mut seed=0x43764a2ab4u64;for _ in 0..1024{let mut k=[0;4];for v in &mut k{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;*v=seed;}k[3]&=(1<<61)-1;cases.push(k);let mut c=[0;4];let mut borrow=false;for i in 0..4{let(v,b1)=R[i].overflowing_sub(k[i]);let(v,b2)=v.overflowing_sub(u64::from(borrow));c[i]=v;borrow=b1||b2;}if k!=[0;4]{cases.push(c);}}
        for k in cases{let(c,negative)=center(k);let value=Fr::from_canonical(k).unwrap();let magnitude=Fr::from_canonical(c).unwrap();assert_eq!(if negative{-magnitude}else{magnitude},value);
            for width in 4..=15{let mut rebuilt=Fr::ZERO;let radix=Fr::from(1u64<<width);for w in (0..255usize.div_ceil(width)).rev(){let d=booth_digit(&c,w,width);assert_eq!(d,reference(&c,w,width));rebuilt=rebuilt*radix+if d<0{-Fr::from(d.unsigned_abs()as u64)}else{Fr::from(d as u64)};}assert_eq!(rebuilt,magnitude);}
        }
    }
    #[test]fn observed_partitions_identities_and_fused_conserve_terms(){
        let mut minus=R;minus[0]-=1;let words=[[0;4],[1,0,0,0],minus,[65535,0,0,0]];let mut bases=[G1Affine::GENERATOR;4];bases[2]=G1Affine::IDENTITY;
        let all=compute(&[(&bases,&words)],12);assert_eq!(all["inputs"],4);assert_eq!(all["identity_bases"],1);assert_eq!(all["small_original"],2);assert_eq!(all["small_centered"],2);
        let left=compute(&[(&bases[..2],&words[..2])],12);let right=compute(&[(&bases[2..],&words[2..])],12);let combined=compute(&[(&bases[..2],&words[..2]),(&bases[2..],&words[2..])],12);assert_eq!(all,combined);
        for w in 0..255usize.div_ceil(12){for name in ["original","centered"]{let key=format!("w={w}/{name}/terms");assert_eq!(all[&key],left[&key]+right[&key]);}}
        ordinary(&bases,&words,1,12);assert!(TOTAL.lock().unwrap().is_empty());begin();ordinary(&bases,&words,2,12);fused(&bases[..2],&words[..2],&bases[2..],&words[2..],12);let published=end();assert_eq!(published["ordinary/part=0/width=12/calls"],1);assert_eq!(published["ordinary/part=1/width=12/calls"],1);assert_eq!(published["fused-combined/width=12/inputs"],4);TOTAL.lock().unwrap().clear();
    }
}
