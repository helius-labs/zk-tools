//! Lane-wise accepted final-exponentiation schedule. The inversion bridge
//! extracts only four base-field norms; extension state remains packed.
use super::{frob12, frob6, C, Half, Q, T};
use zopro_bn254::{Fq2, Fq12, G1Affine};

impl Q {
    #[inline(always)]
    fn constant(value: Fq2) -> Self { Self::new([value; 4]) }

    #[inline(always)]
    fn conjugate(self) -> Self {
        let (a, b) = self.0.halves();
        Self::join(a, Half::zero().sub(&b))
    }

    #[inline(always)]
    fn frobenius(self, power: usize) -> Self {
        if power % 2 == 0 { self } else { self.conjugate() }
    }

    #[inline(never)]
    fn inverse(self) -> Option<Self> {
        let (aa, bb) = self.0.mul(&self.0).halves();
        let norms = aa.add(&bb).get();
        let mut inverses = norms;
        for value in &mut inverses { *value = value.inverse()?; }
        let factor = Half::new(&inverses);
        Some(self.conjugate().scale(factor))
    }
}

impl std::ops::Neg for Q {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self { Q::zero() - self }
}

impl C {
    #[inline(always)]
    fn square(self) -> Self {
        let [a, b, c] = self.0;
        let s0 = a.square();
        let s1 = a.mul(b).double();
        let s2 = (a - b + c).square();
        let s3 = b.mul(c).double();
        let s4 = c.square();
        Self([s0 + s3.nr(), s1 + s4.nr(), s1 + s2 + s3 - s0 - s4])
    }

    #[inline(never)]
    fn inverse(self) -> Option<Self> {
        let [a, b, c] = self.0;
        let t0 = a.square() - b.mul(c).nr();
        let t1 = c.square().nr() - a.mul(b);
        let t2 = b.square() - a.mul(c);
        let norm = a.mul(t0) + (c.mul(t1) + b.mul(t2)).nr();
        let n = norm.inverse()?;
        Some(Self([t0.mul(n), t1.mul(n), t2.mul(n)]))
    }

    #[inline(always)]
    fn frobenius(self, power: usize) -> Self {
        let [a, b, c] = self.0;
        Self([
            a.frobenius(power),
            b.frobenius(power).mul(Q::constant(frob6::FROBENIUS_C1[power % 6])),
            c.frobenius(power).mul(Q::constant(frob6::FROBENIUS_C2[power % 6])),
        ])
    }
}

impl std::ops::Neg for C {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self { Self(self.0.map(std::ops::Neg::neg)) }
}

impl T {
    pub fn conjugate(self) -> Self { Self([self.0[0], -self.0[1]]) }

    #[inline(never)]
    pub fn inverse(self) -> Option<Self> {
        let [a, b] = self.0;
        let n = (a.square() - b.square().nr()).inverse()?;
        Some(Self([a.mul(n), -b.mul(n)]))
    }

    #[inline(never)]
    pub fn frobenius(self, power: usize) -> Self {
        Self([
            self.0[0].frobenius(power),
            self.0[1].frobenius(power).mul_fp2(Q::constant(frob12::FROBENIUS_C1[power % 12])),
        ])
    }

    /// Only valid after the easy part has entered the cyclotomic subgroup.
    #[inline(never)]
    pub fn cyclotomic_square(self) -> Self {
        fn fp4(a: Q, b: Q) -> (Q, Q) {
            (a.square() + b.square().nr(), a.mul(b).double())
        }
        fn minus(t: Q, old: Q) -> Q { (t - old).double() + t }
        fn plus(t: Q, old: Q) -> Q { (t + old).double() + t }
        let (a0, a1) = fp4(self.0[0].0[0], self.0[1].0[1]);
        let (b0, b1) = fp4(self.0[1].0[0], self.0[0].0[2]);
        let (c0, c1) = fp4(self.0[0].0[1], self.0[1].0[2]);
        Self([
            C([minus(a0, self.0[0].0[0]), minus(b0, self.0[0].0[1]), minus(c0, self.0[0].0[2])]),
            C([plus(c1.nr(), self.0[1].0[0]), plus(a1, self.0[1].0[1]), plus(b1, self.0[1].0[2])]),
        ])
    }

    #[inline(never)]
    fn cyclotomic_pow(self, exponent: u64) -> Self {
        let mut result = Self::one();
        for bit in (0..64 - exponent.leading_zeros()).rev() {
            result = result.cyclotomic_square();
            if exponent >> bit & 1 != 0 { result = result.mul(self); }
        }
        result
    }

    fn exp_by_neg_x(self) -> Self {
        self.cyclotomic_pow(4965661367192848881).conjugate()
    }

    pub fn easy_part(self) -> Option<Self> {
        let r = self.conjugate().mul(self.inverse()?);
        Some(r.frobenius(2).mul(r))
    }

    #[inline(never)]
    pub fn final_exponentiation(self) -> Option<Self> {
        let r = self.easy_part()?;
        let y0 = r.exp_by_neg_x();
        let y1 = y0.cyclotomic_square();
        let y2 = y1.cyclotomic_square();
        let y3 = y2.mul(y1);
        let y4 = y3.exp_by_neg_x();
        let y5 = y4.cyclotomic_square();
        let y6 = y5.exp_by_neg_x();
        let y3 = y3.conjugate();
        let y6 = y6.conjugate();
        let y7 = y6.mul(y4);
        let y8 = y7.mul(y3);
        let y9 = y8.mul(y1);
        let y10 = y8.mul(y4);
        let y11 = y10.mul(r);
        let y12 = y9.frobenius(1);
        let y13 = y12.mul(y11);
        let y14 = y8.frobenius(2).mul(y13);
        let y15 = r.conjugate().mul(y9).frobenius(3);
        Some(y15.mul(y14))
    }
}

pub fn fully_packed(pairs: &[(G1Affine, &crate::prepared::G2Prepared); 3]) -> Option<Fq12> {
    let values = super::miller_values(pairs).final_exponentiation()?.get();
    Some(values[0] * values[1] * values[2])
}
