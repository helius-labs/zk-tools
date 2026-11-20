use std::{env, fs};
use zopro_bn254::Fr;

type Row = [Fr; 2];

fn length(bytes: &[u8]) -> usize {
    u64::from_le_bytes(bytes.try_into().unwrap())
        .try_into()
        .unwrap()
}

fn rows(path: &str) -> Vec<Row> {
    let bytes = fs::read(path).unwrap();
    let count = length(&bytes[..8]);
    assert_eq!(bytes.len(), 8 + count * 64);
    bytes[8..]
        .chunks_exact(64)
        .map(|row| {
            std::array::from_fn(|i| {
                Fr::from_be_bytes(row[32 * i..32 * (i + 1)].try_into().unwrap()).unwrap()
            })
        })
        .collect()
}

fn root(path: &str, n: usize) -> Fr {
    use std::io::Read;
    let mut header = [0u8; 564];
    fs::File::open(path)
        .unwrap()
        .read_exact(&mut header)
        .unwrap();
    assert_eq!(&header[..16], b"zopro-euler-v2\0\0");
    let key = &header[24..];
    assert_eq!(&key[..16], b"zopro-key\0\0\0\0\0\0\0");
    assert_eq!(u32::from_le_bytes(key[16..20].try_into().unwrap()), 3);
    let h_points = u32::from_le_bytes(key[44..48].try_into().unwrap());
    assert_eq!(h_points as usize + 1, n);
    let generator = Fr::from_montgomery(std::array::from_fn(|i| {
        u64::from_le_bytes(key[500 + 8 * i..508 + 8 * i].try_into().unwrap())
    }));
    assert_eq!(generator.pow(&[n as u64]), Fr::ONE);
    assert_eq!(generator.pow(&[n as u64 / 2]), -Fr::ONE);
    generator.inverse().unwrap()
}

#[derive(Default)]
struct Inverse {
    packs: usize,
    zero_diff: usize,
    trivial_diff: usize,
    zero_side: usize,
    trivial_side: usize,
    generic_lanes: usize,
    total_lanes: usize,
    unit_root_lanes: usize,
}

fn trivial(value: Fr) -> bool {
    value == Fr::ZERO || value == Fr::ONE || value == -Fr::ONE
}

fn inverse(input: &[Row], wn: Fr) -> Vec<Row> {
    let n = input.len();
    let mut values = input.to_vec();
    for stage in 0..n.ilog2() as usize {
        let stride = 1usize << stage;
        let gap = n / (2 * stride);
        let step = wn.pow(&[stride as u64]);
        let mut stats = Inverse::default();
        for block in values.chunks_exact_mut(2 * gap) {
            let mut w = Fr::ONE;
            let (left, right) = block.split_at_mut(gap);
            for pack in (0..gap).step_by(4) {
                let mut side_zero = [true; 2];
                let mut side_trivial = [true; 2];
                for i in pack..(pack + 4).min(gap) {
                    for c in 0..2 {
                        let x = left[i][c];
                        let y = right[i][c];
                        let diff = x - y;
                        left[i][c] = x + y;
                        right[i][c] = diff * w;
                        side_zero[c] &= diff == Fr::ZERO;
                        side_trivial[c] &= trivial(diff);
                        stats.generic_lanes += usize::from(w != Fr::ONE && !trivial(diff));
                        stats.unit_root_lanes += usize::from(w == Fr::ONE);
                        stats.total_lanes += 1;
                    }
                    w *= step;
                }
                if gap >= 4 {
                    stats.packs += 1;
                    stats.zero_diff += usize::from(side_zero[0] && side_zero[1]);
                    stats.trivial_diff += usize::from(side_trivial[0] && side_trivial[1]);
                    stats.zero_side += side_zero.into_iter().map(usize::from).sum::<usize>();
                    stats.trivial_side += side_trivial.into_iter().map(usize::from).sum::<usize>();
                }
            }
        }
        assert_eq!(stats.total_lanes, n);
        assert_eq!(stats.unit_root_lanes, 2 * stride);
        println!(
            "inverse {stage} {} {} {} {} {} {} {} {}",
            stats.packs,
            stats.zero_diff,
            stats.trivial_diff,
            stats.zero_side,
            stats.trivial_side,
            stats.generic_lanes,
            stats.total_lanes,
            stats.unit_root_lanes,
        );
    }
    values
}

