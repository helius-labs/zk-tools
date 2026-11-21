//! Build the existing dependency-band schedule with bounded index arrays.
//! Group components by sorting (band, root, original position); no hash map
//! or growing per-component vectors are needed during grouping.
use super::*;
use crate::preparation::Reservation;

struct Scratch {
    producer: Vec<u32>,
    depth: Vec<u32>,
    parent: Vec<u32>,
    order: Vec<u32>,
    reads: Vec<usize>,
}
fn add(a: usize, b: usize) -> Option<usize> {
    a.checked_add(b)
}
fn mul(a: usize, b: usize) -> Option<usize> {
    a.checked_mul(b)
}
fn reads_bound(plan: &Plan) -> Option<usize> {
    plan.ops.iter().try_fold(0usize, |maximum, op| {
        let count = match op {
            Op::Hint(h) => h.input().len(),
            Op::Constraint(s) => {
                let mut count = 0usize;
                for sum in &plan.sums[s.sums()] {
                    count = add(
                        count,
                        1 + usize::from(matches!(sum.previous(), Previous::Slot(_))),
                    )?;
                }
                for e in &s.expressions {
                    count = add(
                        count,
                        match e {
                            Expression::Zero | Expression::Constant(_) => 0,
                            Expression::Wire(_) | Expression::Term(_) => 1,
                            Expression::Sum(a, b) => (b - a) as usize,
                        },
                    )?;
                }
                count
            }
        };
        Some(maximum.max(count))
    })
}
fn each_write(plan: &Plan, op: &Op, mut f: impl FnMut(usize)) {
    match op {
        Op::Hint(h) => {
            for wire in h.outputs() {
                f(wire);
            }
        }
        Op::Constraint(s) => {
            if let Some(a) = &s.assignment {
                f(a.wire());
            }
            let first = plan.n_wires + s.first_slot();
            for slot in first..first + plan.stored(s) {
                f(slot);
            }
        }
    }
}
fn find(parent: &mut [u32], mut x: u32) -> u32 {
    while parent[x as usize] != x {
        parent[x as usize] = parent[parent[x as usize] as usize];
        x = parent[x as usize];
    }
    x
}
fn shape_bytes(n: usize, components: usize, bands: usize) -> Option<usize> {
    add(
        add(add(mul(n, 4)?, mul(components, 88)?)?, mul(bands, 88)?)?,
        64,
    )
}
fn vector_bytes<T>(v: &Vec<T>) -> Option<usize> {
    let n = mul(v.capacity(), core::mem::size_of::<T>())?;
    add(n, if n == 0 { 0 } else { 64 })
}
fn retained_bytes(bands: &Vec<Vec<Vec<u32>>>) -> Option<usize> {
    let mut bytes = vector_bytes(bands)?;
    for band in bands {
        bytes = add(bytes, vector_bytes(band)?)?;
        for task in band {
            bytes = add(bytes, vector_bytes(task)?)?;
        }
    }
    Some(bytes)
}
// Remap dependency depths to a monotone band index. The parent array
// is dead until the union-find pass; borrow it as a depth histogram.
fn balance_depths(depths: &mut [u32], histogram: &mut [u32], bands: usize) {
    histogram.fill(0);
    for &depth in depths.iter() {
        histogram[depth as usize] += 1;
    }
    let total = depths.len() as u128;
    let levels = histogram.len() as u128;
    let bands = bands as u128;
    let mut seen = 0u128;
    for (level, cell) in histogram.iter_mut().enumerate() {
        let count = u128::from(*cell);
        let middle = 2 * seen + count;
        // Average the normalized dependency depth and cumulative work.
        let band = bands * (2 * total * level as u128 + middle * levels) / (4 * total * levels);
        *cell = band.min(bands - 1) as u32;
        seen += count;
    }
    for depth in depths {
        *depth = histogram[*depth as usize];
    }
}

// Merge adjacent small independent components before allocating task
// vectors. Every component already appears in program order in this slice.
fn pack_ready_components(
    order: &[u32],
    parent: &mut [u32],
    band: impl Fn(usize) -> usize,
    limit: usize,
) {
    let mut begin = 0;
    let mut used = limit;
    let mut destination = 0;
    let mut last_band = usize::MAX;
    while begin < order.len() {
        let first = order[begin] as usize;
        let root = parent[first];
        let current_band = band(first);
        let mut end = begin + 1;
        while end < order.len() && parent[order[end] as usize] == root {
            end += 1;
        }
        let size = end - begin;
        if current_band != last_band || limit.saturating_sub(used) < size {
            destination = root;
            last_band = current_band;
            used = 0;
        } else {
            for &index in &order[begin..end] {
                parent[index as usize] = destination;
            }
        }
        used += size;
        begin = end;
    }
}

