//! Five-product cubic multiplication using evaluations at 0, 1, -1, u, infinity.
use super::*;

// Scalar Fq operations return canonical Montgomery residues below p. Dividing
// that integer by two modulo p preserves Montgomery scaling. x+p<2p<2^255,
// so the optional addition and right shift never discard a top carry.
#[inline(always)]
fn half(x: Fq) -> Fq {
    let mask = 0u64.wrapping_sub(x.0[0] & 1);
    let mut out = [0u64; 4];
    let mut carry = 0u128;
    for i in 0..4 {
        let sum = x.0[i] as u128 + (Fq::MODULUS[i] & mask) as u128 + carry;
        out[i] = sum as u64;
        carry = sum >> 64;
    }
    debug_assert_eq!(carry, 0);
    for i in 0..3 { out[i] = (out[i] >> 1) | (out[i + 1] << 63); }
    out[3] >>= 1;
    Fq::from_montgomery(out)
}
#[inline(always)]
fn half2(x: Fq2) -> Fq2 { Fq2::new(half(x.c0), half(x.c1)) }
#[inline(always)]
fn times_u(x: Fq2) -> Fq2 { Fq2::new(-x.c1, x.c0) }

pub(super) fn multiply(a: Fq6, b: Fq6) -> Fq6 {
    let v0 = a.c0 * b.c0;
    let v4 = a.c2 * b.c2;
    let sa = a.c0 + a.c2;
    let sb = b.c0 + b.c2;
    let v1 = (sa + a.c1) * (sb + b.c1);
    let vm1 = (sa - a.c1) * (sb - b.c1);
    let vi = (a.c0 - a.c2 + times_u(a.c1)) * (b.c0 - b.c2 + times_u(b.c1));
    let odd_sum = half2(v1 - vm1); // c1+c3
    let c2 = half2(v1 + vm1) - v0 - v4;
    let odd_difference = -times_u(vi - v0 + c2 - v4); // c1-c3
    let c1 = half2(odd_sum + odd_difference);
    let c3 = odd_sum - c1;
    Fq6::new(v0 + c3.mul_by_nonresidue(), c1 + v4.mul_by_nonresidue(), c2)
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{BigInt, Field, UniformRand, AdditiveGroup};
    use ark_ref as ark;
    fn from(x: ark::Fq6) -> Fq6 {
        let cv = |x: ark::Fq2| Fq2::new(Fq::from_montgomery(x.c0.0.0), Fq::from_montgomery(x.c1.0.0));
        Fq6::new(cv(x.c0),cv(x.c1),cv(x.c2))
    }
    #[test]
    fn halving_canonical_montgomery_edges_and_random() {
        let p=Fq::MODULUS;
        assert!(p[3] < 1u64<<62);
        let raw_minus_one=crate::portable::sub4(p,[1,0,0,0]).0;
        let raw_minus_two=crate::portable::sub4(p,[2,0,0,0]).0;
        let mut values=vec![Fq::ZERO,Fq::ONE,-Fq::ONE,Fq::from_montgomery([1,0,0,0]),Fq::from_montgomery(raw_minus_one),Fq::from_montgomery(raw_minus_two)];
        let mut rng=ark_std::test_rng();for _ in 0..4096 { let x=ark::Fq::rand(&mut rng);values.push(Fq::from_montgomery(x.0.0)); }
        let inv2=ark::Fq::from(2u64).inverse().unwrap();
        for x in values {let y=half(x);assert_eq!(y.double(),x);assert_eq!(ark::Fq::new_unchecked(BigInt(y.0)),ark::Fq::new_unchecked(BigInt(x.0))*inv2);assert!(crate::portable::less(y.0,p));}
    }
    #[test]
    fn cubic_products_match_arkworks_and_schoolbook_chains() {
        let mut rng=ark_std::test_rng();
        let mut edges=vec![ark::Fq6::ZERO,ark::Fq6::ONE,-ark::Fq6::ONE];
        for j in 0..6 {for x in [ark::Fq::ONE,-ark::Fq::ONE,ark::Fq::from(2u64)] {let mut a=ark::Fq6::ZERO;let p=[&mut a.c0.c0,&mut a.c0.c1,&mut a.c1.c0,&mut a.c1.c1,&mut a.c2.c0,&mut a.c2.c1];*p.into_iter().nth(j).unwrap()=x;edges.push(a);}}
        let mut checks=0;
        for &a in &edges {for &b in &edges {assert_eq!(multiply(from(a),from(b)),from(a*b));assert_eq!(from(a)*from(b),from(a*b));checks+=1;}}
        for _ in 0..1024 {let mut a=ark::Fq6::rand(&mut rng);let b=ark::Fq6::rand(&mut rng);let c=ark::Fq6::rand(&mut rng);let mut actual=from(a);for _ in 0..8 {actual=multiply(actual,from(b))+from(c);a=a*b+c;assert_eq!(actual,from(a));checks+=1;}}
        assert_eq!(checks,8633);
        println!("8633 cubic edge/changing-chain cases; 4102 half cases; no new proof claims");
    }
}
