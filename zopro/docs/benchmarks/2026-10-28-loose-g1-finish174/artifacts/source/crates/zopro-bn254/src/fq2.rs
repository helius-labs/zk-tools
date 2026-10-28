//! Fq2 = Fq[u] / (u^2 + 1).
use core::ops::{Add, Mul, Neg, Sub};

use crate::{Field, Fq, FqConfig, FpConfig};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
#[repr(C)]
pub struct Fq2 {
    pub c0: Fq,
    pub c1: Fq,
}

impl Fq2 {
    pub const ZERO: Self = Self::new(Fq::ZERO, Fq::ZERO);
    pub const ONE: Self = Self::new(Fq::ONE, Fq::ZERO);

    #[inline(always)]
    pub const fn new(c0: Fq, c1: Fq) -> Self {
        Self { c0, c1 }
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero()
    }

    #[inline(always)]
    pub fn double(&self) -> Self {
        Self::new(self.c0.double(), self.c1.double())
    }

    /// (c0 + c1)(c0 - c1) + 2 c0 c1 u.
    #[cfg_attr(target_arch = "wasm32", inline(never))]
    #[cfg_attr(not(target_arch = "wasm32"), inline(always))]
    pub fn square(&self) -> Self {
        let product = self.c0 * self.c1;
        Self::new((self.c0 + self.c1) * (self.c0 - self.c1), product.double())
    }

    #[inline(always)]
    pub fn mul_by_fp(&self, x: &Fq) -> Self {
        Self::new(self.c0 * *x, self.c1 * *x)
    }

    #[inline(always)]
    pub fn conjugate(&self) -> Self {
        Self::new(self.c0, -self.c1)
    }

    /// The p^power Frobenius: the conjugate for odd powers.
    #[inline(always)]
    pub fn frobenius(&self, power: usize) -> Self {
        if power % 2 == 1 {
            self.conjugate()
        } else {
            *self
        }
    }

    /// Times the sextic non-residue 9 + u: (9 c0 - c1) + (9 c1 + c0) u.
    #[inline(always)]
    pub fn mul_by_nonresidue(&self) -> Self {
        let eight = self.double().double().double();
        Self::new(eight.c0 + self.c0 - self.c1, eight.c1 + self.c1 + self.c0)
    }

    pub fn inverse(&self) -> Option<Self> {
        let norm = self.c0.square() + self.c1.square();
        norm.inverse().map(|t| Self::new(self.c0 * t, -(self.c1 * t)))
    }

    /// The square root, `None` for a non-residue: Algorithm 9 of Adj and
    /// Rodríguez-Henríquez, "Square root computation over even extension
    /// fields", for p = 3 mod 4.
    pub fn sqrt(&self) -> Option<Self> {
        if self.is_zero() {
            return Some(Self::ZERO);
        }
        // (p - 3) / 4 and (p - 1) / 2.
        const P_MINUS_3_OVER_4: [u64; 4] = shift(sub_small(FqConfig::P, 3), 2);
        const P_MINUS_1_OVER_2: [u64; 4] = shift(sub_small(FqConfig::P, 1), 1);
        let a1 = self.pow(&P_MINUS_3_OVER_4);
        let alpha = a1.square() * *self;
        let a0 = alpha.frobenius(1) * alpha;
        let minus_one = -Self::ONE;
        if a0 == minus_one {
            return None;
        }
        let x0 = a1 * *self;
        let root = if alpha == minus_one {
            Self::new(-x0.c1, x0.c0)
        } else {
            (alpha + Self::ONE).pow(&P_MINUS_1_OVER_2) * x0
        };
        (root.square() == *self).then_some(root)
    }

    /// self^e, e little-endian limbs.
    pub fn pow(&self, e: &[u64]) -> Self {
        let mut result = Self::ONE;
        for limb in e.iter().rev() {
            for bit in (0..64).rev() {
                result = result.square();
                if (limb >> bit) & 1 == 1 {
                    result = result * *self;
                }
            }
        }
        result
    }

    /// gnark's order on E2: by the second coordinate, by the first when it
    /// is zero.
    pub fn is_largest(&self) -> bool {
        if self.c1.is_zero() {
            self.c0.is_largest()
        } else {
            self.c1.is_largest()
        }
    }
}

