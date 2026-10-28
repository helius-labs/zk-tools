use super::*;
pub const TILE: usize = 256;
pub fn tiled(a: &mut [Fr], b: &[Fr], values: &[PackedPair]) {
    a.par_chunks_mut(TILE)
        .zip(values.par_chunks(TILE / 4))
        .enumerate()
        .for_each(|(i, (out, values))| old_tile(out, b.get(i * TILE..).unwrap_or(&[]), values));
}
fn old_tile(a: &mut [Fr], b: &[Fr], values: &[PackedPair]) {
    for (i, (out, values)) in a.chunks_mut(8).zip(values.chunks(2)).enumerate() {
        let (da0, db0) = values[0].expand().halves();
        let (da1, db1) = values[1].expand().halves();
        let da = Pair::from_halves(&da0, &da1);
        let db = Pair::from_halves(&db0, &db1);
        let aa = Pair::new(&core::array::from_fn(|j| {
            out.get(j).copied().unwrap_or(Fr::ZERO)
        }));
        let bb = Pair::new(&core::array::from_fn(|j| {
            b.get(8 * i + j).copied().unwrap_or(Fr::ZERO)
        }));
        let result = da.sum2(&bb, &aa, &db).get();
        out.copy_from_slice(&result[..out.len()]);
    }
}
#[cfg(not(accepted_control))]
pub fn compact(a: &mut [Fr], b: &[Fr], values: &[PackedPair]) {
    a.par_chunks_mut(TILE)
        .zip(values.par_chunks(TILE / 4))
        .enumerate()
        .for_each(|(i, (out, values))| compact_tile(out, b.get(i * TILE..).unwrap_or(&[]), values));
}
#[cfg(not(accepted_control))]
fn compact_tile(out: &mut [Fr], b: &[Fr], values: &[PackedPair]) {
    #[cfg(test)]
    let mut written = [0u8; TILE];
    let mut one = [0u16; TILE];
    let mut two = [0u16; TILE];
    let (mut n1, mut n2, mut nz) = (0, 0, 0);
    for (i, &a) in out.iter().enumerate() {
        let bb = b.get(i).copied().unwrap_or(Fr::ZERO);
        let tag = if a == Fr::ZERO {
            if bb == Fr::ZERO {
                nz += 1;
                continue;
            }
            1
        } else if bb == Fr::ZERO {
            2
        } else if a == bb {
            0
        } else {
            two[n2] = i as u16;
            n2 += 1;
            continue;
        };
        one[n1] = i as u16 | tag << 8;
        n1 += 1;
    }
    let old = 150 * out.len().div_ceil(8);
    let new = 100 * n1.div_ceil(8) + 150 * n2.div_ceil(8);
    if new >= old {
        old_tile(out, b, values);
        return;
    }
    if nz != 0 {
        for (i, a) in out.iter_mut().enumerate() {
            if *a == Fr::ZERO && b.get(i).copied().unwrap_or(Fr::ZERO) == Fr::ZERO {
                *a = Fr::ZERO;
                #[cfg(test)]
                {
                    written[i] += 1;
                }
            }
        }
    }
    // Each index enters exactly one category; a row is loaded before its
    // own result is scattered, and no later row reads a different row's A.
    for group in one[..n1].chunks(8) {
        let tags: [u16; 8] = core::array::from_fn(|j| group.get(j).copied().unwrap_or(group[0]));
        let rows = tags.map(|t| (t & 255) as usize);
        let coeff = Pair::new(&core::array::from_fn(|j| {
            if tags[j] >> 8 == 1 {
                b[rows[j]]
            } else {
                out[rows[j]]
            }
        }));
        let d0 = PackedPair::gather_lanes(
            values,
            &core::array::from_fn(|j| {
                rows[j] / 4 * 8 + rows[j] % 4 + if tags[j] >> 8 == 2 { 4 } else { 0 }
            }),
        );
        let mask = (0..8).fold(0u8, |m, j| m | u8::from(tags[j] >> 8 == 0) << j);
        let derivative = if mask == 0 {
            d0
        } else {
            let d1 = PackedPair::gather_lanes(values, &rows.map(|i| i / 4 * 8 + i % 4 + 4));
            d0.add(&if mask == 255 { d1 } else { d1.mask_lanes(mask) })
        };
        let result = coeff.mul(&derivative).get();
        for (j, &t) in group.iter().enumerate() {
            out[(t & 255) as usize] = result[j];
            #[cfg(test)]
            {
                written[(t & 255) as usize] += 1;
            }
        }
    }
    for group in two[..n2].chunks(8) {
        let rows: [usize; 8] =
            core::array::from_fn(|j| group.get(j).copied().unwrap_or(group[0]) as usize);
        let aa = Pair::new(&rows.map(|i| out[i]));
        let bb = Pair::new(&rows.map(|i| b[i]));
        let logical = rows.map(|i| i / 4 * 8 + i % 4);
        let da = PackedPair::gather_lanes(values, &logical);
        let db = PackedPair::gather_lanes(values, &logical.map(|i| i + 4));
        let result = da.sum2(&bb, &aa, &db).get();
        for (j, &i) in group.iter().enumerate() {
            out[i as usize] = result[j];
            #[cfg(test)]
            {
                written[i as usize] += 1;
            }
        }
    }
    #[cfg(test)]
    assert!(written[..out.len()].iter().all(|&n| n == 1));
}
