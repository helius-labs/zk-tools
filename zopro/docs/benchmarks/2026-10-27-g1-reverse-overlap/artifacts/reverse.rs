    let mut remaining = s.groups.len();
    while remaining >= 2 {
        let j0 = remaining - 1;
        let j1 = remaining - 2;
        let (g0, g1) = (&s.groups[j0], &s.groups[j1]);
        let slope0 = g0.numerator.mul(&inverse);
        let next_inverse = inverse.mul(&g0.denominator);
        let slope1 = g1.numerator.mul(&next_inverse);
        inverse = next_inverse.mul(&g1.denominator);
        let x0 = slope0.mul(&slope0).sub(&g0.xsum);
        let y0 = slope0.mul(&g0.ax.sub(&x0)).sub(&g0.ay);
        let (x0, y0) = (x0.get(), y0.get());
        for i in 0..LANES {
            buckets[batch_index::<SIGNED>(batch[j0 * LANES + i].0)] = Affine::new(x0[i], y0[i]);
        }
        let x1 = slope1.mul(&slope1).sub(&g1.xsum);
        let y1 = slope1.mul(&g1.ax.sub(&x1)).sub(&g1.ay);
        let (x1, y1) = (x1.get(), y1.get());
        for i in 0..LANES {
            buckets[batch_index::<SIGNED>(batch[j1 * LANES + i].0)] = Affine::new(x1[i], y1[i]);
        }
        remaining -= 2;
    }
    for (j, g) in s.groups[..remaining].iter().enumerate().rev() {
        let slope = g.numerator.mul(&inverse);
        inverse = inverse.mul(&g.denominator);
        let x = slope.mul(&slope).sub(&g.xsum);
        let y = slope.mul(&g.ax.sub(&x)).sub(&g.ay);
        let (x, y) = (x.get(), y.get());
        for i in 0..LANES {
            buckets[batch_index::<SIGNED>(batch[j * LANES + i].0)] = Affine::new(x[i], y[i]);
        }
    }
