            if cfg!(zopro_split_redc_planes) && LANES == 8 {
                // Keep product and reduction coefficients independent until
                // their low frontier is needed by the Montgomery quotient.
                let mut products = [zero; 6];
                let mut reductions = [zero; 6];
                macro_rules! split_row {
                    ($i:literal) => {{
                        macro_rules! product {
                            ($j:literal,$k:literal) => {
                                products[$j] = _mm256_madd52lo_epu64(products[$j], self.0[$j], b.0[$i]);
                                products[$k] = _mm256_madd52hi_epu64(products[$k], self.0[$j], b.0[$i]);
                            };
                        }
                        product!(0,1);
                        product!(1,2);
                        product!(2,3);
                        product!(3,4);
                        product!(4,5);
                        if FUSED {
                            macro_rules! second {
                                ($j:literal,$k:literal) => {
                                    products[$j] = _mm256_madd52lo_epu64(products[$j], c.0[$j], d.0[$i]);
                                    products[$k] = _mm256_madd52hi_epu64(products[$k], c.0[$j], d.0[$i]);
                                };
                            }
                            second!(0,1);
                            second!(1,2);
                            second!(2,3);
                            second!(3,4);
                            second!(4,5);
                        }
                        let frontier = _mm256_add_epi64(products[0], reductions[0]);
                        let m = _mm256_madd52lo_epu64(zero, frontier, inv);
                        let carry = _mm256_srli_epi64::<52>(_mm256_add_epi64(frontier, mask));
                        let low = _mm256_madd52hi_epu64(carry, m, p[0]);
                        macro_rules! reduction {
                            ($j:literal,$k:literal) => {
                                reductions[$j] = _mm256_madd52lo_epu64(reductions[$j], m, p[$j]);
                                reductions[$k] = _mm256_madd52hi_epu64(reductions[$k], m, p[$j]);
                            };
                        }
                        reduction!(1,2);
                        reduction!(2,3);
                        reduction!(3,4);
                        reduction!(4,5);
                        reductions[1] = _mm256_add_epi64(reductions[1], low);
                        products = [products[1],products[2],products[3],products[4],products[5],zero];
                        reductions = [reductions[1],reductions[2],reductions[3],reductions[4],reductions[5],zero];
                    }};
                }
                split_row!(0);
                split_row!(1);
                split_row!(2);
                split_row!(3);
                t = core::array::from_fn(|j| _mm256_add_epi64(products[j], reductions[j]));
            } else {
                row!(0);
                row!(1);
                row!(2);
                row!(3);
            }
