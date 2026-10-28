use num_bigint::{BigInt, Sign};
use num_traits::{One, ToPrimitive, Zero};

pub const D: u64 = 2520;
pub const N: usize = 5;
pub fn rem(a: BigInt, m: &BigInt) -> BigInt { let r=a%m; if r.sign()==Sign::Minus {r+m}else{r} }
pub fn gcd(mut a: BigInt, mut b: BigInt) -> BigInt {while !b.is_zero(){(a,b)=(b.clone(),a%b);}a}
pub fn inverse(a: &BigInt, m: &BigInt) -> BigInt {
    let (mut x,mut y)=(m.clone(),rem(a.clone(),m));let(mut s,mut t)=(BigInt::zero(),BigInt::one());
    while !y.is_zero(){let q=&x/&y;(x,y)=(y.clone(),x-&q*y);(s,t)=(t.clone(),s-q*t);}
    assert_eq!(x,BigInt::one());rem(s,m)
}
fn urem(a:&BigInt,m:u64)->u64{rem(a.clone(),&m.into()).to_u64().unwrap()}
pub fn mul_mod(a:u64,b:u64,m:u64)->u64{((a as u128*b as u128)%m as u128)as u64}
fn sub_mod(a:u64,b:u64,m:u64)->u64{((a as u128+m as u128-b as u128)%m as u128)as u64}

pub struct Basis {
    pub moduli:[u64;N],
    pub product:BigInt,
    pub cofactors:[BigInt;N],
    pub inverse_cofactor:[u64;N],
    pub start:usize,
}
impl Basis {
    pub fn new(bits:usize,start:usize)->Self{
        let b=1u64<<bits;let moduli=std::array::from_fn(|i|b-(3+D*(i+start)as u64));
        let product=moduli.iter().fold(BigInt::one(),|a,m|a*m);
        let cofactors=std::array::from_fn(|i|&product/moduli[i]);
        let inverse_cofactor=std::array::from_fn(|i|inverse(&cofactors[i],&moduli[i].into()).to_u64().unwrap());
        Self{moduli,product,cofactors,inverse_cofactor,start}
    }
    pub fn residues(&self,x:&BigInt)->[u64;N]{std::array::from_fn(|i|urem(x,self.moduli[i]))}
    pub fn weighted(&self,r:&[u64;N])->[u64;N]{std::array::from_fn(|i|{assert!(r[i]<self.moduli[i]);mul_mod(r[i],self.inverse_cofactor[i],self.moduli[i])})}
    pub fn sum(&self,weighted:&[u64;N])->BigInt{(0..N).fold(BigInt::zero(),|a,i|a+weighted[i]*&self.cofactors[i])}
    pub fn reconstruct(&self,r:&[u64;N])->BigInt{rem(self.sum(&self.weighted(r)),&self.product)}
}

/// Ordinary degree-four finite differences, using exact integer samples.
/// Inputs are the preweighted CRT coefficients, not arbitrary field residues.
pub fn polynomial(coefficients:[i64;N],start:usize,forward:bool)->[i64;N]{
    let factorial=[1i64,1,2,6,24];
    let mut row: [i64;N]=std::array::from_fn(|i|{
        let c=factorial[i]*factorial[N-1-i];let sign=if (N-1-i)%2==0{1}else{-1};
        coefficients[i].checked_mul(c*sign).unwrap()
    });
    let mut edge=[0i64;N];edge[0]=row[if forward{N-1}else{0}];
    for k in 1..N{for i in 0..N-k{row[i]=row[i+1].checked_sub(row[i]).unwrap();}edge[k]=row[if forward{N-1-k}else{0}];}
    let mut out=[0;N];
    for o in &mut out{for k in (0..N-1).rev(){edge[k]=if forward{edge[k].checked_add(edge[k+1]).unwrap()}else{edge[k].checked_sub(edge[k+1]).unwrap()};}*o=edge[0];}
    if !forward{out.reverse();}assert_eq!(start,if forward{0}else{5});out
}
pub fn structured_sum(weighted:&[u64;N],source:&Basis,target:&Basis)->[u64;N]{
    let mask=(1u64<<26)-1;let lo=polynomial(weighted.map(|a|(a&mask)as i64),source.start,source.start==0);
    let hi=polynomial(weighted.map(|a|(a>>26)as i64),source.start,source.start==0);
    let d4=BigInt::from(D).pow(4);
    std::array::from_fn(|j|{
        assert!(lo[j]>=0&&hi[j]>=0);
        let value=(lo[j]as u128)+((hi[j]as u128)<<26);
        mul_mod((value%target.moduli[j]as u128)as u64,urem(&d4,target.moduli[j]),target.moduli[j])
    })
}

