//! Two immutable inputs per pair; output slots need no staged left point.
use super::*;
pub(super) fn apply<C: CurveConfig<Base = Fq2>, const FAST: bool>(
    out: &mut [Affine<C>],
    pairs: &[(u32, &Affine<C>, &Affine<C>)],
    s: &mut Fq2AddScratch,
) {
    let full = pairs.len() / LANES * LANES;
    let mut chains = FqBatch::one();
    s.prefix_records.clear();
    s.prefix_records.reserve(full / LANES);
    for k in (0..full).step_by(LANES) {
        let ax = fq2_pack(core::array::from_fn(|i| pairs[k+i].1.x));
        let bx = fq2_pack(core::array::from_fn(|i| pairs[k+i].2.x));
        let denominator = core::array::from_fn(|i| canonical_sub::<FAST>(&bx[i], &ax[i]));
        let norm = Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains, norm]);
        chains = chains.mul(&norm);
    }
    s.tail.clear();
    let mut inverse = if let Some(value) = direct_chain_inverse(&chains, full == pairs.len(), s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..pairs.len() {
            let (encoded, a, b) = pairs[k];
            let b = if ((encoded >> 31) ^ (encoded >> 30)) & 1 != 0 { -*b } else { *b };
            let denominator = b.x - a.x;
            let norm = denominator.c0.square() + denominator.c1.square();
            let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
            tail[k % LANES] *= norm;
            s.tail.push([a.x, a.y, a.x+b.x, Fq2::new(norm,Fq::ZERO), numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..pairs.len()).rev() {
            let [ax, ay, xsum, norm, numerator] = s.tail[k-full];
            let slope = numerator.mul_by_fp(&inverse[k % LANES]);
            inverse[k % LANES] *= norm.c0;
            let x = slope.square() - xsum;
            let y = slope * (ax-x) - ay;
            out[(pairs[k].0 & 0x3fff_ffff) as usize] = Affine::new(x, if pairs[k].0 >> 31 != 0 {-y} else {y});
        }
        FqBatch::new(&inverse)
    };
    for j in (0..full/LANES).rev() {
        let k = j*LANES;
        let a: [&Affine<C>; LANES] = core::array::from_fn(|i| pairs[k+i].1);
        let b: [&Affine<C>; LANES] = core::array::from_fn(|i| pairs[k+i].2);
        let ax = fq2_pack(a.map(|p|p.x));
        let ay = fq2_pack(a.map(|p|p.y));
        let bx = fq2_pack(b.map(|p|p.x));
        let negative = (0..LANES).fold(0u8, |mask,i| mask | (((((pairs[k+i].0 >> 31) ^ (pairs[k+i].0 >> 30)) & 1) as u8) << i));
        let dy = if FAST {
            let by = fq2_pack(b.map(|p|p.y));
            core::array::from_fn(|i| by[i].signed_difference_canonical(&ay[i], negative))
        } else {
            let by = fq2_pack(core::array::from_fn(|i| if negative >> i & 1 != 0 {-b[i].y} else {b[i].y}));
            core::array::from_fn(|i| canonical_sub::<FAST>(&by[i], &ay[i]))
        };
        let denominator = core::array::from_fn(|i| canonical_sub::<FAST>(&bx[i], &ax[i]));
        let factor = s.prefix_records[j][0].mul(&inverse);
        inverse = inverse.mul(&s.prefix_records[j][1]);
        let slope = Fq2::lanes_scaled_numerator(&dy, &denominator, &factor);
        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &Fq2::lanes_add(&ax, &bx));
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax,&x)), &ay);
        let (x,y) = (fq2_unpack(x),fq2_unpack(y));
        for i in 0..LANES {
            out[(pairs[k+i].0 & 0x3fff_ffff) as usize] = Affine::new(x[i], if pairs[k+i].0 >> 31 != 0 {-y[i]} else {y[i]});
        }
    }
}
impl<C: CurveConfig<Base = Fq2>> BatchAdd<C> {
    pub(crate) fn apply_signed_pair_g2(&mut self, out: &mut [Affine<C>], pairs: &[(u32,&Affine<C>,&Affine<C>)]) {
        assert!(self.borrowed_g2_enabled());
        if !pairs.is_empty() { apply::<C,true>(out,pairs,&mut self.fq2); }
    }
}
