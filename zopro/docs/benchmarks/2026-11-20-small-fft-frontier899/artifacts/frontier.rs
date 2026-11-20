use std::{env, fs};

const NO_WRITER: usize = usize::MAX;
const UNREADY: usize = usize::MAX - 1;
const BANDS: usize = 4;

struct Inputs {
    rows: Vec<[usize; 2]>,
    op_depth: Vec<usize>,
    operation_rows: Vec<Option<usize>>,
    domain: usize,
}

fn image(path: &str) -> Inputs {
    let bytes = fs::read(path).expect("read solver image");
    let start = bytes
        .windows(4)
        .rposition(|w| w == b"ZSP1")
        .expect("solver marker");
    assert_eq!(
        u64::from_le_bytes(bytes[start - 8..start].try_into().unwrap()) as usize,
        bytes.len() - start
    );
    assert_eq!((bytes.len() - start) % 4, 0);
    let data: Vec<usize> = bytes[start..]
        .chunks_exact(4)
        .map(|b| u32::from_le_bytes(b.try_into().unwrap()) as usize)
        .collect();
    let [magic, version, wires, inputs, _, rows, a, b, coefficients, terms, sums, slots, count] =
        data[..13].try_into().unwrap();
    assert_eq!((magic, version), (0x3150535a, 2));
    let term_start = 13 + a + b + 8 * coefficients;
    let sum_start = term_start + 2 * terms;
    let mut pos = sum_start + 3 * sums;
    let mut writer = vec![UNREADY; wires + slots];
    writer[..inputs].fill(NO_WRITER);
    let mut op_depth = Vec::with_capacity(count);
    let mut operation_rows = Vec::with_capacity(count);
    let mut output = vec![[UNREADY; 2]; rows];

    fn read(writers: &[usize], w: usize, reads: &mut Vec<usize>) {
        let op = writers[w];
        assert_ne!(op, UNREADY, "read before writer for {w}");
        if op != NO_WRITER {
            reads.push(op);
        }
    }

    for op in 0..count {
        let mut reads = Vec::new();
        let mut writes = Vec::new();
        let mut sides = [Vec::new(), Vec::new(), Vec::new()];
        let mut row = None;
        if data[pos] == 1 {
            let [_, kind, lo, hi, start, end] = data[pos..pos + 6].try_into().unwrap();
            assert!(kind <= 1);
            for term in lo..hi {
                read(&writer, data[term_start + 2 * term + 1], &mut reads);
            }
            writes.extend(start..end);
            pos += 6;
        } else {
            assert_eq!(data[pos], 0);
            let [index, lo, hi, first_slot] = data[pos + 1..pos + 5].try_into().unwrap();
            row = Some(index);
            let mut slot = first_slot;
            // Expand only the current row's sum chain. Earlier stored sums are
            // treated as atomic at their original writer's band.
            let mut chain = Vec::new();
            let mut local_sums: Vec<Vec<usize>> = Vec::new();
            for sum in lo..hi {
                let [previous, _, wire] = data[sum_start + 3 * sum..sum_start + 3 * sum + 3]
                    .try_into()
                    .unwrap();
                let source = previous & 0x7fff_ffff;
                match source {
                    0x7fff_ffff => chain.clear(),
                    0x7fff_fffe => {}
                    _ => {
                        chain.clear();
                        if source >= wires + first_slot && source < wires + slot {
                            chain.extend_from_slice(&local_sums[source - wires - first_slot]);
                        } else {
                            read(&writer, source, &mut chain);
                        }
                    }
                }
                read(&writer, wire, &mut chain);
                reads.extend_from_slice(&chain);
                if previous >> 31 != 0 {
                    writes.push(wires + slot);
                    local_sums.push(chain.clone());
                    slot += 1;
                }
            }
            for (side, out) in sides.iter_mut().enumerate() {
                let p = pos + 5 + 3 * side;
                match data[p] {
                    0 | 2 => {}
                    1 => {
                        let source = data[p + 1];
                        if source >= wires + first_slot && source < wires + slot {
                            out.extend_from_slice(&local_sums[source - wires - first_slot]);
                        } else {
                            read(&writer, source, out);
                        }
                    }
                    3 | 4 => {
                        let start = data[p + 1];
                        let end = if data[p] == 3 { start + 1 } else { data[p + 2] };
                        for term in start..end {
                            read(&writer, data[term_start + 2 * term + 1], out);
                        }
                    }
                    value => panic!("unknown expression tag {value}"),
                }
                reads.extend_from_slice(out);
            }
            if data[pos + 14] == 1 {
                let assigned = data[pos + 15];
                writes.push(data[pos + 16]);
                if assigned < 2 {
                    // When the other factor is zero, the solver retains the
                    // original expression for the assigned side.
                    sides[assigned] = sides.iter().flatten().copied().collect();
                } else {
                    assert_eq!(assigned, 2);
                }
                pos += 26;
            } else {
                assert_eq!(data[pos + 14], 0);
                pos += 15;
            }
        }
        let latest = |reads: &[usize]| {
            reads
                .iter()
                .max_by_key(|&&i| op_depth[i])
                .copied()
                .unwrap_or(NO_WRITER)
        };
        let dep = latest(&reads);
        let depth = if dep == NO_WRITER {
            0
        } else {
            op_depth[dep] + 1
        };
        if let Some(index) = row {
            assert_eq!(output[index], [UNREADY; 2]);
            output[index] = [latest(&sides[0]), latest(&sides[1])];
        }
        for w in writes {
            assert_eq!(writer[w], UNREADY);
            writer[w] = op;
        }
        operation_rows.push(row);
        op_depth.push(depth);
    }
    assert_eq!(pos, data.len());
    assert!(output.iter().all(|p| !p.contains(&UNREADY)));
    Inputs {
        rows: output,
        op_depth,
        operation_rows,
        domain: rows.next_power_of_two(),
    }
}

