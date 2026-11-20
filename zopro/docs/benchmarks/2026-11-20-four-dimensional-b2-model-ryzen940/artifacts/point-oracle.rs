// Fixed 320-bit reciprocals, verified by model.py against exact rational rounding.
const BASIS: [[i128; 4]; 4] = [
    [
        9931322734385697762,
        4965661367192848881,
        4965661367192848882,
        -4965661367192848881,
    ],
    [
        -4965661367192848881,
        4965661367192848881,
        4965661367192848881,
        9931322734385697763,
    ],
    [
        -4965661367192848882,
        4965661367192848881,
        -4965661367192848881,
        -9931322734385697762,
    ],
    [
        9931322734385697763,
        4965661367192848882,
        -4965661367192848881,
        4965661367192848881,
    ],
];
const RECIPROCAL: [([u64; 5], bool); 4] = [
    (
        [
            9016205553610886694,
            5112920013033249713,
            641158657826981038,
            11421183126753848211,
            0,
        ],
        false,
    ),
    (
        [
            3836165974027454268,
            3314413021792839464,
            6175783621411297151,
            11421183126753848213,
            0,
        ],
        true,
    ),
    (
        [
            11814793079604590126,
            512368312467118847,
            6175783621411297154,
            11421183126753848213,
            0,
        ],
        true,
    ),
    (
        [
            2523014327125652736,
            13938807413310475583,
            6175783621411297149,
            11421183126753848213,
            0,
        ],
        false,
    ),
];
#[inline(always)]
fn product<const N: usize, const M: usize, const K: usize>(a: [u64; N], b: [u64; M]) -> [u64; K] {
    assert_eq!(K, N + M);
    let mut out = [0u64; K];
    for i in 0..N {
        let mut carry = 0u128;
        for j in 0..M {
            let t = a[i] as u128 * b[j] as u128 + out[i + j] as u128 + carry;
            out[i + j] = t as u64;
            carry = t >> 64;
        }
        out[i + M] = carry as u64;
    }
    out
}
// Multiplication is exact. Only the low 128 quotient bits are needed
// because the reduced signed residual fits in 64 magnitude bits.
fn rounded_coordinate(scalar: [u64; 4], reciprocal: [u64; 5], negative: bool) -> u128 {
    let product = product::<4, 5, 9>(scalar, reciprocal);
    let half = 1u64 << 63;
    let round_up = product[4] > half
        || (product[4] == half && (!negative || product[..4].iter().any(|&word| word != 0)));
    let quotient =
        ((product[5] as u128) | ((product[6] as u128) << 64)).wrapping_add(u128::from(round_up));
    if negative {
        quotient.wrapping_neg()
    } else {
        quotient
    }
}

fn split(scalar: [u64; 4]) -> [i128; 4] {
    let quotient = RECIPROCAL.map(|(limbs, sign)| rounded_coordinate(scalar, limbs, sign));
    let low = (scalar[0] as u128) | ((scalar[1] as u128) << 64);
    core::array::from_fn(|j| {
        let base = if j == 0 { low } else { 0 };
        BASIS.iter().zip(quotient).fold(base, |sum, (row, q)| {
            sum.wrapping_sub((row[j] as u128).wrapping_mul(q))
        }) as i128
    })
}

use zopro_bn254::{Fq, Fq2, Fr, G2Affine, G2Projective};

fn phi(mut point: G2Affine) -> G2Affine {
    const BETA: Fq = Fq::from_decimal(
        "21888242871839275220042445260109153167277707414472061641714758635765020556616",
    );
    point.x = point.x.mul_by_fp(&BETA);
    point
}

fn psi(point: G2Affine) -> G2Affine {
    // The same field constants as the prover's existing G2 subgroup check.
    const X: Fq2 = Fq2::new(
        Fq::from_decimal(
            "21575463638280843010398324269430826099269044274347216827212613867836435027261",
        ),
        Fq::from_decimal(
            "10307601595873709700152284273816112264069230130616436755625194854815875713954",
        ),
    );
    const Y: Fq2 = Fq2::new(
        Fq::from_decimal(
            "2821565182194536844548159561693502659359617185244120367078079554186484126554",
        ),
        Fq::from_decimal(
            "3505843767911556378687030309984248845540243509899259641013678093033130930403",
        ),
    );
    G2Affine::new(point.x.conjugate() * X, point.y.conjugate() * Y)
}

fn component(value: i128) -> Fr {
    let magnitude = Fr::from(u64::try_from(value.unsigned_abs()).unwrap());
    if value < 0 {
        -magnitude
    } else {
        magnitude
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lambda = Fr::from_decimal("4407920970296243842393367215006156084916469457145843978461");
    let mu = Fr::from_decimal("147946756881789318990833708069417712966");
    let generator = G2Affine::GENERATOR;
    for i in 0..32u64 {
        let p = (generator * Fr::from(i * 273)).to_affine();
        assert_eq!(phi(p), (p * lambda).to_affine());
        assert_eq!(psi(p), (p * mu).to_affine());
        assert_eq!(psi(phi(p)), phi(psi(p)));
    }
    let mut checked = 0;
    for input in std::env::args_os().skip(1) {
        let bytes = std::fs::read(input)?;
        assert_eq!(&bytes[..8], b"ZPB94000");
        let (entries, tail) = bytes[24..].as_chunks::<33>();
        assert!(tail.is_empty());
        let mut expected = G2Projective::IDENTITY;
        let mut partials = [G2Projective::IDENTITY; 4];
        for (i, record) in entries.iter().enumerate().step_by(127) {
            let words = core::array::from_fn(|j| {
                u64::from_le_bytes(record[1 + 8 * j..9 + 8 * j].try_into().unwrap())
            });
            let base = (generator * Fr::from(((i + 17) * 653) as u64)).to_affine();
            let scalar = Fr::from_canonical(words).unwrap();
            expected += base * scalar;
            for (out, k) in partials.iter_mut().zip(split(words)) {
                *out += base * component(k);
            }
            checked += 1;
        }
        let low = partials[0] + phi(partials[1].to_affine());
        let high = partials[2] + phi(partials[3].to_affine());
        assert_eq!(expected, low + psi(high.to_affine()));
    }
    println!("checked {checked} inputs in paired G2 group sums");
    Ok(())
}
