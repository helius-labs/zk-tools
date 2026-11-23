//! Exact sparse Poseidon spans over the existing solver plan.
//!
//! Recognition checks field coefficient identities and preserves every original
//! wire, stored prefix and constraint row. Unmatched tasks keep the interpreter.
use super::*;
mod constants;
use constants::parameters;
const TASK: u32 = 1 << 27;
const INDEX: u32 = TASK - 1;
const MAX_N: usize = 156;
const MAX_D: usize = 170;
const SCRATCH: usize = 2 * 1024 * 1024;

// Above this count, ordinary u32 operation IDs can contain the tag bits.
// Test the encoding's eligibility before interpreting any task marker.
#[inline(always)]
fn tagged_task(operation_count: usize, first: Option<u32>) -> bool {
    operation_count <= INDEX as usize && first.is_some_and(|code| code & TASK != 0)
}

#[derive(Clone, Copy, Default)]
struct Meta {
    first: usize,
    x: usize,
    fifth: usize,
}
fn positions(t: usize, p: usize, k: usize) -> [usize; 3] {
    if k < 3 * t {
        let b = k / t * 3 * t;
        let i = k % t;
        [b + i, b + t + i, b + 2 * t + i]
    } else if k < 3 * t + p {
        let b = 9 * t + (k - 3 * t) * 3;
        [b, b + 1, b + 2]
    } else {
        let j = k - 3 * t - p;
        let b = 9 * t + 3 * p + j / t * 3 * t;
        let i = j % t;
        [b + i, b + t + i, b + 2 * t + i]
    }
}
fn wire(e: &Expression) -> Option<usize> {
    if let Expression::Wire(w) = e {
        Some(*w as usize)
    } else {
        None
    }
}
fn step(p: &Plan, id: u32) -> Option<&Step> {
    if let Op::Constraint(s) = &p.ops[(id & INDEX) as usize] {
        Some(s)
    } else {
        None
    }
}
fn unit(s: &Step) -> Option<usize> {
    let a = s.assignment.as_ref()?;
    (a.side() == 2 && a.coefficient() == 1 && matches!(s.expressions[2], Expression::Zero))
        .then(|| a.wire())
}
fn shape(p: &Plan, task: &[u32], t: usize, partial: usize, meta: &mut [Meta; MAX_N]) -> bool {
    let n = 7 * t + partial;
    if task.len() < 3 * n {
        return false;
    }
    for k in 0..n {
        let [first, middle, last] = positions(t, partial, k);
        let (Some(a), Some(b), Some(c)) = (
            step(p, task[first]),
            step(p, task[middle]),
            step(p, task[last]),
        ) else {
            return false;
        };
        let (Some(square), Some(fourth), Some(fifth)) = (unit(a), unit(b), unit(c)) else {
            return false;
        };
        let Some(x) = wire(&a.expressions[0]) else {
            return false;
        };
        if wire(&a.expressions[1]) != Some(x) || !b.sums().is_empty() || !c.sums().is_empty() {
            return false;
        }
        if wire(&b.expressions[0]) != Some(square) || wire(&b.expressions[1]) != Some(square) {
            return false;
        }
        if !((wire(&c.expressions[0]) == Some(x) && wire(&c.expressions[1]) == Some(fourth))
            || (wire(&c.expressions[1]) == Some(x) && wire(&c.expressions[0]) == Some(fourth)))
        {
            return false;
        }
        let sums = &p.sums[a.sums()];
        if x != p.n_wires + a.first_slot()
            || sums.last().is_none_or(|s| !s.stored())
            || sums[..sums.len() - 1].iter().any(Sum::stored)
        {
            return false;
        }
        meta[k] = Meta { first, x, fifth };
    }
    true
}
fn zeros(n: usize) -> Option<Vec<Fr>> {
    let mut v = Vec::new();
    v.try_reserve_exact(n).ok()?;
    v.resize(n, Fr::ZERO);
    Some(v)
}
struct Pattern {
    t: usize,
    #[cfg(any(test, not(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android")))))]
    partial: usize,
    n: usize,
    d: usize,
    inputs: Vec<Fr>,
}
impl Pattern {
    fn new(t: usize, partial: usize, a: &[Fr]) -> Option<Self> {
        let n = 7 * t + partial;
        let d = n + t + 1;
        assert!(n <= MAX_N && d <= MAX_D);
        let mut inputs = zeros(n * d)?;
        let mut s = zeros(t * d)?;
        let mut next = zeros(t * d)?;
        for i in 0..t {
            inputs[i * d + n + 1 + i] = Fr::ONE;
            s[i * d + 1 + i] = Fr::ONE;
        }
        let count = 8 * t + partial;
        let mds = count;
        let pre = mds + t * t;
        let sparse = pre + t * t;
        fn dense(s: &mut Vec<Fr>, next: &mut Vec<Fr>, t: usize, d: usize, a: &[Fr], offset: usize) {
            next.fill(Fr::ZERO);
            for i in 0..t {
                for j in 0..t {
                    let coefficient = a[offset + i * t + j];
                    for z in 0..d {
                        let v = s[j * d + z];
                        if v != Fr::ZERO {
                            next[i * d + z] += coefficient * v;
                        }
                    }
                }
            }
            core::mem::swap(s, next);
        }
        dense(&mut s, &mut next, t, d, a, mds);
        let mut k = t;
        for r in [2, 3] {
            for i in 0..t {
                s[i * d] += a[r * t + i];
                inputs[k * d..(k + 1) * d].copy_from_slice(&s[i * d..(i + 1) * d]);
                s[i * d..(i + 1) * d].fill(Fr::ZERO);
                s[i * d + k + 1] = Fr::ONE;
                k += 1;
            }
            dense(&mut s, &mut next, t, d, a, if r == 3 { pre } else { mds });
        }
        let mut c = 4 * t;
        let mut x = [Fr::ZERO; MAX_D];
        x[..d].copy_from_slice(&s[..d]);
        x[0] += a[c];
        c += 1;
        for r in 0..partial {
            let offset = sparse + r * (2 * t - 1);
            let mut rest = [Fr::ZERO; MAX_D];
            if r + 1 < partial {
                rest[0] = a[c];
                c += 1;
            }
            for j in 1..t {
                for z in 0..d {
                    let v = s[j * d + z];
                    if v != Fr::ZERO {
                        rest[z] += a[offset + j] * v;
                    }
                }
            }
            inputs[k * d..(k + 1) * d].copy_from_slice(&x[..d]);
            for i in 1..t {
                s[i * d + k + 1] += a[offset + t + i - 1];
            }
            x = rest;
            x[k + 1] += a[offset];
            k += 1;
        }
        s[..d].copy_from_slice(&x[..d]);
        for _ in 0..4 {
            for i in 0..t {
                s[i * d] += a[c + i];
                inputs[k * d..(k + 1) * d].copy_from_slice(&s[i * d..(i + 1) * d]);
                s[i * d..(i + 1) * d].fill(Fr::ZERO);
                s[i * d + k + 1] = Fr::ONE;
                k += 1;
            }
            c += t;
            dense(&mut s, &mut next, t, d, a, mds);
        }
        assert_eq!(k, n);
        assert_eq!(c, count);
        Some(Self {
            t,
            #[cfg(any(test, not(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android")))))]
            partial,
            n,
            d,
            inputs,
        })
    }
    fn recognizes(&self, p: &Plan, task: &[u32], meta: &[Meta; MAX_N]) -> bool {
        let mut actual = [Fr::ZERO; MAX_D];
        for k in self.t..self.n {
            let first = step(p, task[meta[k].first]).unwrap();
            actual.fill(Fr::ZERO);
            for sum in &p.sums[first.sums()] {
                match sum.previous() {
                    Previous::None => actual.fill(Fr::ZERO),
                    Previous::Chain => {}
                    Previous::Slot(slot) => {
                        let Some(j) = meta[..k].iter().position(|m| m.x == slot) else {
                            return false;
                        };
                        actual[..self.d]
                            .copy_from_slice(&self.inputs[j * self.d..(j + 1) * self.d]);
                    }
                }
                let wire = sum.term.wire();
                let coefficient = p.coefficients[sum.term.coefficient()];
                if wire == 0 {
                    actual[0] += coefficient;
                } else if let Some(j) = meta[..k].iter().position(|m| m.fifth == wire) {
                    actual[j + 1] += coefficient;
                } else if let Some(j) = meta[..k].iter().position(|m| m.x == wire) {
                    for (z, v) in actual[..self.d].iter_mut().enumerate() {
                        *v += coefficient * self.inputs[j * self.d + z];
                    }
                } else {
                    return false;
                }
            }
            if actual[..self.d] != self.inputs[k * self.d..(k + 1) * self.d] {
                return false;
            }
        }
        true
    }
    #[cfg(any(test, not(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android")))))]
    fn tag(&self, p: &Plan, task: &mut [u32]) -> usize {
        tag_spans(p, task, self.t, self.partial, |span, meta| {
            self.recognizes(p, span, meta)
        })
    }
}

