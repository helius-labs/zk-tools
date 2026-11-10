#[cfg(zopro_g2_queue_tree)]
fn queue_tree_extra_peak() -> Option<usize> {
    // Sum both lifetimes conservatively, although attachment descriptors are
    // made after the grouped reducer has dropped its point/record scratch.
    let metadata = 2 * core::mem::size_of::<[u32;199]>()
        + core::mem::size_of::<[usize;200]>() + core::mem::size_of::<[usize;199]>();
    let roots = vector::<Affine<G2Config>>(199, false)?;
    let grouping = crate::msm::workspace_grouping::<G2Config>(199, 1)?;
    let attachments = vector::<(u32,&Affine<G2Config>)>(199, false)?;
    let reducer_fixed = 2 * core::mem::size_of::<[usize;256]>();
    add(add(add(add(metadata,roots)?,grouping)?,attachments)?,reducer_fixed)
}
