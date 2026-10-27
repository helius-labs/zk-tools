    if F::<C>::TREE_PREFIX {
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
OLD_FORWARD
    }
