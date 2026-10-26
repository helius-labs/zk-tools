pub(crate) fn ordinary_enabled() -> bool {
    enabled() && std::env::var("ZOPRO_CPU_BORROWED_ORDINARY").as_deref()==Ok("1")
}

pub(crate) fn msm<C:CurveConfig<Base=Fq>>(bases:&[Affine<C>],scalars:&[[u64;4]],parts:usize,width:usize)->Projective<C> {
    assert_eq!(bases.len(),scalars.len());
    assert!((1024..65536).contains(&bases.len()));
    assert!((1..=2).contains(&parts));assert!((10..=14).contains(&width));
    let part=bases.len().div_ceil(parts);
    let windows=(Fr::MODULUS_BITS as usize+1).div_ceil(width);
    let size=batch_size(width).max(32);
    let sums:Vec<_>=(0..windows).into_par_iter().map(|w| {
        if let Some(sum)=top_window::<C,true>(bases,scalars,w,width){return sum;}
        let mut buckets:Vec<_>=(0..parts).into_par_iter().map(|p|{
            let range=p*part..((p+1)*part).min(bases.len());
            let mut state=Window::new(1usize<<(width-1),size);
            state.feed(&bases[range.clone()],&scalars[range],w,width,size);
            state.finish()
        }).collect();
        let mut combined=buckets.swap_remove(0);
        for other in buckets{super::merge_buckets(&mut combined,&other);}
        crate::msm::weighted_affine(&combined)
    }).collect();
    let mut result=Projective::<C>::IDENTITY;
    for sum in sums.iter().rev(){for _ in 0..width{result=result.double();}result+=sum;}
    result
}
