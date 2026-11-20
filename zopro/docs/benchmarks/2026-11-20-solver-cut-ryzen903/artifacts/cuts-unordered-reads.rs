use std::{env, fs};

const NO_WRITER: usize = usize::MAX;
const UNREADY: usize = usize::MAX - 1;

struct Inputs {
    rows: Vec<[usize; 2]>,
    op_depth: Vec<usize>,
    operation_rows: Vec<Option<usize>>,
    domain: usize,
    dependencies: Vec<Vec<usize>>,
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
    let mut dependencies = Vec::with_capacity(count);
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
        reads.sort_unstable();
        reads.dedup();
        dependencies.push(reads);
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
        dependencies,
    }
}

const LOCAL_BANDS: usize = 8;
const REFINE: usize = 4096;
const PACK_LIMIT: usize = 128;

fn root(parent: &mut [usize], mut id: usize) -> usize {
    while parent[id] != id {
        parent[id] = parent[parent[id]];
        id = parent[id];
    }
    id
}

struct Groups {
    positions: Vec<usize>,
    parent: Vec<usize>,
    grouped: Vec<usize>,
}

impl Groups {
    fn new(n: usize) -> Self {
        Self {
            positions: vec![UNREADY; n],
            parent: Vec::new(),
            grouped: Vec::new(),
        }
    }

    fn make(&mut self, graph: &Inputs, ops: &[usize], bands: &[usize]) -> Vec<Vec<usize>> {
        assert_eq!(ops.len(), bands.len());
        for (idx, &op) in ops.iter().enumerate() {
            assert_eq!(self.positions[op], UNREADY);
            self.positions[op] = idx;
        }
        self.parent.clear();
        self.parent.extend(0..ops.len());
        for (idx, &op) in ops.iter().enumerate() {
            for &dep in &graph.dependencies[op] {
                let j = self.positions[dep];
                if j < idx && bands[j] == bands[idx] {
                    let a = root(&mut self.parent, idx);
                    let b = root(&mut self.parent, j);
                    self.parent[a] = b;
                }
            }
        }
        self.grouped.clear();
        self.grouped.extend(0..ops.len());
        for i in 0..ops.len() {
            self.parent[i] = root(&mut self.parent, i);
        }
        self.grouped
            .sort_unstable_by_key(|&i| (bands[i], self.parent[i], i));
        let mut tasks: Vec<Vec<usize>> = Vec::new();
        let mut begin = 0;
        let mut previous_band = UNREADY;
        while begin < self.grouped.len() {
            let i = self.grouped[begin];
            let band = bands[i];
            let component = self.parent[i];
            let mut end = begin + 1;
            while end < self.grouped.len() && self.parent[self.grouped[end]] == component {
                end += 1;
            }
            let size = end - begin;
            let fits = band == previous_band
                && tasks
                    .last()
                    .is_some_and(|t| t.len().saturating_add(size) <= PACK_LIMIT);
            if !fits {
                tasks.push(Vec::new());
            }
            tasks
                .last_mut()
                .unwrap()
                .extend(self.grouped[begin..end].iter().map(|&i| ops[i]));
            begin = end;
            previous_band = band;
        }
        for &op in ops {
            self.positions[op] = UNREADY;
        }
        tasks
    }

    fn local_span(&mut self, graph: &Inputs, ops: &[usize]) -> (usize, usize) {
        for (i, &op) in ops.iter().enumerate() {
            self.positions[op] = i;
        }
        let mut depth = Vec::with_capacity(ops.len());
        for (idx, &op) in ops.iter().enumerate() {
            depth.push(
                graph.dependencies[op]
                    .iter()
                    .map(|&p| self.positions[p])
                    .filter(|&j| j < idx)
                    .map(|j| depth[j] + 1)
                    .max()
                    .unwrap_or(0),
            );
        }
        for &op in ops {
            self.positions[op] = UNREADY;
        }
        let deepest = depth.iter().max().unwrap() + 1;
        let bands = LOCAL_BANDS.min(deepest);
        let by_depth: Vec<_> = depth.iter().map(|&n| n * bands / deepest).collect();
        let leafs = self.make(graph, ops, &by_depth);
        let mut critical = [0; LOCAL_BANDS];
        for leaf in &leafs {
            let first = ops.binary_search(&leaf[0]).unwrap();
            let band = by_depth[first];
            critical[band] = critical[band].max(leaf.len());
        }
        (critical.into_iter().sum(), leafs.len())
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct Estimate {
    span: usize,
    tasks: usize,
    ops: usize,
}

struct Evaluator<'a> {
    graph: &'a Inputs,
    grouping: Groups,
    refinement: Groups,
}

impl Evaluator<'_> {
    fn band(&mut self, low: usize, high: usize) -> Estimate {
        let ops: Vec<_> = self
            .graph
            .op_depth
            .iter()
            .enumerate()
            .filter_map(|(i, &depth)| (low <= depth && depth < high).then_some(i))
            .collect();
        let mut result = Estimate {
            ops: ops.len(),
            ..Estimate::default()
        };
        for mut group in self.grouping.make(self.graph, &ops, &vec![0; ops.len()]) {
            let (span, tasks) = if group.len() >= REFINE {
                // A group this large is one weak component. The sort also
                // makes the original operation order explicit to the model.
                group.sort_unstable();
                self.refinement.local_span(self.graph, &group)
            } else {
                (group.len(), 1)
            };
            result.span = result.span.max(span);
            result.tasks += tasks;
        }
        result
    }
}

