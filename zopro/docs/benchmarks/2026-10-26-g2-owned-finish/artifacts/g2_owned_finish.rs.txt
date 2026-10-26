//! Finish full owned G2 batches by reusing consumed record/input storage.
use super::*;

/// The caller consumes `batch` immediately after this call. Indices are
/// distinct and x differences nonzero, as in the ordinary packed batch.
/// Each consumed input x becomes the corresponding original bucket x.
pub(super) fn apply<C: CurveConfig<Base=Fq2>, const FAST:bool>(
    buckets:&mut[Affine<C>], batch:&mut[(u32,Affine<C>)], s:&mut Fq2AddScratch,
) {
    assert!(!batch.is_empty() && batch.len()%LANES==0);
    if s.prefix_trace && !s.prefix_traced {
        eprintln!("G2_OWNED_FINISH batch={}",batch.len());s.prefix_traced=true;
    }
    let full=batch.len();
    let mut chains=FqBatch::one();
    s.prefix_records.clear();s.prefix_records.reserve(full/LANES);
    for k in (0..full).step_by(LANES) {
        let ax=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].x));
        let bx=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let norm=Fq2::lanes_norm(&denominator);
        s.prefix_records.push([chains,norm]);chains=chains.mul(&norm);
    }
    s.tail.clear();
    let mut inverse=if let Some(value)=direct_chain_inverse(&chains,true,s.vector_inverse) {
        value
    } else {
        FqBatch::new(&invert_lanes(chains.get(),s.vector_inverse))
    };
    for j in (0..full/LANES).rev() {
        let k=j*LANES;
        let ax=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].x));
        let ay=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].y));
        let bx=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let by=fq2_pack(core::array::from_fn(|i|batch[k+i].1.y));
        let denominator=core::array::from_fn(|i|canonical_sub::<FAST>(&bx[i],&ax[i]));
        let dy=core::array::from_fn(|i|canonical_sub::<FAST>(&by[i],&ay[i]));
        let factor=s.prefix_records[j][0].mul(&inverse);
        inverse=inverse.mul(&s.prefix_records[j][1]);
        let slope=Fq2::lanes_scaled_numerator(&dy,&denominator,&factor);
        let x=Fq2::lanes_sub(&Fq2::lanes_square(&slope),&Fq2::lanes_add(&ax,&bx));
        let x=fq2_unpack(x);
        s.prefix_records[j]=slope;
        for i in 0..LANES {
            let index=batch[k+i].0 as usize;
            batch[k+i].1.x=buckets[index].x;
            buckets[index].x=x[i];
        }
    }
    for j in 0..full/LANES {
        let k=j*LANES;
        let old_x=fq2_pack(core::array::from_fn(|i|batch[k+i].1.x));
        let new_x=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].x));
        let delta=Fq2::lanes_sub(&old_x,&new_x);
        let product=Fq2::lanes_mul(&s.prefix_records[j],&delta);
        let old_y=fq2_pack(core::array::from_fn(|i|buckets[batch[k+i].0 as usize].y));
        let y=fq2_unpack(Fq2::lanes_sub(&product,&old_y));
        for i in 0..LANES {buckets[batch[k+i].0 as usize].y=y[i];}
    }
}

#[cfg(test)]
mod tests;
