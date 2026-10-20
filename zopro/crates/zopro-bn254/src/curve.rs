//! G1 (over Fq) and G2 (over Fq2) of y^2 = x^3 + b: affine points, the point
//! at infinity (0, 0), and Jacobian points (X / Z^2, Y / Z^3), infinity
//! Z = 0.
use core::{
    fmt,
    marker::PhantomData,
    ops::{Add, AddAssign, Neg, Sub, SubAssign},
};

use crate::{Field, Fq, Fq2, Fr};

pub trait CurveConfig: 'static + Copy + Send + Sync + Eq + fmt::Debug + Default {
    type Base: Field;
    const B: Self::Base;
    const GENERATOR_X: Self::Base;
    const GENERATOR_Y: Self::Base;
}

#[derive(Clone, Copy, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Affine<C: CurveConfig> {
    pub x: C::Base,
    pub y: C::Base,
    _curve: PhantomData<C>,
}

#[derive(Clone, Copy, Default)]
pub struct Projective<C: CurveConfig> {
    pub x: C::Base,
    pub y: C::Base,
    pub z: C::Base,
    _curve: PhantomData<C>,
}

impl<C: CurveConfig> fmt::Debug for Affine<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_identity() {
            f.write_str("infinity")
        } else {
            write!(f, "({:?}, {:?})", self.x, self.y)
        }
    }
}

impl<C: CurveConfig> fmt::Debug for Projective<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.to_affine().fmt(f)
    }
}

impl<C: CurveConfig> Affine<C> {
    pub const IDENTITY: Self = Self::new(C::Base::ZERO, C::Base::ZERO);
    pub const GENERATOR: Self = Self::new(C::GENERATOR_X, C::GENERATOR_Y);

    #[inline(always)]
    pub const fn new(x: C::Base, y: C::Base) -> Self {
        Self { x, y, _curve: PhantomData }
    }

    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    /// On the curve, or the point at infinity.
    pub fn is_on_curve(&self) -> bool {
        self.is_identity() || self.y.square() == self.x.square() * self.x + C::B
    }

    #[inline(always)]
    pub fn to_projective(&self) -> Projective<C> {
        if self.is_identity() {
            Projective::IDENTITY
        } else {
            Projective::new(self.x, self.y, C::Base::ONE)
        }
    }

    pub fn mul_limbs(&self, scalar: &[u64]) -> Projective<C> {
        self.to_projective().mul_limbs(scalar)
    }

    pub fn mul(&self, scalar: &Fr) -> Projective<C> {
        self.to_projective().mul_limbs(&scalar.to_canonical())
    }
}

impl<C: CurveConfig> Neg for Affine<C> {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        Self::new(self.x, -self.y)
    }
}

impl<C: CurveConfig> Projective<C> {
    pub const IDENTITY: Self = Self::new(C::Base::ONE, C::Base::ONE, C::Base::ZERO);

    #[inline(always)]
    pub const fn new(x: C::Base, y: C::Base, z: C::Base) -> Self {
        Self { x, y, z, _curve: PhantomData }
    }

    pub fn generator() -> Self {
        Affine::<C>::GENERATOR.to_projective()
    }

    #[inline(always)]
    pub fn is_identity(&self) -> bool {
        self.z.is_zero()
    }

    /// dbl-2009-l.
    pub fn double(&self) -> Self {
        if self.is_identity() {
            return *self;
        }
        let a = self.x.square();
        let b = self.y.square();
        let c = b.square();
        let d = ((self.x + b).square() - a - c).double();
        let e = a.double() + a;
        let f = e.square();
        let x = f - d.double();
        let y = e * (d - x) - c.double().double().double();
        let z = (self.y * self.z).double();
        Self::new(x, y, z)
    }

    /// add-2007-bl.
    pub fn add_projective(&self, other: &Self) -> Self {
        if self.is_identity() {
            return *other;
        }
        if other.is_identity() {
            return *self;
        }
        let z1z1 = self.z.square();
        let z2z2 = other.z.square();
        let u1 = self.x * z2z2;
        let u2 = other.x * z1z1;
        let s1 = self.y * other.z * z2z2;
        let s2 = other.y * self.z * z1z1;
        if u1 == u2 {
            return if s1 == s2 { self.double() } else { Self::IDENTITY };
        }
        let h = u2 - u1;
        let i = h.double().square();
        let j = h * i;
        let r = (s2 - s1).double();
        let v = u1 * i;
        let x = r.square() - j - v.double();
        let y = r * (v - x) - (s1 * j).double();
        let z = ((self.z + other.z).square() - z1z1 - z2z2) * h;
        Self::new(x, y, z)
    }

