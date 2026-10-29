impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    #[cfg(golden_basis)]
    #[inline(always)]
    pub fn golden_product(a0: &Self, a1: &Self, b0: &Self, b1: &Self) -> [Self; 2] {
        assert!(C::P[3] < (1u64 << 62));
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let two_p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            let mut s = [zero; 5];
            let mut cs = zero;
            for j in 0..5 {
                let sv = _mm512_add_epi64(_mm512_add_epi64(a0.0[j], a1.0[j]), cs);
                cs = _mm512_srli_epi64::<52>(sv);
                s[j] = _mm512_and_si512(sv, mask);
            }
            let mut cd = zero;
            let mut x = [zero; 6];
            let mut y = [zero; 6];
            macro_rules! row {
                ($i:literal) => {{
                    let dv = _mm512_add_epi64(
                        _mm512_sub_epi64(_mm512_add_epi64(b0.0[$i], two_p[$i]), b1.0[$i]),
                        cd,
                    );
                    cd = _mm512_srai_epi64::<52>(dv);
                    let di = _mm512_and_si512(dv, mask);
                    macro_rules! product {
                        ($j:literal,$k:literal) => {{
                            let pl = _mm512_madd52lo_epu64(zero, s[$j], b1.0[$i]);
                            let ph = _mm512_madd52hi_epu64(zero, s[$j], b1.0[$i]);
                            x[$j] = _mm512_add_epi64(_mm512_madd52lo_epu64(x[$j], a0.0[$j], di), pl);
                            x[$k] = _mm512_add_epi64(_mm512_madd52hi_epu64(x[$k], a0.0[$j], di), ph);
                            y[$j] =
                                _mm512_add_epi64(_mm512_madd52lo_epu64(y[$j], a1.0[$j], b0.0[$i]), pl);
                            y[$k] =
                                _mm512_add_epi64(_mm512_madd52hi_epu64(y[$k], a1.0[$j], b0.0[$i]), ph);
                        }};
                    }
                    product!(0, 1);
                    product!(1, 2);
                    product!(2, 3);
                    product!(3, 4);
                    product!(4, 5);
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
                }};
            }
            row!(0);
            row!(1);
            row!(2);
            row!(3);
            row!(4);
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

impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    /// Divide an inclusive [0,2p] Montgomery representative by three modulo p.
    /// Only p=1 mod3 is supported. All digits are normalized radix 2^52.
    #[inline(always)]
    pub fn golden_div3(&self)->Self {
        assert_eq!(C::P,crate::FqConfig::P);
        unsafe {
            let zero=Self::splat(0);let mask=Self::splat(MASK);let three=Self::splat(3);
            let sum=self.0.into_iter().fold(zero,|s,v|_mm512_add_epi64(s,v));
            let low=_mm512_and_si512(sum,mask);let high=_mm512_srli_epi64::<52>(sum);
            let q=_mm512_madd52hi_epu64(zero,low,Self::splat(MASK/3));
            let triple=_mm512_add_epi64(q,_mm512_slli_epi64::<1>(q));
            let mut residue=_mm512_add_epi64(_mm512_sub_epi64(low,triple),high);
            for _ in 0..2 {residue=_mm512_mask_sub_epi64(residue,_mm512_cmpge_epu64_mask(residue,three),residue,three);}
            let add_p=_mm512_cmpeq_epi64_mask(residue,Self::splat(2));
            let add_two=_mm512_cmpeq_epi64_mask(residue,Self::splat(1));
            let p=split(C::P).map(Self::splat);let two=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);
            let mut carry=zero;let mut value=[zero;5];
            for j in 0..5 {
                let added=_mm512_or_si512(_mm512_maskz_mov_epi64(add_p,p[j]),_mm512_maskz_mov_epi64(add_two,two[j]));
                let v=_mm512_add_epi64(_mm512_add_epi64(self.0[j],added),carry);
                value[j]=_mm512_and_si512(v,mask);carry=_mm512_srli_epi64::<52>(v);
            }
            let inv3=Self::splat((2*(1u64<<52)+1)/3);carry=zero;
            let mut out=[zero;5];
            for j in 0..5 {
                let v=_mm512_sub_epi64(value[j],carry);
                out[j]=_mm512_and_si512(_mm512_madd52lo_epu64(zero,v,inv3),mask);
                let triple=_mm512_add_epi64(out[j],_mm512_slli_epi64::<1>(out[j]));
                carry=_mm512_srli_epi64::<52>(_mm512_sub_epi64(triple,v));
            }
            Self(out,PhantomData)
        }
    }
    #[inline(always)]
    pub fn golden_square(a:&Self,b:&Self)->[Self;2] {
        let twice=b.add(b);let p=a.mul(&a.sub(&twice));
        let q=a.add(b).mul(&a.add(&twice.add(b)));
        let v=q.sub(&p).golden_div3();[p.add(&v),v]
    }
}
