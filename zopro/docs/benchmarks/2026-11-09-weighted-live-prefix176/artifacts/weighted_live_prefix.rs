//! Build only initialized tree nodes; all original reduction arithmetic remains.
use super::*;
pub(super) fn sum<C: CurveConfig>(
    points: &mut Vec<LimbPoint<C::Base>>,
    n: usize,
    scratch: &mut Scratch<C>,
) -> Projective<C>
where
    C::Base: BatchInverse,
{
    debug_assert!(n.is_power_of_two());
    debug_assert_eq!(points.len(), n);
    {
        // A power-of-two usize has at most usize::BITS reduction levels.
        let mut starts = [0usize; usize::BITS as usize];
        let mut counts = [0usize; usize::BITS as usize];
        counts[0] = n;
        let mut levels = 1;
        let mut next = n;
        while counts[0] > 1 {
            let half = counts[0] / 2;
            for i in 0..half {
                let point = points[2 * i];
                points.push(point);
            }
            reduce_layer(points, &starts[..levels], &mut counts[..levels], scratch);
            starts[levels] = next;
            counts[levels] = half;
            levels += 1;
            next += half;
        }
        let mut sum = affine::<C>(points[0]).to_projective();
        for start in starts[1..levels].iter().rev() {
            sum = sum.double();
            sum -= affine::<C>(points[*start]);
        }
        return sum;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use zopro_bn254::{Field,Fr,G1Config,G2Config};
    use std::sync::{Arc,atomic::{AtomicBool,Ordering}};
    fn fixture<C:CurveConfig>(n:usize,kind:usize)->(Vec<Affine<C>>,Vec<Fr>) where C::Base:BatchInverse {
        let start=Fr::from_le_bytes_mod_order(&[0x9bu8;32]);let mut p=Projective::<C>::generator()*start;
        let mut points=Vec::with_capacity(n);let mut coeffs=Vec::with_capacity(n);
        for i in 0..n {let mut c=start+Fr::from((i+1)as u64);p+=Projective::<C>::generator();let mut q=p;
            match kind {1=>match i%7{0=>{q=Projective::IDENTITY;c=Fr::ZERO;},1|2=>{q=Projective::<C>::generator();c=Fr::ONE;},3=>{q=-Projective::<C>::generator();c=-Fr::ONE;},_=>{}},2=>{c=if i%2==0{Fr::ONE}else{-Fr::ONE};q=Projective::<C>::generator()*c;},3=>{q=Projective::IDENTITY;c=Fr::ZERO;},_=>{}}
            points.push(q);coeffs.push(c);
        }
        (Projective::<C>::batch_to_affine(&points),coeffs)
    }
    fn check<C:CurveConfig>(n:usize,kind:usize,vector:bool,tree:&mut Vec<LimbPoint<C::Base>>)->u64 where C::Base:BatchInverse {
        let (points,coeffs)=fixture::<C>(n,kind);let immutable=points.clone();let scalar=coeffs.iter().enumerate().fold(Fr::ZERO,|s,(i,c)|s+*c*Fr::from((i+1)as u64));let expected=Projective::<C>::generator()*scalar;
        let mut original=vec![LimbPoint::infinity();2*n-1];for(a,&b)in original.iter_mut().zip(&points){*a=limbs::<C>(b);}
        let mut old=Scratch::<C>::default();old.vector_inverse=vector;let want=weighted_sum::<C>(&mut original,n,&mut old);
        tree.clear();tree.reserve(2*n-1);tree.extend(points.iter().copied().map(limbs::<C>));assert_eq!(tree.len(),n);let capacity=tree.capacity();let mut scratch=Scratch::<C>::default();scratch.vector_inverse=vector;
        let got=sum::<C>(tree,n,&mut scratch);assert_eq!(tree.len(),2*n-1);assert_eq!(tree.capacity(),capacity);assert_eq!(points,immutable);assert_eq!(got.to_affine(),expected.to_affine());assert_eq!(want.to_affine(),expected.to_affine());
        for(a,b)in tree.iter().zip(&original){assert!(a.x==b.x&&a.y==b.y,"final initialized limb tree differs n={n}kind={kind}vector={vector}");}
        let complete=weighted_affine::<C>(&points);assert_eq!(complete.to_affine(),expected.to_affine());(2*n-1)as u64
    }
    #[test]
    fn live_prefix_matches_original_and_generator_across_levels_and_reuse(){
        let mut g1=Vec::with_capacity(2047);let mut g2=Vec::with_capacity(2047);let mut words=0;
        for n in [1usize,2,4,8,16,32,64,128,256,512,1024,256,16,1]{for kind in 0..4{for vector in [false,true]{words+=check::<G1Config>(n,kind,vector,&mut g1);words+=check::<G2Config>(n,kind,vector,&mut g2);}}}
        println!("LIVE_PREFIX cases=224 initialized_limb_points_checked={words} all_generator_and_original_outputs=true same_capacity=true immutable_inputs=true");
    }
    #[test]
    fn actual_small_stack_workers_nested_live_prefix_calls(){
        let mut total=0;for workers in[1usize,4,16,32]{let pool=rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();let seen=Arc::new((0..workers).map(|_|AtomicBool::new(false)).collect::<Vec<_>>());
            pool.broadcast(|_|{let i=rayon::current_thread_index().unwrap();assert!(!seen[i].swap(true,Ordering::Relaxed));rayon::join(||{let mut v=Vec::with_capacity(127);check::<G1Config>(64,1,true,&mut v);check::<G1Config>(32,2,false,&mut v);},||{let mut v=Vec::with_capacity(127);check::<G2Config>(64,1,true,&mut v);check::<G2Config>(32,2,false,&mut v);});});
            assert!(seen.iter().all(|v|v.load(Ordering::Relaxed)));total+=workers;println!("LIVE_PREFIX workers={workers} all_actual_indices=true nested_joins=true requested_stack_bytes=524288");}
        assert_eq!(total,53);println!("LIVE_PREFIX actual_workers=53 nested_curve_calls=212");
    }
}
