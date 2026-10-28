//! The prime fields: an element is four little-endian 64-bit limbs in
//! Montgomery form (a 2^256 mod p), the layout the GPU kernels read and
//! gnark's key dumps hold.
use core::{
    fmt,
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

use crate::{backend, portable};

/// A modulus and its Montgomery constants (all derived from `P`).
pub trait FpConfig: 'static + Copy + Send + Sync + Eq + fmt::Debug + Default {
    const P: [u64; 4];
    /// -p^-1 mod 2^64.
    const INV: u64 = portable::inv64(Self::P[0]);
    /// 2^256 mod p: one in Montgomery form.
    const R: [u64; 4] = portable::power_of_two(256, Self::P);
    /// 2^512 mod p, which takes a value into Montgomery form.
    const R2: [u64; 4] = portable::power_of_two(512, Self::P);
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
#[repr(transparent)]
pub struct Fp<C: FpConfig>(pub [u64; 4], PhantomData<C>);

impl<C: FpConfig> Fp<C> {
    pub const ZERO: Self = Self([0; 4], PhantomData);
    pub const ONE: Self = Self(C::R, PhantomData);
    pub const MODULUS: [u64; 4] = C::P;

    /// An element from its Montgomery form, unchecked.
    #[inline(always)]
    pub const fn from_montgomery(limbs: [u64; 4]) -> Self {
        Self(limbs, PhantomData)
    }

    /// The element of a value below p, `None` above.
    #[inline]
    pub const fn from_canonical(limbs: [u64; 4]) -> Option<Self> {
        if !portable::less(limbs, C::P) {
            return None;
        }
        Some(Self(
            portable::multiply(limbs, C::R2, C::P, C::INV),
            PhantomData,
        ))
    }

    /// The element of a decimal literal below p (for constants).
    pub const fn from_decimal(s: &str) -> Self {
        match Self::from_canonical(portable::decimal(s)) {
            Some(x) => x,
            None => panic!("a literal below the modulus"),
        }
    }

    #[inline]
    pub const fn from_u64(x: u64) -> Self {
        Self(
            portable::multiply([x, 0, 0, 0], C::R2, C::P, C::INV),
            PhantomData,
        )
    }

    /// The value, below p.
    #[inline]
    pub fn to_canonical(&self) -> [u64; 4] {
        backend::multiply(self.0, [1, 0, 0, 0], C::P, C::INV)
    }

    /// -self as a constant expression.
    pub const fn const_neg(self) -> Self {
        Self(portable::neg(self.0, C::P), PhantomData)
    }

    #[inline(always)]
    pub fn is_zero(&self) -> bool {
        self.0 == [0; 4]
    }

    #[cfg_attr(target_arch = "wasm32", inline)]
    #[cfg_attr(not(target_arch = "wasm32"), inline(always))]
    pub fn square(&self) -> Self {
        #[cfg(all(zopro_scalar_symmetric_square, target_arch = "x86_64", target_feature = "bmi2", target_feature = "adx", not(no_asm)))]
        if C::P == <crate::FrConfig as FpConfig>::P {
            return Self(crate::scalar_square::square::<C>(self.0), PhantomData);
        }
        Self(backend::multiply(self.0, self.0, C::P, C::INV), PhantomData)
    }

    #[inline(always)]
    pub fn double(&self) -> Self {
        Self(backend::add(self.0, self.0, C::P), PhantomData)
    }

    /// a b + c d with one reduction.
    #[cfg_attr(target_arch = "wasm32", inline)]
    #[cfg_attr(not(target_arch = "wasm32"), inline(always))]
    pub fn sum_of_products(a: &Self, b: &Self, c: &Self, d: &Self) -> Self {
        Self(backend::sum2(a.0, b.0, c.0, d.0, C::P, C::INV), PhantomData)
    }

    /// self^e, e little-endian limbs.
    pub fn pow(&self, e: &[u64]) -> Self {
        let mut result = Self::ONE;
        for limb in e.iter().rev() {
            for bit in (0..64).rev() {
                result = result.square();
                if (limb >> bit) & 1 == 1 {
                    result *= *self;
                }
            }
        }
        result
    }

    /// The inverse, `None` for zero. x86 batches the binary GCD; other
    /// targets use the binary extended Euclidean algorithm. Starting from
    /// 2^512 leaves the inverse in Montgomery form in both cases.
    pub fn inverse(&self) -> Option<Self> {
        #[cfg(all(
            feature = "fast-inverse",
            target_arch = "x86_64",
            target_feature = "bmi2",
            target_feature = "adx",
            not(no_asm)
        ))]
        {
            return crate::inverse::inverse(self);
        }
        #[cfg(not(all(
            feature = "fast-inverse",
            target_arch = "x86_64",
            target_feature = "bmi2",
            target_feature = "adx",
            not(no_asm)
        )))]
        {
            return self.inverse_binary();
        }
    }

    #[cfg(not(all(
        feature = "fast-inverse",
        target_arch = "x86_64",
        target_feature = "bmi2",
        target_feature = "adx",
        not(no_asm)
    )))]
    fn inverse_binary(&self) -> Option<Self> {
        if self.is_zero() {
            return None;
        }
        let one = [1u64, 0, 0, 0];
        let p = C::P;
        let mut u = self.0;
        let mut v = p;
        let mut b = Self(C::R2, PhantomData);
        let mut c = Self::ZERO;
        let halve = |x: &mut Self| {
            if x.0[0] & 1 == 1 {
                // x + p fits: p < 2^254.
                x.0 = portable::add4(x.0, p).0;
            }
            x.0 = shr1(x.0);
        };
        while u != one && v != one {
            while u[0] & 1 == 0 {
                u = shr1(u);
                halve(&mut b);
            }
            while v[0] & 1 == 0 {
                v = shr1(v);
                halve(&mut c);
            }
            if portable::less(v, u) {
                u = portable::sub4(u, v).0;
                b -= c;
            } else {
                v = portable::sub4(v, u).0;
                c -= b;
            }
        }
        Some(if u == one { b } else { c })
    }

    /// Whether the value is above (p - 1) / 2, gnark's "lexicographically
    /// largest".
    pub fn is_largest(&self) -> bool {
        let half = shr1(portable::sub4(C::P, [1, 0, 0, 0]).0);
        portable::less(half, self.to_canonical())
    }

    /// The value as 32 big-endian bytes.
    pub fn to_be_bytes(&self) -> [u8; 32] {
        let limbs = self.to_canonical();
        let mut out = [0u8; 32];
        for (i, limb) in limbs.iter().rev().enumerate() {
            out[8 * i..8 * i + 8].copy_from_slice(&limb.to_be_bytes());
        }
        out
    }

    /// The element of 32 big-endian bytes, `None` at or above p.
    pub fn from_be_bytes(bytes: &[u8; 32]) -> Option<Self> {
        Self::from_canonical(be_limbs(bytes))
    }

    /// The element of big-endian bytes of any length, reduced mod p.
    pub fn from_be_bytes_mod_order(bytes: &[u8]) -> Self {
        let mut result = Self::ZERO;
        let base = Self::from_u64(256);
        for &byte in bytes {
            result = result * base + Self::from_u64(byte as u64);
        }
        result
    }

    /// The number of bits of the value.
    pub fn num_bits(&self) -> u32 {
        let limbs = self.to_canonical();
        for i in (0..4).rev() {
            if limbs[i] != 0 {
                return 64 * i as u32 + 64 - limbs[i].leading_zeros();
            }
        }
        0
    }
}

