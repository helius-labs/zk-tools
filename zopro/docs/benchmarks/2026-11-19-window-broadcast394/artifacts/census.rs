use std::{array, env, fs};

type Word = [u64; 4];

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

fn prod_digit(s: &[u64; 4], high: bool, w: usize, bits: usize) -> i16 {
    let i = usize::from(high) * 2;
    {
        let d = booth_digit(&[s[i], s[i + 1] & 0x7fffffffffffffff, 0, 0], w, bits);
        if s[i + 1] >> 63 != 0 {
            -d
        } else {
            d
        }
    }
}

fn split_value(words: &Word, high: bool) -> i128 {
    let i = usize::from(high) * 2;
    let magnitude = words[i] as i128 | (((words[i + 1] & (u64::MAX >> 1)) as i128) << 64);
    if words[i + 1] >> 63 == 0 {
        magnitude
    } else {
        -magnitude
    }
}

fn signed_digit(value: i128, window: usize, width: usize) -> i16 {
    let magnitude = value.unsigned_abs();
    let offset = window * width;
    let radix = 1u128 << width;
    let carry_in = if offset == 0 {
        0
    } else {
        (magnitude >> (offset - 1)) & 1
    };
    let low = (magnitude >> offset) & (radix - 1);
    let result = ((low + carry_in) as i32 - if low >= radix / 2 { radix as i32 } else { 0 }) as i16;
    if value < 0 {
        -result
    } else {
        result
    }
}

#[derive(Default)]
struct Stats {
    regular_updates: usize,
    selected_updates: usize,
    packets: usize,
    batches: Vec<usize>,
    fallbacks: usize,
}

fn finish(pending: &mut usize, occupied: &mut [Vec<bool>], stats: &mut Stats) {
    if *pending != 0 {
        stats.batches.push(*pending);
        *pending = 0;
        for row in occupied {
            row.fill(false);
        }
    }
}

// An exact digit census with an optimistic structural model of bucket occupancy.
// The first nonzero write counts as initialization; cancellation and queue stalls
// are omitted. Window lanes have disjoint storage and may have independent signs.
fn simulate(input: &[(Word, bool)], width: usize, parts: usize, capacity: usize) -> Stats {
    let mut stats = Stats::default();
    let windows = 127usize.div_ceil(width);
    assert!(windows > 8 && 126 - (windows - 1) * width <= 8);
    let buckets = 1 << (width - 1);
    for part in input.chunks(input.len().div_ceil(parts)) {
        let mut seen = vec![vec![false; buckets]; 8];
        let mut occupied = vec![vec![false; buckets]; 8];
        for high in [true, false] {
            let mut pending = 0;
            for (words, finite) in part {
                if !finite {
                    continue;
                }
                let scalar = split_value(words, high);
                let mut reconstruction = 0i128;
                let mut digits = [0i16; 8];
                for window in 0..windows {
                    let digit = signed_digit(scalar, window, width);
                    assert_eq!(digit, prod_digit(words, high, window, width));
                    reconstruction += (digit as i128) << (window * width);
                    if window < 8 {
                        digits[window] = digit;
                    }
                    if window + 1 < windows && digit != 0 {
                        stats.regular_updates += 1;
                        stats.selected_updates += usize::from(window < 8);
                    }
                }
                assert_eq!(reconstruction, scalar);
                let indices = digits.map(|v| (v.unsigned_abs() as usize).checked_sub(1));
                let all_ready = indices
                    .iter()
                    .enumerate()
                    .all(|(lane, i)| i.is_some_and(|b| seen[lane][b]));
                let conflict = indices
                    .iter()
                    .enumerate()
                    .any(|(lane, i)| i.is_some_and(|b| occupied[lane][b]));
                if conflict || !all_ready {
                    finish(&mut pending, &mut occupied, &mut stats);
                }
                for (lane, i) in indices.into_iter().enumerate() {
                    if let Some(b) = i {
                        seen[lane][b] = true;
                        if all_ready {
                            occupied[lane][b] = true;
                        }
                    }
                }
                if all_ready {
                    stats.packets += 1;
                    pending += 1;
                    if pending == capacity {
                        finish(&mut pending, &mut occupied, &mut stats);
                    }
                } else {
                    stats.fallbacks += digits.iter().filter(|&&v| v != 0).count();
                }
            }
            finish(&mut pending, &mut occupied, &mut stats);
        }
    }
    assert_eq!(stats.batches.iter().sum::<usize>(), stats.packets);
    assert_eq!(stats.packets * 8 + stats.fallbacks, stats.selected_updates);
    stats
}

fn main() {
    let data = fs::read(env::args_os().nth(1).expect("query fixture")).unwrap();
    assert_eq!(&data[..8], b"B2SCAN01");
    let n = u64::from_le_bytes(data[8..16].try_into().unwrap()) as usize;
    assert_eq!(data.len(), 16 + 160 * n);
    let end = 16 + 128 * n;
    let mut entries = Vec::with_capacity(n);
    for (p, s) in data[16..end]
        .chunks_exact(128)
        .zip(data[end..].chunks_exact(32))
    {
        let words: Word =
            array::from_fn(|i| u64::from_le_bytes(s[i * 8..i * 8 + 8].try_into().unwrap()));
        entries.push((phi_split(words), p.iter().any(|&v| v != 0)));
    }
    for (width, parts) in [(12, 2), (11, 4)] {
        for capacity in [8, 16, 32] {
            let mut s = simulate(&entries, width, parts, capacity);
            s.batches.sort_unstable();
            let large = s
                .batches
                .iter()
                .copied()
                .filter(|&n| n >= 16)
                .sum::<usize>();
            let median = s.batches.get(s.batches.len() / 2).copied().unwrap_or(0);
            println!(
                "{width} {parts} {capacity} {} {} {} {} {} {median} {large}",
                s.regular_updates,
                s.selected_updates,
                s.packets,
                s.fallbacks,
                s.batches.len()
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn zero_and_identity_leave_counters_empty() {
        let input = vec![([0; 4], true), ([u64::MAX; 4], false)];
        let s = simulate(&input, 12, 2, 16);
        assert_eq!((s.regular_updates, s.packets, s.batches.len()), (0, 0, 0));
    }
    #[test]
    fn repeated_digits_force_separate_batches() {
        let a = 0x6006006006006006u64;
        let entries = vec![([a, a >> 3, a, a >> 3], true); 160];
        let s = simulate(&entries, 12, 2, 32);
        assert!(s.packets > 0 && s.batches.iter().all(|&v| v == 1));
    }
}
