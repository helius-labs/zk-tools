//! Apply both local transforms and the Euler weights in one expanded leaf.
use super::*;

// Leave the adjacent sum and difference lanes together between the two transforms.
// Their natural coefficient indices differ by N/2 in bit-reversed DIF order.
fn center(a: &mut Pair, b: &mut Pair, inv: &Pair, fwd: &Pair, first: usize, log: usize) {
    let lo = Pair::shuffle_pair::<0, 1, 4, 5>(a, b);
    let hi = Pair::shuffle_pair::<2, 3, 6, 7>(a, b);
    let sum = lo.add(&hi);
    let diff = lo.sub(&hi).mul(inv);
    let left = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &diff);
    let right = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &diff);
    let lo = Pair::shuffle_pair::<0, 2, 4, 6>(&left, &right);
    let hi = Pair::shuffle_pair::<1, 3, 5, 7>(&left, &right);
    let q = bit_reverse(first, log - 2) as u64;
    let r = bit_reverse(first + 1, log - 2) as u64;
    let quarter = 1u64 << (log - 2);
    let even = [
        q,
        q + quarter,
        r,
        r + quarter,
        q,
        q + quarter,
        r,
        r + quarter,
    ];
    let odd = even.map(|x| x + 2 * quarter);
    let s = lo.add(&hi).mul_u28(&even);
    let d = lo.sub(&hi).mul_u28(&odd);
    let sum = s.add(&d);
    let diff = s.sub(&d);
    let left = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &diff);
    let right = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &diff);
    let lo = Pair::shuffle_pair::<0, 1, 4, 5>(&left, &right);
    let hi = Pair::shuffle_pair::<2, 3, 6, 7>(&left, &right).mul(fwd);
    let sum = lo.add(&hi);
    let diff = lo.sub(&hi);
    *a = Pair::shuffle_pair::<0, 1, 4, 5>(&sum, &diff);
    *b = Pair::shuffle_pair::<2, 3, 6, 7>(&sum, &diff);
}

fn leaf(plan: &Plan, values: &mut [PackedPair], level: usize, first: usize) {
    // The ordinary transform expands the same bounded leaf separately in each direction.
    let mut expanded: Vec<_> = values.iter().map(PairStorage::load).collect();
    let mut gap = expanded.len() / 2;
    let mut layer = level;
    while gap != 0 {
        for pair in expanded.chunks_exact_mut(gap * 2) {
            let (lo, hi) = pair.split_at_mut(gap);
            dif_quads(lo, hi, &plan.inverse_roots.layers[layer][..gap]);
        }
        gap /= 2;
        layer += 1;
    }
    let inv = broadcast(&plan.inverse_roots.pair);
    let fwd = broadcast(&plan.roots.pair);
    for (i, [a, b]) in expanded.as_chunks_mut::<2>().0.iter_mut().enumerate() {
        center(a, b, &inv, &fwd, first + 2 * i, plan.n.ilog2() as usize);
    }
    gap = 1;
    while gap < expanded.len() {
        layer -= 1;
        for pair in expanded.chunks_exact_mut(gap * 2) {
            let (lo, hi) = pair.split_at_mut(gap);
            dit_quads(lo, hi, &plan.roots.layers[layer][..gap]);
        }
        gap *= 2;
    }
    for (dst, v) in values.iter_mut().zip(expanded) {
        *dst = PackedPair::store(v);
    }
}

fn transform(plan: &Plan, values: &mut [PackedPair], level: usize, first: usize) {
    if values.len() <= 1024 {
        return leaf(plan, values, level, first);
    }
    columns::<false>(values, &plan.inverse_roots, level);
    let size = values.len() / 32;
    values
        .par_chunks_exact_mut(size)
        .enumerate()
        .for_each(|(i, part)| {
            transform(plan, part, level + 5, first + i * size);
        });
    columns::<true>(values, &plan.roots, level);
}

