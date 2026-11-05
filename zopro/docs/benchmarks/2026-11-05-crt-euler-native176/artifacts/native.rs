//! Eight-lane integer boundary kernels for the whole Euler convolution.
use super::*;
use core::arch::x86_64::*;
use rayon::prelude::*;
use zopro_bn254::{ifma8::Ifma8,FrConfig};
const MASK:u64=(1u64<<52)-1;
pub fn available()->bool{is_x86_feature_detected!("avx512f")&&is_x86_feature_detected!("avx512dq")&&is_x86_feature_detected!("avx512vl")&&is_x86_feature_detected!("avx512ifma")}
#[inline(always)]unsafe fn splat(x:u64)->__m512i{_mm512_set1_epi64(x as i64)}
fn split(w:[u64;4])->[u64;5]{[w[0]&MASK,((w[0]>>52)|(w[1]<<12))&MASK,((w[1]>>40)|(w[2]<<24))&MASK,((w[2]>>28)|(w[3]<<36))&MASK,w[3]>>16]}

impl Projection {
    /// Requires AVX512F/DQ/VL and eight initialized input words. No input mutates.
    #[inline(never)]
    pub unsafe fn project8(&self,input:&[[u64;4];8])->[u32;8]{
        let ptr=input.as_ptr().cast::<__m512i>();
        let(r0,r1,r2,r3)=(_mm512_loadu_si512(ptr),_mm512_loadu_si512(ptr.add(1)),_mm512_loadu_si512(ptr.add(2)),_mm512_loadu_si512(ptr.add(3)));
        let(a,b,c,d)=(_mm512_unpacklo_epi64(r0,r1),_mm512_unpackhi_epi64(r0,r1),_mm512_unpacklo_epi64(r2,r3),_mm512_unpackhi_epi64(r2,r3));
        let low=_mm512_set_epi64(13,9,12,8,5,1,4,0);let high=_mm512_set_epi64(15,11,14,10,7,3,6,2);
        let words=[_mm512_permutex2var_epi64(a,low,c),_mm512_permutex2var_epi64(b,low,d),_mm512_permutex2var_epi64(a,high,c),_mm512_permutex2var_epi64(b,high,d)];
        let mut total=_mm512_setzero_si512();let mut quotient=total;let mask32=splat(u32::MAX as u64);
        for j in 0..8{let chunk=if j%2==0{_mm512_and_si512(words[j/2],mask32)}else{_mm512_srli_epi64::<32>(words[j/2])};total=_mm512_add_epi64(total,_mm512_mullo_epi64(chunk,splat(self.w[j])));quotient=_mm512_add_epi64(quotient,_mm512_srli_epi64::<32>(_mm512_mullo_epi64(chunk,splat(self.u[j]))));}
        let q=splat(self.q as u64);let residual=_mm512_sub_epi64(total,_mm512_mullo_epi64(quotient,q));
        debug_assert_eq!(_mm512_cmpge_epu64_mask(residual,splat(16*self.q as u64)),0);
        let correction=_mm512_srli_epi64::<48>(_mm512_mullo_epi64(residual,splat(self.v)));let residual=_mm512_sub_epi64(residual,_mm512_mullo_epi64(correction,q));
        debug_assert_eq!(_mm512_cmpge_epu64_mask(residual,splat(2*self.q as u64)),0);
        let result=_mm512_mask_sub_epi64(residual,_mm512_cmpge_epu64_mask(residual,q),residual,q);let mut out=[0u32;8];_mm256_storeu_si256(out.as_mut_ptr().cast(),_mm512_cvtepi64_epi32(result));out
    }
}

