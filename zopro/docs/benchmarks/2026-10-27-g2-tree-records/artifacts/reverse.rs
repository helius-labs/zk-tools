    if F::<C>::TREE_PREFIX {
        for (j, record) in s.prefix_records.iter().enumerate().rev() {
            let k = j * LANES;
            let a: [&LimbPoint<F<C>>; LANES] = core::array::from_fn(|i| &points[s.pairs[k + i]]);
            let b: [&LimbPoint<F<C>>; LANES] = core::array::from_fn(|i| &points[s.pairs[k + i] + 1]);
            let (ax, ay) = (F::<C>::load(a.map(|p| &p.x)), F::<C>::load(a.map(|p| &p.y)));
            let (bx, by) = (F::<C>::load(b.map(|p| &p.x)), F::<C>::load(b.map(|p| &p.y)));
            let denominator = F::<C>::lanes_sub(&bx, &ax);
            let factor = record[0].mul(&inverses);
            inverses = inverses.mul(&record[1]);
            let slope = F::<C>::lanes_scaled_numerator(&F::<C>::lanes_sub(&by, &ay), &denominator, &factor);
            let x = F::<C>::lanes_sub(&F::<C>::lanes_square(&slope), &F::<C>::lanes_add(&ax, &bx));
            let y = F::<C>::lanes_sub(&F::<C>::lanes_mul(&slope, &F::<C>::lanes_sub(&ax, &x)), &ay);
            let (x, y) = (F::<C>::store(&x), F::<C>::store(&y));
            for i in 0..LANES { points[s.pairs[k + i]] = LimbPoint {x:x[i], y:y[i]}; }
        }
    } else {
OLD_REVERSE
    }
