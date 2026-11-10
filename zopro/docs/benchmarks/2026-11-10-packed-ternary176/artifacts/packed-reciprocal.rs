//! One-time balanced-radix-6561 recoding in 32 existing bytes plus one sidecar byte.
//! Inputs are two signed-magnitude Phi components, each strictly below 2^126.
pub const RADIX: u128 = 6561;
const MASK: u128 = (1 << 13) - 1;
#[inline(always)]
fn high_product(a:u128,b:u128)->u128{
    let (a0,a1)=(a as u64 as u128,a>>64);
    let (b0,b1)=(b as u64 as u128,b>>64);
    let (p00,p01,p10,p11)=(a0*b0,a0*b1,a1*b0,a1*b1);
    let carry=(p00>>64)+(p01 as u64 as u128)+(p10 as u64 as u128);
    p11+(p01>>64)+(p10>>64)+(carry>>64)
}
#[inline(always)]
fn quotient<const D:u128>(n:u128)->u128{
    debug_assert!(D>0&&D<1u128<<127);
    debug_assert!(n<1u128<<127);
    if D==1{return n}
    let reciprocal=const{if D<=1{0}else{u128::MAX/D+1}};
    let approximate=high_product(n,reciprocal);
    let product=approximate*D;
    approximate-u128::from(product>n)
}


#[inline]
fn component(words: &mut [u64]) -> u8 {
    let sign = if words[1] >> 63 == 0 { 1i128 } else { -1 };
    let mut remaining = words[0] as u128 | ((words[1] & 0x7fff_ffff_ffff_ffff) as u128) << 64;
    debug_assert!(remaining < 1 << 126);
    let mut packed = 0u128;
    let mut overflow = 0u8;
    for window in 0..10 {
        let quotient = quotient::<RADIX>(remaining + RADIX / 2);
        let digit = (remaining as i128 - quotient as i128 * RADIX as i128) * sign;
        debug_assert!(digit.unsigned_abs() <= RADIX / 2);
        let bits = (digit as u128) & MASK;
        packed |= bits << (13 * window);
        if window == 9 { overflow = (bits >> 11) as u8; }
        remaining = quotient;
    }
    debug_assert!(remaining <= 1);
    let carry = (remaining as i128 * sign) as u8 & 3;
    words[0] = packed as u64;
    words[1] = (packed >> 64) as u64;
    overflow | (carry << 2)
}

/// Overwrites the exclusive original word buffer; returns both upper nibbles.
/// Allocate/admit the complete sidecar before calling this on any input row.
#[inline]
pub fn recode_row(words: &mut [u64; 4]) -> u8 {
    let low = component(&mut words[..2]);
    let high = component(&mut words[2..]);
    low | (high << 4)
}

#[inline]
pub fn digit(words: &[u64; 4], sidecar: u8, high: bool, window: usize) -> i16 {
    assert!(window <= 10, "ternary window exceeds the supported component length");
    let nibble = (sidecar >> (4 * usize::from(high))) & 15;
    if window == 10 { return ((nibble as i8) << 4 >> 6) as i16; }
    let offset = 2 * usize::from(high);
    let packed = words[offset] as u128 | (words[offset + 1] as u128) << 64;
    let mut bits = (packed >> (13 * window)) & MASK;
    if window == 9 { bits |= ((nibble & 3) as u128) << 11; }
    ((bits as i16) << 3) >> 3
}
