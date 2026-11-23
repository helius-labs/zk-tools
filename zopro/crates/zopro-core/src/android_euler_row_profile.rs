use zopro_bn254::{Field, Fr};

pub(super) fn trace(a: &[Fr], b: &[Fr]) {
    assert_eq!(a.len(), b.len());
    let n = a.len().next_power_of_two();
    let mut row_counts = [0usize; 7];
    let mut support = [(); 4].map(|_| Vec::with_capacity(n));
    for (&left, &right) in a.iter().zip(b) {
        let l = !left.is_zero();
        let r = !right.is_zero();
        let plus = left != -right;
        let minus = left != right;
        row_counts[0] += usize::from(l);
        row_counts[1] += usize::from(r);
        row_counts[2] += usize::from(plus);
        row_counts[3] += usize::from(minus);
        row_counts[4] += usize::from(left == Fr::ONE || left == -Fr::ONE);
        row_counts[5] += usize::from(right == Fr::ONE || right == -Fr::ONE);
        row_counts[6] += usize::from(l && r && !minus);
        for (mask, present) in support.iter_mut().zip([l, r, plus, minus]) {
            mask.push(present);
        }
    }
    for mask in &mut support {
        mask.resize(n, false);
    }
    let stages = support.each_mut().map(|mask| {
        let levels = n.trailing_zeros();
        (0..levels)
            .map(|stage| {
                let gap = n >> (stage + 1);
                let mut count = [0usize; 3];
                for block in mask.chunks_exact_mut(2 * gap) {
                    let (left, right) = block.split_at_mut(gap);
                    for (l, r) in left.iter_mut().zip(right) {
                        let occupied = usize::from(*l) + usize::from(*r);
                        count[occupied] += 1;
                        *l = occupied > 0;
                        *r = *l;
                    }
                }
                count
            })
            .collect::<Vec<_>>()
    });
    log::info!(
        "ZOPRO_TRACE euler-row-support n={n} active={} a={} b={} sum={} difference={} a_unit={} b_unit={} equal_nonzero={}",
        a.len(), row_counts[0], row_counts[1], row_counts[2], row_counts[3],
        row_counts[4], row_counts[5], row_counts[6],
    );
    for (name, counts) in ["a", "b", "sum", "difference"].into_iter().zip(stages) {
        log::info!("ZOPRO_TRACE euler-butterfly-support {name} {counts:?}");
    }
}