/// Little-endian limbs of 32 big-endian bytes.
pub fn be_limbs(bytes: &[u8; 32]) -> [u64; 4] {
    core::array::from_fn(|i| {
        let at = 32 - 8 * (i + 1);
        u64::from_be_bytes(bytes[at..at + 8].try_into().unwrap())
    })
}

#[inline(always)]
const fn shr1(x: [u64; 4]) -> [u64; 4] {
    [
        (x[0] >> 1) | (x[1] << 63),
        (x[1] >> 1) | (x[2] << 63),
        (x[2] >> 1) | (x[3] << 63),
        x[3] >> 1,
    ]
}

impl<C: FpConfig> fmt::Debug for Fp<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let limbs = self.to_canonical();
        write!(
            f,
            "0x{:016x}{:016x}{:016x}{:016x}",
            limbs[3], limbs[2], limbs[1], limbs[0]
        )
    }
}

impl<C: FpConfig> Add for Fp<C> {
    type Output = Self;
    #[inline(always)]
    fn add(self, other: Self) -> Self {
        Self(backend::add(self.0, other.0, C::P), PhantomData)
    }
}
impl<C: FpConfig> Sub for Fp<C> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, other: Self) -> Self {
        Self(backend::sub(self.0, other.0, C::P), PhantomData)
    }
}
impl<C: FpConfig> Mul for Fp<C> {
    type Output = Self;
    // In WebAssembly the 29-bit multiply is a few kilobytes: inlined into
    // every point formula it took the module from 7.4 MB to 7.9.
    #[cfg_attr(target_arch = "wasm32", inline)]
    #[cfg_attr(not(target_arch = "wasm32"), inline(always))]
    fn mul(self, other: Self) -> Self {
        Self(
            backend::multiply(self.0, other.0, C::P, C::INV),
            PhantomData,
        )
    }
}
impl<C: FpConfig> Neg for Fp<C> {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self(backend::neg(self.0, C::P), PhantomData)
    }
}
crate::assign_ops!(Fp<C>, C: FpConfig);

