#[cfg(test)]
mod reduced_gauss_tests {
    use super::*;
    #[test]
    fn complex_outputs_and_conjugate_scaling_match_scalar_fields() {
        let mut rng = crate::testing::Seeded::new(0x243f_6a88_85a3_08d3);
        let mut fq = || {
            let mut bytes = [0u8; 64];
            for c in bytes.chunks_mut(8) {
                c.copy_from_slice(&rng.next_u64().to_le_bytes());
            }
            Fq::from_random_bytes(&bytes)
        };
        for round in 0..512 {
            let corners = [
                Fq::ZERO,
                Fq::ONE,
                -Fq::ONE,
                Fq::from(2u64),
                -Fq::from(2u64),
                Fq::from(1u64 << 51),
                Fq::from(1u64 << 52),
                Fq::from(u64::MAX),
            ];
            let a: [Fq2; LANES] = core::array::from_fn(|i| {
                if round < 64 {
                    Fq2::new(corners[(round + i) % 8], corners[(round / 8 + i) % 8])
                } else {
                    Fq2::new(fq(), fq())
                }
            });
            let b: [Fq2; LANES] = core::array::from_fn(|i| {
                if round < 64 {
                    Fq2::new(
                        corners[(round / 8 + 3 * i) % 8],
                        corners[(round + 5 * i) % 8],
                    )
                } else {
                    Fq2::new(fq(), fq())
                }
            });
            let factor: [Fq; LANES] = core::array::from_fn(|i| corners[(round + i) % 8]);
            let mut pa = [
                FqBatch::new(&a.map(|v| v.c0)),
                FqBatch::new(&a.map(|v| v.c1)),
            ];
            let pb = [
                FqBatch::new(&b.map(|v| v.c0)),
                FqBatch::new(&b.map(|v| v.c1)),
            ];
            let f = FqBatch::new(&factor);
            let mut scalar = a;
            for _ in 0..8 {
                let product = Fq2::lanes_mul(&pa, &pb);
                let conjugate = Fq2::lanes_scaled_numerator(&pa, &pb, &f);
                let (p0, p1, c0, c1) = (
                    product[0].get(),
                    product[1].get(),
                    conjugate[0].get(),
                    conjugate[1].get(),
                );
                for i in 0..LANES {
                    assert_eq!(Fq2::new(p0[i], p1[i]), scalar[i] * b[i]);
                    assert_eq!(
                        Fq2::new(c0[i], c1[i]),
                        (scalar[i] * b[i].conjugate()).mul_by_fp(&factor[i])
                    );
                }
                // Existing lazy add/sub paths feed changing reduced states.
                pa = Fq2::lanes_add(&product, &Fq2::lanes_sub(&conjugate, &pb));
                scalar = core::array::from_fn(|i| {
                    scalar[i] * b[i] + (scalar[i] * b[i].conjugate()).mul_by_fp(&factor[i]) - b[i]
                });
            }
        }
    }
}