fn bands(input: &Inputs, packed: bool) -> (Vec<usize>, usize) {
    let levels = input.op_depth.iter().max().unwrap() + 1;
    let mut count = vec![0; levels];
    for depth in &input.op_depth {
        count[*depth] += 1;
    }
    let mut seen = 0;
    let mut at = Vec::with_capacity(levels);
    let work = input.op_depth.len();
    for (d, n) in count.into_iter().enumerate() {
        let index = if packed {
            BANDS * (2 * work * d + (2 * seen + n) * levels) / (4 * work * levels)
        } else {
            BANDS * d / levels
        };
        assert!(index < BANDS);
        at.push(index + 1);
        seen += n;
    }
    (input.op_depth.iter().map(|&i| at[i]).collect(), levels)
}

fn summary(shape: &str, source: &str, side: &str, form: &str, level: usize, ready: &[usize]) {
    let count: [usize; BANDS] =
        std::array::from_fn(|band| ready.iter().filter(|&&at| at <= band + 1).count());
    println!(
        "{shape},{source},{side},{form},{level},{},{},{},{},{}",
        ready.len(),
        count[0],
        count[1],
        count[2],
        count[3]
    );
    assert_eq!(count[3], ready.len());
}

fn main() {
    let mut args = env::args().skip(1);
    let shape = args.next().expect("shape label");
    let path = args.next().expect("input image");
    let packed = match args.next().as_deref() {
        Some("packed") => true,
        Some("unpacked") => false,
        _ => panic!("expected packed or unpacked"),
    };
    assert!(args.next().is_none());
    let input = image(&path);
    let n = input.domain;
    let (when, depth_levels) = bands(&input, packed);
    let mut rows = vec![0; n];
    for (op, row) in input.operation_rows.iter().enumerate() {
        if let Some(row) = row {
            rows[*row] = when[op];
        }
    }
    let mut early = vec![[0; 2]; n];
    for (row, pair) in input.rows.iter().enumerate() {
        for side in 0..2 {
            let op = pair[side];
            early[row][side] = if op == NO_WRITER { 0 } else { when[op] };
            assert!(early[row][side] <= rows[row]);
        }
    }
    eprintln!("{shape},{},{n},{depth_levels},{packed}", input.rows.len());
    summary(&shape, "published", "paired", "rows", 0, &rows);
    for (side, name) in ["a", "b", "paired"].into_iter().enumerate() {
        let ready: Vec<usize> = early
            .iter()
            .map(|e| if side < 2 { e[side] } else { e[0].max(e[1]) })
            .collect();
        summary(&shape, "rematerialized", name, "rows", 0, &ready);
        summary(
            &shape,
            "rematerialized",
            name,
            "real-rows",
            0,
            &ready[..input.rows.len()],
        );
        let earlier = ready
            .iter()
            .zip(&rows)
            .filter(|(new, old)| new < old)
            .count();
        eprintln!("{shape},{name},{earlier}");
    }

    for (source, side, mut ready) in [
        ("published", "paired", rows),
        (
            "rematerialized",
            "paired",
            early.iter().map(|e| e[0].max(e[1])).collect(),
        ),
        ("rematerialized", "a", early.iter().map(|e| e[0]).collect()),
        ("rematerialized", "b", early.iter().map(|e| e[1]).collect()),
    ] {
        for level in 1..=5 {
            let width = n >> level;
            let mut ready_pairs = Vec::with_capacity(n / 2);
            let mut ready_packets = Vec::with_capacity(n / 8);
            for stripe in ready.chunks_exact_mut(2 * width) {
                let (top, bottom) = stripe.split_at_mut(width);
                for (a, b) in top.iter_mut().zip(bottom) {
                    *a = (*a).max(*b);
                    *b = *a;
                }
                ready_pairs.extend_from_slice(top);
                ready_packets.extend(
                    top.chunks_exact(4)
                        .map(|group| *group.iter().max().unwrap()),
                );
            }
            summary(&shape, source, side, "butterflies", level, &ready_pairs);
            summary(&shape, source, side, "packets", level, &ready_packets);
            let across_stripes: Vec<usize> = (0..width / 4)
                .map(|column| {
                    ready_packets
                        .chunks_exact(width / 4)
                        .map(|stripe| stripe[column])
                        .max()
                        .unwrap()
                })
                .collect();
            for tile in [1, 16] {
                let tiles: Vec<usize> = across_stripes
                    .chunks_exact(tile)
                    .map(|columns| *columns.iter().max().unwrap())
                    .collect();
                summary(
                    &shape,
                    source,
                    side,
                    if tile == 1 { "columns" } else { "tiles16" },
                    level,
                    &tiles,
                );
            }
        }
    }
}
