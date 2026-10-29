// Fused K/H through signed GLV components and borrowed original points.
use super::*;
use super::balanced::{digit_at, Span};
use zopro_bn254::{G1Affine, G1Config, G1Projective};
const COUNT: usize = 10;
const BITS: usize = 127;
const BETA: Fq = Fq::from_decimal("2203960485148121921418603742825762020974279258880205651966");
fn spans() -> [Span; COUNT] {
    core::array::from_fn(|i| Span {start:i*BITS/COUNT,width:(i+1)*BITS/COUNT-i*BITS/COUNT})
}
#[inline(always)]
fn component(s: &[u64;4], high: bool, span: Span) -> i16 {
    let i=usize::from(high)*2;
    let d=digit_at(&[s[i],s[i+1]&0x7fffffffffffffff,0,0],span);
    if s[i+1]>>63!=0 {-d} else {d}
}
fn feed<'a>(state:&mut Window<'a,G1Config>,points:&'a[G1Affine],short:&[[u64;4]],high:bool,span:Span,size:usize) {
    for (point,s) in points.iter().zip(short) {
        let d=component(s,high,span);
        if d==0||point.is_identity(){continue;}
        let bucket=d.unsigned_abs()as usize-1;
        let encoded=bucket as u32|if d<0{1<<31}else{0};
        if state.busy(bucket) {
            state.queue.push((encoded,point));
            if state.queue.len()==size-1{state.flush_queue();}
            continue;
        }
        state.add(encoded,point);
        if state.batch.len()==size{state.execute();state.take_from_queue();}
    }
}
fn window(k:&[G1Affine],sk:&[[u64;4]],h:&[G1Affine],sh:&[[u64;4]],span:Span)->Vec<G1Affine> {
    let size=batch_size(span.width).max(32);
    let mut state=Window::<G1Config>::new(1usize<<(span.width-1),size);
    feed(&mut state,k,sk,true,span,size);
    feed(&mut state,h,sh,true,span,size);
    state.execute();state.flush_queue();
    for p in &mut state.affine {if !p.is_identity(){p.x*=BETA;}}
    for p in &mut state.projective {p.x*=BETA;}
    feed(&mut state,k,sk,false,span,size);
    feed(&mut state,h,sh,false,span,size);
    state.finish()
}
fn run(k:&[G1Affine],sk:&[[u64;4]],h:&[G1Affine],sh:&[[u64;4]])->G1Projective {
    let short:Vec<_>=sk.par_iter().chain(sh.par_iter()).map(|s|super::super::g2_phi::phi_split(*s)).collect();
    let (sk,sh)=short.split_at(k.len());
    let sums:Vec<_>=spans().par_iter().map(|&span|{
        let mut buckets:Vec<_>=(0..2usize).into_par_iter().map(|part|{
            let range=|n:usize| {let chunk=n.div_ceil(2);let start=(part*chunk).min(n);start..(start+chunk).min(n)};
            let (kr,hr)=(range(k.len()),range(h.len()));
            window(&k[kr.clone()],&sk[kr],&h[hr.clone()],&sh[hr],span)
        }).collect();
        let mut combined=buckets.swap_remove(0);
        for other in buckets {super::super::merge_buckets(&mut combined,&other);}
        crate::msm::weighted_affine(&combined)
    }).collect();
    let mut result=G1Projective::IDENTITY;
    for (span,sum) in spans().iter().zip(&sums).rev(){
        for _ in 0..span.width {result=result.double();}
        result+=sum;
    }
    result
}
pub(super) fn try_fused(k:&[G1Affine],sk:&[[u64;4]],h:&[G1Affine],sh:&[[u64;4]])->Option<G1Projective> {
    if k.len()!=sk.len()||h.len()!=sh.len()||!super::super::fused_kh::eligible(true,Fr::ONE,k.len(),h.len()){return None;}
    let _reservation=crate::preparation::Reservation::analysis(super::super::workspace::glv_kh_peak(k.len(),h.len(),rayon::current_num_threads().max(1)))?;
    Some(run(k,sk,h,sh))
}

