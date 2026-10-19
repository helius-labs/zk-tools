use ark_ff::{fields::*, MontFp};

use crate::*;

pub type Fq2 = Fp2<Fq2Config>;

pub struct Fq2Config;

impl Fp2Config for Fq2Config {
    type Fp = Fq;

    /// NONRESIDUE = -1
    const NONRESIDUE: Fq = MontFp!("-1");

    /// Coefficients for the Frobenius automorphism.
    const FROBENIUS_COEFF_FP2_C1: &'static [Fq] = &[
        // NONRESIDUE**(((q^0) - 1) / 2)
        Fq::ONE,
        // NONRESIDUE**(((q^1) - 1) / 2)
        MontFp!("-1"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue_in_place(fe: &mut Self::Fp) -> &mut Self::Fp {
        fe.neg_in_place()
    }
}

/// a b in Fq2 by Karatsuba with lazy reduction: three 512-bit products
/// (a0 b0, a1 b1, (a0 + a1)(b0 + b1)) combined before two Montgomery
/// reductions, where arkworks' product takes four products and two
/// reductions. c0 = a0 b0 - a1 b1 + p^2 and c1 = (a0 + a1)(b0 + b1) - a0 b0
/// - a1 b1 are both below 2 p^2 < p 2^255. Elsewhere arkworks' product.
#[cfg(all(target_arch = "aarch64", not(no_asm)))]
#[inline(always)]
pub fn mul_lazy(a: &Fq2, b: &Fq2) -> Fq2 {
    use crate::mont_arm64::{redc, sub_add_wide, sub_wide, wide};
    use ark_ff::BigInt;
    const P: BigInt<4> = <FqConfig as MontConfig<4>>::MODULUS;
    const INV: u64 = <FqConfig as MontConfig<4>>::INV;
    const P2: [u64; 8] = {
        let p = P.0;
        let mut t = [0u64; 8];
        let mut i = 0;
        while i < 4 {
            let mut carry = 0u64;
            let mut j = 0;
            while j < 4 {
                let v = (p[i] as u128) * (p[j] as u128) + t[i + j] as u128 + carry as u128;
                t[i + j] = v as u64;
                carry = (v >> 64) as u64;
                j += 1;
            }
            t[i + 4] = carry;
            i += 1;
        }
        t
    };
    #[inline(always)]
    fn add<const N: usize>(a: &mut [u64; N], b: &[u64; N]) {
        let mut carry = false;
        for i in 0..N {
            let (s, c1) = a[i].overflowing_add(b[i]);
            let (s, c2) = s.overflowing_add(carry as u64);
            a[i] = s;
            carry = c1 | c2;
        }
    }
    let t0 = wide(a.c0.0, &b.c0.0);
    let t1 = wide(a.c1.0, &b.c1.0);
    let (mut sa, mut sb) = (a.c0.0 .0, b.c0.0 .0);
    add(&mut sa, &a.c1.0 .0);
    add(&mut sb, &b.c1.0 .0);
    let t2 = wide(BigInt(sa), &BigInt(sb));
    let c0 = sub_add_wide(t0, t1, &P2);
    let c1 = sub_wide(sub_wide(t2, t0), t1);
    Fq2::new(
        Fq::new_unchecked(redc(c0, P, INV)),
        Fq::new_unchecked(redc(c1, P, INV)),
    )
}

#[cfg(not(all(target_arch = "aarch64", not(no_asm))))]
#[inline(always)]
pub fn mul_lazy(a: &Fq2, b: &Fq2) -> Fq2 {
    *a * b
}
