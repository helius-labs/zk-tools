#[allow(dead_code)]
mod inherited {
    include!("/home/ubuntu/src/zopro-perf-work/split-redc-planes176/model.rs");
    const NEW_ROOT: &str = "/home/ubuntu/src/zopro-perf-work/cios-preadd-local176";
    #[derive(Default)]
    struct NewStats {
        outputs: u64,
        prefix_rows: u64,
        max_prefix: u128,
        max_temporary: u128,
        digest: Sha256,
        endpoints: u64,
    }
    fn checked(v: u128, s: &mut NewStats, temporary: bool) -> u128 {
        assert!(v < 32 * B);
        s.max_prefix = s.max_prefix.max(v);
        if temporary {
            s.max_temporary = s.max_temporary.max(v);
        }
        v
    }
    fn candidate(
        f: &Field,
        a: &BigUint,
        b: &BigUint,
        c: &BigUint,
        d: &BigUint,
        op: usize,
        s: &mut NewStats,
        common: &mut Stats,
    ) -> BigUint {
        for x in [a, b, c, d] {
            assert!(x <= &f.two);
            s.endpoints += u64::from(x == &u(0) || x == &f.two);
        }
        let cc = if op == 2 { &f.two - c } else { c.clone() };
        let (ad, bd, cd, dd) = (digit(a), digit(b), digit(&cc), digit(d));
        let raw = a * b + if op == 0 { u(0) } else { &cc * d };
        let mut reference = u(0);
        let mut cumulative_q = u(0);
        let mut t = [0u128; 6];
        for i in 0..4 {
            let mut accepted = t;
            product(&mut accepted, ad, bd[i], common);
            if op != 0 {
                product(&mut accepted, cd, dd[i], common);
            }
            reference += a * u(bd[i]);
            if op != 0 {
                reference += &cc * u(dd[i]);
            }
            let z = checked(
                t[0] + ((ad[0] * bd[i]) & MASK) + if op == 0 { 0 } else { (cd[0] * dd[i]) & MASK },
                s,
                false,
            );
            assert_eq!(z, accepted[0]);
            let q = (z * f.inv52) & MASK;
            let qr = ((&reference & u(MASK)) * u(f.inv52)) & u(MASK);
            assert_eq!(u(q), qr);
            cumulative_q += &qr << (52 * i);
            let carry = (z + MASK) / B + (q * f.pd[0] >> 52);
            assert_eq!(carry, (z + q * f.pd[0]) / B);
            let mut out = [0u128; 6];
            for j in 1..5 {
                let mut contribution = checked(
                    ((ad[j] * bd[i]) & MASK) + (ad[j - 1] * bd[i] >> 52),
                    s,
                    true,
                );
                if op != 0 {
                    contribution = checked(
                        contribution + ((cd[j] * dd[i]) & MASK) + (cd[j - 1] * dd[i] >> 52),
                        s,
                        true,
                    );
                }
                let previous_plus_product = checked(t[j] + contribution, s, false);
                let reduction = checked(
                    previous_plus_product
                        + ((q * f.pd[j]) & MASK)
                        + if j == 1 { carry } else { q * f.pd[j - 1] >> 52 },
                    s,
                    false,
                );
                out[j - 1] = reduction;
            }
            out[4] = checked(
                t[5] + (ad[4] * bd[i] >> 52)
                    + if op == 0 { 0 } else { cd[4] * dd[i] >> 52 }
                    + (q * f.pd[4] >> 52),
                s,
                false,
            );
            reduce_q(&mut accepted, z, q, f.pd, 52, common);
            for j in 0..5 {
                accepted[j] = accepted[j + 1];
            }
            accepted[5] = 0;
            assert_eq!(out, accepted, "coefficientwise accepted CIOS row {i}");
            reference = (reference + qr * &f.p) >> 52;
            assert_eq!(number(&out), reference);
            t = out;
            s.prefix_rows += 1;
        }
        // Reuse only the unchanged accepted product/reduction routines on this
        // new prefix state. The old persistent-plane model/main never runs.
        product(&mut t, ad, bd[4], common);
        if op != 0 {
            product(&mut t, cd, dd[4], common);
        }
        reference += a * u(bd[4]);
        if op != 0 {
            reference += &cc * u(dd[4]);
        }
        assert_eq!(number(&t), reference);
        let bits = f.bits - 208;
        let mask = (1u128 << bits) - 1;
        let frontier = t[0];
        let q = (frontier * f.inv52) & mask;
        let qr = ((&reference & u(mask)) * u(f.inv52)) & u(mask);
        assert_eq!(u(q), qr);
        cumulative_q += &qr << 208;
        assert_eq!(cumulative_q, (&raw * &f.negative) % &f.r);
        reduce_q(&mut t, frontier, q, f.pd, bits, common);
        let mut out = if bits == 52 {
            [t[1], t[2], t[3], t[4], t[5]]
        } else {
            let low = (frontier + mask) / (1u128 << 48) + (q * f.pd[0] >> 48);
            let higher = t[1] - low;
            [
                low + ((higher & mask) << 4),
                (higher >> 48) + ((t[2] & mask) << 4),
                (t[2] >> 48) + ((t[3] & mask) << 4),
                (t[3] >> 48) + ((t[4] & mask) << 4),
                (t[4] >> 48) + (t[5] << 4),
            ]
        };
        for j in 0..4 {
            out[j + 1] += out[j] >> 52;
            out[j] &= MASK;
        }
        assert!(out.iter().all(|x| *x < B));
        let value = number(&out);
        assert_eq!(value, (&raw + &cumulative_q * &f.p) / &f.r);
        assert_eq!(value, (reference + qr * &f.p) >> bits);
        let value = if op != 0 && bits == 48 && value >= f.two {
            common.corrections += 1;
            value - &f.two
        } else {
            value
        };
        assert!(value < f.two);
        assert_eq!(&value % &f.p, (&raw * &f.ri) % &f.p);
        s.outputs += 1;
        s.digest.update(value.to_bytes_le());
        value
    }
    #[derive(Clone, Copy)]
    struct D {
        depth: u64,
        zero: bool,
    }
    struct Work {
        ifmas: u64,
        adds: u64,
        source: u64,
    }
    impl Work {
        fn fma(&mut self, a: D, b: D, c: D) -> D {
            self.ifmas += 1;
            D {
                depth: (a.depth + 4)
                    .max(b.depth + self.source)
                    .max(c.depth + self.source),
                zero: false,
            }
        }
        fn add(&mut self, a: D, b: D) -> D {
            if a.zero {
                return b;
            }
            if b.zero {
                return a;
            }
            self.adds += 1;
            D {
                depth: a.depth.max(b.depth) + 2,
                zero: false,
            }
        }
        fn carry(&mut self, a: D, one: D) -> D {
            let x = self.add(a, one);
            D {
                depth: x.depth + 2,
                zero: false,
            }
        }
    }
    fn source_work(fused: bool, balanced: bool, source: u64) -> Value {
        let ready = D {
            depth: 0,
            zero: false,
        };
        let zero = D {
            depth: 0,
            zero: true,
        };
        let mut w = Work {
            ifmas: 0,
            adds: 0,
            source,
        };
        let mut t = [zero; 6];
        let mut quotients = Vec::new();
        for _ in 0..4 {
            if balanced {
                let mut z = w.fma(t[0], ready, ready);
                if fused {
                    z = w.fma(z, ready, ready);
                }
                let q = w.fma(zero, z, ready);
                quotients.push(q.depth);
                let carry = w.carry(z, ready);
                let low = w.fma(carry, q, ready);
                let mut out = [zero; 6];
                for j in 1..5 {
                    let mut contribution = w.fma(zero, ready, ready);
                    contribution = w.fma(contribution, ready, ready);
                    if fused {
                        contribution = w.fma(contribution, ready, ready);
                        contribution = w.fma(contribution, ready, ready);
                    }
                    let previous_plus_product = w.add(t[j], contribution);
                    let mut reduction = w.fma(previous_plus_product, q, ready);
                    if j == 1 {
                        reduction = w.add(reduction, low);
                    } else {
                        reduction = w.fma(reduction, q, ready);
                    }
                    out[j - 1] = reduction;
                }
                let mut top = w.fma(t[5], ready, ready);
                if fused {
                    top = w.fma(top, ready, ready);
                }
                out[4] = w.fma(top, q, ready);
                t = out;
            } else {
                for j in 0..5 {
                    t[j] = w.fma(t[j], ready, ready);
                    t[j + 1] = w.fma(t[j + 1], ready, ready);
                }
                if fused {
                    for j in 0..5 {
                        t[j] = w.fma(t[j], ready, ready);
                        t[j + 1] = w.fma(t[j + 1], ready, ready);
                    }
                }
                let q = w.fma(zero, t[0], ready);
                quotients.push(q.depth);
                let carry = w.carry(t[0], ready);
                let low = w.fma(carry, q, ready);
                for j in 1..5 {
                    t[j] = w.fma(t[j], q, ready);
                    t[j + 1] = w.fma(t[j + 1], q, ready);
                }
                t[1] = w.add(t[1], low);
                for j in 0..5 {
                    t[j] = t[j + 1];
                }
                t[5] = zero;
            }
        }
        json!({"fused":fused,"coefficient_local":balanced,"hypothetical_source_weight":source,"prefix_IFMAs":w.ifmas,"prefix_adds":w.adds,"quotient_depths":quotients,"coefficient_depths":t[..5].iter().map(|v|v.depth).collect::<Vec<_>>(),"frontier_depth":t[0].depth,"max_coefficient_depth":t[..5].iter().map(|v|v.depth).max().unwrap(),"not_emitted_code":true})
    }
    pub fn run_new() {
        assert!(!Path::new(NEW_ROOT).join("model.json").exists());
        let pins: Value =
            serde_json::from_slice(&fs::read(format!("{NEW_ROOT}/pins.json")).unwrap()).unwrap();
        for (p, h) in pins.as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
        }
        let prior: Value = serde_json::from_slice(
            &fs::read("/home/ubuntu/src/zopro-perf-work/cios-coefficient-local176/model.json")
                .unwrap(),
        )
        .unwrap();
        assert_eq!(prior["mathematical_screen_passed"], true);
        let bounds = prior["bound_rows"].clone();
        assert_eq!(bounds.as_array().unwrap().len(), 52);
        let mut s = NewStats::default();
        let mut common = Stats::default();
        let mut contexts = Vec::new();
        let mut seed = 0x7ac083d69215f4b9u64;
        for (name, hex) in [
            (
                "Fq",
                "30644e72e131a029b85045b68181585d97816a916871ca8d3c208c16d87cfd47",
            ),
            (
                "Fr",
                "30644e72e131a029b85045b68181585d2833e84879b9709143e1f593f0000001",
            ),
        ] {
            for bits in [256, 260] {
                let f = Field::new(BigUint::parse_bytes(hex.as_bytes(), 16).unwrap(), bits);
                let edge = [
                    u(0),
                    u(1),
                    u(2),
                    &f.p - u(1),
                    f.p.clone(),
                    &f.p + u(1),
                    &f.two - u(1),
                    f.two.clone(),
                ];
                for op in 0..3 {
                    let before = s.outputs;
                    for a in &edge {
                        for b in &edge {
                            if op == 0 {
                                candidate(&f, a, b, &u(0), &u(0), op, &mut s, &mut common);
                            } else {
                                for c in &edge {
                                    for d in &edge {
                                        candidate(&f, a, b, c, d, op, &mut s, &mut common);
                                    }
                                }
                            }
                        }
                    }
                    for _ in 0..1357 {
                        let (a, b, c, d) = (
                            random(&f, &mut seed),
                            random(&f, &mut seed),
                            random(&f, &mut seed),
                            random(&f, &mut seed),
                        );
                        candidate(&f, &a, &b, &c, &d, op, &mut s, &mut common);
                    }
                    let mut a = random(&f, &mut seed);
                    for _ in 0..193 {
                        let (b, c, d) = (
                            random(&f, &mut seed),
                            random(&f, &mut seed),
                            random(&f, &mut seed),
                        );
                        a = candidate(&f, &a, &b, &c, &d, op, &mut s, &mut common);
                    }
                    contexts.push(
                        json!({"field":name,"radix":bits,"op":op,"outputs":s.outputs-before}),
                    );
                }
            }
        }
        let mut comparisons = Vec::new();
        let mut selected = true;
        for fused in [false, true] {
            for source in [0, 4] {
                let old = source_work(fused, false, source);
                let new = source_work(fused, true, source);
                assert_eq!(old["prefix_IFMAs"], new["prefix_IFMAs"]);
                assert_eq!(
                    new["prefix_adds"].as_u64().unwrap(),
                    old["prefix_adds"].as_u64().unwrap() + 12
                );
                let improves = new["frontier_depth"].as_u64().unwrap()
                    < old["frontier_depth"].as_u64().unwrap()
                    && new["max_coefficient_depth"].as_u64().unwrap()
                        < old["max_coefficient_depth"].as_u64().unwrap();
                selected &= improves;
                comparisons.push(json!({"old":old,"new":new,"strict_depth_improvement":improves}));
            }
        }
        assert_eq!(s.prefix_rows, 4 * s.outputs);
        let result = json!({"mathematical_screen_passed":true,"necessary_source_depth_screen_passed":selected,"native_qualification_selected":selected,"native_started":false,"contexts":contexts,"new_outputs":s.outputs,"new_prefix_row_checks":s.prefix_rows,"max_prefix":s.max_prefix.to_string(),"max_temporary":s.max_temporary.to_string(),"bound_rows":bounds,"source_comparisons":comparisons,"digest":format!("{:x}",s.digest.finalize()),"endpoint_inputs":s.endpoints,"common_fused_corrections":common.corrections,"additional_persistent_coefficient_planes":0,"declared_transient_product_coefficients":1,"compiler_lifetimes_unmeasured":true,"new_native_calls":0,"new_proofs":0,"new_timings":0,"adopted":false});
        let mut result = result;
        result["coefficient_merge"] = json!("preadd_before_modulus");
        result["bound_rows_inherited_not_recomputed"] = json!(true);
        fs::write(
            format!("{NEW_ROOT}/model.json"),
            serde_json::to_vec_pretty(&result).unwrap(),
        )
        .unwrap();
        println!("{} new outputs / {} coefficientwise+BigUint prefix rows; source-depth selection={selected}",s.outputs,s.prefix_rows);
    }
}
fn main() {
    inherited::run_new();
}
