// Exact inherited arithmetic body; the new dispatch is in the parent.
impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED> {
    // -p^-1 modulo the full Montgomery radix, computed at compile time.
    const TRUNCATED_INVERSE: [u64; 5] = {
        let p = split(C::P);
        let mut out = [0u64; 5];
        let mut carry = 0u128;
        let mut i = 0;
        while i < 5 {
            let mut value = carry + (i == 0) as u128;
            let mut j = 1;
            while j <= i {
                value += p[j] as u128 * out[i - j] as u128;
                j += 1;
            }
            out[i] = (value as u64).wrapping_mul(C::INV) & MASK;
            carry = (value + p[0] as u128 * out[i] as u128) >> 52;
            i += 1;
        }
        if MIXED {
            out[4] &= (1u64 << 48) - 1;
        }
        out
    };

    /// Exact full-radix cancellation with only the upper q*p coefficients.
    /// The omitted T-low + q*p carry into coefficient4 is in0..=9.
    /// Since T+q*p is divisible byR, its outgoing carry is ceil(high4/D),
    /// where D is2^48 or2^52. No approximation error enters the result.
    #[inline(always)]
    fn product_truncated<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {
        unsafe {
            let zero = Self::splat(0);
            let mask = Self::splat(MASK);
            let mut t = [zero; 10];
            for i in 0..5 {
                for j in 0..5 {
                    t[i + j] = _mm512_madd52lo_epu64(t[i + j], self.0[i], b.0[j]);
                    t[i + j + 1] = _mm512_madd52hi_epu64(t[i + j + 1], self.0[i], b.0[j]);
                    if FUSED {
                        t[i + j] = _mm512_madd52lo_epu64(t[i + j], c.0[i], d.0[j]);
                        t[i + j + 1] = _mm512_madd52hi_epu64(t[i + j + 1], c.0[i], d.0[j]);
                    }
                }
            }
            for j in 0..5 {
                t[j + 1] = _mm512_add_epi64(t[j + 1], _mm512_srli_epi64::<52>(t[j]));
                t[j] = _mm512_and_si512(t[j], mask);
            }
            let negative_inverse = Self::TRUNCATED_INVERSE.map(Self::splat);
            let mut q = [zero; 5];
            for i in 0..5 {
                for j in 0..5 - i {
                    q[i + j] = _mm512_madd52lo_epu64(q[i + j], t[i], negative_inverse[j]);
                    if i + j < 4 {
                        q[i + j + 1] =
                            _mm512_madd52hi_epu64(q[i + j + 1], t[i], negative_inverse[j]);
                    }
                }
            }
            for j in 0..4 {
                q[j + 1] = _mm512_add_epi64(q[j + 1], _mm512_srli_epi64::<52>(q[j]));
                q[j] = _mm512_and_si512(q[j], mask);
            }
            q[4] = _mm512_and_si512(q[4], Self::splat(if MIXED { (1 << 48) - 1 } else { MASK }));
            let p = split(C::P).map(Self::splat);
            let mut high = [t[4], t[5], t[6], t[7], t[8], t[9]];
            for i in 0..5 {
                for j in 0..5 {
                    if i + j >= 4 {
                        high[i + j - 4] = _mm512_madd52lo_epu64(high[i + j - 4], q[i], p[j]);
                    }
                    if i + j >= 3 {
                        high[i + j - 3] = _mm512_madd52hi_epu64(high[i + j - 3], q[i], p[j]);
                    }
                }
            }
            let mut out = if MIXED {
                let carry =
                    _mm512_srli_epi64::<48>(_mm512_add_epi64(high[0], Self::splat((1 << 48) - 1)));
                [
                    _mm512_add_epi64(carry, _mm512_slli_epi64::<4>(high[1])),
                    _mm512_slli_epi64::<4>(high[2]),
                    _mm512_slli_epi64::<4>(high[3]),
                    _mm512_slli_epi64::<4>(high[4]),
                    _mm512_slli_epi64::<4>(high[5]),
                ]
            } else {
                let carry = _mm512_srli_epi64::<52>(_mm512_add_epi64(high[0], mask));
                [
                    _mm512_add_epi64(carry, high[1]),
                    high[2],
                    high[3],
                    high[4],
                    high[5],
                ]
            };
            for j in 0..4 {
                out[j + 1] = _mm512_add_epi64(out[j + 1], _mm512_srli_epi64::<52>(out[j]));
                out[j] = _mm512_and_si512(out[j], mask);
            }
            if FUSED && MIXED {
                Self::reduce_fused(out)
            } else {
                Self(out, PhantomData)
            }
        }
    }

}
#[cfg(test)]
mod r256_truncated_tests {
    use super::*;
    pub(super) static HITS:core::sync::atomic::AtomicUsize=core::sync::atomic::AtomicUsize::new(0);
    fn check<C:FpConfig,const MIXED:bool>() {
        let twice=crate::portable::add4(C::P,C::P).0;
        let mut edges=vec![[0;4],[1,0,0,0],C::P,crate::portable::sub4(C::P,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0,twice];
        for bit in [48,52,104,156,208,253] {
            let mut a=[0;4]; a[bit/64]=1u64<<(bit%64);
            edges.extend([crate::portable::sub4(a,[1,0,0,0]).0,a,crate::portable::add4(a,[1,0,0,0]).0]);
        }
        let mut seed=0x7226c1f965a0d33bu64;
        let mut count=0;
        for round in 0..512 {
            let words:[[[u64;4];8];4]=core::array::from_fn(|operand|core::array::from_fn(|lane| {
                if round<edges.len()*2 { edges[(round+operand*5+lane*7)%edges.len()] } else {
                    let mut bytes=[0;64];
                    for chunk in bytes.chunks_mut(8) { seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes()); }
                    let a=Fp::<C>::from_random_bytes(&bytes).0;
                    if seed&1!=0 {crate::portable::add4(a,C::P).0} else {a}
                }
            }));
            let v=words.map(|w|Ifma8::<C,MIXED>::raw(&w));
            let negative=Ifma8::<C,MIXED>::raw(&words[2].map(|w|crate::portable::sub4(twice,w).0));
            let mut actual=v[0]; let mut expected=v[0];
            for step in 0..8 {
                let (a,b)=match step%3 {
                    0=>(actual.mul(&v[1]),expected.product_cios::<false>(&v[1],&expected,&v[1])),
                    1=>(actual.sum2(&v[1],&v[2],&v[3]),expected.product_cios::<true>(&v[1],&v[2],&v[3])),
                    _=>(actual.difference2(&v[1],&v[2],&v[3]),expected.product_cios::<true>(&v[1],&negative,&v[3])),
                };
                let raw_a:[[u64;8];5]=a.0.map(|v|unsafe{core::mem::transmute(v)});
                let raw_b:[[u64;8];5]=b.0.map(|v|unsafe{core::mem::transmute(v)});
                assert_eq!(raw_a,raw_b,"MIXED={MIXED} round={round} step={step}");
                let limit=split(twice);
                for lane in 0..8 {
                    assert!((0..5).all(|i|raw_a[i][lane]<=MASK));
                    let less=(0..5).rev().find(|&i|raw_a[i][lane]!=limit[i]).map(|i|raw_a[i][lane]<limit[i]).unwrap_or(false);
                    assert!(less);
                }
                assert_eq!(a.get(),b.get());
                actual=a;expected=b;count+=8;
            }
        }
        println!("R256_DISPATCH_EXACT_LANES mixed={MIXED} count={count}");
    }
    #[test] fn r256_selected_dispatch_exact_cios_digits() {let before=HITS.load(core::sync::atomic::Ordering::Relaxed);check::<crate::FqConfig,true>();check::<crate::FrConfig,true>();assert_eq!(HITS.load(core::sync::atomic::Ordering::Relaxed)-before,8192);}
    #[test] fn r260_public_dispatch_stays_cios() {let before=HITS.load(core::sync::atomic::Ordering::Relaxed);check::<crate::FqConfig,false>();check::<crate::FrConfig,false>();assert_eq!(HITS.load(core::sync::atomic::Ordering::Relaxed),before);}
}
