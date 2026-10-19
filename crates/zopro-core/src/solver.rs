// Copyright 2020-2025 Consensys Software Inc.
// Licensed under Apache-2.0; see LICENSE-APACHE.
// Modified for Mopro: prepare a validated execution plan for gnark R1CS
// systems whose only hints are InvZeroHint and bits.nBits, and solve it using
// arkworks field arithmetic. The arithmetic follows constraint/bn254/solver.go's
// solveR1C and solveHintGeneric semantics.

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

#[derive(Clone, Copy)]
struct Term {
    coefficient: usize,
    wire: usize,
}

struct Assignment {
    side: usize,
    wire: usize,
    inverse: Fr,
    coefficient: usize,
}
enum Expression {
    Zero,
    Wire(usize),
    Constant(usize),
    Term(usize),
    Sum(Range<usize>),
}
struct Step {
    constraint: usize,
    sums: Range<usize>,
    expressions: [Expression; 3],
    assignment: Option<Assignment>,
}
struct Sum {
    previous: usize,
    term: Term,
}
const NO_SUM: usize = usize::MAX;
// "ZSP1" and the solver image's version: a kernel reads the images its own
// version wrote.
const IMAGE_MAGIC: usize = 0x3150_535a;
pub(crate) const IMAGE_VERSION: usize = 1;
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
    input: Range<usize>,
    outputs: Range<usize>,
}
enum Op {
    Constraint(Step),
    Hint(Hint),
}
pub(crate) struct Plan {
    n_wires: usize,
    n_inputs: usize,
    pub(crate) n_public: usize,
    pub(crate) a_indices: Vec<usize>,
    pub(crate) b_indices: Vec<usize>,
    coefficients: Vec<Fr>,
    terms: Vec<Term>,
    sums: Vec<Sum>,
    n_constraints: usize,
    ops: Vec<Op>,
    /// The ops in epochs of parallel tasks, built at the first solve.
    schedule: std::sync::OnceLock<Schedule>,
}

/// Bands of dependency depth (epochs), each a list of tasks: the connected
/// components of the epoch's ops, whose inputs earlier epochs or earlier
/// ops of the same task produce. Tasks of an epoch run in parallel, each in
/// program order, which is a topological order. gnark's own levels are too
/// narrow for this (a median of 16 ops at 8 -> 2 over 8036 levels), but
/// the circuits' notes and hashes are largely independent until late.
struct Schedule {
    epochs: Vec<Vec<Vec<u32>>>,
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
                (self.a_indices.len() + self.b_indices.len()) * size_of::<usize>(),
            ),
            ("coefficients", self.coefficients.len() * size_of::<Fr>()),
            ("terms", self.terms.len() * size_of::<Term>()),
            ("sums", self.sums.len() * size_of::<Sum>()),
            ("ops", self.ops.len() * size_of::<Op>()),
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