pub struct NativeCrt {scalar:DirectCrt,coefficients:[[u64;5];18],mu:[u64;18],inv_shoup:[u64;17],p:[u64;5],high_reciprocal:u64}
impl NativeCrt {
    pub fn new(qs:&[u32],n:usize)->Self{
        assert_eq!(qs.len(),17);let scalar=DirectCrt::new(qs,n);let r=modulus();
        let values=scalar.weights.iter().copied().chain([scalar.negative_m]).collect::<Vec<_>>();
        let coefficients=std::array::from_fn(|i|split(values[i].0));
        let mu=std::array::from_fn(|i|{let x=BigUint::new(values[i].0.into_iter().flat_map(|w|[w as u32,(w>>32)as u32]).collect());let m=((x<<31usize)/&r).to_u64().unwrap();assert!(m<1<<31);m});
        let inv_shoup=std::array::from_fn(|i|((scalar.inv[i]as u64)<<32)/qs[i]as u64);let p=split(Fr::MODULUS);let high_reciprocal=(1u64<<52)/(p[4]+1);
        assert!(p[4]>=1u64<<45&&p[4]<1u64<<46);assert!(36*(p[4]as u128+1)*p[4]as u128+37*(1u128<<52)<(1u128<<52)*p[4]as u128);
        Self{scalar,coefficients,mu,inv_shoup,p,high_reciprocal}
    }
    /// Requires AVX512F/DQ/VL/IFMA and each residue below its own prime.
    /// The result is the scalar direct-CRT formula; its true signed-integer
    /// interpretation additionally requires the parent convolution bound.
    #[inline(never)]
    pub unsafe fn recover8(&self,residues:&[[u32;8];17])->[Fr;8]{
        let zero=_mm512_setzero_si512();let mask=splat(MASK);let mut t=[zero;6];let mut quotient=zero;let mut fixed=zero;
        for i in 0..18{
            let factor=if i<17{
                let input=_mm512_cvtepu32_epi64(_mm256_loadu_si256(residues[i].as_ptr().cast()));let q=splat(self.scalar.qs[i]as u64);
                debug_assert_eq!(_mm512_cmpge_epu64_mask(input,q),0);
                let estimate=_mm512_srli_epi64::<32>(_mm512_mullo_epi64(input,splat(self.inv_shoup[i])));
                let raw=_mm512_sub_epi64(_mm512_mullo_epi64(input,splat(self.scalar.inv[i]as u64)),_mm512_mullo_epi64(estimate,q));
                debug_assert_eq!(_mm512_cmpge_epu64_mask(raw,splat(2*self.scalar.qs[i]as u64)),0);
                let factor=_mm512_mask_sub_epi64(raw,_mm512_cmpge_epu64_mask(raw,q),raw,q);
                fixed=_mm512_add_epi64(fixed,_mm512_srli_epi64::<31>(_mm512_mullo_epi64(factor,splat(self.scalar.mu[i]))));factor
            }else{_mm512_srli_epi64::<32>(_mm512_add_epi64(fixed,splat(1u64<<31)))};
            quotient=_mm512_add_epi64(quotient,_mm512_srli_epi64::<31>(_mm512_mullo_epi64(factor,splat(self.mu[i]))));
            for j in 0..5{let c=splat(self.coefficients[i][j]);t[j]=_mm512_madd52lo_epu64(t[j],factor,c);t[j+1]=_mm512_madd52hi_epu64(t[j+1],factor,c);}
        }
        for j in 0..5{let p=splat(self.p[j]);t[j]=_mm512_sub_epi64(t[j],_mm512_madd52lo_epu64(zero,quotient,p));t[j+1]=_mm512_sub_epi64(t[j+1],_mm512_madd52hi_epu64(zero,quotient,p));}
        for j in 0..5{let carry=_mm512_srai_epi64::<52>(t[j]);t[j]=_mm512_and_si512(t[j],mask);t[j+1]=_mm512_add_epi64(t[j+1],carry);}
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(t[5],zero),0);
        let correction=_mm512_madd52hi_epu64(zero,t[4],splat(self.high_reciprocal));
        for j in 0..5{let p=splat(self.p[j]);t[j]=_mm512_sub_epi64(t[j],_mm512_madd52lo_epu64(zero,correction,p));t[j+1]=_mm512_sub_epi64(t[j+1],_mm512_madd52hi_epu64(zero,correction,p));}
        for j in 0..5{let carry=_mm512_srai_epi64::<52>(t[j]);t[j]=_mm512_and_si512(t[j],mask);t[j+1]=_mm512_add_epi64(t[j+1],carry);}
        debug_assert_eq!(_mm512_cmpneq_epi64_mask(t[5],zero),0);
        let mut d=[zero;5];let mut borrow=zero;for j in 0..5{let v=_mm512_add_epi64(_mm512_sub_epi64(t[j],splat(self.p[j])),borrow);d[j]=_mm512_and_si512(v,mask);borrow=_mm512_srai_epi64::<52>(v);}
        let keep=_mm512_cmpneq_epi64_mask(borrow,zero);let r: [_;5]=std::array::from_fn(|j|_mm512_mask_blend_epi64(keep,d[j],t[j]));
        let w0=_mm512_or_si512(r[0],_mm512_slli_epi64::<52>(r[1]));let w1=_mm512_or_si512(_mm512_srli_epi64::<12>(r[1]),_mm512_slli_epi64::<40>(r[2]));let w2=_mm512_or_si512(_mm512_srli_epi64::<24>(r[2]),_mm512_slli_epi64::<28>(r[3]));let w3=_mm512_or_si512(_mm512_srli_epi64::<36>(r[3]),_mm512_slli_epi64::<16>(r[4]));
        let(a,b,c,d)=(_mm512_unpacklo_epi64(w0,w1),_mm512_unpackhi_epi64(w0,w1),_mm512_unpacklo_epi64(w2,w3),_mm512_unpackhi_epi64(w2,w3));let low=_mm512_set_epi64(11,10,3,2,9,8,1,0);let high=_mm512_set_epi64(15,14,7,6,13,12,5,4);
        let el=_mm512_permutex2var_epi64(a,low,c);let eh=_mm512_permutex2var_epi64(a,high,c);let ol=_mm512_permutex2var_epi64(b,low,d);let oh=_mm512_permutex2var_epi64(b,high,d);
        let mut raw=[[0u64;4];8];let ptr=raw.as_mut_ptr().cast::<__m512i>();_mm512_storeu_si512(ptr,_mm512_shuffle_i64x2::<0x44>(el,ol));_mm512_storeu_si512(ptr.add(1),_mm512_shuffle_i64x2::<0xee>(el,ol));_mm512_storeu_si512(ptr.add(2),_mm512_shuffle_i64x2::<0x44>(eh,oh));_mm512_storeu_si512(ptr.add(3),_mm512_shuffle_i64x2::<0xee>(eh,oh));
        debug_assert!(raw.iter().all(|w|w.iter().rev().cmp(Fr::MODULUS.iter().rev()).is_lt()));raw.map(Fr::from_montgomery)
    }
}

