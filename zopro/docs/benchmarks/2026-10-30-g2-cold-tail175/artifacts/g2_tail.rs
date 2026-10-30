//! Scalar fallback extracted without arithmetic changes.
use super::*;

#[cold]
#[inline(never)]
pub(super) fn owned<C:CurveConfig<Base=Fq2>>(buckets:&mut[Affine<C>],batch:&[(u32,Affine<C>)],s:&mut Fq2AddScratch,chains:&FqBatch,full:usize)->FqBatch {
let mut tail = chains.get();
        for k in full..batch.len() {
            let (index, b) = batch[k];
            let a = buckets[index as usize];
            let denominator = b.x - a.x;
            let norm = denominator.c0.square() + denominator.c1.square();
            let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
            tail[k % LANES] *= norm;
            s.tail
                .push([a.x, a.y, a.x + b.x, Fq2::new(norm, Fq::ZERO), numerator]);
        }
        let mut inverse = invert_lanes(tail, s.vector_inverse);
        for k in (full..batch.len()).rev() {
            let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
            let slope = numerator.mul_by_fp(&inverse[k % LANES]);
            inverse[k % LANES] *= norm.c0;
            let x = slope.square() - xsum;
            let y = slope * (ax - x) - ay;
            buckets[batch[k].0 as usize] = Affine::new(x, y);
        }
        FqBatch::new(&inverse)
}

#[cold]
#[inline(never)]
pub(super) fn borrowed<C:CurveConfig<Base=Fq2>>(buckets:&mut[Affine<C>],batch:&[(u32,&Affine<C>)],s:&mut Fq2AddScratch,chains:&FqBatch,full:usize)->FqBatch {
let mut tail = chains.get();
            for k in full..batch.len() {
                let (encoded, b) = batch[k];
                let index = encoded & 0x7fffffff;
                let b = if encoded >> 31 != 0 { -*b } else { *b };
                let a = buckets[index as usize];
                let denominator = b.x - a.x;
                let norm = denominator.c0.square() + denominator.c1.square();
                let numerator = ((b.y - a.y) * denominator.conjugate()).mul_by_fp(&tail[k % LANES]);
                tail[k % LANES] *= norm;
                s.tail
                    .push([a.x, a.y, a.x + b.x, Fq2::new(norm, Fq::ZERO), numerator]);
            }
            let mut inverse = invert_lanes(tail, s.vector_inverse);
            for k in (full..batch.len()).rev() {
                let [ax, ay, xsum, norm, numerator] = s.tail[k - full];
                let slope = numerator.mul_by_fp(&inverse[k % LANES]);
                inverse[k % LANES] *= norm.c0;
                let x = slope.square() - xsum;
                let y = slope * (ax - x) - ay;
                buckets[(batch[k].0 & 0x7fffffff) as usize] = Affine::new(x, y);
            }
            FqBatch::new(&inverse)
}
