//! Match the norm and numerator scale; all stored values remain R256.
use super::*;
#[inline(always)]
pub(super) fn norm(dx: &[FqBatch; 2]) -> FqBatch {
    if cfg!(zopro_scaled_norm_numerator) {
        dx[0].sum2_div16(&dx[0], &dx[1], &dx[1])
    } else {
        Fq2::lanes_norm(dx)
    }
}
#[inline(always)]
pub(super) fn numerator(dy: &[FqBatch; 2], dx: &[FqBatch; 2], factor: &FqBatch) -> [FqBatch; 2] {
    if cfg!(zopro_scaled_norm_numerator) {
        let value = [
            dy[0].sum2_div16(&dx[0], &dy[1], &dx[1]),
            dy[1].difference2_div16(&dx[0], &dy[0], &dx[1]),
        ];
        Fq2::lanes_scale(&value, factor)
    } else {
        Fq2::lanes_scaled_numerator(dy, dx, factor)
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};
    use zopro_bn254::G2Config;
    #[test]
    fn scaled_norm_activation() {
        let x = core::array::from_fn(|i| Fq::from_u64(i as u64 + 2));
        let y = core::array::from_fn(|i| Fq::from_u64(i as u64 + 71));
        let v = [FqBatch::new(&x), FqBatch::new(&y)];
        let h = if cfg!(zopro_scaled_norm_numerator) {
            Fq::from_u64(16).inverse().unwrap()
        } else {
            Fq::ONE
        };
        assert_eq!(
            norm(&v).get(),
            core::array::from_fn(|i| (x[i].square() + y[i].square()) * h)
        );
        assert_eq!(core::mem::size_of::<[FqBatch; 2]>(), 640);
    }
    fn points(n: usize, label: &str) -> Vec<Affine<G2Config>> {
        let g = Projective::<G2Config>::generator();
        let base = Fr::from_le_bytes_mod_order(&Sha256::digest(
            format!("scalednorm-native-8a31fc7d/{label}/base").as_bytes(),
        ));
        let delta = Fr::from_le_bytes_mod_order(&Sha256::digest(
            format!("scalednorm-native-8a31fc7d/{label}/delta").as_bytes(),
        ));
        let mut p = g * base;
        let step = g * delta;
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(p);
            p += step;
        }
        Projective::<G2Config>::batch_to_affine(&v)
    }
    #[test]
    fn scaled_norm_signed_owned_tails_reuse() {
        for n in [
            0usize, 1, 7, 8, 9, 15, 16, 31, 32, 63, 64, 199, 200, 201, 351, 352, 353, 511,
        ] {
            let size = (n + 1).next_power_of_two();
            let initial = points(size, &format!("{n}/bucket"));
            let incoming = points(n, &format!("{n}/incoming"));
            for borrowed in [false, true] {
                for canonical in [false, true] {
                    for vector_inverse in [false, true] {
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
                        let mut actual = initial.clone();
                        let mut expected = initial.clone();
                        let mut s = Fq2AddScratch {
                            canonical,
                            vector_inverse,
                            prefix_enabled: true,
                            ..Default::default()
                        };
                        for round in 0..3 {
                            for &(id, p) in &encoded {
                                let j = (id & 0x7fffffff) as usize;
                                let b = if id >> 31 != 0 { -*p } else { *p };
                                assert_ne!(expected[j].x, b.x);
                                expected[j] = (expected[j] + b).to_affine();
                            }
                            if borrowed {
                                if canonical {
                                    g2_borrowed::apply::<G2Config, true>(
                                        &mut actual,
                                        &encoded,
                                        &mut s,
                                    );
                                } else {
                                    g2_borrowed::apply::<G2Config, false>(
                                        &mut actual,
                                        &encoded,
                                        &mut s,
                                    );
                                }
                            } else if canonical {
                                g2_prefix::apply::<G2Config, true>(&mut actual, &owned, &mut s);
                            } else {
                                g2_prefix::apply::<G2Config, false>(&mut actual, &owned, &mut s);
                            }
                            assert_eq!(actual,expected,"n={n}borrowed={borrowed}canonical={canonical}inverse={vector_inverse}round={round}");
                            assert_eq!(s.prefix_records.len(), n / 8);
                            assert_eq!(s.groups.capacity(), 0);
                        }
                    }
                }
            }
        }
    }
}
