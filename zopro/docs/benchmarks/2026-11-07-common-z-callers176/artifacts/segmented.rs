// Included inside the immutable parent kernel's module, reusing its V and
// qualified product butterfly. This is a new multi-output algorithm.
pub fn segments(input: &[(Fq, Fq)], counts: &[usize]) -> Option<[(Fq, Fq); 16]> {
    if input.len() > 16
        || counts.len() > 16
        || counts.iter().sum::<usize>() != input.len()
        || !counts.iter().any(|&n| n >= 3)
        || input.iter().any(|&(x, y)| x.is_zero() && y.is_zero())
    {
        return None;
    }
    let mut p = [(Fq::ZERO, Fq::ZERO); 16];
    p[..input.len()].copy_from_slice(input);
    let mut c = [0usize; 16];
    c[..counts.len()].copy_from_slice(counts);
    let mut z = V::one();
    let mut first = true;
    while c.iter().any(|&n| n > 1) {
        let mut left = [0usize; 8];
        let mut dest = [0usize; 8];
        let mut m = 0;
        let mut carried = [(Fq::ZERO, Fq::ZERO); 16];
        let mut carry_dest = [0usize; 16];
        let mut nc = 0;
        let mut src = 0;
        let mut dst = 0;
        for count in &mut c[..counts.len()] {
            for j in 0..*count / 2 {
                left[m] = src + 2 * j;
                dest[m] = dst + j;
                m += 1;
            }
            if *count % 2 != 0 {
                carried[nc] = p[src + *count - 1];
                carry_dest[nc] = dst + *count / 2;
                nc += 1;
            }
            src += *count;
            *count = count.div_ceil(2);
            dst += *count;
        }
        assert!((1..=8).contains(&m));
        if (0..m).any(|j| p[left[j]].0 == p[left[j] + 1].0) {
            return None;
        }
        let ax = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[left[j]].0
            } else {
                Fq::ZERO
            }
        }));
        let ay = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[left[j]].1
            } else {
                Fq::ZERO
            }
        }));
        let bx = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[left[j] + 1].0
            } else {
                Fq::ONE
            }
        }));
        let by = V::new(&std::array::from_fn(|j| {
            if j < m {
                p[left[j] + 1].1
            } else {
                Fq::ZERO
            }
        }));
        let dx = bx.sub(&ax);
        let dy = by.sub(&ay);
        let (x, y, d, s, t) = if m == 1 {
            let s = dx.mul(&dx);
            let b = ax.mul(&s);
            let v = bx.mul(&s);
            let x = dy.mul(&dy).sub(&b).sub(&v);
            let y = dy.mul(&b.sub(&x)).sub(&ay.mul(&v.sub(&b)));
            let t = (nc != 0).then(|| s.mul(&dx));
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
            p[dest[j]] = (xs[j], ys[j]);
        }
        if nc != 0 {
            let s = V::new(&[s.get()[0]; 8]);
            let t = V::new(&[t.unwrap().get()[0]; 8]);
            for offset in (0..nc).step_by(8) {
                let xs = V::new(&std::array::from_fn(|j| {
                    if offset + j < nc {
                        carried[offset + j].0
                    } else {
                        Fq::ZERO
                    }
                }))
                .mul(&s)
                .get();
                let ys = V::new(&std::array::from_fn(|j| {
                    if offset + j < nc {
                        carried[offset + j].1
                    } else {
                        Fq::ZERO
                    }
                }))
                .mul(&t)
                .get();
                for j in 0..8.min(nc - offset) {
                    p[carry_dest[offset + j]] = (xs[j], ys[j]);
                }
            }
        }
    }
    let zi = z.get()[0].inverse()?;
    let zi2 = zi.square();
    let zi3 = zi2 * zi;
    let xscale = V::new(&[zi2; 8]);
    let yscale = V::new(&[zi3; 8]);
    let mut positions = [0usize; 16];
    let mut at = 0;
    for j in 0..counts.len() {
        positions[j] = at;
        at += c[j];
    }
    let mut output = [(Fq::ZERO, Fq::ZERO); 16];
    for offset in (0..counts.len()).step_by(8) {
        let x = V::new(&std::array::from_fn(|j| {
            if offset + j < counts.len() && c[offset + j] != 0 {
                p[positions[offset + j]].0
            } else {
                Fq::ZERO
            }
        }))
        .mul(&xscale)
        .get();
        let y = V::new(&std::array::from_fn(|j| {
            if offset + j < counts.len() && c[offset + j] != 0 {
                p[positions[offset + j]].1
            } else {
                Fq::ZERO
            }
        }))
        .mul(&yscale)
        .get();
        for j in 0..8.min(counts.len() - offset) {
            output[offset + j] = (x[j], y[j]);
        }
    }
    Some(output)
}
