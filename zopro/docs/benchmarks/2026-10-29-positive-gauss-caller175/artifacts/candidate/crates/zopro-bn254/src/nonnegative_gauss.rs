impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    /// Three nonnegative raw products with two simultaneous unsigned REDCs.
    /// S=a0+a1, N=2p-a1, T=b0+b1, D=b1+2p-b0 are exact integers.
    /// X=S*b0+N*T=real+2p*T, Y=S*b0+a0*D=imag+2p*a0.
    /// Both are <=12p². Prepared raw arrays can exceed the public 2p bound.
    #[cfg(zopro_gauss_positive)]
    #[inline(always)]
    pub fn complex_product_gauss(a0: &Self, a1: &Self, b0: &Self, b1: &Self) -> [Self; 2] {
        assert!(C::P[3] < (1u64 << 62));
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let two_p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            let mut s = [zero; 5];
            let mut n = [zero; 5];
            let mut cs = zero;
            let mut cn = zero;
            for j in 0..5 {
                let sv = _mm512_add_epi64(_mm512_add_epi64(a0.0[j], a1.0[j]), cs);
                cs = _mm512_srli_epi64::<52>(sv);
                s[j] = _mm512_and_si512(sv, mask);
                let nv = _mm512_add_epi64(_mm512_sub_epi64(two_p[j], a1.0[j]), cn);
                cn = _mm512_srai_epi64::<52>(nv);
                n[j] = _mm512_and_si512(nv, mask);
            }
            let mut ct = zero;
            let mut cd = zero;
            let mut x = [zero; 6];
            let mut y = [zero; 6];
            macro_rules! row {
                ($i:literal) => {{
                    let tv = _mm512_add_epi64(_mm512_add_epi64(b0.0[$i], b1.0[$i]), ct);
                    ct = _mm512_srli_epi64::<52>(tv);
                    let ti = _mm512_and_si512(tv, mask);
                    let dv = _mm512_add_epi64(
                        _mm512_sub_epi64(_mm512_add_epi64(b1.0[$i], two_p[$i]), b0.0[$i]),
                        cd,
                    );
                    cd = _mm512_srai_epi64::<52>(dv);
                    let di = _mm512_and_si512(dv, mask);
                    macro_rules! product {
                        ($j:literal,$k:literal) => {{
                            let pl = _mm512_madd52lo_epu64(zero, s[$j], b0.0[$i]);
                            let ph = _mm512_madd52hi_epu64(zero, s[$j], b0.0[$i]);
                            x[$j] = _mm512_add_epi64(_mm512_madd52lo_epu64(x[$j], n[$j], ti), pl);
                            x[$k] = _mm512_add_epi64(_mm512_madd52hi_epu64(x[$k], n[$j], ti), ph);
                            y[$j] =
                                _mm512_add_epi64(_mm512_madd52lo_epu64(y[$j], a0.0[$j], di), pl);
                            y[$k] =
                                _mm512_add_epi64(_mm512_madd52hi_epu64(y[$k], a0.0[$j], di), ph);
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
