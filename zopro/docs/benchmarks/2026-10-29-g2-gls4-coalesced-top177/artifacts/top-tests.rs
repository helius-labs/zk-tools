    #[test]
    fn indexed_top_signed_edges_match_generator_sum() {
        let n = 2053;
        let g = G2Projective::generator();
        let mut cursor = G2Projective::IDENTITY;
        let mut points = Vec::new();
        let mut weights = Vec::new();
        let mut digits = Vec::new();
        let mut short = Vec::new();
        for i in 0..n {
            cursor += g;
            let mut p = cursor;
            let mut weight = Fr::from((i + 1) as u64);
            if i % 17 == 0 {p = G2Projective::IDENTITY;weight = Fr::ZERO;}
            else if i % 5 == 0 {p = -p;weight = -weight;}
            let mut s = [0u64; 5];
            let mut ds = [0i16; 4];
            for c in 0..4 {
                let d = (i * (2*c+1) + c*127) % 1025;
                // The largest magnitude rounds up to digit1024 via Booth carry.
                let magnitude = if d == 1024 {(1u128 << 65)-1} else {(d as u128)<<55};
                s[c] = magnitude as u64;
                s[4] |= ((magnitude >> 64) as u64) << c;
                let negative = (i+c)%3==0;
                if negative {s[4] |= 1 << (c+4);}
                ds[c] = if negative {-(d as i16)} else {d as i16};
            }
            points.push(p);weights.push(weight);short.push(s);digits.push(ds);
        }
        let points = G2Projective::batch_to_affine(&points);
        for c in 0..4 {
            let result = top_component(&points,&short,c);
            assert_eq!(result.len(),1024);
            let expected = weights.iter().zip(&digits).fold(Fr::ZERO,|sum,(&w,d)| {
                let f=Fr::from(d[c].unsigned_abs() as u64);
                sum + w * if d[c]<0 {-f}else{f}
            });
            assert_eq!(crate::msm::weighted_affine(&result).to_affine(),(g*expected).to_affine(),"component={c}");
        }
        assert!(top_component(&[],&[],0).iter().all(G2Affine::is_identity));
    }
    #[test]
    fn hybrid_msm_custom_rayon_pools_match_generator_sum() {
        let n = 2049;
        let g = G2Projective::generator();
        let mut seed = 0x485942524944544fu64;
        let mut cursor = G2Projective::IDENTITY;
        let mut points = Vec::new();
        let mut short = Vec::new();
        let mut expected = Fr::ZERO;
        for i in 0..n {
            cursor += g;
            let p = if i%13==0 {G2Projective::IDENTITY} else if i%3==0 {-cursor} else {cursor};
            let s = random(&mut seed);
            let w = if i%13==0 {Fr::ZERO} else if i%3==0 {-Fr::from((i+1)as u64)}else{Fr::from((i+1)as u64)};
            points.push(p);short.push(split(s.to_canonical()));expected+=s*w;
        }
        let points=G2Projective::batch_to_affine(&points);
        for threads in [1,4,16,32] {
            let pool=rayon::ThreadPoolBuilder::new().num_threads(threads).build().unwrap();
            let actual=pool.install(||run_prepared(&points,&short));
            assert_eq!(actual.to_affine(),(g*expected).to_affine(),"threads={threads}");
        }
    }
