use num_bigint::BigInt;
use num_traits::{One, Signed, Zero};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Pair(pub BigInt, pub BigInt);
impl Pair {
    pub fn new(a: impl Into<BigInt>, b: impl Into<BigInt>) -> Self { Self(a.into(), b.into()) }
}
pub fn modulo(a: BigInt, p: &BigInt) -> BigInt {
    let a = a % p;
    if a.is_negative() { a + p } else { a }
}
pub fn inverse(a: &BigInt, p: &BigInt) -> BigInt {
    let (mut x, mut y) = (p.clone(), modulo(a.clone(), p));
    let (mut s, mut t) = (BigInt::zero(), BigInt::one());
    while !y.is_zero() {
        let q = &x / &y;
        (x, y) = (y.clone(), x - &q * y);
        (s, t) = (t.clone(), s - q * t);
    }
    assert_eq!(x, BigInt::one());
    modulo(s, p)
}
fn round_div(a: BigInt, p: &BigInt) -> BigInt {
    if a.is_negative() { -round_div(-a, p) } else { (a + p / 2u32) / p }
}
fn ceil_div(a: BigInt, p: &BigInt) -> BigInt { (a + p - 1u32) / p }

pub struct Params {
    pub d: u32,
    pub e: u32,
    pub pi: Pair,
    pub p: BigInt,
    pub radix: BigInt,
    pub h: BigInt,
    pub root: BigInt,
    pub neg_inv: Pair,
    pub radix_inverse: BigInt,
}
impl Params {
    pub fn new(d: u32, e: u32, pi: Pair, p: BigInt, bits: usize) -> Self {
        let radix = BigInt::one() << bits;
        let norm = &pi.0 * &pi.0 - e * &pi.0 * &pi.1 + d * &pi.1 * &pi.1;
        assert_eq!(norm, p);
        let k = inverse(&p, &radix);
        let root = modulo(-&pi.0 * inverse(&pi.1, &p), &p);
        assert!(modulo(&root * &root + e * &root + d, &p).is_zero());
        let neg_inv = Pair::new(modulo(-(&pi.0 - e * &pi.1) * &k, &radix), modulo(&pi.1 * &k, &radix));
        let h = (BigInt::one() << 126usize) + 3u32 * &pi.1;
        let radix_inverse = inverse(&radix, &p);
        let s = Self { d, e, pi, p, radix, h, root, neg_inv, radix_inverse };
        let n = s.mul_direct(&s.pi, &s.neg_inv);
        assert!(modulo(n.0 + 1u32, &s.radix).is_zero());
        assert!(modulo(n.1, &s.radix).is_zero());
        s
    }
    pub fn mul_direct(&self, a: &Pair, b: &Pair) -> Pair {
        Pair::new(&a.0 * &b.0 - self.d * &a.1 * &b.1,
            &a.0 * &b.1 + &a.1 * &b.0 - self.e * &a.1 * &b.1)
    }
    pub fn mul_three(&self, a: &Pair, b: &Pair) -> Pair {
        let p = &a.0 * &b.0;
        let q = &a.1 * &b.1;
        let s = (&a.0 + &a.1) * (&b.0 + &b.1);
        Pair::new(&p - self.d * &q, s - p - (self.e + 1) * q)
    }
    pub fn center(&self, a: BigInt) -> BigInt {
        modulo(a + &self.radix / 2u32, &self.radix) - &self.radix / 2u32
    }
    pub fn redc(&self, t: &Pair) -> Pair {
        let q = self.mul_three(t, &self.neg_inv);
        let q = Pair::new(self.center(q.0), self.center(q.1));
        let qp = self.mul_three(&q, &self.pi);
        let z = Pair::new(&t.0 + qp.0, &t.1 + qp.1);
        assert!(modulo(z.0.clone(), &self.radix).is_zero());
        assert!(modulo(z.1.clone(), &self.radix).is_zero());
        Pair::new(z.0 / &self.radix, z.1 / &self.radix)
    }
    pub fn value(&self, z: &Pair) -> BigInt { modulo(&z.0 + &z.1 * &self.root, &self.p) }
    pub fn decode(&self, z: &Pair) -> BigInt { modulo(self.value(z) * &self.radix_inverse, &self.p) }
    pub fn reduce(&self, z: &Pair) -> Pair {
        let q0 = round_div(&z.0 * (&self.pi.0 - self.e * &self.pi.1) + self.d * &self.pi.1 * &z.1, &self.p);
        let q1 = round_div(&self.pi.0 * &z.1 - &self.pi.1 * &z.0, &self.p);
        let q = self.mul_direct(&Pair::new(q0, q1), &self.pi);
        Pair::new(&z.0 - q.0, &z.1 - q.1)
    }
    pub fn encode(&self, z: &BigInt) -> Pair { self.reduce(&Pair::new(modulo(z * &self.radix, &self.p), 0)) }
    pub fn bounds(&self) -> Pair {
        let re = ceil_div((self.d + 1) * &self.h * &self.h, &self.radix)
            + ceil_div(self.pi.0.abs() + self.d * self.pi.1.abs(), &2u32.into());
        let im = ceil_div((self.e + 2) * &self.h * &self.h, &self.radix)
            + ceil_div(self.pi.1.abs() + (&self.pi.0 - self.e * &self.pi.1).abs(), &2u32.into());
        Pair::new(re, im)
    }
    pub fn in_box(&self, z: &Pair) -> bool { z.0.abs() <= self.h && z.1.abs() <= self.h }
}

