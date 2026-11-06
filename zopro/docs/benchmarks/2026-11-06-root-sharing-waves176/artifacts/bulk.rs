//! Optimistic arithmetic only: no broker, queue, worker publication or waiting.
use zopro_bn254::{ifma8::Ifma8, Fq, FqConfig};
pub type V = Ifma8<FqConfig, true>;

// Exact accepted eight-lane product-tree structure, including canonical roots.
#[inline]
pub fn inverse8(a: &V) -> Option<V> {
    let swapped = V::shuffle_pair::<1, 0, 3, 2>(a, a);
    let pairs = a.mul(&swapped);
    let other_pair = V::shuffle_pair::<2, 3, 0, 1>(&pairs, &pairs);
    let quads = pairs.mul(&other_pair);
    let products = quads.get();
    let inverse = (products[0] * products[4]).inverse()?;
    let (lo, hi) = quads.halves();
    let other_quad = V::from_halves(&hi, &lo);
    Some(
        V::new(&[inverse; 8])
            .mul(&other_quad)
            .mul(&other_pair)
            .mul(&swapped),
    )
}

/// Invert the first n roots, where n is 1, 16 or 32. All must be nonzero.
/// Return canonical host fields. This does not wait for any other worker.
#[inline]
pub fn bulk(a: &[Fq; 32], n: usize) -> Option<[Fq; 32]> {
    let mut out = [Fq::ZERO; 32];
    if n == 1 {
        out[0] = a[0].inverse()?;
        return Some(out);
    }
    assert!(n == 16 || n == 32);
    let a0 = V::new(a[0..8].try_into().unwrap());
    let a1 = V::new(a[8..16].try_into().unwrap());
    let p01 = a0.mul(&a1);
    if n == 16 {
        let inv = inverse8(&p01)?;
        out[0..8].copy_from_slice(&inv.mul(&a1).get());
        out[8..16].copy_from_slice(&inv.mul(&a0).get());
    } else {
        let a2 = V::new(a[16..24].try_into().unwrap());
        let a3 = V::new(a[24..32].try_into().unwrap());
        let p23 = a2.mul(&a3);
        let inv = inverse8(&p01.mul(&p23))?;
        let i01 = inv.mul(&p23);
        let i23 = inv.mul(&p01);
        out[0..8].copy_from_slice(&i01.mul(&a1).get());
        out[8..16].copy_from_slice(&i01.mul(&a0).get());
        out[16..24].copy_from_slice(&i23.mul(&a3).get());
        out[24..32].copy_from_slice(&i23.mul(&a2).get());
    }
    Some(out)
}
