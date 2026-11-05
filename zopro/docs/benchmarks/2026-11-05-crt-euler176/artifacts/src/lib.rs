//! Exact reference for auxiliary-prime evaluation of the Euler derivative.
//! BigUint conversion/CRT is deliberately explicit; this is not production code.
use num_bigint::BigUint;
use num_traits::{One, ToPrimitive, Zero};
use tfhe_ntt::prime32::Plan;
use zopro_bn254::Fr;

pub fn integer(x:Fr)->BigUint {BigUint::new(x.to_canonical().into_iter().flat_map(|w|[w as u32,(w>>32)as u32]).collect())}
pub fn field(x:&BigUint)->Fr {
    let b=x.to_bytes_le();assert!(b.len()<=32);let mut padded=[0u8;32];padded[..b.len()].copy_from_slice(&b);
    Fr::from_canonical(std::array::from_fn(|i|u64::from_le_bytes(padded[8*i..8*i+8].try_into().unwrap()))).unwrap()
}
pub fn modulus()->BigUint {BigUint::new(Fr::MODULUS.into_iter().flat_map(|w|[w as u32,(w>>32)as u32]).collect())}
pub fn pow_mod(mut a:u64,mut e:u64,p:u64)->u64 {let mut z=1;while e!=0{if e&1!=0{z=z*a%p;}a=a*a%p;e>>=1;}z}
pub fn prime(q:u32)->bool {
    let n=q as u64;if n<2{return false;}for p in[2u64,3,5,7,11,13,17,19,23,29,31,37]{if n==p{return true;}if n%p==0{return false;}}
    let s=(n-1).trailing_zeros();let d=(n-1)>>s;
    for a in[2,7,61]{let mut y=pow_mod(a%n,d,n);if y==1||y==n-1{continue;}let mut passed=false;for _ in 1..s{y=y*y%n;if y==n-1{passed=true;break;}}if !passed{return false;}}
    true
}
pub fn coefficient_bound(n:usize)->BigUint {let r=modulus()-BigUint::one();r.pow(2)*n}
pub fn choose_primes(max_n:usize)->Vec<u32> {
    assert!(max_n.is_power_of_two()&&max_n<=32768);let bound=coefficient_bound(max_n)*2u32;
    let mut out=Vec::new();let mut product=BigUint::one();let mut k=((1u64<<31)-2)/(1<<16);
    while product<=bound {assert!(k>0);let q=(k*(1<<16)+1)as u32;k-=1;if prime(q){assert_eq!((q as usize-1)%(2*max_n),0);out.push(q);product*=q;}}
    assert!(out.len()>1);assert!((&product/ *out.last().unwrap())<=bound);out
}
pub fn kernel(n:usize)->Vec<Fr> {
    let omega=Fr::root_of_unity(n).unwrap();let mut powers=Fr::ONE;let mut den=Vec::with_capacity(n-1);
    for _ in 1..n {powers*=omega;den.push(powers-Fr::ONE);}
    let mut running=Fr::ONE;let prefixes=den.iter().map(|&d|{let v=running;running*=d;v}).collect::<Vec<_>>();
    let mut inverse=running.inverse().unwrap();let mut k=vec![Fr::ZERO;n];
    for i in (0..den.len()).rev(){k[i+1]=inverse*prefixes[i];inverse*=den[i];}
    k[0]=Fr::from((n-1)as u64)*Fr::from(2u64).inverse().unwrap();k
}
struct Image {q:u32,plan:Plan,kernel:Vec<u32>,prefix:BigUint,inverse_prefix:u32}
pub struct Context {n:usize,images:Vec<Image>,product:BigUint,half:BigUint,modulus:BigUint,twist:Vec<Fr>,restore:Vec<Fr>}
impl Context {
    pub fn new(n:usize,primes:&[u32])->Self {
        assert!(n>=32&&n.is_power_of_two());let r=modulus();let mut product=BigUint::one();
        for &q in primes{assert!(prime(q));assert_eq!((q as usize-1)%(2*n),0);product*=q;}
        assert!(product>coefficient_bound(n)*2u32,"insufficient exact signed CRT bound");
        let rho=Fr::root_of_unity(2*n).unwrap();assert_eq!(rho.pow(&[n as u64]),-Fr::ONE);
        let mut ti=Fr::ONE;let mut to=Fr::from(n as u64);let ri=rho.inverse().unwrap();
        let mut twist=Vec::with_capacity(n);let mut restore=Vec::with_capacity(n);
        for _ in 0..n{twist.push(ti);restore.push(to);ti*=ri;to*=rho;}
        let k=kernel(n).into_iter().zip(&twist).map(|(k,&t)|integer(k*t)).collect::<Vec<_>>();
        let mut prefix=BigUint::one();let mut images=Vec::new();
        for &q in primes {
            let plan=Plan::try_new(n,q).unwrap();
            let mut transformed=k.iter().map(|k|(k%q).to_u32().unwrap()).collect::<Vec<_>>();plan.fwd(&mut transformed);
            let inverse_prefix=pow_mod((&prefix%q).to_u64().unwrap(),q as u64-2,q as u64)as u32;
            images.push(Image{q,plan,kernel:transformed,prefix:prefix.clone(),inverse_prefix});prefix*=q;
        }
        assert_eq!(prefix,product);let half=&product/2u32;
        Self{n,images,product,half,modulus:r,twist,restore}
    }
    pub fn n(&self)->usize{self.n}
    pub fn prime_count(&self)->usize{self.images.len()}
    pub fn modulus_product_bits(&self)->u64{self.product.bits()}
    pub fn known_payloads(&self)->(usize,usize){(self.n*self.images.len()*4,self.n*2*32)}
    /// Returns n*E(A) at original domain points. All values, conversion and CRT
    /// are included in the operator, with a preconstructed public-domain context.
    pub fn derivative(&self,input:&[Fr])->Vec<Fr> {
        assert!(input.len()<=self.n);
        let lifted=(0..self.n).map(|j|integer(input.get(j).copied().unwrap_or(Fr::ZERO)*self.twist[j])).collect::<Vec<_>>();
        let residues=self.images.iter().map(|im|{
            let mut v=lifted.iter().map(|x|(x%im.q).to_u32().unwrap()).collect::<Vec<_>>();
            im.plan.fwd(&mut v);im.plan.mul_assign_normalize(&mut v,&im.kernel);im.plan.inv(&mut v);v
        }).collect::<Vec<_>>();
        (0..self.n).map(|j|{
            let mut x=BigUint::zero();
            for(im,v)in self.images.iter().zip(&residues){let old=(&x%im.q).to_u64().unwrap();let digit=((v[j]as u64+im.q as u64-old)*im.inverse_prefix as u64%im.q as u64)as u32;x+=&im.prefix*digit;}
            assert!(x<self.product);
            let value=if x>self.half{let magnitude=(&self.product-x)%&self.modulus;if magnitude.is_zero(){Fr::ZERO}else{-field(&magnitude)}}else{field(&(x%&self.modulus))};
            value*self.restore[j]
        }).collect()
    }
    pub fn product(&self,a:&[Fr],b:&[Fr])->Vec<Fr>{
        assert!(a.len()<=self.n&&b.len()<=self.n);let count=a.len().max(b.len());if count==0{return Vec::new();}
        let da=self.derivative(a);let db=self.derivative(b);
        (0..count).map(|i|da[i]*b.get(i).copied().unwrap_or(Fr::ZERO)+a.get(i).copied().unwrap_or(Fr::ZERO)*db[i]).collect()
    }
}

