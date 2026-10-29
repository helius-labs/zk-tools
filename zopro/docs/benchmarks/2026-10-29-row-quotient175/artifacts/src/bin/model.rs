use num_bigint::BigUint as N;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;
use zopro_bn254::{FpConfig, FqConfig, FrConfig};
const B: u128 = 1 << 52;
const MASK: u128 = B - 1;
fn digits(n: &N) -> [u128; 5] {
    core::array::from_fn(|j| ((n >> (52 * j)) & N::from(MASK)).to_u128().unwrap())
}
fn integer(a: &[u128]) -> N {
    a.iter()
        .rev()
        .fold(N::zero(), |x, v| (x << 52) + N::from(*v))
}
fn pair(t: &mut [u128], j: usize, a: u128, b: u128) {
    assert!(a < B && b < B);
    let p = a * b;
    t[j] += p & MASK;
    t[j + 1] += p >> 52;
}
fn cancel(t: &mut [u128], off: usize, q: u128, p: &[u128; 5], bits: usize) -> u128 {
    let d = 1u128 << bits;
    assert_eq!((t[off] + q * p[0]) & (d - 1), 0);
    let low = (t[off] + d - 1) / d + (q * p[0] >> bits);
    for j in 1..5 {
        pair(t, off + j, q, p[j]);
    }
    if bits == 52 {
        t[off + 1] += low;
    }
    low
}
#[derive(Default)]
struct Counts {
    cases: u64,
    max_coefficient: u128,
    max_q1: u128,
    max_q2: u128,
    prefix_checks: u64,
}
fn bound(t: &[u128], stats: &mut Counts) {
    let m = *t.iter().max().unwrap();
    assert!(m < 1 << 61);
    stats.max_coefficient = stats.max_coefficient.max(m);
}
fn multiply(input: [&N; 4], fused: bool, mixed: bool, p: &N, inv: &N, stats: &mut Counts) -> N {
    let [a, b, c, d] = input.map(digits);
    let pd = digits(p);
    let iv = digits(inv);
    let a0i = (a[0] * iv[0]) & MASK;
    let c0i = (c[0] * iv[0]) & MASK;
    let partials: [u128; 5] = core::array::from_fn(|i| {
        ((a0i * b[i]) & MASK) + if fused { (c0i * d[i]) & MASK } else { 0 }
    });
    let mut t = [0u128; 6];
    for i in 0..5 {
        let bits = if i == 4 && mixed { 48 } else { 52 };
        let digit_mask = (1u128 << bits) - 1;
        let qraw = ((t[0] * iv[0]) & MASK) + partials[i];
        assert!(qraw < 3 * B);
        let m = qraw & digit_mask;
        stats.max_q1 = stats.max_q1.max(*partials.iter().max().unwrap());
        stats.max_q2 = stats.max_q2.max(qraw);
        let before = integer(&t);
        let row = input[0] * N::from(b[i])
            + if fused {
                input[2] * N::from(d[i])
            } else {
                N::zero()
            };
        for j in 0..5 {
            pair(&mut t, j, a[j], b[i]);
            if fused {
                pair(&mut t, j, c[j], d[i]);
            }
        }
        bound(&t, stats);
        assert_eq!(m, (t[0] * iv[0]) & digit_mask);
        stats.prefix_checks += 1;
        assert_eq!(integer(&t), before + row);
        let low = cancel(&mut t, 0, m, &pd, bits);
        bound(&t, stats);
        if bits == 52 {
            t = [t[1], t[2], t[3], t[4], t[5], 0];
        } else {
            let mask48 = (1u128 << 48) - 1;
            assert!(t[5] < 3 * (1u128 << 44));
            let old = t;
            t = [
                low + ((old[1] & mask48) << 4),
                (old[1] >> 48) + ((old[2] & mask48) << 4),
                (old[2] >> 48) + ((old[3] & mask48) << 4),
                (old[3] >> 48) + ((old[4] & mask48) << 4),
                (old[4] >> 48) + (old[5] << 4),
                0,
            ];
        }
        bound(&t, stats);
    }
    for j in 0..4 {
        t[j + 1] += t[j] >> 52;
        t[j] &= MASK;
    }
    bound(&t, stats);
    let computed_raw = integer(&t[..5]);
    let bits = if mixed { 256usize } else { 260usize };
    let r = N::one() << bits;
    let mut full = input[0] * input[1];
    if fused {
        full += input[2] * input[3];
    }
    let q = (&full * inv) % &r;
    let raw = (&full + q * p) >> bits;
    assert_eq!(computed_raw, raw);
    let mut result = computed_raw;
    if fused && mixed && result >= p * 2u32 {
        result -= p * 2u32;
    }
    assert!(result < p * 2u32);
    assert!(digits(&result).iter().all(|v| *v < B));
    stats.cases += 1;
    result
}
fn main() {
    let mut stats = Counts::default();
    let mut fixtures = Vec::new();
    for pwords in [FqConfig::P, FrConfig::P] {
        let p = N::from_bytes_le(
            &pwords
                .into_iter()
                .flat_map(u64::to_le_bytes)
                .collect::<Vec<_>>(),
        );
        let r = N::one() << 260usize;
        let mut inv = N::one();
        for _ in 0..9 {
            inv = (&inv * (&r + 2u32 - ((&p * &inv) % &r))) % &r;
        }
        let inv = &r - inv;
        assert_eq!((&p * &inv + 1u32) % &r, N::zero());
        fixtures.push(json!({"modulus":format!("{p:x}"),"negative_inverse_low156_reference":digits(&inv)[..3].to_vec()}));
        let twice = &p * 2u32;
        let edge = [
            N::zero(),
            N::one(),
            &p - 1u32,
            p.clone(),
            &p + 1u32,
            &twice - 1u32,
            twice.clone(),
        ];
        let mut values = Vec::new();
        for i in 0usize..2401 {
            values.push(core::array::from_fn::<_, 4, _>(|j| {
                edge[(i / 7usize.pow(j as u32)) % 7].clone()
            }));
        }
        let mut seed = 0x545249504c455133u64;
        for _ in 0..4096 {
            values.push(core::array::from_fn(|_| {
                let mut b = [0u8; 64];
                for c in b.chunks_mut(8) {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    c.copy_from_slice(&seed.to_le_bytes());
                }
                N::from_bytes_le(&b) % (&twice + 1u32)
            }));
        }
        for mixed in [false, true] {
            for fused in [false, true] {
                for tuple in &values {
                    multiply(tuple.each_ref(), fused, mixed, &p, &inv, &mut stats);
                }
                let mut state = values[2401][0].clone();
                for i in 0..1024 {
                    let v = &values[(i * 13 + 7) % values.len()];
                    state = multiply(
                        [&state, &v[1], &v[2], &v[3]],
                        fused,
                        mixed,
                        &p,
                        &inv,
                        &mut stats,
                    );
                }
            }
        }
    }
    assert_eq!(stats.cases, 60168);
    println!(
        "{}",
        json!({"status":"passed","cases":stats.cases,"prefix_checks":stats.prefix_checks,"maximum_coefficient":stats.max_coefficient,"maximum_q1raw":stats.max_q1,"maximum_q2raw":stats.max_q2,"inverse_fixtures":fixtures,"scope":"everydistributedrowquotient andrawrowinteger; fullmixed48/uniform52terminalpacking; exactcanonicalREDCrepresentatives; bothprimes ordinary/fused inclusive2p/chains"})
    );
}
