#[cfg(test)]
mod adaptive_tree_tests {
    use super::*;
    use zopro_bn254::{G2Config, G2Projective};

    #[test]
    fn adaptive_boundaries_preserve_outputs_and_bound_retained_record_capacity() {
        let mut p = G2Projective::generator() * Fr::from(11u64);
        let raw: Vec<_> = (0..4096)
            .map(|_| {
                p += G2Projective::generator();
                p
            })
            .collect();
        let bases = G2Projective::batch_to_affine(&raw);
        for vector_inverse in [false, true] {
            let mut scratch = Scratch::<G2Config> {
                vector_inverse,
                ..Default::default()
            };
            let mut max_groups = 0;
            for m in [
                31usize, 32, 33, 40, 47, 48, 56, 64, 65, 127, 255, 256, 257, 512, 1023, 2048, 32,
                31, 1, 0,
            ] {
                let mut points: Vec<_> = bases[..2 * m]
                    .iter()
                    .copied()
                    .map(limbs::<G2Config>)
                    .collect();
                let mut counts = [2 * m];
                reduce_layer(&mut points, &[0], &mut counts, &mut scratch);
                assert_eq!(scratch.pairs.len(), m);
                assert_eq!(counts, [m]);
                for i in 0..m {
                    assert_eq!(
                        affine::<G2Config>(points[i]),
                        (bases[2 * i] + bases[2 * i + 1]).to_affine()
                    );
                }
                max_groups = max_groups.max(m / LANES);
                let record_bytes = scratch.groups.capacity() * core::mem::size_of::<Group<Fq2>>()
                    + scratch.prefix_records.capacity() * core::mem::size_of::<[FqBatch; 2]>();
                let old_exact_record_bound = max_groups * core::mem::size_of::<Group<Fq2>>();
                assert!(
                    record_bytes <= old_exact_record_bound,
                    "m={m},actual={record_bytes},old={old_exact_record_bound}"
                );
                if Fq2::TREE_PREFIX {
                    assert!(scratch.groups.capacity() <= 3);
                    if m >= 32 {
                        assert!(!scratch.prefix_records.is_empty());
                    }
                } else {
                    assert_eq!(scratch.prefix_records.capacity(), 0);
                }
            }
        }
    }
}
