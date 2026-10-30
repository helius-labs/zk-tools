use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet};
use zopro_bn254::Fr;

const STORED: u32 = 1 << 31;
const NONE: u32 = STORED - 1;
const CHAIN: u32 = STORED - 2;

fn check(ok: bool, why: &str) -> Result<(), String> {
    if ok { Ok(()) } else { Err(why.into()) }
}

// An exact affine form in independent opaque atoms. A nonlinear operation
// receives its output wire as a fresh atom; its internals are not expanded.
// Thus a nonconstant form is never silently reclassified as constant.
#[derive(Clone, Debug, PartialEq, Eq)]
struct Form {
    constant: Fr,
    terms: BTreeMap<usize, Fr>,
}
impl Form {
    fn constant(v: Fr) -> Self { Self { constant: v, terms: BTreeMap::new() } }
    fn atom(w: usize) -> Self {
        Self { constant: Fr::ZERO, terms: [(w, Fr::ONE)].into() }
    }
    fn value(&self) -> Option<Fr> { self.terms.is_empty().then_some(self.constant) }
    fn scale(&self, v: Fr) -> Self {
        if v == Fr::ZERO { return Self::constant(Fr::ZERO); }
        Self { constant: self.constant * v, terms: self.terms.iter().map(|(&w, &c)| (w, c * v)).collect() }
    }
    fn plus(&self, other: &Self) -> Self {
        let mut out = self.clone();
        out.constant += other.constant;
        for (&w, &c) in &other.terms {
            let v = out.terms.entry(w).or_insert(Fr::ZERO);
            *v += c;
            if *v == Fr::ZERO { out.terms.remove(&w); }
        }
        out
    }
    fn minus(&self, other: &Self) -> Self { self.plus(&other.scale(-Fr::ONE)) }
    fn evaluate(&self, atoms: &[Fr]) -> Fr {
        self.terms.iter().fold(self.constant, |v, (&w, &c)| v + c * atoms[w])
    }
}

#[derive(Clone, Copy)]
struct Term { c: usize, w: usize }
#[derive(Clone, Copy)]
struct Sum { p: u32, t: Term }
#[derive(Clone)]
enum Expr { Zero, Wire(usize), Constant(usize), Term(usize), Sum(usize, usize) }
struct Reader<'a> { data: &'a [u8], at: usize }
impl Reader<'_> {
    fn bytes(&mut self, n: usize) -> Result<&[u8], String> {
        let end = self.at.checked_add(n).ok_or("offset overflow")?;
        let v = self.data.get(self.at..end).ok_or("truncated image")?;
        self.at = end;
        Ok(v)
    }
    fn word(&mut self) -> Result<usize, String> {
        Ok(u32::from_le_bytes(self.bytes(4)?.try_into().unwrap()) as usize)
    }
    fn words<const N: usize>(&mut self) -> Result<[usize; N], String> {
        let mut a = [0; N];
        for v in &mut a { *v = self.word()?; }
        Ok(a)
    }
    fn field(&mut self) -> Result<Fr, String> {
        let b = self.bytes(32)?;
        let w = core::array::from_fn(|i| u64::from_le_bytes(b[i * 8..i * 8 + 8].try_into().unwrap()));
        check(w.iter().rev().cmp(Fr::MODULUS.iter().rev()).is_lt(), "noncanonical Montgomery field")?;
        let v = Fr::from_montgomery(w);
        // Image stores canonical Montgomery representatives, not LE integers.
        check(Fr::from_canonical(v.to_canonical()).is_some_and(|x| x == v), "bad field")?;
        check(v.0 == w, "Montgomery storage differs")?;
        Ok(v)
    }
}
fn bounds(a: usize, b: usize, n: usize) -> Result<std::ops::Range<usize>, String> {
    check(a <= b && b <= n, "invalid range")?;
    Ok(a..b)
}
fn get(state: &[Option<Form>], w: usize) -> Result<Form, String> {
    state.get(w).and_then(|x| x.clone()).ok_or("undefined value".into())
}
fn term(t: Term, coeff: &[Fr], state: &[Option<Form>]) -> Result<Form, String> {
    get(state, t.w).map(|v| v.scale(coeff[t.c]))
}
fn expression(e: &Expr, coeff: &[Fr], terms: &[Term], state: &[Option<Form>]) -> Result<Form, String> {
    match *e {
        Expr::Zero => Ok(Form::constant(Fr::ZERO)),
        Expr::Wire(w) => get(state, w),
        Expr::Constant(c) => Ok(Form::constant(coeff[c])),
        Expr::Term(t) => term(terms[t], coeff, state),
        Expr::Sum(a, b) => terms[a..b].iter().try_fold(Form::constant(Fr::ZERO), |v, &t| Ok(v.plus(&term(t, coeff, state)?))),
    }
}
fn syntactic(e: &Expr, coeff: &[Fr], terms: &[Term]) -> Option<Fr> {
    let t = |t: Term| if t.w == 0 || coeff[t.c] == Fr::ZERO { Some(coeff[t.c]) } else { None };
    match *e {
        Expr::Zero => Some(Fr::ZERO),
        Expr::Wire(0) => Some(Fr::ONE),
        Expr::Wire(_) => None,
        Expr::Constant(c) => Some(coeff[c]),
        Expr::Term(i) => t(terms[i]),
        Expr::Sum(a, b) => terms[a..b].iter().try_fold(Fr::ZERO, |v, &x| t(x).map(|x| v + x)),
    }
}

