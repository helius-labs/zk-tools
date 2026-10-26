//! Fq6 = Fq2[v] / (v^3 - (9 + u)), for the pairing.
use core::ops::{Add, Mul, Neg, Sub};

use crate::{Fq, Fq2};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct Fq6 {
    pub c0: Fq2,
    pub c1: Fq2,
    pub c2: Fq2,
}

const fn fq2(c0: &str, c1: &str) -> Fq2 {
    Fq2::new(Fq::from_decimal(c0), Fq::from_decimal(c1))
}

/// (9 + u)^((p^i - 1) / 3) and its square.
const FROBENIUS_C1: [Fq2; 6] = [
    fq2("1", "0"),
    fq2(
        "21575463638280843010398324269430826099269044274347216827212613867836435027261",
        "10307601595873709700152284273816112264069230130616436755625194854815875713954",
    ),
    fq2(
        "21888242871839275220042445260109153167277707414472061641714758635765020556616",
        "0",
    ),
    fq2(
        "3772000881919853776433695186713858239009073593817195771773381919316419345261",
        "2236595495967245188281701248203181795121068902605861227855261137820944008926",
    ),
    fq2("2203960485148121921418603742825762020974279258880205651966", "0"),
    fq2(
        "18429021223477853657660792034369865839114504446431234726392080002137598044644",
        "9344045779998320333812420223237981029506012124075525679208581902008406485703",
    ),
];
const FROBENIUS_C2: [Fq2; 6] = [
    fq2("1", "0"),
    fq2(
        "2581911344467009335267311115468803099551665605076196740867805258568234346338",
        "19937756971775647987995932169929341994314640652964949448313374472400716661030",
    ),
    fq2("2203960485148121921418603742825762020974279258880205651966", "0"),
    fq2(
        "5324479202449903542726783395506214481928257762400643279780343368557297135718",
        "16208900380737693084919495127334387981393726419856888799917914180988844123039",
    ),
    fq2(
        "21888242871839275220042445260109153167277707414472061641714758635765020556616",
        "0",
    ),
    fq2(
        "13981852324922362344252311234282257507216387789820983642040889267519694726527",
        "7629828391165209371577384193250820201684255241773809077146787135900891633097",
    ),
];

impl Fq6 {
    pub const ZERO: Self = Self::new(Fq2::ZERO, Fq2::ZERO, Fq2::ZERO);
    pub const ONE: Self = Self::new(Fq2::ONE, Fq2::ZERO, Fq2::ZERO);

    pub const fn new(c0: Fq2, c1: Fq2, c2: Fq2) -> Self {
        Self { c0, c1, c2 }
    }

    pub fn is_zero(&self) -> bool {
        self.c0.is_zero() && self.c1.is_zero() && self.c2.is_zero()
    }

    /// Times v: (c0, c1, c2) v = (xi c2, c0, c1).
    pub fn mul_by_nonresidue(&self) -> Self {
        Self::new(self.c2.mul_by_nonresidue(), self.c0, self.c1)
    }

    pub fn mul_by_fp2(&self, x: &Fq2) -> Self {
        Self::new(self.c0 * *x, self.c1 * *x, self.c2 * *x)
    }

    /// CH-SQR2 (Chung and Hasan).
    pub fn square(&self) -> Self {
        let s0 = self.c0.square();
        let s1 = (self.c0 * self.c1).double();
        let s2 = (self.c0 - self.c1 + self.c2).square();
        let s3 = (self.c1 * self.c2).double();
        let s4 = self.c2.square();
        Self::new(
            s0 + s3.mul_by_nonresidue(),
            s1 + s4.mul_by_nonresidue(),
            s1 + s2 + s3 - s0 - s4,
        )
    }

    pub fn inverse(&self) -> Option<Self> {
        let t0 = self.c0.square() - (self.c1 * self.c2).mul_by_nonresidue();
        let t1 = self.c2.square().mul_by_nonresidue() - self.c0 * self.c1;
        let t2 = self.c1.square() - self.c0 * self.c2;
        let norm = self.c0 * t0 + (self.c2 * t1 + self.c1 * t2).mul_by_nonresidue();
        norm.inverse().map(|n| Self::new(t0 * n, t1 * n, t2 * n))
    }

    pub fn frobenius(&self, power: usize) -> Self {
        Self::new(
            self.c0.frobenius(power),
            self.c1.frobenius(power) * FROBENIUS_C1[power % 6],
            self.c2.frobenius(power) * FROBENIUS_C2[power % 6],
        )
    }

    /// Times b0 + b1 v.
    pub fn mul_by_01(&self, b0: &Fq2, b1: &Fq2) -> Self {
        let a_a = self.c0 * *b0;
        let b_b = self.c1 * *b1;
        let t1 = (*b1 * (self.c1 + self.c2) - b_b).mul_by_nonresidue() + a_a;
        let t3 = *b0 * (self.c0 + self.c2) - a_a + b_b;
        let t2 = (*b0 + *b1) * (self.c0 + self.c1) - a_a - b_b;
        Self::new(t1, t2, t3)
    }

    /// Times b1 v.
    pub fn mul_by_1(&self, b1: &Fq2) -> Self {
        let b_b = self.c1 * *b1;
        let t1 = (*b1 * (self.c1 + self.c2) - b_b).mul_by_nonresidue();
        let t2 = *b1 * (self.c0 + self.c1) - b_b;
        Self::new(t1, t2, b_b)
    }
}

impl Add for Fq6 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(self.c0 + o.c0, self.c1 + o.c1, self.c2 + o.c2)
    }
}
impl Sub for Fq6 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self::new(self.c0 - o.c0, self.c1 - o.c1, self.c2 - o.c2)
    }
}
impl Neg for Fq6 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.c0, -self.c1, -self.c2)
    }
}
impl Mul for Fq6 {
    type Output = Self;
    /// Karatsuba over the cubic extension (Devegili et al.).
    fn mul(self, o: Self) -> Self {
        let v0 = self.c0 * o.c0;
        let v1 = self.c1 * o.c1;
        let v2 = self.c2 * o.c2;
        let c0 = ((self.c1 + self.c2) * (o.c1 + o.c2) - v1 - v2).mul_by_nonresidue() + v0;
        let c1 = (self.c0 + self.c1) * (o.c0 + o.c1) - v0 - v1 + v2.mul_by_nonresidue();
        let c2 = (self.c0 + self.c2) * (o.c0 + o.c2) - v0 + v1 - v2;
        Self::new(c0, c1, c2)
    }
}
crate::assign_ops!(Fq6);
