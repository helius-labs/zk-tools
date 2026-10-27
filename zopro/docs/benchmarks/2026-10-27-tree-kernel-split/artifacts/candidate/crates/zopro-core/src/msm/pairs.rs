use super::*;

/// Apply ordinary disjoint pairs; selection and exceptional compaction stay in the caller.
pub(super) fn apply<C: CurveConfig, const PREFIX: bool>(
    points: &mut [LimbPoint<C::Base>],
    s: &mut Scratch<C>,
) where C::Base: BatchInverse,
{
    type F<C> = <C as CurveConfig>::Base;
    let m = s.pairs.len();
    let full = m / LANES * LANES;
    s.groups.clear();
    if !PREFIX {s.groups.reserve_exact(full / LANES);}

    let mut chains = FqBatch::one();
    if PREFIX {
        s.prefix_records.clear();
        s.prefix_records.reserve(full / LANES);
        for k in (0..full).step_by(LANES) {
            let a = core::array::from_fn(|i| &points[s.pairs[k + i]].x);
            let b = core::array::from_fn(|i| &points[s.pairs[k + i] + 1].x);
            let dx = F::<C>::lanes_sub(&F::<C>::load(b), &F::<C>::load(a));
            let norm = F::<C>::lanes_norm(&dx);
            s.prefix_records.push([chains, norm]);
            chains = chains.mul(&norm);
        }
    } else {
    for k in (0..full).step_by(LANES) {
        let a: [&LimbPoint<F<C>>; LANES] = core::array::from_fn(|i| &points[s.pairs[k + i]]);
        let b: [&LimbPoint<F<C>>; LANES] = core::array::from_fn(|i| &points[s.pairs[k + i] + 1]);
        let (ax, ay) = (F::<C>::load(a.map(|p| &p.x)), F::<C>::load(a.map(|p| &p.y)));
        let (bx, by) = (F::<C>::load(b.map(|p| &p.x)), F::<C>::load(b.map(|p| &p.y)));
        let denominator = F::<C>::lanes_sub(&bx, &ax);
        let norm = F::<C>::lanes_norm(&denominator);
        let numerator =
            F::<C>::lanes_scaled_numerator(&F::<C>::lanes_sub(&by, &ay), &denominator, &chains);
        chains = chains.mul(&norm);
        s.groups.push(Group {
            ax,
            ay,
            xsum: F::<C>::lanes_add(&ax, &bx),
            norm,
            numerator,
        });
    }

    }
    let mut inverses = if let Some(value)=direct_chain_inverse(&chains,full==m,s.vector_inverse) {
        value
    } else {
        let mut tail = chains.get();
        for k in full..m {
            let (a, b) = (
                affine::<C>(points[s.pairs[k]]),
                affine::<C>(points[s.pairs[k] + 1]),
            );
            let denominator = b.x - a.x;
            {
                s.denominators[k - full] = denominator;
                s.products[k - full] = F::<C>::prefix(&mut tail[k % LANES], denominator);
            }
        }
        let mut inverses = invert_lanes(tail, s.vector_inverse);
        for k in (full..m).rev() {
            let t = k - full;
            F::<C>::reverse(
                &mut inverses[k % LANES],
                s.denominators[t],
                &mut s.products[t],
            );
        }
        FqBatch::new(&inverses)
    };
    if PREFIX {
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
    for (j, group) in s.groups.iter().enumerate().rev() {
        let slope = F::<C>::lanes_scale(&group.numerator, &inverses);
        inverses = inverses.mul(&group.norm);
        let x = F::<C>::lanes_sub(&F::<C>::lanes_square(&slope), &group.xsum);
        let y = F::<C>::lanes_sub(
            &F::<C>::lanes_mul(&slope, &F::<C>::lanes_sub(&group.ax, &x)),
            &group.ay,
        );
        let (x, y) = (F::<C>::store(&x), F::<C>::store(&y));
        for i in 0..LANES {
            {
                points[s.pairs[j * LANES + i]] = LimbPoint { x: x[i], y: y[i] };
            }
        }
    }

    }
    for k in full..m {
        let t = k - full;
        let (a, b) = (
            affine::<C>(points[s.pairs[k]]),
            affine::<C>(points[s.pairs[k] + 1]),
        );
        let slope = (b.y - a.y) * F::<C>::denominator_inverse(s.denominators[t], s.products[t]);
        let x = slope.square() - a.x - b.x;
        let y = slope * (a.x - x) - a.y;
        {
            points[s.pairs[k]] = limbs::<C>(Affine::new(x, y));
        }
    }
}
