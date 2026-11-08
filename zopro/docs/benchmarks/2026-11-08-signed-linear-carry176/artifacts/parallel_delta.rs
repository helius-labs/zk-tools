// Included in signed_delta's native-gated module, outside generated ifma8.
impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED> {
    #[inline(always)]
    fn parallel_delta_splat(x:u64)->__m512i { unsafe{_mm512_set1_epi64(x as i64)} }
    #[inline(always)]
    fn parallel_delta_normalize(s:[__m512i;5])->Self {
        unsafe {
            let mask=Self::parallel_delta_splat(MASK);
            let mut v:[__m512i;5]=core::array::from_fn(|j|{
                if j==0{_mm512_and_si512(s[0],mask)}
                else if j==4{_mm512_add_epi64(s[4],_mm512_srai_epi64::<52>(s[3]))}
                else{_mm512_add_epi64(_mm512_and_si512(s[j],mask),_mm512_srai_epi64::<52>(s[j-1]))}
            });
            let middle=_mm512_or_si512(v[1],_mm512_or_si512(v[2],v[3]));
            let needs=_mm512_test_epi64_mask(middle,Self::parallel_delta_splat(!MASK));
            if needs!=0 {
                core::hint::cold_path();
                for j in 1..4 {
                    v[j+1]=_mm512_add_epi64(v[j+1],_mm512_srai_epi64::<52>(v[j]));
                    v[j]=_mm512_and_si512(v[j],mask);
                }
            }
            v[4]=_mm512_and_si512(v[4],mask);
            Self(v,PhantomData)
        }
    }
    /// Canonical inputs below p; returns exactly the original p+a-b digits.
    #[inline(always)]
    pub fn parallel_sub_canonical(&self,b:&Self)->Self {
        let p=split(C::P);
        let s=core::array::from_fn(|j|unsafe{
            _mm512_sub_epi64(_mm512_add_epi64(Self::parallel_delta_splat(p[j]),self.0[j]),b.0[j])
        });
        Self::parallel_delta_normalize(s)
    }
}
impl<C:FpConfig> Ifma8<C,true> {
    /// Exact original signed dy, with its original strict<2p correction.
    #[inline(always)]
    pub fn parallel_signed_difference_canonical(&self,ay:&Self,negative:u8)->Self {
        let p=split(C::P);let twice=split(crate::portable::add4(C::P,C::P).0);
        let s=core::array::from_fn(|j|unsafe{
            let offset=_mm512_mask_blend_epi64(negative,Self::parallel_delta_splat(p[j]),Self::parallel_delta_splat(twice[j]));
            let signed=_mm512_mask_sub_epi64(self.0[j],negative,Self::parallel_delta_splat(0),self.0[j]);
            _mm512_sub_epi64(_mm512_add_epi64(offset,signed),ay.0[j])
        });
        Self::signed_delta_reduce_fused(Self::parallel_delta_normalize(s).0)
    }
}
#[cfg(zopro_parallel_delta_diagnostics)]
impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED> {
    /// Qualification only; caller supplies canonical raw Montgomery words.
    pub fn parallel_delta_from_words(w:&[[u64;4];8])->Self {
        let d=w.map(split);
        Self(core::array::from_fn(|j|unsafe{
            let column:[u64;8]=core::array::from_fn(|i|d[i][j]);_mm512_loadu_si512(column.as_ptr().cast())
        }),PhantomData)
    }
    pub fn parallel_delta_digits(&self)->[[u64;8];5]{self.0.map(|x|unsafe{core::mem::transmute(x)})}
    pub fn parallel_delta_raw_normalize(s:&[[i64;8];5])->[[u64;8];5]{
        let s=s.map(|c|unsafe{_mm512_loadu_si512(c.as_ptr().cast())});
        Self::parallel_delta_normalize(s).parallel_delta_digits()
    }
}
