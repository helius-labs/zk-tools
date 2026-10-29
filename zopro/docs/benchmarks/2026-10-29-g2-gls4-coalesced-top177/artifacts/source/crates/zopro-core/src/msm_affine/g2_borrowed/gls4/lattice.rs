//! Original local GLS4 lattice and digit bodies; GnarkCrypto v0.21.0 attribution.
//! Sources and exact bound are retained in historical-integer-oracle.json.
use super::digit;
const U:i128=4965661367192848881;
const BASIS:[[i128;4];4]=[[2*U,U,U+1,-U],[-U,U,U,2*U+1],[-U-1,U,-U,-2*U],[2*U+1,U+1,-U,U]];
const RECIP:[[u64;4];4]=[
    [0x46f4bda995d51bb1, 0x8e5da66fc7184ae, 0x9e80318ab0d92b93, 0x0],
    [0x2dff291532e42728, 0x55b4ca7ba3e5577f, 0x9e80318ab0d92b95, 0x0],
    [0x71c4c43fac4db00, 0x55b4ca7ba3e55782, 0x9e80318ab0d92b95, 0x0],
    [0xc170977dcef3cd3f, 0x55b4ca7ba3e5577d, 0x9e80318ab0d92b95, 0x0]
];
const NEGATIVE:[bool;4]=[false,true,true,false];
#[inline(always)]
fn quotient_low(s:[u64;4],c:[u64;4],negative:bool)->u128 {
    let mut p=[0u64;8];
    for i in 0..4 {
        let mut carry=0u128;
        for j in 0..4 {
            let t=s[i]as u128*c[j]as u128+p[i+j]as u128+carry;
            p[i+j]=t as u64;carry=t>>64;
        }
        p[i+4]=carry as u64;
    }
    let q=p[4]as u128|((p[5]as u128)<<64);
    if negative {0u128.wrapping_sub(q).wrapping_sub(u128::from(p[..4].iter().any(|&v|v!=0)))}else{q}
}
#[inline]
pub(super) fn split(s:[u64;4])->[u64;5] {
    let q: [u128;4]=core::array::from_fn(|i|quotient_low(s,RECIP[i],NEGATIVE[i]));
    let low=s[0]as u128|((s[1]as u128)<<64);
    let mut result=[0u64;5];
    for j in 0..4 {
        let mut k=if j==0 {low}else{0};
        for i in 0..4 {k=k.wrapping_sub(q[i].wrapping_mul(BASIS[i][j]as u128));}
        let signed=k as i128;let magnitude=signed.unsigned_abs();
        // The exact lattice residual is below 2^65, so its low 128 bits
        // uniquely identify the signed value; no full-width products needed.
        assert!(magnitude<1u128<<65);
        result[j]=magnitude as u64;
        result[4]|=((magnitude>>64)as u64)<<j|u64::from(signed<0)<<(j+4);
    }
    result
}
#[inline]
pub(super) fn component_digit(s:&[u64;5],component:usize,w:usize,bits:usize)->i16 {
    let d=digit(&[s[component],(s[4]>>component)&1,0,0],w,bits);
    if (s[4]>>(component+4))&1!=0 {-d}else{d}
}