pub struct NativeContext {base:Context,projections:Vec<Projection>,crt:NativeCrt}
impl NativeContext {
    pub fn new(n:usize,qs:&[u32])->Self{assert!(available());let mut base=Context::new(n,qs);let crt=NativeCrt::new(qs,n);for v in&mut base.restore{*v*=crt.scalar.r_inverse;}let projections=qs.iter().map(|&q|Projection::new(q)).collect();Self{base,projections,crt}}
    pub fn n(&self)->usize{self.base.n}
    pub fn derivative(&self,input:&[Fr])->Vec<Fr>{
        let n=self.base.n;assert!(input.len()<=n);let mut words=vec![[0u64;4];n];
        words.par_chunks_mut(8).enumerate().for_each(|(j,out)|{let k=8*j;let a=Ifma8::<FrConfig,true>::new(&std::array::from_fn(|i|input.get(k+i).copied().unwrap_or(Fr::ZERO)));let b=Ifma8::<FrConfig,true>::new(&self.base.twist[k..k+8].try_into().unwrap());let result=a.mul(&b).get();for i in 0..8{out[i]=result[i].0;}});
        let residues=self.base.images.par_iter().zip(&self.projections).map(|(image,projection)|{let mut r=vec![0u32;n];for(k,block)in words.chunks_exact(8).enumerate(){let row:&[[u64;4];8]=block.try_into().unwrap();let v=unsafe{projection.project8(row)};r[8*k..8*k+8].copy_from_slice(&v);}image.plan.fwd(&mut r);image.plan.mul_assign_normalize(&mut r,&image.kernel);image.plan.inv(&mut r);r}).collect::<Vec<_>>();
        drop(words);let mut out=vec![Fr::ZERO;n];out.par_chunks_mut(8).enumerate().for_each(|(j,out)|{let k=8*j;let rows:[[u32;8];17]=std::array::from_fn(|p|residues[p][k..k+8].try_into().unwrap());let values=unsafe{self.crt.recover8(&rows)};let a=Ifma8::<FrConfig,true>::new(&values);let b=Ifma8::<FrConfig,true>::new(&self.base.restore[k..k+8].try_into().unwrap());out.copy_from_slice(&a.mul(&b).get());});out
    }
    pub fn product(&self,a:Vec<Fr>,b:Vec<Fr>)->Vec<Fr>{
        let n=self.base.n;assert!(a.len()<=n&&b.len()<=n);let count=a.len().max(b.len());if count==0{return a;}
        let da=self.derivative(&a);let db=self.derivative(&b);let mut out=vec![Fr::ZERO;count];out.par_chunks_mut(8).enumerate().for_each(|(j,out)|{let k=8*j;let aa=Ifma8::<FrConfig,true>::new(&std::array::from_fn(|i|a.get(k+i).copied().unwrap_or(Fr::ZERO)));let bb=Ifma8::<FrConfig,true>::new(&std::array::from_fn(|i|b.get(k+i).copied().unwrap_or(Fr::ZERO)));let ad=Ifma8::<FrConfig,true>::new(&std::array::from_fn(|i|da.get(k+i).copied().unwrap_or(Fr::ZERO)));let bd=Ifma8::<FrConfig,true>::new(&std::array::from_fn(|i|db.get(k+i).copied().unwrap_or(Fr::ZERO)));out.copy_from_slice(&ad.sum2(&bb,&aa,&bd).get()[..out.len()]);});out
    }
}

