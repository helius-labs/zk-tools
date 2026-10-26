// Copyright 2020-2025 Consensys Software Inc.
// Licensed under Apache-2.0; see LICENSE-APACHE.
// Modified for Mopro: prepare a validated execution plan for gnark R1CS
// systems whose only hints are InvZeroHint and bits.nBits, and solve it using
// arkworks field arithmetic. The arithmetic follows constraint/bn254/solver.go's
// solveR1C and solveHintGeneric semantics.

#[cfg(all(zopro_bounded_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
mod bounded;
use zopro_bn254::Fr;
use std::collections::HashMap;
use std::hash::{BuildHasherDefault, Hasher};
use std::ops::Range;

// Interning a wide circuit's prefixes hashes millions of small keys; the
// standard map's SipHash is most of that time. Keys here are not chosen by
// an adversary: they come from the proving key the caller trusts.
#[derive(Default)]
struct FxHasher(u64);
impl Hasher for FxHasher {
    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
    #[inline]
    fn write(&mut self, bytes: &[u8]) {
        for byte in bytes {
            self.write_u64(u64::from(*byte));
        }
    }
    #[inline]
    fn write_u64(&mut self, value: u64) {
        self.0 = (self.0.rotate_left(5) ^ value).wrapping_mul(0x517c_c1b7_2722_0a95);
    }
    #[inline]
    fn write_usize(&mut self, value: usize) {
        self.write_u64(value as u64);
    }
}
type FxMap<K, V> = HashMap<K, V, BuildHasherDefault<FxHasher>>;

// The plan's indices are u32, as its image stores them: a wide key's plan
// holds millions of them (16 -> 2: 3.1 M sums, 0.37 M ops), and usize
// doubled its memory. The accessors read them as usize.
#[repr(C)]
#[derive(Clone, Copy)]
struct Term {
    coefficient: u32,
    wire: u32,
}
impl Term {
    fn new(coefficient: usize, wire: usize) -> Self {
        Self {
            coefficient: coefficient as u32,
            wire: wire as u32,
        }
    }
    fn coefficient(&self) -> usize {
        self.coefficient as usize
    }
    fn wire(&self) -> usize {
        self.wire as usize
    }
}

// Required inverse payloads replace the old inline Fr in each assignment.
// Encode the common +/-1 directly. Other values keep their exact image bits;
// fixed-size chunks bound slack without a circuit-sized hash/dedup table.
const INVERSE_ONE: u32 = u32::MAX;
const INVERSE_MINUS_ONE: u32 = u32::MAX - 1;
const INVERSE_CHUNK: usize = 128;
#[derive(Default)]
struct Inverses {
    chunks: Vec<Box<[Fr; INVERSE_CHUNK]>>,
    len: usize,
}
impl Inverses {
    fn insert(&mut self, value: Fr) -> Result<u32, &'static str> {
        if value == Fr::ONE { return Ok(INVERSE_ONE); }
        if value == -Fr::ONE { return Ok(INVERSE_MINUS_ONE); }
        let index = u32::try_from(self.len).ok().filter(|&i| i < INVERSE_MINUS_ONE)
            .ok_or("too many solver inverses")?;
        let offset = self.len % INVERSE_CHUNK;
        if offset == 0 { self.chunks.push(Box::new([Fr::ZERO; INVERSE_CHUNK])); }
        self.chunks[self.len / INVERSE_CHUNK][offset] = value;
        self.len += 1;
        Ok(index)
    }
    fn bytes(&self) -> usize {
        self.chunks.len() * core::mem::size_of::<[Fr; INVERSE_CHUNK]>()
            + self.chunks.capacity() * core::mem::size_of::<Box<[Fr; INVERSE_CHUNK]>>()
    }
    fn get(&self, index: u32) -> Fr {
        match index {
            INVERSE_ONE => Fr::ONE,
            INVERSE_MINUS_ONE => -Fr::ONE,
            _ => self.chunks[index as usize / INVERSE_CHUNK][index as usize % INVERSE_CHUNK],
        }
    }
}

struct Assignment {
    side: u8,
    wire: u32,
    coefficient: u32,
    inverse: u32,
}
impl Assignment {
    fn side(&self) -> usize {
        self.side as usize
    }
    fn wire(&self) -> usize {
        self.wire as usize
    }
    fn coefficient(&self) -> usize {
        self.coefficient as usize
    }
}
enum Expression {
    Zero,
    Wire(u32),
    Constant(u32),
    Term(u32),
    Sum(u32, u32),
}
struct Step {
    constraint: u32,
    sums: (u32, u32),
    /// Where the step's first stored sum goes in the solution's vector.
    first_slot: u32,
    expressions: [Expression; 3],
    assignment: Option<Assignment>,
}
impl Step {
    fn constraint(&self) -> usize {
        self.constraint as usize
    }
    fn sums(&self) -> Range<usize> {
        self.sums.0 as usize..self.sums.1 as usize
    }
    fn first_slot(&self) -> usize {
        self.first_slot as usize
    }
}
/// A prefix of a linear expression: an earlier sum plus a term. Most sums
/// (2.7 M of 16 -> 2's 3.1 M) are read once, by the next sum of their own
/// chain, so a solve carries them in a running value and stores only the
/// others, each in a slot of the solution's vector after the wires (0.45 M
/// at 16 -> 2: 14 MB rather than 100). `previous` holds STORED for a stored
/// sum, and below it CHAIN (the running value), NONE, or the slot it adds
/// its term to; while the plan is built, the sum's id or u32::MAX.
#[repr(C)]
#[derive(Clone, Copy)]
struct Sum {
    previous: u32,
    term: Term,
}
impl super::InPlace for Sum { const SIZE: usize = 12; }
const _: () = assert!(std::mem::size_of::<Sum>() == 12);

const STORED: u32 = 1 << 31;
const NONE: u32 = STORED - 1;
const CHAIN: u32 = STORED - 2;
enum Previous {
    None,
    Chain,
    Slot(usize),
}
impl Sum {
    fn previous(&self) -> Previous {
        match self.previous & !STORED {
            NONE => Previous::None,
            CHAIN => Previous::Chain,
            slot => Previous::Slot(slot as usize),
        }
    }
    fn stored(&self) -> bool {
        self.previous & STORED != 0
    }
}
/// A deflated stream inflated a block at a time, read as bytes counted or
/// as little-endian u32 words.
struct Deflated<'a> {
    state: Box<miniz_oxide::inflate::stream::InflateState>,
    input: &'a [u8],
    buffer: Vec<u8>,
    start: usize,
    end: usize,
    done: bool,
    failed: bool,
}
impl<'a> Deflated<'a> {
    fn new(input: &'a [u8]) -> Self {
        Self {
            state: miniz_oxide::inflate::stream::InflateState::new_boxed(miniz_oxide::DataFormat::Raw),
            input,
            buffer: vec![0; 1 << 16],
            start: 0,
            end: 0,
            done: false,
            failed: false,
        }
    }
    /// Inflates more after the bytes not yet read; false at the end.
    fn fill(&mut self) -> bool {
        if self.done {
            return false;
        }
        self.buffer.copy_within(self.start..self.end, 0);
        self.end -= self.start;
        self.start = 0;
        let result = miniz_oxide::inflate::stream::inflate(
            &mut self.state,
            self.input,
            &mut self.buffer[self.end..],
            miniz_oxide::MZFlush::None,
        );
        self.input = &self.input[result.bytes_consumed..];
        self.end += result.bytes_written;
        match result.status {
            Ok(miniz_oxide::MZStatus::StreamEnd) => self.done = true,
            Ok(_) if result.bytes_written > 0 || result.bytes_consumed > 0 => {}
            // No progress, or an error: a stream that ends before its end.
            _ => {
                self.failed = true;
                self.done = true;
            }
        }
        true
    }
    /// How many bytes the next block holds, `None` at the end.
    fn block(&mut self) -> Option<usize> {
        self.start = self.end;
        while self.start == self.end {
            if !self.fill() {
                return None;
            }
        }
        Some(self.end - self.start)
    }
    fn word(&mut self) -> Option<u32> {
        while self.end - self.start < 4 {
            if !self.fill() {
                return None;
            }
        }
        let word = u32::from_le_bytes(self.buffer[self.start..self.start + 4].try_into().unwrap());
        self.start += 4;
        Some(word)
    }
}

/// Gives the sums that need one a slot, in order, and rewrites the built
/// plan's sum ids (n_wires + index, u32::MAX for none) into slots, CHAIN
/// and NONE (see Sum). A sum needs no slot when an expression does not read
/// it and its only reader is the next sum of its own step. The steps' sums
/// are consecutive and in order, as the build appends them.
fn assign_slots(n_wires: usize, sums: &mut [Sum], ops: &mut [Op]) -> usize {
    let id = |previous: u32| (previous != u32::MAX).then(|| previous as usize - n_wires);
    // Readers per sum, two meaning more than one or an expression.
    let mut readers = vec![0u8; sums.len()];
    for sum in sums.iter() {
        if let Some(previous) = id(sum.previous) {
            readers[previous] = readers[previous].saturating_add(1).min(2);
        }
    }
    for op in ops.iter() {
        if let Op::Constraint(step) = op {
            for expression in &step.expressions {
                if let Expression::Wire(wire) = expression {
                    if *wire as usize >= n_wires {
                        readers[*wire as usize - n_wires] = 2;
                    }
                }
            }
        }
    }
    let mut slot_of = vec![u32::MAX; sums.len()];
    let mut n_slots = 0;
    for op in ops.iter_mut() {
        let Op::Constraint(step) = op else { continue };
        step.first_slot = n_slots as u32;
        let range = step.sums();
        for index in range.clone() {
            let chained = readers[index] == 1
                && index + 1 < range.end
                && id(sums[index + 1].previous) == Some(index);
            if !chained {
                slot_of[index] = (n_wires + n_slots) as u32;
                n_slots += 1;
            }
        }
    }
    for (index, sum) in sums.iter_mut().enumerate() {
        let previous = match id(sum.previous) {
            None => NONE,
            Some(previous) if slot_of[previous] == u32::MAX => CHAIN,
            Some(previous) => slot_of[previous],
        };
        let stored = if slot_of[index] == u32::MAX { 0 } else { STORED };
        sum.previous = previous | stored;
    }
    for op in ops.iter_mut() {
        if let Op::Constraint(step) = op {
            for expression in &mut step.expressions {
                if let Expression::Wire(wire) = expression {
                    if *wire as usize >= n_wires {
                        *wire = slot_of[*wire as usize - n_wires];
                    }
                }
            }
        }
    }
    n_slots
}

/// A u32 range of a plan's list, as usize.
fn range32(range: Range<usize>) -> (u32, u32) {
    (range.start as u32, range.end as u32)
}
fn range_of((start, end): (u32, u32)) -> Range<usize> {
    start as usize..end as usize
}
// "ZSP1" and the solver image's version: a kernel reads the images its own
// version wrote.
const IMAGE_MAGIC: usize = 0x3150_535a;
pub(crate) const IMAGE_VERSION: usize = 2;
const DAMAGED_IMAGE: &str = "damaged solver image";

// A cursor over a solver image's little-endian words.
struct Reader<'a> {
    image: &'a [u8],
    at: usize,
}
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> Result<&[u8], &'static str> {
        let end = self.at.checked_add(n).ok_or(DAMAGED_IMAGE)?;
        let bytes = self.image.get(self.at..end).ok_or(DAMAGED_IMAGE)?;
        self.at = end;
        Ok(bytes)
    }
    fn word(&mut self) -> Result<usize, &'static str> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().unwrap()) as usize)
    }
    fn words<const N: usize>(&mut self) -> Result<[usize; N], &'static str> {
        let mut out = [0; N];
        for v in &mut out {
            *v = self.word()?;
        }
        Ok(out)
    }
}
// The first word of a hint record; a constraint record starts with its ID.
const HINT_RECORD: usize = u32::MAX as usize;

// The gnark hints the plan evaluates, by the kind the bridge encodes.
#[derive(Clone, Copy)]
enum HintKind {
    // constraint/solver.InvZeroHint: the inverse, and zero for zero.
    InvZero,
    // std/math/bits.nBits: the low bits of the input, least significant first.
    Bits,
}
struct Hint {
    kind: HintKind,
    input: (u32, u32),
    outputs: (u32, u32),
}
impl Hint {
    fn input(&self) -> Range<usize> {
        range_of(self.input)
    }
    fn outputs(&self) -> Range<usize> {
        range_of(self.outputs)
    }
}
enum Op {
    Constraint(Step),
    Hint(Hint),
}
pub(crate) struct Plan {
    n_wires: usize,
    n_inputs: usize,
    pub(crate) n_public: usize,
    pub(crate) a_indices: Vec<u32>,
    pub(crate) b_indices: Vec<u32>,
    coefficients: Vec<Fr>,
    inverses: Inverses,
    terms: Vec<Term>,
    sums: super::Store<Sum>,
    /// The stored sums' slots, after the wires in the solution's vector.
    n_slots: usize,
    n_constraints: usize,
    ops: Vec<Op>,
    /// Native dot-product grouping: zero, pairs, or eight lanes.
    fuse_sums: u8,
    /// The ops in epochs of parallel tasks, built at the first solve.
    schedule: std::sync::OnceLock<Schedule>,
    definite: std::sync::OnceLock<bool>,
}

/// Bands of dependency depth (epochs), each a list of tasks: the connected
/// components of the epoch's ops, whose inputs earlier epochs or earlier
/// ops of the same task produce. Tasks of an epoch run in parallel, each in
/// program order, which is a topological order. gnark's own levels are too
/// narrow for this (a median of 16 ops at 8 -> 2 over 8036 levels), but
/// the circuits' notes and hashes are largely independent until late.
struct Schedule {
    epochs: Vec<Vec<Vec<u32>>>,
    // A replacement for one oversized serial component. Local waves join
    // before advancing; the component's external inputs are already ready.
    refined: FxMap<usize, Vec<Vec<Vec<u32>>>>,
    _reservation: Option<crate::preparation::Reservation>,
}

