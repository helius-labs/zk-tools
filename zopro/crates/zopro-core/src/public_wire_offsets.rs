//! Optional fixed public offsets shared across the A, private K and B2 queries.
//! The file is borrowed only during preparation. Corrections are derived from
//! the current key so each revised scalar sum equals its original for every witness.
use super::{solver::Plan, words, Fr, G1Affine, G1Projective, G2Affine, G2Projective, Key};
use crate::preparation::Reservation;
use rayon::prelude::*;
use sha2::{Digest, Sha256};
use std::mem::size_of;

const HEADER: usize = 48;
const ROW: usize = 36;
const WINDOWS: usize = 64;
const BUCKETS: usize = 15;

struct Row {
    wire: usize,
    scalar: Fr,
}

pub(super) struct Offsets {
    rows: Vec<Row>,
    pub a: G1Projective,
    pub k: G1Projective,
    pub b2: G2Projective,
    _reservation: Reservation,
}

impl Offsets {
    pub fn restore_public(&self, public: &mut [Fr]) {
        for row in &self.rows {
            let Some(value) = public.get_mut(row.wire) else {
                // The solver plan orders B2 indices by wire.
                break;
            };
            *value += row.scalar;
        }
    }

    pub fn apply(&self, wires: &mut [Fr]) {
        for row in &self.rows {
            wires[row.wire] -= row.scalar;
        }
    }
}

#[derive(Clone, Copy)]
struct Positions {
    a: Option<usize>,
    k: Option<usize>,
    b2: usize,
}

fn u32_le(bytes: &[u8]) -> usize {
    u32::from_le_bytes(bytes.try_into().unwrap()) as usize
}

fn entries(data: &[u8], n: usize) -> Result<&[u8], &'static str> {
    if data.len() < HEADER || &data[..8] != b"ZPWO0001" {
        return Err("invalid public wire offset header");
    }
    let count = u32_le(&data[44..48]);
    let entries = &data[HEADER..];
    if u32_le(&data[40..44]) != n
        || count == 0
        || count > n
        || count.checked_mul(ROW) != Some(entries.len())
    {
        return Err("invalid public wire offset dimensions");
    }
    Ok(entries)
}

fn map_row(row: &[u8], plan: &Plan, private: usize) -> Result<(Positions, Row), &'static str> {
    let b2 = u32_le(&row[..4]);
    let wire = *plan
        .b_indices
        .get(b2)
        .ok_or("public wire offset index exceeds B2")?;
    let scalar = Fr::from_canonical(words(&row[4..])).ok_or("noncanonical public wire offset")?;
    let k = (wire as usize).checked_sub(plan.n_public);
    if scalar == Fr::ZERO || k.is_some_and(|i| i >= private) {
        return Err("invalid public wire offset row");
    }
    Ok((
        Positions {
            a: plan.a_indices.binary_search(&wire).ok(),
            k,
            b2,
        },
        Row {
            wire: wire as usize,
            scalar,
        },
    ))
}

type Sums = (G1Projective, G1Projective, G2Projective);
const ZERO: Sums = (
    G1Projective::IDENTITY,
    G1Projective::IDENTITY,
    G2Projective::IDENTITY,
);

fn correction(
    records: &[u8],
    positions: &[Positions],
    a: &[G1Affine],
    k: &[G1Affine],
    b2: &[G2Affine],
) -> Sums {
    let mut windows = [ZERO; WINDOWS];
    windows
        .par_iter_mut()
        .enumerate()
        .for_each(|(window, sum)| {
            let mut buckets = [ZERO; BUCKETS];
            for (row, pos) in records.chunks_exact(ROW).zip(positions) {
                let digit = (row[4 + window / 2] >> (4 * (window % 2))) & 15;
                if digit != 0 {
                    let bucket = &mut buckets[usize::from(digit) - 1];
                    if let Some(i) = pos.a {
                        bucket.0 += a[i];
                    }
                    if let Some(i) = pos.k {
                        bucket.1 += k[i];
                    }
                    bucket.2 += b2[pos.b2];
                }
            }
            let mut running = ZERO;
            for bucket in buckets.iter().rev() {
                running.0 += bucket.0;
                running.1 += bucket.1;
                running.2 += bucket.2;
                sum.0 += running.0;
                sum.1 += running.1;
                sum.2 += running.2;
            }
        });
    let mut total = ZERO;
    for sum in windows.iter().rev() {
        for _ in 0..4 {
            total = (total.0.double(), total.1.double(), total.2.double());
        }
        total.0 += sum.0;
        total.1 += sum.1;
        total.2 += sum.2;
    }
    total
}