impl<C: FpConfig> From<u64> for Fp<C> {
    #[inline]
    fn from(x: u64) -> Self {
        Self::from_u64(x)
    }
}

impl<C: FpConfig> Fp<C> {
    /// A uniform element from 64 random bytes: their value mod p, whose
    /// bias is below 2^-250.
    pub fn from_random_bytes(bytes: &[u8; 64]) -> Self {
        Self::from_be_bytes_mod_order(bytes)
    }
}

impl<C: FpConfig> Fp<C> {
    /// The modulus's bit length.
    pub const MODULUS_BITS: u32 = 256 - C::P[3].leading_zeros();
}

impl<C: FpConfig> Fp<C> {
    /// The element of little-endian bytes of any length, reduced mod p.
    pub fn from_le_bytes_mod_order(bytes: &[u8]) -> Self {
        let reversed: Vec<u8> = bytes.iter().rev().copied().collect();
        Self::from_be_bytes_mod_order(&reversed)
    }
}

impl<C: FpConfig> Fp<C> {
    /// Whether the representation is below p, as every element built by
    /// this crate's arithmetic is (not checked by `from_montgomery`).
    pub fn is_reduced(&self) -> bool {
        portable::less(self.0, C::P)
    }
}

impl<C: FpConfig> Fp<C> {
    /// The value's decimal digits.
    pub fn to_decimal(&self) -> String {
        let mut limbs = self.to_canonical();
        let mut chunks = Vec::new();
        // Divide by 10^19 until zero, keeping the remainders.
        const BASE: u128 = 10_000_000_000_000_000_000;
        loop {
            let mut remainder = 0u128;
            for limb in limbs.iter_mut().rev() {
                let value = (remainder << 64) | *limb as u128;
                *limb = (value / BASE) as u64;
                remainder = value % BASE;
            }
            chunks.push(remainder as u64);
            if limbs == [0; 4] {
                break;
            }
        }
        let mut out = chunks.pop().unwrap().to_string();
        for chunk in chunks.iter().rev() {
            out.push_str(&format!("{chunk:019}"));
        }
        out
    }
}