/// Raw access to the solution's vectors for the parallel solve: every op
/// writes only its own output wires, its own sums and its own constraint's
/// row, and reads only what earlier epochs or its task wrote.
#[derive(Clone, Copy)]
struct View {
    w: *mut Fr,
    a: *mut Fr,
    b: *mut Fr,
    c: *mut Fr,
}
// SAFETY: see View; the schedule keeps every write disjoint and ordered
// before its reads.
unsafe impl Send for View {}
unsafe impl Sync for View {}
fn fused_sums_mode() -> u8 {
    std::env::var("ZOPRO_CPU_FUSE_SUMS")
        .ok()
        .and_then(|v| v.parse::<u8>().ok())
        .unwrap_or(if crate::simd::cpu_option("ZOPRO_CPU_FUSE_SUMS") { 2 } else { 0 })
        .min(2)
}

/// Epochs of the parallel solve (ZOPRO_SOLVE_EPOCHS; 0 solves on one
/// thread).
fn solve_epochs() -> usize {
    #[cfg(not(target_arch = "wasm32"))]
    if let Some(epochs) = std::env::var("ZOPRO_SOLVE_EPOCHS")
        .ok()
        .and_then(|v| v.parse().ok())
    {
        return epochs;
    }
    SOLVE_EPOCHS
}
/// Measured best: 64 in Chrome on an M5 Pro at four threads (8 -> 2 57 ->
/// 22 ms, 24 -> 2 163 -> 59), 16 natively, where a phone's little cores
/// hold up each epoch's last tasks (a Seeker at 64 solved 2 -> 2 in 71 ms
/// against 28 on one thread; at 16 in 17, 8 -> 2 93 -> 59, 24 -> 2 269 ->
/// 160).
const SOLVE_EPOCHS: usize = if cfg!(target_arch = "wasm32") { 64 } else { 16 };

/// The op before `op` that writes `wire`, if one does. `producer` holds the
/// last writer, and wires are written once.
fn producer_before(producer: &[u32], wire: usize, op: usize) -> Option<usize> {
    let p = producer[wire];
    (p != u32::MAX && (p as usize) < op).then_some(p as usize)
}

impl Plan {
    /// The plan's sizes in memory, for the trace build.
    #[allow(dead_code)]
    pub(crate) fn footprint(&self) -> String {
        use std::mem::size_of;
        let bytes = [
            (
                "indices",
                (self.a_indices.len() + self.b_indices.len()) * size_of::<u32>(),
            ),
            ("coefficients", self.coefficients.len() * size_of::<Fr>()),
            ("terms", self.terms.len() * size_of::<Term>()),
            ("sums owned", self.sums.owned_bytes().unwrap()),
            ("ops", self.ops.len() * size_of::<Op>()),
            ("inverse values", self.inverses.bytes()),
        ];
        let parts: Vec<String> = bytes
            .iter()
            .map(|(name, b)| format!("{name} {} MB", b >> 20))
            .collect();
        format!(
            "{} (terms {}, sums {}, ops {} of {} B, coefficients {})",
            parts.join(", "),
            self.terms.len(),
            self.sums.len(),
            self.ops.len(),
            size_of::<Op>(),
            self.coefficients.len()
        )
    }
}

pub(crate) struct Solution {
    pub(crate) w: Vec<Fr>,
    pub(crate) a: Vec<Fr>,
    pub(crate) b: Vec<Fr>,
    pub(crate) c: Vec<Fr>,
}

pub(crate) struct LinearCTerm {
    pub row: usize,
    pub wire: usize,
    pub coefficient: Fr,
}
pub(crate) struct LinearC {
    pub n_wires: usize,
    pub n_public: usize,
    pub n_values: usize,
    pub terms: Vec<LinearCTerm>,
}

impl Plan {
    // Images from older builds may contain many copies of a fixed constant.
    // Preserve the first five special coefficients and rewrite every direct
    // coefficient reference; sum slots and wire identities do not change.
    fn compact_coefficients(&mut self) {
        let copy = if matches!(self.sums, super::Store::Mapped { .. }) {
            self.sums.len().checked_mul(core::mem::size_of::<Sum>())
        } else { Some(0) };
        let peak = self.coefficients.len().checked_mul(256)
            .and_then(|bytes| bytes.checked_add(copy?));
        let reservation = crate::preparation::Reservation::acquire(self.n_constraints, peak);
        if reservation.is_none() || !crate::simd::cpu_option("ZOPRO_CPU_COMPACT_COEFFS") {
            return;
        }
        let mut ids: FxMap<Fr, u32> = FxMap::default();
        let mut values = Vec::with_capacity(self.coefficients.len().min(8192));
        let mut remap = Vec::with_capacity(self.coefficients.len());
        for &value in &self.coefficients {
            let index = *ids.entry(value).or_insert_with(|| {
                let index = values.len() as u32;
                values.push(value);
                index
            });
            remap.push(index);
        }
        if values.len() == self.coefficients.len() {
            return;
        }
        debug_assert_eq!(&values[..5], &self.coefficients[..5]);
        for term in &mut self.terms {
            term.coefficient = remap[term.coefficient()];
        }
        for sum in self.sums.make_mut() {
            sum.term.coefficient = remap[sum.term.coefficient()];
        }
        for op in &mut self.ops {
            if let Op::Constraint(step) = op {
                for expression in &mut step.expressions {
                    if let Expression::Constant(index) = expression {
                        *index = remap[*index as usize];
                    }
                }
                if let Some(assignment) = &mut step.assignment {
                    assignment.coefficient = remap[assignment.coefficient()];
                }
            }
        }
        self.coefficients = values;
    }