impl Scratch {
    // Reused by outer grouping and every local refinement. All arrays have
    // their maximum capacity before grouping; no per-op writes allocation.
    fn bands(
        &mut self,
        plan: &Plan,
        indices: Option<&[u32]>,
        bands: usize,
        reservation: &mut Reservation,
        pack_tasks: bool,
    ) -> Option<Vec<Vec<Vec<u32>>>> {
        let n = indices.map_or(plan.ops.len(), <[u32]>::len);
        if n == 0 {
            return Some(Vec::new());
        }
        let op_index = |i: usize| indices.map_or(i, |v| v[i] as usize);
        self.producer.fill(u32::MAX);
        for i in 0..n {
            let op = &plan.ops[op_index(i)];
            plan.reads(op, &mut self.reads);
            self.depth[i] = self
                .reads
                .iter()
                .filter_map(|&wire| {
                    let p = self.producer[wire];
                    (p != u32::MAX).then(|| self.depth[p as usize] + 1)
                })
                .max()
                .unwrap_or(0);
            each_write(plan, op, |wire| self.producer[wire] = i as u32);
        }
        let deepest = self.depth[..n].iter().copied().max().unwrap_or(0) as usize + 1;
        let bands = bands.max(1).min(deepest);
        let balance = pack_tasks && indices.is_none();
        if balance {
            balance_depths(&mut self.depth[..n], &mut self.parent[..deepest], bands);
        }
        let band = |i: usize| {
            if balance {
                self.depth[i] as usize
            } else {
                self.depth[i] as usize * bands / deepest
            }
        };
        for i in 0..n {
            self.parent[i] = i as u32;
        }
        for i in 0..n {
            plan.reads(&plan.ops[op_index(i)], &mut self.reads);
            for &wire in &self.reads {
                let p = self.producer[wire] as usize;
                if p < i && band(p) == band(i) {
                    let x = find(&mut self.parent, i as u32);
                    let y = find(&mut self.parent, p as u32);
                    self.parent[x as usize] = y;
                }
            }
        }
        for i in 0..n {
            self.parent[i] = find(&mut self.parent, i as u32);
            self.order[i] = i as u32;
        }
        self.order[..n].sort_unstable_by_key(|&i| (band(i as usize), self.parent[i as usize], i));
        if pack_tasks {
            pack_ready_components(&self.order[..n], &mut self.parent[..n], band, 128);
        }
        let same_component = |a: u32, b: u32| self.parent[a as usize] == self.parent[b as usize];
        let mut components = 1;
        let mut used_bands = 1;
        for pair in self.order[..n].windows(2) {
            components += usize::from(!same_component(pair[0], pair[1]));
            used_bands += usize::from(band(pair[0] as usize) != band(pair[1] as usize));
        }
        // Charge all final vector capacities before allocating any of them.
        if !reservation.increase(plan.n_constraints, shape_bytes(n, components, used_bands)?) {
            return None;
        }
        let mut output = Vec::with_capacity(used_bands);
        let mut begin = 0;
        while begin < n {
            let current = band(self.order[begin] as usize);
            let mut end = begin + 1;
            while end < n && band(self.order[end] as usize) == current {
                end += 1;
            }
            let count = 1 + self.order[begin..end]
                .windows(2)
                .filter(|p| !same_component(p[0], p[1]))
                .count();
            let mut tasks = Vec::with_capacity(count);
            let mut first = begin;
            while first < end {
                let mut last = first + 1;
                while last < end && same_component(self.order[first], self.order[last]) {
                    last += 1;
                }
                let mut task = Vec::with_capacity(last - first);
                for &i in &self.order[first..last] {
                    task.push(op_index(i as usize) as u32);
                }
                tasks.push(task);
                first = last;
            }
            tasks.sort_unstable_by_key(|t| std::cmp::Reverse(t.len()));
            output.push(tasks);
            begin = end;
        }
        debug_assert_eq!(
            retained_bytes(&output),
            shape_bytes(n, components, used_bands)
        );
        Some(output)
    }
}

