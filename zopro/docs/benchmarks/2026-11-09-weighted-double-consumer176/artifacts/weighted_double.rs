//! Fixed-eight G1 weighted folds; original tree arithmetic and both outputs.
use super::*;
mod kernel;

#[cfg(test)]
static TILES: std::sync::atomic::AtomicUsize=std::sync::atomic::AtomicUsize::new(0);

// These logical fixed objects are charged in addition to the unchanged tree,
// Scratch and partial-output capacities. This is not an emitted stack bound.
pub(super) fn extra<C:CurveConfig<Base=Fq>>()->Option<usize> {
    core::mem::size_of::<[[Affine<C>;9];8]>()
        .checked_add(core::mem::size_of::<[Affine<C>;8]>().checked_mul(2)?)?
        .checked_add(core::mem::size_of::<kernel::Output<C>>())
}

fn roots<C:CurveConfig<Base=Fq>>(
    input:&[Affine<C>],tree:&mut Vec<LimbPoint<Fq>>,scratch:&mut Scratch<C>,
)->[Affine<C>;9] {
    assert!(!input.is_empty()&&input.len()<=256);
    tree.resize(511,LimbPoint::<Fq>::infinity());
    tree[input.len()..256].fill(LimbPoint::<Fq>::infinity());
    for (out,&p) in tree.iter_mut().zip(input) {*out=limbs::<C>(p);}
    let mut starts=[0usize;9];let mut counts=[0usize;9];counts[0]=256;
    let mut levels=1;let mut next=256;
    while counts[0]>1 {
        let half=counts[0]/2;
        for i in 0..half {tree[next+i]=tree[2*i];}
        reduce_layer(tree,&starts[..levels],&mut counts[..levels],scratch);
        starts[levels]=next;counts[levels]=half;levels+=1;next+=half;
    }
    assert_eq!(levels,9);
    core::array::from_fn(|i|affine::<C>(tree[if i==0{0}else{starts[9-i]}]))
}

fn partials<C:CurveConfig<Base=Fq>>(buckets:&[Affine<C>])->Vec<(Projective<C>,Affine<C>)> {
    let groups=buckets.len().div_ceil(256);
    let mut partials=vec![(Projective::<C>::IDENTITY,Affine::<C>::IDENTITY);groups];
    partials.par_chunks_mut(8).zip(buckets.par_chunks(8*256)).for_each_init(
        ||(Vec::<LimbPoint<Fq>>::new(),Scratch::<C>::default()),
        |(tree,scratch),(out,input)| {
            #[cfg(test)] TILES.fetch_add(1,std::sync::atomic::Ordering::Relaxed);
            let active=out.len();let mut values=[[Affine::<C>::IDENTITY;9];8];
            for (lane,group) in input.chunks(256).enumerate() {values[lane]=roots(group,tree,scratch);}
            let (weighted,_)=kernel::fold(&values,active);
            for i in 0..active {out[i]=(weighted[i].to_projective(),values[i][0]);}
        });
    partials
}

pub(super) fn weighted<C:CurveConfig<Base=Fq>>(buckets:&[Affine<C>])->Option<Projective<C>> {
    if buckets.len()<2048 || !crate::simd::cpu_option("ZOPRO_CPU_RESOURCE_SCHEDULE") {return None;}
    let partials=partials(buckets);
    let mut sum=Projective::<C>::IDENTITY;
    let mut offset=Projective::<C>::IDENTITY;
    let mut running=Projective::<C>::IDENTITY;
    for (weighted,total) in partials.into_iter().rev() {
        sum+=weighted;offset+=running;running+=total;
    }
    for _ in 0..256usize.ilog2() {offset=offset.double();}
    Some(sum+offset)
}

#[cfg(test)]
mod tests;