    /// madd-2007-bl: plus an affine point.
    pub fn add_affine(&self, other: &Affine<C>) -> Self {
        if other.is_identity() {
            return *self;
        }
        if self.is_identity() {
            return other.to_projective();
        }
        let z1z1 = self.z.square();
        let u2 = other.x * z1z1;
        let s2 = other.y * self.z * z1z1;
        if self.x == u2 {
            return if self.y == s2 { self.double() } else { Self::IDENTITY };
        }
        let h = u2 - self.x;
        let hh = h.square();
        let i = hh.double().double();
        let j = h * i;
        let r = (s2 - self.y).double();
        let v = self.x * i;
        let x = r.square() - j - v.double();
        let y = r * (v - x) - (self.y * j).double();
        let z = (self.z + h).square() - z1z1 - hh;
        Self::new(x, y, z)
    }

    pub fn to_affine(&self) -> Affine<C> {
        match self.z.inverse() {
            None => Affine::IDENTITY,
            Some(zi) => {
                let zi2 = zi.square();
                Affine::new(self.x * zi2, self.y * zi2 * zi)
            }
        }
    }

    /// Many points to affine with one inversion.
    pub fn batch_to_affine(points: &[Self]) -> Vec<Affine<C>> {
        let mut products = Vec::with_capacity(points.len());
        let mut acc = C::Base::ONE;
        for p in points {
            products.push(acc);
            if !p.is_identity() {
                acc *= p.z;
            }
        }
        let mut inverse = acc.inverse().expect("a nonzero product");
        let mut out = vec![Affine::IDENTITY; points.len()];
        for (i, p) in points.iter().enumerate().rev() {
            if p.is_identity() {
                continue;
            }
            let zi = inverse * products[i];
            inverse *= p.z;
            let zi2 = zi.square();
            out[i] = Affine::new(p.x * zi2, p.y * zi2 * zi);
        }
        out
    }

    /// self * scalar, scalar little-endian limbs: double and add.
    pub fn mul_limbs(&self, scalar: &[u64]) -> Self {
        let mut result = Self::IDENTITY;
        for limb in scalar.iter().rev() {
            for bit in (0..64).rev() {
                result = result.double();
                if (limb >> bit) & 1 == 1 {
                    result = result.add_projective(self);
                }
            }
        }
        result
    }

    pub fn mul(&self, scalar: &Fr) -> Self {
        self.mul_limbs(&scalar.to_canonical())
    }
}

impl<C: CurveConfig> PartialEq for Projective<C> {
    fn eq(&self, other: &Self) -> bool {
        match (self.is_identity(), other.is_identity()) {
            (true, true) => true,
            (false, false) => {
                let z1z1 = self.z.square();
                let z2z2 = other.z.square();
                self.x * z2z2 == other.x * z1z1
                    && self.y * z2z2 * other.z == other.y * z1z1 * self.z
            }
            _ => false,
        }
    }
}
impl<C: CurveConfig> Eq for Projective<C> {}

impl<C: CurveConfig> Add for Projective<C> {
    type Output = Self;
    #[inline]
    fn add(self, other: Self) -> Self {
        self.add_projective(&other)
    }
}
impl<C: CurveConfig> Add<Affine<C>> for Projective<C> {
    type Output = Self;
    #[inline]
    fn add(self, other: Affine<C>) -> Self {
        self.add_affine(&other)
    }
}
impl<C: CurveConfig> AddAssign for Projective<C> {
    #[inline]
    fn add_assign(&mut self, other: Self) {
        *self = self.add_projective(&other);
    }
}
impl<C: CurveConfig> AddAssign<Affine<C>> for Projective<C> {
    #[inline]
    fn add_assign(&mut self, other: Affine<C>) {
        *self = self.add_affine(&other);
    }
}
impl<C: CurveConfig> Neg for Projective<C> {
    type Output = Self;
    #[inline]
    fn neg(self) -> Self {
        Self::new(self.x, -self.y, self.z)
    }
}
impl<C: CurveConfig> Sub for Projective<C> {
    type Output = Self;
    #[inline]
    fn sub(self, other: Self) -> Self {
        self.add_projective(&-other)
    }
}
impl<C: CurveConfig> SubAssign for Projective<C> {
    #[inline]
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl<C: CurveConfig> SubAssign<Affine<C>> for Projective<C> {
    #[inline]
    fn sub_assign(&mut self, other: Affine<C>) {
        *self = self.add_affine(&-other);
    }
}
impl<C: CurveConfig> core::iter::Sum for Projective<C> {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Self::IDENTITY, |a, b| a + b)
    }
}
impl<'a, C: CurveConfig> core::iter::Sum<&'a Self> for Projective<C> {
    fn sum<I: Iterator<Item = &'a Self>>(iter: I) -> Self {
        iter.fold(Self::IDENTITY, |a, b| a + *b)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct G1Config;
impl CurveConfig for G1Config {
    type Base = Fq;
    const B: Fq = Fq::from_decimal("3");
    const GENERATOR_X: Fq = Fq::from_decimal("1");
    const GENERATOR_Y: Fq = Fq::from_decimal("2");
}
pub type G1Affine = Affine<G1Config>;
pub type G1Projective = Projective<G1Config>;

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct G2Config;
const fn fq2(c0: &str, c1: &str) -> Fq2 {
    Fq2::new(Fq::from_decimal(c0), Fq::from_decimal(c1))
}
impl CurveConfig for G2Config {
    type Base = Fq2;
    /// 3 / (9 + u).
    const B: Fq2 = fq2(
        "19485874751759354771024239261021720505790618469301721065564631296452457478373",
        "266929791119991161246907387137283842545076965332900288569378510910307636690",
    );
    const GENERATOR_X: Fq2 = fq2(
        "10857046999023057135944570762232829481370756359578518086990519993285655852781",
        "11559732032986387107991004021392285783925812861821192530917403151452391805634",
    );
    const GENERATOR_Y: Fq2 = fq2(
        "8495653923123431417604973247489272438418190587263600148770280649306958101930",
        "4082367875863433681332203403145435568316851327593401208105741076214120093531",
    );
}
pub type G2Affine = Affine<G2Config>;
pub type G2Projective = Projective<G2Config>;

/// The Frobenius twist's coefficients: psi(x, y) = (conj(x) c_x, conj(y) c_y).
pub(crate) const PSI_X: Fq2 = fq2(
    "21575463638280843010398324269430826099269044274347216827212613867836435027261",
    "10307601595873709700152284273816112264069230130616436755625194854815875713954",
);
pub(crate) const PSI_Y: Fq2 = fq2(
    "2821565182194536844548159561693502659359617185244120367078079554186484126554",
    "3505843767911556378687030309984248845540243509899259641013678093033130930403",
);

impl G2Affine {
    pub(crate) fn psi(&self) -> Self {
        Self::new(self.x.conjugate() * PSI_X, self.y.conjugate() * PSI_Y)
    }