pub(super) fn build(plan: &Plan, epochs: usize, local: usize, minimum: usize) -> Option<Schedule> {
    build_with_workspace(plan, epochs, local, minimum, proof_workspace(plan))
}
pub(super) fn build_with_workspace(
    plan: &Plan,
    epochs: usize,
    local: usize,
    minimum: usize,
    _workspace: Option<usize>,
) -> Option<Schedule> {
    let n = plan.ops.len();
    let values = add(plan.n_wires, plan.n_slots)?;
    if n == 0 || epochs == 0 || local == 1 || n > u32::MAX as usize {
        return None;
    }
    // Reject incomplete or multiply-written legacy images before constructing
    // a parallel schedule. This cached check has its own bounded reservation.
    if !*plan.definite.get_or_init(|| plan.definitely_writes_all()) {
        return None;
    }
    let max_reads = reads_bound(plan)?;
    let initial = add(
        add(
            add(mul(values, 4)?, mul(n, 12)?)?,
            mul(max_reads, core::mem::size_of::<usize>())?,
        )?,
        32 * 1024 + 5 * 64,
    )?;
    let mut reservation = {
        #[cfg(zopro_resource_schedule)]
        if std::env::var("ZOPRO_CPU_RESOURCE_SCHEDULE").as_deref() == Ok("1") {
            Reservation::schedule(plan.n_constraints, Some(initial), _workspace)?
        } else {
            Reservation::acquire_solver_schedule(plan.n_constraints, Some(initial))?
        }
        #[cfg(not(zopro_resource_schedule))]
        {
            Reservation::acquire_solver_schedule(plan.n_constraints, Some(initial))?
        }
    };
    let mut scratch = Scratch {
        producer: vec![u32::MAX; values],
        depth: vec![0; n],
        parent: vec![0; n],
        order: vec![0; n],
        reads: Vec::with_capacity(max_reads),
    };
    let pack_tasks = std::env::var("ZOPRO_SOLVE_PACK_TASKS").as_deref() == Ok("1");
    let epochs = scratch.bands(plan, None, epochs, &mut reservation, pack_tasks)?;
    let eligible = if local == 0 {
        0
    } else {
        epochs
            .iter()
            .flatten()
            .filter(|t| t.len() >= minimum)
            .count()
    };
    let map_charge = if eligible == 0 {
        0
    } else {
        add(mul(eligible, 256)?, 64)?
    };
    if !reservation.increase(plan.n_constraints, map_charge) {
        return None;
    }
    let mut refined = if eligible == 0 {
        FxMap::default()
    } else {
        FxMap::with_capacity_and_hasher(eligible, Default::default())
    };
    for task in epochs
        .iter()
        .flatten()
        .filter(|t| local > 0 && t.len() >= minimum)
    {
        let waves = scratch.bands(plan, Some(task), local, &mut reservation, pack_tasks)?;
        refined.insert(task[0] as usize, waves);
    }
    let peak_charge = reservation.bytes();
    let mut retained = add(retained_bytes(&epochs)?, map_charge)?;
    for waves in refined.values() {
        retained = add(retained, retained_bytes(waves)?)?;
    }
    // Release construction credit only after every index/read buffer drops.
    drop(scratch);
    let reservation = reservation.retain(Some(retained));
    if std::env::var_os("ZOPRO_CPU_BOUNDED_SCHEDULE_TRACE").is_some() {
        eprintln!("BOUNDED_SCHEDULE ops={n} values={values} peak_charge={peak_charge} retained_charge={retained} epochs={} refined={}",epochs.len(),refined.len());
    }
    Some(Schedule {
        epochs,
        refined,
        _reservation: Some(reservation),
    })
}

// Standalone Plan users have no Key geometry. Bound possible folded slots,
// product query length and the independent constraint correction as well.
fn proof_workspace(plan: &Plan) -> Option<usize> {
    #[cfg(not(zopro_resource_schedule))]
    {
        let _ = plan;
        return None;
    }
    #[cfg(zopro_resource_schedule)]
    {
        use crate::msm_affine::workspace::{forecast, Geometry};
        let domain = plan.n_constraints.checked_next_power_of_two()?;
        forecast(Geometry {
            wires: plan.n_wires,
            slots: plan.n_slots,
            inputs: plan.n_inputs,
            public: plan.n_public,
            domain,
            a: plan.a_indices.len(),
            b: plan.b_indices.len(),
            k: plan
                .n_wires
                .checked_add(plan.n_slots)?
                .checked_sub(plan.n_public)?,
            h: domain,
            correction: domain,
        })
        .map(|v| v.total)
    }
}

#[cfg(test)]
mod balance_tests {
    use super::balance_depths;

    #[test]
    fn equal_depth_keeps_one_band_and_dependencies_stay_ordered() {
        let input = [0, 1, 4, 1, 2, 2, 2, 2, 2, 3, 7, 5];
        for bands in 1..=8 {
            let mut depths = input;
            let mut histogram = [u32::MAX; 8];
            balance_depths(&mut depths, &mut histogram, bands);
            for (i, &depth) in input.iter().enumerate() {
                assert!((depths[i] as usize) < bands);
                for (j, &other) in input.iter().enumerate() {
                    if depth == other {
                        assert_eq!(depths[i], depths[j]);
                    }
                    if depth < other {
                        assert!(depths[i] <= depths[j]);
                    }
                }
            }
        }
        let mut uniform = [0, 1, 2, 3, 4, 5, 6, 7];
        balance_depths(&mut uniform, &mut [0; 8], 4);
        assert_eq!(uniform, [0, 0, 1, 1, 2, 2, 3, 3]);
    }
}

#[cfg(test)]
mod packing_tests {
    use super::pack_ready_components;
    #[test]
    fn packing_respects_bands_component_order_and_large_tasks() {
        let order = [2, 4, 0, 1, 3, 5, 6, 8, 7, 9, 10, 11];
        let mut roots = [0, 1, 2, 1, 2, 5, 5, 5, 5, 9, 10, 11];
        pack_ready_components(&order, &mut roots, |i| usize::from(i >= 9), 3);
        assert_eq!(
            order.map(|i| roots[i as usize]),
            [2, 2, 2, 1, 1, 5, 5, 5, 5, 9, 9, 9]
        );
    }
}
