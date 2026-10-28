//! Proof-local tagged magnitudes; only the exclusive G1 caller owns these words.
use rayon::prelude::*;
use zopro_bn254::{Affine,CurveConfig,Field,Fq,Fr,G1Affine,G1Config,G1Projective,Projective};
use super::{borrowed_pending,scalar_groups};
const SIGN:u64=1<<63;
const R:[u64;4]=[0x43e1f593f0000001,0x2833e84879b97091,0xb85045b68181585d,0x30644e72e131a029];
fn encode_one(k:&mut[u64;4]){
    debug_assert!(k.iter().rev().cmp(R.iter().rev()).is_lt());let mut c=[0;4];let mut borrow=false;
    for i in 0..4{let(v,b1)=R[i].overflowing_sub(k[i]);let(v,b2)=v.overflowing_sub(u64::from(borrow));c[i]=v;borrow=b1||b2;}
    debug_assert!(!borrow);if c.iter().rev().cmp(k.iter().rev()).is_lt(){*k=c;k[3]|=SIGN;}
}
fn encode(words:&mut[[u64;4]]){words.par_iter_mut().with_min_len(128).for_each(encode_one);}
#[inline(always)]pub(super)fn digit(s:&[u64;4],w:usize,width:usize)->i16{
    let mut magnitude=*s;let negative=magnitude[3]&SIGN!=0;magnitude[3]&=!SIGN;
    let d=super::digit(&magnitude,w,width);if negative{-d}else{d}
}
pub(super)fn top_window<C:CurveConfig<Base=Fq>>(bases:&[Affine<C>],words:&[[u64;4]],w:usize,width:usize)->Option<Projective<C>>{
    let windows=255usize.div_ceil(width);if bases.len()<1024||w+1!=windows{return None;}
    let bits=253usize.checked_sub(w*width)?;if bits>8||!crate::simd::cpu_option("ZOPRO_CPU_TOP_TREE"){return None;}
    let buckets=1usize<<bits;let mut counts=vec![0usize;buckets];
    for(base,s)in bases.iter().zip(words){let d=digit(s,w,width);let magnitude=d.unsigned_abs()as usize;debug_assert!(magnitude<=buckets);if d!=0&&!base.is_identity(){counts[magnitude-1]+=1;}}
    let mut starts=Vec::with_capacity(buckets+1);starts.push(0);for count in counts{starts.push(starts.last().unwrap()+count);}
    if *starts.last().unwrap()==0{return Some(Projective::<C>::IDENTITY);}
    let mut positions=starts[..buckets].to_vec();let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,(base,s))in bases.iter().zip(words).enumerate(){let d=digit(s,w,width);if d!=0&&!base.is_identity(){let b=d.unsigned_abs()as usize-1;order[positions[b]]=i as u32|if d<0{1<<31}else{0};positions[b]+=1;}}
    Some(crate::msm::weighted_affine(&crate::msm::reduce_indexed_buckets_signed(bases,&order,&starts)))
}
fn run(bases:&[G1Affine],words:&mut[[u64;4]])->G1Projective{
    if !(1024..65536).contains(&bases.len()){return super::msm_raw(bases,words);}
    let width=std::env::var("ZOPRO_CPU_DEFER_SIGN_WIDTH").ok().and_then(|v|v.parse().ok()).unwrap_or(13usize).clamp(10,14);
    let parts=std::env::var("ZOPRO_CPU_DEFER_SIGN_PARTS").ok().and_then(|v|v.parse().ok()).unwrap_or(2usize).clamp(1,2);
    encode(words);
    if std::env::var_os("ZOPRO_CPU_CENTER_TRACE").is_some(){eprintln!("CENTER_G1 n={} width={width} parts={parts}",bases.len());}
    borrowed_pending::msm_centered(bases,words,parts,width)
}
pub(super)fn exclusive(bases:&[G1Affine],words:&mut[[u64;4]])->G1Projective{
    assert_eq!(bases.len(),words.len());
    if std::env::var("ZOPRO_CPU_CENTER_G1").as_deref()!=Ok("1")||!(1024..65536).contains(&bases.len())||!borrowed_pending::enabled(){return super::msm_canonical(bases,words);}
    if let Some(mut groups)=crate::simd::cpu_option("ZOPRO_CPU_DEDUP").then(||scalar_groups(words)).flatten(){
        let points=crate::msm::group_affine(bases,&groups.order,&groups.starts,&groups.counts);run(&points,&mut groups.values)
    }else{run(bases,words)}
}
#[cfg(test)]mod tests{
    use super::*;use crate::testing::Seeded;
    #[test]fn tagged_centered_digits_rebuild_original_coefficients(){
        let mut rng=Seeded::new(32);let mut values=vec![Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(65535u64)];values.extend((0..1024).map(|_|rng.fr()));
        let mut words:Vec<_>=values.iter().map(Fr::to_canonical).collect();let address=words.as_ptr();let capacity=words.capacity();encode(&mut words);assert_eq!(words.as_ptr(),address);assert_eq!(words.capacity(),capacity);
        for(&s,wds)in values.iter().zip(&words){for width in 4..=15{let mut value=Fr::ZERO;for w in(0..255usize.div_ceil(width)).rev(){let d=digit(wds,w,width);value=value*Fr::from(1u64<<width)+if d<0{-Fr::from(d.unsigned_abs()as u64)}else{Fr::from(d as u64)};}assert_eq!(value,s);}}
    }
    #[test]fn signed_indexed_top_reduction_handles_complete_curve_cases(){
        let mut rng=Seeded::new(0x1233);let mut points=vec![];let mut weights=vec![];
        for i in 0..2057{let weight=if i%17==0{Fr::ZERO}else if i%3==0{Fr::ONE}else{rng.fr()};points.push((G1Affine::GENERATOR*weight).to_affine());weights.push(weight);}
        let starts=[0,0,1,17,2057];let order:Vec<_>=(0..2057).map(|i|i as u32|if i%2==0{1<<31}else{0}).collect();
        let got=crate::msm::reduce_indexed_buckets_signed::<G1Config>(&points,&order,&starts);
        for(j,range)in starts.windows(2).enumerate(){let expected=(range[0]..range[1]).fold(Fr::ZERO,|sum,i|sum+if i%2==0{-weights[i]}else{weights[i]});assert_eq!(got[j],(G1Affine::GENERATOR*expected).to_affine());}
    }
    #[test]fn centered_borrowed_msms_match_independent_weighted_generator(){
        let mut rng=Seeded::new(0xa132);let pool=rayon::ThreadPoolBuilder::new().num_threads(4).build().unwrap();
        for n in[1024,2057]{let mut point=G1Projective::IDENTITY;let mut raw=vec![];let mut weights=vec![];
            for i in 0..n{point+=G1Affine::GENERATOR;let weight=if i%17==0{Fr::ZERO}else if i%3==0{-Fr::from((i+1)as u64)}else{Fr::from((i+1)as u64)};raw.push(if weight==Fr::ZERO{G1Projective::IDENTITY}else if i%3==0{-point}else{point});weights.push(weight);}
            let bases=G1Projective::batch_to_affine(&raw);
            for mode in 0..4{let values:Vec<_>=(0..n).map(|i|match mode{0=>rng.fr(),1=>if i%2==0{Fr::ONE}else{-Fr::ONE},2=>Fr::from((i%65536)as u64),_=>if i%7==0{Fr::ZERO}else{-rng.fr()}}).collect();let expected=(G1Affine::GENERATOR*values.iter().zip(&weights).fold(Fr::ZERO,|s,(a,b)|s+*a**b)).to_affine();
                let mut words:Vec<_>=values.iter().map(Fr::to_canonical).collect();encode(&mut words);
                for width in 10..=14{for parts in[1,2]{let got=pool.install(||borrowed_pending::msm_centered(&bases,&words,parts,width));assert_eq!(got.to_affine(),expected,"n={n} mode={mode} width={width} parts={parts}");}}
            }
        }
    }
}
