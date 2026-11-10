/// Canonical incoming H words -> signed difference in persistent quarter-Y.
/// For b=4a+r, b/4 mod p = a+r*T, T=(p+1)/4. Form the difference
/// before propagating carries; this avoids normalizing the quarter separately.
#[inline(always)]
pub fn incoming_dy(b:&[Fq;8],ay:&Wide<2>,negative:u8)->Wide<3>{
    unsafe {
        const T:[u64;4]={let p=crate::portable::add4(FqConfig::P,[1,0,0,0]).0;
            [p[0]>>2|p[1]<<62,p[1]>>2|p[2]<<62,p[2]>>2|p[3]<<62,p[3]>>2]};
        let raw=Raw::raw(&b.map(|v|v.0));let k=_mm512_and_si512(raw.0[0],splat(3));
        let t=const{super::split(T)};let two=const{modulus(2)};let three=const{modulus(3)};
        let mut carry=splat(0);let mut out=[splat(0);5];
        for j in 0..5 {
            let shifted=if j<4{_mm512_or_si512(_mm512_srli_epi64::<2>(raw.0[j]),
                _mm512_and_si512(_mm512_slli_epi64::<50>(raw.0[j+1]),splat(MASK)))}
                else{_mm512_srli_epi64::<2>(raw.0[j])};
            // This digit can exceed radix52, but is <4*2^52. Signed carry
            // propagation below incorporates that excess without truncation.
            let v=_mm512_add_epi64(shifted,_mm512_mullo_epi64(k,splat(t[j])));
            let signed=_mm512_mask_sub_epi64(v,negative,splat(0),v);
            let p=_mm512_mask_blend_epi64(negative,splat(two[j]),splat(three[j]));
            let d=_mm512_add_epi64(_mm512_sub_epi64(_mm512_add_epi64(p,signed),ay.0.0[j]),carry);
            carry=_mm512_srai_epi64::<52>(d);out[j]=_mm512_and_si512(d,splat(MASK));
        }
        // Positive 2p+quarter(b)-ay is in (0,3p); negative
        // 3p-quarter(b)-ay is in (0,3p], inclusive only at both zeros.
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry,splat(0)),0);
        bounded(Ifma8(out,PhantomData))
    }
}
