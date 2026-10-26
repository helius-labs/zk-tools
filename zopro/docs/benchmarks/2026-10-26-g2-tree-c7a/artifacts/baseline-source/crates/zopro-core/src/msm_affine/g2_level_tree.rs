//! Probe-only grouped G2 tree. Production query dispatch is unchanged.
use super::g2_phi::{BETA,hb_digit,hb_window,phi_split};
use super::merge_buckets;
use rayon::prelude::*;
use zopro_bn254::{G2Affine,G2Config,G2Projective};

fn component(bases:&[G2Affine],short:&[[u64;4]],high:bool,w:usize)->Vec<G2Affine>{
    const HALF:usize=2048;
    let mut counts=vec![0usize;HALF];
    for (p,s) in bases.iter().zip(short) {
        let d=hb_digit(s,high,w,12);
        if d!=0&&!p.is_identity(){counts[d.unsigned_abs()as usize-1]+=1;}
    }
    let mut starts=Vec::with_capacity(HALF+1);starts.push(0);
    for &n in &counts{starts.push(starts.last().unwrap()+n);}
    let mut positions=starts[..HALF].to_vec();
    let mut order=vec![0u32;*starts.last().unwrap()];
    for (i,(p,s)) in bases.iter().zip(short).enumerate(){
        let d=hb_digit(s,high,w,12);
        if d!=0&&!p.is_identity(){let b=d.unsigned_abs()as usize-1;
            order[positions[b]]=i as u32|if d<0{1<<31}else{0};positions[b]+=1;
        }
    }
    crate::msm::group_affine_signed::<G2Config>(bases,&order,&starts,&counts)
}
fn window(bases:&[G2Affine],short:&[[u64;4]],w:usize)->Vec<G2Affine>{
    // Preserve the accepted narrow top digit's exceptional/indexed behavior.
    if w==10{return hb_window(bases,short,w,12);}
    let mut high=component(bases,short,true,w);
    for p in &mut high{if !p.is_identity(){p.x=p.x.mul_by_fp(&BETA);}}
    let low=component(bases,short,false,w);
    merge_buckets(&mut high,&low);high
}
fn tree_raw(bases:&[G2Affine],short:&mut[[u64;4]])->G2Projective{
    short.par_iter_mut().for_each(|s|*s=phi_split(*s));
    let chunk=bases.len().div_ceil(2);
    let sums:Vec<_>=(0..11).into_par_iter().map(|w|{
        let mut buckets:Vec<_>=(0..2).into_par_iter().map(|part|{
            let start=(part*chunk).min(bases.len());let end=(start+chunk).min(bases.len());
            window(&bases[start..end],&short[start..end],w)
        }).collect();
        let mut combined=buckets.swap_remove(0);
        for b in buckets{merge_buckets(&mut combined,&b);}
        crate::msm::weighted_affine(&combined)
    }).collect();
    let mut result=G2Projective::IDENTITY;
    for sum in sums.iter().rev(){for _ in 0..12{result=result.double();}result+=sum;}
    result
}
pub(crate) fn probe(bases:&[G2Affine],words:&mut[[u64;4]],tree:bool)
    ->Result<(G2Projective,usize),String>
{
    if bases.len()!=words.len(){return Err("component dimensions".into());}
    let (ordinary,peak)=super::workspace::level_tree_peak(bases.len()).ok_or("component geometry/forecast")?;
    if !tree{return Ok((super::g2_phi::raw_exclusive(bases,words),ordinary));}
    let _charge=crate::preparation::Reservation::analysis(Some(peak)).ok_or("component scratch admission denied")?;
    Ok((tree_raw(bases,words),peak))
}

#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::Fr;
    fn inputs(n:usize,kind:usize)->(Vec<G2Affine>,Vec<[u64;4]>,Fr){
        let mut rng=crate::testing::Seeded::new(0x7472_6565+kind as u64);
        let mut points=Vec::with_capacity(n);let mut scalars=Vec::with_capacity(n);
        let mut point=G2Projective::IDENTITY;let mut expected=Fr::ZERO;
        for i in 0..n{point+=G2Affine::GENERATOR;let mut weight=Fr::from((i+1)as u64);let mut p=point;
            if i%17==0{p=G2Projective::IDENTITY;weight=Fr::ZERO;}
            else if i%7==0{p=-p;weight=-weight;}
            else if i%11==0{p=G2Affine::GENERATOR.to_projective();weight=Fr::ONE;}
            let s=match kind{0=>Fr::ZERO,1=>if i%2==0{Fr::ONE}else{-Fr::ONE},2=>Fr::from(4095u64),_=>rng.fr()};
            expected+=weight*s;points.push(p);scalars.push(s.to_canonical());
        }(G2Projective::batch_to_affine(&points),scalars,expected)
    }
    #[test]fn complete_query_matches_generator_oracle_across_workers(){
        for workers in [1,4]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap();
            pool.install(||for n in [0,1,127,1024,4097]{for kind in 0..4{
                let(p,s,sum)=inputs(n,kind);let got=probe(&p,&mut s.clone(),true).unwrap().0;
                assert_eq!(got.to_affine(),(G2Affine::GENERATOR*sum).to_affine(),"workers{workers} n{n} kind{kind}");
            }});
        }
    }
    #[test]fn actual_size_random_query_matches_independent_generator_sum(){
        let pool=rayon::ThreadPoolBuilder::new().num_threads(32).build().unwrap();pool.install(||{
            let(p,s,sum)=inputs(30089,3);let want=(G2Affine::GENERATOR*sum).to_affine();
            for tree in [false,true]{assert_eq!(probe(&p,&mut s.clone(),tree).unwrap().0.to_affine(),want);}
        });
    }
    #[test]fn checked_forecast_covers_empty_and_boundary_and_rejects_geometry(){
        for n in [0,1,1024,30089,65535]{let(a,b)=super::super::workspace::level_tree_peak(n).unwrap();assert!(b>=a);}
        for n in [65536,usize::MAX]{assert!(super::super::workspace::level_tree_peak(n).is_none());}
        assert!(probe(&[],&mut[[0;4]],true).is_err());
    }
}
