//! Compute only forward FFT outputs whose four-row Euler product can be nonzero.
use super::*;

fn live(a: &[Fr], b: &[Fr], index: usize) -> bool {
    [a,b].into_iter().any(|input| {
        input.get(index..).is_some_and(|rest| rest.iter().take(4).any(|x| *x != Fr::ZERO))
    })
}

fn mask(a: &[Fr], b: &[Fr], width: usize, column: usize) -> u32 {
    (0..32).fold(0, |bits, row| {
        bits | (u32::from(live(a, b, 4*(row*width + column))) << row)
    })
}

fn needs(mut outputs: u32) -> [u32; 5] {
    // The last layer combines stripes sixteen apart. Each wanted butterfly
    // output needs both predecessor values, including an output needed alone.
    let mut layers = [0; 5];
    for (order, low) in [0x55555555,0x33333333,0x0f0f0f0f,0x00ff00ff,0x0000ffffu32].into_iter().enumerate().rev() {
        layers[order] = outputs;
        let gap = 1 << order;
        let pairs = (outputs | (outputs >> gap)) & low;
        outputs = pairs | (pairs << gap);
    }
    layers
}

#[inline(never)]
fn column<const CACHED: bool>(stripes: &mut [&mut [PackedPair];32], j: usize,
                              roots: &Twiddles, width: usize, start: usize, bits: u32) {
    if bits == 0 { return; }
    let mut local: [Pair;32] = core::array::from_fn(|row| stripes[row][j].load());
    let col = start+j;
    if bits == u32::MAX {
        column_kernel::<CACHED,true>(&mut local,roots,0,width,col);
    } else {
        for (order, wanted) in needs(bits).into_iter().enumerate() {
            let gap = 1 << order;
            for block in (0..32).step_by(2*gap) {
                for offset in 0..gap {
                    let lo = block + offset;
                    let hi = lo + gap;
                    let low = wanted & (1<<lo) != 0;
                    let high = wanted & (1<<hi) != 0;
                    if !low && !high { continue; }
                    let root = if CACHED {
                        &roots.columns[0][31*col+32-2*gap+offset]
                    } else {
                        &roots.layers[4-order][offset*width+col]
                    };
                    let a = local[lo];
                    let t = local[hi].mul(&broadcast(root));
                    if low { local[lo] = a.add(&t); }
                    if high { local[hi] = a.sub(&t); }
                }
            }
        }
    }
    // All slots already contain valid field values. A discarded pack has
    // zero A and B in every consumer lane, so its derivative is immaterial.
    let mut used = bits;
    while used != 0 {
        let row = used.trailing_zeros() as usize;
        stripes[row][j] = PackedPair::store(local[row]);
        used &= used-1;
    }
}

fn visit<const CACHED: bool>(mut stripes: [&mut [PackedPair];32], roots: &Twiddles,
                             width: usize, start: usize, a: &[Fr], b: &[Fr]) {
    let len = stripes[0].len();
    if len > 64 {
        let half = len/2;
        let left = core::array::from_fn(|i| {
            let (l,r) = core::mem::take(&mut stripes[i]).split_at_mut(half);
            stripes[i]=r;
            l
        });
        rayon::join(||visit::<CACHED>(left,roots,width,start,a,b),
                    ||visit::<CACHED>(stripes,roots,width,start+half,a,b));
        return;
    }
    for j in 0..len {
        column::<CACHED>(&mut stripes,j,roots,width,start,mask(a,b,width,start+j));
    }
}

pub(super) fn finish(values: &mut [PackedPair], roots: &Twiddles, a: &[Fr], b: &[Fr]) {
    let width=values.len()/32;
    let mut pieces=values.chunks_exact_mut(width);
    let stripes=core::array::from_fn(|_|pieces.next().unwrap());
    if roots.columns.is_empty() {visit::<false>(stripes,roots,width,0,a,b);}
    else {visit::<true>(stripes,roots,width,0,a,b);}
}

pub(super) fn report(a:&[Fr], b:&[Fr], width:usize) {
    let live_rows=(0..a.len().max(b.len())).filter(|&i|
        a.get(i).is_some_and(|v|*v!=Fr::ZERO) || b.get(i).is_some_and(|v|*v!=Fr::ZERO)).count();
    let mut none=0; let mut single=0;
    for col in 0..width {
        for (order, wanted) in needs(mask(a,b,width,col)).into_iter().enumerate() {
            let gap=1<<order;
            for block in (0..32).step_by(2*gap) {
                for j in 0..gap {
                    let low=(wanted>>(block+j))&1;
                    let high=(wanted>>(block+j+gap))&1;
                    none+=usize::from(low+high==0);
                    single+=usize::from(low+high==1);
                }
            }
        }
    }
    eprintln!("FFT_OUTPUT_SUPPORT {live_rows} {none} {single} {}",width*80);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn pruned_outputs_match_full_euler_products() {
        for n in [8192usize,32768] {
            let coset=Fr::from(5u64);
            let mut plan=Plan::new(n,Fr::root_of_unity(n).unwrap().pow(&[3]),coset,denominator(n,coset).unwrap());
            let packed: Vec<_>=(0..n/4).map(|i| PackedPair::store(Pair::new(
                &core::array::from_fn(|j|Fr::from((i*8+j+113) as u64).pow(&[9]))))).collect();
            for cached in [false,true] {
                if cached {plan.roots.prepare_columns();}
                let mut expected=packed.clone();columns::<true>(&mut expected,&plan.roots,0);
                for (count,period,b_len) in [(n,1,n),(n/2+1,9,n/3),(n-1,19,n-2),(1,1,0),(0,1,0),(n,usize::MAX,n)] {
                    let input=|len:usize,shift:usize| -> Vec<Fr> {
                        (0..len).map(|i|if period==usize::MAX || (i/7+shift)%period!=0 {Fr::ZERO}
                            else {Fr::from((i+1+shift) as u64)}).collect()
                    };
                    let mut a=input(count,0);let b=input(b_len,2);let mut ordinary=a.clone();
                    let mut pruned=packed.clone();finish(&mut pruned,&plan.roots,&a,&b);
                    for i in 0..n/4 {
                        if live(&a,&b,4*i) {assert_eq!(expected[i].load().get(),pruned[i].load().get());}
                    }
                    euler_combine_products(&mut ordinary,&b,&expected);
                    euler_combine_products(&mut a,&b,&pruned);assert_eq!(a,ordinary);
                }
            }
        }
    }
}