    // Header: version, wires, inputs (including ONE), public (including ONE),
    // constraints, hints, A index count, B index count; followed by both index
    // lists, constraint records [ID, len(L), len(R), len(O), (coefficient,
    // wire)...] and hint records [HINT_RECORD, kind, 1, len(input),
    // (coefficient, wire)..., first output, end of outputs]. Records arrive in
    // gnark's level order, while a constraint's ID selects the FFT row.
    pub(crate) fn new(program: &[u8], coefficients: &[u8]) -> Result<Self, &'static str> {
        if !program.len().is_multiple_of(4) {
            return Err("invalid solver encoding");
        }
        let words = program.as_chunks::<4>().0.iter().map(|v| u32::from_le_bytes(*v));
        Self::build(program.len(), words, coefficients)
    }

    /// From the program deflated, as a key file keeps it, inflated a block
    /// at a time rather than whole (80 MB at 16 -> 2): once to measure it,
    /// for the bounds `build` checks, then into the plan.
    pub(crate) fn from_deflated(program: &[u8], coefficients: &[u8]) -> Result<Self, &'static str> {
        let mut measured = Deflated::new(program);
        let mut len = 0;
        while let Some(block) = measured.block() {
            len += block;
        }
        if measured.failed || !len.is_multiple_of(4) {
            return Err("the key's program is damaged");
        }
        let mut words = Deflated::new(program);
        let plan = Self::build(len, std::iter::from_fn(|| words.word()), coefficients)?;
        if words.failed {
            return Err("the key's program is damaged");
        }
        Ok(plan)
    }

    /// The plan from the program's `len` bytes as `words`.
    fn build(
        len: usize,
        words: impl Iterator<Item = u32>,
        coefficients: &[u8],
    ) -> Result<Self, &'static str> {
        if !coefficients.len().is_multiple_of(32) {
            return Err("invalid solver encoding");
        }
        struct Program(usize);
        impl Program {
            fn len(&self) -> usize {
                self.0
            }
        }
        let program = Program(len);
        let mut words = words.map(|v| v as usize);
        let mut read = || words.next().ok_or("truncated solver program");
        if read()? != 2 {
            return Err("unsupported solver program version");
        }
        let n_wires = read()?;
        let n_inputs = read()?;
        let n_public = read()?;
        let n_constraints = read()?;
        let n_hints = read()?;
        let n_a = read()?;
        let n_b = read()?;
        // Bound all allocations by the supplied encoding. Every internal wire
        // needs its own constraint, and every record occupies at least 4 words.
        if n_public == 0
            || n_public > n_inputs
            || n_inputs > n_wires
            || n_wires - n_inputs > n_constraints
            || n_constraints > program.len() / 16
            || n_hints > program.len() / 32
            || n_inputs > program.len() / 4
        {
            return Err("invalid solver dimensions");
        }
        let mut indices = |n: usize| -> Result<Vec<u32>, &'static str> {
            if n > n_wires || n > program.len() / 4 {
                return Err("invalid solver index count");
            }
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                let index = read()?;
                if index >= n_wires || out.last().is_some_and(|previous| *previous as usize >= index) {
                    return Err("invalid solver wire index");
                }
                out.push(index as u32);
            }
            Ok(out)
        };
        let a_indices = indices(n_a)?;
        let b_indices = indices(n_b)?;
        let mut coefficients = super::frs(coefficients);
        if coefficients.len() < 5
            || coefficients[..5] != [Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE, -Fr::from(2u64)]
            || coefficients.iter().any(|c| !c.is_reduced())
        {
            return Err("invalid solver coefficients");
        }
        let n_coefficients = coefficients.len();
        let compact_reservation = crate::preparation::Reservation::acquire(n_constraints, n_constraints.checked_mul(4).and_then(|n| n.checked_add(n_coefficients)).and_then(|n| n.checked_mul(256)));
        let compact = compact_reservation.is_some() && crate::simd::cpu_option("ZOPRO_CPU_COMPACT_COEFFS");
        let mut constant_ids: FxMap<Fr, u32> = FxMap::default();
        if compact {
            for (i, &value) in coefficients.iter().enumerate() {
                constant_ids.entry(value).or_insert(i as u32);
            }
        }
        let constant_index =
            |value: Fr, original: Option<usize>, coefficients: &mut Vec<Fr>, ids: &mut FxMap<Fr, u32>| -> usize {
                if compact {
                    *ids.entry(value).or_insert_with(|| {
                        let index = coefficients.len() as u32;
                        coefficients.push(value);
                        index
                    }) as usize
                } else if let Some(index) = original {
                    // A single source constant already has the required value
                    // and identity. Keep it without interning or appending.
                    index
                } else if coefficients.last() == Some(&value) {
                    // Reuse an adjacent generated constant without a map or
                    // extra storage when full coefficient interning is off.
                    coefficients.len() - 1
                } else {
                    let index = coefficients.len();
                    coefficients.push(value);
                    index
                }
            };
        let mut solved = vec![false; n_wires];
        solved[..n_inputs].fill(true);
        let mut seen = vec![false; n_constraints];
        let mut terms = Vec::new();
        let mut sums = Vec::new();
        let mut prefixes: FxMap<(u32, u32, u32), u32> = FxMap::default();
        let mut inverses = Inverses::default();
        let mut ops = Vec::with_capacity(n_constraints + n_hints);
        for _ in 0..n_constraints + n_hints {
            let constraint = read()?;
            if constraint == HINT_RECORD {
                let kind = match read()? {
                    0 => HintKind::InvZero,
                    1 => HintKind::Bits,
                    _ => return Err("unsupported hint"),
                };
                if read()? != 1 {
                    return Err("hint takes one input");
                }
                let start = terms.len();
                let mut constant = Fr::ZERO;
                let mut constant_terms = 0usize;
                let mut original_constant = 0usize;
                for _ in 0..read()? {
                    let coefficient = read()?;
                    let wire = read()?;
                    if coefficient >= n_coefficients || wire >= n_wires || !solved[wire] {
                        return Err("invalid hint input");
                    }
                    if wire == 0 {
                        constant += coefficients[coefficient];
                        constant_terms += 1;
                        original_constant = coefficient;
                    } else if coefficient != 0 {
                        terms.push(Term::new(coefficient, wire));
                    }
                }
                if constant != Fr::ZERO {
                    let index = constant_index(
                        constant, (constant_terms == 1).then_some(original_constant),
                        &mut coefficients, &mut constant_ids,
                    );
                    terms.push(Term::new(index, 0));
                }
                let outputs = read()?..read()?;
                if outputs.is_empty()
                    || outputs.end > n_wires
                    || solved[outputs.clone()].contains(&true)
                    || matches!(kind, HintKind::InvZero) && outputs.len() != 1
                    || outputs.len() > 256
                {
                    return Err("invalid hint outputs");
                }
                solved[outputs.clone()].fill(true);
                ops.push(Op::Hint(Hint {
                    kind,
                    input: range32(start..terms.len()),
                    outputs: range32(outputs),
                }));
                continue;
            }
            if constraint >= n_constraints || seen[constraint] {
                return Err("duplicate or out-of-range constraint");
            }
            seen[constraint] = true;
            let lengths = [read()?, read()?, read()?];
            let sum_start = sums.len();
            let mut expressions = [Expression::Zero, Expression::Zero, Expression::Zero];
            let mut assignment = None;
            for (side, len) in lengths.into_iter().enumerate() {
                let start = terms.len();
                let mut constant = Fr::ZERO;
                let mut constant_terms = 0usize;
                let mut original_constant = 0usize;
                for _ in 0..len {
                    let coefficient = read()?;
                    let wire = read()?;
                    if coefficient >= n_coefficients || wire >= n_wires {
                        return Err("invalid solver term");
                    }
                    if solved[wire] {
                        if wire == 0 {
                            constant += coefficients[coefficient];
                            constant_terms += 1;
                            original_constant = coefficient;
                        } else if coefficient != 0 {
                            terms.push(Term::new(coefficient, wire));
                        }
                    } else {
                        if assignment.is_some() {
                            return Err("constraint has multiple unsolved terms");
                        }
                        let inverse = match coefficient {
                            1 => Fr::ONE,
                            3 => -Fr::ONE,
                            _ => coefficients[coefficient]
                                .inverse()
                                .ok_or("unsolved term has zero coefficient")?,
                        };
                        assignment = Some(Assignment {
                            side: side as u8,
                            wire: wire as u32,
                            inverse: inverses.insert(inverse)?,
                            coefficient: coefficient as u32,
                        });
                    }
                }
                if constant != Fr::ZERO {
                    let index = constant_index(
                        constant, (constant_terms == 1).then_some(original_constant),
                        &mut coefficients, &mut constant_ids,
                    );
                    terms.push(Term::new(index, 0));
                }
                expressions[side] = match &terms[start..] {
                    [] => Expression::Zero,
                    [term] if term.wire() == 0 => Expression::Constant(term.coefficient),
                    [term] if term.coefficient() == 1 => Expression::Wire(term.wire),
                    [_] => Expression::Term(start as u32),
                    _ if terms.len() - start <= 2 => {
                        Expression::Sum(start as u32, terms.len() as u32)
                    }
                    _ => {
                        // Immutable wires let later expressions reuse sums from
                        // earlier constraints. Intern prefixes so long common
                        // linear expressions are evaluated only once per proof.
                        // Every new sum is scheduled at its first use, after
                        // all of its circuit wires have been solved.
                        let mut previous = u32::MAX;
                        for term in &terms[start..] {
                            if n_wires + sums.len() >= CHAIN as usize {
                                return Err("a solver program with too many sums");
                            }
                            previous = *prefixes
                                .entry((previous, term.coefficient, term.wire))
                                .or_insert_with(|| {
                                    let id = (n_wires + sums.len()) as u32;
                                    sums.push(Sum {
                                        previous,
                                        term: *term,
                                    });
                                    id
                                });
                        }
                        // The DAG owns its terms; repeated source expressions
                        // need not occupy the prepared key's memory.
                        terms.truncate(start);
                        Expression::Wire(previous)
                    }
                };
            }
            if let Some(assignment) = &assignment {
                solved[assignment.wire()] = true;
            }
            ops.push(Op::Constraint(Step {
                constraint: constraint as u32,
                sums: range32(sum_start..sums.len()),
                first_slot: 0,
                expressions,
                assignment,
            }));
        }
        if words.next().is_some() || solved.contains(&false) || seen.contains(&false) {
            return Err("incomplete solver program");
        }
        drop(prefixes);
        let n_slots = assign_slots(n_wires, &mut sums, &mut ops);
        let mut plan = Self {
            n_wires,
            n_inputs,
            n_public,
            a_indices,
            b_indices,
            coefficients,
            inverses,
            terms,
            sums: crate::Store::Owned(sums),
            n_slots,
            n_constraints,
            ops,
            fuse_sums: fused_sums_mode(),
            schedule: std::sync::OnceLock::new(),
            definite: std::sync::OnceLock::new(),
        };
        plan.compact_coefficients();
        Ok(plan)
    }

    // The plan as an image, for a host to keep and load without building it
    // again from the program: a header of [magic, version, wires, inputs,
    // public, constraints, A indices, B indices, coefficients, terms, sums,
    // slots, ops], then each list in order. Every value is a little-endian
    // u32 but the coefficients and inverses, four little-endian Montgomery
    // words.
    pub(crate) fn image(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(usize::try_from(self.image_len()).expect("solver image fits address space"));
        self.write_image(&mut out).expect("a valid solver image fits its encoding");
        out
    }

    pub(crate) fn image_len(&self) -> u64 {
        52 + 4 * (self.a_indices.len() as u64 + self.b_indices.len() as u64)
            + 32 * self.coefficients.len() as u64
            + 8 * self.terms.len() as u64
            + 12 * self.sums.len() as u64
            + self.ops.iter().map(|op| match op {
                Op::Hint(_) => 24u64,
                Op::Constraint(step) => if step.assignment.is_some() { 104 } else { 60 },
            }).sum::<u64>()
    }

    pub(crate) fn write_image(&self, out: &mut impl std::io::Write) -> std::io::Result<()> {
        fn word(out: &mut impl std::io::Write, value: usize) -> std::io::Result<()> {
            let value = u32::try_from(value).map_err(|_| std::io::Error::new(
                std::io::ErrorKind::InvalidInput, "solver index exceeds u32"))?;
            out.write_all(&value.to_le_bytes())
        }
        fn field(out: &mut impl std::io::Write, value: &Fr) -> std::io::Result<()> {
            for word in value.0 {
                out.write_all(&word.to_le_bytes())?;
            }
            Ok(())
        }
        for v in [
            IMAGE_MAGIC,
            IMAGE_VERSION,
            self.n_wires,
            self.n_inputs,
            self.n_public,
            self.n_constraints,
            self.a_indices.len(),
            self.b_indices.len(),
            self.coefficients.len(),
            self.terms.len(),
            self.sums.len(),
            self.n_slots,
            self.ops.len(),
        ] {
            word(out, v)?;
        }
        for v in self.a_indices.iter().chain(&self.b_indices) {
            word(out, *v as usize)?;
        }
        for v in &self.coefficients {
            field(out, v)?;
        }
        for term in &self.terms {
            word(out, term.coefficient())?;
            word(out, term.wire())?;
        }
        for sum in self.sums.iter() {
            word(out, sum.previous as usize)?;
            word(out, sum.term.coefficient())?;
            word(out, sum.term.wire())?;
        }
        for op in &self.ops {
            match op {
                Op::Hint(hint) => {
                    for v in [
                        1,
                        hint.kind as usize,
                        hint.input().start,
                        hint.input().end,
                        hint.outputs().start,
                        hint.outputs().end,
                    ] {
                        word(out, v)?;
                    }
                }
                Op::Constraint(step) => {
                    let sums = step.sums();
                    for v in [0, step.constraint(), sums.start, sums.end, step.first_slot()] {
                        word(out, v)?;
                    }
                    for expression in &step.expressions {
                        let (tag, a, b) = match expression {
                            Expression::Zero => (0, 0, 0),
                            Expression::Wire(v) => (1, *v as usize, 0),
                            Expression::Constant(v) => (2, *v as usize, 0),
                            Expression::Term(v) => (3, *v as usize, 0),
                            Expression::Sum(start, end) => (4, *start as usize, *end as usize),
                        };
                        for v in [tag, a, b] {
                            word(out, v)?;
                        }
                    }
                    match &step.assignment {
                        None => word(out, 0)?,
                        Some(assignment) => {
                            for v in [1, assignment.side(), assignment.wire(), assignment.coefficient()] {
                                word(out, v)?;
                            }
                            field(out, &self.inverses.get(assignment.inverse))?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    // A plan from `image`, with every index bounded by its list, so that a
    // damaged image is an error rather than a fault while solving. What the
    // bounds cannot see, a plan that solves the wrong witness, the host's
    // verification of every proof does.
    pub(crate) fn from_image(image: &[u8]) -> Result<Self, &'static str> {
        Self::from_image_with_owner(image, None)
    }

    pub(crate) fn from_image_with_owner(image: &[u8], owner: Option<&super::Image>) -> Result<Self, &'static str> {
        let mut r = Reader { image, at: 0 };
        if r.word()? != IMAGE_MAGIC || r.word()? != IMAGE_VERSION {
            return Err("unsupported solver image");
        }
        let [n_wires, n_inputs, n_public, n_constraints, n_a, n_b, n_coefficients, n_terms, n_sums, n_slots, n_ops] =
            r.words::<11>()?;
        // Every list must fit in what is left of the image before anything
        // is allocated for it; in u64, which a count cannot overflow.
        let needed = 4 * (n_a as u64 + n_b as u64)
            + 32 * n_coefficients as u64
            + 8 * n_terms as u64
            + 12 * n_sums as u64
            + 16 * n_ops as u64;
        if n_public == 0
            || n_public > n_inputs
            || n_inputs > n_wires
            || n_slots > n_sums
            || n_wires + n_slots >= CHAIN as usize
            || needed > (image.len() - r.at) as u64
        {
            return Err(DAMAGED_IMAGE);
        }
        let indices = |r: &mut Reader, n: usize| -> Result<Vec<u32>, &'static str> {
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                let index = r.word()?;
                if index >= n_wires || out.last().is_some_and(|previous| *previous as usize >= index) {
                    return Err(DAMAGED_IMAGE);
                }
                out.push(index as u32);
            }
            Ok(out)
        };
        let a_indices = indices(&mut r, n_a)?;
        let b_indices = indices(&mut r, n_b)?;
        let coefficients = super::frs(r.bytes(32 * n_coefficients)?);
        if coefficients.len() < 5
            || coefficients[..5] != [Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE, -Fr::from(2u64)]
            || coefficients.iter().any(|c| !c.is_reduced())
        {
            return Err(DAMAGED_IMAGE);
        }
        let term = |r: &mut Reader| -> Result<Term, &'static str> {
            let [coefficient, wire] = r.words::<2>()?;
            if coefficient >= n_coefficients || wire >= n_wires {
                return Err(DAMAGED_IMAGE);
            }
            Ok(Term::new(coefficient, wire))
        };
        let range = |start: usize, end: usize, len: usize| {
            if start <= end && end <= len {
                Ok(start..end)
            } else {
                Err(DAMAGED_IMAGE)
            }
        };
        let mut terms = Vec::with_capacity(n_terms);
        for _ in 0..n_terms {
            terms.push(term(&mut r)?);
        }
        // A stored sum adds its term to a slot an earlier one filled.
        // Prefixes already have the on-disk u32 layout. Retain the immutable
        // owner when the exact input slice is contained in it and aligned.
        // Every record is still validated below before the plan is returned.
        let mapped_sums = owner.and_then(|owner| {
            let bytes = (**owner).as_ref();
            let offset = (image.as_ptr() as usize).checked_sub(bytes.as_ptr() as usize)?;
            if offset.checked_add(image.len())? > bytes.len() { return None; }
            super::Store::<Sum>::mapped(owner, offset.checked_add(r.at)?, n_sums.checked_mul(12)?).ok()
        });
        let mut sums = Vec::with_capacity(if mapped_sums.is_some() { 0 } else { n_sums });
        let mut stored = 0;
        for _ in 0..n_sums {
            let previous = r.word()? as u32;
            let filled = n_wires..n_wires + stored;
            match previous & !STORED {
                NONE | CHAIN => {}
                slot if filled.contains(&(slot as usize)) => {}
                _ => return Err(DAMAGED_IMAGE),
            }
            stored += (previous & STORED != 0) as usize;
            let sum = Sum { previous, term: term(&mut r)? };
            if mapped_sums.is_none() { sums.push(sum); }
        }
        if stored != n_slots {
            return Err(DAMAGED_IMAGE);
        }
        let sums = mapped_sums.unwrap_or(super::Store::Owned(sums));
        let mut inverses = Inverses::default();
        let mut ops = Vec::with_capacity(n_ops);
        for _ in 0..n_ops {
            if r.word()? == 1 {
                let [kind, input_start, input_end, outputs_start, outputs_end] = r.words::<5>()?;
                let kind = match kind {
                    0 => HintKind::InvZero,
                    1 => HintKind::Bits,
                    _ => return Err(DAMAGED_IMAGE),
                };
                let input = range(input_start, input_end, n_terms)?;
                let outputs = range(outputs_start, outputs_end, n_wires)?;
                if outputs.is_empty()
                    || outputs.len() > 256
                    || matches!(kind, HintKind::InvZero) && outputs.len() != 1
                {
                    return Err(DAMAGED_IMAGE);
                }
                ops.push(Op::Hint(Hint {
                    kind,
                    input: range32(input),
                    outputs: range32(outputs),
                }));
                continue;
            }
            let [constraint, sums_start, sums_end, first_slot] = r.words::<4>()?;
            if constraint >= n_constraints {
                return Err(DAMAGED_IMAGE);
            }
            let step_sums = range(sums_start, sums_end, n_sums)?;
            let step_stored = sums[step_sums.clone()].iter().filter(|s| s.stored()).count();
            if first_slot + step_stored > n_slots {
                return Err(DAMAGED_IMAGE);
            }
            let mut expressions = [Expression::Zero, Expression::Zero, Expression::Zero];
            for expression in &mut expressions {
                let [tag, a, b] = r.words::<3>()?;
                *expression = match tag {
                    0 => Expression::Zero,
                    1 if a < n_wires + n_slots => Expression::Wire(a as u32),
                    2 if a < n_coefficients => Expression::Constant(a as u32),
                    3 if a < n_terms => Expression::Term(a as u32),
                    4 => {
                        let (start, end) = range32(range(a, b, n_terms)?);
                        Expression::Sum(start, end)
                    }
                    _ => return Err(DAMAGED_IMAGE),
                };
            }
            let assignment = match r.word()? {
                0 => None,
                1 => {
                    let [side, wire, coefficient] = r.words::<3>()?;
                    let inverse = super::frs(r.bytes(32)?)[0];
                    if side > 2
                        || wire >= n_wires
                        || coefficient >= n_coefficients
                        || !inverse.is_reduced()
                    {
                        return Err(DAMAGED_IMAGE);
                    }
                    Some(Assignment {
                        side: side as u8,
                        wire: wire as u32,
                        inverse: inverses.insert(inverse)?,
                        coefficient: coefficient as u32,
                    })
                }
                _ => return Err(DAMAGED_IMAGE),
            };
            ops.push(Op::Constraint(Step {
                constraint: constraint as u32,
                sums: range32(step_sums),
                first_slot: first_slot as u32,
                expressions,
                assignment,
            }));
        }
        if r.at != image.len() {
            return Err(DAMAGED_IMAGE);
        }
        let mut plan = Self {
            n_wires,
            n_inputs,
            n_public,
            a_indices,
            b_indices,
            coefficients,
            inverses,
            terms,
            sums,
            n_slots,
            n_constraints,
            ops,
            fuse_sums: fused_sums_mode(),
            schedule: std::sync::OnceLock::new(),
            definite: std::sync::OnceLock::new(),
        };
        plan.compact_coefficients();
        Ok(plan)
    }

    pub(crate) fn matches(&self, a: usize, b: usize, k: usize, domain: usize) -> bool {
        self.a_indices.len() == a
            && self.b_indices.len() == b
            && self.n_wires - self.n_public == k
            && self.n_constraints <= domain
    }

    #[cfg(test)]
    pub(crate) fn solve(&self, witness: &[Fr]) -> Result<Solution, String> {
        self.solve_in(witness, self.n_constraints)
    }

    /// `solve`, with room in a, b and c for `domain` elements: the quotient
    /// pads them to the domain in place rather than reallocating them.
    pub(crate) fn solve_in(&self, witness: &[Fr], domain: usize) -> Result<Solution, String> {
        self.solve_in_with_slots(witness, domain, &[], true)
    }

    pub(crate) fn solve_in_with_slots(&self, witness: &[Fr], domain: usize, slots: &[usize], materialize_c: bool) -> Result<Solution, String> {
        if witness.len() != self.n_inputs - 1 {
            return Err("invalid witness size".into());
        }
        let started = crate::time::Instant::now();
        let uninitialized = self.use_uninitialized()
            && *self.definite.get_or_init(|| self.definitely_writes_all());
        let row = || if uninitialized { Vec::with_capacity(domain.max(self.n_constraints)) }
                     else { crate::zero_frs(self.n_constraints, domain) };
        let (mut w, (mut a, (mut b, mut c))) = rayon::join(
            || if uninitialized { Vec::with_capacity(self.n_wires + self.n_slots) }
               else { crate::zero_frs(self.n_wires + self.n_slots, 0) },
            || rayon::join(row, || rayon::join(row, || if materialize_c { row() } else { Vec::new() })),
        );
        let started = crate::trace("solve-allocate", started);
        if uninitialized {
            let spare = w.spare_capacity_mut();
            spare[0].write(Fr::ONE);
            for (out, value) in spare[1..self.n_inputs].iter_mut().zip(witness) {
                out.write(*value);
            }
        } else {
            w[0] = Fr::ONE;
            w[1..self.n_inputs].copy_from_slice(witness);
        }
        let view = View {
            w: w.as_mut_ptr(),
            a: a.as_mut_ptr(),
            b: b.as_mut_ptr(),
            c: if materialize_c { c.as_mut_ptr() } else { core::ptr::null_mut() },
        };
        let epochs = solve_epochs();
        if epochs == 0 || self.ops.len() < 4096 {
            for op in &self.ops {
                // SAFETY: one thread, in program order.
                unsafe { self.run(op, view)? };
            }
        } else {
            use rayon::prelude::*;
            let schedule = self.schedule.get_or_init(|| self.build_schedule(epochs));
            if schedule.epochs.is_empty() {
                for op in &self.ops {
                    // SAFETY: original program order with initialized inputs.
                    unsafe { self.run(op, view)? };
                }
            } else if schedule.refined.is_empty() {
                for epoch in &schedule.epochs {
                    epoch.par_iter().try_for_each(|task| {
                        #[cfg(zopro_resource_schedule)]
                        if self.poseidon_task(task) {
                            // SAFETY: only prepared, encoding-eligible leaves carry tags.
                            return unsafe { self.run_poseidon_task(task, view) };
                        }
                        task.iter().try_for_each(|&op| {
                            // SAFETY: untagged leaves retain their original raw indices.
                            unsafe { self.run(&self.ops[op as usize], view) }
                        })
                    })?;
                }
            } else {
                // SAFETY: both levels preserve the schedule's dependency
                // ordering and disjoint writes; all jobs join before return.
                unsafe {
                    self.execute_schedule(schedule, view)?;
                }
            }
        }
        if uninitialized {
            // SAFETY: the cached data-flow check establishes that all reads
            // follow initialization and every slot/row has exactly one writer.
            // On an error above, vector lengths remain zero and no uninitialized
            // field can be observed or dropped. All scheduled writes completed.
            unsafe {
                w.set_len(self.n_wires + self.n_slots);
                a.set_len(self.n_constraints);
                b.set_len(self.n_constraints);
                if materialize_c { c.set_len(self.n_constraints); }
            }
        }
        // Retain this proof's allocation until its MSMs finish instead of
        // copying the wires into another allocation.
        let started = crate::trace("solve-execute", started);
        let selected: Vec<_> = slots.iter().map(|&slot| w[slot]).collect();
        w.truncate(self.n_wires);
        w.extend(selected);
        crate::trace("solve-copy", started);
        Ok(Solution { w, a, b, c })
    }

    /// Conservative peak for optional linear-C/equality/scheduling metadata.
    /// Count repeated expression ranges without materializing a second circuit.
    pub(crate) fn preparation_peak(&self) -> Option<usize> {
        if self.n_constraints > crate::preparation::MAX_DOMAIN { return None; }
        let terms = self.ops.iter().try_fold(0usize, |n, op| {
            let count = if let Op::Constraint(step) = op {
                (match step.expressions[2] {
                    Expression::Zero => 0,
                    Expression::Sum(start, end) => (end - start) as usize,
                    _ => 1,
                }) + usize::from(step.assignment.as_ref().is_some_and(|a| a.side() == 2))
            } else { 0 };
            n.checked_add(count)
        })?;
        terms.checked_mul(128)?
            .checked_add(self.terms.len().checked_mul(16)?)?
            .checked_add(self.n_wires.checked_add(self.n_slots)?.checked_mul(1024)?)?
            .checked_add(self.ops.len().checked_mul(512)?)?
            .checked_add(self.n_public.checked_mul(256 * 1024)?)
    }

    /// C rows are linear in wires and retained expression-prefix slots.
    /// Include the unknown term removed from a C-side assignment.
    pub(crate) fn for_each_c(&self, mut visit: impl FnMut(usize,usize,Fr)) {
        for op in &self.ops {
            let Op::Constraint(step) = op else { continue };
            let mut emit = |wire, coefficient| {
                if coefficient != Fr::ZERO {
                    visit(step.constraint(), wire, coefficient);
                }
            };
            match &step.expressions[2] {
                Expression::Zero => (),
                Expression::Wire(wire) => emit(*wire as usize, Fr::ONE),
                Expression::Constant(coefficient) => emit(0, self.coefficients[*coefficient as usize]),
                Expression::Term(term) => {
                    let term = &self.terms[*term as usize];
                    emit(term.wire(), self.coefficients[term.coefficient()]);
                }
                Expression::Sum(start, end) => {
                    for term in &self.terms[range_of((*start, *end))] {
                        emit(term.wire(), self.coefficients[term.coefficient()]);
                    }
                }
            }
            if let Some(assignment) = &step.assignment {
                if assignment.side() == 2 {
                    emit(assignment.wire(), self.coefficients[assignment.coefficient()]);
                }
            }
        }
    }
    pub(crate) fn linear_c(&self) -> LinearC {
        let mut terms=Vec::new();
        self.for_each_c(|row,wire,coefficient|terms.push(LinearCTerm{row,wire,coefficient}));
        LinearC {n_wires:self.n_wires,n_public:self.n_public,n_values:self.n_wires+self.n_slots,terms}
    }
    pub(crate) fn basis_wires(&self)->usize {self.n_wires}
    pub(crate) fn basis_c_private(&self, private:&[Fr])->Vec<Fr> {
        assert_eq!(private.len(),self.n_wires-self.n_public);
        let mut values=vec![Fr::ZERO;self.n_constraints];
        self.for_each_c(|row,wire,c|if (self.n_public..self.n_wires).contains(&wire){values[row]+=private[wire-self.n_public]*c;});
        values
    }
    #[cfg(not(target_arch="wasm32"))]
    pub(crate) fn basis_c_digest(&self)->[u8;32] {
        use sha2::{Digest,Sha256};let mut h=Sha256::new();
        for x in [self.n_wires,self.n_public,self.n_slots,self.n_constraints]{h.update((x as u64).to_le_bytes());}
        self.for_each_c(|row,wire,c|{h.update((row as u64).to_le_bytes());h.update((wire as u64).to_le_bytes());for x in c.0{h.update(x.to_le_bytes());}});
        h.finalize().into()
    }

    fn use_uninitialized(&self) -> bool {
        std::env::var("ZOPRO_CPU_UNINIT_SOLVER").map(|v| v == "1")
            .unwrap_or_else(|_| self.n_constraints >= (1 << 18)
                && crate::simd::cpu_option("ZOPRO_CPU_UNINIT_SOLVER"))
    }

    /// Establish definite assignment independently of the image parser.
    /// Old or incomplete images keep the initialized-buffer path.
    fn definitely_writes_all(&self) -> bool {
        let peak = self.n_wires.checked_add(self.n_slots).and_then(|n| n.checked_add(self.n_constraints));
        #[cfg(zopro_resource_schedule)]
        let reservation = if cfg!(all(zopro_bounded_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))
            && std::env::var("ZOPRO_CPU_RESOURCE_SCHEDULE").as_deref() == Ok("1")
            && std::env::var("ZOPRO_CPU_BOUNDED_SCHEDULE").as_deref() == Ok("1")
        {
            crate::preparation::Reservation::analysis(peak.and_then(|bytes| bytes.checked_add(128)))
        } else {
            crate::preparation::Reservation::acquire(self.n_constraints, peak)
        };
        #[cfg(not(zopro_resource_schedule))]
        let reservation = crate::preparation::Reservation::acquire(self.n_constraints, peak);
        let Some(_reservation) = reservation else { return false };
        fn write(ready: &mut [bool], index: usize) -> bool {
            let Some(slot) = ready.get_mut(index) else { return false };
            if *slot { return false; }
            *slot = true;
            true
        }
        let mut ready = vec![false; self.n_wires + self.n_slots];
        ready[..self.n_inputs].fill(true);
        let mut rows = vec![false; self.n_constraints];
        for op in &self.ops {
            match op {
                Op::Hint(hint) => {
                    if self.terms[hint.input()].iter().any(|t| !ready[t.wire()]) {
                        return false;
                    }
                    for wire in hint.outputs() {
                        if !write(&mut ready, wire) { return false; }
                    }
                }
                Op::Constraint(step) => {
                    let mut slot = self.n_wires + step.first_slot();
                    for sum in &self.sums[step.sums()] {
                        if !ready[sum.term.wire()] { return false; }
                        if let Previous::Slot(previous) = sum.previous() {
                            if !ready[previous] { return false; }
                        }
                        if sum.stored() {
                            if !write(&mut ready, slot) { return false; }
                            slot += 1;
                        }
                    }
                    for expression in &step.expressions {
                        let initialized = match expression {
                            Expression::Zero | Expression::Constant(_) => true,
                            Expression::Wire(wire) => ready[*wire as usize],
                            Expression::Term(term) => ready[self.terms[*term as usize].wire()],
                            Expression::Sum(start, end) => self.terms[range_of((*start, *end))].iter().all(|t| ready[t.wire()]),
                        };
                        if !initialized { return false; }
                    }
                    if let Some(assignment) = &step.assignment {
                        if !write(&mut ready, assignment.wire()) { return false; }
                    }
                    if !write(&mut rows, step.constraint()) { return false; }
                }
            }
        }
        ready.iter().all(|&v| v) && rows.iter().all(|&v| v)
    }

    /// Builds the parallel solve's schedule now, at the key's load, rather
    /// than at its first proof (140 ms for 24 -> 2 on a Seeker).
    pub(crate) fn prepare(&self) { self.prepare_with_workspace(None,false); }
    pub(crate) fn prepare_with_workspace(&self,workspace:Option<usize>,exact:bool) {
        if self.use_uninitialized() {
            let started = crate::time::Instant::now();
            let eligible = self.definite.get_or_init(|| self.definitely_writes_all());
            log::info!("ZOPRO_TRACE solver-definite={eligible}");
            crate::trace("solver-definite", started);
        }
        let epochs = solve_epochs();
        if epochs > 0 && self.ops.len() >= 4096 {
            self.schedule.get_or_init(|| self.build_schedule_with_workspace(epochs,workspace,exact));
        }
    }

    /// The epochs and their tasks (see Schedule).
    fn build_schedule(&self, epochs: usize) -> Schedule {
        self.build_schedule_with_workspace(epochs,None,false)
    }
    fn build_schedule_with_workspace(&self,epochs:usize,workspace:Option<usize>,exact:bool)->Schedule {
        let mut schedule=self.build_schedule_before_poseidon(epochs,workspace,exact);
        #[cfg(zopro_resource_schedule)] self.tag_poseidon(&mut schedule);
        #[cfg(zopro_poseidon_census)] sparse_poseidon::census::emit(self,&schedule);
        schedule
    }
    fn build_schedule_before_poseidon(&self,epochs:usize,workspace:Option<usize>,exact:bool)->Schedule {
        #[cfg(all(zopro_bounded_schedule,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))]
        if std::env::var("ZOPRO_CPU_BOUNDED_SCHEDULE").as_deref()==Ok("1") {
            let local=std::env::var("ZOPRO_SOLVE_LOCAL_BANDS").ok().and_then(|v|v.parse::<usize>().ok()).unwrap_or(if crate::simd::cpu_option("ZOPRO_SOLVE_LOCAL"){8}else{0}).min(1024);
            let minimum=std::env::var("ZOPRO_SOLVE_LOCAL_MIN_OPS").ok().and_then(|v|v.parse::<usize>().ok()).unwrap_or(4096).max(32);
            if local!=1 {return (if exact {bounded::build_with_workspace(self,epochs,local,minimum,workspace)}else{bounded::build(self,epochs,local,minimum)}).unwrap_or(Schedule{epochs:Vec::new(),refined:FxMap::default(),_reservation:None});}
        }
        let Some(reservation) = crate::preparation::Reservation::acquire(self.n_constraints, self.preparation_peak()) else {
            return Schedule { _reservation: None, epochs: Vec::new(), refined: FxMap::default() };
        };
        let local = std::env::var("ZOPRO_SOLVE_LOCAL_BANDS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(if crate::simd::cpu_option("ZOPRO_SOLVE_LOCAL") { 8 } else { 0 })
            .min(1024);
        let minimum = std::env::var("ZOPRO_SOLVE_LOCAL_MIN_OPS")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(4096)
            .max(32);
        let penalty = std::env::var("ZOPRO_SOLVE_LOCAL_PENALTY")
            .ok()
            .and_then(|v| v.parse::<usize>().ok())
            .unwrap_or(128)
            .min(65536);
        let mut schedule = self.build_schedule_with_refinement(epochs, local, minimum, penalty);
        schedule._reservation = Some(reservation);
        schedule
    }

    fn build_schedule_with_refinement(
        &self,
        epochs: usize,
        local: usize,
        minimum: usize,
        penalty: usize,
    ) -> Schedule {
        let n = self.ops.len();
        // Which op writes each wire and sum.
        let mut producer = vec![u32::MAX; self.n_wires + self.n_slots];
        let mut depth = vec![0u32; n];
        let mut reads = Vec::new();
        for (i, op) in self.ops.iter().enumerate() {
            self.reads(op, &mut reads);
            depth[i] = reads
                .iter()
                .filter_map(|&wire| {
                    (producer[wire] != u32::MAX).then(|| depth[producer[wire] as usize] + 1)
                })
                .max()
                .unwrap_or(0);
            for wire in self.writes(op) {
                producer[wire] = i as u32;
            }
        }
        let deepest = depth.iter().copied().max().unwrap_or(0) as usize + 1;
        let epoch_of = |op: usize| depth[op] as usize * epochs / deepest;
        // Components of each epoch's ops, by union-find over reads of wires
        // the same epoch produces.
        let mut parent: Vec<u32> = (0..n as u32).collect();
        fn find(parent: &mut [u32], mut x: u32) -> u32 {
            while parent[x as usize] != x {
                parent[x as usize] = parent[parent[x as usize] as usize];
                x = parent[x as usize];
            }
            x
        }
        for (i, op) in self.ops.iter().enumerate() {
            self.reads(op, &mut reads);
            for &wire in &reads {
                let p = producer_before(&producer, wire, i);
                if let Some(p) = p {
                    if epoch_of(p) == epoch_of(i) {
                        let (x, y) = (find(&mut parent, i as u32), find(&mut parent, p as u32));
                        parent[x as usize] = y;
                    }
                }
            }
        }
        let mut tasks: Vec<std::collections::HashMap<u32, Vec<u32>>> = (0..epochs)
            .map(|_| std::collections::HashMap::new())
            .collect();
        for i in 0..n {
            let root = find(&mut parent, i as u32);
            tasks[epoch_of(i)].entry(root).or_default().push(i as u32);
        }
        let epochs: Vec<Vec<Vec<u32>>> = tasks
            .into_iter()
            .map(|tasks| {
                let mut tasks: Vec<Vec<u32>> = tasks.into_values().collect();
                tasks.sort_unstable_by_key(|task| std::cmp::Reverse(task.len()));
                tasks
            })
            .filter(|tasks| !tasks.is_empty())
            .collect();
        let mut refined = FxMap::default();
        if local > 0 {
            for task in epochs.iter().flatten().filter(|t| t.len() >= minimum) {
                let candidates: &[usize] = if local == 1 {
                    &[4, 8, 16, 32, 64, 128]
                } else {
                    std::slice::from_ref(&local)
                };
                let mut best = None;
                let mut best_score = task.len();
                for &bands in candidates {
                    let waves = self.refine_task(task, bands);
                    let workers = rayon::current_num_threads().div_ceil(2).max(1);
                    let score: usize = waves
                        .iter()
                        .map(|tasks| {
                            let largest = tasks.iter().map(Vec::len).max().unwrap_or(0);
                            let total: usize = tasks.iter().map(Vec::len).sum();
                            largest.max(total.div_ceil(workers)) + penalty
                        })
                        .sum();
                    if local != 1 || score < best_score {
                        best_score = score;
                        best = Some(waves);
                    }
                }
                if let Some(waves) = best {
                    if local != 1 || best_score <= task.len() - task.len() / 5 {
                        refined.insert(task[0] as usize, waves);
                    }
                }
            }
        }
        Schedule { _reservation: None, epochs, refined }
    }

    /// Split a component into local dependency bands. Parent components
    /// have no dependencies on another component in the same outer epoch.
    fn refine_task(&self, task: &[u32], bands: usize) -> Vec<Vec<Vec<u32>>> {
        debug_assert!(!task.is_empty() && bands > 0);
        let n = task.len();
        let mut producer = vec![u32::MAX; self.n_wires + self.n_slots];
        let mut depth = vec![0usize; n];
        let mut reads = Vec::new();
        for (i, &op) in task.iter().enumerate() {
            self.reads(&self.ops[op as usize], &mut reads);
            depth[i] = reads
                .iter()
                .filter_map(|&w| (producer[w] != u32::MAX).then(|| depth[producer[w] as usize] + 1))
                .max()
                .unwrap_or(0);
            for wire in self.writes(&self.ops[op as usize]) {
                producer[wire] = i as u32;
            }
        }
        let deepest = depth.iter().copied().max().unwrap_or(0) + 1;
        let bands = bands.min(deepest);
        let band = |i: usize| depth[i] * bands / deepest;
        let mut parent: Vec<usize> = (0..n).collect();
        fn find(parent: &mut [usize], mut x: usize) -> usize {
            while parent[x] != x {
                parent[x] = parent[parent[x]];
                x = parent[x];
            }
            x
        }
        for (i, &op) in task.iter().enumerate() {
            self.reads(&self.ops[op as usize], &mut reads);
            for &wire in &reads {
                let p = producer[wire] as usize;
                // Ignore external inputs and slots written/read by this op.
                if p < i && band(p) == band(i) {
                    let x = find(&mut parent, i);
                    let y = find(&mut parent, p);
                    parent[x] = y;
                }
            }
        }
        let mut tasks: Vec<FxMap<usize, Vec<u32>>> = (0..bands).map(|_| FxMap::default()).collect();
        for (i, &op) in task.iter().enumerate() {
            tasks[band(i)]
                .entry(find(&mut parent, i))
                .or_default()
                .push(op);
        }
        tasks
            .into_iter()
            .map(|tasks| {
                let mut tasks: Vec<_> = tasks.into_values().collect();
                tasks.sort_unstable_by_key(|t| std::cmp::Reverse(t.len()));
                tasks
            })
            .filter(|t| !t.is_empty())
            .collect()
    }

    /// # Safety
    /// Every read is initialized, and the schedule orders all writers
    /// before readers. Distinct tasks write disjoint wires, slots and rows.
    unsafe fn execute_schedule(&self, schedule: &Schedule, v: View) -> Result<(), String> {
        use rayon::prelude::*;
        let serial = |task: &[u32]| -> Result<(), String> {
            #[cfg(zopro_resource_schedule)]
            if self.poseidon_task(task) {
                // SAFETY: only prepared, encoding-eligible leaves carry tags.
                return unsafe { self.run_poseidon_task(task, v) };
            }
            for &op in task {
                // SAFETY: operations retain program order within a task.
                unsafe {
                    self.run(&self.ops[op as usize], v)?;
                }
            }
            Ok(())
        };
        for epoch in &schedule.epochs {
            epoch.par_iter().try_for_each(|task| {
                if let Some(waves) = schedule.refined.get(&(task[0] as usize)) {
                    for wave in waves {
                        wave.par_iter().try_for_each(|part| serial(part))?;
                    }
                    Ok(())
                } else {
                    serial(task)
                }
            })?;
        }
        Ok(())
    }

    /// The wires and sums an op reads.
    fn reads(&self, op: &Op, out: &mut Vec<usize>) {
        out.clear();
        match op {
            Op::Hint(hint) => out.extend(self.terms[hint.input()].iter().map(|t| t.wire())),
            Op::Constraint(step) => {
                for sum in &self.sums[step.sums()] {
                    out.push(sum.term.wire());
                    if let Previous::Slot(slot) = sum.previous() {
                        out.push(slot);
                    }
                }
                for expression in &step.expressions {
                    match expression {
                        Expression::Wire(wire) => out.push(*wire as usize),
                        Expression::Term(term) => out.push(self.terms[*term as usize].wire()),
                        Expression::Sum(start, end) => out.extend(
                            self.terms[range_of((*start, *end))].iter().map(|t| t.wire()),
                        ),
                        Expression::Zero | Expression::Constant(_) => {}
                    }
                }
            }
        }
    }

    /// How many of a step's sums are stored.
    fn stored(&self, step: &Step) -> usize {
        self.sums[step.sums()].iter().filter(|sum| sum.stored()).count()
    }

    /// The wires and sums an op writes.
    fn writes(&self, op: &Op) -> Vec<usize> {
        match op {
            Op::Hint(hint) => hint.outputs().collect(),
            Op::Constraint(step) => step
                .assignment
                .iter()
                .map(|assignment| assignment.wire())
                .chain(self.n_wires + step.first_slot()..self.n_wires + step.first_slot() + self.stored(step))
                .collect(),
        }
    }

    /// One op, reading and writing through `v`.
    ///
    /// # Safety
    /// `v` points at the solve's vectors, and every wire the op reads is
    /// written (by an earlier op of the same thread, or before the epoch).
    unsafe fn run(&self, op: &Op, v: View) -> Result<(), String> {
        let get = |i: usize| unsafe { *v.w.add(i) };
        let set = |i: usize, value: Fr| unsafe { *v.w.add(i) = value };
        let term_value = |term: &Term| -> Fr {
            if term.wire() == 0 {
                return self.coefficients[term.coefficient()];
            }
            match term.coefficient() {
                1 => get(term.wire()),
                2 => get(term.wire()).double(),
                3 => -get(term.wire()),
                _ => self.coefficients[term.coefficient()] * get(term.wire()),
            }
        };
        let step = match op {
            Op::Constraint(step) => step,
            Op::Hint(hint) => {
                let mut value = Fr::ZERO;
                for term in &self.terms[hint.input()] {
                    value += term_value(term);
                }
                match hint.kind {
                    HintKind::InvZero => {
                        set(hint.outputs().start, value.inverse().unwrap_or(Fr::ZERO))
                    }
                    HintKind::Bits => {
                        let bits = value.to_canonical();
                        for (i, wire) in hint.outputs().enumerate() {
                            set(wire, if (bits[i / 64] >> (i % 64)) & 1 == 1 { Fr::ONE } else { Fr::ZERO });
                        }
                    }
                }
                return Ok(());
            }
        };
        let mut slot = self.n_wires + step.first_slot();
        let mut chain = Fr::ZERO;
        let sums = &self.sums[step.sums()];
        let mut i = 0;
        while i < sums.len() {
            let first = &sums[i];
            let mut last = first;
            let term = if self.fuse_sums >= 2
                && i + 8 <= sums.len()
                && sums[i..i + 8].iter().enumerate().all(|(j, sum)| {
                    (j == 7 || !sum.stored())
                        && (j == 0 || matches!(sum.previous(), Previous::Chain))
                        && sum.term.wire() != 0
                        && sum.term.coefficient() > 4
                })
            {
                let coefficients = core::array::from_fn(|j| {
                    self.coefficients[sums[i + j].term.coefficient()]
                });
                let values = core::array::from_fn(|j| get(sums[i + j].term.wire()));
                last = &sums[i + 7];
                i += 7;
                dot8(coefficients, values)
            } else if self.fuse_sums != 0
                && !first.stored()
                && i + 1 < sums.len()
                && matches!(sums[i + 1].previous(), Previous::Chain)
                && first.term.wire() != 0
                && sums[i + 1].term.wire() != 0
                && first.term.coefficient() > 4
                && sums[i + 1].term.coefficient() > 4
            {
                last = &sums[i + 1];
                i += 1;
                Fr::sum_of_products(
                    &self.coefficients[first.term.coefficient()],
                    &get(first.term.wire()),
                    &self.coefficients[last.term.coefficient()],
                    &get(last.term.wire()),
                )
            } else {
                term_value(&first.term)
            };
            chain = match first.previous() {
                Previous::None => term,
                Previous::Chain => chain + term,
                Previous::Slot(previous) => get(previous) + term,
            };
            if last.stored() {
                set(slot, chain);
                slot += 1;
            }
            i += 1;
        }
        let mut values = [Fr::ZERO; 3];
        for (value, expression) in values.iter_mut().zip(&step.expressions) {
            *value = match expression {
                Expression::Zero => Fr::ZERO,
                Expression::Wire(wire) => get(*wire as usize),
                Expression::Constant(coefficient) => self.coefficients[*coefficient as usize],
                Expression::Term(term) => term_value(&self.terms[*term as usize]),
                Expression::Sum(start, end) => {
                    let mut sum = Fr::ZERO;
                    for term in &self.terms[range_of((*start, *end))] {
                        sum += term_value(term);
                    }
                    sum
                }
            };
        }
        let fail = || format!("constraint #{} is not satisfied", step.constraint());
        if let Some(assignment) = &step.assignment {
            let term = match assignment.side() {
                0 | 1 => {
                    let other = values[1 - assignment.side()];
                    if other == Fr::ZERO {
                        if values[2] != Fr::ZERO {
                            return Err(fail());
                        }
                        Fr::ZERO
                    } else {
                        let quotient = if other == Fr::ONE {
                            values[2]
                        } else {
                            values[2] * other.inverse().unwrap()
                        };
                        let term = quotient - values[assignment.side()];
                        values[assignment.side()] = quotient;
                        term
                    }
                }
                _ => {
                    let product = values[0] * values[1];
                    let term = product - values[2];
                    values[2] = product;
                    term
                }
            };
            set(
                assignment.wire(),
                match assignment.coefficient() {
                    1 => term,
                    3 => -term,
                    _ => term * self.inverses.get(assignment.inverse),
                },
            );
        } else if values[0] * values[1] != values[2] {
            return Err(fail());
        }
        // SAFETY: each constraint's row is written by its own op only.
        unsafe {
            *v.a.add(step.constraint()) = values[0];
            *v.b.add(step.constraint()) = values[1];
            if !v.c.is_null() {
                *v.c.add(step.constraint()) = values[2];
            }
        }
        Ok(())
    }

    fn evaluate_term(&self, term: &Term, w: &[Fr]) -> Fr {
        if term.wire() == 0 {
            return self.coefficients[term.coefficient()];
        }
        match term.coefficient() {
            1 => w[term.wire()],
            2 => w[term.wire()].double(),
            3 => -w[term.wire()],
            _ => self.coefficients[term.coefficient()] * w[term.wire()],
        }
    }
}

// A linear-expression dot product in the host's R=2^256 representation.
fn dot8(coefficients: [Fr; 8], values: [Fr; 8]) -> Fr {
    #[cfg(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    ))]
    {
        use zopro_bn254::{ifma8::Ifma8, FrConfig};
        return Ifma8::<FrConfig, true>::new(&coefficients)
            .mul(&Ifma8::new(&values))
            .horizontal_sum();
    }
    #[cfg(not(all(
        feature = "ifma8",
        target_arch = "x86_64",
        target_feature = "avx512ifma",
        target_feature = "avx512vl",
        target_feature = "avx512dq",
        not(no_ifma)
    )))]
    {
        (0..8).step_by(2).map(|i| {
            Fr::sum_of_products(&coefficients[i], &values[i], &coefficients[i + 1], &values[i + 1])
        }).fold(Fr::ZERO, |acc, x| acc + x)
    }
}