impl Plan {
    // Header: version, wires, inputs (including ONE), public (including ONE),
    // constraints, hints, A index count, B index count; followed by both index
    // lists, constraint records [ID, len(L), len(R), len(O), (coefficient,
    // wire)...] and hint records [HINT_RECORD, kind, 1, len(input),
    // (coefficient, wire)..., first output, end of outputs]. Records arrive in
    // gnark's level order, while a constraint's ID selects the FFT row.
    pub(crate) fn new(program: &[u8], coefficients: &[u8]) -> Result<Self, &'static str> {
        if !program.len().is_multiple_of(4) || !coefficients.len().is_multiple_of(32) {
            return Err("invalid solver encoding");
        }
        let mut words = program
            .as_chunks::<4>()
            .0
            .iter()
            .map(|v| u32::from_le_bytes(*v) as usize);
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
        let mut indices = |n: usize| -> Result<Vec<usize>, &'static str> {
            if n > n_wires || n > program.len() / 4 {
                return Err("invalid solver index count");
            }
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                let index = read()?;
                if index >= n_wires || out.last().is_some_and(|previous| *previous >= index) {
                    return Err("invalid solver wire index");
                }
                out.push(index);
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
        let mut solved = vec![false; n_wires];
        solved[..n_inputs].fill(true);
        let mut seen = vec![false; n_constraints];
        let mut terms = Vec::new();
        let mut sums = Vec::new();
        let mut prefixes: FxMap<(usize, usize, usize), usize> = FxMap::default();
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
                for _ in 0..read()? {
                    let coefficient = read()?;
                    let wire = read()?;
                    if coefficient >= n_coefficients || wire >= n_wires || !solved[wire] {
                        return Err("invalid hint input");
                    }
                    if wire == 0 {
                        constant += coefficients[coefficient];
                    } else if coefficient != 0 {
                        terms.push(Term { coefficient, wire });
                    }
                }
                if constant != Fr::ZERO {
                    terms.push(Term {
                        coefficient: coefficients.len(),
                        wire: 0,
                    });
                    coefficients.push(constant);
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
                    input: start..terms.len(),
                    outputs,
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
                for _ in 0..len {
                    let coefficient = read()?;
                    let wire = read()?;
                    if coefficient >= n_coefficients || wire >= n_wires {
                        return Err("invalid solver term");
                    }
                    if solved[wire] {
                        if wire == 0 {
                            constant += coefficients[coefficient];
                        } else if coefficient != 0 {
                            terms.push(Term { coefficient, wire });
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
                            side,
                            wire,
                            inverse,
                            coefficient,
                        });
                    }
                }
                if constant != Fr::ZERO {
                    terms.push(Term {
                        coefficient: coefficients.len(),
                        wire: 0,
                    });
                    coefficients.push(constant);
                }
                expressions[side] = match &terms[start..] {
                    [] => Expression::Zero,
                    [term] if term.wire == 0 => Expression::Constant(term.coefficient),
                    [term] if term.coefficient == 1 => Expression::Wire(term.wire),
                    [_] => Expression::Term(start),
                    _ if terms.len() - start <= 2 => Expression::Sum(start..terms.len()),
                    _ => {
                        // Immutable wires let later expressions reuse sums from
                        // earlier constraints. Intern prefixes so long common
                        // linear expressions are evaluated only once per proof.
                        // Every new sum is scheduled at its first use, after
                        // all of its circuit wires have been solved.
                        let mut previous = NO_SUM;
                        for term in &terms[start..] {
                            previous = *prefixes
                                .entry((previous, term.coefficient, term.wire))
                                .or_insert_with(|| {
                                    let id = n_wires + sums.len();
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
                solved[assignment.wire] = true;
            }
            ops.push(Op::Constraint(Step {
                constraint,
                sums: sum_start..sums.len(),
                expressions,
                assignment,
            }));
        }
        if words.next().is_some() || solved.contains(&false) || seen.contains(&false) {
            return Err("incomplete solver program");
        }
        Ok(Self {
            n_wires,
            n_inputs,
            n_public,
            a_indices,
            b_indices,
            coefficients,
            terms,
            sums,
            n_constraints,
            ops,
            schedule: std::sync::OnceLock::new(),
        })
    }

    // The plan as an image, for a host to keep and load without building it
    // again from the program: a header of [magic, version, wires, inputs,
    // public, constraints, A indices, B indices, coefficients, terms, sums,
    // ops], then each list in order. Every value is a little-endian u32 but
    // the coefficients and inverses, four little-endian Montgomery words.
    pub(crate) fn image(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(
            48 + 4 * (self.a_indices.len() + self.b_indices.len())
                + 32 * self.coefficients.len()
                + 8 * self.terms.len()
                + 12 * self.sums.len()
                + 100 * self.ops.len(),
        );
        let word = |out: &mut Vec<u8>, v: usize| out.extend_from_slice(&(v as u32).to_le_bytes());
        let field = |out: &mut Vec<u8>, v: &Fr| {
            for w in v.0 {
                out.extend_from_slice(&w.to_le_bytes());
            }
        };
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
            self.ops.len(),
        ] {
            word(&mut out, v);
        }
        for v in self.a_indices.iter().chain(&self.b_indices) {
            word(&mut out, *v);
        }
        for v in &self.coefficients {
            field(&mut out, v);
        }
        for term in &self.terms {
            word(&mut out, term.coefficient);
            word(&mut out, term.wire);
        }
        for sum in &self.sums {
            word(&mut out, sum.previous);
            word(&mut out, sum.term.coefficient);
            word(&mut out, sum.term.wire);
        }
        for op in &self.ops {
            match op {
                Op::Hint(hint) => {
                    for v in [
                        1,
                        hint.kind as usize,
                        hint.input.start,
                        hint.input.end,
                        hint.outputs.start,
                        hint.outputs.end,
                    ] {
                        word(&mut out, v);
                    }
                }
                Op::Constraint(step) => {
                    for v in [0, step.constraint, step.sums.start, step.sums.end] {
                        word(&mut out, v);
                    }
                    for expression in &step.expressions {
                        let (tag, a, b) = match expression {
                            Expression::Zero => (0, 0, 0),
                            Expression::Wire(v) => (1, *v, 0),
                            Expression::Constant(v) => (2, *v, 0),
                            Expression::Term(v) => (3, *v, 0),
                            Expression::Sum(range) => (4, range.start, range.end),
                        };
                        for v in [tag, a, b] {
                            word(&mut out, v);
                        }
                    }
                    match &step.assignment {
                        None => word(&mut out, 0),
                        Some(assignment) => {
                            for v in [1, assignment.side, assignment.wire, assignment.coefficient] {
                                word(&mut out, v);
                            }
                            field(&mut out, &assignment.inverse);
                        }
                    }
                }
            }
        }
        out
    }

    // A plan from `image`, with every index bounded by its list, so that a
    // damaged image is an error rather than a fault while solving. What the
    // bounds cannot see, a plan that solves the wrong witness, the host's
    // verification of every proof does.
    pub(crate) fn from_image(image: &[u8]) -> Result<Self, &'static str> {
        let mut r = Reader { image, at: 0 };
        if r.word()? != IMAGE_MAGIC || r.word()? != IMAGE_VERSION {
            return Err("unsupported solver image");
        }
        let [n_wires, n_inputs, n_public, n_constraints, n_a, n_b, n_coefficients, n_terms, n_sums, n_ops] =
            r.words::<10>()?;
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
            || n_wires.checked_add(n_sums).is_none()
            || needed > (image.len() - r.at) as u64
        {
            return Err(DAMAGED_IMAGE);
        }
        let indices = |r: &mut Reader, n: usize| -> Result<Vec<usize>, &'static str> {
            let mut out = Vec::with_capacity(n);
            for _ in 0..n {
                let index = r.word()?;
                if index >= n_wires || out.last().is_some_and(|previous| *previous >= index) {
                    return Err(DAMAGED_IMAGE);
                }
                out.push(index);
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
            Ok(Term { coefficient, wire })
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
        let mut sums = Vec::with_capacity(n_sums);
        for index in 0..n_sums {
            let previous = match r.word()? {
                v if v == u32::MAX as usize => NO_SUM,
                v if (n_wires..n_wires + index).contains(&v) => v,
                _ => return Err(DAMAGED_IMAGE),
            };
            sums.push(Sum {
                previous,
                term: term(&mut r)?,
            });
        }
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
                    input,
                    outputs,
                }));
                continue;
            }
            let [constraint, sums_start, sums_end] = r.words::<3>()?;
            if constraint >= n_constraints {
                return Err(DAMAGED_IMAGE);
            }
            let step_sums = range(sums_start, sums_end, n_sums)?;
            let mut expressions = [Expression::Zero, Expression::Zero, Expression::Zero];
            for expression in &mut expressions {
                let [tag, a, b] = r.words::<3>()?;
                *expression = match tag {
                    0 => Expression::Zero,
                    1 if a < n_wires + n_sums => Expression::Wire(a),
                    2 if a < n_coefficients => Expression::Constant(a),
                    3 if a < n_terms => Expression::Term(a),
                    4 => Expression::Sum(range(a, b, n_terms)?),
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
                        side,
                        wire,
                        inverse,
                        coefficient,
                    })
                }
                _ => return Err(DAMAGED_IMAGE),
            };
            ops.push(Op::Constraint(Step {
                constraint,
                sums: step_sums,
                expressions,
                assignment,
            }));
        }
        if r.at != image.len() {
            return Err(DAMAGED_IMAGE);
        }
        Ok(Self {
            n_wires,
            n_inputs,
            n_public,
            a_indices,
            b_indices,
            coefficients,
            terms,
            sums,
            n_constraints,
            ops,
            schedule: std::sync::OnceLock::new(),
        })
    }

    pub(crate) fn matches(&self, a: usize, b: usize, k: usize, domain: usize) -> bool {
        self.a_indices.len() == a
            && self.b_indices.len() == b
            && self.n_wires - self.n_public == k
            && self.n_constraints <= domain
    }

    pub(crate) fn solve(&self, witness: &[Fr]) -> Result<Solution, String> {
        if witness.len() != self.n_inputs - 1 {
            return Err("invalid witness size".into());
        }
        let mut w = vec![Fr::ZERO; self.n_wires + self.sums.len()];
        w[0] = Fr::ONE;
        w[1..self.n_inputs].copy_from_slice(witness);
        let mut a = vec![Fr::ZERO; self.n_constraints];
        let mut b = a.clone();
        let mut c = a.clone();
        let view = View {
            w: w.as_mut_ptr(),
            a: a.as_mut_ptr(),
            b: b.as_mut_ptr(),
            c: c.as_mut_ptr(),
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
            for epoch in &schedule.epochs {
                epoch.par_iter().try_for_each(|task| {
                    task.iter().try_for_each(|&op| {
                        // SAFETY: see View and Schedule.
                        unsafe { self.run(&self.ops[op as usize], view) }
                    })
                })?;
            }
        }
        w.truncate(self.n_wires);
        Ok(Solution { w, a, b, c })
    }

    /// Builds the parallel solve's schedule now, at the key's load, rather
    /// than at its first proof (140 ms for 24 -> 2 on a Seeker).
    pub(crate) fn prepare(&self) {
        let epochs = solve_epochs();
        if epochs > 0 && self.ops.len() >= 4096 {
            self.schedule.get_or_init(|| self.build_schedule(epochs));
        }
    }

    /// The epochs and their tasks (see Schedule).
    fn build_schedule(&self, epochs: usize) -> Schedule {
        let n = self.ops.len();
        // Which op writes each wire and sum.
        let mut producer = vec![u32::MAX; self.n_wires + self.sums.len()];
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
        let epochs = tasks
            .into_iter()
            .map(|tasks| {
                let mut tasks: Vec<Vec<u32>> = tasks.into_values().collect();
                tasks.sort_unstable_by_key(|task| std::cmp::Reverse(task.len()));
                tasks
            })
            .filter(|tasks| !tasks.is_empty())
            .collect();
        Schedule { epochs }
    }

    /// The wires and sums an op reads.
    fn reads(&self, op: &Op, out: &mut Vec<usize>) {
        out.clear();
        match op {
            Op::Hint(hint) => out.extend(self.terms[hint.input.clone()].iter().map(|t| t.wire)),
            Op::Constraint(step) => {
                for index in step.sums.clone() {
                    let sum = &self.sums[index];
                    out.push(sum.term.wire);
                    if sum.previous != NO_SUM {
                        out.push(sum.previous);
                    }
                }
                for expression in &step.expressions {
                    match expression {
                        Expression::Wire(wire) => out.push(*wire),
                        Expression::Term(term) => out.push(self.terms[*term].wire),
                        Expression::Sum(range) => {
                            out.extend(self.terms[range.clone()].iter().map(|t| t.wire))
                        }
                        Expression::Zero | Expression::Constant(_) => {}
                    }
                }
            }
        }
    }

    /// The wires and sums an op writes.
    fn writes(&self, op: &Op) -> Vec<usize> {
        match op {
            Op::Hint(hint) => hint.outputs.clone().collect(),
            Op::Constraint(step) => step
                .assignment
                .iter()
                .map(|assignment| assignment.wire)
                .chain(step.sums.clone().map(|index| self.n_wires + index))
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
            if term.wire == 0 {
                return self.coefficients[term.coefficient];
            }
            match term.coefficient {
                1 => get(term.wire),
                2 => get(term.wire).double(),
                3 => -get(term.wire),
                _ => self.coefficients[term.coefficient] * get(term.wire),
            }
        };
        let step = match op {
            Op::Constraint(step) => step,
            Op::Hint(hint) => {
                let mut value = Fr::ZERO;
                for term in &self.terms[hint.input.clone()] {
                    value += term_value(term);
                }
                match hint.kind {
                    HintKind::InvZero => {
                        set(hint.outputs.start, value.inverse().unwrap_or(Fr::ZERO))
                    }
                    HintKind::Bits => {
                        let bits = value.to_canonical();
                        for (i, wire) in hint.outputs.clone().enumerate() {
                            set(wire, if (bits[i / 64] >> (i % 64)) & 1 == 1 { Fr::ONE } else { Fr::ZERO });
                        }
                    }
                }
                return Ok(());
            }
        };
        for index in step.sums.clone() {
            let sum = &self.sums[index];
            let term = term_value(&sum.term);
            set(
                self.n_wires + index,
                if sum.previous == NO_SUM {
                    term
                } else {
                    get(sum.previous) + term
                },
            );
        }
        let mut values = [Fr::ZERO; 3];
        for (value, expression) in values.iter_mut().zip(&step.expressions) {
            *value = match expression {
                Expression::Zero => Fr::ZERO,
                Expression::Wire(wire) => get(*wire),
                Expression::Constant(coefficient) => self.coefficients[*coefficient],
                Expression::Term(term) => term_value(&self.terms[*term]),
                Expression::Sum(range) => {
                    let mut sum = Fr::ZERO;
                    for term in &self.terms[range.clone()] {
                        sum += term_value(term);
                    }
                    sum
                }
            };
        }
        let fail = || format!("constraint #{} is not satisfied", step.constraint);
        if let Some(assignment) = &step.assignment {
            let term = match assignment.side {
                0 | 1 => {
                    let other = values[1 - assignment.side];
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
                        let term = quotient - values[assignment.side];
                        values[assignment.side] = quotient;
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
                assignment.wire,
                match assignment.coefficient {
                    1 => term,
                    3 => -term,
                    _ => term * assignment.inverse,
                },
            );
        } else if values[0] * values[1] != values[2] {
            return Err(fail());
        }
        // SAFETY: each constraint's row is written by its own op only.
        unsafe {
            *v.a.add(step.constraint) = values[0];
            *v.b.add(step.constraint) = values[1];
            *v.c.add(step.constraint) = values[2];
        }
        Ok(())
    }

    fn evaluate_term(&self, term: &Term, w: &[Fr]) -> Fr {
        if term.wire == 0 {
            return self.coefficients[term.coefficient];
        }
        match term.coefficient {
            1 => w[term.wire],
            2 => w[term.wire].double(),
            3 => -w[term.wire],
            _ => self.coefficients[term.coefficient] * w[term.wire],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
                let mut w = vec![Fr::ZERO; plan.n_wires + plan.sums.len()];
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
        let mut coefficients = small().1;
        coefficients.extend(encode_fields(&[
            Fr::from(3u64),
            Fr::from(5u64),
            Fr::from(7u64),
        ]));
        let mut program = vec![
            2, 7, 4, 1, 3, 0, 7, 7, 0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6,
        ];
        let base = vec![2, 1, 3, 2, 5, 3]; // 2X - Y + 3Z
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
        assert_eq!(word(4, 2), Some("unsupported solver image"));
        // The wire count, the first A index, and both words of the first
        // term, past the lists they index.
        assert_eq!(word(8, 0), Some(DAMAGED_IMAGE));
        assert_eq!(word(48, u32::MAX), Some(DAMAGED_IMAGE));
        let header = |i: usize| {
            u32::from_le_bytes(image[8 + 4 * i..12 + 4 * i].try_into().unwrap()) as usize
        };
        let terms = 48 + 4 * (header(4) + header(5)) + 32 * header(6);
        assert!(header(7) > 0, "the plan has terms");
        assert_eq!(word(terms, 1000), Some(DAMAGED_IMAGE));
        assert_eq!(word(terms + 4, 1000), Some(DAMAGED_IMAGE));
        // Every prefix and an extension.
        for end in 0..image.len() {
            assert!(Plan::from_image(&image[..end]).is_err(), "{end}");
        }
        assert!(Plan::from_image(&[image.as_slice(), &[0]].concat()).is_err());
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
