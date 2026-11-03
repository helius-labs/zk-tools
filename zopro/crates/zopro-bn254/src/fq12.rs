//! Fq12 = Fq6[w] / (w^2 - v), the pairing's target field.
use core::ops::{Add, Mul, Neg, Sub};

use crate::{Fq, Fq2, Fq6};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Debug)]
pub struct Fq12 {
    pub c0: Fq6,
    pub c1: Fq6,
}

const fn fq2(c0: &str, c1: &str) -> Fq2 {
    Fq2::new(Fq::from_decimal(c0), Fq::from_decimal(c1))
}

/// (9 + u)^((p^i - 1) / 6).
const FROBENIUS_C1: [Fq2; 12] = [
    fq2("1", "0"),
    fq2(
        "8376118865763821496583973867626364092589906065868298776909617916018768340080",
        "16469823323077808223889137241176536799009286646108169935659301613961712198316",
    ),
    fq2(
        "21888242871839275220042445260109153167277707414472061641714758635765020556617",
        "0",
    ),
    fq2(
        "11697423496358154304825782922584725312912383441159505038794027105778954184319",
        "303847389135065887422783454877609941456349188919719272345083954437860409601",
    ),
    fq2(
        "21888242871839275220042445260109153167277707414472061641714758635765020556616",
        "0",
    ),
    fq2(
        "3321304630594332808241809054958361220322477375291206261884409189760185844239",
        "5722266937896532885780051958958348231143373700109372999374820235121374419868",
    ),
    fq2(
        "21888242871839275222246405745257275088696311157297823662689037894645226208582",
        "0",
    ),
    fq2(
        "13512124006075453725662431877630910996106405091429524885779419978626457868503",
        "5418419548761466998357268504080738289687024511189653727029736280683514010267",
    ),
    fq2("2203960485148121921418603742825762020974279258880205651966", "0"),
    fq2(
        "10190819375481120917420622822672549775783927716138318623895010788866272024264",
        "21584395482704209334823622290379665147239961968378104390343953940207365798982",
    ),
    fq2("2203960485148121921418603742825762020974279258880205651967", "0"),
    fq2(
        "18566938241244942414004596690298913868373833782006617400804628704885040364344",
        "16165975933942742336466353786298926857552937457188450663314217659523851788715",
    ),
];

impl Fq12 {
    pub const ONE: Self = Self::new(Fq6::ONE, Fq6::ZERO);

    pub const fn new(c0: Fq6, c1: Fq6) -> Self {
        Self { c0, c1 }
    }

    pub fn square(&self) -> Self {
        let product = self.c0 * self.c1;
        let c0 = (self.c0 + self.c1) * (self.c0 + self.c1.mul_by_nonresidue())
            - product
            - product.mul_by_nonresidue();
        Self::new(c0, product + product)
    }

    pub fn conjugate(&self) -> Self {
        Self::new(self.c0, -self.c1)
    }

    pub fn inverse(&self) -> Option<Self> {
        let t = self.c0.square() - self.c1.square().mul_by_nonresidue();
        t.inverse().map(|t| Self::new(self.c0 * t, -(self.c1 * t)))
    }

    pub fn frobenius(&self, power: usize) -> Self {
        Self::new(
            self.c0.frobenius(power),
            self.c1.frobenius(power).mul_by_fp2(&FROBENIUS_C1[power % 12]),
        )
    }

    /// Times c0 + c3 w + c4 v w: a D-type twist's line.
    pub fn mul_by_034(&self, c0: &Fq2, c3: &Fq2, c4: &Fq2) -> Self {
        let a = self.c0.mul_by_fp2(c0);
        let b = self.c1.mul_by_01(c3, c4);
        let e = (self.c0 + self.c1).mul_by_01(&(*c0 + *c3), c4);
        Self::new(b.mul_by_nonresidue() + a, e - (a + b))
    }

    /// self^e for an exponent of one limb.
    pub fn pow_u64(&self, e: u64) -> Self {
        let mut result = Self::ONE;
        for bit in (0..64).rev() {
            result = result.square();
            if (e >> bit) & 1 == 1 {
                result = result * *self;
            }
        }
        result
    }
}

impl Add for Fq12 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(self.c0 + o.c0, self.c1 + o.c1)
    }
}
impl Sub for Fq12 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self::new(self.c0 - o.c0, self.c1 - o.c1)
    }
}
impl Neg for Fq12 {
    type Output = Self;
    fn neg(self) -> Self {
        Self::new(-self.c0, -self.c1)
    }
}
impl Mul for Fq12 {
    type Output = Self;
    fn mul(self, o: Self) -> Self {
        let v0 = self.c0 * o.c0;
        let v1 = self.c1 * o.c1;
        Self::new(
            v0 + v1.mul_by_nonresidue(),
            (self.c0 + self.c1) * (o.c0 + o.c1) - v0 - v1,
        )
    }
}
crate::assign_ops!(Fq12);
