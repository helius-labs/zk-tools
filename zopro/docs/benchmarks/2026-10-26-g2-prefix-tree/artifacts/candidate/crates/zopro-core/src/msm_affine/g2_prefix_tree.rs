//! Component-only kernel/storage ablation, with production dispatch untouched.
use super::g2_phi::{BETA,hb_digit,hb_window,phi_split};
use super::merge_buckets;
use rayon::prelude::*;
use zopro_bn254::{G2Affine,G2Projective};

fn component(bases:&[G2Affine],short:&[[u64;4]],high:bool,w:usize)->Vec<G2Affine>{
    const HALF:usize=2048;
    let mut counts=vec![0usize;HALF];
    for(p,s)in bases.iter().zip(short){let d=hb_digit(s,high,w,12);if d!=0&&!p.is_identity(){counts[d.unsigned_abs()as usize-1]+=1;}}
    let mut starts=Vec::with_capacity(HALF+1);starts.push(0);
    for &n in &counts{starts.push(starts.last().unwrap()+n);}
    let mut positions=starts[..HALF].to_vec();let mut order=vec![0u32;*starts.last().unwrap()];
    for(i,(p,s))in bases.iter().zip(short).enumerate(){let d=hb_digit(s,high,w,12);if d!=0&&!p.is_identity(){let b=d.unsigned_abs()as usize-1;order[positions[b]]=i as u32|if d<0{1<<31}else{0};positions[b]+=1;}}
    crate::msm::group_g2_prefix(bases,&order,&starts,&counts)
}
fn window(bases:&[G2Affine],short:&[[u64;4]],w:usize)->Vec<G2Affine>{
    if w==10{return hb_window(bases,short,w,12);}
    let mut high=component(bases,short,true,w);
    for p in &mut high{if !p.is_identity(){p.x=p.x.mul_by_fp(&BETA);}}
    let low=component(bases,short,false,w);merge_buckets(&mut high,&low);high
}
fn raw(bases:&[G2Affine],short:&mut[[u64;4]])->G2Projective{
    short.par_iter_mut().for_each(|s|*s=phi_split(*s));let chunk=bases.len().div_ceil(2);
    let sums:Vec<_>=(0..11).into_par_iter().map(|w|{
        let mut buckets:Vec<_>=(0..2).into_par_iter().map(|p|{let start=(p*chunk).min(bases.len());let end=(start+chunk).min(bases.len());window(&bases[start..end],&short[start..end],w)}).collect();
        let mut combined=buckets.swap_remove(0);for b in buckets{merge_buckets(&mut combined,&b);}crate::msm::weighted_affine(&combined)
    }).collect();let mut result=G2Projective::IDENTITY;for sum in sums.iter().rev(){for _ in 0..12{result=result.double();}result+=sum;}result
}
pub(crate) fn probe(bases:&[G2Affine],words:&mut[[u64;4]])->Result<(G2Projective,usize),String>{
    if bases.len()!=words.len(){return Err("component dimensions".into());}
    if !crate::simd::cpu_option("ZOPRO_CPU_DIRECT")||std::env::var("ZOPRO_CPU_G2_PREFIX_RECORDS").as_deref()!=Ok("1"){return Err("native direct prefix configuration required".into());}
    let peak=super::workspace::prefix_tree_peak(bases.len()).ok_or("component geometry/forecast")?;
    let _charge=crate::preparation::Reservation::analysis(Some(peak)).ok_or("component scratch admission denied")?;
    Ok((raw(bases,words),peak))
}

#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::Fr;
    fn data(n:usize,kind:usize)->(Vec<G2Affine>,Vec<[u64;4]>,Fr){
        let mut rng=crate::testing::Seeded::new(0x7072_6566+kind as u64);let mut points=Vec::new();let mut scalars=Vec::new();let mut p=G2Projective::IDENTITY;let mut sum=Fr::ZERO;
        for i in 0..n{p+=G2Affine::GENERATOR;let mut q=p;let mut weight=Fr::from((i+1)as u64);if i%17==0{q=G2Projective::IDENTITY;weight=Fr::ZERO;}else if i%7==0{q=-q;weight=-weight;}else if i%11==0{q=G2Affine::GENERATOR.to_projective();weight=Fr::ONE;}let s=match kind{0=>Fr::ZERO,1=>if i%2==0{Fr::ONE}else{-Fr::ONE},2=>Fr::from(4095u64),_=>rng.fr()};points.push(q);scalars.push(s.to_canonical());sum+=weight*s;}(G2Projective::batch_to_affine(&points),scalars,sum)
    }
    #[test]fn prefix_query_complete_curve_generator_oracles(){for workers in [1,4]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();pool.install(||for n in [0,1,127,1024,4097]{for kind in 0..4{let(p,s,sum)=data(n,kind);assert_eq!(probe(&p,&mut s.clone()).unwrap().0.to_affine(),(G2Affine::GENERATOR*sum).to_affine());}});}}
    #[test]fn actual_size_prefix_matches_stream_and_generator(){let pool=rayon::ThreadPoolBuilder::new().num_threads(32).build().unwrap();pool.install(||{let(p,s,sum)=data(30089,3);let want=(G2Affine::GENERATOR*sum).to_affine();assert_eq!(probe(&p,&mut s.clone()).unwrap().0.to_affine(),want);assert_eq!(super::super::g2_phi::raw_exclusive(&p,&mut s.clone()).to_affine(),want);});}
    #[test]fn prefix_forecast_and_geometry_are_checked(){for n in [0,1,1024,30089,65535]{let p=super::super::workspace::prefix_tree_peak(n).unwrap();let(old,_)=super::super::workspace::level_tree_peak(n).unwrap();assert!(p>=old);}for n in [65536,usize::MAX]{assert!(super::super::workspace::prefix_tree_peak(n).is_none());}assert!(probe(&[],&mut[[0;4]]).is_err());}
}
