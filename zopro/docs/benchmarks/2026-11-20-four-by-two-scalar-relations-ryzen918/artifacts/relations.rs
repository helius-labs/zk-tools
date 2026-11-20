use std::{collections::BTreeMap, env, fs, path::Path};
use zopro_bn254::Fr;

type Words = [u64; 4];

#[derive(Clone, Copy, Debug)]
struct Relation {
    parent: usize,
    child: usize,
    exponent: u8,
    negative: bool,
    offset: i8,
    digits: usize,
    cost: usize,
}

impl Relation {
    fn saving(self) -> usize {
        self.digits - self.cost
    }

    fn scale(self, value: Fr) -> Fr {
        let scaled = value * Fr::from(1 << self.exponent);
        if self.negative { -scaled } else { scaled }
    }
}

fn offset(value: i8) -> Fr {
    let magnitude = Fr::from(u64::from(value.unsigned_abs()));
    if value < 0 { -magnitude } else { magnitude }
}

fn read(path: &Path) -> Vec<Fr> {
    let data = fs::read(path).unwrap();
    assert_eq!(&data[..8], b"ZPS91800");
    let word = |bytes: &[u8]| u64::from_le_bytes(bytes.try_into().unwrap());
    let n = usize::try_from(word(&data[8..16])).unwrap();
    assert_eq!(data.len(), 16 + 32 * n);
    data[16..]
        .chunks_exact(32)
        .map(|row| {
            let field = Fr::from_montgomery(std::array::from_fn(|j| word(&row[8 * j..8 * j + 8])));
            assert_eq!(Fr::from_canonical(field.to_canonical()), Some(field));
            field
        })
        .collect()
}

fn digit_count(words: Words, width: usize, signed: bool) -> usize {
    let bits = 254 + usize::from(signed);
    let bit = |i: usize| words.get(i / 64).map_or(0, |&word| (word >> (i % 64)) & 1);
    (0..bits.div_ceil(width))
        .filter(|&w| {
            let start = w * width;
            let mut digit = (0..width).map(|j| bit(start + j) << j).sum::<u64>();
            if signed {
                // Booth recoding uses the sign bit from the preceding window.
                let carry_in = if start == 0 { 0 } else { bit(start - 1) };
                let carry_out = bit(start + width - 1);
                digit = (digit + carry_in).wrapping_sub(carry_out << width);
            }
            digit != 0
        })
        .count()
}

fn enumerate(values: &[Fr], digits: &[usize], max_exp: u8, max_offset: i8) -> Vec<Relation> {
    let table: BTreeMap<Words, usize> = values.iter().enumerate().map(|(i, v)| (v.0, i)).collect();
    let offsets: Vec<_> = (-max_offset..=max_offset).map(|d| (d, offset(d))).collect();
    let mut edges = Vec::new();
    for (parent, value) in values.iter().enumerate() {
        let mut scaled = *value;
        for exponent in 0..=max_exp {
            for negative in [false, true] {
                let base = if negative { -scaled } else { scaled };
                for &(amount, shift) in &offsets {
                    if let Some(&child) = table.get(&(base + shift).0) {
                        if child == parent {
                            continue;
                        }
                        // One point merge, exponent doublings, and one
                        // correction combination (plus a double for |d|=2).
                        let cost = 1
                            + usize::from(exponent)
                            + usize::from(amount != 0)
                            + usize::from(amount.unsigned_abs() == 2);
                        if digits[child] > cost {
                            let edge = Relation {
                                parent,
                                child,
                                exponent,
                                negative,
                                offset: amount,
                                digits: digits[child],
                                cost,
                            };
                            assert_eq!(values[child], edge.scale(*value) + shift);
                            edges.push(edge);
                        }
                    }
                }
            }
            scaled = scaled.double();
        }
    }
    edges.sort_unstable_by_key(|edge| {
        (
            std::cmp::Reverse(edge.saving()),
            edge.parent,
            edge.child,
            edge.exponent,
            edge.negative,
            edge.offset,
        )
    });
    edges
}

fn select(n: usize, edges: &[Relation]) -> (Vec<Relation>, usize) {
    let mut claimed = vec![false; n];
    let mut best = vec![0; n];
    let mut chosen = Vec::new();
    for &edge in edges {
        best[edge.child] = best[edge.child].max(edge.saving());
        if !claimed[edge.child] && !claimed[edge.parent] {
            claimed[edge.child] = true;
            claimed[edge.parent] = true;
            chosen.push(edge);
        }
    }
    (chosen, best.into_iter().sum())
}

