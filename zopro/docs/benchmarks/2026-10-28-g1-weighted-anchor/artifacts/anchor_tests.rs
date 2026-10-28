// New graph and scratch-reuse checks; existing generator fixture remains.
impl CallerG1 {
    pub fn set_weighted(&mut self, enabled: bool) {
        self.0.scratch.fq.weighted_anchor = enabled;
    }
}

#[cfg(test)]
mod weighted_anchor_tests {
    use super::*;

    #[test]
    fn vector_y_anchor_matches_scalar_field_graph() {
        let mut rng = crate::testing::Seeded::new(0x5941_4e43_484f_5231);
        let mut field = || {
            let mut bytes = [0u8; 64];
            for chunk in bytes.chunks_exact_mut(8) {
                chunk.copy_from_slice(&rng.next_u64().to_le_bytes());
            }
            Fq::from_random_bytes(&bytes)
        };
        for k in 0..1024 {
            let ax: [Fq; LANES] = core::array::from_fn(|i| {
                if k < 8 {
                    [Fq::ZERO, Fq::ONE, -Fq::ONE, Fq::from(2u64)][(k + i) % 4]
                } else {
                    field()
                }
            });
            let bx: [Fq; LANES] = core::array::from_fn(|i| {
                if k < 8 {
                    [Fq::ZERO, Fq::ONE, -Fq::ONE, Fq::from(3u64)][(k + 3 * i) % 4]
                } else {
                    field()
                }
            });
            let ay: [Fq; LANES] = core::array::from_fn(|_| field());
            let slope: [Fq; LANES] = core::array::from_fn(|_| field());
            let a = FqBatch::new(&ax);
            let b = FqBatch::new(&bx);
            let s = FqBatch::new(&slope);
            let y = FqBatch::new(&ay);
            for fast in [false, true] {
                let sum = if fast {
                    canonical_add::<true>(&a, &b)
                } else {
                    canonical_add::<false>(&a, &b)
                };
                let anchor = a.add(&sum);
                let square = s.mul(&s);
                let x = square.sub(&sum);
                let output_y = s.mul(&anchor.sub(&square)).sub(&y);
                let (x, output_y) = (x.get(), output_y.get());
                for i in 0..LANES {
                    let want_x = slope[i].square() - ax[i] - bx[i];
                    let want_y = slope[i] * (ax[i] - want_x) - ay[i];
                    assert_eq!(x[i], want_x);
                    assert_eq!(output_y[i], want_y);
                }
            }
        }
    }

    #[test]
    fn alternating_anchor_modes_preserve_owned_borrowed_tails_and_scratch() {
        for n in [0, 1, 7, 8, 9, 31, 199, 200, 201, 351, 352, 353, 504] {
            for canonical in [false, true] {
                for vector in [false, true] {
                    for write in [false, true] {
                        let mut f = CallerG1::new(n, false, 17);
                        f.0.scratch.fq.canonical = canonical;
                        f.0.scratch.fq.vector_inverse = vector;
                        f.0.scratch.fq.write_groups = write;
                        let mut rounds = 0;
                        for enabled in [false, true, false, true] {
                            f.set_weighted(enabled);
                            f.run(3);
                            rounds += 3;
                            f.check(rounds);
                        }
                        let mut f = CallerG1::new(n, true, 19);
                        f.0.scratch.fq.canonical = canonical;
                        f.0.scratch.fq.vector_inverse = vector;
                        f.0.scratch.fq.write_groups = write;
                        let mut rounds = 0;
                        for enabled in [false, true, false, true] {
                            f.set_weighted(enabled);
                            f.with_borrowed(|b| {
                                b.run(3);
                                rounds += 3;
                                b.check(rounds);
                            });
                        }
                    }
                }
            }
        }
    }
}
