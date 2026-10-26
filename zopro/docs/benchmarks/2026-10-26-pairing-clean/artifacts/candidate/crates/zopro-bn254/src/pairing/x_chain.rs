//! Fixed addition/subtraction schedule from Gnark Crypto v0.21.0 Expt.
//! https://github.com/Consensys-Incorporated/gnark-crypto/blob/v0.21.0/ecc/bn254/internal/fptower/e12_pairing.go
//! This mathematical schedule uses conjugation only in the cyclotomic subgroup.
use super::*;
#[inline]
fn execute<T: Copy>(
    x: T,
    square: impl Fn(T) -> T,
    mul: impl Fn(T, T) -> T,
    conjugate: impl Fn(T) -> T,
) -> T {
    let repeat = |mut t: T, n: usize| {
        for _ in 0..n {
            t = square(t);
        }
        t
    };
    let x2 = square(x);
    let x3 = mul(x, x2);
    let x5 = mul(x2, x3);
    let x7 = mul(x2, x5);
    let mut t = mul(x, x7);
    t = repeat(t, 3);
    t = mul(x5, t);
    t = repeat(t, 5);
    t = mul(t, conjugate(x3));
    t = repeat(t, 4);
    t = mul(t, x3);
    t = repeat(t, 5);
    t = mul(t, x5);
    t = repeat(t, 4);
    t = mul(t, conjugate(x5));
    t = repeat(t, 4);
    t = mul(t, conjugate(x3));
    t = repeat(t, 4);
    t = mul(t, x);
    t = repeat(t, 5);
    t = mul(t, x5);
    t = repeat(t, 5);
    t = mul(t, x7);
    t = repeat(t, 4);
    t = mul(t, conjugate(x7));
    t = repeat(t, 7);
    t = mul(t, x5);
    t = repeat(t, 5);
    t = mul(t, conjugate(x));
    t = repeat(t, 4);
    mul(t, x)
}
pub(super) fn power(f: Fq12) -> Fq12 {
    execute(
        f,
        |x| x.cyclotomic_square(),
        |a, b| a * b,
        |a| a.conjugate(),
    )
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn symbolic_chain_has_exact_exponent_and_operation_counts() {
        use std::cell::Cell;
        let s = Cell::new(0);
        let m = Cell::new(0);
        let c = Cell::new(0);
        let value = execute(
            1i128,
            |x| {
                s.set(s.get() + 1);
                2 * x
            },
            |a, b| {
                m.set(m.get() + 1);
                a + b
            },
            |a| {
                c.set(c.get() + 1);
                -a
            },
        );
        assert_eq!(value, X as i128);
        assert_eq!((s.get(), m.get(), c.get()), (60, 17, 5));
    }
    #[test]
    fn fixed_chain_and_complete_final_match_general_reference() {
        for seed in 0..128u64 {
            let component =
                |k| Fq2::new(Fq::from_u64(seed * 17 + k), Fq::from_u64(seed * 31 + k + 1));
            let f = Fq12::new(
                crate::Fq6::new(component(1), component(2), component(3)),
                crate::Fq6::new(component(4), component(5), component(6)),
            );
            let easy = f.conjugate() * f.inverse().unwrap();
            let easy = easy.frobenius(2) * easy;
            for x in [easy, easy.conjugate(), easy * easy] {
                assert_eq!(power(x), x.pow_u64(X));
                assert_eq!(power(x), x.cyclotomic_pow_u64(X));
            }
            assert_eq!(
                final_exponentiation(f),
                super::super::final_exponentiation_reference(f)
            );
        }
        assert_eq!(final_exponentiation(Fq12::default()), None);
        assert_eq!(power(Fq12::ONE), Fq12::ONE);
    }
}