pub(super) fn prepare(key: &Key, data: &[u8]) -> Result<Option<Offsets>, &'static str> {
    let records = entries(data, key.b2.len())?;
    let count = records.len() / ROW;
    let Some(plan) = key.solver.as_ref() else {
        return Ok(None);
    };
    let Some(folded) = key.lagrange.as_ref().and_then(|q| q.folded.as_ref()) else {
        return Ok(None);
    };
    if !super::affine_msm()
        || key.equal_witness.is_some()
        || key.has_gpu()
        || key.a.len() != plan.a_indices.len()
        || key.b2.len() != plan.b_indices.len()
    {
        return Ok(None);
    }
    let held = count
        .checked_mul(size_of::<Row>())
        .and_then(|n| n.checked_add(size_of::<Offsets>()));
    let Some(reservation) = Reservation::analysis(held) else {
        return Ok(None);
    };
    let leaves = rayon::current_num_threads().min(WINDOWS);
    let windows = (WINDOWS + 8 + leaves * (BUCKETS + 8)) * size_of::<Sums>();
    let scratch = count
        .checked_mul(size_of::<Positions>())
        .and_then(|n| n.checked_add(windows))
        .and_then(|n| n.checked_add(data.len()));
    let Some(_temporary) = Reservation::analysis(scratch) else {
        return Ok(None);
    };
    let mut digest = Sha256::new();
    for p in &key.b2[..] {
        for coord in [p.x.c0, p.x.c1, p.y.c0, p.y.c1] {
            for limb in coord.0 {
                digest.update(limb.to_le_bytes());
            }
        }
    }
    if digest.finalize()[..] != data[8..40] {
        return Err("public wire offset B2 digest mismatch");
    }
    let mut rows = Vec::with_capacity(count);
    let mut positions = Vec::with_capacity(count);
    let mut previous = None;
    for row in records.chunks_exact(ROW) {
        let (p, r) = map_row(row, plan, folded.private.len())?;
        if previous.is_some_and(|i| p.b2 <= i) {
            return Err("public wire offset indices must increase");
        }
        previous = Some(p.b2);
        rows.push(r);
        positions.push(p);
    }
    let (a, k, b2) = correction(records, &positions, &key.a, &folded.private, &key.b2);
    Ok(Some(Offsets {
        rows,
        a,
        k,
        b2,
        _reservation: reservation,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn corrections_match_direct_scalar_multiplication() {
        let a = [G1Affine::GENERATOR, -G1Affine::GENERATOR];
        let k = [-G1Affine::GENERATOR, G1Affine::GENERATOR];
        let b2 = [
            G2Affine::GENERATOR,
            -G2Affine::GENERATOR,
            G2Affine::GENERATOR,
        ];
        let positions = [
            Positions {
                a: Some(0),
                k: None,
                b2: 0,
            },
            Positions {
                a: None,
                k: Some(0),
                b2: 1,
            },
            Positions {
                a: Some(1),
                k: Some(1),
                b2: 2,
            },
        ];
        let scalars = [Fr::ONE, -Fr::ONE, Fr::from(13u64).pow(&[47])];
        let mut records = Vec::new();
        let mut expected = ZERO;
        for (position, scalar) in positions.iter().zip(scalars) {
            records.extend_from_slice(&(position.b2 as u32).to_le_bytes());
            for limb in scalar.to_canonical() {
                records.extend_from_slice(&limb.to_le_bytes());
            }
            if let Some(i) = position.a {
                expected.0 += a[i] * scalar;
            }
            if let Some(i) = position.k {
                expected.1 += k[i] * scalar;
            }
            expected.2 += b2[position.b2] * scalar;
        }
        for workers in [1, 4] {
            let pool = rayon::ThreadPoolBuilder::new()
                .num_threads(workers)
                .build()
                .unwrap();
            assert_eq!(
                pool.install(|| correction(&records, &positions, &a, &k, &b2)),
                expected
            );
        }
    }

    #[test]
    fn header_dimensions_must_match_exactly() {
        let mut file = vec![0; HEADER + ROW];
        file[..8].copy_from_slice(b"ZPWO0001");
        file[40..44].copy_from_slice(&2u32.to_le_bytes());
        file[44..48].copy_from_slice(&1u32.to_le_bytes());
        assert_eq!(entries(&file, 2).unwrap().len(), ROW);
        assert!(entries(&file, 3).is_err());
        assert!(entries(&file[..HEADER - 1], 2).is_err());
        file.push(0);
        assert!(entries(&file, 2).is_err());
    }
}
