#[cfg(test)]
mod tree_prefix_tests {
    use super::*;
    use zopro_bn254::{G2Config, G2Projective};
    #[test]
    fn prefix_tree_complete_pairs_tails_and_reuse() {
        let mut p = G2Projective::generator() * Fr::from(11u64);
        let raw: Vec<_> = (0..4096).map(|_| {p+=G2Projective::generator();p}).collect();
        let bases = G2Projective::batch_to_affine(&raw);
        for vector_inverse in [false,true] {
            let mut scratch = Scratch::<G2Config> {vector_inverse,..Default::default()};
            for n in [0usize,1,2,3,7,8,9,15,16,17,31,32,33,127,128,129,255,256,257,2048,257,8,1,0] {
                for pattern in 0..5 {
                    let counts=[n,1,n/3,0,2,3,5];
                    let mut starts=Vec::new();let mut len=0;
                    for count in counts {starts.push(len);len+=count;}
                    let input:Vec<Affine<G2Config>> = (0..len).map(|i|match pattern {
                        0=>bases[i],
                        1=>if i%3==0 {Affine::IDENTITY}else{bases[i]},
                        2=>bases[i/2],
                        3=>if i%2==0 {bases[i/2]}else{-bases[i/2]},
                        _=>if i%4==0 {Affine::IDENTITY}else if i%4==1 {-bases[i/2]}else {bases[i/2]},
                    }).collect();
                    let mut expected=Vec::new();
                    for (&start,&count) in starts.iter().zip(&counts) {
                        expected.push((0..count.div_ceil(2)).map(|i| {
                            let a=input[start+2*i];
                            if 2*i+1<count {(a+input[start+2*i+1]).to_affine()}else{a}
                        }).collect::<Vec<_>>());
                    }
                    let mut points:Vec<_>=input.iter().copied().map(limbs::<G2Config>).collect();
                    let mut actual_counts=counts;
                    reduce_layer(&mut points,&starts,&mut actual_counts,&mut scratch);
                    for bucket in 0..counts.len() {
                        assert_eq!(actual_counts[bucket],counts[bucket].div_ceil(2));
                        for (i,&expected) in expected[bucket].iter().enumerate() {
                            assert_eq!(affine::<G2Config>(points[starts[bucket]+i]),expected,
                                "inverse={vector_inverse} n={n} pattern={pattern} bucket={bucket} i={i}");
                        }
                    }
                    if Fq2::TREE_PREFIX {assert_eq!(scratch.groups.capacity(),0);}
                    else {assert_eq!(scratch.prefix_records.capacity(),0);}
                }
            }
        }
        assert!(core::mem::size_of::<[FqBatch;2]>()<=core::mem::size_of::<Group<Fq2>>());
    }
}
