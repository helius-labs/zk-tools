//! No-allocation eligibility check for the existing borrowed affine batch.
#[inline]
pub(super) fn eligible<T>(queue:&[(u32,T)],pending:usize)->bool{
    const MASK:u32=0x7fffffff;
    let lanes=crate::simd::BATCH_LANES;
    if queue.len()<lanes || (pending!=0&&pending<lanes){return false;}
    let end=queue.len();
    for i in 0..lanes{
        let bucket=queue[end-1-i].0&MASK;
        for j in 0..i{if queue[end-1-j].0&MASK==bucket{return false;}}
    }
    true
}
#[cfg(test)]mod tests{
    use super::*;
    #[test]fn simd_width_pending_and_signed_destination_guards(){
        let lanes=crate::simd::BATCH_LANES;
        let mut q=(0..lanes+1).map(|i|((i as u32)|if i%2==0{1<<31}else{0},())).collect::<Vec<_>>();
        assert!(!eligible(&q[..lanes-1],0));assert!(eligible(&q[..lanes],0));assert!(eligible(&q,lanes));assert!(!eligible(&q,lanes-1));
        let last=q.len()-1;q[last].0=q[last-1].0^(1<<31);assert!(!eligible(&q,0));
        q[last].0=1000;assert!(eligible(&q,0));
    }
}
