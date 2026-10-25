#!/usr/bin/env python3
"""Generate the eight-lane IFMA kernel from the tested four-lane kernel."""
from pathlib import Path
import re
import subprocess
import sys

root = Path(__file__).resolve().parents[1]
path = root / "crates/zopro-bn254/src/ifma.rs"
source = path.read_text().split("#[cfg(test)]")[0]
start = source.index("    #[inline(always)]\n    pub fn shuffle")
body = source.index("{", start)
depth, end = 1, body + 1
while depth:
    depth += (source[end] == "{") - (source[end] == "}")
    end += 1
source = source[:start] + source[end:]
assert source.count("const LANES: usize = 4;") == 1, "lane-count selector changed"
source = source.replace("const LANES: usize = 4;", "const LANES: usize = 8;")
source = source.replace("Ifma", "Ifma8").replace("__m256i", "__m512i")
source = source.replace("_mm256_", "_mm512_")
source = source.replace("_si256", "_si512")
source = source.replace("set1_epi64x", "set1_epi64").replace("set_epi64x", "set_epi64")
source = source.replace("&[[u64; 4]; 4]", "&[[u64; 4]; 8]")
source = source.replace("&[Fp<C>; 4]", "&[Fp<C>; 8]")
source = source.replace("-> [Fp<C>; 4]", "-> [Fp<C>; 8]")
source = source.replace("[[u64; 4]; 5]", "[[u64; 8]; 5]")
source, count = re.subn(
    r"_mm512_set_epi64\(\s*x\[3\]\[j\] as i64,\s*x\[2\]\[j\] as i64,\s*x\[1\]\[j\] as i64,\s*x\[0\]\[j\] as i64,?\s*\)",
    "_mm512_set_epi64(" + ", ".join(f"x[{i}][j] as i64" for i in range(7, -1, -1)) + ")",
    source,
)
assert count == 1, "four-lane input constructor changed"
# Eight rows of four host limbs transpose to four vector columns before
# bit packing. This avoids forty scalar splits and vector lane inserts.
start = source.index("    #[inline(always)]\n    fn raw(")
body = source.index("{", start)
depth, end = 1, body + 1
while depth:
    depth += (source[end] == "{") - (source[end] == "}")
    end += 1
source = source[:start] + r"""    #[inline(always)]
    fn raw(x: &[[u64; 4]; 8]) -> Self {
        // SAFETY: the input is 256 initialized bytes; loads are unaligned.
        // All permutations address lanes in the two supplied vectors.
        unsafe {
            let ptr=x.as_ptr().cast::<__m512i>();
            let r0=_mm512_loadu_si512(ptr);
            let r1=_mm512_loadu_si512(ptr.add(1));
            let r2=_mm512_loadu_si512(ptr.add(2));
            let r3=_mm512_loadu_si512(ptr.add(3));
            let a=_mm512_unpacklo_epi64(r0,r1);
            let b=_mm512_unpackhi_epi64(r0,r1);
            let c=_mm512_unpacklo_epi64(r2,r3);
            let d=_mm512_unpackhi_epi64(r2,r3);
            let low=_mm512_set_epi64(13,9,12,8,5,1,4,0);
            let high=_mm512_set_epi64(15,11,14,10,7,3,6,2);
            let w0=_mm512_permutex2var_epi64(a,low,c);
            let w1=_mm512_permutex2var_epi64(b,low,d);
            let w2=_mm512_permutex2var_epi64(a,high,c);
            let w3=_mm512_permutex2var_epi64(b,high,d);
            let mask=Self::splat(MASK);
            Self([
                _mm512_and_si512(w0,mask),
                _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<52>(w0),_mm512_slli_epi64::<12>(w1)),mask),
                _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<40>(w1),_mm512_slli_epi64::<24>(w2)),mask),
                _mm512_and_si512(_mm512_or_si512(_mm512_srli_epi64::<28>(w2),_mm512_slli_epi64::<36>(w3)),mask),
                _mm512_srli_epi64::<16>(w3),
            ],PhantomData)
        }
    }""" + source[end:]