    /// In the prime-order subgroup, for a point on the curve: psi(P) =
    /// [6 x^2] P (El Housni, Guillevic and Piellard), as arkworks checks.
    pub fn is_in_subgroup(&self) -> bool {
        const SIX_X_SQUARED: [u64; 2] = [17887900258952609094, 8020209761171036667];
        if self.is_identity() {
            return true;
        }
        self.mul_limbs(&SIX_X_SQUARED) == self.psi().to_projective()
    }
}

impl<C: CurveConfig> core::ops::Mul<Fr> for Projective<C> {
    type Output = Self;
    fn mul(self, scalar: Fr) -> Self {
        Projective::mul(&self, &scalar)
    }
}
impl<C: CurveConfig> core::ops::Mul<Fr> for Affine<C> {
    type Output = Projective<C>;
    fn mul(self, scalar: Fr) -> Projective<C> {
        Affine::mul(&self, &scalar)
    }
}
impl<C: CurveConfig> Add for Affine<C> {
    type Output = Projective<C>;
    fn add(self, other: Self) -> Projective<C> {
        self.to_projective().add_affine(&other)
    }
}
impl<'a, C: CurveConfig> AddAssign<&'a Projective<C>> for Projective<C> {
    #[inline]
    fn add_assign(&mut self, other: &'a Projective<C>) {
        *self = self.add_projective(other);
    }
}
impl<'a, C: CurveConfig> AddAssign<&'a Affine<C>> for Projective<C> {
    #[inline]
    fn add_assign(&mut self, other: &'a Affine<C>) {
        *self = self.add_affine(other);
    }
}
impl<'a, C: CurveConfig> SubAssign<&'a Affine<C>> for Projective<C> {
    #[inline]
    fn sub_assign(&mut self, other: &'a Affine<C>) {
        *self = self.add_affine(&-*other);
    }
}

/// The sum of scalars[i] bases[i] by unsigned 4-bit windows into
/// projective buckets: simple, for small inputs and as tests' reference.
pub fn msm<C: CurveConfig>(bases: &[Affine<C>], scalars: &[Fr]) -> Projective<C> {
    assert_eq!(bases.len(), scalars.len(), "as many scalars as bases");
    let scalars: Vec<[u64; 4]> = scalars.iter().map(|s| s.to_canonical()).collect();
    let mut result = Projective::<C>::IDENTITY;
    for window in (0..64).rev() {
        for _ in 0..4 {
            result = result.double();
        }
        let mut buckets = [Projective::<C>::IDENTITY; 15];
        for (base, s) in bases.iter().zip(&scalars) {
            let digit = (s[window / 16] >> (4 * (window % 16))) & 15;
            if digit != 0 {
                buckets[digit as usize - 1] += *base;
            }
        }
        let (mut running, mut sum) = (Projective::<C>::IDENTITY, Projective::<C>::IDENTITY);
        for bucket in buckets.iter().rev() {
            running += *bucket;
            sum += running;
        }
        result += sum;
    }
    result
}
