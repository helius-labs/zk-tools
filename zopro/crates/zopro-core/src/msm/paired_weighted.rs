//! Pair two independent original256-bucket trees; share only affine batches.
use super::*;
const GROUP:usize=256;
const NODES:usize=2*(2*GROUP-1);
const SEGMENTS:usize=2*(GROUP.ilog2()as usize+1);

fn workspace<C:CurveConfig>(n:usize)->Option<usize>where C::Base:BatchInverse{
 use crate::msm_affine::workspace::vector as v;
 if n<2*GROUP||n%(2*GROUP)!=0{return None;}
 let jobs=n/(2*GROUP);let pairs=GROUP;let batches=pairs/LANES;
 let records=if C::Base::TREE_PREFIX{
  v::<Group<C::Base>>((32-1)/LANES,false)?.checked_add(v::<[FqBatch;2]>(batches,false)?)?
 }else{v::<Group<C::Base>>(batches,false)?};
 let per=v::<LimbPoint<C::Base>>(NODES,false)?
  .checked_add(v::<usize>(pairs,false)?)?
  .checked_add(records)?
  .checked_add(core::mem::size_of::<Vec<[FqBatch;2]>>())?;
 per.checked_mul(jobs)?.checked_add(v::<[(Projective<C>,Affine<C>);2]>(jobs,false)?)
}
pub(super)fn try_weighted<C:CurveConfig>(buckets:&[Affine<C>])->Option<Projective<C>>where C::Base:BatchInverse{
 let needed=workspace::<C>(buckets.len())?;
 if needed>workspace_weighted::<C>(buckets.len())?{return None;}
 Some(run(buckets))
}
fn scratch<C:CurveConfig>()->Scratch<C>where C::Base:BatchInverse{
 let mut s=Scratch::<C>::default();s.pairs.reserve_exact(GROUP);
 s.groups.reserve_exact(if C::Base::TREE_PREFIX{(32-1)/LANES}else{GROUP/LANES});
 if C::Base::TREE_PREFIX{s.prefix_records.reserve_exact(GROUP/LANES);}
 s
}
fn forest<C:CurveConfig>(buckets:&[Affine<C>],tree:&mut Vec<LimbPoint<C::Base>>,scratch:&mut Scratch<C>)->[(Projective<C>,Affine<C>);2]where C::Base:BatchInverse{
 assert_eq!(buckets.len(),2*GROUP);tree.resize(NODES,LimbPoint::<C::Base>::infinity());
 let bases=[0,2*GROUP-1];for g in 0..2{for i in 0..GROUP{tree[bases[g]+i]=limbs::<C>(buckets[g*GROUP+i]);}}
 let mut starts=[0usize;SEGMENTS];let mut counts=[0usize;SEGMENTS];let mut next=[GROUP,bases[1]+GROUP];
 starts[1]=bases[1];counts[0]=GROUP;counts[1]=GROUP;let mut levels=2;
 while counts[0]>1{
  let half=counts[0]/2;
  for g in 0..2{for i in 0..half{tree[next[g]+i]=tree[bases[g]+2*i];}}
  reduce_layer(tree,&starts[..levels],&mut counts[..levels],scratch);
  for g in 0..2{starts[levels+g]=next[g];counts[levels+g]=half;next[g]+=half;}
  levels+=2;
 }
 debug_assert_eq!(levels,SEGMENTS);debug_assert_eq!(next,[2*GROUP-1,NODES]);
 core::array::from_fn(|g|{let total=affine::<C>(tree[bases[g]]);let mut sum=total.to_projective();for j in(2+g..levels).step_by(2).rev(){sum=sum.double();sum-=affine::<C>(tree[starts[j]]);} (sum,total)})
}
fn run<C:CurveConfig>(buckets:&[Affine<C>])->Projective<C>where C::Base:BatchInverse{
 let partials:Vec<_>=buckets.par_chunks(2*GROUP).map_init(||(Vec::new(),scratch::<C>()),|(tree,s),b|forest::<C>(b,tree,s)).collect();
 let(mut sum,mut offset,mut running)=(Projective::<C>::IDENTITY,Projective::<C>::IDENTITY,Projective::<C>::IDENTITY);
 for pair in partials.into_iter().rev(){for(weighted,total)in pair.into_iter().rev(){sum+=weighted;offset+=running;running+=total;}}
 for _ in 0..GROUP.ilog2(){offset=offset.double();}sum+offset
}
#[cfg(test)]
#[path="paired_weighted_tests.rs"]
mod tests;