fn verify(values: &[Fr], chosen: &[Relation]) {
    let weights: Vec<_> = (0..values.len())
        .map(|i| Fr::from((i as u64 + 1).pow(3)))
        .collect();
    let reference: Fr = values
        .iter()
        .zip(&weights)
        .fold(Fr::ZERO, |s, (v, w)| s + *v * *w);
    let mut updated = weights.clone();
    let mut correction = Fr::ZERO;
    for &r in chosen {
        let x = weights[r.child];
        updated[r.parent] += r.scale(x);
        updated[r.child] = Fr::ZERO;
        correction += offset(r.offset) * x;
    }
    let rebuilt = values
        .iter()
        .zip(updated)
        .fold(correction, |s, (v, w)| s + *v * w);
    assert_eq!(reference, rebuilt);
}

fn main() {
    let arg: Vec<_> = env::args_os().collect();
    assert_eq!(arg.len(), 3);
    let query = arg[1].to_str().unwrap();
    let (width, signed) = match query {
        "k" => (12, true),
        "b2" => (11, false),
        _ => panic!("query"),
    };
    let input = read(Path::new(&arg[2]));
    let mut counts = BTreeMap::new();
    for value in &input {
        if *value != Fr::ZERO {
            *counts.entry(value.to_canonical()).or_insert(0usize) += 1;
        }
    }
    let values: Vec<_> = counts
        .keys()
        .map(|&value| Fr::from_canonical(value).unwrap())
        .collect();
    let digits: Vec<_> = counts
        .keys()
        .map(|&value| digit_count(value, width, signed))
        .collect();
    let arrivals: usize = digits.iter().sum();
    println!(
        "input\t{query}\t{}\t{}\t{}\t{width}\t{signed}\t{arrivals}",
        input.len(),
        counts.values().sum::<usize>(),
        values.len()
    );
    for (label, exp, delta) in [
        ("sign", 0, 0),
        ("dyadic", 3, 0),
        ("offset", 0, 2),
        ("combined", 3, 2),
    ] {
        let edges = enumerate(&values, &digits, exp, delta);
        let (chosen, bound) = select(values.len(), &edges);
        verify(&values, &chosen);
        let removed: usize = chosen.iter().map(|edge| edge.digits).sum();
        let cost: usize = chosen.iter().map(|edge| edge.cost).sum();
        let high = chosen.iter().filter(|r| r.digits >= 8).count();
        println!(
            "model\t{label}\t{}\t{}\t{removed}\t{cost}\t{}\t{bound}\t{high}",
            edges.len(),
            chosen.len(),
            removed - cost
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn affine_aliases_match_exact_field_relation_and_weighted_sum() {
        let x = Fr::from(659);
        let values = [x, x.double() * Fr::from(4) + Fr::ONE, -x, Fr::from(1119)];
        let edges = enumerate(&values, &[21, 22, 21, 21], 3, 2);
        assert!(
            edges
                .iter()
                .any(|r| r.parent == 0 && r.child == 1 && r.exponent == 3 && r.offset == 1)
        );
        assert!(
            edges
                .iter()
                .any(|r| r.parent == 0 && r.child == 2 && r.negative && r.offset == 0)
        );
        let (chosen, bound) = select(values.len(), &edges);
        assert!(!chosen.is_empty());
        assert!(chosen.iter().map(|r| r.saving()).sum::<usize>() <= bound);
        verify(&values, &chosen);
    }

    #[test]
    fn digit_counts_match_independent_integer_windows() {
        for i in 0..8192u64 {
            let value = i * 16127721;
            for width in [11, 12] {
                let m = (1 << width) - 1;
                let mut left = value;
                let mut unsigned = 0;
                let mut signed = 0;
                let mut carry = 0;
                while left != 0 || carry != 0 {
                    let raw = left & m;
                    unsigned += usize::from(raw != 0);
                    let next = (raw >> (width - 1)) & 1;
                    signed += usize::from(raw + carry != (next << width));
                    carry = next;
                    left >>= width;
                }
                assert_eq!(digit_count([value, 0, 0, 0], width, false), unsigned);
                assert_eq!(digit_count([value, 0, 0, 0], width, true), signed);
            }
        }
    }
}