fn total(e: &[Estimate]) -> usize {
    e.iter().map(|v| v.span).sum()
}

fn print_plan(name: &str, breaks: &[usize], e: &[Estimate]) {
    print!("{name}");
    for &cut in &breaks[1..breaks.len() - 1] {
        print!(",{cut}");
    }
    print!(",{}", total(e));
    for band in e {
        print!(",{}:{}:{}", band.span, band.ops, band.tasks);
    }
    println!();
}

fn ordinary(graph: &Inputs, levels: usize) -> Vec<usize> {
    let n = graph.op_depth.len();
    let mut freq = vec![0; levels];
    for &level in &graph.op_depth {
        freq[level] += 1;
    }
    let mut previous = 0;
    let mut seen = 0;
    let mut result = vec![0];
    for (level, count) in freq.into_iter().enumerate() {
        let band = ((2 * n * level + (2 * seen + count) * levels) / (n * levels)).min(3);
        if band != previous {
            assert_eq!(band, previous + 1);
            result.push(level);
            previous = band;
        }
        seen += count;
    }
    result.push(levels);
    assert_eq!(result.len(), 5);
    result
}

fn main() {
    let args: Vec<_> = env::args().collect();
    assert_eq!(args.len(), 2);
    let g = image(&args[1]);
    let depth = g.op_depth.iter().max().unwrap() + 1;
    eprintln!(
        "{} rows, {} domain, {} row operations",
        g.rows.len(),
        g.domain,
        g.operation_rows.iter().flatten().count()
    );
    let baseline = ordinary(&g, depth);
    let mut candidates: Vec<_> = (0..=32)
        .map(|i| i * depth / 32)
        .chain(baseline.iter().copied())
        .collect();
    candidates.sort_unstable();
    candidates.dedup();
    let size = candidates.len();
    let mut eval = Evaluator {
        graph: &g,
        grouping: Groups::new(g.op_depth.len()),
        refinement: Groups::new(g.op_depth.len()),
    };
    let mut score = vec![vec![Estimate::default(); size]; size];
    for i in 0..size {
        for j in i + 1..size {
            score[i][j] = eval.band(candidates[i], candidates[j]);
        }
    }
    let indices: Vec<_> = baseline
        .iter()
        .map(|c| candidates.binary_search(c).unwrap())
        .collect();
    let base_cost: Vec<_> = indices.windows(2).map(|x| score[x[0]][x[1]]).collect();
    print_plan("ordinary", &baseline, &base_cost);

    // Enumerate a frozen cut grid. Limit to four nonempty bands. The
    // balanced variant also excludes extremely small or large work bands.
    for balanced in [false, true] {
        let mut best = (usize::MAX, [0; 5]);
        for i in 1..size - 3 {
            for j in i + 1..size - 2 {
                for k in j + 1..size - 1 {
                    let cut = [0, i, j, k, size - 1];
                    let costs: Vec<_> = cut.windows(2).map(|x| score[x[0]][x[1]]).collect();
                    if costs.iter().any(|x| x.ops == 0) {
                        continue;
                    }
                    let n = g.op_depth.len();
                    if balanced && costs.iter().any(|x| x.ops * 10 < n || x.ops * 20 > n * 9) {
                        continue;
                    }
                    let value = total(&costs);
                    if value < best.0 {
                        best = (value, cut);
                    }
                }
            }
        }
        let cut = best.1;
        assert_eq!(cut[4], size - 1);
        let depths = cut.map(|x| candidates[x]);
        let measures: Vec<_> = cut.windows(2).map(|x| score[x[0]][x[1]]).collect();
        assert_eq!(
            measures.iter().map(|x| x.ops).sum::<usize>(),
            g.op_depth.len()
        );
        print_plan(
            if balanced { "balanced" } else { "span" },
            &depths,
            &measures,
        );
    }

    // Each DP state ends at a grid break after one more nonempty band.
    // Try six bands with the same operation-span objective and also report
    // the best number of bands under a fixed 128-operation join cost.
    let mut rows = vec![vec![None::<(usize, usize)>; size]; 13];
    rows[0][0] = Some((0, 0));
    for count in 1..rows.len() {
        for j in 1..size {
            rows[count][j] = (0..j)
                .filter(|&i| score[i][j].ops > 0)
                .filter_map(|i| rows[count - 1][i].map(|(cost, _)| (cost + score[i][j].span, i)))
                .min();
        }
    }
    let best_count = (2..rows.len())
        .min_by_key(|&c| rows[c][size - 1].unwrap().0 + 128 * c)
        .unwrap();
    for (name, count) in [("span-six", 6), ("join-adjusted", best_count)] {
        let mut j = size - 1;
        let mut route = vec![j];
        for c in (1..=count).rev() {
            j = rows[c][j].unwrap().1;
            route.push(j);
        }
        route.reverse();
        let cut = route.iter().map(|&i| candidates[i]).collect::<Vec<_>>();
        let cost = route
            .windows(2)
            .map(|pair| score[pair[0]][pair[1]])
            .collect::<Vec<_>>();
        print_plan(name, &cut, &cost);
    }
}