# Transpose four limb columns back to eight contiguous host fields.
start = source.index("        let limbs: [[u64; 8]; 5] = x.0.map(")
end = source.index("    #[inline(always)]\n    pub fn zero", start)
source = source[:start] + r"""        // Fully initialize eight repr-transparent Fp values in four stores.
        unsafe {
            let [t0,t1,t2,t3,t4]=x.0;
            let w0=_mm512_or_si512(t0,_mm512_slli_epi64::<52>(t1));
            let w1=_mm512_or_si512(_mm512_srli_epi64::<12>(t1),_mm512_slli_epi64::<40>(t2));
            let w2=_mm512_or_si512(_mm512_srli_epi64::<24>(t2),_mm512_slli_epi64::<28>(t3));
            let w3=_mm512_or_si512(_mm512_srli_epi64::<36>(t3),_mm512_slli_epi64::<16>(t4));
            let a=_mm512_unpacklo_epi64(w0,w1);
            let b=_mm512_unpackhi_epi64(w0,w1);
            let c=_mm512_unpacklo_epi64(w2,w3);
            let d=_mm512_unpackhi_epi64(w2,w3);
            let low=_mm512_set_epi64(11,10,3,2,9,8,1,0);
            let high=_mm512_set_epi64(15,14,7,6,13,12,5,4);
            let even_lo=_mm512_permutex2var_epi64(a,low,c);
            let even_hi=_mm512_permutex2var_epi64(a,high,c);
            let odd_lo=_mm512_permutex2var_epi64(b,low,d);
            let odd_hi=_mm512_permutex2var_epi64(b,high,d);
            let mut out=core::mem::MaybeUninit::<[Fp<C>;8]>::uninit();
            let p=out.as_mut_ptr().cast::<__m512i>();
            _mm512_storeu_si512(p,_mm512_shuffle_i64x2::<0x44>(even_lo,odd_lo));
            _mm512_storeu_si512(p.add(1),_mm512_shuffle_i64x2::<0xee>(even_lo,odd_lo));
            _mm512_storeu_si512(p.add(2),_mm512_shuffle_i64x2::<0x44>(even_hi,odd_hi));
            _mm512_storeu_si512(p.add(3),_mm512_shuffle_i64x2::<0xee>(even_hi,odd_hi));
            out.assume_init()
        }
    }
""" + source[end:]
source = source.replace("//! Four independent", "//! Eight independent")
source = "// Generated by scripts/ifma8.py from ifma.rs; regenerate after changes.\n" + source
source += r'''
impl<C: FpConfig, const MIXED: bool> Ifma8<C, MIXED> {
    /// Join two four-lane values without changing their Montgomery radix.
    #[inline(always)]
    pub fn from_halves(lo: &crate::ifma::Ifma<C, MIXED>, hi: &crate::ifma::Ifma<C, MIXED>) -> Self {
        unsafe {
            Self(core::array::from_fn(|j| _mm512_inserti64x4::<1>(_mm512_castsi256_si512(lo.0[j]), hi.0[j])), PhantomData)
        }
    }
    /// Sum all eight independent field lanes.
    #[inline(always)]
    #[allow(unexpected_cfgs)]
    pub fn horizontal_sum(&self) -> Fp<C> {
        #[cfg(zopro_horizontal_limb_sum)]
        if MIXED { return self.horizontal_sum_limb_table(); }
        unsafe {
            let mut value = *self;
            for index in [
                _mm512_set_epi64(3, 2, 1, 0, 7, 6, 5, 4),
                _mm512_set_epi64(5, 4, 7, 6, 1, 0, 3, 2),
                _mm512_set_epi64(6, 7, 4, 5, 2, 3, 0, 1),
            ] {
                let shifted = Self(value.0.map(|v| _mm512_permutexvar_epi64(index, v)), PhantomData);
                value = value.add(&shifted);
            }
            value.get()[0]
        }
    }
    #[inline(always)]
    pub fn halves(&self) -> (crate::ifma::Ifma<C, MIXED>, crate::ifma::Ifma<C, MIXED>) {
        unsafe {
            (crate::ifma::Ifma(self.0.map(|v| _mm512_castsi512_si256(v)), PhantomData),
             crate::ifma::Ifma(self.0.map(|v| _mm512_extracti64x4_epi64::<1>(v)), PhantomData))
        }
    }
    /// Apply the same four-lane shuffle to two independent transforms.
    #[inline(always)]
    pub fn shuffle_pair<const A: usize, const B: usize, const C0: usize, const D: usize>(a: &Self, b: &Self) -> Self {
        const fn index(i: usize) -> i64 { if i < 4 { i as i64 } else { (i + 4) as i64 } }
        unsafe {
            let idx = _mm512_set_epi64(index(D)+4,index(C0)+4,index(B)+4,index(A)+4,index(D),index(C0),index(B),index(A));
            Self(core::array::from_fn(|j| _mm512_permutex2var_epi64(a.0[j],idx,b.0[j])),PhantomData)
        }
    }
}
impl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED>{
    #[inline(always)]
    fn horizontal_sum_limb_table(&self)->Fp<C>{
        if !MIXED{return self.horizontal_sum();}
        // Eight normalized lanes below2p sum below16p. Each limb sum
        // below2^55 fits signed64; carry normalization fits five52bit limbs.
        let mut digits:[u64;5]=unsafe{self.0.map(|x|_mm512_reduce_add_epi64(x)as u64)};
        for j in 0..4{let carry=digits[j]>>52;digits[j]&=MASK;digits[j+1]+=carry;}
        let q=((digits[4]as u128*Self::RECIPROCAL as u128)>>52)as usize;
        debug_assert!(q<16);
        let multiples=const{
            let p=split(C::P);let mut table=[[0u64;5];16];let mut q=0;
            while q<16{let mut carry=0u64;let mut j=0;while j<5{let v=p[j]*q as u64+carry;table[q][j]=v&MASK;carry=v>>52;j+=1;}assert!(carry==0);q+=1;}table
        };
        let mut borrow=0i64;
        for j in 0..5{let v=digits[j]as i64-multiples[q][j]as i64+borrow;digits[j]=v as u64&MASK;borrow=v>>52;}
        debug_assert_eq!(borrow,0);debug_assert!(digits[4]<1<<48);
        let words=[digits[0]|digits[1]<<52, digits[1]>>12|digits[2]<<40, digits[2]>>24|digits[3]<<28, digits[3]>>36|digits[4]<<16];
        Fp::from_montgomery(crate::portable::reduce_once(words,C::P))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{FqConfig,FrConfig};
    fn check<C:FpConfig,const MIXED:bool>() {
        let mut seed=0xfedcba987654321u64;
        let mut random=|| {let mut bytes=[0;64];for chunk in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());} Fp::<C>::from_random_bytes(&bytes)};
        let edges=[Fp::<C>::ZERO,Fp::<C>::ONE,-Fp::<C>::ONE,Fp::<C>::from_u64(2),
            Fp::from_montgomery(crate::portable::sub4(C::P,[1,0,0,0]).0),
            Fp::from_montgomery(crate::portable::sub4(C::P,[2,0,0,0]).0),
            Fp::from_montgomery([u64::MAX,u64::MAX,u64::MAX,C::P[3]-1]),
            Fp::from_montgomery([u64::MAX,u64::MAX,u64::MAX,0])];
        let mut a=core::array::from_fn(|i| edges[i]);
        for round in 0..512 {
            let b=core::array::from_fn(|i|if round<edges.len(){edges[(round+i)%edges.len()]}else{random()});
            let (av,bv)=(Ifma8::<C,MIXED>::new(&a),Ifma8::<C,MIXED>::new(&b));
            assert_eq!(av.get(),a);
            assert_eq!(av.horizontal_sum(),a.into_iter().fold(Fp::<C>::ZERO,|acc,x|acc+x));
            let (lo,hi)=av.halves();
            assert_eq!(lo.get().as_slice(),&a[..4]);
            assert_eq!(hi.get().as_slice(),&a[4..]);
            assert_eq!(Ifma8::from_halves(&lo,&hi).get(),a);
            assert_eq!(Ifma8::shuffle_pair::<0,5,2,7>(&av,&bv).get(),[a[0],b[1],a[2],b[3],a[4],b[5],a[6],b[7]]);
            assert_eq!(av.mul(&bv).get(),core::array::from_fn(|i|a[i]*b[i]));
            assert_eq!(av.add(&bv).get(),core::array::from_fn(|i|a[i]+b[i]));
            assert_eq!(av.sub(&bv).get(),core::array::from_fn(|i|a[i]-b[i]));
            assert_eq!(av.add_canonical(&bv).get(),core::array::from_fn(|i|a[i]+b[i]));
            assert_eq!(av.sub_canonical(&bv).get(),core::array::from_fn(|i|a[i]-b[i]));
            assert_eq!(av.sum2(&bv,&bv,&av).get(),core::array::from_fn(|i|(a[i]*b[i]).double()));
            let (v,w)=(av.sub(&bv),bv.add(&av));
            assert_eq!(v.horizontal_sum(),core::array::from_fn::<_,8,_>(|i|a[i]-b[i]).into_iter().fold(Fp::<C>::ZERO,|acc,x|acc+x));
            assert_eq!(v.sum2(&w,&w,&v).get(),core::array::from_fn(|i|((a[i]-b[i])*(a[i]+b[i])).double()));
            assert_eq!(av.difference2(&av,&bv,&bv).get(),core::array::from_fn(|i|a[i]*a[i]-b[i]*b[i]));
            assert_eq!(v.difference2(&w,&w,&v).get(),[Fp::<C>::ZERO;8]);
            assert_eq!(av.mul(&Ifma8::one()).get(),a);
            assert_eq!(av.sub(&av).get(),[Fp::<C>::ZERO;8]);
            a=b;
        }
    }
    fn check_fused_reduction<C: FpConfig>() {
        let twice = crate::portable::add4(C::P, C::P).0;
        let four = crate::portable::add4(twice, twice).0;
        let high_boundary = [0, 0, 0, split(twice)[4] << 16];
        let boundaries = [
            [0; 4], C::P,
            crate::portable::sub4(twice, [1, 0, 0, 0]).0,
            twice, crate::portable::add4(twice, [1, 0, 0, 0]).0,
            crate::portable::sub4(four, [1, 0, 0, 0]).0,
            crate::portable::sub4(high_boundary, [1, 0, 0, 0]).0,
            high_boundary, crate::portable::add4(high_boundary, [1, 0, 0, 0]).0,
        ];
        let limit = split(twice);
        for round in 0..boundaries.len() * 2 {
            let values = core::array::from_fn(|i| boundaries[if round < boundaries.len() { round } else { (round + i) % boundaries.len() }]);
            let raw = Ifma8::<C, true>::raw(&values);
            let expected = Ifma8::<C, true>::reduce(raw.0, twice);
            let actual = Ifma8::<C, true>::reduce_fused(raw.0);
            let expected: [[u64; LANES]; 5] = expected.0.map(|x| unsafe { core::mem::transmute(x) });
            let actual: [[u64; LANES]; 5] = actual.0.map(|x| unsafe { core::mem::transmute(x) });
            assert_eq!(actual, expected);
            for lane in 0..LANES {
                for j in 0..5 { assert!(actual[j][lane] < 1 << 52); }
                assert!((0..5).rev().map(|j| actual[j][lane]).cmp((0..5).rev().map(|j| limit[j])).is_lt());
            }
        }
    }
    #[test]
    fn fused_reduction_boundaries() {
        check_fused_reduction::<FqConfig>();
        check_fused_reduction::<FrConfig>();
    }

    fn check_limb_horizontal<C:FpConfig,A:ark_ff::PrimeField>() {
        use ark_ff::Field;
        let inv_r=A::from(2u64).pow([256]).inverse().unwrap();
        let convert=|v:Fp<C>|A::from_le_bytes_mod_order(&v.0.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())*inv_r;
        let twice=crate::portable::add4(C::P,C::P).0;
        let edges=[[0;4],C::P,crate::portable::sub4(C::P,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0];
        let mut seed=0x193a73e2fa91u64;
        for round in 0..512 {
            let mut random=||{let mut bytes=[0u8;64];for b in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;b.copy_from_slice(&seed.to_le_bytes());}Fp::<C>::from_random_bytes(&bytes)};
            let a=core::array::from_fn(|i|if round<16{Fp::<C>::from_montgomery(edges[(round+i)%4])}else{random()});
            let b=core::array::from_fn(|i|if round<16{Fp::<C>::from_montgomery(edges[(round+2*i)%4])}else{random()});
            let aa=a.map(&convert);let bb=b.map(&convert);
            let av=Ifma8::<C,true>::new(&a);let bv=Ifma8::<C,true>::new(&b);
            for (value,want) in [(av,aa),(av.add(&bv),core::array::from_fn(|i|aa[i]+bb[i])),(av.sub(&bv),core::array::from_fn(|i|aa[i]-bb[i])),(av.mul(&bv),core::array::from_fn(|i|aa[i]*bb[i]))] {
                let expected:A=want.into_iter().sum();
                for got in [value.horizontal_sum_limb_table(),value.horizontal_sum()] {assert!(crate::portable::less(got.0,C::P));assert_eq!(convert(got),expected);}
            }
        }
    }
    #[test]
    fn limb_horizontal_sum_matches_arkworks() {
        check_limb_horizontal::<FqConfig,ark_ref::Fq>();
        check_limb_horizontal::<FrConfig,ark_ref::Fr>();
    }

    #[test]fn fq(){check::<FqConfig,false>();}
    #[test]fn fr(){check::<FrConfig,false>();}
    #[test]fn fq_mixed(){check::<FqConfig,true>();}
    #[test]fn fr_mixed(){check::<FrConfig,true>();}
}
'''
# Share the exact raw-storage reference test with the four-lane kernel.
complete = path.read_text()
for name in ["compact_storage_roundtrip", "signed_difference_scaled_matches_arkworks", "small_integer_multiply_matches_arkworks", "cios_carry_matches_arkworks", "complex_product_shared_matches_arkworks"]:
    start = complete.index("    #[test]\n    fn " + name + "()")
    body = complete.index("{", start)
    depth, end = 1, body + 1
    while depth:
        depth += (complete[end] == "{") - (complete[end] == "}")
        end += 1
    test = complete[start:end].replace("Ifma", "Ifma8")
    marker = "    fn check<C:FpConfig,const MIXED:bool>() {"
    assert source.count(marker) == 1
    source = source.replace(marker, test + "\n" + marker)
source = subprocess.check_output(["rustfmt", "--emit", "stdout", "--edition", "2021"], input=source, text=True)
output = path.parent / "ifma8.rs"
if "--check" in sys.argv:
    if output.read_text() != source:
        raise SystemExit("ifma8.rs is stale; run scripts/ifma8.py")
else:
    output.write_text(source)