fn tag_spans(
    p: &Plan,
    task: &mut [u32],
    t: usize,
    partial: usize,
    recognizes: impl Fn(&[u32], &[Meta; MAX_N]) -> bool,
) -> usize {
    let span_len = 3 * (7 * t + partial);
    let mut pos = 0;
    let mut count = 0;
    let mut meta = [Meta::default(); MAX_N];
    while pos + span_len <= task.len() {
        let span = &task[pos..];
        let tagged = (span[0] >> 28) as usize;
        if tagged != 0 {
            let (partial, _) = parameters(tagged);
            pos += 3 * (7 * tagged + partial);
            continue;
        }
        if shape(p, span, t, partial, &mut meta) && recognizes(span, &meta) {
            task[pos] |= (t as u32) << 28;
            count += 1;
            pos += span_len;
        } else {
            pos += 1;
        }
    }
    if count != 0 {
        task[0] |= TASK;
    }
    count
}
const EMIT_MARK: u32 = 1 << 31;
const EMIT_NORMAL: u32 = 1 << 30;
const EMIT_BASE: u32 = (1 << 24) - 1;

// Store a first address and two forward displacements. Larger or decreasing
// addresses keep their original operation indices.
fn pack_three([first, second, third]: [usize; 3]) -> Option<u32> {
    let next = second.checked_sub(first)?;
    let last = third.checked_sub(second)?;
    if first > EMIT_BASE as usize || next > 15 || last > 15 {
        return None;
    }
    Some(first as u32 | (next as u32) << 24 | (last as u32) << 28)
}

