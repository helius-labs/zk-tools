//! Bounded G1 coordinate finish. Private native eight-lane experiment.
use super::*;
impl<C:FpConfig> Ifma8<C,true> {
    // Normalize a signed coefficient expression representing a nonnegative
    // integer below R. Coefficients have magnitude below 2^55.
    #[inline(always)]
    fn finish_signed(t:[__m512i;5])->Self {
        unsafe {
            let mut out=[Self::splat(0);5];let mut carry=Self::splat(0);
            for j in 0..5 {
                let v=_mm512_add_epi64(t[j],carry);
                out[j]=_mm512_and_si512(v,Self::splat(MASK));
                carry=_mm512_srai_epi64::<52>(v);
            }
            debug_assert_eq!(_mm512_cmpneq_epi64_mask(carry,Self::splat(0)),0);
            Self(out,PhantomData)
        }
    }
    #[inline(always)]
    fn finish_export(self)->[Fp<C>;8] {
        let twice=const{crate::portable::add4(C::P,C::P).0};
        let canonical=Self::reduce(Self::reduce(self.0,twice).0,C::P);
        let limbs:[[u64;8];5]=canonical.0.map(|v|unsafe{core::mem::transmute(v)});
        core::array::from_fn(|i|Fp::from_montgomery([
            limbs[0][i]|(limbs[1][i]<<52),
            (limbs[1][i]>>12)|(limbs[2][i]<<40),
            (limbs[2][i]>>24)|(limbs[3][i]<<28),
            (limbs[3][i]>>36)|(limbs[4][i]<<16),
        ]))
    }
    /// s<2p, ax,ay<p, xsum<2p; all normalized R256 values.
    /// Requires 5p<R. The loose operands stay inside this routine.
    /// Return canonical host coordinates for X=s²-xsum, Y=s(ax-X)-ay.
    #[inline(always)]
    pub fn affine_finish_loose(&self,ax:&Self,ay:&Self,xsum:&Self)->([Fp<C>;8],[Fp<C>;8]) {
        const { assert!(crate::portable::add4(crate::portable::add4(C::P,C::P).0,crate::portable::add4(C::P,C::P).0).1==0); }
        // Check the stronger 5p<R modulus property at compile time.
        const { let p2=crate::portable::add4(C::P,C::P).0;let p4=crate::portable::add4(p2,p2).0;assert!(crate::portable::add4(p4,C::P).1==0); }
        unsafe {
            let p=split(C::P).map(Self::splat);
            let p2=split(crate::portable::add4(C::P,C::P).0).map(Self::splat);
            let u=self.mul(self); // u< p+4p²/R < 1.8p.
            let x=Self::finish_signed(core::array::from_fn(|j|
                _mm512_sub_epi64(_mm512_add_epi64(p2[j],u.0[j]),xsum.0[j])));
            let delta=Self::finish_signed(core::array::from_fn(|j|
                _mm512_sub_epi64(_mm512_add_epi64(_mm512_add_epi64(p2[j],ax.0[j]),xsum.0[j]),u.0[j])));
            // delta<5p<R. The unchanged nonfused CIOS recurrence is valid
            // for these normalized digits; result<p+10p²/R<3p.
            let product=self.mul(&delta);
            let y=Self::finish_signed(core::array::from_fn(|j|
                _mm512_sub_epi64(_mm512_add_epi64(p[j],product.0[j]),ay.0[j])));
            (x.finish_export(),y.finish_export())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn loose_finish_matches_independent_arkworks_and_canonical_exports() {
        fn check<C:FpConfig,A:ark_ff::PrimeField>() {
            use ark_ff::Field;
            let number=|x:[u64;4]| { let mut bytes=[0u8;32];for(c,w)in bytes.chunks_mut(8).zip(x){c.copy_from_slice(&w.to_le_bytes());}A::from_le_bytes_mod_order(&bytes) };
            let ri=A::from(2u64).pow([256]).inverse().unwrap();
            let twice=crate::portable::add4(C::P,C::P).0;
            let max=crate::portable::sub4(C::P,[1,0,0,0]).0;
            let max2=crate::portable::sub4(twice,[1,0,0,0]).0;
            let edges=[[0;4],[1,0,0,0],max,C::P,max2];
            let mut seed=0xc017d1a54b9e2813u64;
            let mut checked=0;
            for batch in 0..4160 {
                let input:[[[u64;4];8];4]=core::array::from_fn(|which|core::array::from_fn(|i|{
                    if batch<64 {let e=edges[(batch/((which+1)*3)+i+which)%edges.len()];if which==1||which==2 {if crate::portable::less(e,C::P){e}else{max}}else{e}}
                    else {
                        let mut bytes=[0;64];for c in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;c.copy_from_slice(&seed.to_le_bytes());}
                        let w=Fp::<C>::from_random_bytes(&bytes).0;
                        if (which==0||which==3)&&(batch+i)%2==0 {crate::portable::add4(w,C::P).0}else{w}
                    }
                }));
                let v=input.map(|x|Ifma8::<C,true>::raw(&x));
                let (x,y)=v[0].affine_finish_loose(&v[1],&v[2],&v[3]);
                let ox=v[0].mul(&v[0]).sub(&v[3]);
                let oy=v[0].mul(&v[1].sub(&ox)).sub(&v[2]);
                assert_eq!(x,ox.get());assert_eq!(y,oy.get());
                for i in 0..8 {
                    let s=number(input[0][i])*ri;
                    let ax=number(input[1][i])*ri;let ay=number(input[2][i])*ri;let sum=number(input[3][i])*ri;
                    let want=s*s-sum;
                    assert_eq!(number(x[i].0)*ri,want);
                    assert_eq!(number(y[i].0)*ri,s*(ax-want)-ay);
                    assert!(crate::portable::less(x[i].0,C::P));assert!(crate::portable::less(y[i].0,C::P));
                    checked+=2;
                }
            }
            println!("LOOSE_FINISH_ORACLE_COORDINATES={checked}");
        }
        check::<crate::FqConfig,ark_ref::Fq>();check::<crate::FrConfig,ark_ref::Fr>();
    }
}
