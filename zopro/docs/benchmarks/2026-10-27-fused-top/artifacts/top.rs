// Treat the existing K/H slices as one indexed input without copying points.
// The short positive top digit shares its bucket reduction across both terms.
fn top(k: &[G1Affine], sk: &[[u64;4]], h: &[G1Affine], sh: &[[u64;4]], w:usize, bits:usize) -> Option<G1Projective> {
    let windows=(Fr::MODULUS_BITS as usize+1).div_ceil(bits);
    if k.len().min(h.len())<1024 || w+1!=windows { return None; }
    let remaining=Fr::MODULUS_BITS as usize-w*bits;
    if remaining>8 || !crate::simd::cpu_option("ZOPRO_CPU_TOP_TREE") { return None; }
    let n=k.len().checked_add(h.len())?;
    if n>u32::MAX as usize { return None; }
    let count=1usize<<remaining;
    let mut counts=vec![0usize;count];
    for (p,s) in k.iter().zip(sk).chain(h.iter().zip(sh)) {
        let d=digit(s,w,bits);
        debug_assert!(d>=0 && d as usize<=count);
        if d!=0 && !p.is_identity() {counts[d as usize-1]+=1;}
    }
    let mut starts=Vec::with_capacity(count+1); starts.push(0);
    for n in counts {starts.push(starts.last().unwrap()+n);}
    if *starts.last().unwrap()==0 {return Some(G1Projective::IDENTITY);}
    let mut positions=starts[..count].to_vec();
    let mut order=vec![0u32;*starts.last().unwrap()];
    for (i,(p,s)) in k.iter().zip(sk).chain(h.iter().zip(sh)).enumerate() {
        let d=digit(s,w,bits);
        if d!=0 && !p.is_identity() {
            let bucket=d as usize-1;
            order[positions[bucket]]=i as u32; positions[bucket]+=1;
        }
    }
    let buckets=crate::msm::reduce_indexed_buckets_by(
        |i| {let i=i as usize; if i<k.len(){k[i]}else{h[i-k.len()]}}, &order,&starts);
    Some(crate::msm::weighted_affine(&buckets))
}

#[cfg(test)]
mod top_tests {
    use super::*;
    use crate::testing::Seeded;
    #[test]
    fn combined_top_generator_oracle_and_fallbacks() {
        let nk=1027;let nh=2051;
        let make=|n,offset| {
            let mut p=G1Affine::GENERATOR*Fr::from(offset);
            let mut ps=Vec::new();let mut ws=Vec::new();
            for i in 0..n {
                p+=G1Affine::GENERATOR;
                let (q,w)=if i%17==0 {(G1Projective::IDENTITY,Fr::ZERO)}
                    else if i%3==0 {(-p,-Fr::from(offset+i as u64+1))}
                    else {(p,Fr::from(offset+i as u64+1))};
                ps.push(q);ws.push(w);
            }
            (G1Projective::batch_to_affine(&ps),ws)
        };
        let(k,kw)=make(nk,11);let(h,hw)=make(nh,100003);
        for threads in [1,4] {
            let pool=rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            pool.install(|| {
                let mut rng=Seeded::new(0xf053d70);
                for bits in 12..=14 {
                    let w=(Fr::MODULUS_BITS as usize+1).div_ceil(bits)-1;
                    for kind in 0..6 {
                        let mut want=Fr::ZERO;
                        let mut scalars=|weights:&[Fr]| -> Vec<[u64;4]> {
                            weights.iter().enumerate().map(|(i,weight)| {
                                let scalar=match kind {0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,
                                    3=>rng.fr(),4=>if i%2==0{-Fr::ONE}else{Fr::ZERO},
                                    _=>{let x=Fr::from(2u64).pow(&[(w*bits) as u64]);
                                        if i%2==0{x-Fr::ONE}else{x+Fr::ONE}}};
                                let s=scalar.to_canonical(); let d=digit(&s,w,bits);
                                want+=Fr::from(d as u64)* *weight;s
                            }).collect()
                        };
                        let sk=scalars(&kw);let sh=scalars(&hw);
                        assert_eq!(top(&k,&sk,&h,&sh,w,bits).unwrap().to_affine(),(G1Affine::GENERATOR*want).to_affine());
                        let(a,b)=rayon::join(||top_window::<G1Config,true>(&k,&sk,w,bits).unwrap(),||top_window::<G1Config,true>(&h,&sh,w,bits).unwrap());
                        assert_eq!(top(&k,&sk,&h,&sh,w,bits).unwrap().to_affine(),(a+b).to_affine());
                        assert!(top(&k[..1023],&sk[..1023],&h,&sh,w,bits).is_none());
                        assert!(top(&k,&sk,&h,&sh,w-1,bits).is_none());
                    }
                }
            });
        }
    }
}
