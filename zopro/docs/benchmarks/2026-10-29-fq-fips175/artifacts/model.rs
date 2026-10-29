use num_bigint::BigUint as N;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;
use zopro_bn254::{FpConfig, FqConfig, FrConfig};

const B: u128 = 1 << 52;
const MASK: u128 = B - 1;

#[derive(Default)]
struct Bounds {
    lo: u128,
    hi: u128,
    carry: u128,
    wide: u128,
    cases: usize,
}

fn digits(n: &N) -> [u128; 5] {
    let mask = N::from(MASK);
    core::array::from_fn(|j| ((n >> (52 * j)) & &mask).to_u128().unwrap())
}
fn integer(a: &[u128; 5]) -> N {
    a.iter()
        .rev()
        .fold(N::zero(), |n, x| (n << 52) + N::from(*x))
}
fn pair(lo: &mut u128, hi: &mut u128, a: u128, b: u128) {
    assert!(a < B && b < B);
    let v = a.checked_mul(b).unwrap();
    *lo = lo.checked_add(v & MASK).unwrap();
    *hi = hi.checked_add(v >> 52).unwrap();
}
fn test<C: FpConfig>(bounds: &mut Bounds) {
    let p = N::from_bytes_le(
        &C::P
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>(),
    );
    let limit = &p * 2u32;
    let mut edges = vec![
        N::zero(),
        N::one(),
        N::from(2u32),
        &p - 1u32,
        p.clone(),
        &p + 1u32,
        &limit - 1u32,
        limit.clone(),
    ];
    for bit in [48, 52, 104, 156, 208, 253] {
        let x: N = N::one() << (bit as usize);
        edges.extend([&x - 1u32, x.clone(), &x + 1u32]);
    }
    edges.retain(|x| x <= &limit);
    let mut cases = vec![];
    for (i, a) in edges.iter().enumerate() {
        for (j, b) in edges.iter().enumerate() {
            cases.push([
                a.clone(),
                b.clone(),
                edges[(i * 13 + j) % edges.len()].clone(),
                edges[(i + j * 17) % edges.len()].clone(),
            ]);
        }
    }
    let mut seed = 0x4649505344494147u64;
    for _ in 0..4096 {
        cases.push(core::array::from_fn(|_| {
            let mut bytes = [0u8; 64];
            for chunk in bytes.chunks_mut(8) {
                seed ^= seed << 13;
                seed ^= seed >> 7;
                seed ^= seed << 17;
                chunk.copy_from_slice(&seed.to_le_bytes());
            }
            N::from_bytes_le(&bytes) % (&limit + 1u32)
        }));
    }
    let pd = digits(&p);
    let inv = u128::from(C::INV) & MASK;
    assert_eq!((pd[0] * inv + 1) & MASK, 0);
    for input in cases {
        for mixed in [false, true] {
            for fused in [false, true] {
                let [a, b, c, d] = input.each_ref().map(digits);
                let mut carry = 0u128;
                let mut q = [0u128; 5];
                let mut out = [0u128; 5];
                for k in 0..5 {
                    let mut lo = carry;
                    let mut hi = 0;
                    for i in 0..=k {
                        let j = k - i;
                        pair(&mut lo, &mut hi, a[i], b[j]);
                        if fused {
                            pair(&mut lo, &mut hi, c[i], d[j]);
                        }
                    }
                    for i in 0..k {
                        pair(&mut lo, &mut hi, q[i], pd[k - i]);
                    }
                    let bits = if mixed && k == 4 { 48 } else { 52 };
                    let mask = (1u128 << bits) - 1;
                    q[k] = (lo.checked_mul(inv).unwrap()) & mask;
                    // q*p0's low bits are canceled, so the existing carry identity
                    // needs only its high half, for either52-bit or final48-bit step.
                    assert_eq!((lo + q[k] * pd[0]) & mask, 0);
                    let low = (lo + mask) >> bits;
                    carry = low + (q[k] * pd[0] >> bits);
                    carry = carry
                        .checked_add(if bits == 48 { hi << 4 } else { hi })
                        .unwrap();
                    bounds.lo = bounds.lo.max(lo);
                    bounds.hi = bounds.hi.max(hi);
                    bounds.carry = bounds.carry.max(carry);
                    assert!(lo < 32 * B && hi < 16 * B && carry < (1 << 61));
                }
                for k in 5..=8 {
                    let mut lo = 0;
                    let mut hi = 0;
                    for i in k - 4..=4 {
                        let j = k - i;
                        pair(&mut lo, &mut hi, a[i], b[j]);
                        if fused {
                            pair(&mut lo, &mut hi, c[i], d[j]);
                        }
                        pair(&mut lo, &mut hi, q[i], pd[j]);
                    }
                    let wide = carry.checked_add(if mixed { lo << 4 } else { lo }).unwrap();
                    out[k - 5] = wide & MASK;
                    carry = (wide >> 52)
                        .checked_add(if mixed { hi << 4 } else { hi })
                        .unwrap();
                    bounds.lo = bounds.lo.max(lo);
                    bounds.hi = bounds.hi.max(hi);
                    bounds.carry = bounds.carry.max(carry);
                    bounds.wide = bounds.wide.max(wide);
                    assert!(lo < 32 * B && hi < 16 * B && carry < (1 << 61) && wide < (1 << 61));
                }
                out[4] = carry;
                assert!(out.iter().all(|x| *x < B));
                let u = integer(&out);
                let qq = integer(&q);
                let r = N::one() << if mixed { 256 } else { 260 };
                let t = &input[0] * &input[1]
                    + if fused {
                        &input[2] * &input[3]
                    } else {
                        N::zero()
                    };
                assert_eq!(&u * &r, &t + &qq * &p);
                // Independent conventional whole-integer REDC, no diagonal splitting.
                let mut reference = t.clone();
                let mut qr = [0u128; 5];
                for (i, slot) in qr.iter_mut().enumerate() {
                    let bits = if mixed && i == 4 { 48 } else { 52 };
                    let mask = (N::one() << bits) - 1u32;
                    *slot = (((&reference & &mask) * N::from(inv)) & &mask)
                        .to_u128()
                        .unwrap();
                    reference += N::from(*slot) * &p;
                    assert!((&reference & &mask).is_zero());
                    reference >>= bits;
                }
                assert_eq!(q, qr);
                assert_eq!(u, reference);
                assert!(u < &p * if fused && mixed { 3u32 } else { 2u32 });
                let corrected = if fused && mixed && u >= limit {
                    &u - &limit
                } else {
                    u.clone()
                };
                assert!(corrected < limit);
                assert_eq!((&corrected * &r) % &p, t % &p);
                bounds.cases += 1;
            }
        }
    }
}
fn main() {
    let mut b = Bounds::default();
    test::<FqConfig>(&mut b);
    test::<FrConfig>(&mut b);
    let mut identities = 0;
    for bits in [48, 52] {
        let base = 1u128 << bits;
        let mask = base - 1;
        for lo in [
            0,
            1,
            base - 1,
            base,
            base + 1,
            2 * base - 1,
            15 * base + 7,
            31 * base - 1,
        ] {
            // Use the actual p0/inverse, and another odd modulus digit.
            for p0 in [u128::from(FqConfig::P[0]) & MASK, 3] {
                let mut inv = 1u128;
                for _ in 0..7 {
                    inv = inv.wrapping_mul(2u128.wrapping_sub(p0.wrapping_mul(inv)));
                }
                let q = lo.wrapping_mul(inv.wrapping_neg()) & mask;
                assert_eq!((lo + q * p0) & mask, 0);
                assert_eq!(
                    (lo + q * p0) >> bits,
                    ((lo + mask) >> bits) + (q * p0 >> bits)
                );
                identities += 1;
            }
        }
    }
    println!("{}",serde_json::to_string_pretty(&json!({"passed":true,"cases":b.cases,"carry_identity_cases":identities,"both_primes":true,"radices":[256,260],"ordinary_and_fused":true,"exact_integer_identity":true,"independent_full_integer_REDC":true,"max_lo":b.lo.to_string(),"max_hi":b.hi.to_string(),"max_carry":b.carry.to_string(),"max_wide":b.wide.to_string(),"native_scope":"Fq/R256/eight lanes only","SDK_requests":0,"Gnark_exports":0,"proof_ledger":17317})).unwrap());
}
