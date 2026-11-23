//! Pack Euler inputs directly into the first radix-32 inverse transform.
use super::*;
use core::mem::MaybeUninit;

const COLUMNS_PER_TASK: usize = 64;

// Construct the first inverse radix-32 columns directly from the two input
// polynomials. Each invocation owns the same column range in all 32 stripes.
fn pack_inverse_columns<const CACHED: bool>(
    mut stripes: [&mut [MaybeUninit<PackedPair>]; 32],
    a: &[Fr],
    b: &[Fr],
    roots: &Twiddles,
    q: usize,
    start: usize,
) {
    let columns = stripes[0].len();
    if columns > COLUMNS_PER_TASK {
        let mid = columns / 2;
        let left = core::array::from_fn(|i| {
            let stripe = core::mem::take(&mut stripes[i]);
            let (left, right) = stripe.split_at_mut(mid);
            stripes[i] = right;
            left
        });
        rayon::join(
            || pack_inverse_columns::<CACHED>(left, a, b, roots, q, start),
            || pack_inverse_columns::<CACHED>(stripes, a, b, roots, q, start + mid),
        );
        return;
    }
    let count = a.len().max(b.len());
    let zero = Pair::new(&[Fr::ZERO; 8]);
    for column in 0..columns {
        let mut local = core::array::from_fn(|row| {
            let i = 4 * (row * q + start + column);
            if i >= count {
                return zero;
            }
            Pair::new(&core::array::from_fn(|lane| {
                let values = if lane < 4 { a } else { b };
                values.get(i + lane % 4).copied().unwrap_or(Fr::ZERO)
            }))
        });
        column_kernel::<CACHED, false>(&mut local, roots, 0, q, start + column);
        for (stripe, value) in stripes.iter_mut().zip(local) {
            stripe[column].write(PackedPair::store(value));
        }
    }
}

#[inline(never)]
pub(super) fn first_layer(
    plan: &Plan,
    a: &[Fr],
    b: &[Fr],
    mut values: Vec<PackedPair>,
) -> Vec<PackedPair> {
    let n = plan.n / 4;
    let q = n / 32;
    assert!(n >= 2048 && n.is_power_of_two());
    let roots = &plan.inverse_roots;
    assert!(values.is_empty() && values.capacity() >= n);
    let mut chunks = values.spare_capacity_mut()[..n].chunks_exact_mut(q);
    let stripes = core::array::from_fn(|_| chunks.next().unwrap());
    if roots.columns.is_empty() {
        pack_inverse_columns::<false>(stripes, a, b, roots, q, 0);
    } else {
        pack_inverse_columns::<true>(stripes, a, b, roots, q, 0);
    }
    // SAFETY: the 32 disjoint stripes cover exactly n slots. Both branches
    // join every producer and write each slot before this call.
    unsafe { values.set_len(n) };
    values
}

pub(super) fn inverse(plan: &Plan, a: &[Fr], b: &[Fr]) -> Vec<PackedPair> {
    let mut values = first_layer(plan, a, b, Vec::with_capacity(plan.n / 4));
    let n = values.len();
    let q = n / 32;
    let roots = &plan.inverse_roots;
    let cutoff = leaf::cutoff(n, rayon::current_num_threads());
    values.par_chunks_mut(q).for_each(|part| {
        stored_transform_inner::<PackedPair, false>(part, roots, 5, cutoff);
    });
    values
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn packed_input_matches_inverse_for_prefixes_and_root_layouts() {
        let n = 8192;
        let root = Fr::root_of_unity(n).unwrap().pow(&[3]);
        let coset = Fr::from(5u64);
        let mut plan = Plan::new(n, root, coset, crate::fft::denominator(n, coset).unwrap());
        for cache in [false, true] {
            if cache {
                plan.inverse_roots.prepare_columns();
            }
            for (na, nb) in [(0, 0), (3, 129), (4097, 4102), (n, n - 1)] {
                let make = |len, seed| {
                    (0..len)
                        .map(|i| Fr::from(i as u64 + seed))
                        .collect::<Vec<_>>()
                };
                let a = make(na, 11);
                let b = make(nb, 29);
                let mut expected: Vec<_> = (0..n / 4)
                    .map(|i| {
                        PackedPair::store(Pair::new(&core::array::from_fn(|j| {
                            (if j < 4 { &a } else { &b })
                                .get(4 * i + j % 4)
                                .copied()
                                .unwrap_or(Fr::ZERO)
                        })))
                    })
                    .collect();
                stored_transform::<PackedPair, false>(&mut expected, &plan.inverse_roots, 0);
                let got = inverse(&plan, &a, &b);
                for (x, y) in got.iter().zip(&expected) {
                    assert_eq!(x.load().get(), y.load().get());
                }
            }
        }
    }
}
