mod screen {
    include!("model.rs");
    pub fn check_new_cases() -> Value {
        let mut state = 0x92d68ca4b7513e07u64;
        let mut word = || {
            state ^= state << 13;
            state ^= state >> 7;
            state ^= state << 17;
            state
        };
        let mut scalar = || loop {
            let w = [word(), word(), word(), word() & ((1u64 << 62) - 1)];
            if let Some(s) = Fr::from_canonical(w) {
                break s;
            }
        };
        let mut receipts = Vec::new();
        let mut high_bit_cases = 0;
        for seed in 0..4 {
            let a = scalar();
            let step = scalar();
            high_bit_cases += usize::from(a.to_canonical()[3] >> 61 != 0)
                + usize::from(step.to_canonical()[3] >> 61 != 0);
            let mut p = G2Affine::GENERATOR.mul(&a);
            let delta = G2Affine::GENERATOR.mul(&step);
            let points = (0..4095)
                .map(|_| {
                    let q = p.to_affine();
                    p += delta;
                    (q.x, q.y)
                })
                .collect::<Vec<_>>();
            for n in [5usize, 9, 17, 33, 65, 129, 257, 513, 1025, 4095] {
                let s = a * Fr::from_u64(n as u64) + step * Fr::from_u64((n * (n - 1) / 2) as u64);
                let expected = G2Affine::GENERATOR.mul(&s).to_affine();
                let expected = vec![(expected.x, expected.y)];
                for scale in [Fq::ONE, Fq::from_u64(11)] {
                    let r = tree::<V>(&points[..n], &[n], scale).unwrap();
                    let q = tree::<Q>(&points[..n], &[n], scale).unwrap();
                    assert_eq!(r.points, expected);
                    assert_eq!(q.points, expected);
                    let mut h = Sha256::new();
                    for &(x, y) in &expected {
                        for f in [x.c0, x.c1, y.c0, y.c1] {
                            for w in f.to_canonical() {
                                h.update(w.to_le_bytes());
                            }
                        }
                    }
                    receipts.push(json!({"seed":seed,"n":n,"scale":if scale==Fq::ONE{1}else{11},"constructions_checked":2,"expected_output_sha256":format!("{:x}",h.finalize())}));
                }
            }
        }
        assert!(high_bit_cases > 0);
        json!({"passed":true,"new_complete_reductions":2*receipts.len(),"new_generator_oracle_arguments":receipts.len(),"scalars_with_bit253_set":high_bit_cases,"source_sha256":hash(r("model.rs")),"receipts":receipts,"original_580_cases_rerun":false,"new_timings":0,"new_proofs":0,"scope":"new full-width Fr arguments checked against generator multiplication of a*n+step*n*(n-1)/2; shared accepted field"})
    }
}
fn main() {
    let v = screen::check_new_cases();
    let p = "/home/ubuntu/src/zopro-perf-work/g2-real-norm-z176/generator-oracle.json";
    assert!(!std::path::Path::new(p).exists());
    std::fs::write(p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
    println!("PASS160new complete reductions/80generator arguments; original cases not replayed; no timings/proofs");
}
