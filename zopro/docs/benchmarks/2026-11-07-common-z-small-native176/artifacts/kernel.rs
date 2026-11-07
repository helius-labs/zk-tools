use zopro_bn254::{ifma8::Ifma8, Field, Fq, FqConfig, G1Affine, G1Projective};
type V = Ifma8<FqConfig, true>;

#[inline(always)]
fn swap_half(a: &V) -> V {
    let (lo, hi) = a.halves();
    V::from_halves(&hi, &lo)
}

// D is broadcast. q_i is the product of the other seven lanes. Inputs in
// inactive lanes are one. Every multiplication is the accepted IFMA method.
#[inline(always)]
fn products(a: &V) -> (V, V) {
    let r1 = V::shuffle_pair::<1, 0, 3, 2>(a, a);
    let p2 = a.mul(&r1);
    let r2 = V::shuffle_pair::<2, 3, 0, 1>(&p2, &p2);
    let p4 = p2.mul(&r2);
    let r4 = swap_half(&p4);
    (p4.mul(&r4), r1.mul(&r2).mul(&r4))
}

#[inline(never)]
pub fn fast(input: &[G1Affine]) -> Option<G1Affine> {
    if !(2..=16).contains(&input.len()) || input.iter().any(G1Affine::is_identity) {
        return None;
    }
    let mut p = [(Fq::ZERO, Fq::ZERO); 16];
    for (out, a) in p.iter_mut().zip(input) {
        *out = (a.x, a.y);
    }
    let mut n = input.len();
    let mut z = V::one();
    let mut first = true;
    while n > 1 {
        let m = n / 2;
        if (0..m).any(|j| p[2 * j].0 == p[2 * j + 1].0) {
            return None;
        }
        let ax = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[2 * j].0
            } else {
                Fq::ZERO
            }
        }));
        let ay = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[2 * j].1
            } else {
                Fq::ZERO
            }
        }));
        let bx = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[2 * j + 1].0
            } else {
                Fq::ONE
            }
        }));
        let by = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[2 * j + 1].1
            } else {
                Fq::ZERO
            }
        }));
        let dx = bx.sub(&ax);
        let dy = by.sub(&ay);
        let carry = if n % 2 == 1 { Some(p[n - 1]) } else { None };
        let (x, y, d, s, t) = if m == 1 {
            let s = dx.mul(&dx);
            let b = ax.mul(&s);
            let c = bx.mul(&s);
            let x = dy.mul(&dy).sub(&b).sub(&c);
            let y = dy.mul(&b.sub(&x)).sub(&ay.mul(&c.sub(&b)));
            let t = carry.map(|_| s.mul(&dx));
            (x, y, dx, s, t)
        } else {
            let (d, q) = products(&dx);
            let s = d.mul(&d);
            let t = s.mul(&d);
            let u = dy.mul(&q);
            let x = u.mul(&u).sub(&ax.add(&bx).mul(&s));
            let y = u.mul(&ax.mul(&s).sub(&x)).sub(&ay.mul(&t));
            (x, y, d, s, Some(t))
        };
        z = if first { d } else { z.mul(&d) };
        first = false;
        let xs = x.get();
        let ys = y.get();
        for j in 0..m {
            p[j] = (xs[j], ys[j]);
        }
        if let Some((x, y)) = carry {
            // Product/square broadcasts are valid in lane0 for every level;
            // singleton-pair d need not be broadcast in inactive lanes.
            p[m] = (x * s.get()[0], y * t.unwrap().get()[0]);
        }
        n = m + usize::from(carry.is_some());
    }
    Some(G1Projective::new(p[0].0, p[0].1, z.get()[0]).to_affine())
}
pub fn sum(input: &[G1Affine]) -> (G1Affine, bool) {
    match fast(input) {
        Some(p) => (p, true),
        None => (
            input
                .iter()
                .fold(G1Projective::IDENTITY, |a, p| a + *p)
                .to_affine(),
            false,
        ),
    }
}
pub fn check_products(a: [Fq; 8]) {
    let (d, q) = products(&V::new(&a));
    let d = d.get();
    let q = q.get();
    let expected = a.iter().copied().fold(Fq::ONE, |a, b| a * b);
    assert_eq!(d, [expected; 8]);
    for j in 0..8 {
        let other = a
            .iter()
            .enumerate()
            .filter(|&(i, _)| i != j)
            .fold(Fq::ONE, |p, (_, a)| p * *a);
        assert_eq!(q[j], other);
    }
}
