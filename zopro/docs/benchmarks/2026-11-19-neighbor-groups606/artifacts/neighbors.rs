// The signed BN254 decomposition and digit extraction follow the verified block study.
type Words = [u64; 4];
const C1: [u64; 3] = [7978627105577135859, 15644699364383830999, 2];
const C2: [u64; 4] = [
    11953552847224317658,
    8825887400277225869,
    5534624963584316111,
    2,
];
const V1X: u128 = 9931322734385697763;
const V1Y_ABS: u128 = 147946756881789319000765030803803410728;
const V2X: u128 = 147946756881789319010696353538189108491;
const V2Y: u128 = 9931322734385697763;
#[inline(always)]
fn product<const N: usize, const M: usize, const K: usize>(a: [u64; N], b: [u64; M]) -> [u64; K] {
    assert_eq!(K, N + M);
    let mut out = [0u64; K];
    for i in 0..N {
        let mut carry = 0u128;
        for j in 0..M {
            let t = a[i] as u128 * b[j] as u128 + out[i + j] as u128 + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        out[i + M] = carry as u64;
    }
    out
}
#[inline(always)]
fn sign_magnitude(value: u128) -> u128 {
    let sign = ((value as i128) >> 127) as u128;
    let m = (value ^ sign).wrapping_sub(sign);
    assert!(m < 1u128 << 126);
    m | (sign & (1u128 << 127))
}
fn phi_split(s: [u64; 4]) -> [u64; 4] {
    let a = product::<2, 3, 5>([s[2], s[3]], C1);
    let b = product::<3, 4, 7>([s[1], s[2], s[3]], C2);
    debug_assert_eq!(a[4], 0);
    debug_assert_eq!(b[6], 0);
    let q1 = a[3] as u128 + u128::from(a[2] >> 63);
    let q2 = (b[4] as u128 | ((b[5] as u128) << 64)) + u128::from(b[3] >> 63);
    let low = s[0] as u128 | ((s[1] as u128) << 64);
    let k1 = low
        .wrapping_sub(q1.wrapping_mul(V1X))
        .wrapping_sub(q2.wrapping_mul(V2X));
    let k2 = q1.wrapping_mul(V1Y_ABS).wrapping_sub(q2.wrapping_mul(V2Y));
    let (a, b) = (sign_magnitude(k1), sign_magnitude(k2));
    [a as u64, (a >> 64) as u64, b as u64, (b >> 64) as u64]
}

fn booth_digit(words: &[u64; 4], w: usize, width: usize) -> i16 {
    if w == 0 {
        let value = words[0] & ((1u64 << width) - 1);
        return (value as i64 - ((value >> (width - 1)) << width) as i64) as i16;
    }
    let offset = w * width - 1;
    let (word, shift) = (offset / 64, offset % 64);
    let mut value = if word < 4 { words[word] >> shift } else { 0 };
    if shift + width + 1 > 64 && word < 3 {
        value |= words[word + 1] << (64 - shift);
    }
    value &= (1u64 << (width + 1)) - 1;
    (((value + 1) >> 1) as i64 - ((value >> width) << width) as i64) as i16
}

use std::cmp::Ordering;
use std::collections::BTreeSet;

const FIELD: Words = [
    4891460686036598785,
    2896914383306846353,
    13281191951274694749,
    3486998266802970665,
];

fn compare(a: &Words, b: &Words) -> Ordering {
    a.iter().rev().cmp(b.iter().rev())
}

fn subtract(a: Words, b: Words) -> (Words, bool) {
    let mut out = [0; 4];
    let mut borrow = false;
    for i in 0..4 {
        let (v, first) = a[i].overflowing_sub(b[i]);
        let (v, second) = v.overflowing_sub(u64::from(borrow));
        out[i] = v;
        borrow = first || second;
    }
    (out, borrow)
}

fn difference(a: Words, b: Words) -> Words {
    let (delta, borrow) = subtract(a, b);
    if !borrow {
        return delta;
    }
    let (magnitude, _) = subtract(b, a);
    subtract(FIELD, magnitude).0
}

fn coordinates(value: Words) -> [i128; 2] {
    let split = phi_split(value);
    std::array::from_fn(|j| {
        let hi = split[2 * j + 1];
        let magnitude = (i128::from(hi & (u64::MAX >> 1)) << 64) | i128::from(split[2 * j]);
        if hi >> 63 == 0 {
            magnitude
        } else {
            -magnitude
        }
    })
}

fn weight(coords: [i128; 2]) -> usize {
    coords
        .into_iter()
        .map(|value| {
            let v = value.unsigned_abs();
            let words = [v as u64, (v >> 64) as u64, 0, 0];
            let mut rebuilt = 0i128;
            let mut count = 0;
            for i in 0..11 {
                let digit = booth_digit(&words, i, 12);
                count += usize::from(digit != 0);
                rebuilt += i128::from(digit) << (i * 12);
            }
            assert_eq!(rebuilt, v as i128);
            count
        })
        .sum()
}

fn spatial_order(coords: [i128; 2]) -> Words {
    let x = (coords[0] + (1i128 << 126)) as u128;
    let y = (coords[1] + (1i128 << 126)) as u128;
    let mut key = [0; 4];
    for i in 0..128 {
        let pair = (((x >> i) & 1) | (((y >> i) & 1) << 1)) as u64;
        key[3 - i / 32] |= pair << (2 * (i % 32));
    }
    key
}

#[derive(Clone)]
struct Entry {
    value: Words,
    coords: [i128; 2],
    spatial: Words,
    cost: usize,
}

// A disjoint pair can replace either scalar by its difference from the other.
// Charge one point addition for forming the pair sum before evaluating its two scalars.
fn saving(a: &Entry, b: &Entry) -> usize {
    let left = a
        .cost
        .saturating_sub(weight(coordinates(difference(a.value, b.value))) + 1);
    let right = b
        .cost
        .saturating_sub(weight(coordinates(difference(b.value, a.value))) + 1);
    left.max(right)
}

fn matching(edges: &[usize]) -> (usize, usize) {
    let mut prev = (0, 0);
    let mut cur = (0, 0);
    for &gain in edges {
        let next = if prev.0 + gain > cur.0 {
            (prev.0 + gain, prev.1 + 1)
        } else {
            cur
        };
        prev = cur;
        cur = next;
    }
    cur
}

fn measure(values: &[Words], tag: &str) {
    let mut entries: Vec<_> = values
        .iter()
        .copied()
        .map(|value| {
            let coords = coordinates(value);
            Entry {
                value,
                coords,
                spatial: spatial_order(coords),
                cost: weight(coords),
            }
        })
        .collect();
    let total: usize = entries.iter().map(|v| v.cost).sum();
    for mode in ["scalar", "first", "second", "spatial"] {
        entries.sort_unstable_by(|a, b| match mode {
            "scalar" => compare(&a.value, &b.value),
            "first" => a.coords.cmp(&b.coords),
            "second" => (a.coords[1], a.coords[0]).cmp(&(b.coords[1], b.coords[0])),
            _ => a.spatial.cmp(&b.spatial),
        });
        let edges: Vec<_> = entries.windows(2).map(|p| saving(&p[0], &p[1])).collect();
        let (net, pairs) = matching(&edges);
        println!("{tag} {mode} {} {total} {net} {pairs}", entries.len());
    }
}

fn controls(n: usize) -> Vec<Words> {
    let mut state = 0xa52b31fa9b643cc1u64;
    let mut distinct = BTreeSet::new();
    while distinct.len() < n {
        let mut v: Words = std::array::from_fn(|_| {
            state = state.wrapping_add(0x9e3779b97f4a7c15);
            let mut x = state;
            x = (x ^ (x >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
            x = (x ^ (x >> 27)).wrapping_mul(0x94d049bb133111eb);
            x ^ (x >> 31)
        });
        v[3] &= u64::MAX >> 2;
        if v != [0; 4] && compare(&v, &FIELD) == Ordering::Less {
            distinct.insert(v);
        }
    }
    distinct.into_iter().collect()
}

fn main() {
    let file = std::env::args_os()
        .nth(1)
        .expect("supply a canonical little-endian B2 capture");
    let data = std::fs::read(file).unwrap();
    assert_eq!(data.len() % 32, 0);
    let mut values = BTreeSet::new();
    for scalar in data.chunks_exact(32) {
        let x = std::array::from_fn(|i| {
            u64::from_le_bytes(scalar[8 * i..8 * i + 8].try_into().unwrap())
        });
        assert_eq!(compare(&x, &FIELD), Ordering::Less);
        if x != [0; 4] {
            values.insert(x);
        }
    }
    let values: Vec<_> = values.into_iter().collect();
    measure(&values, "witness");
    measure(&controls(values.len()), "control");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn disjoint_pairs_and_field_wrap() {
        assert_eq!(matching(&[4, 7, 4]), (8, 2));
        assert_eq!(matching(&[0, 0]), (0, 0));
        let one = [1, 0, 0, 0];
        let neg = subtract(FIELD, one).0;
        assert_eq!(difference([0; 4], one), neg);
        assert_eq!(difference(one, neg), [2, 0, 0, 0]);
        for v in controls(128) {
            assert_eq!(difference(v, difference(v, one)), one);
            assert_eq!(difference(difference(v, one), v), neg);
        }
    }
}
