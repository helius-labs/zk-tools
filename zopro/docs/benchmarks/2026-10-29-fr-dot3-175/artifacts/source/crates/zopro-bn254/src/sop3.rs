//! Three canonical field products with one Montgomery reduction.
#[inline(always)]
pub(crate) fn sum3(left: [[u64; 4]; 3], right: [[u64; 4]; 3], p: [u64; 4], inv: u64) -> [u64; 4] {
    let mut t = [0u64; 5];
    for i in 0..4 {
        for k in 0..3 {
            let mut carry = 0u64;
            for j in 0..4 {
                let wide =
                    (left[k][j] as u128) * (right[k][i] as u128) + t[j] as u128 + carry as u128;
                t[j] = wide as u64;
                carry = (wide >> 64) as u64;
            }
            #[cfg(test)]
            assert!(!t[4].overflowing_add(carry).1);
            t[4] += carry;
        }
        let q = t[0].wrapping_mul(inv);
        let wide = (q as u128) * (p[0] as u128) + t[0] as u128;
        #[cfg(test)]
        assert_eq!(wide as u64, 0);
        let mut carry = (wide >> 64) as u64;
        for j in 1..4 {
            let wide = (q as u128) * (p[j] as u128) + t[j] as u128 + carry as u128;
            t[j - 1] = wide as u64;
            carry = (wide >> 64) as u64;
        }
        let wide = t[4] as u128 + carry as u128;
        t[3] = wide as u64;
        t[4] = (wide >> 64) as u64;
        #[cfg(test)]
        assert_eq!(t[4], 0);
    }
    crate::portable::reduce_once([t[0], t[1], t[2], t[3]], p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Fp, FpConfig, FqConfig, FrConfig};

    #[test]
    fn small_radix_exact_cancellation() {
        let p = 13u128;
        let b = 16u128;
        let r = b * b;
        let inv = (0..b).find(|x| (x * p + 1) % b == 0).unwrap();
        let rinv = (0..p).find(|x| (x * r) % p == 1).unwrap();
        let mut count = 0;
        for a in 0..p {
            for c in 0..p {
                for d in 0..p {
                    for f in 0..p {
                        let left = [a, c, (a + d + 7) % p];
                        let right = [d, f, (c + f + 3) % p];
                        let input = left.iter().zip(right).map(|(x, y)| x * y).sum::<u128>();
                        let mut t = 0;
                        let mut quotient = 0;
                        for i in 0..2 {
                            assert!(t < 4 * p);
                            for k in 0..3 {
                                t += left[k] * ((right[k] >> (4 * i)) & (b - 1));
                            }
                            let q = t * inv & (b - 1);
                            quotient += q << (4 * i);
                            t += q * p;
                            assert_eq!(t & (b - 1), 0);
                            assert!(t < 4 * p * b);
                            t >>= 4;
                        }
                        assert_eq!(t * r, input + quotient * p);
                        assert!(t < 2 * p);
                        let reduced = if t >= p { t - p } else { t };
                        assert_eq!(reduced, input * rinv % p);
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(count, 28561);
    }

    fn check<C: FpConfig, A: ark_ff::PrimeField>() {
        use ark_ff::Field;
        let rinv = A::from(2u64).pow([256]).inverse().unwrap();
        let convert = |x: Fp<C>| {
            A::from_le_bytes_mod_order(
                &x.0.into_iter()
                    .flat_map(u64::to_le_bytes)
                    .collect::<Vec<_>>(),
            ) * rinv
        };
        let p = C::P;
        let mut edges = vec![
            [0; 4],
            [1, 0, 0, 0],
            crate::portable::sub4(p, [1, 0, 0, 0]).0,
            C::R,
            C::R2,
        ];
        for bit in [48, 52, 63, 64, 127, 128, 191, 192, 253] {
            let mut x = [0u64; 4];
            x[bit / 64] = 1u64 << (bit % 64);
            for raw in [
                crate::portable::sub4(x, [1, 0, 0, 0]).0,
                x,
                crate::portable::add4(x, [1, 0, 0, 0]).0,
            ] {
                if crate::portable::less(raw, p) {
                    edges.push(raw);
                }
            }
        }
        let mut cases = vec![];
        for (i, a) in edges.iter().enumerate() {
            for (j, b) in edges.iter().enumerate() {
                cases.push([
                    *a,
                    *b,
                    edges[(i * 7 + j) % edges.len()],
                    edges[(i + j * 13) % edges.len()],
                    edges[(i * 3 + j * 5) % edges.len()],
                    edges[(i * 11 + j) % edges.len()],
                ]);
            }
        }
        let mut seed = 0x534f50334652444fu64;
        for _ in 0..4096 {
            cases.push(core::array::from_fn(|_| {
                let mut bytes = [0u8; 64];
                for chunk in bytes.chunks_mut(8) {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    chunk.copy_from_slice(&seed.to_le_bytes());
                }
                Fp::<C>::from_random_bytes(&bytes).0
            }));
        }
        for input in cases {
            let values = input.map(Fp::<C>::from_montgomery);
            let a = values.map(&convert);
            let got = Fp::<C>::from_montgomery(sum3(
                [input[0], input[2], input[4]],
                [input[1], input[3], input[5]],
                p,
                C::INV,
            ));
            let expected = a[0] * a[1] + a[2] * a[3] + a[4] * a[5];
            assert_eq!(convert(got), expected);
            assert!(crate::portable::less(got.0, p));
            assert_eq!(
                got,
                Fp::<C>::sum_of_products(&values[0], &values[1], &values[2], &values[3])
                    + values[4] * values[5]
            );
            let mut chain = got;
            let mut want = expected;
            for _ in 0..16 {
                chain = Fp::<C>::from_montgomery(sum3(
                    [chain.0, input[2], input[4]],
                    [input[1], input[3], input[5]],
                    p,
                    C::INV,
                ));
                want = want * a[1] + a[2] * a[3] + a[4] * a[5];
                assert_eq!(convert(chain), want);
                assert!(crate::portable::less(chain.0, p));
            }
        }
    }
    #[test]
    fn canonical_sum3_and_chains_match_arkworks() {
        check::<FqConfig, ark_ref::Fq>();
        check::<FrConfig, ark_ref::Fr>();
    }
}
