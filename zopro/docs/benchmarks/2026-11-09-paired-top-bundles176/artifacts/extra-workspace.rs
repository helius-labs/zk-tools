
// Concurrent top components keep one additional parent per existing partition.
// Leaves remain nonforking; their max live count is still the worker count.
#[cfg(zopro_g2_paired_top)]
pub(super) fn paired_top_extra(n: usize, parts: usize) -> Option<usize> {
    if !(1024..65536).contains(&n) || parts != 2 { return None; }
    let parent = top::<G2Config>(n.div_ceil(parts), 7)?;
    let conservative_merge = mul(vector::<Affine<G2Config>>(64, false)?, 3)?;
    mul(add(parent.held, conservative_merge)?, parts)
}
