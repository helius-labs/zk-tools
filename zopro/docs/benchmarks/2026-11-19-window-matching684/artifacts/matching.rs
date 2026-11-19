//! Compute and verify a disjoint two-window grouping plan for signed GLV components.
use std::collections::{HashMap, HashSet};
use std::{env, fs};

const PRIME: i128 = (1 << 61) - 1;
type Key = (i16, i16);

fn digits(x: i128, width: usize) -> Vec<i16> {
    let mag = x.unsigned_abs();
    assert!(mag < 1u128 << 126);
    let radix = 1i32 << width;
    let ds: Vec<_> = (0..126)
        .step_by(width)
        .map(|offset| {
            let bits = ((mag << 1) >> offset) as i32 & (2 * radix - 1);
            (x.signum() as i32 * (((bits + 1) >> 1) - ((bits >> width) * radix))) as i16
        })
        .collect();
    assert_eq!(
        ds.iter()
            .rev()
            .fold(0i128, |v, d| v * i128::from(radix) + i128::from(*d)),
        x
    );
    ds
}

fn canon(a: i16, b: i16) -> Option<(Key, i16)> {
    if a == 0 || b == 0 {
        return None;
    }
    let sign = a.signum();
    Some(((a * sign, b * sign), sign))
}

// At most one pair per window. Skipping a window has zero savings.
fn matching(gains: &[Vec<usize>]) -> (usize, Vec<(usize, usize)>) {
    let count = 1usize << gains.len();
    let mut score = vec![0; count];
    let mut partner = vec![None; count];
    for mask in 1..count {
        let i = mask.trailing_zeros() as usize;
        let rest = mask ^ (1 << i);
        score[mask] = score[rest];
        for j in i + 1..gains.len() {
            if rest & (1 << j) == 0 {
                continue;
            }
            let v = gains[i][j] + score[rest ^ (1 << j)];
            if v > score[mask] {
                score[mask] = v;
                partner[mask] = Some(j);
            }
        }
    }
    let mut mask = count - 1;
    let mut pairs = Vec::new();
    while mask != 0 {
        let i = mask.trailing_zeros() as usize;
        let j = partner[mask];
        mask ^= 1 << i;
        if let Some(j) = j {
            pairs.push((i, j));
            mask ^= 1 << j;
        }
    }
    (score[count - 1], pairs)
}

fn add(buckets: &mut [i128], digit: i16, point: i128) {
    if digit != 0 {
        let slot = &mut buckets[digit.unsigned_abs() as usize];
        *slot = (*slot + i128::from(digit.signum()) * point).rem_euclid(PRIME);
    }
}

fn run(scalars: &[i128], width: usize, tile: usize) {
    let rows: Vec<_> = scalars.iter().map(|s| digits(*s, width)).collect();
    let windows = rows[0].len();
    let mut gains = vec![vec![0usize; windows]; windows];
    let mut hash = HashSet::new();
    let mut max_keys = 0;
    for (i, line) in gains.iter_mut().enumerate() {
        for (j, value) in line.iter_mut().enumerate().skip(i + 1) {
            for chunk in rows.chunks(tile) {
                hash.clear();
                for ds in chunk {
                    if let Some((key, _)) = canon(ds[i], ds[j]) {
                        *value += usize::from(!hash.insert(key));
                    }
                }
                max_keys = max_keys.max(hash.len());
            }
        }
    }
    let (saved, pairs) = matching(&gains);
    let buckets = (1 << (width - 1)) + 1;
    let mut ordinary = vec![vec![0i128; buckets]; windows];
    let mut occupied = vec![vec![false; buckets]; windows];
    let mut arrivals = 0;
    let weight = |i: usize| ((i as i128 + 127).pow(4) + 211).rem_euclid(PRIME);
    for (i, ds) in rows.iter().enumerate() {
        for (w, &d) in ds.iter().enumerate() {
            if d != 0 {
                arrivals += 1;
                occupied[w][d.unsigned_abs() as usize] = true;
            }
            add(&mut ordinary[w], d, weight(i));
        }
    }
    let mut rebuilt = vec![vec![0i128; buckets]; windows];
    let mut actual_saved = 0;
    for &(a, b) in &pairs {
        for (c, chunk) in rows.chunks(tile).enumerate() {
            let mut sums = HashMap::new();
            for (k, ds) in chunk.iter().enumerate() {
                let p = weight(c * tile + k);
                if let Some((key, sign)) = canon(ds[a], ds[b]) {
                    let sum = sums.entry(key).or_insert((0i128, 0usize));
                    sum.0 = (sum.0 + i128::from(sign) * p).rem_euclid(PRIME);
                    sum.1 += 1;
                } else {
                    add(&mut rebuilt[a], ds[a], p);
                    add(&mut rebuilt[b], ds[b], p);
                }
            }
            for ((da, db), (sum, n)) in sums {
                add(&mut rebuilt[a], da, sum);
                add(&mut rebuilt[b], db, sum);
                actual_saved += n - 1;
            }
        }
        assert_eq!(rebuilt[a], ordinary[a]);
        assert_eq!(rebuilt[b], ordinary[b]);
    }
    assert_eq!(actual_saved, saved);
    let seeds: usize = occupied.iter().flatten().filter(|b| **b).count();
    // Descending Pippenger scan, skipping empty buckets when extending the running sum.
    let reductions: usize = occupied
        .iter()
        .map(|v| {
            let n = v.iter().filter(|b| **b).count();
            n.saturating_sub(1) + v.iter().rposition(|b| *b).unwrap_or(0).saturating_sub(1)
        })
        .sum();
    println!("{width} {tile} {arrivals} {seeds} {reductions} {saved} {max_keys} {pairs:?}");
}

fn main() {
    let bytes = fs::read(env::args().nth(1).expect("signed component file")).unwrap();
    assert!(!bytes.is_empty() && bytes.len() % 32 == 0);
    for component in [0, 1] {
        let scalars: Vec<_> = bytes
            .chunks_exact(32)
            .map(|r| {
                i128::from_le_bytes(r[component * 16..component * 16 + 16].try_into().unwrap())
            })
            .collect();
        for width in [10, 11, 12] {
            for tile in [256, 4096, scalars.len()] {
                run(&scalars, width, tile);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn best_matching_can_pass_over_the_adjacent_window() {
        let gains = vec![
            vec![0, 5, 6, 0],
            vec![0, 0, 0, 7],
            vec![0, 0, 0, 5],
            vec![0, 0, 0, 0],
        ];
        assert_eq!(matching(&gains), (13, vec![(0, 2), (1, 3)]));
    }

    #[test]
    fn signs_zeros_and_radix_boundaries() {
        assert_eq!(canon(-3, 4), Some(((3, -4), -1)));
        assert_eq!(canon(3, -4), Some(((3, -4), 1)));
        assert_eq!(canon(0, -4), None);
        for width in [10, 11, 12] {
            for x in [
                0,
                1,
                (1 << (width - 1)) - 1,
                1 << (width - 1),
                (1i128 << 126) - 1,
            ] {
                let d = digits(x, width);
                assert_eq!(digits(-x, width), d.iter().map(|k| -*k).collect::<Vec<_>>());
            }
        }
    }
}