pub fn polynomial_values(coeff:&[Fr],n:usize)->(Vec<Fr>,Vec<Fr>){
    let omega=Fr::root_of_unity(n).unwrap();let mut x=Fr::ONE;let mut values=Vec::new();let mut derivatives=Vec::new();
    for _ in 0..n {let mut v=Fr::ZERO;let mut d=Fr::ZERO;for(j,&c)in coeff.iter().enumerate().rev(){v=v*x+c;d=d*x+c*Fr::from(j as u64);}values.push(v);derivatives.push(d);x*=omega;}(values,derivatives)
}
pub fn direct_derivative(values:&[Fr],n:usize)->Vec<Fr> {
    let omega=Fr::root_of_unity(n).unwrap();let inverse=omega.inverse().unwrap();let scale=Fr::from(n as u64).inverse().unwrap();
    let mut inverse_frequency=Fr::ONE;let mut coeff=Vec::new();
    for _ in 0..n{let mut weight=Fr::ONE;let mut sum=Fr::ZERO;for &v in values{sum+=v*weight;weight*=inverse_frequency;}coeff.push(sum*scale);inverse_frequency*=inverse;}
    polynomial_values(&coeff,n).1
}

#[cfg(test)]mod tests {
    use super::*;
    fn random(seed:&mut Fr)->Fr{*seed=seed.square()+Fr::from(17u64);*seed}
    #[test]fn circulant_kernel_matches_coefficient_euler_derivative(){
        let mut seed=Fr::from(71u64);
        for n in[2,4,8,16,32] {let k=kernel(n);let coeff=(0..n).map(|_|random(&mut seed)).collect::<Vec<_>>();let(a,expected)=polynomial_values(&coeff,n);let got=(0..n).map(|i|(0..n).fold(Fr::ZERO,|z,j|z+k[(i+n-j)%n]*a[j])).collect::<Vec<_>>();assert_eq!(got,expected);
            let rho=Fr::root_of_unity(2*n).unwrap();let mut t=Fr::ONE;let ri=rho.inverse().unwrap();let twist=(0..n).map(|_|{let v=t;t*=ri;v}).collect::<Vec<_>>();
            let twisted_a=a.iter().zip(&twist).map(|(&a,&t)|a*t).collect::<Vec<_>>();let twisted_k=k.iter().zip(&twist).map(|(&a,&t)|a*t).collect::<Vec<_>>();let mut conv=vec![Fr::ZERO;n];for i in 0..n{for j in 0..n{let v=twisted_a[i]*twisted_k[j];if i+j<n{conv[i+j]+=v}else{conv[i+j-n]-=v}}}let mut restore=Fr::ONE;for(i,v)in conv.into_iter().enumerate(){assert_eq!(v*restore,expected[i]);restore*=rho;}
        }
    }
    #[test]fn auxiliary_convolutions_match_exact_signed_integer_coefficients(){
        let primes=choose_primes(32768);assert!(primes.len()>1);
        for &q in&primes {let n=32;let plan=Plan::try_new(n,q).unwrap();let a=(0..n).map(|i|(q-1)-i as u32).collect::<Vec<_>>();let b=(0..n).map(|i|((i*i+37)%61)as u32).collect::<Vec<_>>();let mut expected=vec![0i128;n];for i in 0..n{for j in 0..n{let v=a[i]as i128*b[j]as i128;if i+j<n{expected[i+j]+=v}else{expected[i+j-n]-=v}}}assert!(expected.iter().any(|&x|x<0));let mut aa=a;let mut bb=b;plan.fwd(&mut aa);plan.fwd(&mut bb);plan.mul_assign_normalize(&mut aa,&bb);plan.inv(&mut aa);for (a,b)in aa.iter().zip(expected){assert_eq!(*a as i128,b.rem_euclid(q as i128));}}
    }
    #[test]fn crt_euler_product_handles_full_fields_partial_arrays_and_boundaries(){
        let primes=choose_primes(32768);let mut seed=Fr::from(119u64);
        for n in[32,64,128] {let ctx=Context::new(n,&primes);for kind in 0..6{let mut a=(0..n).map(|i|match kind{0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>if i%2==0{Fr::ONE}else{-Fr::ONE},_=>random(&mut seed)}).collect::<Vec<_>>();let mut b=(0..n).map(|_|random(&mut seed)).collect::<Vec<_>>();if kind==4{a.truncate(n/2+1);b.truncate(n-3);}if kind==5{a.clear();b.clear();}let da=direct_derivative(&a,n);let db=direct_derivative(&b,n);let expected=(0..a.len().max(b.len())).map(|i|(da[i]*b.get(i).copied().unwrap_or(Fr::ZERO)+a.get(i).copied().unwrap_or(Fr::ZERO)*db[i])*Fr::from(n as u64)).collect::<Vec<_>>();assert_eq!(ctx.product(&a,&b),expected);}}
        assert!(std::panic::catch_unwind(||Context::new(32,&primes[..1])).is_err());
    }
}
