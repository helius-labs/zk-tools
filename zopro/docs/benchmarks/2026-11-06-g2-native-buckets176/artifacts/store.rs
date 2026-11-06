//! Private experiment: canonical radix-52 Fq scratch, never a key-point table.
use crate::{Fq, FqConfig, FpConfig, ifma8::Ifma8};
use core::{arch::x86_64::*, marker::PhantomData};
pub type Vector = Ifma8<FqConfig, true>;
const MASK:u64=(1<<52)-1;
const fn split(w:[u64;4])->[u64;5] {
    [w[0]&MASK,((w[0]>>52)|(w[1]<<12))&MASK,((w[1]>>40)|(w[2]<<24))&MASK,((w[2]>>28)|(w[3]<<36))&MASK,w[3]>>16]
}
#[inline(always)]
fn canonical(value:&Vector)->Vector {
    // SAFETY: this module requires AVX-512. IFMA outputs are normalized <2p;
    // one radix-52 subtraction and masked selection yield canonical values.
    unsafe {
        let p=split(FqConfig::P);let mut difference=[_mm512_setzero_si512();5];let mut borrow=_mm512_setzero_si512();let mask=_mm512_set1_epi64(MASK as i64);
        for j in 0..5 {let v=_mm512_add_epi64(_mm512_sub_epi64(value.0[j],_mm512_set1_epi64(p[j]as i64)),borrow);difference[j]=_mm512_and_si512(v,mask);borrow=_mm512_srai_epi64::<52>(v);}
        let keep=_mm512_cmpneq_epi64_mask(borrow,_mm512_setzero_si512());
        Ifma8(core::array::from_fn(|j|_mm512_mask_blend_epi64(keep,difference[j],value.0[j])),PhantomData)
    }
}
pub struct Store(Vec<[u64;5]>);
impl Store {
    pub fn new(len:usize)->Self {Self(vec![[0;5];len])}
    #[inline(always)]
    pub fn set(&mut self,index:usize,value:Fq) {self.0[index]=split(crate::portable::reduce_once(value.0,FqConfig::P));}
    #[inline(always)]
    pub fn get(&self,index:usize)->Fq {
        let d=self.0[index];Fq::from_montgomery([d[0]|d[1]<<52,d[1]>>12|d[2]<<40,d[2]>>24|d[3]<<28,d[3]>>36|d[4]<<16])
    }
    #[inline(always)]
    pub fn gather(&self,indices:[u32;8])->Vector {
        assert!(indices.iter().all(|&i|(i as usize)<self.0.len()));
        // SAFETY: each index names an initialized five-u64 element; Vec's
        // allocation fits isize, so all scaled offsets stay inside it.
        unsafe {
            let offsets=_mm512_set_epi64(indices[7]as i64*5,indices[6]as i64*5,indices[5]as i64*5,indices[4]as i64*5,indices[3]as i64*5,indices[2]as i64*5,indices[1]as i64*5,indices[0]as i64*5);
            let base=self.0.as_ptr().cast::<i64>();
            Ifma8(core::array::from_fn(|j|_mm512_i64gather_epi64::<8>(_mm512_add_epi64(offsets,_mm512_set1_epi64(j as i64)),base)),PhantomData)
        }
    }
    #[inline(always)]
    pub fn scatter(&mut self,indices:[u32;8],value:&Vector,active:u8) {
        assert!(indices.iter().all(|&i|(i as usize)<self.0.len()));
        let value=canonical(value);
        // SAFETY: bounds are checked for every lane and every limb. Masked
        // lanes do not write. Duplicate indices have unspecified lane order;
        // the MSM's busy mask provides distinct active indices per batch.
        unsafe {
            let offsets=_mm512_set_epi64(indices[7]as i64*5,indices[6]as i64*5,indices[5]as i64*5,indices[4]as i64*5,indices[3]as i64*5,indices[2]as i64*5,indices[1]as i64*5,indices[0]as i64*5);
            let base=self.0.as_mut_ptr().cast::<i64>();
            for j in 0..5 {_mm512_mask_i64scatter_epi64::<8>(base,active,_mm512_add_epi64(offsets,_mm512_set1_epi64(j as i64)),value.0[j]);}
        }
    }
}
#[inline(always)]
pub fn zero_mask(value:&Vector)->u8 {
    // Values are normalized <2p. The two zero representations are 0 and p.
    unsafe {let p=split(FqConfig::P);let mut zero=255u8;let mut modulus=255u8;for j in 0..5 {zero&=_mm512_cmpeq_epi64_mask(value.0[j],_mm512_setzero_si512());modulus&=_mm512_cmpeq_epi64_mask(value.0[j],_mm512_set1_epi64(p[j]as i64));}zero|modulus}
}
#[inline(always)]
pub fn replace_with_one(value:&Vector,mask:u8)->Vector {
    unsafe {let one=Vector::one();Ifma8(core::array::from_fn(|j|_mm512_mask_blend_epi64(mask,value.0[j],one.0[j])),PhantomData)}
}
#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::{PrimeField,Field};
    #[test]
    fn storage_masks_and_lazy_values_match_arkworks() {
        let inv_r=ark_ref::Fq::from(2u64).pow([256]).inverse().unwrap();
        let convert=|v:Fq|ark_ref::Fq::from_le_bytes_mod_order(&v.0.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>())*inv_r;
        let p=FqConfig::P;let twice=crate::portable::add4(p,p).0;
        let edges=[[0;4],p,crate::portable::sub4(p,[1,0,0,0]).0,crate::portable::sub4(twice,[1,0,0,0]).0];
        let mut seed=0x812380981u64;let mut store=Store::new(17);let mut expected=[ark_ref::Fq::from(0u64);17];let indices=[8,1,15,3,12,7,4,10];
        for round in 0..512 {
            let mut random=||{let mut b=[0u8;64];for chunk in b.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}Fq::from_random_bytes(&b)};
            let a=core::array::from_fn(|i|if round<16 {Fq::from_montgomery(edges[(round+i)%4])}else{random()});
            let b=core::array::from_fn(|i|if round<16 {Fq::from_montgomery(edges[(round+2*i)%4])}else{random()});
            let av=Vector::new(&a);let bv=Vector::new(&b);
            let zero=zero_mask(&av);for i in 0..8 {assert_eq!(zero>>i&1!=0,convert(a[i])==ark_ref::Fq::from(0u64));}
            assert_eq!(replace_with_one(&av,zero).get().map(&convert),core::array::from_fn(|i|if zero>>i&1!=0{ark_ref::Fq::from(1u64)}else{convert(a[i])}));
            for (value,want) in [(av,a.map(&convert)),(av.sub(&bv),core::array::from_fn(|i|convert(a[i])-convert(b[i]))),(av.mul(&bv),core::array::from_fn(|i|convert(a[i])*convert(b[i])))] {
                let active=[0x55u8,0xaau8,0xffu8][round%3];store.scatter(indices,&value,active);
                for i in 0..8 {if active>>i&1!=0 {expected[indices[i]as usize]=want[i];}}
                let got=store.gather(indices).get();for i in 0..8 {assert!(crate::portable::less(got[i].0,p));assert_eq!(convert(got[i]),expected[indices[i]as usize]);assert_eq!(store.get(indices[i]as usize),got[i]);}
            }
            store.set(16,a[0]);assert_eq!(convert(store.get(16)),convert(a[0]));
        }
    }
    #[test]
    #[should_panic]
    fn gather_rejects_out_of_bounds_before_memory_access() {Store::new(7).gather([7;8]);}
}
