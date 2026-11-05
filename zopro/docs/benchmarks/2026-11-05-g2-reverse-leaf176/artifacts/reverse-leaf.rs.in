//! Original complete reverse loop, separated once per batch.
use super::*;
#[inline(never)]
pub(super) fn apply<C: CurveConfig<Base=Fq2>,const FAST:bool>(
    buckets:&mut[Affine<C>],batch:&[(u32,&Affine<C>)],s:&Fq2AddScratch,full:usize,mut inverse:FqBatch,
){
    #[cfg(test)]super::reverse_leaf_tests::GROUPS.fetch_add(full/LANES,std::sync::atomic::Ordering::Relaxed);
    for j in (0..full / LANES).rev() {
        let k = j * LANES;
        let a: [Affine<C>; LANES] =
            core::array::from_fn(|i| buckets[(batch[k + i].0 & 0x7fffffff) as usize]);
        let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1);
        let ax = fq2_pack(a.map(|p| p.x));
        let ay = fq2_pack(a.map(|p| p.y));
        let bx = fq2_pack(b.map(|p| p.x));
        let dy=if FAST{
            let by=fq2_pack(b.map(|p|p.y));
            let negative=signs::<true,C>(&batch[k..k+LANES]);
            core::array::from_fn(|i|by[i].signed_difference_canonical(&ay[i],negative))
        }else{
            let by=fq2_pack(core::array::from_fn(|i|if batch[k+i].0>>31!=0{-b[i].y}else{b[i].y}));
            core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]))
        };
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let factor = s.prefix_records[j][0].mul(&inverse);
        inverse = inverse.mul(&s.prefix_records[j][1]);
        let slope = Fq2::lanes_scaled_numerator(&dy, &denominator, &factor);
        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &Fq2::lanes_add(&ax, &bx));
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);
        let (x, y) = (fq2_unpack(x), fq2_unpack(y));
        for i in 0..LANES {
            buckets[(batch[k + i].0 & 0x7fffffff) as usize] = Affine::new(x[i], y[i]);
        }
    }
}
