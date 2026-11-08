//! Proof-local bundles of unchanged G2 indexed bucket tiles.
use super::*;
use zopro_bn254::G2Affine;
#[cfg(test)]use zopro_bn254::G2Projective;
const TILE:usize=1024;

fn additional_charge(tasks:usize,workers:usize)->Option<usize>{
    // A job stores one shared task slice and one disjoint mutable output slice.
    // The original task/partial/output arrays retain their original charge.
    let jobs=tasks.checked_mul(core::mem::size_of::<(&[(usize,usize)],&mut[G2Affine])>())?;
    let local=TILE.checked_mul(2*core::mem::size_of::<usize>())?
        .checked_add(2*core::mem::size_of::<Vec<usize>>()+256)?;
    jobs.checked_add(core::mem::size_of::<Vec<usize>>()+256)?
        .checked_add(local.checked_mul(workers.max(1))?)
}

pub(super)fn try_reduce<C:CurveConfig>(bases:&[Affine<C>],order:&[u32],starts:&[usize])->Option<Vec<Affine<C>>>
where C::Base:BatchInverse {
    if core::mem::size_of::<C::Base>()!=64{return None;}
    let tasks=starts.windows(2).try_fold(0usize,|n,r|n.checked_add(r[1].checked_sub(r[0])?.div_ceil(TILE)))?;
    let charge=additional_charge(tasks,rayon::current_num_threads());
    admitted::<C>(bases,order,starts,charge)
}

fn admitted<C:CurveConfig>(bases:&[Affine<C>],order:&[u32],starts:&[usize],charge:Option<usize>)->Option<Vec<Affine<C>>>
where C::Base:BatchInverse {
    let charge=charge?;
    let _owner=crate::preparation::Reservation::analysis(Some(charge))?;
    let out=reduce::<C>(bases,order,starts);
    #[cfg(zopro_coalesce_probe)]crate::coalesce_query_probe::record(charge);
    Some(out)
}

fn reduce<C:CurveConfig>(bases:&[Affine<C>],order:&[u32],starts:&[usize])->Vec<Affine<C>>
where C::Base:BatchInverse {
    let mut tasks=Vec::new();let mut partial_starts=Vec::with_capacity(starts.len());partial_starts.push(0);
    for range in starts.windows(2){for first in (range[0]..range[1]).step_by(TILE){tasks.push((first,(first+TILE).min(range[1])));}partial_starts.push(tasks.len());}
    let mut partials=vec![Affine::<C>::IDENTITY;tasks.len()];
    {
        let mut jobs=Vec::with_capacity(tasks.len());let mut remaining=partials.as_mut_slice();let mut first=0;
        while first<tasks.len(){let mut end=first;let mut n=0;while end<tasks.len()&&n+tasks[end].1-tasks[end].0<=TILE{n+=tasks[end].1-tasks[end].0;end+=1;}debug_assert!(end>first);
            let (out,rest)=remaining.split_at_mut(end-first);remaining=rest;jobs.push((&tasks[first..end],out));first=end;
        }
        jobs.into_par_iter().for_each(|(tiles,out)|{
            let n=tiles.iter().map(|&(a,b)|b-a).sum::<usize>();debug_assert!(n<=TILE);
            let mut points=Vec::with_capacity(n);let mut offsets=Vec::with_capacity(tiles.len());let mut counts=Vec::with_capacity(tiles.len());
            for &(first,end)in tiles{offsets.push(points.len());counts.push(end-first);points.extend(order[first..end].iter().map(|&i|limbs::<C>(bases[i as usize])));}
            let mut scratch=Scratch::<C>::default();while counts.iter().any(|&n|n>1){reduce_layer(&mut points,&offsets,&mut counts,&mut scratch);}
            for(i,result)in out.iter_mut().enumerate(){*result=affine::<C>(points[offsets[i]]);}
        });
    }
    let counts=partial_starts.windows(2).map(|r|r[1]-r[0]).collect::<Vec<_>>();let partial_order=(0..partials.len()as u32).collect::<Vec<_>>();group_affine(&partials,&partial_order,&partial_starts,&counts)
}

#[cfg(test)]mod tests{
    use super::*;use zopro_bn254::{G2Config,Fr};
    #[test]fn ragged_complete_buckets_match_generator_and_original_under_small_stack_pools(){
        let sizes=[0usize,1,7,8,31,32,1023,1024,1025,2049,4099];let maximum=*sizes.last().unwrap();let g=G2Projective::generator();
        let coefficients=(0..maximum).map(|i|match i%11{0=>0i64,1|2=>1,3=>-1,4=>2,5=>-2,_=>(i%31+3)as i64}).collect::<Vec<_>>();
        let scalar=|n:i64|if n<0{-Fr::from(n.unsigned_abs())}else{Fr::from(n as u64)};
        let bases=G2Projective::batch_to_affine(&coefficients.iter().map(|&n|g*scalar(n)).collect::<Vec<_>>());
        for workers in[1,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();pool.install(||{
            for n in sizes{for mode in 0..3{let bins=match mode{0=>1,1=>128,_=>1031};let mut lists=vec![vec![];bins];for i in 0..n{let b=match mode{0=>0,1=>(i*37)%bins,_=>if i%3==0{bins-1}else{i%7}};lists[b].push(((i*37)%n)as u32);}
                let mut starts=vec![0];let mut order=vec![];for list in &lists{order.extend(list);starts.push(order.len());}let saved_bases=bases.clone();let saved_order=order.clone();let actual=reduce::<G2Config>(&bases,&order,&starts);let original=super::super::reduce_indexed_buckets::<G2Config>(&bases,&order,&starts);assert_eq!(actual,original,"n={n}mode={mode}workers={workers}");
                for(i,list)in lists.iter().enumerate(){let coefficient=list.iter().map(|&i|coefficients[i as usize]).sum::<i64>();assert_eq!(actual[i],(g*scalar(coefficient)).to_affine(),"n={n}bin={i}mode={mode}workers={workers}");}assert_eq!(bases,saved_bases);assert_eq!(order,saved_order);
            }}
        });}
    }
    #[test]fn denied_charge_and_overflow_leave_inputs_untouched(){let bases=[G2Affine::GENERATOR];let order=[0u32];let starts=[0usize,1];assert!(admitted::<G2Config>(&bases,&order,&starts,None).is_none());assert_eq!(bases,[G2Affine::GENERATOR]);assert_eq!(order,[0]);assert_eq!(starts,[0,1]);assert!(additional_charge(usize::MAX,32).is_none());assert!(additional_charge(1,usize::MAX).is_none());assert!(additional_charge(128,32).unwrap()<1024*1024);}
}
