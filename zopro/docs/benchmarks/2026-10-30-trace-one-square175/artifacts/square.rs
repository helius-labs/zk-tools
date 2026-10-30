// Private trace-one square. X=a²+b²; Y=2ab+b².
impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
 #[inline(always)] pub fn trace_one_square(a:&Self,b:&Self)->[Self;2]{unsafe{
 let zero=Self::splat(0);let mask=Self::splat(MASK);let p=split(C::P).map(Self::splat);let inv=Self::splat(C::INV&MASK);let mut x=[zero;6];let mut y=[zero;6];
 macro_rules! row{($i:literal)=>{{
 macro_rules! square{($j:literal,$k:literal)=>{{if $j>=$i{
 let al=_mm512_madd52lo_epu64(zero,a.0[$i],a.0[$j]);let ah=_mm512_madd52hi_epu64(zero,a.0[$i],a.0[$j]);
 let bl=_mm512_madd52lo_epu64(zero,b.0[$i],b.0[$j]);let bh=_mm512_madd52hi_epu64(zero,b.0[$i],b.0[$j]);
 let(al,ah,bl,bh)=if $j==$i{(al,ah,bl,bh)}else{(_mm512_slli_epi64::<1>(al),_mm512_slli_epi64::<1>(ah),_mm512_slli_epi64::<1>(bl),_mm512_slli_epi64::<1>(bh))};
 x[$j]=_mm512_add_epi64(x[$j],_mm512_add_epi64(al,bl));x[$k]=_mm512_add_epi64(x[$k],_mm512_add_epi64(ah,bh));
 y[$j]=_mm512_add_epi64(y[$j],bl);y[$k]=_mm512_add_epi64(y[$k],bh);
 }}};}square!(0,1);square!(1,2);square!(2,3);square!(3,4);square!(4,5);
 macro_rules! cross{($j:literal,$k:literal)=>{{let lo=_mm512_madd52lo_epu64(zero,a.0[$j],b.0[$i]);let hi=_mm512_madd52hi_epu64(zero,a.0[$j],b.0[$i]);y[$j]=_mm512_add_epi64(y[$j],_mm512_slli_epi64::<1>(lo));y[$k]=_mm512_add_epi64(y[$k],_mm512_slli_epi64::<1>(hi));}};}cross!(0,1);cross!(1,2);cross!(2,3);cross!(3,4);cross!(4,5);
                    macro_rules! cancel {
                        ($t:ident) => {{
                            let m = _mm512_madd52lo_epu64(zero, $t[0], inv);
                            let m = if MIXED && $i == 4 {
                                _mm512_and_si512(m, Self::splat((1u64 << 48) - 1))
                            } else {
                                _mm512_and_si512(m, mask)
                            };
                            let low = if MIXED && $i == 4 {
                                let c = _mm512_srli_epi64::<48>(_mm512_add_epi64(
                                    $t[0],
                                    Self::splat((1u64 << 48) - 1),
                                ));
                                _mm512_madd52hi_epu64(c, _mm512_slli_epi64::<4>(m), p[0])
                            } else {
                                let c = _mm512_srli_epi64::<52>(_mm512_add_epi64($t[0], mask));
                                _mm512_madd52hi_epu64(c, m, p[0])
                            };
                            macro_rules! reduction {
                                ($j:literal, $k:literal) => {{
                                    $t[$j] = _mm512_madd52lo_epu64($t[$j], m, p[$j]);
                                    $t[$k] = _mm512_madd52hi_epu64($t[$k], m, p[$j]);
                                }};
                            }
                            reduction!(1, 2);
                            reduction!(2, 3);
                            reduction!(3, 4);
                            reduction!(4, 5);
                            if MIXED && $i == 4 {
                                let mask48 = Self::splat((1u64 << 48) - 1);
                                $t = [
                                    _mm512_add_epi64(
                                        low,
                                        _mm512_slli_epi64::<4>(_mm512_and_si512($t[1], mask48)),
                                    ),
                                    _mm512_add_epi64(
                                        _mm512_srli_epi64::<48>($t[1]),
                                        _mm512_slli_epi64::<4>(_mm512_and_si512($t[2], mask48)),
                                    ),
                                    _mm512_add_epi64(
                                        _mm512_srli_epi64::<48>($t[2]),
                                        _mm512_slli_epi64::<4>(_mm512_and_si512($t[3], mask48)),
                                    ),
                                    _mm512_add_epi64(
                                        _mm512_srli_epi64::<48>($t[3]),
                                        _mm512_slli_epi64::<4>(_mm512_and_si512($t[4], mask48)),
                                    ),
                                    _mm512_add_epi64(
                                        _mm512_srli_epi64::<48>($t[4]),
                                        _mm512_slli_epi64::<4>($t[5]),
                                    ),
                                    zero,
                                ];
                            } else {
                                $t[1] = _mm512_add_epi64($t[1], low);
                                $t = [$t[1], $t[2], $t[3], $t[4], $t[5], zero];
                            }
                        }};
                    }
                    cancel!(x);
                    cancel!(y);
}};}row!(0);row!(1);row!(2);row!(3);row!(4);
            for j in 0..4 {
                x[j + 1] = _mm512_add_epi64(x[j + 1], _mm512_srli_epi64::<52>(x[j]));
                x[j] = _mm512_and_si512(x[j], mask);
                y[j + 1] = _mm512_add_epi64(y[j + 1], _mm512_srli_epi64::<52>(y[j]));
                y[j] = _mm512_and_si512(y[j], mask);
            }
            let x = [x[0], x[1], x[2], x[3], x[4]];
            let y = [y[0], y[1], y[2], y[3], y[4]];
            if MIXED {
                [Self::reduce_fused(x), Self::reduce_fused(y)]
            } else {
                [Self(x, PhantomData), Self(y, PhantomData)]
            }
        }
    }

}