fn forward_demand(input: &[Row]) {
    let n = input.len();
    // The Euler product asks for D(A) only where B is nonzero, and conversely.
    let mut demand: Vec<_> = input
        .iter()
        .map(|[a, b]| [*b != Fr::ZERO, *a != Fr::ZERO])
        .collect();
    // Traverse the forward DIT graph in reverse, from outputs to derivatives.
    for stage in 0..n.ilog2() as usize {
        let gap = n >> (stage + 1);
        let mut packs = 0;
        let mut empty_packs = 0;
        let mut empty_sides = 0;
        let mut missing_outputs = 0;
        let mut missing_twiddles = 0;
        let mut unit_roots = 0;
        for block in demand.chunks_exact_mut(2 * gap) {
            let (left, right) = block.split_at_mut(gap);
            for pack in (0..gap).step_by(4) {
                let mut side_empty = [true; 2];
                for i in pack..(pack + 4).min(gap) {
                    for c in 0..2 {
                        missing_outputs += usize::from(!left[i][c]) + usize::from(!right[i][c]);
                        let needed = left[i][c] || right[i][c];
                        left[i][c] = needed;
                        right[i][c] = needed;
                        side_empty[c] &= !needed;
                        // A unit-root lane has no nontrivial product to eliminate.
                        missing_twiddles += usize::from(!needed && i != 0);
                        unit_roots += usize::from(i == 0);
                    }
                }
                if gap >= 4 {
                    packs += 1;
                    empty_packs += usize::from(side_empty[0] && side_empty[1]);
                    empty_sides += side_empty.into_iter().map(usize::from).sum::<usize>();
                }
            }
        }
        let mut residue_classes = vec![[false; 2]; gap];
        for (index, [a, b]) in input.iter().enumerate() {
            let residue = &mut residue_classes[index % gap];
            residue[0] |= *b != Fr::ZERO;
            residue[1] |= *a != Fr::ZERO;
        }
        let direct_twiddles = residue_classes[1..]
            .iter()
            .flatten()
            .filter(|&&needed| !needed)
            .count()
            * (n / (2 * gap));
        assert_eq!(missing_twiddles, direct_twiddles);
        assert_eq!(unit_roots, 2 << stage);
        println!("forward {stage} {packs} {empty_packs} {empty_sides} {missing_outputs} {missing_twiddles} {unit_roots}");
    }
}

fn oracle(input: &[Row], dif: &[Row], wn: Fr) {
    let n = input.len();
    for k in [0, 1, 2, 3, n / 32 - 1, n / 32, n / 2 + 1, n - 1] {
        let jump = wn.pow(&[k as u64]);
        let mut factor = Fr::ONE;
        let mut expected = [Fr::ZERO; 2];
        for [a, b] in input {
            expected[0] += *a * factor;
            expected[1] += *b * factor;
            factor *= jump;
        }
        let reversed = k.reverse_bits() >> (usize::BITS - n.ilog2());
        assert_eq!(expected, dif[reversed]);
    }
}

fn main() {
    let args: Vec<_> = env::args().skip(1).collect();
    assert_eq!(args.len(), 2);
    let mut input = rows(&args[0]);
    let count = input.len();
    let n = count.next_power_of_two();
    assert!(n >= 128);
    let wn = root(&args[1], n);
    input.resize(n, [Fr::ZERO; 2]);
    println!("rows {count} {n}");
    let diff = inverse(&input, wn);
    oracle(&input, &diff, wn);
    forward_demand(&input);
    println!("oracle 16");
}