#[cfg(test)]mod tests {
    use super::*;
    use crate::testing::Seeded;
    #[test]fn signed_split_matches_g1_endomorphism_and_balanced_digits() {
        let lambda=Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let mut mapped=G1Affine::GENERATOR;mapped.x*=BETA;
        assert_eq!(mapped,(G1Affine::GENERATOR*lambda).to_affine());
        let mut rng=Seeded::new(0x676c766b68646967);
        let mut values=vec![Fr::ZERO,Fr::ONE,-Fr::ONE];
        for bit in 0..254 {let a=Fr::from(2u64).pow(&[bit]);values.extend([a,a-Fr::ONE,a+Fr::ONE]);}
        values.extend((0..4096).map(|_|rng.fr()));
        let signed=|s:[u64;4],high:bool|{let i=usize::from(high)*2;let value=s[i]as u128|((s[i+1]&0x7fffffffffffffff)as u128)<<64;if s[i+1]>>63!=0{-(value as i128)}else{value as i128}};
        for value in values {
            let short=super::super::super::g2_phi::phi_split(value.to_canonical());
            let mut components=[Fr::ZERO;2];
            for high in [false,true] {
                let mut reconstructed=0i128;
                for span in spans(){let d=component(&short,high,span);assert!(d.unsigned_abs()as usize<=1usize<<(span.width-1));reconstructed+=(d as i128)<<span.start;}
                assert_eq!(reconstructed,signed(short,high));
                let magnitude=reconstructed.unsigned_abs();
                assert!(magnitude<1u128<<126);
                let s=Fr::from_canonical([magnitude as u64,(magnitude>>64)as u64,0,0]).unwrap();
                components[usize::from(high)]=if reconstructed<0{-s}else{s};
            }
            assert_eq!(components[0]+lambda*components[1],value);
        }
        assert_eq!(spans().iter().map(|s|1usize<<(s.width-1)).sum::<usize>(),34816);
    }
    fn points(n:usize,offset:u64)->(Vec<G1Affine>,Vec<Fr>) {
        let mut p=G1Affine::GENERATOR*Fr::from(offset);let mut points=Vec::new();let mut weights=Vec::new();
        for i in 0..n {p+=G1Affine::GENERATOR;let(mut q,mut w)=(p,Fr::from(offset+i as u64+1));
            if i%17==0 {q=G1Projective::IDENTITY;w=Fr::ZERO;}else if i%3==0 {q=-q;w=-w;}else if i%11==0 {q=G1Affine::GENERATOR.to_projective();w=Fr::ONE;}
            points.push(q);weights.push(w);
        }
        (G1Projective::batch_to_affine(&points),weights)
    }
    #[test]fn combined_signed_glv_queries_match_generator_with_threads_and_tails() {
        let mut rng=Seeded::new(0x676c766b6870726f);
        for threads in [1,4] {let pool=rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();pool.install(||{
            for (nk,nh) in [(0,0),(3,7),(1024,1025),(8191,8192),(30529,32768)] {
                let(k,kw)=points(nk,7);let(h,hw)=points(nh,1_000_003);
                for kind in 0..5 {let mut expected=Fr::ZERO;
                    let mut words=|weights:&[Fr]|weights.iter().enumerate().map(|(i,&w)|{
                        let s=match kind {0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>rng.fr(),_=>{let b=[1,11,12,13,63,64,125,126,127,128,251,253][i%12];let x=Fr::from(2u64).pow(&[b]);if i%2==0{x-Fr::ONE}else{x+Fr::ONE}}};
                        expected+=s*w;s.to_canonical()
                    }).collect::<Vec<_>>();
                    let(sk,sh)=(words(&kw),words(&hw));let before=(sk.clone(),sh.clone());
                    assert_eq!(run(&k,&sk,&h,&sh).to_affine(),(G1Affine::GENERATOR*expected).to_affine(),"threads={threads} nk={nk} nh={nh} kind={kind}");
                    assert_eq!((sk,sh),before);
                }
            }
        });}
    }
    #[test]fn resource_bounds_and_dimension_fallback() {
        assert!(super::super::super::workspace::glv_kh_peak(usize::MAX,7,32).is_none());
        assert!(super::super::super::workspace::glv_kh_peak(30529,32768,32).unwrap()>32*(30529+32768));
        assert!(try_fused(&[],&[],&[],&[]).is_none());
        assert!(try_fused(&[G1Affine::GENERATOR],&[],&[],&[]).is_none());
    }
}
