//! Four-dimensional G2 decomposition with transformations of proof-local buckets.
//! Gnark's BN254 lattice and the GLV/GLS endomorphisms; no key point copies.
use super::{affine_buckets,batch_size,digit,fill_window_impl,merge_buckets,ScalarGroups};
use rayon::prelude::*;
use zopro_bn254::{Fq,Fq2,G2Affine,G2Config,G2Projective};
const MIN_QUERY:usize=1024;
const MAX_QUERY:usize=1<<16;
const U:i128=4965661367192848881;
const BASIS:[[i128;4];4]=[[2*U,U,U+1,-U],[-U,U,U,2*U+1],[-U-1,U,-U,-2*U],[2*U+1,U+1,-U,U]];
const RECIP:[[u64;4];4]=[
    [0x46f4bda995d51bb1, 0x8e5da66fc7184ae, 0x9e80318ab0d92b93, 0x0],
    [0x2dff291532e42728, 0x55b4ca7ba3e5577f, 0x9e80318ab0d92b95, 0x0],
    [0x71c4c43fac4db00, 0x55b4ca7ba3e55782, 0x9e80318ab0d92b95, 0x0],
    [0xc170977dcef3cd3f, 0x55b4ca7ba3e5577d, 0x9e80318ab0d92b95, 0x0]
];
const NEGATIVE:[bool;4]=[false,true,true,false];
const BETA:Fq=Fq::from_decimal("21888242871839275220042445260109153167277707414472061641714758635765020556616");
const PSI_X:Fq2=Fq2::new(Fq::from_decimal("21575463638280843010398324269430826099269044274347216827212613867836435027261"),Fq::from_decimal("10307601595873709700152284273816112264069230130616436755625194854815875713954"));
const PSI_Y:Fq2=Fq2::new(Fq::from_decimal("2821565182194536844548159561693502659359617185244120367078079554186484126554"),Fq::from_decimal("3505843767911556378687030309984248845540243509899259641013678093033130930403"));
#[inline(always)]
fn quotient_low(s:[u64;4],c:[u64;4],negative:bool)->u128 {
    let mut p=[0u64;8];
    for i in 0..4 {
        let mut carry=0u128;
        for j in 0..4 {
            let t=s[i]as u128*c[j]as u128+p[i+j]as u128+carry;
            p[i+j]=t as u64;carry=t>>64;
        }
        p[i+4]=carry as u64;
    }
    let q=p[4]as u128|((p[5]as u128)<<64);
    if negative {0u128.wrapping_sub(q).wrapping_sub(u128::from(p[..4].iter().any(|&v|v!=0)))}else{q}
}
#[inline]
fn split(s:[u64;4])->[u64;5] {
    let q: [u128;4]=core::array::from_fn(|i|quotient_low(s,RECIP[i],NEGATIVE[i]));
    let low=s[0]as u128|((s[1]as u128)<<64);
    let mut result=[0u64;5];
    for j in 0..4 {
        let mut k=if j==0 {low}else{0};
        for i in 0..4 {k=k.wrapping_sub(q[i].wrapping_mul(BASIS[i][j]as u128));}
        let signed=k as i128;let magnitude=signed.unsigned_abs();
        // The exact lattice residual is below 2^65, so its low 128 bits
        // uniquely identify the signed value; no full-width products needed.
        assert!(magnitude<1u128<<65);
        result[j]=magnitude as u64;
        result[4]|=((magnitude>>64)as u64)<<j|u64::from(signed<0)<<(j+4);
    }
    result
}
#[inline]
fn component_digit(s:&[u64;5],component:usize,w:usize,bits:usize)->i16 {
    let d=digit(&[s[component],(s[4]>>component)&1,0,0],w,bits);
    if (s[4]>>(component+4))&1!=0 {-d}else{d}
}
fn transform(point:&mut G2Affine,component:usize) {
    if point.is_identity(){return;}
    if component&1!=0 {point.x=point.x.mul_by_fp(&BETA);}
    if component&2!=0 {point.x=point.x.conjugate()*PSI_X;point.y=point.y.conjugate()*PSI_Y;}
}
fn component_buckets(bases:&[G2Affine],short:&[[u64;5]],component:usize,w:usize,bits:usize)->Vec<G2Affine> {
    let mut state=fill_window_impl::<G2Config,false,false>(bases,short.iter().map(|s|component_digit(s,component,w,bits)),bits,batch_size(bits).max(32));
    assert!(state.batch.is_empty()&&state.queue.is_empty());
    for point in &mut state.affine {transform(point,component);}
    for point in &mut state.projective {
        if component&1!=0 {point.x=point.x.mul_by_fp(&BETA);}
        if component&2!=0 {point.x=point.x.conjugate()*PSI_X;point.y=point.y.conjugate()*PSI_Y;point.z=point.z.conjugate();}
    }
    affine_buckets(state)
}
fn window(bases:&[G2Affine],short:&[[u64;5]],w:usize,bits:usize)->Vec<G2Affine> {
    let mut combined=component_buckets(bases,short,0,w,bits);
    for component in 1..4 {
        let part=component_buckets(bases,short,component,w,bits);
        merge_buckets(&mut combined,&part);
    }
    combined
}
fn run(bases:&[G2Affine],words:&[[u64;4]],bits:usize,parts:usize)->G2Projective {
    assert_eq!(bases.len(),words.len());
    let short:Vec<_>=words.par_iter().map(|s|split(*s)).collect();
    let chunk=bases.len().div_ceil(parts);let windows=66usize.div_ceil(bits);
    let sums:Vec<_>=(0..windows).into_par_iter().map(|w| {
        let mut pieces:Vec<_>=(0..parts).into_par_iter().map(|p| {
            let start=(p*chunk).min(bases.len());let end=(start+chunk).min(bases.len());
            window(&bases[start..end],&short[start..end],w,bits)
        }).collect();
        let mut combined=pieces.swap_remove(0);
        for piece in pieces {merge_buckets(&mut combined,&piece);}
        crate::msm::weighted_affine(&combined)
    }).collect();
    let mut result=G2Projective::IDENTITY;
    for sum in sums.iter().rev(){for _ in 0..bits {result=result.double();}result+=sum;}
    result
}
fn params()->(usize,usize) {
    let bits=std::env::var("ZOPRO_CPU_G2_GLS4_WIDTH").ok().and_then(|v|v.parse().ok()).unwrap_or(11usize).clamp(10,13);
    let parts=std::env::var("ZOPRO_CPU_G2_GLS4_PARTS").ok().and_then(|v|v.parse().ok()).unwrap_or(2usize).clamp(1,4);
    (bits,parts)
}
pub(super) fn raw(bases:&[G2Affine],words:&[[u64;4]])->G2Projective {
    if !(MIN_QUERY..MAX_QUERY).contains(&bases.len()){return super::g2_horner::raw(bases,words);}
    let(bits,parts)=params();run(bases,words,bits,parts)
}
pub(super) fn grouped(bases:&[G2Affine],groups:&ScalarGroups)->G2Projective {
    if !(MIN_QUERY..MAX_QUERY).contains(&bases.len())||!(MIN_QUERY..MAX_QUERY).contains(&groups.values.len()){return super::g2_horner::grouped(bases,groups);}
    let points=crate::msm::group_affine(bases,&groups.order,&groups.starts,&groups.counts);
    let(bits,parts)=params();run(&points,&groups.values,bits,parts)
}
#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::Fr;
    fn magnitude(s:&[u64;5],component:usize)->Fr {
        let lo=Fr::from(s[component]);let high=Fr::from((s[4]>>component)&1)*Fr::from(2).pow(&[64]);
        let value=lo+high;if(s[4]>>(component+4))&1!=0 {-value}else{value}
    }
    #[test]
    fn gls4_split_reconstructs_boundaries_and_random_scalars() {
        let l=Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
        let mu=Fr::from_decimal("147946756881789318990833708069417712966");
        let mut values=vec![Fr::ZERO,Fr::ONE,-Fr::ONE,l,mu,l-Fr::ONE,mu-Fr::ONE,l+Fr::ONE,mu+Fr::ONE];
        for bit in 0..254 {let n=Fr::from(2).pow(&[bit]);values.extend([n-Fr::ONE,n,n+Fr::ONE]);}
        values.extend((0..8192u64).map(|i|Fr::from(i+37).pow(&[137])));
        for value in values {let short=split(value.to_canonical());assert_eq!(short[4]>>8,0);let c: [Fr;4]=core::array::from_fn(|i|magnitude(&short,i));assert_eq!(c[0]+c[1]*l+c[2]*mu+c[3]*l*mu,value);}
    }
    #[test]
    fn gls4_endomorphisms_match_scalar_multiplication() {
        let l=Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");let mu=Fr::from_decimal("147946756881789318990833708069417712966");
        for i in 0..24u64 {let point=(G2Affine::GENERATOR*Fr::from(i)).to_affine();for component in 0..4 {let mut actual=point;transform(&mut actual,component);let scalar=(if component&1!=0 {l}else{Fr::ONE})*(if component&2!=0 {mu}else{Fr::ONE});assert_eq!(actual,(point*scalar).to_affine());}}
    }
    fn bases(n:usize)->(Vec<G2Affine>,Vec<Fr>) {
        let weights:Vec<_>=(0..n).map(|i|if i%17==0 {Fr::ZERO}else if i%3==0 {-Fr::from(i as u64%23+3)}else{Fr::from(i as u64%23+3)}).collect();
        (weights.iter().map(|&s|(G2Affine::GENERATOR*s).to_affine()).collect(),weights)
    }
    #[test]
    fn gls4_raw_matches_weighted_generator_for_all_widths_parts_and_boundaries() {
        let(bases,weights)=bases(1027);let mu=Fr::from_decimal("147946756881789318990833708069417712966");
        for kind in 0..5 {
            let values:Vec<_>=(0..bases.len()).map(|i|match kind {0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>mu+Fr::from(i as u64%3)-Fr::ONE,_=>Fr::from(i as u64+91).pow(&[127])}).collect();
            let words:Vec<_>=values.iter().map(Fr::to_canonical).collect();let before=words.clone();let scalar=values.iter().zip(&weights).fold(Fr::ZERO,|acc,(&s,&w)|acc+s*w);let expected=(G2Affine::GENERATOR*scalar).to_affine();
            for bits in 10..=13 {for parts in [1,2,4] {assert_eq!(run(&bases,&words,bits,parts).to_affine(),expected,"kind={kind} bits={bits} parts={parts}");}}
            assert_eq!(words,before,"B1 canonical scalars are borrowed");
        }
    }
    #[test]
    fn gls4_grouped_preserves_shared_values() {
        let(bases,weights)=bases(4096);let values:Vec<_>=(0..bases.len()).map(|i|Fr::from((i%1500+101)as u64).pow(&[137])).collect();let words:Vec<_>=values.iter().map(Fr::to_canonical).collect();let groups=super::super::scalar_groups(&words).unwrap();let before=groups.values.clone();assert!(groups.values.len()>=MIN_QUERY);
        let points=crate::msm::group_affine(&bases,&groups.order,&groups.starts,&groups.counts);let expected=(G2Affine::GENERATOR*values.iter().zip(&weights).fold(Fr::ZERO,|acc,(&s,&w)|acc+s*w)).to_affine();assert_eq!(run(&points,&groups.values,11,2).to_affine(),expected);assert_eq!(grouped(&bases,&groups).to_affine(),expected);assert_eq!(groups.values,before);
    }
}
