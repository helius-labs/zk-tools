//! Adjacent products share the field backend's existing Montgomery reduction.
use super::Fr;

pub(super) fn dot(coefficients: &[Fr], values: &[Fr]) -> Fr {
    assert_eq!(coefficients.len(), values.len());
    let mut result = Fr::ZERO;
    let mut i = 0;
    while i + 1 < values.len() {
        result += Fr::sum_of_products(
            &coefficients[i], &values[i],
            &coefficients[i + 1], &values[i + 1],
        );
        i += 2;
    }
    if i < values.len() {
        result += coefficients[i] * values[i];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn odd_even_empty_and_boundary_inputs_match_separate_products() {
        let mut random = 0x243f_6a88_85a3_08d3u64;
        let mut field = || {
            let mut words = [0u64; 4];
            for word in &mut words {
                random ^= random << 13;
                random ^= random >> 7;
                random ^= random << 17;
                *word = random;
            }
            words[3] &= 0x1fff_ffff_ffff_ffff;
            Fr::from_canonical(words).unwrap()
        };
        for length in 0..=13 {
            for family in 0..16 {
                let coefficients: Vec<_> = (0..length).map(|i| match (i + family) % 8 {
                    0 => Fr::ZERO, 1 => Fr::ONE, 2 => -Fr::ONE,
                    3 => Fr::from(2u64), 4 => -Fr::from(2u64), _ => field(),
                }).collect();
                let values: Vec<_> = (0..length).map(|i| match (i + family) % 7 {
                    0 => -Fr::ONE, 1 => Fr::ZERO, 2 => Fr::ONE, _ => field(),
                }).collect();
                let expected = coefficients.iter().zip(&values)
                    .fold(Fr::ZERO, |sum, (a, b)| sum + *a * *b);
                assert_eq!(dot(&coefficients, &values), expected, "length={length} family={family}");
            }
        }
    }

    #[test]
    fn width_three_dense_and_partial_products_preserve_values() {
        let (partial, constants) = super::super::parameters(3);
        let mds = 24 + partial;
        let state = [Fr::ZERO, -Fr::ONE, Fr::from(23u64)];
        for offset in [mds, mds + 9] {
            for row in 0..3 {
                let start = offset + row * 3;
                let expected = constants[start] * state[0]
                    + constants[start + 1] * state[1]
                    + constants[start + 2] * state[2];
                assert_eq!(dot(&constants[start..start + 3], &state), expected);
            }
        }
        for round in 0..partial {
            let offset = mds + 18 + round * 5;
            assert_eq!(dot(&constants[offset + 1..offset + 3], &state[1..]),
                constants[offset + 1] * state[1] + constants[offset + 2] * state[2]);
        }
    }
}
