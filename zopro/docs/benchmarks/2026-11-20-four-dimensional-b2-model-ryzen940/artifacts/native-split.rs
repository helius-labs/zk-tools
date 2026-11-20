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
const C1: [u64; 3] = [7978627105577135859, 15644699364383830999, 2];
const C2: [u64; 4] = [
    11953552847224317658,
    8825887400277225869,
    5534624963584316111,
    2,
];
const V1X: u128 = 9931322734385697763;
const V1Y_ABS: u128 = 147946756881789319000765030803803410728;
const V2X: u128 = 147946756881789319010696353538189108491;
const V2Y: u128 = 9931322734385697763;
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
#[inline(always)]
fn sign_magnitude(value: u128) -> u128 {
    let sign = ((value as i128) >> 127) as u128;
    let m = (value ^ sign).wrapping_sub(sign);
    assert!(m < 1u128 << 126);
    m | (sign & (1u128 << 127))
}
fn phi_split(s: [u64; 4]) -> [u64; 4] {
    let a = product::<2, 3, 5>([s[2], s[3]], C1);
    let b = product::<3, 4, 7>([s[1], s[2], s[3]], C2);
    debug_assert_eq!(a[4], 0);
    debug_assert_eq!(b[6], 0);
    let q1 = a[3] as u128 + u128::from(a[2] >> 63);
    let q2 = (b[4] as u128 | ((b[5] as u128) << 64)) + u128::from(b[3] >> 63);
    let low = s[0] as u128 | ((s[1] as u128) << 64);
    let k1 = low
        .wrapping_sub(q1.wrapping_mul(V1X))
        .wrapping_sub(q2.wrapping_mul(V2X));
    let k2 = q1.wrapping_mul(V1Y_ABS).wrapping_sub(q2.wrapping_mul(V2Y));
    let (a, b) = (sign_magnitude(k1), sign_magnitude(k2));
    [a as u64, (a >> 64) as u64, b as u64, (b >> 64) as u64]
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

fn time<T>(words: &[[u64; 4]], operation: impl Fn([u64; 4]) -> T) -> u128 {
    let start = std::time::Instant::now();
    for word in words {
        std::hint::black_box(operation(std::hint::black_box(*word)));
    }
    start.elapsed().as_nanos()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    use std::io::{BufWriter, Write};
    let args = std::env::args_os().skip(1).collect::<Vec<_>>();
    if args.len() != 2 {
        return Err("usage: split BORROWED_INPUT OUTPUT".into());
    }
    let input = std::fs::read(&args[0])?;
    assert_eq!(&input[..8], b"ZPB94000");
    let (entries, remainder) = input[24..].as_chunks::<33>();
    assert!(remainder.is_empty());
    let words: Vec<[u64; 4]> = entries
        .iter()
        .map(|r| {
            core::array::from_fn(|i| {
                u64::from_le_bytes(r[1 + i * 8..9 + i * 8].try_into().unwrap())
            })
        })
        .collect();
    let mut output = BufWriter::new(std::fs::File::create(&args[1])?);
    for &word in &words {
        for component in split(word) {
            output.write_all(&component.to_le_bytes())?;
        }
    }
    output.flush()?;
    for _ in 0..3 {
        time(&words, phi_split);
        time(&words, split);
    }
    for block in 0..12 {
        let (two, four) = if block % 2 == 0 {
            (time(&words, phi_split), time(&words, split))
        } else {
            let four = time(&words, split);
            let two = time(&words, phi_split);
            (two, four)
        };
        println!("{block} {two} {four}");
    }
    Ok(())
}
