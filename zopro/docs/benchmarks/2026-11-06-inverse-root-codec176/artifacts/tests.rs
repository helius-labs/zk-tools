use super::*;
#[test]
fn half_roots_preserve_lazy_products_and_all_inverse_lanes() {
    let mut state = Fq::from(0x726f6f7473u64);
    let mut checked = 0usize;
    for case in 0..1025usize {
        let mut expected = [Fq::ONE; 8];
        let mut product = FqBatch::one();
        for round in 0..1 + case % 7 {
            let values = core::array::from_fn(|lane| {
                state = state.square() + Fq::from((case * 47 + round * 13 + lane + 1) as u64);
                match case % 11 {
                    0 => Fq::ONE,
                    1 => -Fq::ONE,
                    2 if lane % 2 == 0 => -Fq::ONE,
                    _ => {
                        if state.is_zero() {
                            Fq::ONE
                        } else {
                            state
                        }
                    }
                }
            });
            for lane in 0..8 {
                expected[lane] *= values[lane];
            }
            product = product.mul(&FqBatch::new(&values));
        }
        let (lo, hi) = product.0.halves();
        let actual = product.get();
        assert_eq!(lo.get()[0], actual[0]);
        assert_eq!(hi.get()[0], actual[4]);
        assert_eq!(actual, expected);
        let output = product.inverse_nonzero().get();
        for lane in 0..8 {
            assert_eq!(output[lane], expected[lane].inverse().unwrap());
            assert_eq!(output[lane] * expected[lane], Fq::ONE);
            checked += 1;
        }
    }
    assert_eq!(checked, 8200);
    println!("HALF_ROOT_ORACLE 1025fullwidth/lazy/pattern cases;8200 inverse lanes plus2050 root exports");
}