// This mirrors assignment semantics, including the important zero-other
// branch. Proportional O/R is deliberately NOT cancelled for unknown R:
// R=0 selects a different solver value. No nonzero assumption is invented.
fn assignment(side: usize, values: &[Form; 3], inverse: Fr) -> Option<Form> {
    let out = if side == 2 {
        if let Some(c) = values[0].value() {
            values[1].scale(c).minus(&values[2])
        } else if let Some(c) = values[1].value() {
            values[0].scale(c).minus(&values[2])
        } else { return None; }
    } else {
        match values[1 - side].value() {
            Some(c) if c == Fr::ZERO => Form::constant(Fr::ZERO),
            Some(c) => values[2].scale(c.inverse().unwrap()).minus(&values[side]),
            None if values[2].value() == Some(Fr::ZERO) && values[side].value() == Some(Fr::ZERO) => Form::constant(Fr::ZERO),
            None => return None,
        }
    };
    Some(out.scale(inverse))
}

fn write(state: &mut [Option<Form>], w: usize, f: Form, max_terms: &mut usize) -> Result<(), String> {
    let out = state.get_mut(w).ok_or("write out of range")?;
    check(out.is_none(), "duplicate write")?;
    *max_terms = (*max_terms).max(f.terms.len());
    *out = Some(f);
    Ok(())
}