const fn sub_small(x: [u64; 4], k: u64) -> [u64; 4] {
    crate::portable::sub4(x, [k, 0, 0, 0]).0
}

const fn shift(x: [u64; 4], k: u32) -> [u64; 4] {
    [
        (x[0] >> k) | (x[1] << (64 - k)),
        (x[1] >> k) | (x[2] << (64 - k)),
        (x[2] >> k) | (x[3] << (64 - k)),
        x[3] >> k,
    ]
}

impl Add for Fq2 {
    type Output = Self;
    #[inline(always)]
    fn add(self, o: Self) -> Self {
        Self::new(self.c0 + o.c0, self.c1 + o.c1)
    }
}
impl Sub for Fq2 {
    type Output = Self;
    #[inline(always)]
    fn sub(self, o: Self) -> Self {
        Self::new(self.c0 - o.c0, self.c1 - o.c1)
    }
}
impl Neg for Fq2 {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::new(-self.c0, -self.c1)
    }
}
impl Mul for Fq2 {
    type Output = Self;
    // Out of line in WebAssembly, as Fq's (fp.rs).
    #[cfg_attr(target_arch = "wasm32", inline)]
    #[cfg_attr(not(target_arch = "wasm32"), inline(always))]
    fn mul(self, o: Self) -> Self {
        mul(&self, &o)
    }
}
crate::assign_ops!(Fq2);

impl Field for Fq2 {
    const ZERO: Self = Fq2::ZERO;
    const ONE: Self = Fq2::ONE;
    #[inline(always)]
    fn is_zero(&self) -> bool {
        Fq2::is_zero(self)
    }
    #[inline(always)]
    fn square(&self) -> Self {
        Fq2::square(self)
    }
    #[inline(always)]
    fn double(&self) -> Self {
        Fq2::double(self)
    }
    fn inverse(&self) -> Option<Self> {
        Fq2::inverse(self)
    }
}

/// a b by Karatsuba with lazy reduction: three 512-bit products (a0 b0,
/// a1 b1, (a0 + a1)(b0 + b1)) combined before two Montgomery reductions.
/// c0 = a0 b0 - a1 b1 + p^2 and c1 = (a0 + a1)(b0 + b1) - a0 b0 - a1 b1 are
/// both below 2 p^2 < p 2^255 (19 ns rather than 21 for four products and
/// two reductions on an M5 Pro).
#[cfg(all(target_arch = "aarch64", not(no_asm)))]
#[inline(always)]
fn mul(a: &Fq2, b: &Fq2) -> Fq2 {
    use crate::arm64::{redc, sub_add_wide, sub_wide, wide};
    const P: [u64; 4] = FqConfig::P;
    const INV: u64 = FqConfig::INV;
    const P2: [u64; 8] = {
        let mut t = [0u64; 8];
        let mut i = 0;
        while i < 4 {
            let mut carry = 0u64;
            let mut j = 0;
            while j < 4 {
                let v = (P[i] as u128) * (P[j] as u128) + t[i + j] as u128 + carry as u128;
                t[i + j] = v as u64;
                carry = (v >> 64) as u64;
                j += 1;
            }
            t[i + 4] = carry;
            i += 1;
        }
        t
    };
    let t0 = wide(a.c0.0, &b.c0.0);
    let t1 = wide(a.c1.0, &b.c1.0);
    let sa = crate::portable::add4(a.c0.0, a.c1.0).0;
    let sb = crate::portable::add4(b.c0.0, b.c1.0).0;
    let t2 = wide(sa, &sb);
    let c0 = sub_add_wide(t0, t1, &P2);
    let c1 = sub_wide(sub_wide(t2, t0), t1);
    Fq2::new(
        Fq::from_montgomery(redc(c0, P, INV)),
        Fq::from_montgomery(redc(c1, P, INV)),
    )
}

/// a b as two sums of two products, one reduction each.
#[cfg(not(all(target_arch = "aarch64", not(no_asm))))]
#[cfg_attr(target_arch = "wasm32", inline(never))]
#[cfg_attr(not(target_arch = "wasm32"), inline(always))]
fn mul(a: &Fq2, b: &Fq2) -> Fq2 {
    Fq2::new(
        Fq::sum_of_products(&a.c0, &b.c0, &(-a.c1), &b.c1),
        Fq::sum_of_products(&a.c0, &b.c1, &a.c1, &b.c0),
    )
}
