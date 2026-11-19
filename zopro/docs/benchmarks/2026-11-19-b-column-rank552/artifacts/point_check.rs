use ark_bn254::{Fq, Fq2, Fr, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInt, PrimeField, Zero};
use std::io::{Cursor, Read};

fn field<F: PrimeField<BigInt = BigInt<4>>>(input: &mut Cursor<Vec<u8>>) -> F {
    let mut bytes = [0u8; 32];
    input.read_exact(&mut bytes).unwrap();
    let limbs = std::array::from_fn(|i| u64::from_le_bytes(bytes[8*i..8*i+8].try_into().unwrap()));
    F::from_bigint(BigInt::new(limbs)).expect("canonical field element")
}

fn point(input: &mut Cursor<Vec<u8>>) -> G2Affine {
    let x = Fq2::new(field::<Fq>(input), field::<Fq>(input));
    let y = Fq2::new(field::<Fq>(input), field::<Fq>(input));
    let p = G2Affine::new_unchecked(x, y);
    assert!(p.is_on_curve() && p.is_in_correct_subgroup_assuming_on_curve());
    p
}

fn main() {
    let path = std::env::args().nth(1).unwrap();
    let mut input = Cursor::new(std::fs::read(path).unwrap());
    let mut checked = 0;
    while input.position() < input.get_ref().len() as u64 {
        let expected = point(&mut input);
        let mut count = [0; 4];
        input.read_exact(&mut count).unwrap();
        let mut sum = G2Projective::zero();
        for _ in 0..u32::from_le_bytes(count) {
            let scalar = field::<Fr>(&mut input);
            sum += point(&mut input).mul_bigint(scalar.into_bigint());
        }
        assert_eq!(sum.into_affine(), expected);
        checked += 1;
    }
    assert!(checked > 0);
    println!("verified {checked} G2 relations");
}
