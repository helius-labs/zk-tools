use super::*;
#[cfg(all(zopro_resource_schedule,zopro_poseidon_across8,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] mod across_refs;
#[cfg(all(zopro_resource_schedule,zopro_poseidon_across8,feature="ifma8",target_arch="x86_64",target_feature="avx512ifma",target_feature="avx512vl",target_feature="avx512dq",not(no_ifma)))] mod parallel_refs;

fn u32_at(data: &[u8], pos: &mut usize) -> usize {
    let x = u32::from_le_bytes(data[*pos..*pos + 4].try_into().unwrap());
    *pos += 4;
    x as usize
}
fn field_at(data: &[u8], pos: &mut usize) -> Fr {
    let limbs = core::array::from_fn(|i| {
        u64::from_le_bytes(data[*pos + 8 * i..*pos + 8 * i + 8].try_into().unwrap())
    });
    *pos += 32;
    Fr::from_canonical(limbs).unwrap()
}
fn fixture(t: usize, n: usize, forms: &[Vec<(usize, Fr)>], reverse: bool) -> Plan {
    let inputs = t + 1;
    let n_wires = inputs + 3 * n;
    let (partial, _) = parameters(t);
    let mut coefficients = vec![Fr::ZERO, Fr::ONE, Fr::from(2u64), -Fr::ONE, -Fr::from(2u64)];
    let mut sums = Vec::new();
    let mut ordered: Vec<Option<Op>> = (0..3 * n).map(|_| None).collect();
    for k in 0..n {
        let [f, m, l] = positions(t, partial, k);
        let begin = sums.len();
        for (j, (symbol, value)) in forms[k].iter().enumerate() {
            let wire = if *symbol == 0 {
                0
            } else if *symbol <= n {
                inputs + 3 * (*symbol - 1) + 2
            } else {
                *symbol - n
            };
            let coefficient = coefficients.len();
            coefficients.push(*value);
            sums.push(Sum {
                previous: (if j == 0 { NONE } else { CHAIN })
                    | if j + 1 == forms[k].len() { STORED } else { 0 },
                term: Term::new(coefficient, wire),
            });
        }
        let end = sums.len();
        let x = (n_wires + k) as u32;
        let square = (inputs + 3 * k) as u32;
        let fourth = square + 1;
        let fifth = square + 2;
        let assignment = |wire| {
            Some(Assignment {
                side: 2,
                wire,
                coefficient: 1,
                inverse: super::super::INVERSE_ONE,
            })
        };
        ordered[f] = Some(Op::Constraint(Step {
            constraint: f as u32,
            sums: (begin as u32, end as u32),
            first_slot: k as u32,
            expressions: [Expression::Wire(x), Expression::Wire(x), Expression::Zero],
            assignment: assignment(square),
        }));
        ordered[m] = Some(Op::Constraint(Step {
            constraint: m as u32,
            sums: (end as u32, end as u32),
            first_slot: (k + 1) as u32,
            expressions: [
                Expression::Wire(square),
                Expression::Wire(square),
                Expression::Zero,
            ],
            assignment: assignment(fourth),
        }));
        ordered[l] = Some(Op::Constraint(Step {
            constraint: l as u32,
            sums: (end as u32, end as u32),
            first_slot: (k + 1) as u32,
            expressions: if reverse {
                [
                    Expression::Wire(fourth),
                    Expression::Wire(x),
                    Expression::Zero,
                ]
            } else {
                [
                    Expression::Wire(x),
                    Expression::Wire(fourth),
                    Expression::Zero,
                ]
            },
            assignment: assignment(fifth),
        }));
    }
    Plan {
        n_wires,
        n_inputs: inputs,
        n_public: inputs,
        a_indices: Vec::new(),
        b_indices: Vec::new(),
        coefficients,
        inverses: super::super::Inverses::default(),
        terms: Vec::new(),
        sums: super::super::super::Store::Owned(sums),
        n_slots: n,
        n_constraints: 3 * n,
        ops: ordered.into_iter().map(Option::unwrap).collect(),
        fuse_sums: 2,
        schedule: std::sync::OnceLock::new(),
        definite: std::sync::OnceLock::new(),
    }
}
fn execute(
    p: &Plan,
    task: Option<&[u32]>,
    seed: usize,
    materialize: bool,
) -> (Result<(), String>, Vec<Vec<Fr>>) {
    let mut w = vec![Fr::from(0x5a5au64); p.n_wires + p.n_slots];
    w[0] = Fr::ONE;
    let mut random = 0x2545f4914f6cdd1du64.wrapping_mul(seed as u64 + 1);
    for i in 1..p.n_inputs {
        w[i] = match seed {
            0 => Fr::ZERO,
            1 => Fr::ONE,
            2 => -Fr::ONE,
            3 => Fr::from(2u64),
            _ => {
                let mut limbs = [0u64; 4];
                for x in &mut limbs {
                    random ^= random << 13;
                    random ^= random >> 7;
                    random ^= random << 17;
                    *x = random;
                }
                limbs[3] &= 0x1fff_ffff_ffff_ffff;
                Fr::from_canonical(limbs).unwrap()
            }
        };
    }
    let mut a = vec![Fr::from(77u64); p.n_constraints];
    let mut b = a.clone();
    let mut c = if materialize { a.clone() } else { Vec::new() };
    let v = View {
        w: w.as_mut_ptr(),
        a: a.as_mut_ptr(),
        b: b.as_mut_ptr(),
        c: if materialize {
            c.as_mut_ptr()
        } else {
            core::ptr::null_mut()
        },
    };
    let result = if let Some(task) = task {
        unsafe { p.run_poseidon_task(task, v) }
    } else {
        let mut result = Ok(());
        for op in &p.ops {
            if let Err(e) = unsafe { p.run(op, v) } {
                result = Err(e);
                break;
            }
        }
        result
    };
    (result, vec![w, a, b, c])
}
#[test]
fn every_width_exact_rows_and_adversarial_rejection() {
    let bytes = include_bytes!("all-widths.bin");
    let mut cursor = 0;
    let widths = u32_at(bytes, &mut cursor);
    assert_eq!(widths, 12);
    let mut comparisons = 0;
    let mut rejected = 0;
    let mut errors = 0;
    for _ in 0..widths {
        let t = u32_at(bytes, &mut cursor);
        let n = u32_at(bytes, &mut cursor);
        let (partial, a) = parameters(t);
        assert_eq!(n, 7 * t + partial);
        let mut forms = Vec::new();
        for _ in 0..n {
            let length = u32_at(bytes, &mut cursor);
            let mut row = Vec::new();
            for _ in 0..length {
                row.push((u32_at(bytes, &mut cursor), field_at(bytes, &mut cursor)));
            }
            forms.push(row);
        }
        let pattern = Pattern::new(t, partial, a).unwrap();
        // The fixture coefficients come from a separate Python big-integer
        // symbolic evaluator. The production Pattern does not construct them.
        for reverse in [false, true] {
            let plan = fixture(t, n, &forms, reverse);
            let image = plan.image();
            let mut task: Vec<u32> = (0..plan.ops.len() as u32).collect();
            assert_eq!(pattern.tag(&plan, &mut task), 1, "width {t}");
            assert_eq!(plan.image(), image);
            for seed in 0..8 {
                for c in [false, true] {
                    let reference = execute(&plan, None, seed, c);
                    assert!(reference.0.is_ok());
                    assert_eq!(
                        execute(&plan, Some(&task), seed, c),
                        reference,
                        "width {t}, reverse {reverse}, seed {seed}, C {c}"
                    );
                    comparisons += 1;
                }
            }
            // Unmatched operations after the span must still run and reject.
            let mut bad = fixture(t, n, &forms, reverse);
            let row = bad.n_constraints;
            bad.n_constraints += 1;
            bad.ops.push(Op::Constraint(Step {
                constraint: row as u32,
                sums: (0, 0),
                first_slot: 0,
                expressions: [
                    Expression::Constant(1),
                    Expression::Constant(1),
                    Expression::Zero,
                ],
                assignment: None,
            }));
            let mut task: Vec<u32> = (0..bad.ops.len() as u32).collect();
            assert_eq!(pattern.tag(&bad, &mut task), 1);
            for c in [false, true] {
                let reference = execute(&bad, None, 7, c);
                assert!(reference.0.is_err());
                assert_eq!(execute(&bad, Some(&task), 7, c), reference);
                errors += 1;
            }
            // One altered affine coefficient must not be accepted.
            let mut bad = fixture(t, n, &forms, reverse);
            let first = positions(t, partial, t)[0];
            let step = match &bad.ops[first] {
                Op::Constraint(s) => s,
                _ => unreachable!(),
            };
            let coefficient = bad.sums[step.sums().start].term.coefficient();
            bad.coefficients[coefficient] += Fr::ONE;
            let mut task: Vec<u32> = (0..bad.ops.len() as u32).collect();
            assert_eq!(pattern.tag(&bad, &mut task), 0);
            assert!(task.iter().all(|v| v >> 27 == 0));
            rejected += 1;
            // A negative C assignment is outside the exact shape contract.
            let mut bad = fixture(t, n, &forms, reverse);
            if let Op::Constraint(s) = &mut bad.ops[first] {
                let a = s.assignment.as_mut().unwrap();
                a.coefficient = 3;
                a.inverse = super::super::INVERSE_MINUS_ONE;
            }
            let mut task: Vec<u32> = (0..bad.ops.len() as u32).collect();
            assert_eq!(pattern.tag(&bad, &mut task), 0);
            rejected += 1;
        }
    }
    assert_eq!(cursor, bytes.len());
    assert_eq!(comparisons, 384);
    assert_eq!(rejected, 48);
    assert_eq!(errors, 48);
    eprintln!(
        "POSEIDON_ALL_WIDTHS comparisons={comparisons} rejected={rejected} tail_errors={errors}"
    );
}

#[test]
fn raw_operation_indices_above_tag_limit_fall_back() {
    assert!(!tagged_task(0, None));
    assert!(!tagged_task(INDEX as usize, Some(INDEX - 1)));
    // This is the first valid raw index which overlaps the task marker.
    let raw = TASK;
    assert!(!tagged_task(raw as usize + 1, Some(raw)));
    assert!(!tagged_task(211_000_000, Some(160_000_000)));
    for width in 0..16u32 {
        let encoded = TASK | (width << 28) | 17;
        assert!(tagged_task(100, Some(encoded)));
        assert!(tagged_task(INDEX as usize, Some(encoded)));
        assert!(!tagged_task(INDEX as usize + 1, Some(encoded)));
        // Reinterpret the same bits as a raw operation in a larger plan.
        let count = (encoded as usize).saturating_add(1);
        assert!(!tagged_task(count, Some(encoded)));
    }
}
