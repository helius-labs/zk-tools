use num_bigint::BigUint as N;
use num_traits::{ToPrimitive, Zero};
use serde_json::json;
use zopro_bn254::{FpConfig, FqConfig, FrConfig};
const B: u128 = 1 << 52;
const MASK: u128 = B - 1;
fn integer(a: &[u128]) -> N {
    a.iter()
        .rev()
        .fold(N::zero(), |x, n| (x << 52) + N::from(*n))
}
fn main() {
    let mut cases = 0;
    let mut seed = 0x2410_a17d_5eed_0715u64;
    for (pw, iv) in [(FqConfig::P, FqConfig::INV), (FrConfig::P, FrConfig::INV)] {
        let p = N::from_bytes_le(
            &pw.into_iter()
                .flat_map(u64::to_le_bytes)
                .collect::<Vec<_>>(),
        );
        let pd: [u128; 5] =
            core::array::from_fn(|j| ((&p >> (j * 52)) & N::from(MASK)).to_u128().unwrap());
        for bits in [48, 52] {
            let radix = 1u128 << bits;
            for i in 0..4103 {
                let mut next = || {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    seed as u128
                };
                let mut t: [u128; 6] = core::array::from_fn(|_| next() % (128 * B));
                t[5] = next() % (1 << 40);
                if i < 7 {
                    t[0] = [
                        0,
                        1,
                        radix - 1,
                        radix,
                        2 * radix - 1,
                        127 * radix,
                        128 * radix - 1,
                    ][i];
                }
                let before = integer(&t);
                let q = (t[0] * (iv as u128 & MASK)) & (radix - 1);
                assert_eq!((t[0] + q * pd[0]) & (radix - 1), 0);
                let low = (t[0] + radix - 1) / radix + (q * pd[0] >> bits);
                for j in 1..5 {
                    let v = q * pd[j];
                    t[j] += v & MASK;
                    t[j + 1] += v >> 52;
                }
                let out = if bits == 52 {
                    t[1] += low;
                    [t[1], t[2], t[3], t[4], t[5], 0]
                } else {
                    let m = radix - 1;
                    [
                        low + ((t[1] & m) << 4),
                        (t[1] >> 48) + ((t[2] & m) << 4),
                        (t[2] >> 48) + ((t[3] & m) << 4),
                        (t[3] >> 48) + ((t[4] & m) << 4),
                        (t[4] >> 48) + (t[5] << 4),
                        0,
                    ]
                };
                assert_eq!(integer(&out), (before + N::from(q) * &p) >> bits);
                assert!(out.iter().all(|v| *v < 1 << 61));
                cases += 1;
            }
        }
    }
    assert_eq!(cases, 16412);
    println!(
        "{}",
        json!({"status":"passed","cases":cases,"scope":"independent arbitrary redundant-coefficient cancelled-low carry and postrow52/mixed48 packing integer identities; complements60168fullproduct model, not repeated old model vectors","model_repeated":false})
    );
}