/// For common ceil(2^64/m)=4097, compute the top64 bits of4097*sum
/// using only64bit shifts/addition and a carry; no u128 multiply needed.
pub fn common_quotient(sum:u64)->u64{
    let base=1u64<<52;assert!(sum<5*base);
    let lo=(sum&(base-1))<<12;let(_,carry)=lo.overflowing_add(sum);
    (sum>>52)+u64::from(carry)
}

#[derive(Clone,Debug)]pub struct State{pub left:[u64;N],pub right:[u64;N]}
pub struct Rns {
    pub p:BigInt,pub left:Basis,pub right:Basis,pub bits:usize,
    pub alpha_factor:[u64;N],pub m_inverse_right:[u64;N],pub m_right:[u64;N],pub n_left:[u64;N],
    pub p_right:[u64;N],pub difference_offset_left:[u64;N],pub difference_offset_right:[u64;N],
    pub reciprocal_up:[u64;N],pub largest_q_low:u64,pub largest_q_up:u64,
}
impl Rns {
    pub fn new(p:BigInt,bits:usize)->Self{
        let left=Basis::new(bits,0);let right=Basis::new(bits,5);let b=BigInt::one()<<bits;
        let all:Vec<_>=left.moduli.iter().chain(&right.moduli).copied().collect();
        for i in 0..all.len(){assert!(gcd(p.clone(),all[i].into()).is_one());for j in 0..i{assert!(gcd(all[i].into(),all[j].into()).is_one());}}
        assert!(left.product>64u32*&p);assert!(right.product>8u32*&p);
        let csum:u64=(0..N).map(|i|3+D*i as u64).sum();assert!(BigInt::from(csum)<b);
        let sum_moduli:u128=right.moduli.iter().map(|m|*m as u128).sum();
        //4p/N+sum(n_j)/2^64<1, checked with exact cross multiplication.
        assert!((4u32*&p<<64usize)+BigInt::from(sum_moduli)*&right.product<(&right.product<<64usize));
        let alpha_factor=std::array::from_fn(|i|urem(&(-inverse(&p,&left.moduli[i].into())*left.inverse_cofactor[i]),left.moduli[i]));
        let m_inverse_right=std::array::from_fn(|j|inverse(&left.product,&right.moduli[j].into()).to_u64().unwrap());
        let m_right=right.residues(&left.product);let n_left=left.residues(&right.product);let p_right=right.residues(&p);
        let offset=64u32*&p*&p;let difference_offset_left=left.residues(&offset);let difference_offset_right=right.residues(&offset);
        let reciprocal_up=std::array::from_fn(|j|((1u128<<64)+right.moduli[j]as u128-1)/right.moduli[j]as u128).map(|n|n as u64);
        if bits==52{assert_eq!(reciprocal_up,[4097;N]);}
        Self{p,left,right,bits,alpha_factor,m_inverse_right,m_right,n_left,p_right,difference_offset_left,difference_offset_right,reciprocal_up,largest_q_low:0,largest_q_up:0}
    }
    pub fn state(&self,x:&BigInt)->State{assert!(x>=&BigInt::zero()&&x< &(8u32*&self.p));State{left:self.left.residues(x),right:self.right.residues(x)}}
    pub fn encode(&self,x:&BigInt)->State{self.state(&rem(x*&self.left.product,&self.p))}
    pub fn integer(&self,x:&State)->BigInt{let v=self.right.reconstruct(&x.right);assert_eq!(self.left.residues(&v),x.left);v}
    pub fn decode(&self,x:&State)->BigInt{rem(self.integer(x)*inverse(&self.left.product,&self.p),&self.p)}
    pub fn apply(&mut self,a:&State,b:&State,c:&State,d:&State,mode:u8)->State{
        assert!(mode<=2);
        let numerators=|mods:&[u64;N],x:&[u64;N],y:&[u64;N],z:&[u64;N],w:&[u64;N],off:&[u64;N]|{
            std::array::from_fn(|i|{let p=mul_mod(x[i],y[i],mods[i]);let q=mul_mod(z[i],w[i],mods[i]);match mode{0=>p,1=>((p as u128+q as u128)%mods[i]as u128)as u64,2=>((sub_mod(p,q,mods[i])as u128+off[i]as u128)%mods[i]as u128)as u64,_=>unreachable!()}})
        };
        let tl=numerators(&self.left.moduli,&a.left,&b.left,&c.left,&d.left,&self.difference_offset_left);
        let tr=numerators(&self.right.moduli,&a.right,&b.right,&c.right,&d.right,&self.difference_offset_right);
        let alpha=std::array::from_fn(|i|mul_mod(tl[i],self.alpha_factor[i],self.left.moduli[i]));
        let k=alpha.iter().sum::<u64>()>>self.bits;self.largest_q_low=self.largest_q_low.max(k);assert!(k<5);
        let s=structured_sum(&alpha,&self.left,&self.right);
        let right=std::array::from_fn(|j|{
            let q=sub_mod(s[j],mul_mod(k,self.m_right[j],self.right.moduli[j]),self.right.moduli[j]);
            let z=((tr[j]as u128+mul_mod(q,self.p_right[j],self.right.moduli[j])as u128)%self.right.moduli[j]as u128)as u64;
            mul_mod(z,self.m_inverse_right[j],self.right.moduli[j])
        });
        let beta=self.right.weighted(&right);let raw:u128=(0..N).map(|j|beta[j]as u128*self.reciprocal_up[j]as u128).sum();let q=(raw>>64)as u64;
        if self.bits==52{assert_eq!(q,common_quotient(beta.iter().sum()));}assert!(q<5);self.largest_q_up=self.largest_q_up.max(q);
        let reverse=structured_sum(&beta,&self.right,&self.left);
        let left=std::array::from_fn(|i|sub_mod(reverse[i],mul_mod(q,self.n_left[i],self.left.moduli[i]),self.left.moduli[i]));
        State{left,right}
    }
}

