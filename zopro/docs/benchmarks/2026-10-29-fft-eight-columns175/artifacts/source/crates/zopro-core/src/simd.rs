// Four independent field elements at once. In WebAssembly with SIMD,
// mont29_simd.s (scripts/mont29_simd.py) runs zopro-bn254's mont29
// multiplication on four lanes, and this module adds, subtracts and
// shuffles in the same lane form; elsewhere the lanes run one by one.
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
use core::marker::PhantomData;
use zopro_bn254::{FpConfig, Fq, FqConfig, Fr, FrConfig};

/// A BN254 field the generated multiplication covers.
#[cfg_attr(
    not(all(target_arch = "wasm32", target_feature = "simd128")),
    allow(dead_code)
)]
pub(crate) trait Mont29: Copy + zopro_bn254::Field {
    type Config: FpConfig;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    type Native: NativeVector<Self>;
    fn modulus() -> [u64; 4];
    fn repr(self) -> [u64; 4];
    fn from_repr(repr: [u64; 4]) -> Self;
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    const CORE: unsafe extern "C" fn(*const u32, *const u32, *mut u32);
}
impl Mont29 for Fq {
    type Config = FqConfig;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    type Native = zopro_bn254::ifma::Ifma<FqConfig, true>;
    #[inline(always)]
    fn modulus() -> [u64; 4] {
        FqConfig::P
    }
    #[inline(always)]
    fn repr(self) -> [u64; 4] {
        self.0
    }
    #[inline(always)]
    fn from_repr(repr: [u64; 4]) -> Self {
        Fq::from_montgomery(repr)
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    const CORE: unsafe extern "C" fn(*const u32, *const u32, *mut u32) = lanes::mont29_mul4_fq;
}
impl Mont29 for Fr {
    type Config = FrConfig;
    #[cfg(all(
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    type Native = zopro_bn254::ifma::Ifma<FrConfig, false>;
    #[inline(always)]
    fn modulus() -> [u64; 4] {
        FrConfig::P
    }
    #[inline(always)]
    fn repr(self) -> [u64; 4] {
        self.0
    }
    #[inline(always)]
    fn from_repr(repr: [u64; 4]) -> Self {
        Fr::from_montgomery(repr)
    }
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
    const CORE: unsafe extern "C" fn(*const u32, *const u32, *mut u32) = lanes::mont29_mul4_fr;
}

/// Four elements in lane form. In WebAssembly with SIMD, limb j (29 bits)
/// of the four elements lies in one u32x4, so a chain of field operations
/// splits and joins its operands once rather than at every product; values
/// stay reduced. Elsewhere it holds the four elements.
#[derive(Clone, Copy)]
#[cfg(not(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
)))]
#[cfg_attr(
    not(all(target_arch = "wasm32", target_feature = "simd128")),
    repr(transparent)
)]
pub(crate) struct Lanes<F: Mont29>(
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))] [core::arch::wasm32::v128; 9],
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))] [F; 4],
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))] PhantomData<F>,
);
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
#[derive(Clone, Copy)]
pub(crate) struct Lanes<F: Mont29>(F::Native, core::marker::PhantomData<F>);

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
impl<F: Mont29> Lanes<F> {
    #[inline(always)]
    pub(crate) fn new(a: &[F; 4]) -> Self {
        Self(F::Native::new(a), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn get(&self) -> [F; 4] {
        self.0.get()
    }
    #[inline(always)]
    pub(crate) fn one() -> Self {
        Self(F::Native::one(), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn zero() -> Self {
        Self(F::Native::zero(), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn mul(&self, b: &Self) -> Self {
        Self(self.0.mul(&b.0), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn sum2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        Self(self.0.sum2(&b.0, &c.0, &d.0), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn difference2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        Self(
            self.0.difference2(&b.0, &c.0, &d.0),
            core::marker::PhantomData,
        )
    }
    #[inline(always)]
    pub(crate) fn add(&self, b: &Self) -> Self {
        Self(self.0.add(&b.0), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn sub(&self, b: &Self) -> Self {
        Self(self.0.sub(&b.0), core::marker::PhantomData)
    }
    #[inline(always)]
    pub(crate) fn shuffle<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        a: &Self,
        b: &Self,
    ) -> Self {
        Self(
            F::Native::shuffle::<I0, I1, I2, I3>(&a.0, &b.0),
            core::marker::PhantomData,
        )
    }
}

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) trait NativeVector<F: Mont29>: Copy + Send + Sync {
    fn new(a: &[F; 4]) -> Self;
    fn get(&self) -> [F; 4];
    fn one() -> Self;
    fn zero() -> Self;
    fn mul(&self, b: &Self) -> Self;
    fn sum2(&self, b: &Self, c: &Self, d: &Self) -> Self;
    fn difference2(&self, b: &Self, c: &Self, d: &Self) -> Self;
    fn add(&self, b: &Self) -> Self;
    fn sub(&self, b: &Self) -> Self;
    fn shuffle<const A: usize, const B: usize, const C: usize, const D: usize>(
        a: &Self,
        b: &Self,
    ) -> Self;
}
#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
impl<F: Mont29, const MIXED: bool> NativeVector<F> for zopro_bn254::ifma::Ifma<F::Config, MIXED> {
    #[inline(always)]
    fn new(a: &[F; 4]) -> Self {
        Self::new(&a.map(|v| zopro_bn254::Fp::from_montgomery(v.repr())))
    }
    #[inline(always)]
    fn get(&self) -> [F; 4] {
        self.get().map(|v| F::from_repr(v.0))
    }
    #[inline(always)]
    fn one() -> Self {
        Self::one()
    }
    #[inline(always)]
    fn zero() -> Self {
        Self::zero()
    }
    #[inline(always)]
    fn mul(&self, b: &Self) -> Self {
        self.mul(b)
    }
    #[inline(always)]
    fn sum2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        self.sum2(b, c, d)
    }
    #[inline(always)]
    fn difference2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        self.difference2(b, c, d)
    }
    #[inline(always)]
    fn add(&self, b: &Self) -> Self {
        self.add(b)
    }
    #[inline(always)]
    fn sub(&self, b: &Self) -> Self {
        self.sub(b)
    }
    #[inline(always)]
    fn shuffle<const A: usize, const B: usize, const C: usize, const D: usize>(
        a: &Self,
        b: &Self,
    ) -> Self {
        Self::shuffle::<A, B, C, D>(a, b)
    }
}

pub(crate) type FqLanes = Lanes<Fq>;
pub(crate) type FrLanes = Lanes<Fr>;

#[cfg(all(
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
impl FrLanes {
    #[inline(always)]
    pub(crate) fn pair(a: &Self, b: &Self) -> zopro_bn254::ifma8::Ifma8<FrConfig> {
        zopro_bn254::ifma8::Ifma8::from_halves(&a.0, &b.0)
    }
    #[inline(always)]
    pub(crate) fn unpair(pair: &zopro_bn254::ifma8::Ifma8<FrConfig>) -> (Self, Self) {
        let (a, b) = pair.halves();
        (
            Self(a, core::marker::PhantomData),
            Self(b, core::marker::PhantomData),
        )
    }
}

#[cfg(not(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
)))]
pub(crate) type FqBatch = FqLanes;
#[cfg(not(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
)))]
pub(crate) const BATCH_LANES: usize = 4;
#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
pub(crate) const BATCH_LANES: usize = 8;
#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
#[derive(Clone, Copy)]
pub(crate) struct FqBatch(zopro_bn254::ifma8::Ifma8<FqConfig, true>);
#[cfg(all(
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
impl FqBatch {
    #[inline(always)]
    pub(crate) fn signed_difference_scaled(&self, ay:&Self, scale:&Self, negative:u8)->Self {
        Self(self.0.signed_difference_scaled(&ay.0,&scale.0,negative))
    }
    #[inline(always)]
    pub(crate) fn new(a: &[Fq; 8]) -> Self {
        Self(zopro_bn254::ifma8::Ifma8::new(a))
    }
    #[inline(always)]
    pub(crate) fn get(&self) -> [Fq; 8] {
        self.0.get()
    }
    /// Invert eight nonzero values with a product tree and one scalar inverse.
    #[inline]
    pub(crate) fn inverse_nonzero(&self) -> Self {
        use zopro_bn254::ifma8::Ifma8;
        let swapped = Self(Ifma8::shuffle_pair::<1, 0, 3, 2>(&self.0, &self.0));
        let pairs = self.mul(&swapped);
        let other_pair = Self(Ifma8::shuffle_pair::<2, 3, 0, 1>(&pairs.0, &pairs.0));
        let quads = pairs.mul(&other_pair);
        let products = quads.get();
        let inverse = (products[0] * products[4]).inverse().unwrap();
        let (lo, hi) = quads.0.halves();
        let other_quad = Self(Ifma8::from_halves(&hi, &lo));
        Self::new(&[inverse; 8])
            .mul(&other_quad)
            .mul(&other_pair)
            .mul(&swapped)
    }
    #[inline(always)]
    pub(crate) fn one() -> Self {
        Self(zopro_bn254::ifma8::Ifma8::one())
    }
    #[inline(always)]
    pub(crate) fn zero() -> Self {
        Self(zopro_bn254::ifma8::Ifma8::zero())
    }
    #[inline(always)]
    pub(crate) fn mul(&self, b: &Self) -> Self {
        Self(self.0.mul(&b.0))
    }
    #[inline(always)]
    pub(crate) fn add(&self, b: &Self) -> Self {
        Self(self.0.add(&b.0))
    }
    #[inline(always)]
    pub(crate) fn sub(&self, b: &Self) -> Self {
        Self(self.0.sub(&b.0))
    }
    #[inline(always)]
    pub(crate) fn add_canonical(&self, b: &Self) -> Self {
        Self(self.0.add_canonical(&b.0))
    }
    #[inline(always)]
    pub(crate) fn sub_canonical(&self, b: &Self) -> Self {
        Self(self.0.sub_canonical(&b.0))
    }
    #[inline(always)]
    pub(crate) fn sum2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        Self(self.0.sum2(&b.0, &c.0, &d.0))
    }
    #[inline(always)]
    pub(crate) fn difference2(&self, b: &Self, c: &Self, d: &Self) -> Self {
        Self(self.0.difference2(&b.0, &c.0, &d.0))
    }
    #[inline(always)]
    pub(crate) fn load(a: [&FqLimbs; 8]) -> Self {
        Self::new(&a.map(|v| v.0))
    }
    #[inline(always)]
    pub(crate) fn store(&self) -> [FqLimbs; 8] {
        self.get().map(FqLimbs)
    }
}

