// Replaces append/execute/stage in the retained tree; Route/window are unchanged.
fn ensure(out: &mut Vec<G2Affine>, slots: usize) {
    if out.len() < slots {
        out.reserve_exact(slots-out.len());
        out.resize(slots, G2Affine::IDENTITY);
    }
}
fn append<'a>(
    out: &mut [G2Affine], batch: &mut Vec<(u32,&'a G2Affine,&'a G2Affine)>,
    index: usize, left: Leaf<'a>, right: Leaf<'a>,
) -> bool {
    if left.point.is_identity() { out[index]=right.value(); return true; }
    if right.point.is_identity() { out[index]=left.value(); return true; }
    if left.point.x == right.point.x {
        out[index]=(left.value()+right.value()).to_affine(); return true;
    }
    assert!(index < 0x4000_0000);
    let encoded=index as u32 | if left.negative {1<<31} else {0} | if right.negative {1<<30} else {0};
    batch.push((encoded,left.point,right.point));
    false
}
fn execute(out: &mut [G2Affine], batch: &mut Vec<(u32,&G2Affine,&G2Affine)>,
           adder: &mut BatchAdd<G2Config>, real_outputs: usize) -> (usize,usize) {
    let real_pairs=batch.len();
    if real_pairs==0 { return (0,0); }
    let (encoded,left,right)=batch[0];
    let padding=(8-real_pairs%8)%8;
    assert!(out.len()>=real_outputs+padding && real_outputs+padding<0x4000_0000);
    for i in 0..padding { batch.push(((encoded&0xc000_0000)|(real_outputs+i)as u32,left,right)); }
    adder.apply_signed_pair_g2(out,batch);
    (real_pairs,padding)
}
fn stage<'a>(counts: &[usize;BAND], get: impl Fn(usize,usize)->Leaf<'a>, state: &mut State) -> [usize;BAND] {
    let mut counts=*counts;
    let mut starts=[0usize;BAND];
    let mut batch=state.pairs.take();
    let slots=counts.iter().map(|n|n.div_ceil(2)).sum::<usize>();
    let pair_count=counts.iter().map(|n|n/2).sum::<usize>();
    ensure(&mut state.current,slots+7);
    batch.reserve_exact(pair_count.next_multiple_of(8));
    let mut exceptional=0;let mut cursor=0;
    for b in 0..BAND {
        starts[b]=cursor;
        for i in (0..counts[b]).step_by(2) {
            let left=get(b,i);
            if i+1<counts[b] {
                exceptional+=usize::from(append(&mut state.current,&mut batch,cursor,left,get(b,i+1)));
            } else { state.current[cursor]=left.value(); }
            cursor+=1;
        }
        counts[b]=counts[b].div_ceil(2);
    }
    assert_eq!(cursor,slots);
    let metrics=execute(&mut state.current,&mut batch,&mut state.adder,slots);
    #[cfg(test)] { state.stats.calls+=usize::from(metrics.0>0);state.stats.real_pairs+=metrics.0;state.stats.dummy_pairs+=metrics.1;state.stats.exceptional+=exceptional; }
    #[cfg(not(test))] let _=(metrics,exceptional);
    state.pairs.restore(batch);
    while counts.iter().any(|&n|n>1) {
        let slots=counts.iter().map(|n|n.div_ceil(2)).sum::<usize>();
        let pair_count=counts.iter().map(|n|n/2).sum::<usize>();
        ensure(&mut state.next,slots+7);
        let mut batch=state.pairs.take();
        batch.reserve_exact(pair_count.next_multiple_of(8));
        let mut next_starts=[0usize;BAND];
        let mut exceptional=0;let mut cursor=0;
        for b in 0..BAND {
            next_starts[b]=cursor;
            for i in (0..counts[b]).step_by(2) {
                let left=Leaf {point:&state.current[starts[b]+i],negative:false};
                if i+1<counts[b] {
                    let right=Leaf {point:&state.current[starts[b]+i+1],negative:false};
                    exceptional+=usize::from(append(&mut state.next,&mut batch,cursor,left,right));
                } else { state.next[cursor]=left.value(); }
                cursor+=1;
            }
            counts[b]=counts[b].div_ceil(2);
        }
        assert_eq!(cursor,slots);
        let metrics=execute(&mut state.next,&mut batch,&mut state.adder,slots);
        #[cfg(test)] { state.stats.calls+=usize::from(metrics.0>0);state.stats.real_pairs+=metrics.0;state.stats.dummy_pairs+=metrics.1;state.stats.exceptional+=exceptional; }
        #[cfg(not(test))] let _=(metrics,exceptional);
        state.pairs.restore(batch);
        core::mem::swap(&mut state.current,&mut state.next);
        starts=next_starts;
    }
    core::array::from_fn(|b|if counts[b]==0 {usize::MAX} else {starts[b]})
}
