// Destination-ordered accumulation: one live accumulator per bucket, one
// independent update per bucket in each round. No collision queue or tree copies.
fn stage<'a>(counts: &[usize; BAND], get: impl Fn(usize, usize) -> Leaf<'a>, state: &mut State) -> [usize; BAND] {
    state.current.clear();
    state.current.resize(BAND, G2Affine::IDENTITY);
    for b in 0..BAND {
        if counts[b] != 0 { state.current[b] = get(b, 0).value(); }
    }
    let depth = counts.iter().copied().max().unwrap_or(0);
    for round in 1..depth {
        let mut batch = state.pairs.take();
        batch.reserve_exact(BAND);
        let mut exceptional = 0;
        for b in 0..BAND {
            if round >= counts[b] { continue; }
            let right = get(b, round);
            let left = &mut state.current[b];
            if left.is_identity() { *left = right.value(); exceptional += 1; }
            else if right.point.is_identity() { exceptional += 1; }
            else if left.x == right.point.x { *left = (*left + right.value()).to_affine(); exceptional += 1; }
            else { batch.push((b as u32 | if right.negative {1 << 31} else {0}, right.point)); }
        }
        let metrics = execute(&mut state.current, &mut batch, &mut state.adder);
        #[cfg(test)] { state.stats.calls += usize::from(metrics.0 > 0); state.stats.real_pairs += metrics.0; state.stats.dummy_pairs += metrics.1; state.stats.exceptional += exceptional; }
        #[cfg(not(test))] let _ = (metrics, exceptional);
        state.pairs.restore(batch);
    }
    core::array::from_fn(|b| if counts[b] == 0 {usize::MAX} else {b})
}
