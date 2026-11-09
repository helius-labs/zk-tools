    #[test]
    fn exact_rows_preserve_slots_euler_products_and_errors() {
        let (small_words, coefficients) = small();
        let (sums_words, sums_coefficients) = with_sums();
        let fixtures = [
            (small_words, coefficients.clone(), vec![Fr::from(9u64), Fr::from(3u64)]),
            (sums_words, sums_coefficients, vec![Fr::from(5u64), -Fr::from(5u64), Fr::from(25u64)]),
            (with_hints(), coefficients.clone(), vec![Fr::from(3u64)]),
        ];
        let mut cases = 0;
        for workers in [1,16,32] {
            rayon::ThreadPoolBuilder::new().num_threads(workers).build().unwrap().install(|| {
                for (program, coefficients, witness) in &fixtures {
                    let original = Plan::new(&encode_words(program), coefficients).unwrap();
                    let loaded = reloaded(&original);
                    for plan in [&original, &loaded] {
                        let slots: Vec<_> = (plan.n_wires..plan.n_wires + plan.n_slots).collect();
                        for domain in [8usize,16,64] {
                            for materialize_c in [false,true] {
                                let exact = plan.solve_in_with_slots(witness, 0, &slots, materialize_c).unwrap();
                                let padded = plan.solve_in_with_slots(witness, domain, &slots, materialize_c).unwrap();
                                assert_eq!(exact.w,padded.w);
                                assert_eq!(exact.a,padded.a);
                                assert_eq!(exact.b,padded.b);
                                assert_eq!(exact.c,padded.c);
                                assert_eq!(exact.a.capacity(),plan.n_constraints);
                                assert_eq!(exact.b.capacity(),plan.n_constraints);
                                assert_eq!(exact.c.capacity(),if materialize_c {plan.n_constraints}else{0});
                                assert!(padded.a.capacity() >= domain);
                                assert_eq!(exact.w.len(),plan.n_wires+slots.len());
                                let root=Fr::root_of_unity(domain).unwrap();
                                let coset=Fr::from(5u64);
                                let mut fft=crate::fft::Plan::new(domain,root,coset,crate::fft::denominator(domain,coset).unwrap());
                                fft.prepare_euler_basis();
                                let x=fft.euler_product_values(exact.a,exact.b);
                                let y=fft.euler_product_values(padded.a,padded.b);
                                assert_eq!(x,y);
                                assert_eq!(x.len(),plan.n_constraints);
                                cases += 1;
                            }
                        }
                    }
                }
                let hints=Plan::new(&encode_words(&with_hints()), &coefficients).unwrap();
                for invalid in [Fr::ZERO,Fr::from(4u64)] {
                    for c in [false,true] {
                        assert_eq!(hints.solve_in_with_slots(&[invalid],0,&[],c).err(),
                                   hints.solve_in_with_slots(&[invalid],64,&[],c).err());
                        assert!(hints.solve_in_with_slots(&[invalid],0,&[],c).is_err());
                    }
                }
                let mut incomplete=Plan::new(&encode_words(&fixtures[0].0),&fixtures[0].1).unwrap();
                incomplete.ops.pop();
                assert!(!incomplete.definitely_writes_all());
                let x=incomplete.solve_in_with_slots(&fixtures[0].2,0,&[],true).unwrap();
                let y=incomplete.solve_in_with_slots(&fixtures[0].2,64,&[],true).unwrap();
                assert_eq!((x.w,x.a,x.b,x.c),(y.w,y.a,y.b,y.c));
            });
        }
        assert_eq!(cases,108);
        println!("EXACT_ROW_CASES={cases};workers=1,16,32;slots,C,loaded,invalid,incomplete,Euler");
    }

