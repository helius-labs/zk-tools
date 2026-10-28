//! Same forward arithmetic and records; direct referenced coordinate loads.
use super::*;
#[inline(always)]
pub(super) fn forward<C:CurveConfig<Base=Fq>,const FAST:bool,const SIGNED:bool>(buckets:&[Affine<C>],batch:&[(u32,impl std::borrow::Borrow<Affine<C>>+Copy)],s:&mut FqAddScratch,full:usize)->FqBatch{
let mut chains=FqBatch::one();
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
                let ax = FqBatch::from_refs(a.map(|p| &p.x));
                core::ptr::addr_of_mut!((*slot).ax).write(ax);
                let bx = FqBatch::from_refs(b.map(|p| &p.x));
                let denominator = canonical_sub::<FAST>(&bx, &ax);
                core::ptr::addr_of_mut!((*slot).xsum).write(canonical_add::<FAST>(&ax, &bx));
                core::ptr::addr_of_mut!((*slot).denominator).write(denominator);
                let ay = FqBatch::from_refs(a.map(|p| &p.y));
                core::ptr::addr_of_mut!((*slot).ay).write(ay);
                let by = FqBatch::from_refs(b.map(|p| &p.y));
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
            let ax = FqBatch::from_refs(a.map(|p| &p.x));
            let ay = FqBatch::from_refs(a.map(|p| &p.y));
            let bx = FqBatch::from_refs(b.map(|p| &p.x));
            let by = FqBatch::from_refs(b.map(|p| &p.y));
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

chains
}
