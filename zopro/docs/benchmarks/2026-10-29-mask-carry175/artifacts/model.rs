use num_bigint::BigUint;
use serde_json::json;
fn masked(mut t: [u64; 5], bits: u32) -> [u64; 5] {
    let b = 1u64 << bits;
    let mask = b - 1;
    let q = t.map(|x| x >> bits);
    for j in 0..4 {
        t[j] &= mask;
    }
    for j in 1..5 {
        t[j] += q[j - 1];
    }
    let c1 = u64::from(t[1] >= b);
    let c2 = u64::from(t[2] >= b || (t[2] == mask && c1 != 0));
    let c3 = u64::from(t[3] >= b || (t[3] == mask && c2 != 0));
    [
        t[0],
        t[1] & mask,
        (t[2] + c1) & mask,
        (t[3] + c2) & mask,
        t[4] + c3,
    ]
}
fn check(t: [u64; 5], bits: u32) {
    let got = masked(t, bits);
    let mut want = t;
    for j in 0..4 {
        want[j + 1] += want[j] >> bits;
        want[j] &= (1u64 << bits) - 1;
    }
    assert_eq!(got, want);
    let integer = |a: [u64; 5]| {
        a.into_iter()
            .enumerate()
            .map(|(j, v)| BigUint::from(v) << (bits as usize * j))
            .sum::<BigUint>()
    };
    assert_eq!(integer(t), integer(got));
}
fn main() {
    let mut n = 0;
    for bits in [4u32, 8, 52] {
        let b = 1u64 << bits;
        let limit = if bits == 52 { 1u64 << 61 } else { b * b };
        let edge = [0, 1, b - 2, b - 1, b, b + 1, limit - 2, limit - 1];
        for code in 0usize..32768 {
            check(core::array::from_fn(|j| edge[(code >> (3 * j)) & 7]), bits);
            n += 1;
        }
        let mut seed = 0x4e4f524d4d41534bu64;
        for _ in 0..8192 {
            check(
                core::array::from_fn(|_| {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    seed % limit
                }),
                bits,
            );
            n += 1;
        }
    }
    let mut exhaustive = 0;
    for code in 0u64..(1 << 20) {
        let t = core::array::from_fn(|j| (code >> (4 * j)) & 15);
        let got = masked(t, 2);
        let integer = |a: [u64; 5]| {
            a.into_iter()
                .enumerate()
                .map(|(j, x)| (x as u128) << (2 * j))
                .sum::<u128>()
        };
        assert_eq!(integer(t), integer(got));
        assert!(got[..4].iter().all(|v| *v < 4));
        exhaustive += 1;
    }
    println!(
        "{}",
        json!({"normalization_cases":n,"independent_BigUint_integer_equalities":n,"exhaustive_radix4_all_coefficient_arrays":exhaustive,"radices":[16,256,4503599627370496u64],"maximum_native_coefficient_exclusive":1u64<<61,"formula_condition":"t_j < B^2, ensuring each second-stage overflow is at most one","status":"passed"})
    );
}
