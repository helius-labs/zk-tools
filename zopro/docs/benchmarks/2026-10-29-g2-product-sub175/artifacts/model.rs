use num_bigint::{BigInt, BigUint, Sign};
use num_traits::{One, ToPrimitive, Zero};
use serde_json::json;
use zopro_bn254::{FpConfig, FqConfig, FrConfig};
fn digits(n: &BigUint) -> [i128; 5] {
    core::array::from_fn(|j| {
        ((n >> (52 * j)) & BigUint::from((1u64 << 52) - 1))
            .to_u64()
            .unwrap() as i128
    })
}
fn integer(t: [i128; 5]) -> BigInt {
    t.into_iter()
        .enumerate()
        .map(|(j, v)| BigInt::from(v) << (52 * j))
        .sum()
}
fn signed(t: [i128; 5], p: &BigUint, z: &BigUint) -> ([i128; 5], i128) {
    let pd = digits(p);
    let zd = digits(z);
    let mut carry = 0;
    let out = core::array::from_fn(|j| {
        let v = t[j] + pd[j] - zd[j] + carry;
        assert!(v > -(1i128 << 62) && v < (1i128 << 62));
        carry = v >> 52;
        v & ((1i128 << 52) - 1)
    });
    (out, carry)
}
fn run<C: FpConfig>() -> serde_json::Value {
    let p = BigUint::from_bytes_le(
        &C::P
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>(),
    );
    let r = BigUint::one() << 256usize;
    let mut inverse = BigUint::one();
    for _ in 0..8 {
        inverse = (&inverse * (&r + 2u32 - ((&p * &inverse) % &r))) % &r;
    }
    assert_eq!((&p * &inverse) % &r, BigUint::one());
    let negative_inverse = &r - inverse;
    let twice = &p * 2u32;
    let thrice = &p * 3u32;
    let mut seed = 0x53554250524f4f46u64;
    let mut next = |modulus: &BigUint| {
        let mut b = [0u8; 64];
        for c in b.chunks_mut(8) {
            seed ^= seed << 13;
            seed ^= seed >> 7;
            seed ^= seed << 17;
            c.copy_from_slice(&seed.to_le_bytes());
        }
        BigUint::from_bytes_le(&b) % modulus
    };
    let edge = [
        BigUint::zero(),
        BigUint::one(),
        &p - 1u32,
        p.clone(),
        &twice - 1u32,
        twice.clone(),
    ];
    let mut tuple = Vec::new();
    for i in 0usize..1296 {
        tuple.push(core::array::from_fn::<_, 4, _>(|j| {
            edge[(i / 6usize.pow(j as u32)) % 6].clone()
        }));
    }
    for _ in 0..4096 {
        tuple.push(core::array::from_fn(|_| next(&(&twice + 1u32))));
    }
    let mut count = 0;
    let mut largest = BigUint::zero();
    for a in tuple {
        let z = next(&p);
        for difference in [false, true] {
            let c = if difference {
                &twice - &a[2]
            } else {
                a[2].clone()
            };
            let x = &a[0] * &a[1] + c * &a[3];
            let q = (&x * &negative_inverse) % &r;
            let wide = &x + &q * &p;
            assert_eq!(&wide % &r, BigUint::zero());
            let t = &wide >> 256usize;
            assert!(t < thrice);
            let expanded = digits(&t);
            let (got, carry) = signed(expanded, &p, &z);
            assert_eq!(carry, 0);
            let value = integer(got);
            assert_eq!(value, BigInt::from_biguint(Sign::Plus, &t + &p - &z));
            let mut out = value.to_biguint().unwrap();
            if out >= twice {
                out -= &twice;
            }
            assert!(out < twice);
            assert_eq!((&out + &z) % &p, &t % &p);
            largest = largest.max(t);
            count += 1;
        }
    }
    let mut ts = vec![
        BigUint::zero(),
        BigUint::one(),
        &p - 1u32,
        p.clone(),
        &twice - 1u32,
        twice.clone(),
        &thrice - 1u32,
    ];
    for bits in [52, 104, 156, 208, 254] {
        let x: BigUint = BigUint::one() << (bits as usize);
        ts.extend([&x - 1u32, x.clone(), x + 1u32]);
    }
    let zs = [BigUint::zero(), BigUint::one(), &p - 1u32];
    let mut pairs = Vec::new();
    for t in ts {
        if t < thrice {
            for z in &zs {
                pairs.push((t.clone(), z.clone()));
            }
        }
    }
    for _ in 0..4096 {
        pairs.push((next(&thrice), next(&p)));
    }
    let mut normalization = 0;
    let mut min = i128::MAX;
    let mut max = 0i128;
    for (t, z) in pairs {
        let mut coeff = digits(&t);
        for j in 0..4 {
            let n = (coeff[j + 1].min(511)) as i128;
            coeff[j] += n << 52;
            coeff[j + 1] -= n;
        }
        assert_eq!(integer(coeff), BigInt::from_biguint(Sign::Plus, t.clone()));
        let (out, carry) = signed(coeff, &p, &z);
        assert_eq!(carry, 0);
        assert_eq!(integer(out), BigInt::from_biguint(Sign::Plus, &t + &p - &z));
        for c in coeff {
            assert!(c < 1i128 << 61);
            min = min.min(c);
            max = max.max(c);
        }
        normalization += 1;
    }
    json!({"modulus":format!("{p:x}"),"exact_fused_REDC_cases":count,"signed_unnormalized_normalization_cases":normalization,"maximum_modeled_raw_REDC":format!("{largest:x}"),"maximum_expanded_coefficient":max,"minimum_expanded_coefficient":min,"output_below_2p":true,"status":"passed"})
}
fn main() {
    let fq = run::<FqConfig>();
    let fr = run::<FrConfig>();
    println!(
        "{}",
        json!({"status":"passed","fields":[fq,fr],"new_exact_REDC_cases":21568,"new_normalization_cases":8324,"scope":"mixedR256 fused sums/differences and canonical subtrahend; independent BigUint REDC and signed integer normalization"})
    );
}
