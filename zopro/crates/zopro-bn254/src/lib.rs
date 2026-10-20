//! BN254 for zopro: the base and scalar fields, the tower up to Fq12, G1
//! and G2, and the optimal ate pairing that verifies proofs.
//!
//! A field element is four 64-bit limbs in Montgomery form, the layout the
//! GPU kernels and gnark's key dumps share; an affine point is its two
//! coordinates, the point at infinity (0, 0) as gnark encodes it (neither
//! curve has a point with x = y = 0). Multiplication is assembly on
//! AArch64 and BMI2/ADX x86, AVX-512 IFMA in batches, 29-bit limbs in
//! WebAssembly, and 128-bit products elsewhere.
//! The formulas follow arkworks' (MIT OR Apache-2.0), against which the
//! tests check every operation.
#![deny(unsafe_op_in_unsafe_fn)]

#[cfg(all(target_arch = "aarch64", not(no_asm)))]
mod arm64;
pub mod curve;
pub mod fp;
pub mod fq12;
pub mod fq2;
pub mod fq6;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub mod ifma;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub mod ifma8;
#[cfg(all(
    feature = "fast-inverse",
    target_arch = "x86_64",
    target_feature = "bmi2",
    target_feature = "adx",
    not(no_asm)
))]
mod inverse;
#[cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]
mod mont29;
pub mod pairing;
pub(crate) mod portable;
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "bmi2",
    target_feature = "adx",
    not(no_asm)
))]
mod x86_64;

pub use curve::{
    Affine, CurveConfig, G1Affine, G1Config, G1Projective, G2Affine, G2Config, G2Projective,
    Projective,
};
pub use fp::{Fp, FpConfig};
pub use fq12::Fq12;
pub use fq2::Fq2;
pub use fq6::Fq6;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct FqConfig;
impl FpConfig for FqConfig {
    const P: [u64; 4] = portable::decimal(
        "21888242871839275222246405745257275088696311157297823662689037894645226208583",
    );
}
/// The base field.
pub type Fq = Fp<FqConfig>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default, Hash)]
pub struct FrConfig;
impl FpConfig for FrConfig {
    const P: [u64; 4] = portable::decimal(
        "21888242871839275222246405745257275088548364400416034343698204186575808495617",
    );
}
/// The scalar field.
pub type Fr = Fp<FrConfig>;

impl Fr {
    /// 2^28 divides r - 1: the largest power-of-two domain.
    pub const TWO_ADICITY: u32 = 28;
    /// A primitive 2^28-th root of unity: 5^((r - 1) / 2^28), 5 generating
    /// the multiplicative group.
    pub const TWO_ADIC_ROOT_OF_UNITY: Fr = Fr::from_decimal(
        "19103219067921713944291392827692070036145651957329286315305642004821462161904",
    );
}

impl Fq {
    /// The square root, `None` for a non-residue: p = 3 mod 4, so
    /// a^((p + 1) / 4) when it squares back.
    pub fn sqrt(&self) -> Option<Self> {
        const E: [u64; 4] = shr2(portable::add4(FqConfig::P, [1, 0, 0, 0]).0);
        let root = self.pow(&E);
        (root.square() == *self).then_some(root)
    }
}

const fn shr2(x: [u64; 4]) -> [u64; 4] {
    [
        (x[0] >> 2) | (x[1] << 62),
        (x[1] >> 2) | (x[2] << 62),
        (x[2] >> 2) | (x[3] << 62),
        x[3] >> 2,
    ]
}

/// The arithmetic each target multiplies with.
pub(crate) mod backend {
    #[cfg(all(target_arch = "aarch64", not(no_asm)))]
    pub use crate::arm64::{add, multiply, neg, sub, sum2};
    #[cfg(not(any(
        all(target_arch = "aarch64", not(no_asm)),
        all(
            target_arch = "x86_64",
            target_feature = "bmi2",
            target_feature = "adx",
            not(no_asm)
        )
    )))]
    pub use crate::portable::{add, neg, sub};
    #[cfg(all(
        not(target_arch = "wasm32"),
        not(all(target_arch = "aarch64", not(no_asm))),
        not(all(
            target_arch = "x86_64",
            target_feature = "bmi2",
            target_feature = "adx",
            not(no_asm)
        ))
    ))]
    pub use crate::portable::{multiply, sum2};
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "bmi2",
        target_feature = "adx",
        not(no_asm)
    ))]
    pub use crate::x86_64::{add, multiply, neg, sub, sum2};

    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn multiply(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
        crate::mont29::multiply(a, b, p, inv)
    }
    #[cfg(target_arch = "wasm32")]
    #[inline]
    pub fn sum2(
        a: [u64; 4],
        b: [u64; 4],
        c: [u64; 4],
        d: [u64; 4],
        p: [u64; 4],
        inv: u64,
    ) -> [u64; 4] {
        crate::mont29::sum2([a, c], [b, d], p, inv)
    }
}

/// `+=`, `-=` and `*=` from `+`, `-` and `*`.
#[macro_export]
#[doc(hidden)]
macro_rules! assign_ops {
    ($t:ty $(, $g:ident : $b:path)?) => {
        impl$(<$g: $b>)? core::ops::AddAssign for $t {
            #[inline(always)]
            fn add_assign(&mut self, other: Self) {
                *self = *self + other;
            }
        }
        impl$(<$g: $b>)? core::ops::SubAssign for $t {
            #[inline(always)]
            fn sub_assign(&mut self, other: Self) {
                *self = *self - other;
            }
        }
        impl$(<$g: $b>)? core::ops::MulAssign for $t {
            #[inline(always)]
            fn mul_assign(&mut self, other: Self) {
                *self = *self * other;
            }
        }
    };
}

/// What the generic code (MSMs, curves) needs of a field.
pub trait Field:
    'static
    + Copy
    + Send
    + Sync
    + Eq
    + core::fmt::Debug
    + Default
    + core::ops::Add<Output = Self>
    + core::ops::Sub<Output = Self>
    + core::ops::Mul<Output = Self>
    + core::ops::Neg<Output = Self>
    + core::ops::AddAssign
    + core::ops::SubAssign
    + core::ops::MulAssign
{
    const ZERO: Self;
    const ONE: Self;
    fn is_zero(&self) -> bool;
    fn square(&self) -> Self;
    fn double(&self) -> Self;
    fn inverse(&self) -> Option<Self>;
}

impl<C: FpConfig> Field for Fp<C> {
    const ZERO: Self = Fp::<C>::ZERO;
    const ONE: Self = Fp::<C>::ONE;
    #[inline(always)]
    fn is_zero(&self) -> bool {
        Fp::is_zero(self)
    }
    #[inline(always)]
    fn square(&self) -> Self {
        Fp::square(self)
    }
    #[inline(always)]
    fn double(&self) -> Self {
        Fp::double(self)
    }
    fn inverse(&self) -> Option<Self> {
        Fp::inverse(self)
    }
}

#[cfg(test)]
mod tests;

impl Fr {
    /// A primitive n-th root of unity for a power of two n up to 2^28: the
    /// generator of arkworks' and gnark's FFT domains of that size.
    pub fn root_of_unity(n: usize) -> Option<Fr> {
        if !n.is_power_of_two() || n.trailing_zeros() > Self::TWO_ADICITY {
            return None;
        }
        let mut root = Self::TWO_ADIC_ROOT_OF_UNITY;
        for _ in n.trailing_zeros()..Self::TWO_ADICITY {
            root = root.square();
        }
        Some(root)
    }
}
