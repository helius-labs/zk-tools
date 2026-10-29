use num_bigint::BigUint as N;
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;
const B: u128 = 1 << 52;
const MASK: u128 = B - 1;
fn words(n: &N) -> [u128; 5] {
    core::array::from_fn(|j| ((n >> (52 * j)) & N::from(MASK)).to_u128().unwrap())
}
fn integer(v: &[u128]) -> N {
    v.iter()
        .rev()
        .fold(N::zero(), |a, b| (a << 52) + N::from(*b))
}
#[derive(Default)]
struct Stats {
    pairs: usize,
    quotients: usize,
    postrows: usize,
    max_partial: u128,
    max_coeff: u128,
}
fn reduce(input: [&N; 4], parts: &[u128; 5], mixed: bool, p: &N, inv: &N, st: &mut Stats) -> N {
    let [a, b, c, d] = input.map(words);
    let pd = words(p);
    let iv = words(inv)[0];
    let mut t = [0u128; 6];
    for i in 0..5 {
        let bits = if mixed && i == 4 { 48 } else { 52 };
        let mask = (1u128 << bits) - 1;
        let q = (((t[0] * iv) & MASK) + parts[i]) & mask;
        for j in 0..5 {
            for (x, y) in [(a[j], b[i]), (c[j], d[i])] {
                let z = x * y;
                t[j] += z & MASK;
                t[j + 1] += z >> 52;
            }
        }
        assert_eq!(q, (t[0] * iv) & mask);
        st.quotients += 1;
        let before = integer(&t);
        assert_eq!((t[0] + q * pd[0]) & mask, 0);
        let low = (t[0] + mask) / (mask + 1) + (q * pd[0] >> bits);
        for j in 1..5 {
            let z = q * pd[j];
            t[j] += z & MASK;
            t[j + 1] += z >> 52;
        }
        if bits == 52 {
            t[1] += low;
            t = [t[1], t[2], t[3], t[4], t[5], 0];
        } else {
            let old = t;
            t = [
                low + ((old[1] & mask) << 4),
                (old[1] >> 48) + ((old[2] & mask) << 4),
                (old[2] >> 48) + ((old[3] & mask) << 4),
                (old[3] >> 48) + ((old[4] & mask) << 4),
                (old[4] >> 48) + (old[5] << 4),
                0,
            ];
        }
        assert_eq!(integer(&t), (before + N::from(q) * p) >> bits);
        st.postrows += 1;
        let max = *t.iter().max().unwrap();
        assert!(max < 1 << 61);
        st.max_coeff = st.max_coeff.max(max);
    }
    for j in 0..4 {
        t[j + 1] += t[j] >> 52;
        t[j] &= MASK;
    }
    let got = integer(&t[..5]);
    let rb = if mixed { 256usize } else { 260usize };
    let r = N::one() << rb;
    let full = input[0] * input[1] + input[2] * input[3];
    let q = (&full * inv) % &r;
    assert_eq!(got, (&full + q * p) >> rb);
    let out = if mixed && got >= p * 2u32 {
        got - p * 2u32
    } else {
        got
    };
    assert!(out < p * 2u32);
    out
}
fn product(a: [&N; 4], conj: bool, mixed: bool, p: &N, inv: &N, st: &mut Stats) -> [N; 2] {
    let [a0, a1, b0, b1] = a.map(words);
    let iv = words(inv)[0];
    let u = (a0[0] * iv) & MASK;
    let v = (a1[0] * iv) & MASK;
    let mut real = [0u128; 5];
    let mut imag = [0u128; 5];
    for i in 0..5 {
        let (x, y, z) = if conj {
            (
                ((u + v) * b0[i]) & MASK,
                (u * (b0[i] + b1[i])) & MASK,
                (v * (B + b1[i] - b0[i])) & MASK,
            )
        } else {
            (
                (u * b0[i]) & MASK,
                (v * b1[i]) & MASK,
                ((u + v) * (b0[i] + b1[i])) & MASK,
            )
        };
        if conj {
            real[i] = x + z;
            imag[i] = x + 4 * B - y - 2 * b1[i];
            assert!(real[i] < 2 * B && imag[i] < 5 * B);
        } else {
            real[i] = x + 4 * B - y - 2 * b1[i];
            imag[i] = z + 2 * B - x - y;
            assert!(real[i] < 5 * B && imag[i] < 3 * B);
        }
        st.max_partial = st.max_partial.max(real[i]).max(imag[i]);
    }
    let negative = p * 2u32 - if conj { a[0] } else { a[1] };
    let out = if conj {
        [
            reduce([a[0], a[2], a[1], a[3]], &real, mixed, p, inv, st),
            reduce([a[1], a[2], &negative, a[3]], &imag, mixed, p, inv, st),
        ]
    } else {
        [
            reduce([a[0], a[2], &negative, a[3]], &real, mixed, p, inv, st),
            reduce([a[0], a[3], a[1], a[2]], &imag, mixed, p, inv, st),
        ]
    };
    st.pairs += 1;
    out
}
fn main() {
    let mut st = Stats::default();
    let mut fixtures = vec![];
    for hex in [
        "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
        "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
    ] {
        let p = N::parse_bytes(hex.as_bytes(), 16).unwrap();
        let r = N::one() << 260usize;
        let mut inv = N::one();
        for _ in 0..9 {
            inv = (&inv * (&r + 2u32 - ((&p * &inv) % &r))) % &r;
        }
        let inv = &r - inv;
        assert_eq!((&p * &inv + 1u32) % &r, N::zero());
        fixtures.push(json!({"p":hex,"inverse_low52":words(&inv)[0]}));
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
        let mut values = vec![];
        for i in 0..2401usize {
            values.push(core::array::from_fn::<_, 4, _>(|j| {
                edge[(i / 7usize.pow(j as u32)) % 7].clone()
            }));
        }
        let mut seed = 0x640215eaaba7521u64;
        for _ in 0..4096 {
            values.push(core::array::from_fn(|_| {
                let mut bytes = [0; 64];
                for c in bytes.chunks_mut(8) {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    c.copy_from_slice(&seed.to_le_bytes());
                }
                N::from_bytes_le(&bytes) % (&twice + 1u32)
            }));
        }
        for mixed in [false, true] {
            for conj in [false, true] {
                for tuple in &values {
                    product(tuple.each_ref(), conj, mixed, &p, &inv, &mut st);
                }
                let mut a0 = values[2401][0].clone();
                let mut a1 = values[2401][1].clone();
                for i in 0..1024 {
                    let v = &values[(i * 13 + 7) % values.len()];
                    let out = product([&a0, &a1, &v[2], &v[3]], conj, mixed, &p, &inv, &mut st);
                    [a0, a1] = out;
                }
            }
        }
    }
    assert_eq!(st.pairs, 60168);
    assert_eq!(st.quotients, 601680);
    assert_eq!(st.postrows, 601680);
    println!(
        "{}",
        json!({"status":"passed","complex_products":st.pairs,"raw_products":st.pairs*2,"quotient_checks":st.quotients,"postrow_integer_checks":st.postrows,"maximum_quotient_partial":st.max_partial,"maximum_postrow_coefficient":st.max_coeff,"fixtures":fixtures,"scope":"both primes/R256R260/normalandconjugate/7^4edges+4096random+1024chain perconfiguration; exactquotients/postrowinteger/fullREDCrepresentatives"})
    );
}
