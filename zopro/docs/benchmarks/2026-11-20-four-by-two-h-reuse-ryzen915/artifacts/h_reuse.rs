use std::{env, fs, path::Path};

const FULL: usize = 21;
const WINDOWS: usize = FULL + 1;
const WIDTH: usize = 12;
type Digits = [u16; WINDOWS];

fn read_words(path: &Path) -> (usize, Vec<Option<Digits>>) {
    let data = fs::read(path).unwrap();
    assert_eq!(&data[..8], b"ZPH91500");
    let word = |bytes: &[u8]| u64::from_le_bytes(bytes.try_into().unwrap());
    assert_eq!(word(&data[8..16]), WIDTH as u64);
    let parts = usize::try_from(word(&data[16..24])).unwrap();
    let count = usize::try_from(word(&data[24..32])).unwrap();
    assert_eq!(data.len(), 32 + 33 * count);
    let rows = data[32..]
        .chunks_exact(33)
        .map(|row| {
            let limbs: [u64; 4] = std::array::from_fn(|i| word(&row[1 + i * 8..9 + i * 8]));
            assert!(row[0] <= 1 && limbs[3] >> 62 == 0);
            let mut reconstructed = [0; 4];
            let digits = std::array::from_fn(|window| {
                let offset = window * WIDTH;
                let (limb, shift) = (offset / 64, offset % 64);
                let mut value = limbs[limb] >> shift;
                if shift + WIDTH > 64 && limb + 1 < limbs.len() {
                    value |= limbs[limb + 1] << (64 - shift);
                }
                value &= (1 << WIDTH) - 1;
                reconstructed[limb] |= value << shift;
                if shift + WIDTH > 64 && limb + 1 < limbs.len() {
                    reconstructed[limb + 1] |= value >> (64 - shift);
                }
                value as u16
            });
            assert_eq!(reconstructed, limbs);
            (row[0] == 1).then_some(digits)
        })
        .collect();
    (parts, rows)
}

#[derive(Default, Debug)]
struct Work {
    full_arrivals: usize,
    top_arrivals: usize,
    full_occupied: usize,
    top_occupied: usize,
}

fn work(rows: &[Digits]) -> (Work, Vec<bool>) {
    let mut work = Work::default();
    let mut buckets = vec![false; (1 << WIDTH) * WINDOWS];
    for row in rows {
        for (w, &d) in row.iter().enumerate() {
            if d == 0 {
                continue;
            }
            let used = &mut buckets[w * (1 << WIDTH) + usize::from(d)];
            if w == FULL {
                work.top_arrivals += 1;
                work.top_occupied += usize::from(!*used);
            } else {
                work.full_arrivals += 1;
                work.full_occupied += usize::from(!*used);
            }
            *used = true;
        }
    }
    (work, buckets)
}

fn column_pairs() -> impl Iterator<Item = (usize, usize)> {
    (1..WINDOWS).flat_map(|b| (0..b).map(move |a| (a, b)))
}

fn classes(rows: &[Digits], used: &[bool], a: usize, b: usize, entries: &mut Vec<(u32, usize)>) {
    entries.clear();
    entries.extend(
        rows.iter()
            .zip(used)
            .enumerate()
            .filter_map(|(i, (row, &used))| {
                let (x, y) = (u32::from(row[a]), u32::from(row[b]));
                (!used && x != 0 && y != 0).then_some(((x << WIDTH) | y, i))
            }),
    );
    entries.sort_unstable();
}

// A group whose members have equal digits in k full windows needs at most
// (m - 1)(k - 1) fewer additions after formation, and contributes at least
// that many tickets across the k(k - 1)/2 pairs of columns. Each occupied
// digit-pair class can fund at most class_size - 1 tickets for disjoint groups.
fn bound(rows: &[Digits]) -> (usize, usize) {
    let mut full_tickets = 0;
    let mut top_eligible = vec![false; rows.len()];
    let used = vec![false; rows.len()];
    let mut entries = Vec::new();
    for (a, b) in column_pairs() {
        classes(rows, &used, a, b, &mut entries);
        for group in entries.chunk_by(|x, y| x.0 == y.0) {
            if b == FULL {
                if group.len() >= 2 {
                    for &(_, i) in group {
                        top_eligible[i] = true;
                    }
                }
            } else {
                full_tickets += group.len() - 1;
            }
        }
    }
    let top = top_eligible.iter().filter(|&&flag| flag).count() / 2;
    (full_tickets, top)
}

#[derive(Clone, Copy, Default, Debug)]
struct Round {
    formations: usize,
    full_avoided: usize,
    top_avoided: usize,
}

impl Round {
    fn net(self) -> usize {
        self.full_avoided + self.top_avoided - self.formations
    }

    fn add(&mut self, other: Self) {
        self.formations += other.formations;
        self.full_avoided += other.full_avoided;
        self.top_avoided += other.top_avoided;
    }
}

struct Formations {
    rows: Vec<Digits>,
    inputs: Vec<(usize, usize)>,
}

impl Formations {
    fn new(original: &[Digits]) -> Self {
        Self {
            rows: original.to_vec(),
            inputs: Vec::new(),
        }
    }