pub fn polynomial_l1_bound()->u64{
    let mut peak=0;for forward in [true,false]{let mut coefficients=[[0i64;N];N];for i in 0..N{coefficients[i][i]=1;}
        // Propagate the five independent input unit vectors through the
        // linear map; an independent conservative bound is established
        // below for every difference/extrapolation intermediate.
        for c in coefficients{let out=polynomial(c,if forward{0}else{5},forward);for v in out{peak=peak.max(v.unsigned_abs());}}
    }
    // Difference table<=16*24;5 extrapolation steps are bounded by
    // repeating four triangular additions, at most2^20 times that bound.
    // This deliberately loose bound still fits signed64 for26bit inputs.
    let bound=384u64<<20;assert!((bound as u128*((1u128<<26)-1))<(1u128<<63));assert!(peak<=bound);bound
}

#[cfg(test)]mod tests{
    use super::*;
    #[test]fn structured_crt_both_directions_matches_direct_sum(){
        let a=Basis::new(52,0);let b=Basis::new(52,5);let cases=[[0;5],[1,2,3,4,5],a.moduli.map(|n|n-1),[(1<<26)-1;5]];
        for(source,target)in [(&a,&b),(&b,&a)]{for mut weights in cases{for i in 0..N{weights[i]%=source.moduli[i];}let want=target.residues(&source.sum(&weights));assert_eq!(structured_sum(&weights,source,target),want);}}
        polynomial_l1_bound();
    }
    #[test]fn common_quotient_handles_carry_boundaries(){
        let b=1u64<<52;for hi in 0..5u64{for lo in [0,1,b-1,b-2,b/2,(b>>12)-1]{let s=hi*b+lo;if s<5*b{assert_eq!(common_quotient(s),((s as u128*4097)>>64)as u64);}}}
    }
    #[test]fn small_prime_fused_products_and_signed_offset(){
        let mut r=Rns::new(101u32.into(),20);let v=[0u32,1,2,50,99,100];
        for a in v{for b in v{for mode in 0..3{let x=r.encode(&a.into());let y=r.encode(&b.into());let z=r.apply(&x,&y,&x,&x,mode);let want=match mode{0=>(a*b)%101,1=>(a*b+a*a)%101,_=>(a*b+10100-a*a)%101};assert_eq!(r.decode(&z),BigInt::from(want));assert!(r.integer(&z)<404u32.into());}}}
    }
}