pub fn census(data: &[u8]) -> Result<Value, String> {
    let mut r = Reader { data, at: 0 };
    check(r.word()? == 0x3150535a && r.word()? == 2, "unsupported solver image")?;
    let [nw, ni, np, nc, na, nb, ncoef, nt, ns, nslots, nops] = r.words::<11>()?;
    check(0 < np && np <= ni && ni <= nw && nslots <= ns, "invalid dimensions")?;
    let minimum = 4u64 * (na as u64 + nb as u64) + 32 * ncoef as u64 + 8 * nt as u64 + 12 * ns as u64 + 16 * nops as u64;
    check(minimum <= (data.len() - r.at) as u64, "lists exceed file")?;
    let mut queries = [vec![], vec![]];
    for (q, n) in queries.iter_mut().zip([na, nb]) {
        for _ in 0..n {
            let w = r.word()?;
            check(w < nw && q.last().is_none_or(|&last| last < w), "unsorted query")?;
            q.push(w);
        }
    }
    let mut coeff = Vec::with_capacity(ncoef);
    for _ in 0..ncoef { coeff.push(r.field()?); }
    check(ncoef >= 5 && coeff[..5] == [Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE, -Fr::from(2u64)], "special coefficients differ")?;
    let read_term = |r: &mut Reader<'_>| -> Result<Term, String> {
        let [c, w] = r.words::<2>()?;
        check(c < ncoef && w < nw, "invalid term")?;
        Ok(Term { c, w })
    };
    let mut terms = Vec::with_capacity(nt);
    for _ in 0..nt { terms.push(read_term(&mut r)?); }
    let mut sums = Vec::with_capacity(ns);
    for _ in 0..ns {
        let p = r.word()? as u32;
        sums.push(Sum { p, t: read_term(&mut r)? });
    }
    let mut state = vec![None; nw.checked_add(nslots).ok_or("value overflow")?];
    state[0] = Some(Form::constant(Fr::ONE));
    for w in 1..ni { state[w] = Some(Form::atom(w)); }
    let mut rows = vec![false; nc];
    let mut seen_sums = vec![false; ns];
    let mut candidates = BTreeSet::new();
    let mut constant_writes = 0;
    let mut affine_writes = 0;
    let mut opaque_writes = 0;
    let mut syntactic_assignments = 0;
    let mut hint_constants = 0;
    let mut hints = 0;
    let mut constraints = 0;
    let mut assignments = 0;
    let mut max_terms = 1;
    let mut slot_constants = 0;
    let mut expanded_constant_sides = 0;
    let mut hidden_constant_sides = 0;
    let mut sides = [0usize; 3];
    for _ in 0..nops {
        match r.word()? {
            1 => {
                let [kind, a, b, start, end] = r.words::<5>()?;
                check(kind <= 1 && start < end && end <= nw && end - start <= 256, "invalid hint")?;
                check(kind != 0 || end - start == 1, "inverse hint arity")?;
                let form = terms[bounds(a, b, nt)?].iter().try_fold(Form::constant(Fr::ZERO), |v, &t| Ok::<_, String>(v.plus(&term(t, &coeff, &state)?)))?;
                let known = form.value();
                let bits = known.map(|v|v.to_canonical());
                for (i, w) in (start..end).enumerate() {
                    let value = if kind == 0 { known.map(|v| v.inverse().unwrap_or(Fr::ZERO)) }
                        else { bits.map(|v| if v[i / 64] >> (i % 64) & 1 == 1 { Fr::ONE } else { Fr::ZERO }) };
                    if value.is_some() { hint_constants += 1; candidates.insert(w); }
                    let form = value.map(Form::constant).unwrap_or_else(|| Form::atom(w));
                    write(&mut state, w, form, &mut max_terms)?;
                }
                hints += 1;
            }
            0 => {
                let [row, a, b, first] = r.words::<4>()?;
                check(row < nc && !rows[row], "invalid constraint row")?;
                rows[row] = true;
                let mut slot = nw.checked_add(first).ok_or("slot overflow")?;
                let mut chain = Form::constant(Fr::ZERO);
                for j in bounds(a, b, ns)? {
                    check(!seen_sums[j], "duplicate sum execution")?;
                    seen_sums[j] = true;
                    let s = sums[j];
                    let t = term(s.t, &coeff, &state)?;
                    let p = s.p & !STORED;
                    chain = match p {
                        NONE => t,
                        CHAIN => chain.plus(&t),
                        w => get(&state, w as usize)?.plus(&t),
                    };
                    if s.p & STORED != 0 {
                        slot_constants += usize::from(chain.value().is_some());
                        write(&mut state, slot, chain.clone(), &mut max_terms)?;
                        slot += 1;
                    }
                }
                let mut expressions = Vec::with_capacity(3);
                for _ in 0..3 {
                    let [tag, a, b] = r.words::<3>()?;
                    expressions.push(match tag {
                        0 => Expr::Zero,
                        1 => { check(a < state.len(), "wire expression out of range")?; Expr::Wire(a) },
                        2 => { check(a < ncoef, "constant out of range")?; Expr::Constant(a) },
                        3 => { check(a < nt, "term expression out of range")?; Expr::Term(a) },
                        4 => { bounds(a, b, nt)?; Expr::Sum(a, b) },
                        _ => return Err("invalid expression tag".into()),
                    });
                }
                let values: [Form; 3] = expressions.iter().map(|e| expression(e, &coeff, &terms, &state)).collect::<Result<Vec<_>, _>>()?.try_into().unwrap();
                for i in 0..3 {
                    if values[i].value().is_some() {
                        expanded_constant_sides += 1;
                        hidden_constant_sides += usize::from(syntactic(&expressions[i], &coeff, &terms).is_none());
                    }
                }
                match r.word()? {
                    0 => {},
                    1 => {
                        let [side, w, c] = r.words::<3>()?;
                        check(side <= 2 && w < nw && c < ncoef && coeff[c] != Fr::ZERO, "invalid assignment")?;
                        let inverse = r.field()?;
                        check(coeff[c] * inverse == Fr::ONE, "assignment inverse mismatch")?;
                        let syn = if side == 2 { syntactic(&expressions[0], &coeff, &terms).is_some() || syntactic(&expressions[1], &coeff, &terms).is_some() }
                            else { syntactic(&expressions[1 - side], &coeff, &terms).is_some() };
                        syntactic_assignments += usize::from(syn);
                        let affine = assignment(side, &values, inverse);
                        let out = match affine {
                            Some(f) => {
                                candidates.insert(w);
                                if f.value().is_some() { constant_writes += 1; } else { affine_writes += 1; }
                                f
                            }
                            None => { opaque_writes += 1; Form::atom(w) }
                        };
                        write(&mut state, w, out, &mut max_terms)?;
                        assignments += 1;
                        sides[side] += 1;
                    }
                    _ => return Err("invalid assignment tag".into()),
                }
                constraints += 1;
            }
            _ => return Err("invalid operation tag".into()),
        }
    }
    check(r.at == data.len(), "trailing bytes")?;
    check(rows.iter().all(|&v| v) && seen_sums.iter().all(|&v| v), "incomplete constraint/sum coverage")?;
    check(state.iter().all(Option::is_some), "unwritten values")?;
    let mut query_results = BTreeMap::new();
    for (q, name) in queries.iter().zip(["A", "B"]) {
        let candidate_rows: Vec<_> = q.iter().copied().filter(|w| candidates.contains(w)).collect();
        let constant_rows = candidate_rows.iter().filter(|&&w| state[w].as_ref().unwrap().value().is_some()).count();
        let nonconstant = candidate_rows.len() - constant_rows;
        let atom_terms: usize = candidate_rows.iter().map(|&w| state[w].as_ref().unwrap().terms.len()).sum();
        query_results.insert(name, json!({"points":q.len(),"noninput_candidate_rows":candidate_rows.len(),"constant_rows":constant_rows,"affine_nonconstant_rows":nonconstant,"expanded_atom_terms":atom_terms}));
    }
    Ok(json!({"complete":true,"image_version":2,"bytes":data.len(),"wires":nw,"inputs":ni,"public":np,"constraints":constraints,"hints":hints,"ops":nops,"sum_records":ns,"stored_slots":nslots,"assignments":assignments,"assignment_sides":sides,"syntactic_linear_assignments":syntactic_assignments,"expanded_affine_assignments":affine_writes,"expanded_constant_assignments":constant_writes,"opaque_nonlinear_assignments":opaque_writes,"constant_hint_outputs":hint_constants,"constant_stored_slots":slot_constants,"constant_expression_sides":expanded_constant_sides,"hidden_constant_expression_sides":hidden_constant_sides,"maximum_form_terms":max_terms,"analysis_form_terms":state.iter().map(|f|f.as_ref().unwrap().terms.len()).sum::<usize>(),"queries":query_results,"production_changed":false,"new_proofs":0,"new_witness_solves":0,"new_latency_samples":0}))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn f(v: u64) -> Fr { Fr::from(v) }
    #[test]
    fn affine_forms_match_independent_field_evaluation() {
        for seed in 0..128u64 {
            let atoms = [f(seed), f(seed * 19 + 7), f(seed * seed + 9)];
            let a = Form::atom(0).scale(f(3)).plus(&Form::atom(1).scale(-f(2))).plus(&Form::constant(f(7)));
            let b = Form::atom(1).scale(f(2)).minus(&Form::atom(0).scale(f(3)));
            assert_eq!(a.evaluate(&atoms), f(3) * atoms[0] - f(2) * atoms[1] + f(7));
            assert_eq!(a.plus(&b).value(), Some(f(7)));
            assert_eq!(a.scale(Fr::ZERO).value(), Some(Fr::ZERO));
            for side in 0..3 {
                let v = [a.clone(), Form::constant(f(5)), Form::atom(2)];
                let value = assignment(side, &v, f(11));
                let [l,r,o] = v.clone().map(|x|x.evaluate(&atoms));
                let expected = match side {0 => Some((o * r.inverse().unwrap() - l) * f(11)), 1 => None, _ => Some((l * r - o) * f(11))};
                assert_eq!(value.map(|x|x.evaluate(&atoms)), expected);
            }
        }
    }
    #[test]
    fn unknown_divisor_is_not_cancelled_and_zero_branch_is_preserved() {
        let z = Form::constant(Fr::ZERO);
        let atom = Form::atom(0);
        assert!(assignment(0, &[z.clone(),atom.clone(),atom.clone()], Fr::ONE).is_none());
        assert!(assignment(0, &[atom.clone(),atom.clone(),z.clone()], Fr::ONE).is_none());
        assert_eq!(assignment(0, &[z.clone(),atom,z.clone()], Fr::ONE).unwrap().value(), Some(Fr::ZERO));
        assert_eq!(assignment(0, &[Form::constant(f(17)),z.clone(),z], Fr::ONE).unwrap().value(), Some(Fr::ZERO));
    }
    fn image() -> Vec<u8> {
        let mut out = vec![];
        let words = |out: &mut Vec<u8>, a: &[u32]| for x in a { out.extend_from_slice(&x.to_le_bytes()); };
        // Four wires: one, input x, constant from cancellation, linear y.
        // One stored slot is x-x+5, then y=slot*x (a linear assignment).
        words(&mut out, &[0x3150535a,2,4,2,1,2,3,2,6,0,3,1,2]);
        words(&mut out, &[1,2,3,2,3]);
        let cs = [Fr::ZERO,Fr::ONE,f(2),-Fr::ONE,-f(2),f(5)];
        for c in cs { for w in c.0 { out.extend_from_slice(&w.to_le_bytes()); } }
        words(&mut out, &[NONE|STORED,1,1, CHAIN,3,1, CHAIN|STORED,5,0]);
        // Row0: slot0=x, then running=x-x, slot1 should not be written:
        // instead first record is not stored; final slot is 5.
        let at = 52 + 20 + 192;
        out[at..at+4].copy_from_slice(&NONE.to_le_bytes());
        // Assign wire2 = slot * 0 = 0 (side O), testing slot propagation.
        words(&mut out, &[0,0,0,3,0, 1,4,0, 0,0,0, 0,0,0, 1,2,2,1]);
        for w in Fr::ONE.0 { out.extend_from_slice(&w.to_le_bytes()); }
        // Assign wire3 = slot*x (side O) = 5*x, a hidden linear multiplier.
        words(&mut out, &[0,1,3,3,1, 1,4,0, 1,1,0, 0,0,0, 1,2,3,1]);
        for w in Fr::ONE.0 { out.extend_from_slice(&w.to_le_bytes()); }
        out
    }
    #[test]
    fn parser_propagates_stored_cancellation_and_rejects_corruption() {
        let b = image();
        let v = census(&b).unwrap();
        assert_eq!(v["expanded_affine_assignments"],1);
        assert_eq!(v["expanded_constant_assignments"],1);
        assert_eq!(v["constant_stored_slots"],1);
        assert_eq!(v["queries"]["A"]["noninput_candidate_rows"],2);
        assert_eq!(v["queries"]["B"]["constant_rows"],1);
        for cut in [0,4,51,b.len()-1] { assert!(census(&b[..cut]).is_err()); }
        let mut bad = b.clone(); bad.push(0); assert!(census(&bad).is_err());
        let mut bad = b.clone(); bad[56..60].copy_from_slice(&1u32.to_le_bytes()); assert!(census(&bad).is_err());
        let mut state = vec![Some(Form::atom(0)),None];
        let mut max = 0;
        assert!(write(&mut state,0,Form::constant(Fr::ZERO),&mut max).is_err());
        assert!(get(&state,1).is_err());
    }
}
