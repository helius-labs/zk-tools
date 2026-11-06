pub(super) fn apply<C: CurveConfig<Base = Fq2>, const FAST: bool>(
    buckets: &mut [Affine<C>],
    batch: &[(u32, &Affine<C>)],
    s: &mut Fq2AddScratch,
) {
    let (groups, _) = batch.as_chunks::<LANES>();
    let full = groups.len() * LANES;
    let mut chains = FqBatch::one();
    s.prefix_records.clear();
    s.prefix_records.reserve(full / LANES);
    for group in groups {
        let ax = fq2_pack(core::array::from_fn(|i| {
            buckets[(group[i].0 & 0x7fffffff) as usize].x
        }));
        let bx = fq2_pack(core::array::from_fn(|i| group[i].1.x));
        let denominator = core::array::from_fn(|i| canonical_sub::<FAST>(&bx[i], &ax[i]));
        let norm = Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains, norm]);
        chains = chains.mul(&norm);
    }
    s.tail.clear();
    let mut inverse =
        if let Some(value) = direct_chain_inverse(&chains, full == batch.len(), s.vector_inverse) {
            value
        } else {
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
        };
    for (group, record) in groups.iter().zip(s.prefix_records.iter()).rev() {
        let a: [Affine<C>; LANES] =
            core::array::from_fn(|i| buckets[(group[i].0 & 0x7fffffff) as usize]);
        let b: [&Affine<C>; LANES] = core::array::from_fn(|i| group[i].1);
        let ax = fq2_pack(a.map(|p| p.x));
        let ay = fq2_pack(a.map(|p| p.y));
        let bx = fq2_pack(b.map(|p| p.x));
        let dy=if FAST{
            let by=fq2_pack(b.map(|p|p.y));
            let negative=signs::<true,C>(group);
            core::array::from_fn(|i|by[i].signed_difference_canonical(&ay[i],negative))
        }else{
            let by=fq2_pack(core::array::from_fn(|i|if group[i].0>>31!=0{-b[i].y}else{b[i].y}));
            core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]))
        };
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let factor = record[0].mul(&inverse);
        inverse = inverse.mul(&record[1]);
        let slope = Fq2::lanes_scaled_numerator(&dy, &denominator, &factor);
        let x = Fq2::lanes_sub(&Fq2::lanes_square(&slope), &Fq2::lanes_add(&ax, &bx));
        let y = Fq2::lanes_sub(&Fq2::lanes_mul(&slope, &Fq2::lanes_sub(&ax, &x)), &ay);
        let (x, y) = (fq2_unpack(x), fq2_unpack(y));
        for i in 0..LANES {
            buckets[(group[i].0 & 0x7fffffff) as usize] = Affine::new(x[i], y[i]);
        }
    }
}