#[inline(always)]
fn unpack_three(word: u32) -> [usize; 3] {
    let first = (word & EMIT_BASE) as usize;
    let second = first + ((word >> 24) & 15) as usize;
    [first, second, second + (word >> 28) as usize]
}

// Run only after all original coefficient and shape checks have tagged the
// spans. The first round still executes its original operations. Every later
// S-box has three distinct words that belong exclusively to that S-box.
fn compile_emits(plan: &Plan, task: &mut [u32]) -> usize {
    if !plan.poseidon_task(task) {
        return 0;
    }
    let mut compiled = 0;
    let mut pos = 0;
    while pos < task.len() {
        let width = (task[pos] >> 28) as usize;
        if width == 0 {
            pos += 1;
            continue;
        }
        let (partial, _) = parameters(width);
        let rounds = 7 * width + partial;
        for k in width..rounds {
            let [f, m, l] = positions(width, partial, k).map(|i| i + pos);
            let [first, middle, last] = [f, m, l].map(|i| step(plan, task[i]).unwrap());
            let slot = plan.n_wires + first.first_slot();
            if slot >= EMIT_NORMAL as usize {
                continue;
            }
            let steps = [first, middle, last];
            let Some(rows) = pack_three(steps.map(Step::constraint)) else {
                continue;
            };
            let Some(wires) = pack_three(steps.map(|s| s.assignment.as_ref().unwrap().wire()))
            else {
                continue;
            };
            let orientation = if wire(&last.expressions[0]) == Some(slot) {
                EMIT_NORMAL
            } else {
                0
            };
            task[f] = EMIT_MARK | orientation | slot as u32;
            task[m] = rows;
            task[l] = wires;
            compiled += 1;
        }
        pos += 3 * rounds;
    }
    compiled
}

impl Plan {
    #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
    pub(super) fn apple_poseidon_epochs(&self) -> Option<usize> {
        if rayon::current_num_threads() < 16
            || std::env::var("ZOPRO_CPU_BOUNDED_SCHEDULE").as_deref() == Ok("0")
            || std::env::var("ZOPRO_CPU_SPARSE_POSEIDON").as_deref() == Ok("0")
        {
            return None;
        }
        // These bands keep more of each hash within one task for recognition.
        match self.n_constraints {
            16385..=32768 => Some(4),
            32769..=131072 => Some(8),
            131073..=262144 => Some(6),
            _ => None,
        }
    }

