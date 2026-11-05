use super::*;

pub struct Projection {q:u32,w:[u64;8],u:[u64;8],v:u64}
impl Projection {
    pub fn new(q:u32)->Self {
        assert!(q>1<<30&&q<1<<31);let mut power=1u64;
        let w=std::array::from_fn(|_|{let out=power;power=power*((1u64<<32)%q as u64)%q as u64;out});
        let u=w.map(|w|(w<<32)/q as u64);Self{q,w,u,v:(1u64<<48)/q as u64}
    }
    pub fn project(&self,words:[u64;4])->u32 {
        let mut low=0u64;let mut quotient=0u64;
        for i in 0..8{let c=(words[i/2]>>(32*(i%2)))as u32 as u64;low=low.wrapping_add(c*self.w[i]);quotient+=(c*self.u[i])>>32;}
        let residual=low.wrapping_sub(quotient.wrapping_mul(self.q as u64));assert!(residual<16*self.q as u64);
        let correction=(residual*self.v)>>48;let residual=residual-correction*self.q as u64;assert!(residual<2*self.q as u64);
        (if residual>=self.q as u64{residual-self.q as u64}else{residual})as u32
    }
}
pub struct DirectCrt {qs:Vec<u32>,inv:Vec<u32>,mu:Vec<u64>,weights:Vec<Fr>,negative_m:Fr,r_inverse:Fr}
impl DirectCrt {
    pub fn new(qs:&[u32],n:usize)->Self {
        let m=qs.iter().fold(BigUint::one(),|m,&q|m*q);assert!(m>coefficient_bound(n)*16u32);let r=modulus();
        let mut inv=Vec::new();let mut mu=Vec::new();let mut weights=Vec::new();
        for &q in qs{let mi=&m/q;inv.push(pow_mod((&mi%q).to_u64().unwrap(),q as u64-2,q as u64)as u32);mu.push((1u64<<63)/q as u64);weights.push(field(&(mi%&r)));}
        Self{qs:qs.to_vec(),inv,mu,weights,negative_m:-field(&(&m%&r)),r_inverse:Fr::from_canonical(Fr::ONE.0).unwrap().inverse().unwrap()}
    }
    pub fn recover(&self,residues:&[u32])->(Fr,u64) {
        assert_eq!(residues.len(),self.qs.len());let mut fixed=0u64;let mut sum=Fr::ZERO;
        for i in 0..residues.len(){assert!(residues[i]<self.qs[i]);let t=residues[i]as u64*self.inv[i]as u64%self.qs[i]as u64;fixed+=(t*self.mu[i])>>31;sum+=self.weights[i]*Fr::from(t);}
        let correction=(fixed+(1u64<<31))>>32;assert!(correction<=self.qs.len()as u64);sum+=self.negative_m*Fr::from(correction);(sum,correction)
    }
}
impl Context {
    pub fn derivative_direct_reference(&self,input:&[Fr])->Vec<Fr>{
        assert!(input.len()<=self.n);let qs=self.images.iter().map(|im|im.q).collect::<Vec<_>>();let crt=DirectCrt::new(&qs,self.n);
        let words=(0..self.n).map(|j|(input.get(j).copied().unwrap_or(Fr::ZERO)*self.twist[j]).0).collect::<Vec<_>>();
        let residues=self.images.iter().map(|im|{let projection=Projection::new(im.q);let mut v=words.iter().map(|&w|projection.project(w)).collect::<Vec<_>>();im.plan.fwd(&mut v);im.plan.mul_assign_normalize(&mut v,&im.kernel);im.plan.inv(&mut v);v}).collect::<Vec<_>>();
        (0..self.n).map(|j|{let row=residues.iter().map(|r|r[j]).collect::<Vec<_>>();crt.recover(&row).0*(self.restore[j]*crt.r_inverse)}).collect()
    }
    pub fn product_direct_reference(&self,a:&[Fr],b:&[Fr])->Vec<Fr>{
        assert!(a.len()<=self.n&&b.len()<=self.n);let count=a.len().max(b.len());if count==0{return Vec::new();}let da=self.derivative_direct_reference(a);let db=self.derivative_direct_reference(b);(0..count).map(|i|da[i]*b.get(i).copied().unwrap_or(Fr::ZERO)+a.get(i).copied().unwrap_or(Fr::ZERO)*db[i]).collect()
    }
}
#[cfg(test)]mod tests {
    use super::*;use num_bigint::{BigInt,Sign};
    fn bigint_word(w:[u64;4])->BigUint{BigUint::new(w.into_iter().flat_map(|w|[w as u32,(w>>32)as u32]).collect())}
    #[test]fn integer_projection_matches_biguint_word_and_radix_boundaries(){
        let qs=choose_primes(32768);let mut state=0x6a09e667f3bcc908u64;
        for q in qs{let projection=Projection::new(q);let mut cases=vec![[0;4],[u64::MAX;4],Fr::MODULUS,[q as u64,0,0,0]];
            for bit in 0..256{let mut w=[0;4];w[bit/64]=1u64<<(bit%64);cases.push(w);if bit%64!=0{w[0]=w[0].wrapping_sub(1);cases.push(w);}}
            for _ in 0..4096{cases.push(std::array::from_fn(|_|{state^=state<<13;state^=state>>7;state^=state<<17;state}));}
            for w in cases{assert_eq!(projection.project(w),(&bigint_word(w)%q).to_u32().unwrap());}
        }
    }
    #[test]fn fixed_integer_CRT_correction_recovers_signed_coefficients(){
        let qs=choose_primes(32768);let crt=DirectCrt::new(&qs,32768);let m=qs.iter().fold(BigUint::one(),|m,&q|m*q);let c=coefficient_bound(32768);let r=BigInt::from(modulus());
        let mut cases=vec![BigInt::zero(),BigInt::one(),-BigInt::one(),BigInt::from(c.clone()),-BigInt::from(c.clone()),&r-1,-&r+1];
        let mut seed=BigUint::from(0x9e3779b97f4a7c15u64);for i in 0..1024{seed=(seed*6364136223846793005u64+1442695040888963407u64)%(&c+BigUint::one());let v=BigInt::from(seed.clone());cases.push(if i%2==0{v}else{-v});}
        for v in cases{let residues=qs.iter().map(|&q|{let p=BigInt::from(q);((&v%&p+&p)%p).to_u32().unwrap()}).collect::<Vec<_>>();let(got,correction)=crt.recover(&residues);
            let mut total=BigUint::zero();for(i,&q)in qs.iter().enumerate(){let mi=&m/q;let t=residues[i]as u64*crt.inv[i]as u64%q as u64;total+=mi*t;}
            assert_eq!(BigInt::from(total)-BigInt::from(m.clone())*BigInt::from(correction),v);
            let expected=(&v%&r+&r)%&r;assert_ne!(expected.sign(),Sign::Minus);assert_eq!(integer(got),expected.to_biguint().unwrap());
        }
    }
    #[test]fn raw_Montgomery_import_and_direct_CRT_match_both_operator_oracles(){
        let qs=choose_primes(32768);let mut seed=Fr::from(239u64);
        for n in[32,64]{let context=Context::new(n,&qs);for family in 0..4{
            let a_coeff=(0..n).map(|i|{seed=seed.square()+Fr::from(17u64);if family==0{Fr::ZERO}else if family==1&&i%2==0{-Fr::ONE}else{seed}}).collect::<Vec<_>>();let b_coeff=(0..n).map(|_|{seed=seed.square()+Fr::from(19u64);seed}).collect::<Vec<_>>();
            let(mut a,da)=polynomial_values(&a_coeff,n);let(mut b,db)=polynomial_values(&b_coeff,n);
            if family>=2{a.truncate(if family==2{n/2+1}else{0});b.truncate(if family==2{n-1}else{0});}
            let reference=context.product(&a,&b);let got=context.product_direct_reference(&a,&b);assert_eq!(got,reference);
            if family<2{let expected=(0..n).map(|i|(da[i]*b[i]+a[i]*db[i])*Fr::from(n as u64)).collect::<Vec<_>>();assert_eq!(got,expected);}
        }}
    }
}

#[path="native.rs"]pub mod native;
