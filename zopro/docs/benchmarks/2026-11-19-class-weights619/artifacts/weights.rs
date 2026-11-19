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


use std::{env, fs};

fn digits(value: Words, glv: bool) -> Vec<i16> {
    if !glv {
        return (0..22).map(|i| booth_digit(&value, i, 12)).collect();
    }
    let split = phi_split(value);
    [0, 2].into_iter().flat_map(|i| {
        let words = [split[i], split[i + 1] & !(1u64 << 63), 0, 0];
        (0..11).map(move |j| booth_digit(&words, j, 12))
    }).collect()
}

fn copies(values: &[Words], counts: &[usize], target: usize) -> Vec<usize> {
    let mut capacity = [0; 2];
    let wide = |v: &Words| usize::from(v[2] != 0 || v[3] != 0);
    for (v, &n) in values.iter().zip(counts) { capacity[wide(v)] += n - 1; }
    let extra = target - values.len();
    let quota = [extra.saturating_sub(capacity[1]), extra.min(capacity[1])];
    let mut allocated = [0; 2];
    let mut seen = [0; 2];
    values.iter().zip(counts).map(|(v, &n)| {
        let k = wide(v);
        seen[k] += n - 1;
        let desired = if capacity[k] == 0 { 0 } else {
            (seen[k] as u128 * quota[k] as u128 / capacity[k] as u128) as usize
        };
        let parts = 1 + desired - allocated[k];
        assert!(parts <= n);
        allocated[k] = desired;
        parts
    }).collect()
}

fn main() {
    let args: Vec<_> = env::args().collect();
    let raw = fs::read(&args[1]).unwrap();
    let counts_raw = fs::read(&args[2]).unwrap();
    let target: usize = args[3].parse().unwrap();
    let expected_nonzero: usize = args[4].parse().unwrap();
    assert_eq!(raw.len() % 32, 0);
    let values: Vec<Words> = raw.chunks_exact(32).map(|v| std::array::from_fn(|i|
        u64::from_le_bytes(v[8 * i..8 * (i + 1)].try_into().unwrap()))).collect();
    assert_eq!(values.iter().collect::<std::collections::HashSet<_>>().len(), values.len());
    let field = [4891460686036598785,2896914383306846353,13281191951274694749,3486998266802970665u64];
    assert!(values.iter().all(|v| v != &[0; 4] && v.iter().rev().cmp(field.iter().rev()).is_lt()));
    assert_eq!(counts_raw.len(), values.len() * 8);
    let counts: Vec<usize> = counts_raw.chunks_exact(8).map(|v| u64::from_le_bytes(v.try_into().unwrap()) as usize).collect();
    assert!(counts.iter().all(|&n| n > 0));
    assert_eq!(counts.iter().sum::<usize>(), expected_nonzero);
    let copies = copies(&values, &counts, target);
    let positions: Vec<usize> = copies.iter().enumerate().flat_map(|(i, &n)| std::iter::repeat_n(i,n)).collect();
    assert_eq!(positions.len(), target);
    let wide = positions.iter().filter(|&&i| values[i][2] != 0 || values[i][3] != 0).count();
    for (name, glv) in [("canonical", false), ("glv", true)] {
        let windows: Vec<_> = values.iter().map(|&v| digits(v,glv)).collect();
        for (order, stride) in [("ordinary", 1), ("striped", 32)] {
            let mut arrivals = 0u64;
            let mut adjacent = 0u64;
            let mut expected = 0.0;
            let mut bins: Vec<_> = (0..22).map(|_| vec![0u64; 2049]).collect();
            let mut previous = [0usize; 22];
            for i in (0..stride).flat_map(|s| (s..target).step_by(stride)) {
                for (w, d) in windows[positions[i]].iter().enumerate() {
                    let b = usize::from(d.unsigned_abs());
                    if b == 0 { continue; }
                    adjacent += u64::from(previous[w] == b);
                    arrivals += 1;
                    bins[w][b] += 1;
                    previous[w] = b;
                }
            }
            for group in &bins {
                let n: u64 = group.iter().sum();
                if n > 0 {
                    expected += group.iter().filter(|&&c| c > 0).map(|c| c*(c-1)).sum::<u64>() as f64 / n as f64;
                }
            }
            println!("{name},{order},{target},{wide},{},{arrivals},{adjacent},{expected:.3}",copies.iter().max().unwrap());
        }
    }
}