/// `a[i] * b[i]` in Fq for four lanes.
#[inline(always)]
pub(crate) fn fq_mul4(a: &[Fq; 4], b: &[Fq; 4]) -> [Fq; 4] {
    FqLanes::new(a).mul(&FqLanes::new(b)).get()
}

#[cfg(not(any(
    all(target_arch = "wasm32", target_feature = "simd128"),
    all(
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )
)))]
impl<F: Mont29> Lanes<F> {
    #[inline(always)]
    pub(crate) fn new(a: &[F; 4]) -> Self {
        Self(*a)
    }
    #[inline(always)]
    pub(crate) fn get(&self) -> [F; 4] {
        self.0
    }
    #[inline(always)]
    pub(crate) fn one() -> Self {
        Self([F::ONE; 4])
    }
    #[inline(always)]
    pub(crate) fn zero() -> Self {
        Self([F::ZERO; 4])
    }
    #[inline(always)]
    pub(crate) fn mul(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i] * b.0[i]))
    }
    #[inline(always)]
    pub(crate) fn add(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i] + b.0[i]))
    }
    #[inline(always)]
    pub(crate) fn sub(&self, b: &Self) -> Self {
        Self(core::array::from_fn(|i| self.0[i] - b.0[i]))
    }
    /// Lanes I0..I3 of `a` followed by `b` (indices 0..8).
    #[inline(always)]
    pub(crate) fn shuffle<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        a: &Self,
        b: &Self,
    ) -> Self {
        let both = [
            a.0[0], a.0[1], a.0[2], a.0[3], b.0[0], b.0[1], b.0[2], b.0[3],
        ];
        Self([both[I0], both[I1], both[I2], both[I3]])
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
impl<F: Mont29> Lanes<F> {
    #[inline(always)]
    pub(crate) fn new(a: &[F; 4]) -> Self {
        Self(lanes::limbs(&a.map(F::repr)), PhantomData)
    }
    #[inline(always)]
    pub(crate) fn get(&self) -> [F; 4] {
        lanes::words(&self.0).map(F::from_repr)
    }
    #[inline(always)]
    pub(crate) fn one() -> Self {
        Self::new(&[F::ONE; 4])
    }
    #[inline(always)]
    pub(crate) fn zero() -> Self {
        Self([core::arch::wasm32::u32x4_splat(0); 9], PhantomData)
    }
    #[inline(always)]
    pub(crate) fn mul(&self, b: &Self) -> Self {
        Self(
            lanes::product(F::CORE, &self.0, &b.0, &lanes::modulus::<F>()),
            PhantomData,
        )
    }
    #[inline(always)]
    pub(crate) fn add(&self, b: &Self) -> Self {
        Self(
            lanes::add(&self.0, &b.0, &lanes::modulus::<F>()),
            PhantomData,
        )
    }
    #[inline(always)]
    pub(crate) fn sub(&self, b: &Self) -> Self {
        Self(
            lanes::sub(&self.0, &b.0, &lanes::modulus::<F>()),
            PhantomData,
        )
    }
    /// Lanes I0..I3 of `a` followed by `b` (indices 0..8).
    #[inline(always)]
    pub(crate) fn shuffle<const I0: usize, const I1: usize, const I2: usize, const I3: usize>(
        a: &Self,
        b: &Self,
    ) -> Self {
        Self(
            core::array::from_fn(|j| {
                core::arch::wasm32::u32x4_shuffle::<I0, I1, I2, I3>(a.0[j], b.0[j])
            }),
            PhantomData,
        )
    }
}

/// One Fq element as the MSM's bucket points store it: 29-bit limbs in
/// WebAssembly with SIMD, so that four points load into lane form with a
/// lane load per limb instead of a word split per element; the element
/// itself elsewhere. Reduced, as every element here.
#[derive(Clone, Copy, Default, PartialEq, Eq)]
pub(crate) struct FqLimbs(
    #[cfg(all(target_arch = "wasm32", target_feature = "simd128"))] [u32; 9],
    #[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))] Fq,
);

#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
impl FqLimbs {
    #[inline(always)]
    pub(crate) fn new(value: Fq) -> Self {
        Self(value)
    }
    #[inline(always)]
    pub(crate) fn get(self) -> Fq {
        self.0
    }
    #[inline(always)]
    pub(crate) fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}
#[cfg(not(all(target_arch = "wasm32", target_feature = "simd128")))]
impl FqLanes {
    #[inline(always)]
    pub(crate) fn load(a: [&FqLimbs; 4]) -> Self {
        Self::new(&a.map(|limbs| limbs.0))
    }
    #[inline(always)]
    pub(crate) fn store(&self) -> [FqLimbs; 4] {
        self.get().map(FqLimbs)
    }
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
impl FqLimbs {
    #[inline(always)]
    pub(crate) fn new(value: Fq) -> Self {
        Self(split(&value.0))
    }
    #[inline(always)]
    pub(crate) fn get(self) -> Fq {
        Fq::from_montgomery(word(self.0.map(|limb| limb as u64)))
    }
    #[inline(always)]
    pub(crate) fn is_zero(&self) -> bool {
        self.0 == [0; 9]
    }
}
#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
impl FqLanes {
    /// Four limb-form elements, a lane load per limb.
    #[inline(always)]
    pub(crate) fn load(a: [&FqLimbs; 4]) -> Self {
        use core::arch::wasm32::*;
        let mut out = [u32x4_splat(0); 9];
        for j in 0..9 {
            // Nine limbs each, 4-byte aligned: every address is in bounds.
            out[j] = unsafe {
                let v = v128_load32_lane::<0>(out[j], a[0].0.as_ptr().add(j));
                let v = v128_load32_lane::<1>(v, a[1].0.as_ptr().add(j));
                let v = v128_load32_lane::<2>(v, a[2].0.as_ptr().add(j));
                v128_load32_lane::<3>(v, a[3].0.as_ptr().add(j))
            };
        }
        Self(out, PhantomData)
    }
    #[inline(always)]
    pub(crate) fn store(&self) -> [FqLimbs; 4] {
        use core::arch::wasm32::*;
        let mut out = [FqLimbs([0; 9]); 4];
        for j in 0..9 {
            unsafe {
                v128_store32_lane::<0>(self.0[j], out[0].0.as_mut_ptr().add(j));
                v128_store32_lane::<1>(self.0[j], out[1].0.as_mut_ptr().add(j));
                v128_store32_lane::<2>(self.0[j], out[2].0.as_mut_ptr().add(j));
                v128_store32_lane::<3>(self.0[j], out[3].0.as_mut_ptr().add(j));
            }
        }
        out
    }
}

// The 29-bit limbs of mont29.
#[allow(dead_code)]
#[inline(always)]
fn split(a: &[u64; 4]) -> [u32; 9] {
    const MASK: u64 = (1 << 29) - 1;
    [
        a[0] & MASK,
        ((a[0] >> 29) | (a[1] << 35)) & MASK,
        ((a[0] >> 58) | (a[1] << 6)) & MASK,
        ((a[1] >> 23) | (a[2] << 41)) & MASK,
        ((a[1] >> 52) | (a[2] << 12)) & MASK,
        ((a[2] >> 17) | (a[3] << 47)) & MASK,
        ((a[2] >> 46) | (a[3] << 18)) & MASK,
        (a[3] >> 11) & MASK,
        (a[3] >> 40) & MASK,
    ]
    .map(|limb| limb as u32)
}

// The words of nine reduced 29-bit limbs.
#[allow(dead_code)]
#[inline(always)]
fn word(t: [u64; 9]) -> [u64; 4] {
    [
        t[0] | (t[1] << 29) | (t[2] << 58),
        (t[2] >> 6) | (t[3] << 23) | (t[4] << 52),
        (t[4] >> 12) | (t[5] << 17) | (t[6] << 46),
        (t[6] >> 18) | (t[7] << 11) | (t[8] << 40),
    ]
}

// mont29's final shift by 2^24 and conditional subtraction, in words.
#[allow(dead_code)]
#[inline(always)]
fn join(t: [u64; 10], p: [u64; 4]) -> [u64; 4] {
    let out = [
        (t[0] >> 24) | (t[1] << 5) | (t[2] << 34) | (t[3] << 63),
        (t[3] >> 1) | (t[4] << 28) | (t[5] << 57),
        (t[5] >> 7) | (t[6] << 22) | (t[7] << 51),
        (t[7] >> 13) | (t[8] << 16) | (t[9] << 45),
    ];
    let (d, borrow) = sub4(out, p);
    if borrow {
        out
    } else {
        d
    }
}

#[allow(dead_code)]
#[inline(always)]
fn sub4(a: [u64; 4], b: [u64; 4]) -> ([u64; 4], bool) {
    let mut r = [0u64; 4];
    let mut borrow = false;
    for i in 0..4 {
        let (d, b1) = a[i].overflowing_sub(b[i]);
        let (d, b2) = d.overflowing_sub(borrow as u64);
        r[i] = d;
        borrow = b1 | b2;
    }
    (r, borrow)
}

#[cfg(all(target_arch = "wasm32", target_feature = "simd128"))]
mod lanes {
    use super::{split, Mont29};
    use core::arch::wasm32::*;

    core::arch::global_asm!(include_str!("mont29_simd.s"));
    // Arguments point at [v128; 9] (x, y) and [v128; 10] (out).
    extern "C" {
        pub(super) fn mont29_mul4_fq(x: *const u32, y: *const u32, out: *mut u32);
        pub(super) fn mont29_mul4_fr(x: *const u32, y: *const u32, out: *mut u32);
    }

    const MASK: u32 = (1 << 29) - 1;

    #[inline(always)]
    pub(super) fn limbs(a: &[[u64; 4]; 4]) -> [v128; 9] {
        let l = [split(&a[0]), split(&a[1]), split(&a[2]), split(&a[3])];
        core::array::from_fn(|j| u32x4(l[0][j], l[1][j], l[2][j], l[3][j]))
    }

    // The words of four reduced lane-form elements.
    #[inline(always)]
    pub(super) fn words(l: &[v128; 9]) -> [[u64; 4]; 4] {
        let lane = |i: usize| -> [u64; 9] {
            core::array::from_fn(|j| {
                (match i {
                    0 => u32x4_extract_lane::<0>(l[j]),
                    1 => u32x4_extract_lane::<1>(l[j]),
                    2 => u32x4_extract_lane::<2>(l[j]),
                    _ => u32x4_extract_lane::<3>(l[j]),
                }) as u64
            })
        };
        [
            super::word(lane(0)),
            super::word(lane(1)),
            super::word(lane(2)),
            super::word(lane(3)),
        ]
    }

    #[inline(always)]
    pub(super) fn modulus<F: Mont29>() -> [v128; 9] {
        let p = split(&F::modulus());
        core::array::from_fn(|j| u32x4_splat(p[j]))
    }

    // Limbs below 2^29 again, carrying upward; the top limb keeps the rest.
    #[inline(always)]
    fn carry(mut v: [v128; 9]) -> [v128; 9] {
        let mask = u32x4_splat(MASK);
        for j in 0..8 {
            v[j + 1] = i32x4_add(v[j + 1], i32x4_shr(v[j], 29));
            v[j] = v128_and(v[j], mask);
        }
        v
    }

    // v - p where v >= p, else v, per lane; v < 2p.
    #[inline(always)]
    fn reduce(v: [v128; 9], p: &[v128; 9]) -> [v128; 9] {
        let mut d = [u32x4_splat(0); 9];
        for j in 0..9 {
            d[j] = i32x4_sub(v[j], p[j]);
        }
        let d = carry(d);
        // A negative top limb means v < p.
        let below = i32x4_shr(d[8], 31);
        core::array::from_fn(|j| v128_bitselect(v[j], d[j], below))
    }

    #[inline(always)]
    pub(super) fn add(a: &[v128; 9], b: &[v128; 9], p: &[v128; 9]) -> [v128; 9] {
        reduce(carry(core::array::from_fn(|j| i32x4_add(a[j], b[j]))), p)
    }

    #[inline(always)]
    pub(super) fn sub(a: &[v128; 9], b: &[v128; 9], p: &[v128; 9]) -> [v128; 9] {
        let d = carry(core::array::from_fn(|j| i32x4_sub(a[j], b[j])));
        // Where a < b the top limb is negative: add p back.
        let below = i32x4_shr(d[8], 31);
        carry(core::array::from_fn(|j| {
            i32x4_add(d[j], v128_and(p[j], below))
        }))
    }

    // The core's output t (limbs of 2^24 times the unreduced product, below
    // 2p) as reduced lane form: shift by 24 bits, subtract p once.
    #[inline(always)]
    pub(super) fn product(
        core: unsafe extern "C" fn(*const u32, *const u32, *mut u32),
        x: &[v128; 9],
        y: &[v128; 9],
        p: &[v128; 9],
    ) -> [v128; 9] {
        let mut t = [u32x4_splat(0); 10];
        unsafe { core(x.as_ptr().cast(), y.as_ptr().cast(), t.as_mut_ptr().cast()) };
        let mask = u32x4_splat(MASK);
        let v = core::array::from_fn(|j| {
            v128_and(v128_or(u32x4_shr(t[j], 24), i32x4_shl(t[j + 1], 5)), mask)
        });
        reduce(v, p)
    }
}

/// Native IFMA paths can be disabled individually while diagnosing a CPU.
/// Other targets keep their existing defaults.
pub(crate) fn cpu_option(name: &str) -> bool {
    std::env::var(name).map(|v| v != "0").unwrap_or(cfg!(all(
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))
}

#[cfg(all(
    test,
    feature = "ifma8",
    target_arch = "x86_64",
    target_feature = "avx512ifma",
    target_feature = "avx512vl",
    target_feature = "avx512dq",
    not(no_ifma)
))]
#[test]
fn packed_inverses_match_scalar_inverses() {
    for seed in 1..33u64 {
        let values = core::array::from_fn(|i| match i {
            0 => Fq::ONE,
            1 => -Fq::ONE,
            _ => Fq::from(seed + i as u64).pow(&[127]),
        });
        let inverses = FqBatch::new(&values).inverse_nonzero().get();
        for i in 0..8 {
            assert_eq!(inverses[i], values[i].inverse().unwrap());
        }
    }
}