impl Plan {
    pub(crate) fn private_value_indices(&self, slots: &[usize]) -> Vec<u32> {
        (self.n_public..self.n_wires)
            .chain(slots.iter().copied())
            .map(|i| i as u32)
            .collect()
    }
    pub(crate) fn value_count(&self) -> usize {
        self.n_wires + self.n_slots
    }
    pub(crate) fn equal_wire_groups(&self) -> Vec<[usize; 4]> {
        // This optional metadata requires unique, initialized producers.
        // Incomplete legacy images retain the original MSM path.
        if !*self.definite.get_or_init(|| self.definitely_writes_all()) {
            return Vec::new();
        }

        fn term(key: &mut Vec<u32>, t: &Term) {
            key.extend([t.coefficient, (t.wire() == 0) as u32]);
        }
        fn expr(key: &mut Vec<u32>, e: &Expression, p: &Plan) {
            match e {
                Expression::Zero => key.push(0),
                Expression::Wire(w) => key.extend([1, (*w == 0) as u32]),
                Expression::Constant(c) => key.extend([2, *c]),
                Expression::Term(t) => {
                    key.push(3);
                    term(key, &p.terms[*t as usize]);
                }
                Expression::Sum(a, b) => {
                    key.extend([4, b - a]);
                    for t in &p.terms[*a as usize..*b as usize] {
                        term(key, t);
                    }
                }
            }
        }
        let schedule = self.build_schedule_with_refinement(16, 0, 4096, 128);
        let mut blocks = Vec::<[usize; 4]>::new();
        let mut seen = vec![false; self.n_wires + self.n_slots];
        for epoch in &schedule.epochs {
            let mut classes: FxMap<Vec<u32>, Vec<&Vec<u32>>> = FxMap::default();
            for task in epoch {
                let mut key = vec![task.len() as u32];
                for &i in task {
                    match &self.ops[i as usize] {
                        Op::Hint(h) => {
                            key.extend([
                                0,
                                match h.kind {
                                    HintKind::InvZero => 0,
                                    HintKind::Bits => 1,
                                },
                                h.input().len() as u32,
                                h.outputs().len() as u32,
                            ]);
                            for t in &self.terms[h.input()] {
                                term(&mut key, t);
                            }
                        }
                        Op::Constraint(s) => {
                            key.extend([1, s.sums().len() as u32]);
                            for sum in &self.sums[s.sums()] {
                                key.extend([
                                    match sum.previous() {
                                        Previous::None => 0,
                                        Previous::Chain => 1,
                                        Previous::Slot(_) => 2,
                                    },
                                    sum.stored() as u32,
                                ]);
                                term(&mut key, &sum.term);
                            }
                            for e in &s.expressions {
                                expr(&mut key, e, self);
                            }
                            match &s.assignment {
                                None => key.push(0),
                                Some(a) => key.extend([1, a.side as u32, a.coefficient]),
                            }
                        }
                    }
                }
                classes.entry(key).or_default().push(task);
            }
            for mut tasks in classes.into_values() {
                tasks.sort_unstable_by_key(|t| t[0]);
                for group in tasks.chunks_exact(4) {
                    for i in 0..group[0].len() {
                        let wires: [Vec<_>; 4] =
                            core::array::from_fn(|j| self.writes(&self.ops[group[j][i] as usize]));
                        assert!(wires.iter().all(|w| w.len() == wires[0].len()));
                        for k in 0..wires[0].len() {
                            let block = core::array::from_fn(|j| wires[j][k]);
                            for &w in &block {
                                assert!(!seen[w]);
                                seen[w] = true;
                            }
                            blocks.push(block);
                        }
                    }
                }
            }
        }
        blocks
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn source_constants_and_accumulated_constants_preserve_solver_images() {
        // Single source constants retain their original index. A sum of two
        // constants still needs the generated-constant path when interning is off.
        for split_seven in [false, true] {
            let mut words = vec![2, 2, 2, 2, 13, 0, 0, 0];
            for row in 0..12 {
                if split_seven {
                    words.extend([row, 2, 1, 1, 2, 0, 7, 0, 5, 0, 6, 0]);
                } else {
                    words.extend([row, 1, 1, 1, 5, 0, 5, 0, 6, 0]);
                }
            }
            words.extend([12, 1, 1, 1, 1, 1, 1, 0, 1, 1]);
            let coefficients = encode_fields(&[
                Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE,
                -Fr::from(2u64), Fr::from(7u64), Fr::from(49u64), Fr::from(5u64),
            ]);
            let plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
            assert!(plan.coefficients.len() <= 9);
            if !crate::simd::cpu_option("ZOPRO_CPU_COMPACT_COEFFS") {
                assert_eq!(plan.coefficients.len(), if split_seven { 9 } else { 8 });
            }
            let image = plan.image();
            for plan in [plan, Plan::from_image(&image).unwrap()] {
                assert_eq!(plan.image(), image);
                for y in [0u64, 9, 123] {
                    let y = Fr::from(y);
                    let solution = plan.solve(&[y]).unwrap();
                    assert_eq!(solution.w, [Fr::ONE, y]);
                    assert_eq!(&solution.a[..12], &[Fr::from(7u64); 12]);
                    assert_eq!(&solution.b[..12], &[Fr::from(7u64); 12]);
                    assert_eq!(&solution.c[..12], &[Fr::from(49u64); 12]);
                    assert_eq!((solution.a[12], solution.b[12], solution.c[12]),
                        (y, Fr::ONE, y));
                }
            }
        }
    }

    #[test]
    fn source_constants_in_hints_preserve_inverse_witness() {
        for split_seven in [false, true] {
            let mut words = vec![2, 2, 1, 1, 1, 1, 1, 0, 1];
            if split_seven {
                words.extend([u32::MAX, 0, 1, 2, 2, 0, 7, 0, 1, 2]);
            } else {
                words.extend([u32::MAX, 0, 1, 1, 5, 0, 1, 2]);
            }
            // InvZero(7) * 7 = 1, checking the generated hint wire.
            words.extend([0, 1, 1, 1, 1, 1, 5, 0, 1, 0]);
            let coefficients = encode_fields(&[
                Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE,
                -Fr::from(2u64), Fr::from(7u64), Fr::from(49u64), Fr::from(5u64),
            ]);
            let plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
            if !crate::simd::cpu_option("ZOPRO_CPU_COMPACT_COEFFS") {
                assert_eq!(plan.coefficients.len(), if split_seven { 9 } else { 8 });
            }
            let image = plan.image();
            for plan in [plan, Plan::from_image(&image).unwrap()] {
                assert_eq!(plan.image(), image);
                let solution = plan.solve(&[]).unwrap();
                let inverse = Fr::from(7u64).inverse().unwrap();
                assert_eq!(solution.w, [Fr::ONE, inverse]);
                assert_eq!(solution.a, [inverse]);
                assert_eq!(solution.b, [Fr::from(7u64)]);
                assert_eq!(solution.c, [Fr::ONE]);
            }
        }
    }

    #[test]
    fn mapped_prefixes_validate_roundtrip_and_keep_the_owner_alive() {
        let (words, coefficients) = with_sums();
        let reference = Plan::new(&encode_words(&words), &coefficients).unwrap();
        let image = reference.image();
        let witness = [Fr::from(5u64), -Fr::from(5u64), Fr::from(25u64)];
        let expected = reference.solve(&witness).unwrap();
        let mut backing = vec![0u8; image.len() + 16];
        let offset = (4 - backing.as_ptr() as usize % 4) % 4;
        backing[offset..offset + image.len()].copy_from_slice(&image);
        let owner: super::super::Image = std::sync::Arc::new(backing);
        let bytes = owner.as_ref().as_ref();
        let mut mapped = Plan::from_image_with_owner(&bytes[offset..offset + image.len()], Some(&owner)).unwrap();
        assert_eq!(matches!(mapped.sums, super::super::Store::Mapped { .. }), cfg!(target_endian = "little"));
        if cfg!(target_endian = "little") { assert_eq!(mapped.sums.owned_bytes(), Some(0)); }
        assert_eq!(mapped.image(), image);
        let got = mapped.solve(&witness).unwrap();
        assert_eq!(got.w, expected.w);
        assert_eq!(got.a, expected.a);
        assert_eq!(got.b, expected.b);
        assert_eq!(got.c, expected.c);
        // Rewrites get private storage and preserve the immutable image.
        mapped.sums.make_mut();
        assert!(matches!(mapped.sums, super::super::Store::Owned(_)));
        assert_eq!(mapped.image(), image);
        assert_eq!(&owner.as_ref().as_ref()[offset..offset + image.len()], &image);
        let mapped = Plan::from_image_with_owner(&owner.as_ref().as_ref()[offset..offset + image.len()], Some(&owner)).unwrap();
        drop(owner);
        assert_eq!(mapped.solve(&witness).unwrap().w, expected.w);
        assert_eq!(mapped.image(), image);
    }

    #[test]
    fn mapped_prefixes_reject_damage_and_fall_back_for_unaligned_or_unrelated_owners() {
        let (words, coefficients) = with_sums();
        let plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
        let image = plan.image();
        let owner: super::super::Image = std::sync::Arc::new(image.clone());
        for end in 0..image.len() {
            assert!(Plan::from_image_with_owner(&owner.as_ref().as_ref()[..end], Some(&owner)).is_err());
        }
        let offset = 52 + 4 * (plan.a_indices.len() + plan.b_indices.len())
            + 32 * plan.coefficients.len() + 8 * plan.terms.len();
        let mut damaged = image.clone();
        damaged[offset + 8..offset + 12].copy_from_slice(&u32::MAX.to_le_bytes());
        let damaged: super::super::Image = std::sync::Arc::new(damaged);
        assert!(Plan::from_image_with_owner(damaged.as_ref().as_ref(), Some(&damaged)).is_err());
        let mut backing = vec![0u8; image.len() + 16];
        let offset = (4 - backing.as_ptr() as usize % 4) % 4 + 1;
        backing[offset..offset + image.len()].copy_from_slice(&image);
        let unaligned: super::super::Image = std::sync::Arc::new(backing);
        let p = Plan::from_image_with_owner(&unaligned.as_ref().as_ref()[offset..offset + image.len()], Some(&unaligned)).unwrap();
        assert!(matches!(p.sums, super::super::Store::Owned(_)));
        assert_eq!(p.image(), image);
        let p = Plan::from_image_with_owner(&image, Some(&owner)).unwrap();
        assert!(matches!(p.sums, super::super::Store::Owned(_)));
        assert_eq!(p.image(), image);
    }

    #[test]
    fn inverse_chunks_preserve_values_and_bound_capacity() {
        let mut pool = Inverses::default();
        assert_eq!(pool.insert(Fr::ONE).unwrap(), INVERSE_ONE);
        assert_eq!(pool.insert(-Fr::ONE).unwrap(), INVERSE_MINUS_ONE);
        assert!(pool.chunks.is_empty());
        let mut expected = vec![Fr::ZERO];
        expected.extend((2..=1025u64).map(Fr::from));
        expected.extend((2..=1025u64).map(|v| -Fr::from(v)));
        for (i, &value) in expected.iter().enumerate() {
            assert_eq!(pool.insert(value).unwrap(), i as u32);
            assert_eq!(pool.len, i + 1);
            assert_eq!(pool.chunks.len(), (i + 1).div_ceil(INVERSE_CHUNK));
            assert!(pool.chunks.capacity() <= (2 * pool.chunks.len()).max(4));
        }
        for (i, &value) in expected.iter().enumerate() { assert_eq!(pool.get(i as u32), value); }
        assert_eq!(pool.get(INVERSE_ONE), Fr::ONE);
        assert_eq!(pool.get(INVERSE_MINUS_ONE), -Fr::ONE);
        assert_eq!(core::mem::size_of::<Assignment>(), 16);
        if cfg!(target_pointer_width = "64") {
            assert_eq!(core::mem::size_of::<Op>(), 72);
        } else {
            assert!(core::mem::size_of::<Op>() <= 72);
        }
        pool.len = INVERSE_MINUS_ONE as usize;
        assert!(pool.insert(Fr::ZERO).is_err());
    }

    #[test]
    fn arbitrary_assignment_coefficients_solve_and_roundtrip() {
        for k in [Fr::from(2u64), -Fr::from(7u64), Fr::from(999u64), Fr::ONE, -Fr::ONE] {
            // (k * internal X) * ONE = public Y.
            let words = [2,3,2,2,1,0,0,0, 0,1,1,1, 5,2, 1,0, 1,1];
            let coefficients = encode_fields(&[Fr::ZERO,Fr::ONE,Fr::from(2u64),-Fr::ONE,-Fr::from(2u64),k]);
            let plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
            let image = plan.image();
            for plan in [plan,Plan::from_image(&image).unwrap()] {
                assert_eq!(plan.image(), image);
                for value in [0,1,37,1001u64] {
                    let x = Fr::from(value);let solution = plan.solve(&[k*x]).unwrap();
                    assert_eq!(solution.w,[Fr::ONE,k*x,x]);
                    assert_eq!(solution.a,[k*x]);assert_eq!(solution.b,[Fr::ONE]);assert_eq!(solution.c,[k*x]);
                }
            }
        }
    }

    #[test]
    fn image_preserves_arbitrary_reduced_inverse_payloads() {
        let words = [2,3,2,2,1,0,0,0, 0,1,1,1, 5,2, 1,0, 1,1];
        let coefficients = encode_fields(&[Fr::ZERO,Fr::ONE,Fr::from(2u64),-Fr::ONE,-Fr::from(2u64),Fr::from(7u64)]);
        for value in [Fr::ZERO,Fr::ONE,-Fr::ONE,Fr::from(123u64),-Fr::from(456u64)] {
            let mut plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
            let index = plan.inverses.insert(value).unwrap();
            let Op::Constraint(step) = &mut plan.ops[0] else { panic!("constraint"); };
            step.assignment.as_mut().unwrap().inverse = index;
            let image = plan.image();let loaded = Plan::from_image(&image).unwrap();
            assert_eq!(loaded.image(), image);
            let Op::Constraint(step) = &loaded.ops[0] else { panic!("constraint"); };
            assert_eq!(loaded.inverses.get(step.assignment.as_ref().unwrap().inverse), value);
        }
    }



    fn encode_words(words: &[u32]) -> Vec<u8> {
        words.iter().flat_map(|v| v.to_le_bytes()).collect()
    }
    fn encode_fields(values: &[Fr]) -> Vec<u8> {
        values
            .iter()
            .flat_map(|v| v.0.iter().flat_map(|w| w.to_le_bytes()))
            .collect()
    }
    fn small() -> (Vec<u32>, Vec<u8>) {
        // ONE, public Y, secret X, internal X²; X² = Y is checked before
        // row zero in storage, but after it in dependency order.
        (
            vec![
                2, 4, 3, 2, 2, 0, 3, 2, 0, 2, 3, 2, 3, 1, 1, 1, 1, 1, 2, 1, 2, 1, 3, 0, 1, 1, 1, 1,
                3, 1, 0, 1, 1,
            ],
            encode_fields(&[Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE, -Fr::from(2u64)]),
        )
    }

    #[test]
    fn refined_components_match_sequential_execution() {
        fn plan(branches: usize) -> Plan {
            let n_wires = branches + 5;
            let mut coefficients =
                vec![Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE, -Fr::from(2u64)];
            coefficients.extend((0..branches).map(|i| Fr::from(i as u64 + 7)));
            let assignment = |wire| {
                Some(Assignment {
                    side: 2,
                    wire: wire as u32,
                    coefficient: 1,
                    inverse: INVERSE_ONE,
                })
            };
            let mut ops = vec![Op::Constraint(Step {
                constraint: 0,
                sums: (0, 0),
                first_slot: 0,
                expressions: [Expression::Wire(2), Expression::Wire(2), Expression::Zero],
                assignment: assignment(3),
            })];
            let mut sums = Vec::new();
            for i in 0..branches {
                sums.push(Sum {
                    previous: NONE | STORED,
                    term: Term::new(i + 5, 3),
                });
                ops.push(Op::Constraint(Step {
                    constraint: i as u32 + 1,
                    sums: (i as u32, i as u32 + 1),
                    first_slot: i as u32,
                    expressions: [
                        Expression::Wire((n_wires + i) as u32),
                        Expression::Constant(1),
                        Expression::Zero,
                    ],
                    assignment: assignment(i + 4),
                }));
            }
            let terms = (0..branches).map(|i| Term::new(1, i + 4)).collect();
            ops.push(Op::Constraint(Step {
                constraint: branches as u32 + 1,
                sums: (branches as u32, branches as u32),
                first_slot: branches as u32,
                expressions: [
                    Expression::Sum(0, branches as u32),
                    Expression::Constant(1),
                    Expression::Zero,
                ],
                assignment: assignment(branches + 4),
            }));
            ops.push(Op::Constraint(Step {
                constraint: branches as u32 + 2,
                sums: (branches as u32, branches as u32),
                first_slot: branches as u32,
                expressions: [
                    Expression::Wire(branches as u32 + 4),
                    Expression::Constant(1),
                    Expression::Wire(1),
                ],
                assignment: None,
            }));
            Plan {
                n_wires,
                n_inputs: 3,
                n_public: 2,
                a_indices: Vec::new(),
                b_indices: Vec::new(),
                coefficients,
                inverses: Inverses::default(),
                terms,
                sums: crate::Store::Owned(sums),
                n_slots: branches,
                n_constraints: branches + 3,
                ops,
                fuse_sums: 2,
                schedule: std::sync::OnceLock::new(),
                definite: std::sync::OnceLock::new(),
            }
        }
        fn verify(plan: &Plan, schedule: &Schedule) {
            for parent in schedule.epochs.iter().flatten() {
                let Some(waves) = schedule.refined.get(&(parent[0] as usize)) else {
                    continue;
                };
                let mut flattened: Vec<_> = waves.iter().flatten().flatten().copied().collect();
                flattened.sort_unstable();
                assert_eq!(&flattened, parent, "each original op occurs exactly once");
                let mut location = std::collections::HashMap::new();
                for (wave, tasks) in waves.iter().enumerate() {
                    for (task, ops) in tasks.iter().enumerate() {
                        for (position, &op) in ops.iter().enumerate() {
                            assert!(location.insert(op, (wave, task, position)).is_none());
                        }
                    }
                }
                let mut producers = std::collections::HashMap::new();
                for &op in parent {
                    for wire in plan.writes(&plan.ops[op as usize]) {
                        assert!(producers.insert(wire, op).is_none());
                    }
                }
                let mut reads = Vec::new();
                for &op in parent {
                    plan.reads(&plan.ops[op as usize], &mut reads);
                    for wire in &reads {
                        let Some(&producer) = producers.get(wire) else {
                            continue;
                        };
                        if producer == op {
                            continue;
                        }
                        let p = location[&producer];
                        let c = location[&op];
                        assert!(
                            p.0 < c.0 || (p.0 == c.0 && p.1 == c.1 && p.2 < c.2),
                            "a producer must complete before its reader"
                        );
                    }
                }
            }
        }
        fn run(
            plan: &Plan,
            schedule: Option<&Schedule>,
            x: Fr,
            bad: bool,
            materialize_c: bool,
        ) -> (Result<(), String>, Vec<Vec<Fr>>) {
            let weight = plan.coefficients[5..]
                .iter()
                .copied()
                .fold(Fr::ZERO, |a, b| a + b);
            let mut w = vec![Fr::ZERO; plan.n_wires + plan.n_slots];
            w[0] = Fr::ONE;
            w[1] = x * x * weight + if bad { Fr::ONE } else { Fr::ZERO };
            w[2] = x;
            let mut a = vec![Fr::ZERO; plan.n_constraints];
            let mut b = a.clone();
            let mut c = if materialize_c { a.clone() } else { Vec::new() };
            let view = View {
                w: w.as_mut_ptr(),
                a: a.as_mut_ptr(),
                b: b.as_mut_ptr(),
                c: if materialize_c {
                    c.as_mut_ptr()
                } else {
                    core::ptr::null_mut()
                },
            };
            let result = if let Some(schedule) = schedule {
                // SAFETY: the checked schedule owns all writes and orders dependencies.
                unsafe { plan.execute_schedule(schedule, view) }
            } else {
                let mut result = Ok(());
                for op in &plan.ops {
                    // SAFETY: the synthetic plan is in dependency order.
                    if let Err(error) = unsafe { plan.run(op, view) } {
                        result = Err(error);
                        break;
                    }
                }
                result
            };
            (result, vec![w, a, b, c])
        }
        let plan = plan(128);
        assert!(plan.definitely_writes_all());
        for outer in [1, 2, 4] {
            for local in [0, 1, 2, 4, 8, 64] {
                let schedule = plan.build_schedule_with_refinement(outer, local, 16, 0);
                verify(&plan, &schedule);
                if outer == 1 && local >= 2 {
                    assert!(!schedule.refined.is_empty());
                }
                for threads in [1, 4, 16] {
                    let pool = rayon::ThreadPoolBuilder::new()
                        .num_threads(threads)
                        .build()
                        .unwrap();
                    for x in [Fr::ZERO, Fr::ONE, -Fr::ONE, Fr::from(1234567u64)] {
                        for materialize_c in [false, true] {
                            let expected = run(&plan, None, x, false, materialize_c);
                            assert!(expected.0.is_ok());
                            let actual = pool
                                .install(|| run(&plan, Some(&schedule), x, false, materialize_c));
                            assert_eq!(actual, expected);
                            assert!(pool
                                .install(|| run(&plan, Some(&schedule), x, true, materialize_c))
                                .0
                                .is_err());
                        }
                    }
                }
            }
        }
    }

    /// The parallel solve of a dumped circuit (ZOPRO_DUMP=<dir>, every
    /// shape with a witness) against the one-thread solve, wire for wire.
    #[test]
    fn parallel_solve_matches_the_sequential_one() {
        let Ok(dump) = std::env::var("ZOPRO_DUMP") else {
            return;
        };
        for entry in std::fs::read_dir(dump).unwrap() {
            let dir = entry.unwrap().path();
            let (Ok(program), Ok(coefficients), Ok(witness)) = (
                std::fs::read(dir.join("program")),
                std::fs::read(dir.join("coefficients")),
                std::fs::read(dir.join("witness")),
            ) else {
                continue;
            };
            let plan = Plan::new(&program, &coefficients).unwrap();
            let witness = crate::decode_frs(&witness);
            let view = |solution: &Solution| {
                [
                    solution.w.clone(),
                    solution.a.clone(),
                    solution.b.clone(),
                    solution.c.clone(),
                ]
            };
            let parallel = view(&plan.solve(&witness).unwrap());
            let sequential = {
                let mut w = vec![Fr::ZERO; plan.n_wires + plan.n_slots];
                w[0] = Fr::ONE;
                w[1..plan.n_inputs].copy_from_slice(&witness);
                let (mut a, mut b, mut c) = (
                    vec![Fr::ZERO; plan.n_constraints],
                    vec![Fr::ZERO; plan.n_constraints],
                    vec![Fr::ZERO; plan.n_constraints],
                );
                let v = View {
                    w: w.as_mut_ptr(),
                    a: a.as_mut_ptr(),
                    b: b.as_mut_ptr(),
                    c: c.as_mut_ptr(),
                };
                for op in &plan.ops {
                    unsafe { plan.run(op, v).unwrap() };
                }
                w.truncate(plan.n_wires);
                [w, a, b, c]
            };
            assert!(
                parallel == sequential,
                "{}: the parallel solve differs",
                dir.display()
            );
            let schedule = plan.schedule.get().expect("a schedule");
            let widest = schedule
                .epochs
                .iter()
                .map(|tasks| tasks[0].len())
                .sum::<usize>();
            eprintln!(
                "{}: {} ops, critical path {} ops over {} epochs",
                dir.display(),
                plan.ops.len(),
                widest,
                schedule.epochs.len()
            );
        }
    }

    #[test]
    fn solves_and_rejects_invalid_witnesses() {
        let (words, coefficients) = small();
        let plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
        assert!(plan.matches(3, 2, 2, 2));
        assert!(!plan.matches(3, 2, 1, 2));
        for x in [Fr::ZERO, Fr::ONE, -Fr::ONE, Fr::from(123456u64)] {
            let solution = plan.solve(&[x * x, x]).unwrap();
            assert_eq!(solution.w, [Fr::ONE, x * x, x, x * x]);
            assert_eq!(solution.a, [x * x, x]);
            assert_eq!(solution.b, [Fr::ONE, x]);
            assert_eq!(solution.c, [x * x, x * x]);
            assert_eq!(
                plan.solve(&[x * x + Fr::ONE, x]).err().unwrap(),
                "constraint #0 is not satisfied"
            );
            // Failure does not contaminate the next witness.
            assert!(plan.solve(&[x * x, x]).is_ok());
        }
        assert!(plan.solve(&[]).is_err());
        assert!(plan.solve(&[Fr::ZERO; 3]).is_err());
    }

    #[test]
    fn unknown_input_with_zero_divisor() {
        let mut coefficients = small().1;
        coefficients.extend(encode_fields(&[Fr::from(7u64)]));
        for side in 0..2 {
            let mut words = vec![2, 4, 3, 1, 1, 0, 4, 4, 0, 1, 2, 3, 0, 1, 2, 3, 0];
            let mut expressions = [vec![1, 0, 5, 3], vec![1, 2], vec![1, 1]];
            if side == 1 {
                expressions.swap(0, 1);
            }
            words.extend(
                expressions
                    .iter()
                    .map(|expression| (expression.len() / 2) as u32),
            );
            for expression in expressions {
                words.extend(expression);
            }
            let plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
            let result = plan.solve(&[Fr::ZERO, Fr::ZERO]).unwrap();
            assert_eq!(result.w[3], Fr::ZERO);
            assert!(plan.solve(&[Fr::ONE, Fr::ZERO]).is_err());
            let result = plan.solve(&[Fr::from(24u64), Fr::from(3u64)]).unwrap();
            assert_eq!(result.w[3], Fr::ONE); // (1 + 7*1) * 3 = 24
        }
    }

    // ONE, X, Y, Z, then U, V and V²: a long linear expression shared by
    // three constraints, interned as sums.
    fn with_sums() -> (Vec<u32>, Vec<u8>) {
        with_linear_base(vec![2, 1, 3, 2, 5, 3])
    }

    fn with_linear_base(base: Vec<u32>) -> (Vec<u32>, Vec<u8>) {
        let mut coefficients = small().1;
        coefficients.extend(encode_fields(&[
            Fr::from(3u64),
            Fr::from(5u64),
            Fr::from(7u64),
        ]));
        let mut program = vec![
            2, 7, 4, 1, 3, 0, 7, 7, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6,
        ];
        let mut first = base.clone();
        first.extend([6, 0]);
        let mut second = base;
        second.extend([1, 4, 7, 0]);
        for (row, expressions) in [
            [first.clone(), first, vec![1, 4]],
            [second.clone(), vec![1, 0], vec![1, 5]],
            [second, vec![1, 5], vec![1, 6]],
        ]
        .into_iter()
        .enumerate()
        {
            program.push(row as u32);
            program.extend(
                expressions
                    .iter()
                    .map(|expression| (expression.len() / 2) as u32),
            );
            for expression in expressions {
                program.extend(expression);
            }
        }
        (program, coefficients)
    }

    #[test]
    fn fused_sums_preserve_stored_prefixes_and_images() {
        for terms in [3, 8, 11] {
            let base: Vec<u32> = (0..terms).flat_map(|i| [5 + i % 3, 1 + i % 3]).collect();
            let (program, coefficients) = with_linear_base(base.clone());
            let mut reference = Plan::new(&encode_words(&program), &coefficients).unwrap();
            reference.fuse_sums = 0;
            assert!(reference.sums.windows(terms as usize).any(|run| {
                run.iter().enumerate().all(|(i, s)| {
                    (i + 1 == run.len() || !s.stored())
                        && (i == 0 || matches!(s.previous(), Previous::Chain))
                        && s.term.wire() != 0 && s.term.coefficient() > 4
                })
            }));
            for mode in [1, 2] {
                let mut fused = Plan::from_image(&reference.image()).unwrap();
                fused.fuse_sums = mode;
                for sample in 0..50u64 {
                    let x = Fr::from(sample);
                    let y = -x;
                    let z = x * x;
                    let inputs = [x, y, z];
                    let weights = [Fr::from(3u64), Fr::from(5u64), Fr::from(7u64)];
                    let value: Fr = base.chunks_exact(2)
                        .map(|term| weights[(term[0] - 5) as usize] * inputs[(term[1] - 1) as usize])
                        .fold(Fr::ZERO, |acc, x| acc + x);
                    let u = (value + Fr::from(5u64)).square();
                    let v = value + u + Fr::from(7u64);
                    let expected = reference.solve(&inputs).unwrap();
                    let actual = fused.solve(&inputs).unwrap();
                    assert_eq!(actual.w, [Fr::ONE, x, y, z, u, v, v * v]);
                    assert_eq!(actual.w, expected.w);
                    assert_eq!(actual.a, expected.a);
                    assert_eq!(actual.b, expected.b);
                    assert_eq!(actual.c, expected.c);
                }
            }
        }
    }

    #[test]
    fn repeated_linear_expressions_use_fresh_witness_values() {
        let (program, coefficients) = with_sums();
        let plan = Plan::new(&encode_words(&program), &coefficients).unwrap();
        for sample in 0..50u64 {
            let x = Fr::from(sample);
            let y = -x;
            let z = x * x;
            let base = x.double() - y + Fr::from(3u64) * z;
            let u = (base + Fr::from(5u64)).square();
            let v = base + u + Fr::from(7u64);
            let solution = plan.solve(&[x, y, z]).unwrap();
            assert_eq!(solution.w, [Fr::ONE, x, y, z, u, v, v * v]);
            assert_eq!(solution.a, [base + Fr::from(5u64), v, v]);
            assert_eq!(solution.b, [base + Fr::from(5u64), Fr::ONE, v]);
            assert_eq!(solution.c, [u, v, v * v]);
        }
    }

    // ONE, secret X; hints: the two low bits of X (b0, b1) and its inverse.
    // Constraints: b0 and b1 are bits, b0 + 2 b1 = X, X * inverse = 1.
    fn with_hints() -> Vec<u32> {
        vec![
            2,
            5,
            2,
            1,
            4,
            2,
            3,
            4,
            1,
            2,
            3,
            0,
            2,
            3,
            4, // header, A and B wires
            u32::MAX,
            1,
            1,
            1,
            1,
            1,
            2,
            4, // b0, b1 = nBits(X)
            u32::MAX,
            0,
            1,
            1,
            1,
            1,
            4,
            5, // inverse = InvZeroHint(X)
            0,
            1,
            1,
            1,
            1,
            2,
            1,
            2,
            1,
            2, // b0 * b0 = b0
            1,
            1,
            1,
            1,
            1,
            3,
            1,
            3,
            1,
            3, // b1 * b1 = b1
            2,
            2,
            1,
            1,
            1,
            2,
            2,
            3,
            1,
            0,
            1,
            1, // (b0 + 2 b1) * 1 = X
            3,
            1,
            1,
            1,
            1,
            1,
            1,
            4,
            1,
            0, // X * inverse = 1
        ]
    }

    #[test]
    fn solves_bits_and_inverse_hints() {
        let coefficients = small().1;
        let plan = Plan::new(&encode_words(&with_hints()), &coefficients).unwrap();
        assert!(plan.matches(3, 4, 4, 4));
        for x in 1..4u64 {
            let x = Fr::from(x);
            let bit = |i: u32| Fr::from((x.to_canonical()[0] >> i) & 1);
            let solution = plan.solve(&[x]).unwrap();
            let inverse = x.inverse().unwrap();
            assert_eq!(solution.w, [Fr::ONE, x, bit(0), bit(1), inverse]);
            assert_eq!(solution.a[3], x);
            assert_eq!(solution.b[3], inverse);
            assert_eq!(solution.c[3], Fr::ONE);
        }
        // The inverse of zero is zero, which X * inverse = 1 rejects.
        assert_eq!(
            plan.solve(&[Fr::ZERO]).err().unwrap(),
            "constraint #3 is not satisfied"
        );
        // Omitting output rows must still reject unsatisfied constraints.
        assert_eq!(plan.solve_in_with_slots(&[Fr::ZERO], 4, &[], false).err(),
                   Some("constraint #3 is not satisfied".to_string()));
        assert_eq!(plan.solve_in_with_slots(&[Fr::from(4u64)], 4, &[], false).err(),
                   Some("constraint #2 is not satisfied".to_string()));
        // Two bits do not hold four.
        assert_eq!(
            plan.solve(&[Fr::from(4u64)]).err().unwrap(),
            "constraint #2 is not satisfied"
        );
    }

    #[test]
    fn rejects_invalid_hints() {
        let coefficients = small().1;
        for (index, value) in [
            (16, 2), // unknown hint kind
            (17, 2), // two inputs
            (20, 2), // input from an unsolved wire
            (21, 1), // outputs over a solved wire
            (22, 2), // no outputs
            (29, 6), // outputs out of order
            (5, 1),  // one hint record more than counted
        ] {
            let mut words = with_hints();
            words[index] = value;
            assert!(
                Plan::new(&encode_words(&words), &coefficients).is_err(),
                "word {index} = {value}"
            );
        }
    }

    #[test]
    fn validates_program_before_solving() {
        let (words, coefficients) = small();
        let bytes = encode_words(&words);
        for len in 0..bytes.len() {
            assert!(
                Plan::new(&bytes[..len], &coefficients).is_err(),
                "length {len}"
            );
        }
        let mut changed = words.clone();
        changed.push(0);
        assert!(Plan::new(&encode_words(&changed), &coefficients).is_err());
        for (index, value) in [
            (0, 1),
            (1, u32::MAX),
            (2, 0),
            (3, 4),
            (4, u32::MAX),
            (5, 1),
            (6, u32::MAX),
            (8, 4),
            (9, 0),
            (13, 0),
            (21, 0),
            (22, 4),
            (23, 1),
            (20, 4),
            (17, 5),
            (18, 3),
            (19, 5),
        ] {
            let mut changed = words.clone();
            changed[index] = value;
            assert!(
                Plan::new(&encode_words(&changed), &coefficients).is_err(),
                "word {index} = {value}"
            );
        }
        let mut changed = coefficients.clone();
        changed[0] = 1;
        assert!(Plan::new(&bytes, &changed).is_err());
        assert!(Plan::new(&bytes, &coefficients[..coefficients.len() - 1]).is_err());
        // Noncanonical table entries cannot enter field arithmetic.
        let mut changed = coefficients.clone();
        changed.extend(Fr::MODULUS.iter().flat_map(|w| w.to_le_bytes()));
        assert!(Plan::new(&bytes, &changed).is_err());
    }

    #[test]
    fn linear_c_metadata_matches_fresh_solver_rows() {
        let (small_words, coefficients) = small();
        let (sums_words, sums_coefficients) = with_sums();
        for (program, coefficients, witness) in [
            (small_words, coefficients.clone(), vec![Fr::from(9u64), Fr::from(3u64)]),
            (sums_words, sums_coefficients, vec![Fr::from(5u64), -Fr::from(5u64), Fr::from(25u64)]),
            (with_hints(), coefficients, vec![Fr::from(3u64)]),
        ] {
            let plan = Plan::new(&encode_words(&program), &coefficients).unwrap();
            for plan in [&plan, &reloaded(&plan)] {
                assert!(plan.definitely_writes_all());
                let slots: Vec<_> = (plan.n_wires..plan.n_wires + plan.n_slots).collect();
                let solution = plan.solve_in_with_slots(&witness, plan.n_constraints, &slots, true).unwrap();
                let mut c = vec![Fr::ZERO; plan.n_constraints];
                for term in plan.linear_c().terms {
                    c[term.row] += term.coefficient * solution.w[term.wire];
                }
                assert_eq!(c, solution.c);
                let without_c = plan.solve_in_with_slots(&witness, plan.n_constraints, &slots, false).unwrap();
                assert_eq!(solution.w, without_c.w);
                assert_eq!(solution.a, without_c.a);
                assert_eq!(solution.b, without_c.b);
                assert!(without_c.c.is_empty());
            }
        }
    }

    #[test]
    fn incomplete_images_keep_initialized_storage() {
        let (words, coefficients) = small();
        let mut plan = Plan::new(&encode_words(&words), &coefficients).unwrap();
        assert!(plan.definitely_writes_all());
        plan.ops.pop(); // Missing one output row must retain the safe fallback.
        let plan = reloaded(&plan);
        assert!(!plan.definitely_writes_all());
        assert!(plan.equal_wire_groups().is_empty());
        let solution = plan.solve(&[Fr::from(9u64), Fr::from(3u64)]).unwrap();
        assert!(solution.a.iter().any(|v| *v == Fr::ZERO));
        assert_eq!(solution.w[3], Fr::from(9u64));
    }

    #[test]
    fn an_image_solves_as_its_plan() {
        let (small_words, coefficients) = small();
        let (sums_words, sums_coefficients) = with_sums();
        for (program, coefficients, witness) in [
            (
                small_words,
                coefficients.clone(),
                vec![Fr::from(9u64), Fr::from(3u64)],
            ),
            (
                sums_words,
                sums_coefficients,
                vec![Fr::from(5u64), -Fr::from(5u64), Fr::from(25u64)],
            ),
            (with_hints(), coefficients, vec![Fr::from(3u64)]),
        ] {
            let plan = Plan::new(&encode_words(&program), &coefficients).unwrap();
            let loaded = reloaded(&plan);
            let (expected, actual) = (
                plan.solve(&witness).unwrap(),
                loaded.solve(&witness).unwrap(),
            );
            assert_eq!(
                (expected.w, expected.a, expected.b, expected.c),
                (actual.w, actual.a, actual.b, actual.c)
            );
            assert_eq!(
                (
                    loaded.n_wires,
                    loaded.n_public,
                    &loaded.a_indices,
                    &loaded.b_indices
                ),
                (
                    plan.n_wires,
                    plan.n_public,
                    &plan.a_indices,
                    &plan.b_indices
                )
            );
        }
    }

    #[test]
    fn a_damaged_image_is_an_error() {
        let (program, coefficients) = with_sums();
        let image = Plan::new(&encode_words(&program), &coefficients)
            .unwrap()
            .image();
        let word = |at: usize, value: u32| {
            let mut damaged = image.clone();
            damaged[at..at + 4].copy_from_slice(&value.to_le_bytes());
            Plan::from_image(&damaged).err()
        };
        assert_eq!(word(4, IMAGE_VERSION as u32 + 1), Some("unsupported solver image"));
        // The wire count, the first A index, and both words of the first
        // term, past the lists they index.
        assert_eq!(word(8, 0), Some(DAMAGED_IMAGE));
        assert_eq!(word(52, u32::MAX), Some(DAMAGED_IMAGE));
        let header = |i: usize| {
            u32::from_le_bytes(image[8 + 4 * i..12 + 4 * i].try_into().unwrap()) as usize
        };
        let terms = 52 + 4 * (header(4) + header(5)) + 32 * header(6);
        assert!(header(7) > 0, "the plan has terms");
        assert_eq!(word(terms, 1000), Some(DAMAGED_IMAGE));
        assert_eq!(word(terms + 4, 1000), Some(DAMAGED_IMAGE));
        // Every prefix and an extension.
        for end in 0..image.len() {
            assert!(Plan::from_image(&image[..end]).is_err(), "{end}");
        }
        assert!(Plan::from_image(&[image.as_slice(), &[0]].concat()).is_err());
    }

    /// From the program deflated, a block at a time, the plan the whole
    /// program builds; a damaged or cut stream is an error.
    #[test]
    fn builds_from_the_deflated_program() {
        let (program, coefficients) = with_sums();
        let program = encode_words(&program);
        let whole = Plan::new(&program, &coefficients).unwrap().image();
        let deflated = miniz_oxide::deflate::compress_to_vec(&program, 6);
        assert_eq!(Plan::from_deflated(&deflated, &coefficients).unwrap().image(), whole);
        // Longer than a block.
        let mut long = program.clone();
        long.extend(std::iter::repeat_n(0u8, 1 << 17));
        assert!(Plan::from_deflated(&miniz_oxide::deflate::compress_to_vec(&long, 6), &coefficients).is_err());
        assert!(Plan::from_deflated(&deflated[..deflated.len() / 2], &coefficients).is_err());
        assert!(Plan::from_deflated(&[0xff; 64], &coefficients).is_err());
    }

    fn reloaded(plan: &Plan) -> Plan {
        let image = plan.image();
        let loaded = Plan::from_image(&image).unwrap();
        assert_eq!(loaded.image(), image);
        loaded
    }

    #[test]
    fn gnark_solver_fixtures() {
        let Some(dir) = std::env::var_os("MOPRO_GNARK_SOLVER_FIXTURES") else {
            eprintln!(
                "set MOPRO_GNARK_SOLVER_FIXTURES to compare with native gnark (enabled in CI)"
            );
            return;
        };
        let mut count = 0;
        for entry in std::fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.extension().is_none_or(|ext| ext != "case") {
                continue;
            }
            let encoded = std::fs::read(&path).unwrap();
            let mut data = &encoded[..];
            let mut read = || {
                let n = u32::from_le_bytes(data[..4].try_into().unwrap()) as usize;
                let value = &data[4..4 + n];
                data = &data[4 + n..];
                value
            };
            let program = read();
            let coefficients = read();
            let witness = super::super::frs(read());
            let plan = reloaded(&Plan::new(program, coefficients).unwrap());
            let solution = plan.solve(&witness).unwrap();
            for actual in [&solution.w, &solution.a, &solution.b, &solution.c] {
                assert_eq!(encode_fields(actual), read(), "{}", path.display());
            }
            let invalid = super::super::frs(read());
            assert!(plan.solve(&invalid).is_err(), "{}", path.display());
            assert!(data.is_empty());
            count += 1;
        }
        assert_eq!(
            count, 16,
            "generate fixtures with go test -run TestKernelSolverFixtures first"
        );
    }
}

impl Plan {
    pub(crate) fn stored_slot_count(&self)->usize{self.n_slots}
    pub(crate) fn input_count(&self)->usize{self.n_inputs}
}

#[cfg(zopro_resource_schedule)] mod sparse_poseidon;
