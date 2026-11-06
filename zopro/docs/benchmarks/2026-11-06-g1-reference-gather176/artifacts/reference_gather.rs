//! Temporary forward gathers borrow points; arithmetic and records remain unchanged.
use super::*;
#[inline(never)]
pub(super) fn gather<C: CurveConfig<Base = Fq>, const FAST: bool, const SIGNED: bool>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, impl std::borrow::Borrow<Affine<C>> + Copy)],
    s: &mut FqAddScratch,
) {
    let full = batch.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.groups.clear();
    if s.write_groups {
        // Reserve all records before taking any pointers. Each record is
        // Copy and its length is published only after all five fields are
        // initialized. A panic before then leaves no partial record in len.
        s.groups.reserve(full / LANES);
        for k in (0..full).step_by(LANES) {
            let a: [&Affine<C>; LANES] = core::array::from_fn(|i| &buckets[batch_index::<SIGNED>(batch[k + i].0)]);
            let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1.borrow());
            // SAFETY: reserve covers full/LANES records, k/LANES is in
            // that range, and no allocation changes during these writes.
            // addr_of_mut! creates no reference to uninitialized storage.
            unsafe {
                let slot = s.groups.as_mut_ptr().add(k / LANES);
                let ax = FqBatch::new(&a.map(|p| p.x));
                core::ptr::addr_of_mut!((*slot).ax).write(ax);
                let bx = FqBatch::new(&b.map(|p| p.x));
                let denominator = canonical_sub::<FAST>(&bx, &ax);
                core::ptr::addr_of_mut!((*slot).xsum).write(canonical_add::<FAST>(&ax, &bx));
                core::ptr::addr_of_mut!((*slot).denominator).write(denominator);
                let ay = FqBatch::new(&a.map(|p| p.y));
                core::ptr::addr_of_mut!((*slot).ay).write(ay);
                let by = FqBatch::new(&b.map(|p| p.y));
                let numerator = scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&chains,signs::<SIGNED,C>(&batch[k..k+LANES]));
                core::ptr::addr_of_mut!((*slot).numerator).write(numerator);
                chains = chains.mul(&denominator);
                // All five fields of this record are now initialized.
                s.groups.set_len(k / LANES + 1);
            }
        }
    } else {
        if s.reserve_groups {
            s.groups.reserve(full / LANES);
        }
        for k in (0..full).step_by(LANES) {
            let a: [&Affine<C>; LANES] = core::array::from_fn(|i| &buckets[batch_index::<SIGNED>(batch[k + i].0)]);
            let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1.borrow());
            let ax = FqBatch::new(&a.map(|p| p.x));
            let ay = FqBatch::new(&a.map(|p| p.y));
            let bx = FqBatch::new(&b.map(|p| p.x));
            let by = FqBatch::new(&b.map(|p| p.y));
            // Host coordinates are canonical; only the resulting differences are lazy.
            let denominator = canonical_sub::<FAST>(&bx, &ax);
            let numerator = scaled_signed_dy::<FAST,SIGNED>(&by,&ay,&chains,signs::<SIGNED,C>(&batch[k..k+LANES]));
            chains = chains.mul(&denominator);
            s.groups.push(FqAddGroup {
                ax,
                ay,
                xsum: canonical_add::<FAST>(&ax, &bx),
                denominator,
                numerator,
            });
        }
    }
    s.tail.clear();
    let mut inverse = if let Some(value)=direct_chain_inverse(&chains,full==batch.len(),s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..batch.len() {
            let (index, b) = batch[k];
            let b = *b.borrow();
            let b = if SIGNED && index>>31!=0 {-b}else{b};
            let a = buckets[batch_index::<SIGNED>(index)];
            let denominator = b.x - a.x;
            let numerator = (b.y - a.y) * tail[k % LANES];
            tail[k % LANES] *= denominator;
            s.tail.push([a.x, a.y, a.x + b.x, denominator, numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, denominator, numerator] = s.tail[k - full];
            let slope = numerator * inverse[k % LANES];
            inverse[k % LANES] *= denominator;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch_index::<SIGNED>(batch[k].0)] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
    };
    for (j, g) in s.groups.iter().enumerate().rev() {
        let slope = g.numerator.mul(&inverse);
        inverse = inverse.mul(&g.denominator);
        let x = slope.mul(&slope).sub(&g.xsum);
        let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);
        let (x, y) = (x.get(), y.get());
        for i in 0..LANES {
            buckets[batch_index::<SIGNED>(batch[j * LANES + i].0)] = Affine::new(x[i], y[i]);
        }
    }
}