    fn round(&mut self, include_top: bool) -> Round {
        let end = self.rows.len();
        let mut used = vec![false; end];
        let mut entries = Vec::new();
        let mut results = Round::default();
        for (a, b) in column_pairs() {
            if b == FULL && !include_top {
                continue;
            }
            classes(&self.rows[..end], &used, a, b, &mut entries);
            for group in entries.chunk_by(|x, y| x.0 == y.0) {
                for pair in group.chunks_exact(2) {
                    let (left, right) = (pair[0].1, pair[1].1);
                    assert!(!used[left] && !used[right]);
                    used[left] = true;
                    used[right] = true;
                    let mut combined = [0; WINDOWS];
                    for (w, dest) in combined.iter_mut().enumerate() {
                        if self.rows[left][w] != 0 && self.rows[left][w] == self.rows[right][w] {
                            *dest = self.rows[left][w];
                            self.rows[left][w] = 0;
                            self.rows[right][w] = 0;
                            if w == FULL {
                                results.top_avoided += 1;
                            } else {
                                results.full_avoided += 1;
                            }
                        }
                    }
                    assert!(combined[a] != 0 && combined[b] != 0);
                    self.rows.push(combined);
                    self.inputs.push((left, right));
                    results.formations += 1;
                }
            }
        }
        results
    }

    fn validate(&self, original: &[Digits]) {
        // Replay synthetic rows backwards, expanding each point sum back to
        // its inputs. Every coefficient of every original point must return.
        let mut restored = self.rows.clone();
        for (i, &(left, right)) in self.inputs.iter().enumerate().rev() {
            let output = original.len() + i;
            for w in 0..WINDOWS {
                let value = restored[output][w];
                if value != 0 {
                    assert_eq!(restored[left][w], 0);
                    assert_eq!(restored[right][w], 0);
                    restored[left][w] = value;
                    restored[right][w] = value;
                }
            }
        }
        assert_eq!(&restored[..original.len()], original);
    }
}

fn analyze(parts: usize, input: &[Option<Digits>], include_top: bool) {
    let mut total_work = Work::default();
    let mut total_bound = (0, 0);
    let mut rounds = [Round::default(); 3];
    let mut formed_points = 0;
    for range in input.chunks(input.len().div_ceil(parts)) {
        let original: Vec<_> = range.iter().copied().flatten().collect();
        let (before, occupancy) = work(&original);
        total_work.full_arrivals += before.full_arrivals;
        total_work.top_arrivals += before.top_arrivals;
        total_work.full_occupied += before.full_occupied;
        total_work.top_occupied += before.top_occupied;
        let (full, top) = bound(&original);
        total_bound.0 += full;
        total_bound.1 += top;
        let mut state = Formations::new(&original);
        let mut cumulative = Round::default();
        for (i, aggregate) in rounds.iter_mut().enumerate() {
            let step = state.round(include_top);
            if i == 0 {
                let upper = if include_top {
                    full + top
                } else {
                    full + step.top_avoided
                };
                assert!(step.net() <= upper);
            }
            cumulative.add(step);
            aggregate.add(step);
            let (after, current) = work(&state.rows);
            assert_eq!(occupancy, current);
            assert_eq!(
                before.full_arrivals - after.full_arrivals,
                cumulative.full_avoided
            );
            assert_eq!(
                before.top_arrivals - after.top_arrivals,
                cumulative.top_avoided
            );
            state.validate(&original);
        }
        formed_points += cumulative.formations;
    }
    println!(
        "parts\t{parts}\t{include_top}\t{}\t{}\t{}\t{}\t{}\t{}\t{formed_points}",
        total_work.full_arrivals,
        total_work.top_arrivals,
        total_work.full_occupied,
        total_work.top_occupied,
        total_bound.0,
        total_bound.1
    );
    for (i, result) in rounds.iter().enumerate() {
        println!(
            "round\t{}\t{}\t{}\t{}\t{}",
            i + 1,
            result.formations,
            result.full_avoided,
            result.top_avoided,
            result.net()
        );
    }
}

fn main() {
    let source = env::args_os().nth(1).expect("capture path");
    let (parts, rows) = read_words(Path::new(&source));
    assert_eq!(parts, 2);
    println!("inputs\t{}\t{}", rows.len(), rows.iter().flatten().count());
    analyze(parts, &rows, false);
    analyze(parts, &rows, true);
    analyze(1, &rows, true);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeated_pair_conserves_tokens_across_rounds() {
        let original = [
            [4, 3, 0, 2],
            [4, 3, 1, 0],
            [4, 3, 0, 2],
            [4, 3, 1, 0],
            [8, 7, 0, 2],
            [8, 7, 1, 0],
            [0, 0, 1, 0],
            [0, 0, 0, 0],
        ]
        .map(|small| {
            let mut all = [0; WINDOWS];
            all[..4].copy_from_slice(&small);
            all
        });
        let mut state = Formations::new(&original);
        let early = state.round(false);
        assert_eq!((early.formations, early.net()), (3, 3));
        let next = state.round(false);
        assert_eq!((next.formations, next.net()), (1, 1));
        assert_eq!(work(&original).1, work(&state.rows).1);
        state.validate(&original);
    }

    #[test]
    fn disjoint_bound_covers_every_tiny_pairing() {
        let pairings = [[(0, 1), (2, 3)], [(0, 2), (1, 3)], [(0, 3), (1, 2)]];
        for mask in 0u32..(1 << 12) {
            let mut rows = [[0; WINDOWS]; 4];
            for (r, row) in rows.iter_mut().enumerate() {
                for (j, w) in [0, 1, FULL].into_iter().enumerate() {
                    row[w] = ((mask >> (3 * r + j)) & 1) as u16;
                }
            }
            let (full, top) = bound(&rows);
            for pairing in pairings {
                let saved: usize = pairing
                    .into_iter()
                    .map(|(a, b)| {
                        rows[a]
                            .iter()
                            .zip(rows[b])
                            .filter(|(x, y)| **x != 0 && **x == *y)
                            .count()
                            .saturating_sub(1)
                    })
                    .sum();
                assert!(saved <= full + top);
            }
        }
    }
}
