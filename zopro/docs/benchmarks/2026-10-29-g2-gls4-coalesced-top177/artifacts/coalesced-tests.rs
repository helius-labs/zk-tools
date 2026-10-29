    #[test]
    fn coalesced_top_skewed_complete_curve_groups() {
        let g=G2Projective::generator();
        for threads in [1,4,32] {
            let pool=rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            pool.install(|| {
                for n in [0usize,1,7,1025,4097] {
                    let mut points=Vec::new();let mut short=Vec::new();let mut expected=Fr::ZERO;
                    for i in 0..n {
                        let (p,weight)=match i%5 {
                            0=>(G2Affine::IDENTITY,Fr::ZERO),
                            1|2=>(G2Affine::GENERATOR,Fr::ONE),
                            _=>(-G2Affine::GENERATOR,-Fr::ONE),
                        };
                        let d=if i%2==0 {1024u64}else{1};
                        let magnitude=if d==1024 {(1u128<<65)-1}else{1u128<<55};
                        let negative=i%3==0;
                        let mut s=[0u64;5];s[0]=magnitude as u64;
                        s[4]=(magnitude>>64)as u64 | if negative {1<<4}else{0};
                        points.push(p);short.push(s);
                        expected+=weight*if negative {-Fr::from(d)}else{Fr::from(d)};
                    }
                    let buckets=top_component(&points,&short,0);
                    assert_eq!(buckets.len(),1024);
                    assert!(buckets[1..1023].iter().all(G2Affine::is_identity));
                    assert_eq!(crate::msm::weighted_affine(&buckets).to_affine(),(g*expected).to_affine(),"threads={threads} n={n}");
                }
            });
        }
    }
