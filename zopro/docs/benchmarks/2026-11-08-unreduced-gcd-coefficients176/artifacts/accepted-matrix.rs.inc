#[inline(always)]
pub(crate) fn decode_row(e:u64)->(i64,i64) {
    let low=e as u32 as i32 as i64;
    let f=select_unpredictable(e==1u64<<31,1i64<<31,low);
    let g=(e.wrapping_sub(f as u64) as i64)>>32;
    (f,g)
}
#[inline(always)]
pub(crate) fn zero_run_matrix(mut a:u64,mut b:u64)->(u64,u64,i64,i64,i64,i64) {
    let (mut row0,mut row1)=(1u64,1u64<<32);
    let mut remaining=BITS as u32;
    let first=a.trailing_zeros().min(remaining);
    a >>= first; row1 <<= first; remaining-=first;
    while remaining!=0 {
        debug_assert_eq!(a&1,1);debug_assert_eq!(b&1,1);
        let smaller=a<b;
        let difference=a.wrapping_sub(b);
        let magnitude=select_unpredictable(smaller,difference.wrapping_neg(),difference);
        b=select_unpredictable(smaller,a,b);
        let previous_row0=row0;
        row0=select_unpredictable(smaller,row1,row0);
        row1=select_unpredictable(smaller,previous_row0,row1);
        row0=row0.wrapping_sub(row1);
        let zeros=magnitude.trailing_zeros().min(remaining);
        debug_assert!(zeros>0 && zeros<=31);
        a=magnitude>>zeros;row1 <<= zeros;remaining-=zeros;
    }
    let(f0,g0)=decode_row(row0);let(f1,g1)=decode_row(row1);
    (a,b,f0,g0,f1,g1)
}
