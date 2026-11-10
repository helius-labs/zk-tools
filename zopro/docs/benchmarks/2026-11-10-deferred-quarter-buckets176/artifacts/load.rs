#[inline(always)]
pub fn load_y(a:&[Stored;8])->Wide<2>{
    unsafe {
        let mut raw=Raw::raw(&a.map(|v|v.0));
        // Stored payloads are <2p<2^255. The format bit at255 becomes bit47
        // of the high radix52 limb. Clear it before any field operation.
        let tag=splat(1u64<<47);
        let original=_mm512_test_epi64_mask(raw.0[4],tag);
        raw.0[4]=_mm512_andnot_si512(tag,raw.0[4]);
        if original==0{return bounded(raw);}
        // Untagged lanes may be lazy >=p. The private quarter helper permits
        // <2p input, returns <2p, and these lanes are blended back unchanged.
        let converted=quarter_raw(raw);
        for j in 0..5{raw.0[j]=_mm512_mask_blend_epi64(original,raw.0[j],converted.0.0[j]);}
        bounded(raw)
    }
}
