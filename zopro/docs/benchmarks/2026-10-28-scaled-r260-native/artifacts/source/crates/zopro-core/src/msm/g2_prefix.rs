//! G2 inversion prefix/norm records; native experimental path.
use super::*;
pub(super) fn apply<C:CurveConfig<Base=Fq2>,const FAST:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut Fq2AddScratch,
) {
    #[cfg(all(zopro_g2_scaled_r260,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
    if FAST && batch.len()%LANES==0 {return g2_scaled::apply(buckets,batch,&mut s.scaled_prefix_records);}
    let full=batch.len()/LANES*LANES;
    let mut chains=FqBatch::one();
    s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].x));
        let bx=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let norm=Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains,norm]);chains=chains.mul(&norm);
    }
    s.tail.clear();
    let mut inverse = if let Some(value)=direct_chain_inverse(&chains,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..batch.len() {
            let (index, b) = batch[k];
            let a = buckets[index as usize];
            let denominator = b.x - a.x;
            let norm = denominator.c0.square() + denominator.c1.square();
            let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
            tail[k % LANES] *= norm;
            s.tail
                .push([a.x, a.y, a.x + b.x, Fq2::new(norm, Fq::ZERO), numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
            let slope = numerator.mul_by_fp(&inverse[k % LANES]);
            inverse[k % LANES] *= norm.c0;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch[k].0 as usize] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
    };
    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let a:[Affine<C>;LANES]=core::array::from_fn(|i|buckets[batch[k+i].0 as usize]);
        let b:[Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);
        let ax=fq2_pack(a.map(|p|p.x));let ay=fq2_pack(a.map(|p|p.y));
        let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(b.map(|p|p.y));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));
        let factor=s.prefix_records[j][0].mul(&inverse);
        inverse=inverse.mul(&s.prefix_records[j][1]);
        let slope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);
        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));
        let y=Fq2::lanes_sub(&Fq2::lanes_mul(&slope,&Fq2::lanes_sub(&ax,&x)),&ay);
        let(x,y)=(fq2_unpack(x),fq2_unpack(y));
        for i in 0..LANES {buckets[batch[k+i].0 as usize]=Affine::new(x[i],y[i]);}
    }
}

#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::G2Config;
    fn points(n:usize,offset:u64)->Vec<Affine<G2Config>>{let g=Projective::<G2Config>::generator();let mut p=g.mul(&Fr::from(offset));let mut out=Vec::new();for _ in 0..n{out.push(p);p+=g;}Projective::<G2Config>::batch_to_affine(&out)}
    #[test]fn g2_prefix_complete_curve_tails_reuse(){
        for n in [0,1,7,8,9,15,16,80,200,352,504]{
            let size=(n+1usize).next_power_of_two();let input=points(size,7);let incoming=points(n,50007);
            for canonical in [false,true]{for vector_inverse in [false,true]{
                let batch:Vec<_>=incoming.iter().enumerate().map(|(i,&p)|((i*37%size)as u32,if i%3==0{-p}else{p})).collect();
                let mut actual=input.clone();let mut expected=input.clone();let mut s=Fq2AddScratch{canonical,vector_inverse,..Default::default()};
                for _ in 0..3{for &(id,p)in &batch{let i=id as usize;assert_ne!(expected[i].x,p.x);expected[i]=(expected[i]+p).to_affine();}
                    if canonical{apply::<G2Config,true>(&mut actual,&batch,&mut s)}else{apply::<G2Config,false>(&mut actual,&batch,&mut s)}
                    assert_eq!(actual,expected);assert_eq!(s.groups.capacity(),0);
                }
            }}
        }
    }
    #[test]fn g2_prefix_dispatch_and_control(){
        let input=points(16,7);let incoming=points(9,50007);let batch:Vec<_>=incoming.into_iter().enumerate().map(|(i,p)|(i as u32,p)).collect();
        let expected:Vec<_>=input.iter().enumerate().map(|(i,p)|if i<9{(*p+batch[i].1).to_affine()}else{*p}).collect();
        for enabled in [false,true]{let mut actual=input.clone();let mut s=Fq2AddScratch{canonical:true,vector_inverse:true,prefix_enabled:enabled,..Default::default()};stream_g2_impl::<G2Config,true>(&mut actual,&batch,&mut s);assert_eq!(actual,expected);if enabled{assert_eq!(s.groups.capacity(),0);assert!(!s.prefix_records.is_empty());}else{assert_eq!(s.prefix_records.capacity(),0);assert!(!s.groups.is_empty());}}
    }
}