#[inline(never)]
pub(super) fn from_inputs(plan: &Plan, a: &[Fr], b: &[Fr]) -> Vec<PackedPair> {
    let mut values = input_columns::first_layer(plan, a, b);
    let width = values.len() / 32;
    values
        .par_chunks_exact_mut(width)
        .enumerate()
        .for_each(|(i, part)| {
            transform(plan, part, 5, i * width);
        });
    columns::<true>(&mut values, &plan.roots, 0);
    values
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn column_center_matches_separate_transforms() {
        for n in [8192usize, 32768, 262144] {
            let root = Fr::root_of_unity(n).unwrap().pow(&[3]);
            let coset = Fr::from(5u64);
            let mut plan = Plan::new(n, root, coset, denominator(n, coset).unwrap());
            plan.prepare_euler_basis();
            let a: Vec<_> = (0..n / 2 + 3).map(|i| Fr::from((i + 17) as u64)).collect();
            let b: Vec<_> = (0..n - 1)
                .map(|i| Fr::from((i + 1) as u64).square())
                .collect();
            let got = from_inputs(&plan, &a, &b);
            let mut expected = input_columns::inverse(&plan, &a, &b);
            for (i, value) in expected.iter_mut().enumerate() {
                let factors =
                    core::array::from_fn(|j| bit_reverse(4 * i + j % 4, n.ilog2() as usize) as u64);
                *value = PackedPair::store(value.load().mul_u28(&factors));
            }
            stored_transform::<PackedPair, true>(&mut expected, &plan.roots, 0);
            assert_eq!(got.len(), expected.len());
            for (x, y) in got.iter().zip(&expected) {
                assert_eq!(x.load().get(), y.load().get());
            }
        }
    }
    #[test]
    fn derivative_coefficients_match_existing_transforms() {
        for n in [512usize, 8192, 32768, 262144] {
            for power in [1, 3] {
                let root = Fr::root_of_unity(n).unwrap().pow(&[power]);
                let coset = Fr::from(5u64);
                let mut plan = Plan::new(n, root, coset, denominator(n, coset).unwrap());
                plan.prepare_euler_basis();
                let mut expect = (0..n / 4)
                    .map(|i| {
                        PackedPair::store(Pair::new(&core::array::from_fn(|j| {
                            if 4 * i + (j % 4) > n * 2 / 3 {
                                Fr::ZERO
                            } else {
                                Fr::from((11 * i + 17 * j) as u64).square()
                            }
                        })))
                    })
                    .collect::<Vec<_>>();
                let mut got = expect.clone();
                stored_transform::<PackedPair, false>(&mut expect, &plan.inverse_roots, 0);
                for (i, v) in expect.iter_mut().enumerate() {
                    let factors = core::array::from_fn(|j| {
                        bit_reverse(i * 4 + j % 4, n.ilog2() as usize) as u64
                    });
                    *v = PackedPair::store(v.load().mul_u28(&factors));
                }
                stored_transform::<PackedPair, true>(&mut expect, &plan.roots, 0);
                transform(&plan, &mut got, 0, 0);
                for (a, b) in got.iter().zip(&expect) {
                    assert_eq!(a.load().get(), b.load().get());
                }
            }
        }
    }
}

fn columns<const FORWARD: bool>(values: &mut [PackedPair], roots: &Twiddles, level: usize) {
    let size = values.len() / 32;
    let mut pieces = values.chunks_exact_mut(size);
    let [v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19, v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31]: [&mut [PackedPair];32]=core::array::from_fn(|_|pieces.next().unwrap());
    stored_columns::<PackedPair, FORWARD>(
        v0, v1, v2, v3, v4, v5, v6, v7, v8, v9, v10, v11, v12, v13, v14, v15, v16, v17, v18, v19,
        v20, v21, v22, v23, v24, v25, v26, v27, v28, v29, v30, v31, roots, level,
    );
}
