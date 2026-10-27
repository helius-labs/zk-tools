use num_bigint::BigInt;
use num_traits::{One, Signed, ToPrimitive, Zero};

#[derive(Clone, Debug)]
pub struct Z { pub re: BigInt, pub im: BigInt }

impl Z {
    pub fn new(re: impl Into<BigInt>, im: impl Into<BigInt>) -> Self { Self { re:re.into(), im:im.into() } }
    pub fn mul(&self,b:&Self)->Self {Self::new(&self.re*&b.re-&self.im*&b.im,&self.re*&b.im+&self.im*&b.re)}
    pub fn add(&self,b:&Self)->Self {Self::new(&self.re+&b.re,&self.im+&b.im)}
    pub fn rot(&self)->Self {Self::new(-&self.im,self.re.clone())}
}

pub struct Params {
    pub a:BigInt, pub b:BigInt, pub prime:BigInt, pub radix:BigInt,
    pub half:BigInt, pub bound:BigInt, pub neg_inv:Z,
    pub i:BigInt, pub radix_inverse:BigInt,
}
pub fn modulo(x:BigInt,m:&BigInt)->BigInt {let r=x%m;if r.is_negative(){r+m}else{r}}
pub fn inverse(x:&BigInt,m:&BigInt)->BigInt {
    let(mut a,mut b)=(m.clone(),modulo(x.clone(),m));let(mut s,mut t)=(BigInt::zero(),BigInt::one());
    while !b.is_zero(){let q=&a/&b;(a,b)=(b.clone(),a-&q*&b);(s,t)=(t.clone(),s-q*t);}
    assert_eq!(a,BigInt::one());modulo(s,m)
}
fn round_div(x:BigInt,d:&BigInt)->BigInt {if x.is_negative(){-round_div(-x,d)}else{(x+d/2u32)/d}}

impl Params {
    pub fn new()->Self {
        let u=BigInt::from(4965661367192848881u64);
        let a=6u32*&u*&u+3u32*&u;let b=3u32*&u+1u32;
        let prime=BigInt::parse_bytes(b"21888242871839275222246405745257275088548364400416034343698204186575808495617",10).unwrap();
        assert_eq!(&a*&a+&b*&b,prime);
        let radix=BigInt::one()<<128usize;let half=&a/2u32;let bound=&half+&b;
        let k=inverse(&prime,&radix);
        let neg_inv=Z::new(modulo(-&a*&k,&radix),modulo(&b*&k,&radix));
        let i=modulo(-&a*inverse(&b,&prime),&prime);
        let radix_inverse=inverse(&radix,&prime);
        assert_eq!(modulo(&i*&i,&prime),&prime-1u32);
        let n=Z::new(a.clone(),b.clone());let product=neg_inv.mul(&n);
        assert_eq!(modulo(product.re+1u32,&radix),BigInt::zero());assert_eq!(modulo(product.im,&radix),BigInt::zero());
        let max=(2u32*&bound*&bound+&radix-1u32)/&radix+(&a+&b+1u32)/2u32;
        assert!(&max+&b<a);assert!(max<BigInt::one()<<127usize);assert!(&bound*2u32<BigInt::one()<<127usize);
        Self{a,b,prime,radix,half,bound,neg_inv,i,radix_inverse}
    }
    pub fn reduce(&self,z:&Z)->Z {
        let q0=round_div(&z.re*&self.a+&z.im*&self.b,&self.prime);
        let q1=round_div(&z.im*&self.a-&z.re*&self.b,&self.prime);
        let out=Z::new(&z.re-&q0*&self.a+&q1*&self.b,&z.im-&q0*&self.b-&q1*&self.a);
        assert!(out.re.abs()<=self.bound&&out.im.abs()<=self.bound);out
    }
    pub fn balance(&self,mut z:Z)->Z {
        if z.re>self.half {z.re-=&self.a;z.im-=&self.b;}else if z.re< -&self.half {z.re+=&self.a;z.im+=&self.b;}
        if z.im>self.half {z.re+=&self.b;z.im-=&self.a;}else if z.im< -&self.half {z.re-=&self.b;z.im+=&self.a;}
        assert!(z.re.abs()<=self.bound&&z.im.abs()<=self.bound,"{z:?}");z
    }
    pub fn center(&self,x:BigInt)->BigInt {modulo(x+&self.radix/2u32,&self.radix)-&self.radix/2u32}
    pub fn redc(&self,t:&Z)->Z {
        let m=t.mul(&self.neg_inv);let m=Z::new(self.center(m.re),self.center(m.im));
        let s=t.add(&m.mul(&Z::new(self.a.clone(),self.b.clone())));
        assert!(modulo(s.re.clone(),&self.radix).is_zero()&&modulo(s.im.clone(),&self.radix).is_zero());
        let z=Z::new(s.re/&self.radix,s.im/&self.radix);
        assert!(z.re.to_i128().is_some()&&z.im.to_i128().is_some());self.balance(z)
    }
    pub fn encode(&self,x:&BigInt)->Z {self.reduce(&Z::new(modulo(x*&self.radix,&self.prime),0))}
    pub fn decode(&self,z:&Z)->BigInt {modulo((&z.re+&z.im*&self.i)*&self.radix_inverse,&self.prime)}
}

#[cfg(test)]mod tests {
 use super::*;
 #[test]fn modulus_constants_and_closure_inequalities(){let p=Params::new();assert!(p.a.to_i128().is_some());assert!(p.bound.to_i128().is_some());}
 #[test]fn all_coordinate_corners_and_gaussian_rotation(){let p=Params::new();let vals=vec![BigInt::zero(),BigInt::one(),-BigInt::one(),p.half.clone(),-&p.half,p.bound.clone(),-&p.bound];let zs:Vec<_>=vals.iter().flat_map(|x|vals.iter().map(move|y|Z::new(x.clone(),y.clone()))).collect();for a in &zs{assert_eq!(p.decode(&a.rot()),modulo(p.decode(a)*&p.i,&p.prime));for b in &zs{let got=p.redc(&a.mul(b));assert_eq!(p.decode(&got),modulo(p.decode(a)*p.decode(b),&p.prime));let sum=p.balance(a.add(b));assert_eq!(p.decode(&sum),modulo(p.decode(a)+p.decode(b),&p.prime));}}}
 #[test]fn deterministic_fullwidth_chains_and_host_import(){let p=Params::new();let mut seed=0x6a09e667f3bcc909u64;let mut x=BigInt::one();let mut z=p.encode(&x);for _ in 0..4096{let mut bytes=[0u8;32];for chunk in bytes.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;chunk.copy_from_slice(&seed.to_le_bytes());}let y=modulo(BigInt::from_bytes_le(num_bigint::Sign::Plus,&bytes),&p.prime);let g=p.encode(&y);z=p.redc(&z.mul(&g));x=modulo(x*&y,&p.prime);assert_eq!(p.decode(&z),x);let host=modulo(&y*&p.radix*&p.radix,&p.prime);let imported=p.redc(&Z::new(host,0));assert_eq!(p.decode(&imported),y);}}
}
