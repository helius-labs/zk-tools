#[cfg(zopro_late_m3)]
pub(super) fn late_m3_peak(n:usize,workers:usize)->Option<usize> {
    let mut p=shape::<G2Config>(4096,10,2,batch_size(13).max(32))?;
    let top=top::<G2Config>(n.div_ceil(2),11)?;
    // Two concurrent top partitions, signed-index arrays and retained high sums.
    // Ordinary shape already charges a full dense parent for the top as well.
    let extra=mul(vector::<Affine<G2Config>>(1024,false)?,3)?;
    p.held=add(p.held,mul(add(top.held,extra)?,2)?)?;
    p.held=add(p.held,vector::<[u16;20]>(n,false)?)?;
    p.held=add(p.held,mul(core::mem::size_of::<Projective<G2Config>>(),20)?)?;
    p.leaf=p.leaf.max(top.leaf);
    // Extra optional reservation is conservative on top of unchanged existing
    // proof forecasts. Suspended parents are charged once; nonforking leaves
    // at most once per participating Rayon worker, not once per window task.
    add(p.held,mul(p.leaf,workers.max(1))?)
}