    pub(super) fn tag_poseidon(&self, s: &mut Schedule) -> bool {
        if self.ops.len() > INDEX as usize || s.epochs.is_empty() {
            return false;
        }
        #[cfg(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios")))]
        if !s.refined.is_empty() {
            return false;
        }
        #[cfg(not(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android"))))]
        if !crate::simd::cpu_option("ZOPRO_CPU_SPARSE_POSEIDON") {
            return false;
        }
        #[cfg(all(target_os = "macos", target_arch = "aarch64"))]
        let reservation = crate::preparation::Reservation::scalar_grouping(Some(SCRATCH));
        #[cfg(not(all(target_os = "macos", target_arch = "aarch64")))]
        let reservation = crate::preparation::Reservation::analysis(Some(SCRATCH));
        let Some(_reservation) = reservation else {
            return false;
        };
        let mut counts = [0usize; 14];
        for t in 2..=13 {
            let (partial, a) = parameters(t);
            #[cfg(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android")))]
            {
                use rayon::prelude::*;
                // Initialize the symbolic matrix once, only after a task
                // matches the operation shape. Leaf tasks are independent.
                let pattern = std::sync::OnceLock::new();
                let tasks = s.epochs.par_iter_mut();
                #[cfg(any(target_os = "ios", target_os = "android"))]
                let tasks = tasks.flat_map(|epoch| epoch.par_iter_mut());
                #[cfg(target_os = "macos")]
                let tasks = tasks.flat_map_iter(|epoch| epoch);
                let tag = |task: &mut Vec<u32>| {
                    tag_spans(self, task, t, partial, |span, meta| {
                        pattern
                            .get_or_init(|| Pattern::new(t, partial, a))
                            .as_ref()
                            .is_some_and(|p| p.recognizes(self, span, meta))
                    })
                };
                #[cfg(target_os = "android")]
                let tasks = tasks.filter(|task| !s.refined.contains_key(&((task[0] & INDEX) as usize)));
                counts[t] = tasks.map(tag).sum();
                #[cfg(target_os = "android")]
                if !s.refined.is_empty() {
                    counts[t] += s.refined.par_iter_mut()
                        .flat_map(|(_, waves)| waves.par_iter_mut())
                        .flat_map(|wave| wave.par_iter_mut()).map(tag).sum::<usize>();
                }
            }
            #[cfg(not(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android"))))]
            {
                let Some(pattern) = Pattern::new(t, partial, a) else {
                    break;
                };
                for task in s.epochs.iter_mut().flatten() {
                    if !s.refined.contains_key(&((task[0] & INDEX) as usize)) {
                        counts[t] += pattern.tag(self, task);
                    }
                }
                for task in s.refined.values_mut().flatten().flatten() {
                    counts[t] += pattern.tag(self, task);
                }
            }
        }
        let emit = std::env::var("ZOPRO_CPU_POSEIDON_EMIT");
        if emit.as_deref() == Ok("1")
            || (cfg!(all(target_arch = "aarch64", any(target_os = "macos", target_os = "ios", target_os = "android")))
                && emit.as_deref() != Ok("0"))
        {
            #[cfg(any(target_os = "ios", target_os = "android"))]
            {
                use rayon::prelude::*;
                let tasks = s.epochs.par_iter_mut().flat_map(|epoch| epoch.par_iter_mut());
                #[cfg(target_os = "android")]
                let tasks = tasks.filter(|task| !s.refined.contains_key(&((task[0] & INDEX) as usize)));
                tasks.for_each(|task| { compile_emits(self, task); });
                #[cfg(target_os = "android")]
                if !s.refined.is_empty() {
                    s.refined.par_iter_mut().flat_map(|(_, waves)| waves.par_iter_mut())
                        .flat_map(|wave| wave.par_iter_mut())
                        .for_each(|task| { compile_emits(self, task); });
                }
            }
            #[cfg(not(any(target_os = "ios", target_os = "android")))]
            {
                for task in s.epochs.iter_mut().flatten() {
                    if !s.refined.contains_key(&((task[0] & INDEX) as usize)) {
                        compile_emits(self, task);
                    }
                }
                for task in s.refined.values_mut().flatten().flatten() {
                    compile_emits(self, task);
                }
            }
        }
        if std::env::var("ZOPRO_CPU_POSEIDON_TRACE").as_deref() == Ok("1") {
            eprintln!("POSEIDON_SPANS {counts:?}");
        }
        #[cfg(all(target_os = "android", feature = "trace"))]
        if std::env::var("ZOPRO_ANDROID_POSEIDON_WAVES").as_deref() == Ok("1") {
            let mut totals = [0usize; 14];
            for (epoch_id, epoch) in s.epochs.iter().enumerate() {
                let mut waves = Vec::<usize>::new();
                for task in epoch.iter().filter(|task| self.poseidon_task(task)) {
                    let mut pos = 0;
                    let mut wave = 0;
                    while pos < task.len() {
                        let width = (task[pos] >> 28) as usize;
                        if width == 0 {
                            pos += 1;
                            continue;
                        }
                        totals[width] += 1;
                        if width == 3 {
                            if waves.len() <= wave {
                                waves.push(0);
                            }
                            waves[wave] += 1;
                            wave += 1;
                        }
                        let (partial, _) = parameters(width);
                        pos += 3 * (7 * width + partial);
                    }
                }
                if !waves.is_empty() {
                    log::info!("ZOPRO_TRACE android-poseidon-waves epoch={epoch_id} width=3 waves={waves:?}");
                }
            }
            log::info!("ZOPRO_TRACE android-poseidon-widths {totals:?}");
        }
        counts.into_iter().any(|count| count != 0)
    }
    #[inline(always)]
    pub(super) fn poseidon_task(&self, task: &[u32]) -> bool {
        tagged_task(self.ops.len(), task.first().copied())
    }

