#[test]
fn reduced_radix_exact_quotient_remainder_and_signed_carries() {
    let mut seed = 0x626f756e64735f31u64;
    let mut checked = 0u64;
    for bits in [10u32, 12] {
        let b = 1u128 << bits;
        let l = b.pow(4);
        for p in [
            12 * l + l / 3 + 1,
            13 * l - l / 7,
            49 * l + l / 5,
            61 * l - l / 3,
        ] {
            let top = p / l;
            let u = top + 1;
            assert!(
                7 * b + 4 * top * u < top * b,
                "conservative error is below one"
            );
            let mu = b / u;
            let twice = 2 * p;
            let edge = [0, 1, p - 1, p, p + 1, twice - 1, twice];
            let mut pairs: Vec<_> = edge
                .iter()
                .flat_map(|&a| edge.iter().map(move |&c| (a, c)))
                .collect();
            for _ in 0..100_000 {
                let mut next = || {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    seed as u128 % (twice + 1)
                };
                pairs.push((next(), next()));
            }
            for (a, c) in pairs {
                let digits = |mut x: u128| -> [i128; 5] {
                    core::array::from_fn(|_| {
                        let v = (x % b) as i128;
                        x /= b;
                        v
                    })
                };
                let av = digits(a);
                let cv = digits(c);
                let bias = digits(twice);
                for sub in [false, true] {
                    let raw: [i128; 5] = core::array::from_fn(|j| {
                        if sub {
                            bias[j] + av[j] - cv[j]
                        } else {
                            av[j] + cv[j]
                        }
                    });
                    let x = if sub { twice + a - c } else { a + c };
                    let h = (raw[4] - 1).max(0) as u128;
                    assert!(h * l <= x && x - h * l < 3 * l);
                    let q = h * mu / b;
                    assert!(q < 4 && q * p <= x);
                    let pd = digits(p);
                    let mut carry = 0i128;
                    let mut out = 0u128;
                    let mut weight = 1u128;
                    for j in 0..5 {
                        let t = raw[j] - q as i128 * pd[j] + carry;
                        assert!(t.abs() < 8 * b as i128);
                        carry = t >> bits;
                        let d = (t & (b as i128 - 1)) as u128;
                        out += d * weight;
                        weight *= b;
                    }
                    assert_eq!(carry, 0);
                    assert_eq!(out, x - q * p);
                    assert!(out < twice);
                    assert_eq!(
                        out % p,
                        if sub {
                            (twice + a - c) % p
                        } else {
                            (a + c) % p
                        }
                    );
                    checked += 1;
                }
            }
        }
    }
    println!("REDUCED_RADIX_PAIRS={checked}");
}
