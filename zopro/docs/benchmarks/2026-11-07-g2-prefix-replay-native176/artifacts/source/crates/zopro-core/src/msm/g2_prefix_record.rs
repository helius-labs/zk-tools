//! Keep only the prefix when reverse evaluation can recompute its norm.
use super::*;

pub(super) struct Record {
    pub(super) prefix: FqBatch,
    #[cfg(not(zopro_g2_prefix_replay))]
    norm: FqBatch,
}

impl Record {
    #[inline(always)]
    pub(super) fn new(prefix: FqBatch, norm: FqBatch) -> Self {
        #[cfg(zopro_g2_prefix_replay)]
        let _ = norm;
        Self {
            prefix,
            #[cfg(not(zopro_g2_prefix_replay))]
            norm,
        }
    }

    #[inline(always)]
    pub(super) fn norm(&self, denominator: &[FqBatch; 2]) -> FqBatch {
        #[cfg(zopro_g2_prefix_replay)]
        {
            Fq2::lanes_norm(denominator)
        }
        #[cfg(not(zopro_g2_prefix_replay))]
        {
            let _ = denominator;
            self.norm
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{Fr, G2Config, Projective};

    #[test]
    fn prefix_record_size_and_signed_owned_reuse() {
        let expected_size = if cfg!(zopro_g2_prefix_replay) {
            320
        } else {
            640
        };
        assert_eq!(core::mem::size_of::<Record>(), expected_size);
        assert_eq!(core::mem::align_of::<Record>(), 64);
        let generator = Projective::<G2Config>::generator();
        for n in [
            0usize, 1, 7, 8, 9, 15, 16, 63, 64, 199, 200, 201, 351, 352, 353, 511,
        ] {
            let size = (n + 1).next_power_of_two();
            let initial = Projective::<G2Config>::batch_to_affine(
                &(0..size)
                    .map(|i| generator * Fr::from(610031 + (i as u64) * 11))
                    .collect::<Vec<_>>(),
            );
            let incoming = Projective::<G2Config>::batch_to_affine(
                &(0..n)
                    .map(|i| generator * Fr::from(1930021 + (i as u64) * 19))
                    .collect::<Vec<_>>(),
            );
            for borrowed in [false, true] {
                for canonical in [false, true] {
                    for vector_inverse in [false, true] {
                        let mut actual = initial.clone();
                        let mut expected = initial.clone();
                        let mut scratch = Fq2AddScratch {
                            canonical,
                            vector_inverse,
                            prefix_enabled: true,
                            ..Default::default()
                        };
                        let encoded: Vec<_> = incoming
                            .iter()
                            .enumerate()
                            .map(|(i, p)| {
                                (
                                    (i * 53 % size) as u32
                                        | if borrowed && i % 3 == 1 { 1 << 31 } else { 0 },
                                    p,
                                )
                            })
                            .collect();
                        let owned: Vec<_> = encoded.iter().map(|(id, p)| (*id, **p)).collect();
                        for iteration in 0..4 {
                            for &(id, p) in &encoded {
                                let j = (id & 0x7fffffff) as usize;
                                let q = if id >> 31 != 0 { -*p } else { *p };
                                assert_ne!(expected[j].x, q.x);
                                expected[j] = (expected[j] + q).to_affine();
                            }
                            if borrowed {
                                if canonical {
                                    g2_borrowed::apply::<G2Config, true>(
                                        &mut actual,
                                        &encoded,
                                        &mut scratch,
                                    );
                                } else {
                                    g2_borrowed::apply::<G2Config, false>(
                                        &mut actual,
                                        &encoded,
                                        &mut scratch,
                                    );
                                }
                            } else if canonical {
                                g2_prefix::apply::<G2Config, true>(
                                    &mut actual,
                                    &owned,
                                    &mut scratch,
                                );
                            } else {
                                g2_prefix::apply::<G2Config, false>(
                                    &mut actual,
                                    &owned,
                                    &mut scratch,
                                );
                            }
                            assert_eq!(actual,expected,"n={n} borrowed={borrowed} canonical={canonical} inverse={vector_inverse} iteration={iteration}");
                            assert_eq!(scratch.prefix_records.len(), n / 8);
                            assert_eq!(scratch.groups.capacity(), 0);
                            let payload =
                                scratch.prefix_records.len() * core::mem::size_of::<Record>();
                            assert_eq!(payload, (n / 8) * expected_size);
                        }
                    }
                }
            }
        }
    }
}
