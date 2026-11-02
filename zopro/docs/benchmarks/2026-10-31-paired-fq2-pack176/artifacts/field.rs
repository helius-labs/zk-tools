//! Load both components of eight existing extension-field coordinates together.
use crate::{ifma8::Ifma8, Fq2, FqConfig};
use core::{arch::x86_64::*, marker::PhantomData};

impl Ifma8<FqConfig, true> {
    #[inline(always)]
    pub fn from_fq2_refs(values: [&Fq2; 8]) -> [Self; 2] {
        // Fq is transparent over [u64;4]; repr(C) Fq2 has two adjacent
        // initialized Fq values, with no padding. References permit aliasing.
        const { assert!(size_of::<Fq2>() == 64); }
        const { assert!(align_of::<Fq2>() == align_of::<u64>()); }
        // SAFETY: each reference covers all 64 initialized bytes. These loads
        // permit arbitrary field alignment. The module's feature guard covers
        // all intrinsics; all shuffle selectors address supplied vector lanes.
        unsafe {
            let r: [__m512i; 8] = values.map(|p| _mm512_loadu_si512((p as *const Fq2).cast()));
            let a = [
                _mm512_unpacklo_epi64(r[0], r[1]), _mm512_unpackhi_epi64(r[0], r[1]),
                _mm512_unpacklo_epi64(r[2], r[3]), _mm512_unpackhi_epi64(r[2], r[3]),
                _mm512_unpacklo_epi64(r[4], r[5]), _mm512_unpackhi_epi64(r[4], r[5]),
                _mm512_unpacklo_epi64(r[6], r[7]), _mm512_unpackhi_epi64(r[6], r[7]),
            ];
            let b = [
                _mm512_shuffle_i64x2::<0x88>(a[0], a[2]), _mm512_shuffle_i64x2::<0x88>(a[1], a[3]),
                _mm512_shuffle_i64x2::<0xdd>(a[0], a[2]), _mm512_shuffle_i64x2::<0xdd>(a[1], a[3]),
                _mm512_shuffle_i64x2::<0x88>(a[4], a[6]), _mm512_shuffle_i64x2::<0x88>(a[5], a[7]),
                _mm512_shuffle_i64x2::<0xdd>(a[4], a[6]), _mm512_shuffle_i64x2::<0xdd>(a[5], a[7]),
            ];
            let words = [
                _mm512_shuffle_i64x2::<0x88>(b[0], b[4]), _mm512_shuffle_i64x2::<0x88>(b[1], b[5]),
                _mm512_shuffle_i64x2::<0x88>(b[2], b[6]), _mm512_shuffle_i64x2::<0x88>(b[3], b[7]),
                _mm512_shuffle_i64x2::<0xdd>(b[0], b[4]), _mm512_shuffle_i64x2::<0xdd>(b[1], b[5]),
                _mm512_shuffle_i64x2::<0xdd>(b[2], b[6]), _mm512_shuffle_i64x2::<0xdd>(b[3], b[7]),
            ];
            let mask = _mm512_set1_epi64(((1u64 << 52) - 1) as i64);
            core::array::from_fn(|component| {
                let [w0,w1,w2,w3]: [__m512i;4] = core::array::from_fn(|i| words[component*4+i]);
                Self([
                    _mm512_and_si512(w0,mask),
                    _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<52>(w0),_mm512_slli_epi64::<12>(w1)),mask),
                    _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<40>(w1),_mm512_slli_epi64::<24>(w2)),mask),
                    _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<28>(w2),_mm512_slli_epi64::<36>(w3)),mask),
                    _mm512_srli_epi64::<16>(w3),
                ],PhantomData)
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{portable,Fq,FpConfig};
    #[repr(C)]
    struct Record { tag: u64, coordinate: Fq2, other: Fq2 }
    #[test]
    fn paired_pack_raw_words_alignment_aliases_and_products() {
        let p=FqConfig::P;
        let two_p=portable::add4(p,p).0;
        let boundaries=[[0;4],[1,0,0,0],portable::sub4(p,[1,0,0,0]).0,p,
            portable::add4(p,[1,0,0,0]).0,portable::sub4(two_p,[1,0,0,0]).0,two_p];
        let mut seed=0x8ae1_3d37_42c8_a70bu64;
        let mut next=|| {seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed};
        let records:Vec<_>=(0..257).map(|i| {
            let c0=if i<boundaries.len(){boundaries[i]}else{let mut w=core::array::from_fn(|_|next()); w[3]&=(1<<61)-1; w};
            let c1=if i<boundaries.len(){boundaries[boundaries.len()-1-i]}else{let mut w=core::array::from_fn(|_|next()); w[3]&=(1<<61)-1; w};
            Record{tag:i as u64,coordinate:Fq2::new(Fq::from_montgomery(c0),Fq::from_montgomery(c1)),other:Fq2::ONE}
        }).collect();
        let mut lane_inputs=0;
        for step in 0..1031usize {
            for mode in 0..3 {
                let refs=core::array::from_fn(|lane| &records[match mode{0=>(step+lane*37)%records.len(),1=>step%records.len(),_ =>(step+7-lane)%records.len()}].coordinate);
                let packed=Ifma8::from_fq2_refs(refs);
                for component in 0..2 {
                    let fields=refs.map(|v|if component==0 {v.c0}else{v.c1});
                    let original=Ifma8::<FqConfig,true>::new(&fields);
                    for limb in 0..5 {
                        let actual:[u64;8]=unsafe{core::mem::transmute(packed[component].0[limb])};
                        let old:[u64;8]=unsafe{core::mem::transmute(original.0[limb])};
                        assert_eq!(actual,old);
                        for lane in 0..8 {
                            let bit=limb*52;let word=bit/64;let offset=bit%64;
                            let mut expected=fields[lane].0[word]>>offset;
                            if offset!=0 && word+1<4 {expected|=fields[lane].0[word+1]<<(64-offset);}
                            expected&=(1u64<<52)-1;
                            assert_eq!(actual[lane],expected);
                        }
                    }
                    assert_eq!(packed[component].get().map(|v|v.0),original.get().map(|v|v.0));
                    assert_eq!(packed[component].mul(&packed[1-component]).get(),refs.map(|r|{let canonical=|v:Fq|{let mut w=v.0;while !portable::less(w,p){w=portable::sub4(w,p).0;}Fq::from_montgomery(w)};canonical(r.c0)*canonical(r.c1)}));
                }
                lane_inputs+=16;
            }
        }
        assert_eq!(lane_inputs,49488);
        assert!(records.iter().any(|r| (&r.coordinate as *const Fq2 as usize)%64!=0));
        println!("PAIRED_PACK_FIELD_LANES={lane_inputs}");
    }
}
