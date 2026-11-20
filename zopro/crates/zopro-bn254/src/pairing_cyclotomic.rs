//! Square the three Fq4 coordinate pairs of one cyclotomic Fq12.
//! Each of the first six lanes holds one Fq coefficient of three Fq2 values.
use crate::{ifma8::Ifma8, Fq, Fq12, Fq2, Fq6, FqConfig};
use core::{arch::x86_64::*, marker::PhantomData};

type Eight = Ifma8<FqConfig>;
const IMAGINARY: u8 = 0b1010_1010;

#[derive(Clone, Copy)]
struct CoordinatePairs {
    a: Eight,
    b: Eight,
}

#[inline(always)]
fn permute2(a: Eight, b: Eight, lanes: [i64; 8]) -> Eight {
    // SAFETY: this module is compiled only on AVX-512 IFMA/VL/DQ. All
    // operations preserve the normalized vector digits and Montgomery radix.
    unsafe {
        let indices = _mm512_loadu_si512(lanes.as_ptr().cast());
        Ifma8(
            core::array::from_fn(|i| _mm512_permutex2var_epi64(a.0[i], indices, b.0[i])),
            PhantomData,
        )
    }
}

#[inline(always)]
fn select(a: Eight, b: Eight, use_b: u8) -> Eight {
    // SAFETY: see permute2.
    unsafe {
        Ifma8(
            core::array::from_fn(|i| _mm512_mask_blend_epi64(use_b, a.0[i], b.0[i])),
            PhantomData,
        )
    }
}

#[inline(always)]
fn swap(x: Eight) -> Eight {
    // SAFETY: see permute2.
    unsafe {
        let indices = _mm512_set_epi64(6, 7, 4, 5, 2, 3, 0, 1);
        Ifma8(
            x.0.map(|word| _mm512_permutexvar_epi64(indices, word)),
            PhantomData,
        )
    }
}

#[inline(always)]
fn square_pairs(x: Eight) -> Eight {
    let other = swap(x);
    let left = select(x.add(&other), x.add(&x), IMAGINARY);
    let right = select(x.sub(&other), other, IMAGINARY);
    left.mul(&right)
}

#[inline(always)]
fn nonresidue(x: Eight) -> Eight {
    let nine = x.mul_u28(&[9; 8]);
    let other = swap(x);
    select(nine.sub(&other), nine.add(&other), IMAGINARY)
}

#[inline(always)]
fn adjust(x: Eight, old: Eight, positive: u8) -> Eight {
    let twice = old.add(&old);
    let triple = x.mul_u28(&[3; 8]);
    select(triple.sub(&twice), triple.add(&twice), positive)
}

impl CoordinatePairs {
    #[inline]
    fn new(x: Fq12) -> Self {
        let pack = |x: Fq2, y: Fq2, z: Fq2| {
            Eight::new(&[x.c0, x.c1, y.c0, y.c1, z.c0, z.c1, Fq::ZERO, Fq::ZERO])
        };
        Self {
            a: pack(x.c0.c0, x.c1.c0, x.c0.c1),
            b: pack(x.c1.c1, x.c0.c2, x.c1.c2),
        }
    }

    #[inline]
    fn get(self) -> Fq12 {
        let a = self.a.get();
        let b = self.b.get();
        Fq12::new(
            Fq6::new(
                Fq2::new(a[0], a[1]),
                Fq2::new(a[4], a[5]),
                Fq2::new(b[2], b[3]),
            ),
            Fq6::new(
                Fq2::new(a[2], a[3]),
                Fq2::new(b[0], b[1]),
                Fq2::new(b[4], b[5]),
            ),
        )
    }

    #[inline(never)]
    fn square(self) -> Self {
        let aa = square_pairs(self.a);
        let bb = square_pairs(self.b);
        let real = aa.add(&nonresidue(bb));
        let imaginary = square_pairs(self.a.add(&self.b)).sub(&aa).sub(&bb);
        let scaled_imaginary = nonresidue(imaginary);
        let a = permute2(real, scaled_imaginary, [0, 1, 12, 13, 2, 3, 6, 7]);
        let b = permute2(real, imaginary, [8, 9, 4, 5, 10, 11, 6, 7]);
        Self {
            a: adjust(a, self.a, 0b0000_1100),
            b: adjust(b, self.b, 0b0011_0011),
        }
    }
}

// Fq12 multiplication retains the existing scalar implementation. Keep
// vectors live between consecutive squarings of the same element.
#[inline(never)]
pub(super) fn pow(x: Fq12, exponent: u64) -> Fq12 {
    if exponent == 0 {
        return Fq12::ONE;
    }
    let mut power = CoordinatePairs::new(x);
    for bit in (0..(63 - exponent.leading_zeros())).rev() {
        power = power.square();
        if (exponent >> bit) & 1 != 0 {
            power = CoordinatePairs::new(power.get() * x);
        }
    }
    power.get()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Fr;
    use crate::{
        pairing::{
            final_exponentiation, final_exponentiation_with_packing, multi_miller_loop, G2Prepared,
        },
        G1Affine, G2Affine,
    };

    #[test]
    fn packed_cyclotomic_squares_and_powers_match_scalar() {
        assert_eq!(
            final_exponentiation_with_packing(Fq12::default(), true),
            None
        );
        let g1 = G1Affine::GENERATOR;
        let g2 = G2Affine::GENERATOR;
        let mut seed = 0x53c078976ccd96b9u64;
        for index in 0..24 {
            let f = if index == 0 {
                Fq12::ONE
            } else {
                let p = g1.mul(&Fr::from_u64(index)).to_affine();
                let q = g2.mul(&Fr::from_u64(87 - index)).to_affine();
                multi_miller_loop(&[(p, &G2Prepared::new(&q))])
            };
            assert_eq!(
                final_exponentiation_with_packing(f, true),
                final_exponentiation(f)
            );
            let unitary = f.conjugate() * f.inverse().unwrap();
            let x = unitary.frobenius(2) * unitary;
            let mut reference = x;
            let mut packed = CoordinatePairs::new(x);
            for _ in 0..96 {
                packed = packed.square();
                reference = reference.square();
                assert_eq!(packed.get(), reference);
            }
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            for exponent in [0, 1, 2, 3, 4965661367192848881, u64::MAX, seed] {
                assert_eq!(pow(x, exponent), x.pow_u64(exponent));
            }
        }
    }
}
