use std::collections::{BTreeSet, HashMap, HashSet};
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

fn grouping(words: &[Words]) -> Option<Vec<Words>> {
    if words.len() < 256 {
        return None;
    }
    let n = words.len();
    let sample = n.min(2048);
    let stride = n.div_ceil(sample);
    let observed = n.div_ceil(stride).min(sample);
    let unique = words
        .iter()
        .step_by(stride)
        .take(sample)
        .collect::<BTreeSet<_>>();
    if unique.len() * 200 > observed * 199 {
        return None;
    }
    let percent = if (32768..65536).contains(&n) { 95 } else { 80 };
    let zeros = words
        .iter()
        .step_by(stride)
        .take(sample)
        .filter(|s| **s == [0; 4])
        .count();
    let nonzero = observed - zeros;
    if unique.len() - usize::from(zeros != 0) == nonzero && nonzero * 100 > observed * percent {
        return None;
    }
    let mut map = BTreeSet::new();
    let mut values = Vec::new();
    for &w in words {
        if w != [0; 4] && map.insert(w) {
            values.push(w);
        }
    }
    if values.len() * 100 > n * percent {
        None
    } else {
        Some(values)
    }
}

type Digits = [i16; 11];
fn digits(split: &[u64; 4], component: usize) -> Digits {
    let limbs = &split[2 * component..];
    let magnitude = (limbs[0] as u128) | (((limbs[1] & (u64::MAX >> 1)) as u128) << 64);
    let negative = limbs[1] >> 63 != 0;
    let words = [magnitude as u64, (magnitude >> 64) as u64, 0, 0];
    let result = std::array::from_fn(|window| {
        let d = booth_digit(&words, window, 12);
        if negative {
            -d
        } else {
            d
        }
    });
    let reconstructed = result
        .iter()
        .rev()
        .fold(0i128, |a, &d| a * 4096 + i128::from(d));
    assert_eq!(
        reconstructed,
        if negative {
            -(magnitude as i128)
        } else {
            magnitude as i128
        }
    );
    result
}

fn savings(rows: &[Digits], start: usize, end: usize) -> (usize, usize) {
    let mut counts = HashMap::<Digits, usize>::new();
    for row in rows {
        let digits = &row[start..end];
        if digits.iter().filter(|&&v| v != 0).take(2).count() < 2 {
            continue;
        }
        let sign = digits.iter().find(|&&x| x != 0).unwrap().signum();
        let mut key = [0; 11];
        for (out, &x) in key.iter_mut().zip(digits) {
            *out = x * sign;
        }
        *counts.entry(key).or_default() += 1;
    }
    let mut gains: Vec<_> = counts
        .into_iter()
        .filter(|(_, n)| *n > 1)
        .map(|(pattern, n)| (n - 1) * (pattern.iter().filter(|&&v| v != 0).count() - 1))
        .collect();
    gains.sort_unstable_by(|a, b| b.cmp(a));
    (gains.iter().sum(), gains.iter().take(16).sum())
}

fn evaluate(rows: &[Digits]) -> (usize, usize, usize, usize) {
    let adds = (0..11)
        .map(|w| {
            let mut bins = HashSet::new();
            let mut active = 0usize;
            for row in rows {
                if row[w] != 0 {
                    active += 1;
                    bins.insert(row[w].unsigned_abs());
                }
            }
            active - bins.len()
        })
        .sum();
    let mut best = [0; 12];
    let mut limited = [0; 12];
    let mut whole = 0;
    for end in 1..=11 {
        best[end] = best[end - 1];
        limited[end] = limited[end - 1];
        for start in 0..end - 1 {
            let (all, top) = savings(rows, start, end);
            best[end] = best[end].max(best[start] + all);
            limited[end] = limited[end].max(limited[start] + top);
            if start == 0 && end == 11 {
                whole = all;
            }
        }
    }
    assert!(whole <= best[11] && limited[11] <= best[11] && best[11] <= adds);
    (adds, whole, best[11], limited[11])
}

fn main() {
    let argv: Vec<_> = std::env::args().collect();
    let bytes = std::fs::read(&argv[1]).unwrap();
    assert_eq!(bytes.len(), 30089 * 32);
    let words: Vec<Words> = bytes.chunks_exact(32).map(|raw| {
        std::array::from_fn(|i| u64::from_le_bytes(raw[8*i..8*i+8].try_into().unwrap()))
    }).collect();
    let grouped=grouping(&words);
    let effective=grouped.as_deref().unwrap_or(&words);
    let parts:Vec<_>=effective.iter().copied().map(phi_split).collect();
    for c in 0..2 {
        let rows:Vec<_>=parts.iter().map(|s|digits(s,c)).collect();
        let (adds, _, unrestricted, _) = evaluate(&rows);
        println!("base {c} {adds} {unrestricted} {}", rows.len());
        for tile in [128,1024,4096,16384] {
            let mut gains=[[(0usize,0usize);12];11];
            for range in rows.chunks(rows.len().div_ceil(2)) {
                for chunk in range.chunks(tile) {
                    for end in 2..=11 {for start in 0..end-1 {
                        let (all,top)=savings(chunk,start,end);
                        gains[start][end].0+=all;
                        gains[start][end].1+=top;
                    }}
                }
            }
            for end in 2..=11 {for start in 0..end-1 {
                let (all,top)=gains[start][end];
                println!("gain {c} {tile} {start} {end} {all} {top}");
            }}
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn signed_patterns_save_additions_only_for_shared_nonzero_digits() {
        let mut p = [0; 11];
        p[..4].copy_from_slice(&[2, -3, 0, 7]);
        let mut other = p;
        other[3] = 9;
        let mut one = [0; 11];
        one[0] = 5;
        let rows = [p, p, p.map(|x| -x), other, one, one];
        assert_eq!(savings(&rows, 0, 4), (4, 4));
        assert_eq!(savings(&rows, 0, 2), (3, 3));
        assert_eq!(savings(&rows, 2, 4), (0, 0));
        assert_eq!(evaluate(&rows).1, 4);
    }
}
