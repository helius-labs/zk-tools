use zopro_core::TreeCaller;
#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::{Fr, G1Projective};
    use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
    use ark_ff::{AdditiveGroup, BigInt, Field, PrimeField};
    fn word(seed: &mut u64) -> [u64; 4] {
        loop {
            let mut w = [0; 4];
            for x in &mut w {
                *seed ^= *seed << 13;
                *seed ^= *seed >> 7;
                *seed ^= *seed << 17;
                *x = *seed;
            }
            w[3] &= 0x3fffffffffffffff;
            if Fr::from_bigint(BigInt(w)).is_some() {
                return w;
            }
        }
    }
    fn coordinates(s: Fr) -> [[u64; 4]; 2] {
        let p = (G1Projective::generator() * s).into_affine();
        match p.xy() {
            Some((x, y)) => [x.into_bigint().0, y.into_bigint().0],
            None => [[0; 4]; 2],
        }
    }
    fn check(words: &[[u64; 4]], counts: &[usize]) {
        let mut f = TreeCaller::from_words(words, counts);
        let scalars: Vec<_> = f
            .order()
            .iter()
            .map(|&i| Fr::from_bigint(BigInt(words[i as usize])).unwrap())
            .collect();
        let mut offset = 0;
        let expected: Vec<_> = counts
            .iter()
            .map(|&n| {
                let sum = scalars[offset..offset + n].iter().copied().sum();
                offset += n;
                coordinates(sum)
            })
            .collect();
        for fresh in [false, true] {
            f.run(fresh, 3);
            f.check();
            assert_eq!(f.coordinates(), expected);
        }
    }
    #[test]
    fn complete_grouped_and_indexed_outputs_match_arkworks() {
        let mut seed = 0x128f961aa734bc21;
        for n in [
            0, 1, 2, 3, 7, 8, 9, 15, 16, 17, 31, 32, 33, 127, 257, 384, 511, 2048, 35973,
        ] {
            let words: Vec<_> = (0..n).map(|_| word(&mut seed)).collect();
            check(&words, &[n]);
        }
        let counts = [0, 1, 2, 3, 7, 8, 9, 23, 24, 25, 26, 47, 48, 49];
        let words: Vec<_> = (0..counts.iter().sum()).map(|_| word(&mut seed)).collect();
        check(&words, &counts);
        for a in [Fr::ZERO, Fr::ONE, -Fr::ONE, Fr::from(2u64)] {
            for b in [Fr::ZERO, Fr::ONE, -Fr::ONE, Fr::from(2u64)] {
                let mut w: Vec<_> = (0..96).map(|_| word(&mut seed)).collect();
                for i in (0..96).step_by(7) {
                    w[i] = a.into_bigint().0;
                    w[i + 1] = b.into_bigint().0;
                }
                check(&w, &[17, 31, 0, 48]);
            }
        }
        println!("G1_TREE_ARK grouped cases=36 full254bit/boundary/mixed/fresh+reuse");
    }
    #[test]
    fn complete_weighted_outputs_match_arkworks() {
        let mut seed = 0x7318fa462b1e9c2d;
        for n in [
            0usize, 1, 2, 4, 8, 16, 32, 64, 128, 256, 257, 258, 260, 264, 272, 288, 320, 384, 512,
            1024, 2048, 4096,
        ] {
            let w: Vec<_> = (0..n).map(|_| word(&mut seed)).collect();
            let sum = w.iter().enumerate().fold(Fr::ZERO, |a, (i, w)| {
                a + Fr::from((i + 1) as u64) * Fr::from_bigint(BigInt(*w)).unwrap()
            });
            assert_eq!(TreeCaller::weighted_words(&w), coordinates(sum));
        }
        println!("G1_TREE_ARK weighted cases=22 full254bit/multiple+partial chunks");
    }
}