pub fn final_high_survives(bits: usize) -> bool {
    assert!((105..=156).contains(&bits));
    let last = bits - 104;
    156 - last < bits
}
pub fn coupled_sites(bits: usize, modulus_second_digits: usize) -> (usize, usize, usize) {
    let mut raw = 0;
    let mut modulus = 0;
    let mut cancellation = 0;
    for row in 0..3 {
        for i in 0..3 {
            let high = row != 2 || i != 2 || final_high_survives(bits);
            raw += 3 * (1 + usize::from(high));
            // Three-product row: pi.real, pi.imag and their sum. The
            // real and sum constants occupy three digits, imag may not.
            modulus += (2 + usize::from(i < modulus_second_digits)) * (1 + usize::from(high));
        }
        cancellation += 3;
    }
    (raw, modulus, cancellation)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn two_small_prime_rings_match_exhaustive_residue_arithmetic() {
        for (d,e,pi) in [(3,0,Pair::new(1,2)), (1,1,Pair::new(4,1))] {
            let p = Params::new(d,e,pi,13u32.into(),11);
            for x in 0..13 { for y in 0..13 {
                let a = p.encode(&x.into()); let b = p.encode(&y.into());
                assert_eq!(p.mul_three(&a,&b),p.mul_direct(&a,&b));
                assert_eq!(p.decode(&p.redc(&p.mul_three(&a,&b))),BigInt::from(x*y%13));
            }}
        }
    }
    #[test]
    fn terminal_coefficient_is_zero_at130_but_not131() {
        for bits in [128usize,130,131,132] {
            let contribution = BigInt::one() << (156 - (bits - 104));
            let nonzero = !modulo(contribution, &(BigInt::one()<<bits)).is_zero();
            assert_eq!(final_high_survives(bits),nonzero);
        }
        assert_eq!(coupled_sites(130,2),(51,46,9));
        assert_eq!(coupled_sites(131,2),(54,48,9));
    }
    #[test]
    fn centered_cancellation_preserves_signed_products() {
        let p=Params::new(1,1,Pair::new(4,1),13u32.into(),11);
        for x in [-4097,-2048,-2047,-1,0,1,2047,2048,4097] {
            for y in [-2049,-1,0,1,2049] {
                let t=Pair::new(x,y);let z=p.redc(&t);
                assert_eq!(p.value(&z),modulo(p.value(&t)*&p.radix_inverse,&p.p));
            }
        }
    }
}
