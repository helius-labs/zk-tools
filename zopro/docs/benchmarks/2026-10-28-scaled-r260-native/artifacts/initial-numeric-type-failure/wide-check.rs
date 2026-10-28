use num_bigint::BigUint as U;
use zopro_bn254::ifma8::scaled::diagnostics as native;
use zopro_bn254::{FpConfig, Fq, FqConfig};
fn u(x: u64) -> U {
    U::from(x)
}
fn low(x: &U) -> u64 {
    x.to_u64_digits().first().copied().unwrap_or(0)
}
fn modulus() -> U {
    U::from_bytes_le(
        &FqConfig::P
            .iter()
            .flat_map(|w| w.to_le_bytes())
            .collect::<Vec<_>>(),
    )
}
fn random(seed: &mut u64) -> U {
    let mut b = [0; 40];
    for c in b.chunks_mut(8) {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        c.copy_from_slice(&seed.to_le_bytes());
    }
    U::from_bytes_le(&b)
}
fn digits(a: &U) -> [u64; 5] {
    let mask = u((1 << 52) - 1);
    assert!(a.bits() <= 260);
    core::array::from_fn(|j| low(&((a >> (j * 52)) & &mask)))
}
fn integer(a: [u64; 5]) -> U {
    a.iter().rev().fold(u(0), |acc, w| (acc << 52usize) + u(*w))
}
fn field(a: &U) -> Fq {
    let mut w = a.to_u64_digits();
    assert!(w.len() <= 4);
    w.resize(4, 0);
    Fq::from_montgomery(w.try_into().unwrap())
}
fn raw(a: Fq) -> U {
    U::from_bytes_le(&a.0.iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<_>>())
}
#[test]
fn native_cios_wide_domains_match_full_radix_redc() {
    let p = modulus();
    let r = u(1) << 260usize;
    let mask = &r - u(1);
    let phi = (&r >> 1usize) - u(1);
    let nprime = (&r - p.modpow(&phi, &r)) & &mask;
    let mut seed = 0x72b5d36e9412ca6f;
    let mut total = 0;
    for (name, limits, fused, bound) in [
        ("ordinary", [8, 8, 0, 0], false, 2),
        ("norm", [8, 8, 8, 8], true, 3),
        ("numerator", [4, 8, 4, 8], true, 2),
        ("finish", [2, 16, 2, 16], true, 2),
        ("prefix", [2, 3, 0, 0], false, 2),
    ] {
        let choices: Vec<Vec<U>> = limits
            .iter()
            .map(|n| {
                if *n == 0 {
                    vec![u(0)]
                } else {
                    vec![u(0), u(1), &p - u(1), &p * u(*n) - u(2), &p * u(*n) - u(1)]
                }
            })
            .collect();
        let mut cases = vec![];
        for a in &choices[0] {
            for b in &choices[1] {
                for c in &choices[2] {
                    for d in &choices[3] {
                        cases.push([a.clone(), b.clone(), c.clone(), d.clone()]);
                    }
                }
            }
        }
        for _ in 0..8192 {
            cases.push(core::array::from_fn(|i| {
                if limits[i] == 0 {
                    u(0)
                } else {
                    random(&mut seed) % (&p * u(limits[i]))
                }
            }));
        }
        for chunk in cases.chunks(8) {
            let v: [[U; 4]; 8] = core::array::from_fn(|i| chunk[i % chunk.len()].clone());
            let a = core::array::from_fn(|i| digits(&v[i][0]));
            let b = core::array::from_fn(|i| digits(&v[i][1]));
            let c = core::array::from_fn(|i| digits(&v[i][2]));
            let d = core::array::from_fn(|i| digits(&v[i][3]));
            let out = native::cios(a, b, c, d, fused);
            for i in 0..8 {
                let mut t = &v[i][0] * &v[i][1];
                if fused {
                    t += &v[i][2] * &v[i][3];
                }
                let expected = (&t + ((&t * &nprime) & &mask) * &p) >> 260usize;
                let actual = integer(out[i]);
                assert_eq!(actual, expected, "{name}/lane{i}");
                assert!(actual < &p * u(bound));
            }
        }
        println!(
            "{name}:{} distinct scalar tuples; padded SIMD lanes are repeats",
            cases.len()
        );
        total += cases.len();
    }
    assert_eq!(total, 42885);
}
#[test]
fn native_coordinate_codecs_match_host_raw() {
    let p = modulus();
    let mut seed = 0x24f853d197ac6b01;
    let mut values = vec![u(0), u(1), &p - u(1), &p - u(2)];
    for k in [
        1usize, 2, 51, 52, 53, 63, 64, 65, 103, 104, 105, 127, 128, 155, 156, 157, 191, 192, 207,
        208, 209, 251, 252, 253,
    ] {
        values.push((u(1) << k) % &p);
    }
    for _ in 0..2048 {
        values.push(random(&mut seed) % &p);
    }
    for chunk in values.chunks(8) {
        let a = core::array::from_fn(|i| field(&chunk[i % chunk.len()]));
        assert_eq!(native::codec_x(a), a);
        assert_eq!(native::codec_y(a), a);
    }
    println!(
        "{} canonical raw input cases; both X and Y codecs; padded lanes repeated",
        values.len()
    );
}
#[test]
fn native_wide_exports_match_modular_division() {
    let p = modulus();
    let mut seed = 0x1f2d398ac04567b9;
    let mut total = 0;
    for (x, mult, k) in [(true, 12u64, 2usize), (false, 4u64, 1usize)] {
        let inv = (u(1) << k).modpow(&(&p - u(2)), &p);
        let mut cases = vec![
            u(0),
            u(1),
            &p - u(1),
            &p,
            &p + u(1),
            &p * u(mult) - u(1),
            &p * u(mult),
        ];
        for n in 0..mult {
            for j in 0..4 {
                cases.push(&p * u(n) + u(j));
            }
        }
        for _ in 0..8192 {
            cases.push(random(&mut seed) % (&p * u(mult)));
        }
        for chunk in cases.chunks(8) {
            let v: [U; 8] = core::array::from_fn(|i| chunk[i % chunk.len()].clone());
            let out = native::export_raw(core::array::from_fn(|i| digits(&v[i])), x);
            for i in 0..8 {
                assert_eq!(raw(out[i]), (&v[i] * &inv) % &p);
                assert!(raw(out[i]) < p);
            }
        }
        println!(
            "{} wide export cases x={x}; inclusive bound included",
            cases.len()
        );
        total += cases.len();
    }
    assert_eq!(total, 16462);
}