#[cfg(test)]mod tests{
    use super::*;use num_bigint::{BigInt,Sign};
    fn word(w:[u64;4])->BigUint{BigUint::new(w.into_iter().flat_map(|w|[w as u32,(w>>32)as u32]).collect())}
    #[test]fn native_projection_all_primes_lanes_boundaries_and_random_words(){
        assert!(available());let mut seed=0x6a09e667f3bcc909u64;let qs=choose_primes(32768);let mut cases=vec![[0;4],[u64::MAX;4],Fr::MODULUS];for bit in 0..256{let mut w=[0u64;4];w[bit/64]=1u64<<(bit%64);cases.push(w);}for _ in 0..4096{cases.push(std::array::from_fn(|_|{seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;seed}));}while cases.len()%8!=0{cases.push([u64::MAX;4]);}
        for q in qs{let p=Projection::new(q);for row in cases.chunks_exact(8){let row:&[[u64;4];8]=row.try_into().unwrap();let got=unsafe{p.project8(row)};for i in 0..8{assert_eq!(got[i],(word(row[i])%q).to_u32().unwrap());assert_eq!(got[i],p.project(row[i]));}}}
    }
    #[test]fn native_shared_dot_matches_scalar_and_bounded_integer_coefficients(){
        assert!(available());let qs=choose_primes(32768);let native=NativeCrt::new(&qs,32768);let r=BigInt::from(modulus());let c=coefficient_bound(32768);let mut cases=vec![BigInt::zero(),BigInt::one(),-BigInt::one(),BigInt::from(c.clone()),-BigInt::from(c.clone()),&r-1,-&r+1];let mut seed=BigUint::from(0xd1b54a32d192ed03u64);for i in 0..4096{seed=(seed*6364136223846793005u64+1442695040888963407u64)%(&c+BigUint::one());let x=BigInt::from(seed.clone());cases.push(if i%2==0{x}else{-x});}while cases.len()%8!=0{cases.push(BigInt::zero());}
        for values in cases.chunks_exact(8){let residues:[[u32;8];17]=std::array::from_fn(|i|std::array::from_fn(|j|{let q=BigInt::from(qs[i]);((&values[j]%&q+&q)%q).to_u32().unwrap()}));let got=unsafe{native.recover8(&residues)};for j in 0..8{let row=residues.iter().map(|r|r[j]).collect::<Vec<_>>();assert_eq!(got[j],native.scalar.recover(&row).0);let expected=(&values[j]%&r+&r)%&r;assert_ne!(expected.sign(),Sign::Minus);assert_eq!(integer(got[j]),expected.to_biguint().unwrap());}}
        // Arbitrary residue vectors qualify the fixed dot/selector implementation,
        // not the bounded signed-integer interpretation of those vectors.
        let mut state=0x94d049bb133111ebu64;for mode in 0..1024{let rows=std::array::from_fn(|i|std::array::from_fn(|j|{state^=state<<13;state^=state>>7;state^=state<<17;match mode%4{0=>0,1=>qs[i]-1,2=>if j%2==0{0}else{qs[i]-1},_=>(state%qs[i]as u64)as u32}}));let got=unsafe{native.recover8(&rows)};for j in 0..8{assert_eq!(got[j],native.scalar.recover(&rows.iter().map(|r|r[j]).collect::<Vec<_>>()).0);}}
    }
    #[test]fn native_complete_operator_matches_both_exact_oracles_and_thread_counts(){
        assert!(available());let qs=choose_primes(32768);let mut seed=Fr::from(307u64);
        for n in[32,64,128]{let native=NativeContext::new(n,&qs);let original=Context::new(n,&qs);for family in 0..4{let mut a=(0..n).map(|i|{seed=seed.square()+Fr::from(17u64);if family==0{Fr::ZERO}else if family==1&&i%2==0{-Fr::ONE}else{seed}}).collect::<Vec<_>>();let mut b=(0..n).map(|_|{seed=seed.square()+Fr::from(19u64);seed}).collect::<Vec<_>>();if family==2{a.truncate(n/2+1);b.truncate(n-3);}if family==3{a.clear();b.clear();}let expected=original.product(&a,&b);let direct=original.product_direct_reference(&a,&b);assert_eq!(expected,direct);for threads in[1,4]{let pool=rayon::ThreadPoolBuilder::new().num_threads(threads).stack_size(512*1024).build().unwrap();let got=pool.install(||native.product(a.clone(),b.clone()));assert_eq!(got,expected);}}}
    }
}
