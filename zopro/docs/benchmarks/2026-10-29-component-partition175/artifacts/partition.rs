//! Two window tasks own scalar components instead of two point ranges.
use super::*;
#[cfg(test)]
static CALLS:std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);

fn component(bases:&[G2Affine],short:&[[u64;4]],high:bool,w:usize,bits:usize)->Vec<G2Affine>{
    if w+1==127usize.div_ceil(bits)&&126-w*bits<=8 {
        return hb_top_component(bases,short,high,w,bits,126-w*bits);
    }
    super::super::g2_borrowed::try_component(bases,short,high,w,bits)
        .expect("eligible native borrowed G2 path")
}
fn evaluate(bases:&[G2Affine],short:&mut [[u64;4]],bits:usize)->G2Projective{
    assert_eq!(bases.len(),short.len());
    short.par_iter_mut().for_each(|s|*s=phi_split(*s));
    let windows=127usize.div_ceil(bits);
    let sums:Vec<_>=(0..windows).into_par_iter().map(|w|{
        let(mut high,low)=rayon::join(
            ||component(bases,short,true,w,bits),
            ||component(bases,short,false,w,bits));
        for p in &mut high {if !p.is_identity(){p.x=p.x.mul_by_fp(&BETA);}}
        merge_buckets(&mut high,&low);
        crate::msm::weighted_affine(&high)
    }).collect();
    let mut result=G2Projective::IDENTITY;
    for sum in sums.iter().rev(){for _ in 0..bits {result=result.double();} result+=sum;}
    result
}
pub(super) fn try_raw(bases:&[G2Affine],words:&mut [[u64;4]])->Option<G2Projective>{
    if !crate::msm::BatchAdd::<G2Config>::default().borrowed_g2_enabled(){return None;}
    #[cfg(test)]CALLS.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
    Some(evaluate(bases,words,12))
}

#[cfg(test)]mod tests{
    use super::*;
    use zopro_bn254::{Field,Fr};
    fn random(seed:&mut u64)->Fr{
        let mut bytes=[0u8;32];for chunk in bytes.chunks_mut(8){*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}Fr::from_le_bytes_mod_order(&bytes)
    }
    fn fixture(n:usize,adversarial:bool)->(Vec<G2Affine>,Vec<Fr>,Vec<Fr>){
        let g=G2Projective::generator();let mut p=g*Fr::from(12345u64);let mut points=Vec::new();let mut coefficients=Vec::new();let mut scalars=Vec::new();let mut seed=0xd217_9bc5_33a6_1e08;
        for i in 0..n{
            let mut c=Fr::from(i as u64+12345);let mut point=p;p+=g;
            if adversarial {match i%19{0=>{c=Fr::ZERO;point=G2Projective::IDENTITY;},1=>{c=-c;point=-point;},2=>{c=Fr::ONE;point=g;},_=>{}}}
            coefficients.push(c);points.push(point);
            scalars.push(if adversarial{match i%11{0=>Fr::ZERO,1|2=>Fr::ONE,3=>-Fr::ONE,4=>Fr::from(2u64),5=>-Fr::from(2u64),_=>random(&mut seed)}}else{random(&mut seed)});
        }
        (G2Projective::batch_to_affine(&points),coefficients,scalars)
    }
    #[test]fn complete_queries_match_independent_generator(){
        let pools=[1,4].map(|n|rayon::ThreadPoolBuilder::new().num_threads(n).build().unwrap());
        for n in [0,1,31,1024,2049,4097,30089,65535]{for adversarial in [false,true]{
            let(bases,c,s)=fixture(n,adversarial);let expected=G2Projective::generator()*c.iter().zip(&s).fold(Fr::ZERO,|a,(c,s)|a+*c**s);
            for pool in &pools{let mut words=s.iter().map(Fr::to_canonical).collect::<Vec<_>>();let actual=pool.install(||evaluate(&bases,&mut words,12));assert_eq!(actual.to_affine(),expected.to_affine(),"n{n} adversarial{adversarial}");}
        }}
    }
    #[test]fn component_buckets_match_independent_signed_sums(){
        let(bases,c,s)=fixture(4097,true);let short=s.iter().map(|s|phi_split(s.to_canonical())).collect::<Vec<_>>();let bits=12;
        for w in [0,10]{let n=if w==10{1usize<<(126-w*bits)}else{1usize<<(bits-1)};for high in[false,true]{
            let actual=component(&bases,&short,high,w,bits);assert_eq!(actual.len(),n);
            let mut sums=vec![Fr::ZERO;n];for(i,words)in short.iter().enumerate(){let d=hb_digit(words,high,w,bits);if d!=0{sums[d.unsigned_abs()as usize-1]+=if d<0{-c[i]}else{c[i]};}}
            for b in[0,1,3,n/2,n-1]{let expected=G2Projective::generator()*sums[b];assert_eq!(actual[b],expected.to_affine(),"w{w} high{high} b{b}");}
        }}
    }
    #[test]fn compiled_dispatch_matches_policy(){
        use std::sync::atomic::Ordering;
        let(bases,c,s)=fixture(1024,true);let mut words=s.iter().map(Fr::to_canonical).collect::<Vec<_>>();CALLS.store(0,Ordering::Relaxed);
        let actual=super::super::raw_exclusive(&bases,&mut words);let expected=G2Projective::generator()*c.iter().zip(&s).fold(Fr::ZERO,|a,(c,s)|a+*c**s);assert_eq!(actual.to_affine(),expected.to_affine());
        assert_eq!(CALLS.load(Ordering::Relaxed)>0,cfg!(zopro_phi_component_partition));
    }
}
