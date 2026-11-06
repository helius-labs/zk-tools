//! Ordinary merge operands borrow the immutable right bucket array.
use super::*;

pub(super) fn try_merge<C: CurveConfig<Base = Fq2>>(
    left: &mut [Affine<C>],
    right: &[Affine<C>],
    packed: &mut BatchAdd<C>,
) -> bool {
    assert_eq!(left.len(), right.len());
    // A rejected dispatch must leave both operands untouched.
    if !packed.borrowed_g2_enabled() {
        return false;
    }
    let mut pairs = Vec::with_capacity(left.len());
    for (i, (a, b)) in left.iter_mut().zip(right).enumerate() {
        if a.is_identity() {
            *a = *b;
        } else if b.is_identity() {
        } else if a.x == b.x {
            *a = (*a + *b).to_affine();
        } else {
            pairs.push((i as u32, b));
        }
    }
    if !pairs.is_empty() {
        packed.apply_signed_borrowed_g2(left, &pairs);
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{Fr, G1Config, G2Affine, G2Config, G2Projective};
    use std::sync::atomic::{AtomicUsize, Ordering};
    pub(crate) static DISPATCHES: AtomicUsize = AtomicUsize::new(0);

    fn operands(n: usize, exceptional: bool) -> (Vec<G2Affine>, Vec<G2Affine>, Vec<G2Affine>) {
        let g = G2Projective::generator();
        let a: Vec<_> = (0..n).map(|i| {
            if exceptional && i % 11 == 0 { Fr::ZERO }
            else { Fr::from(i as u64 + 17001) }
        }).collect();
        let b: Vec<_> = (0..n).map(|i| match (exceptional, i % 11) {
            (true, 1) => Fr::ZERO,
            (true, 2) => a[i],
            (true, 3) => -a[i],
            _ => Fr::from(i as u64 + 330001),
        }).collect();
        let points = |s: &[Fr]| G2Projective::batch_to_affine(
            &s.iter().map(|s| g * *s).collect::<Vec<_>>());
        let expected = points(&a.iter().zip(&b).map(|(a,b)| *a + *b).collect::<Vec<_>>());
        (points(&a), points(&b), expected)
    }

    #[test]
    fn record_layout_and_requested_heap_bound() {
        use core::mem::size_of;
        assert_eq!(size_of::<G2Affine>(), 128);
        assert_eq!(size_of::<(u32,G2Affine)>(), 136);
        assert_eq!(size_of::<(u32,&G2Affine)>(), 16);
        for n in [0,1,7,8,9,511,512] {
            let owned = Vec::<(u32,G2Affine)>::with_capacity(n);
            let borrowed = Vec::<(u32,&G2Affine)>::with_capacity(n);
            assert_eq!(owned.capacity(), n);
            assert_eq!(borrowed.capacity(), n);
            assert!(borrowed.capacity()*size_of::<(u32,&G2Affine)>()
                <= owned.capacity()*size_of::<(u32,G2Affine)>());
        }
        assert_eq!(512*(size_of::<(u32,G2Affine)>()-size_of::<(u32,&G2Affine)>()), 61440);
    }

    #[test]
    fn ordinary_tails_exceptional_pairs_and_reuse() {
        for n in [0,1,7,8,9,15,16,17,199,200,255,256,511,512] {
            for exceptional in [false,true] {
                let (left,right,expected) = operands(n,exceptional);
                let saved = right.clone();
                let mut packed = BatchAdd::<G2Config>::default();
                assert!(packed.borrowed_g2_enabled());
                for _ in 0..3 {
                    let mut actual = left.clone();
                    assert!(try_merge(&mut actual,&right,&mut packed));
                    assert_eq!(actual,expected,"n={n}, exceptional={exceptional}");
                }
                assert_eq!(right,saved);
            }
        }
    }

    #[test]
    fn disabled_typed_dispatch_does_not_mutate_inputs() {
        let (left,right,_) = operands(17,true);
        for mode in 0..4 {
            let mut actual = left.clone();
            let mut packed = BatchAdd::<G2Config>::default();
            match mode {
                0 => packed.direct = false,
                1 => packed.fq2.canonical = false,
                2 => packed.fq2.vector_inverse = false,
                _ => packed.fq2.prefix_enabled = false,
            }
            assert!(!try_merge(&mut actual,&right,&mut packed));
            assert_eq!(actual,left);
        }
        let mut a = vec![Projective::<G1Config>::generator().to_affine()];
        let b = a.clone();
        assert!(!Fq::try_merge_borrowed(&mut a,&b));
        assert_eq!(a,b);
    }

    #[test]
    fn public_merge_chunk_boundaries_and_small_stack_pools() {
        for n in [0,1,7,8,9,511,512,513,1025,2048] {
            let (left,right,expected) = operands(n,true);
            let original = right.clone();
            for workers in [1,4,16,32] {
                let pool = rayon::ThreadPoolBuilder::new().num_threads(workers)
                    .stack_size(512*1024).build().unwrap();
                DISPATCHES.store(0,Ordering::Relaxed);
                let mut actual = left.clone();
                pool.install(|| crate::msm_affine::merge_buckets(&mut actual,&right));
                assert_eq!(actual,expected,"n={n}, workers={workers}");
                assert_eq!(right,original);
                assert_eq!(DISPATCHES.load(Ordering::Relaxed),n.div_ceil(512));
            }
        }
    }
}

pub(super) fn dispatch<C: CurveConfig<Base = Fq2>>(
    left: &mut [Affine<C>], right: &[Affine<C>],
) -> bool {
    let result = try_merge(left,right,&mut BatchAdd::<C>::default());
    #[cfg(test)]
    if result { tests::DISPATCHES.fetch_add(1,std::sync::atomic::Ordering::Relaxed); }
    result
}
