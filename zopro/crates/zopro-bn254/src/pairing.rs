//! The optimal ate pairing on BN254 (a D-type sextic twist), as arkworks
//! computes it: G2 points prepared into their Miller loop's line
//! coefficients, the loop over the signed digits of 6x + 2, and the final
//! exponentiation's easy part and Fuentes-Castañeda et al.'s hard part.
use crate::{curve::PSI_X, curve::PSI_Y, Fq, Fq12, Fq2, G1Affine, G2Affine};

/// The BN parameter x (positive for BN254).
const X: u64 = 4965661367192848881;

/// The signed binary digits of 6x + 2, least significant first.
const ATE_LOOP_COUNT: [i8; 65] = [
    0, 0, 0, 1, 0, 1, 0, -1, 0, 0, -1, 0, 0, 0, 1, 0, 0, -1, 0, -1, 0, 0, 0, 1, 0, -1, 0, 0, 0, 0,
    -1, 0, 0, 1, 0, -1, 0, 0, 1, 0, 0, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, -1, 0, -1, 0,
    0, 0, 1, 0, 1, 1,
];

type Line = (Fq2, Fq2, Fq2);

/// A G2 point's line coefficients for the Miller loop.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct G2Prepared {
    lines: Vec<Line>,
}

/// A point in homogeneous projective coordinates (x = X / Z, y = Y / Z).
#[derive(Clone, Copy)]
struct Homogeneous {
    x: Fq2,
    y: Fq2,
    z: Fq2,
}

impl Homogeneous {
    fn double(&mut self, two_inv: &Fq) -> Line {
        let a = (self.x * self.y).mul_by_fp(two_inv);
        let b = self.y.square();
        let c = self.z.square();
        let e = <crate::G2Config as crate::CurveConfig>::B * (c.double() + c);
        let f = e.double() + e;
        let g = (b + f).mul_by_fp(two_inv);
        let h = (self.y + self.z).square() - (b + c);
        let i = e - b;
        let j = self.x.square();
        let e_square = e.square();
        self.x = a * (b - f);
        self.y = g.square() - (e_square.double() + e_square);
        self.z = b * h;
        (-h, j.double() + j, i)
    }

    fn add(&mut self, q: &G2Affine) -> Line {
        let theta = self.y - q.y * self.z;
        let lambda = self.x - q.x * self.z;
        let c = theta.square();
        let d = lambda.square();
        let e = lambda * d;
        let f = self.z * c;
        let g = self.x * d;
        let h = e + f - g.double();
        self.x = lambda * h;
        self.y = theta * (g - h) - e * self.y;
        self.z *= e;
        let j = theta * q.x - lambda * q.y;
        (lambda, -theta, j)
    }
}

impl G2Prepared {
    pub fn new(q: &G2Affine) -> Self {
        if q.is_identity() {
            return Self { lines: Vec::new() };
        }
        let two_inv = Fq::from_u64(2).inverse().expect("2 is invertible");
        let mut lines = Vec::with_capacity(ATE_LOOP_COUNT.len() + 40);
        let mut r = Homogeneous {
            x: q.x,
            y: q.y,
            z: Fq2::ONE,
        };
        let neg_q = -*q;
        for &digit in ATE_LOOP_COUNT.iter().rev().skip(1) {
            lines.push(r.double(&two_inv));
            match digit {
                1 => lines.push(r.add(q)),
                -1 => lines.push(r.add(&neg_q)),
                _ => {}
            }
        }
        let q1 = mul_by_char(q);
        let mut q2 = mul_by_char(&q1);
        q2.y = -q2.y;
        lines.push(r.add(&q1));
        lines.push(r.add(&q2));
        Self { lines }
    }

    pub fn is_identity(&self) -> bool {
        self.lines.is_empty()
    }
}

/// The Frobenius twist psi.
fn mul_by_char(q: &G2Affine) -> G2Affine {
    G2Affine::new(q.x.frobenius(1) * PSI_X, q.y.frobenius(1) * PSI_Y)
}

/// A line evaluated at p, into f.
fn ell(f: &mut Fq12, line: &Line, p: &G1Affine) {
    let c0 = line.0.mul_by_fp(&p.y);
    let c1 = line.1.mul_by_fp(&p.x);
    *f = f.mul_by_034(&c0, &c1, &line.2);
}

/// The product of the Miller loops of the pairs.
pub fn multi_miller_loop(pairs: &[(G1Affine, &G2Prepared)]) -> Fq12 {
    let pairs: Vec<_> = pairs
        .iter()
        .filter(|(p, q)| !p.is_identity() && !q.is_identity())
        .map(|(p, q)| (*p, q.lines.iter()))
        .collect();
    let mut pairs = pairs;
    let mut f = Fq12::ONE;
    for i in (1..ATE_LOOP_COUNT.len()).rev() {
        if i != ATE_LOOP_COUNT.len() - 1 {
            f = f.square();
        }
        for (p, lines) in pairs.iter_mut() {
            ell(&mut f, lines.next().expect("a line per step"), p);
        }
        let digit = ATE_LOOP_COUNT[i - 1];
        if digit == 1 || digit == -1 {
            for (p, lines) in pairs.iter_mut() {
                ell(&mut f, lines.next().expect("a line per step"), p);
            }
        }
    }
    for _ in 0..2 {
        for (p, lines) in pairs.iter_mut() {
            ell(&mut f, lines.next().expect("the two last lines"), p);
        }
    }
    f
}

/// f^x, then inverted (x is positive): the hard part's f^-x.
fn exp_by_neg_x(f: Fq12) -> Fq12 {
    f.pow_u64(X).conjugate()
}

/// f^((p^12 - 1) / r), `None` for zero.
pub fn final_exponentiation(f: Fq12) -> Option<Fq12> {
    let f1 = f.conjugate();
    let f2 = f.inverse()?;
    let mut r = f1 * f2;
    let f2 = r;
    r = r.frobenius(2) * f2;
    let y0 = exp_by_neg_x(r);
    let y1 = y0.square();
    let y2 = y1.square();
    let y3 = y2 * y1;
    let y4 = exp_by_neg_x(y3);
    let y5 = y4.square();
    let y6 = exp_by_neg_x(y5);
    let y3 = y3.conjugate();
    let y6 = y6.conjugate();
    let y7 = y6 * y4;
    let y8 = y7 * y3;
    let y9 = y8 * y1;
    let y10 = y8 * y4;
    let y11 = y10 * r;
    let y12 = y9.frobenius(1);
    let y13 = y12 * y11;
    let y14 = y8.frobenius(2) * y13;
    let y15 = (r.conjugate() * y9).frobenius(3);
    Some(y15 * y14)
}

/// Whether the product of the pairings of the pairs is one.
pub fn pairing_product_is_one(pairs: &[(G1Affine, &G2Prepared)]) -> bool {
    final_exponentiation(multi_miller_loop(pairs)) == Some(Fq12::ONE)
}

/// e(p, q).
pub fn pairing(p: &G1Affine, q: &G2Affine) -> Fq12 {
    let q = G2Prepared::new(q);
    final_exponentiation(multi_miller_loop(&[(*p, &q)])).unwrap_or(Fq12::ONE)
}
