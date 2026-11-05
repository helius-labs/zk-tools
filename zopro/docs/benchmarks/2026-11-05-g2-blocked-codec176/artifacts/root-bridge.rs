// Appended inside msm_affine. Diagnostic preparation only; original functions stay exact.
#[cfg(zopro_codec_root)]
pub(super) fn codec_root(bases:&[zopro_bn254::G2Affine],words:&[[u64;4]]) -> (Vec<zopro_bn254::G2Affine>,Vec<[u64;4]>,bool) {
    assert_eq!(bases.len(),words.len());
    let groups=crate::simd::cpu_option("ZOPRO_CPU_DEDUP").then(||scalar_groups(words)).flatten();
    match groups {
        Some(g)=>{
            assert!((1024..65536).contains(&bases.len())&&(1024..65536).contains(&g.values.len()));
            let points=crate::msm::group_affine(bases,&g.order,&g.starts,&g.counts);
            (points,g.values,true)
        }
        None=>(bases.to_vec(),words.to_vec(),false),
    }
}
