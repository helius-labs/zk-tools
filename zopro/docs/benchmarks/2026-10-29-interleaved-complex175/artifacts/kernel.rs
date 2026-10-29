impl<C:FpConfig,const MIXED:bool>Ifma8<C,MIXED>{
    #[cfg(zopro_interleaved_complex)]
    #[inline(always)]
    pub fn complex_interleaved<const CONJ:bool>(a0: &Self, a1: &Self, b0: &Self, b1: &Self) -> [Self; 2] {
        assert!(C::P[3] < (1u64 << 62));
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let p = split(C::P).map(Self::splat);
            let two_p = split(crate::portable::add4(C::P, C::P).0).map(Self::splat);
            let inv = Self::splat(C::INV & MASK);
            let mut borrow = zero;
            let mut x = [zero; 6];
            let mut y = [zero; 6];
            macro_rules! row {
                ($i:literal) => {{
                    let v = _mm512_add_epi64(_mm512_sub_epi64(two_p[$i],b1.0[$i]),borrow);
                    borrow = _mm512_srai_epi64::<52>(v);
                    let negative = _mm512_and_si512(v,mask);
                    let real_b1 = if CONJ {b1.0[$i]}else{negative};
                    let imag_b1 = if CONJ {negative}else{b1.0[$i]};
                    macro_rules! product {
                        ($j:literal,$k:literal)=>{{
                            x[$j]=_mm512_madd52lo_epu64(x[$j],a0.0[$j],b0.0[$i]);
                            x[$k]=_mm512_madd52hi_epu64(x[$k],a0.0[$j],b0.0[$i]);
                            x[$j]=_mm512_madd52lo_epu64(x[$j],a1.0[$j],real_b1);
                            x[$k]=_mm512_madd52hi_epu64(x[$k],a1.0[$j],real_b1);
                            y[$j]=_mm512_madd52lo_epu64(y[$j],a1.0[$j],b0.0[$i]);
                            y[$k]=_mm512_madd52hi_epu64(y[$k],a1.0[$j],b0.0[$i]);
                            y[$j]=_mm512_madd52lo_epu64(y[$j],a0.0[$j],imag_b1);
                            y[$k]=_mm512_madd52hi_epu64(y[$k],a0.0[$j],imag_b1);
                        }};
                    }
                    product!(0,1);product!(1,2);product!(2,3);product!(3,4);product!(4,5);
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
#[cfg(test)]mod interleaved_complex_tests{
use super::*;use ark_ff::{Field,PrimeField};
fn check<C:FpConfig,A:PrimeField,const MIXED:bool>(){
    let p=C::P;let twice=crate::portable::add4(p,p).0;
    let edges=[[0;4],[1,0,0,0],crate::portable::sub4(p,[1,0,0,0]).0,p,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
    let numeric=|w:[u64;4]|A::from_le_bytes_mod_order(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>());
    let inv=A::from(2u64).pow([if MIXED{256}else{260}]).inverse().unwrap();let hostinv=A::from(2u64).pow([256]).inverse().unwrap();
    let mut seed=0x53545245414d4e45u64;let limit=split(twice);let mut checked=0;
    for group in 0..(1296+4096)/8{
        let input:[[[u64;4];8];4]=core::array::from_fn(|slot|core::array::from_fn(|lane|{let i=group*8+lane;if i<1296{edges[(i/6usize.pow(slot as u32))%6]}else{let mut bytes=[0u8;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}let v=Fp::<C>::from_random_bytes(&bytes).0;if seed&1==0{v}else{crate::portable::add4(v,p).0}}}));
        let v=input.map(|w|Ifma8::<C,MIXED>::raw(&w));let scalar=input.map(|w|w.map(|a|numeric(a)*inv));
        for conj in[false,true]{let mut x=[v[0],v[1]];let mut expected=[scalar[0],scalar[1]];
            for _ in 0..8{
                let out=if conj{Ifma8::complex_interleaved::<true>(&x[0],&x[1],&v[2],&v[3])}else{Ifma8::complex_interleaved::<false>(&x[0],&x[1],&v[2],&v[3])};
                let next:[[_;8];2]=core::array::from_fn(|slot|core::array::from_fn(|lane|{let a=expected[0][lane];let b=expected[1][lane];let c=scalar[2][lane];let d=scalar[3][lane];match(conj,slot){(false,0)=>a*c-b*d,(false,1)=>a*d+b*c,(true,0)=>a*c+b*d,(true,1)=>b*c-a*d,_=>unreachable!()}}));
                for slot in 0..2 {let host=out[slot].get();let digits:[[u64;8];5]=out[slot].0.map(|v|unsafe{core::mem::transmute(v)});for lane in 0..8{assert_eq!(numeric(host[lane].0)*hostinv,next[slot][lane]);assert!((0..5).all(|j|digits[j][lane]<(1u64<<52)));assert!((0..5).rev().map(|j|digits[j][lane]).cmp((0..5).rev().map(|j|limit[j])).is_lt());checked+=1;}}
                x=out;expected=next;
            }
        }
    }
    eprintln!("INTERLEAVED_COMPLEX_CHECKED_OUTPUTS={checked};MIXED={MIXED}");
}
#[test]fn fq_interleaved_complex_arkworks(){check::<crate::FqConfig,ark_ref::Fq,true>();check::<crate::FqConfig,ark_ref::Fq,false>();}
#[test]fn fr_interleaved_complex_arkworks(){check::<crate::FrConfig,ark_ref::Fr,true>();check::<crate::FrConfig,ark_ref::Fr,false>();}
}