    pub(super) unsafe fn run_poseidon_task(&self, task: &[u32], v: View) -> Result<(), String> {
        if !self.poseidon_task(task) {
            for &op in task {
                unsafe {
                    self.run(&self.ops[op as usize], v)?;
                }
            }
            return Ok(());
        }
        let mut pos = 0;
        while pos < task.len() {
            let t = (task[pos] >> 28) as usize;
            if t == 0 {
                unsafe {
                    self.run(&self.ops[(task[pos] & INDEX) as usize], v)?;
                }
                pos += 1;
            } else {
                let (partial, a) = parameters(t);
                #[cfg(all(target_os = "android", target_arch = "aarch64", not(no_asm)))]
                {
                    static SPECIALIZED: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
                    let enabled = SPECIALIZED.get_or_init(|| {
                        std::env::var("ZOPRO_ANDROID_POSEIDON_WIDTH3").as_deref() == Ok("1")
                    });
                    if t == 3 && *enabled {
                        unsafe { self.run_poseidon::<3>(&task[pos..], t, partial, a, v)? };
                        pos += 3 * (7 * t + partial);
                        continue;
                    }
                }
                unsafe {
                    self.run_poseidon::<0>(&task[pos..], t, partial, a, v)?;
                }
                pos += 3 * (7 * t + partial);
            }
        }
        Ok(())
    }
    unsafe fn emit_poseidon(
        &self,
        task: &[u32],
        t: usize,
        partial: usize,
        k: usize,
        x: Fr,
        v: View,
    ) -> Fr {
        let [f, m, l] = positions(t, partial, k);
        let (slot, rows, wires, normal) = if task[f] & EMIT_MARK != 0 {
            (
                (task[f] & !(EMIT_MARK | EMIT_NORMAL)) as usize,
                unpack_three(task[m]),
                unpack_three(task[l]),
                task[f] & EMIT_NORMAL != 0,
            )
        } else {
            let steps = [f, m, l].map(|i| step(self, task[i]).unwrap());
            let slot = self.n_wires + steps[0].first_slot();
            (
                slot,
                steps.map(Step::constraint),
                steps.map(|s| s.assignment.as_ref().unwrap().wire()),
                wire(&steps[2].expressions[0]) == Some(slot),
            )
        };
        let square = x.square();
        let fourth = square.square();
        let fifth = fourth * x;
        // Exact recognition proves these are precisely the original writes.
        unsafe {
            *v.w.add(slot) = x;
            *v.w.add(wires[0]) = square;
            *v.w.add(wires[1]) = fourth;
            *v.w.add(wires[2]) = fifth;
            v.a.add(rows[0]).write(x);
            v.b.add(rows[0]).write(x);
            v.a.add(rows[1]).write(square);
            v.b.add(rows[1]).write(square);
            v.a.add(rows[2]).write(if normal { x } else { fourth });
            v.b.add(rows[2]).write(if normal { fourth } else { x });
            if !v.c.is_null() {
                v.c.add(rows[0]).write(square);
                v.c.add(rows[1]).write(fourth);
                v.c.add(rows[2]).write(fifth);
            }
        }
        fifth
    }
    unsafe fn run_poseidon<const WIDTH: usize>(
        &self,
        task: &[u32],
        t: usize,
        partial: usize,
        a: &[Fr],
        v: View,
    ) -> Result<(), String> {
        let t = if WIDTH == 0 { t } else { WIDTH };
        let partial = if WIDTH == 3 { 57 } else { partial };
        for &id in &task[..3 * t] {
            unsafe {
                self.run(&self.ops[(id & INDEX) as usize], v)?;
            }
        }
        let count = 8 * t + partial;
        let mds = count;
        let pre = mds + t * t;
        let sparse = pre + t * t;
        let mut s = [Fr::ZERO; 13];
        for (i, x) in s[..t].iter_mut().enumerate() {
            let row = step(self, task[2 * t + i]).unwrap();
            *x = unsafe { *v.w.add(row.assignment.as_ref().unwrap().wire()) };
        }
        fn dense(s: &mut [Fr; 13], t: usize, a: &[Fr], offset: usize) {
            let mut next = [Fr::ZERO; 13];
            #[cfg(all(target_os = "android", target_arch = "aarch64", not(no_asm)))]
            if t == 3 {
                static SUM3: std::sync::OnceLock<bool> = std::sync::OnceLock::new();
                let enabled = SUM3.get_or_init(|| std::env::var("ZOPRO_ANDROID_SOLVER_SUM3").as_deref() == Ok("1"));
                if *enabled {
                    let inputs = [s[0], s[1], s[2]];
                    for (i, output) in next[..3].iter_mut().enumerate() {
                        let row = offset + i * 3;
                        *output = Fr::sum_of_three_products([a[row], a[row + 1], a[row + 2]], inputs);
                    }
                    *s = next;
                    return;
                }
            }
            for i in 0..t {
                if t == 3 {
                    let row = offset + i * 3;
                    next[i] =
                        Fr::sum_of_products(&a[row], &s[0], &a[row + 1], &s[1]) + a[row + 2] * s[2];
                } else {
                    for j in 0..t {
                        next[i] += a[offset + i * t + j] * s[j];
                    }
                }
            }
            *s = next;
        }
        dense(&mut s, t, a, mds);
        let mut k = t;
        for r in [2, 3] {
            for i in 0..t {
                s[i] = unsafe { self.emit_poseidon(task, t, partial, k, s[i] + a[r * t + i], v) };
                k += 1;
            }
            dense(&mut s, t, a, if r == 3 { pre } else { mds });
        }
        let mut c = 4 * t;
        let mut x = s[0] + a[c];
        c += 1;
        for r in 0..partial {
            let offset = sparse + r * (2 * t - 1);
            let constant = if r + 1 < partial {
                let z = a[c];
                c += 1;
                z
            } else {
                Fr::ZERO
            };
            #[cfg(all(target_os = "android", target_arch = "aarch64", not(no_asm)))]
            if WIDTH == 3 {
                let head = unsafe { self.emit_poseidon(task, t, partial, k, x, v) };
                k += 1;
                let coefficients = [a[offset], a[offset + 1], a[offset + 2]];
                x = constant + Fr::sum_of_three_products(coefficients, [head, s[1], s[2]]);
                s[1] += a[offset + 3] * head;
                s[2] += a[offset + 4] * head;
                continue;
            }
            let mut rest = constant;
            if t == 3 {
                rest += Fr::sum_of_products(&a[offset + 1], &s[1], &a[offset + 2], &s[2]);
            } else {
                for j in 1..t {
                    rest += a[offset + j] * s[j];
                }
            }
            let head = unsafe { self.emit_poseidon(task, t, partial, k, x, v) };
            k += 1;
            for i in 1..t {
                s[i] += a[offset + t + i - 1] * head;
            }
            x = a[offset] * head + rest;
        }
        s[0] = x;
        for r in 0..4 {
            for i in 0..t {
                s[i] = unsafe { self.emit_poseidon(task, t, partial, k, s[i] + a[c + i], v) };
                k += 1;
            }
            c += t;
            if r != 3 {
                dense(&mut s, t, a, mds);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests;

#[cfg(zopro_resource_schedule)]
mod aliases;
#[cfg(zopro_resource_schedule)]
pub(crate) use aliases::{B2Origins, RuntimeAliases};
