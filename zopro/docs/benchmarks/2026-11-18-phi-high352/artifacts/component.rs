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
    signed_residuals(s, q1, q2)
}

#[inline(always)]
fn signed_residuals(s: [u64; 4], q1: u128, q2: u128) -> [u64; 4] {
    let low = s[0] as u128 | ((s[1] as u128) << 64);
    let k1 = low
        .wrapping_sub(q1.wrapping_mul(V1X))
        .wrapping_sub(q2.wrapping_mul(V2X));
    let k2 = q1.wrapping_mul(V1Y_ABS).wrapping_sub(q2.wrapping_mul(V2Y));
    let (a, b) = (sign_magnitude(k1), sign_magnitude(k2));
    [a as u64, (a >> 64) as u64, b as u64, (b >> 64) as u64]
}
// Compute the rounded quotient by B^cut for B = 2^64. Accumulate whole
// limbs of the product in units of B^(cut-1). In the omitted diagonals
// each partial product contributes strictly less than one such unit;
// each truncated partial product on the boundary also loses less than
// one. If their sum could cross the rounding threshold, use the exact
// product. Both quotients here fit in 128 bits on canonical scalars.
#[inline(always)]
fn high_quotient<const N: usize, const M: usize, const K: usize, const CUT: usize>(
    a: [u64; N], b: [u64; M],
) -> u128 {
    let mut upper = [0u64; 4];
    for i in 0..N {
        let mut carry = 0u128;
        let first = (CUT - 2).saturating_sub(i);
        for j in first..M {
            let term = a[i] as u128 * b[j] as u128;
            if i + j == CUT - 2 {
                carry = term >> 64;
            } else {
                let index = i + j - (CUT - 1);
                let sum = term + upper[index] as u128 + carry;
                upper[index] = sum as u64;
                carry = sum >> 64;
            }
        }
        upper[i + M - (CUT - 1)] = carry as u64;
    }
    let half = 1u64 << 63;
    if (half - (N * M) as u64..half).contains(&upper[0]) {
        let exact = product::<N, M, K>(a, b);
        let high = exact.get(CUT + 1).copied().unwrap_or(0);
        return ((exact[CUT] as u128) | (high as u128) << 64)
            .wrapping_add((exact[CUT-1] >> 63) as u128);
    }
    ((upper[1] as u128) | (upper[2] as u128) << 64)
        .wrapping_add((upper[0] >> 63) as u128)
}

#[inline(never)]
fn split_high(s: [u64; 4]) -> [u64; 4] {
    let q1 = high_quotient::<2,3,5,3>([s[2],s[3]], C1);
    let q2 = high_quotient::<3,4,7,4>([s[1],s[2],s[3]], C2);
    signed_residuals(s,q1,q2)
}

fn main() {
    let b = u64::MAX;
    let half = 1u64 << 63;
    let mut decisions = 0usize;
    let mut changed = 0usize;
    for x in (1u64 << 62)-64..=(1u64 << 62)+64 {
        for y in 0..64 {
            let a = [b-y,1];
            let c = [b, x, 0];
            let exact = product::<2,3,5>(a,c);
            let required = exact[3] as u128 + (exact[2] >> 63) as u128;
            let low_approx = (a[0] as u128 * x as u128 >> 64) + x as u128;
            let approx = (low_approx + half as u128) >> 64;
            let rem = low_approx as u64;
            decisions += (half-6..half).contains(&rem) as usize;
            changed += (approx != required) as usize;
            assert_eq!(high_quotient::<2,3,5,3>(a,c),required);
        }
    }
    assert!(changed > 0 && decisions >= changed);
    println!("boundary_pairs={} exact_fallback_band={} crossing_threshold={}",129*64,decisions,changed);
    let mut state=235634u64;
    let mut next=|| { state^=state<<13;state^=state>>7;state^=state<<17;state };
    let inputs:Vec<_>=(0..4096).map(|_|[next(),next(),next(),next()>>3]).collect();
    assert!(inputs.iter().all(|&s|phi_split(s)==split_high(s)));
    for b in 0..41 {
        for m in if b%2==0 {[0,1]} else {[1,0]} {
            let start=std::time::Instant::now();
            for _ in 0..16 {
                for &s in std::hint::black_box(&inputs) {
                    std::hint::black_box(if m==0 {phi_split(s)} else {split_high(s)});
                }
            }
            println!("{} {} {:.3}",b,m,start.elapsed().as_nanos() as f64/(16.*4096.));
        }
    }
}
