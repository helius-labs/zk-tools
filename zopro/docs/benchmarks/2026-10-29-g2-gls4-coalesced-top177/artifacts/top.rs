// This is the original signed indexed top algorithm adapted only to GLS4 digits.
fn top_component(bases: &[G2Affine], short: &[[u64; 5]], component: usize) -> Vec<G2Affine> {
    const N: usize = 1 << (BITS - 1);
    let bucket = |d: i16| d.unsigned_abs() as usize - 1 + if d < 0 { N } else { 0 };
    let mut counts = vec![0usize; 2 * N];
    for (base, s) in bases.iter().zip(short) {
        let d = component_digit(s, component, WINDOWS - 1, BITS);
        assert!(d.unsigned_abs() as usize <= N);
        if d != 0 && !base.is_identity() {
            counts[bucket(d)] += 1;
        }
    }
    let mut starts = Vec::with_capacity(2 * N + 1);
    starts.push(0);
    for count in counts {
        starts.push(starts.last().unwrap() + count);
    }
    let len = *starts.last().unwrap();
    if len == 0 {
        return vec![G2Affine::IDENTITY; N];
    }
    let mut positions = starts[..2 * N].to_vec();
    let mut order = vec![0u32; len];
    for (i, (base, s)) in bases.iter().zip(short).enumerate() {
        let d = component_digit(s, component, WINDOWS - 1, BITS);
        if d != 0 && !base.is_identity() {
            let b = bucket(d);
            order[positions[b]] = i as u32;
            positions[b] += 1;
        }
    }
    let mut reduced = crate::msm::reduce_indexed_buckets(bases, &order, &starts);
    let mut negative = reduced.split_off(N);
    for p in &mut negative {
        p.y = -p.y;
    }
    super::super::merge_buckets(&mut reduced, &negative);
    reduced
}
fn top_window(bases: &[G2Affine], short: &[[u64; 5]], x_factor: &Fq2) -> Vec<G2Affine> {
    let mut combined = top_component(bases, short, 3);
    for component in (0..3).rev() {
        for p in &mut combined {
            if !p.is_identity() {
                if component == 1 {
                    p.x = p.x.conjugate() * *x_factor;
                    p.y = p.y.conjugate() * PSI_Y;
                } else {
                    p.x = p.x.mul_by_fp(&BETA);
                }
            }
        }
        let next = top_component(bases, short, component);
        super::super::merge_buckets(&mut combined, &next);
    }
    combined
}
