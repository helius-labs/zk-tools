use num_bigint::{BigInt, Sign};
use num_traits::{One, ToPrimitive, Zero};
pub const B:u64=1<<52;
pub const MASK:u64=B-1;
pub const N:usize=5;
pub fn rem(x:BigInt,m:&BigInt)->BigInt{let r=x%m;if r.sign()==Sign::Minus{r+m}else{r}}
pub fn inverse(x:&BigInt,m:&BigInt)->BigInt{let(mut a,mut b)=(m.clone(),rem(x.clone(),m));let(mut s,mut t)=(BigInt::zero(),BigInt::one());while !b.is_zero(){let q=&a/&b;(a,b)=(b.clone(),a-&q*b);(s,t)=(t.clone(),s-q*t);}assert!(a.is_one());rem(s,m)}
pub fn gcd(mut a:u64,mut b:u64)->u64{while b!=0{(a,b)=(b,a%b);}a}
pub fn inv_word(a:u64,m:u64)->u64{let(mut x,mut y)=(m as i128,(a%m)as i128);let(mut s,mut t)=(0i128,1i128);while y!=0{let q=x/y;(x,y)=(y,x-q*y);(s,t)=(t,s-q*t);}assert_eq!(x,1);s.rem_euclid(m as i128)as u64}
pub fn naf(mut x:i64)->Vec<(u32,i64)>{let mut out=Vec::new();let mut e=0;while x!=0{if x&1!=0{let d=2-x.rem_euclid(4);out.push((e,d));x-=d;}x/=2;e+=1;}out}
pub fn weight(x:i64)->u32{naf(x).len()as u32}
pub fn inverse_weight(a:u64,m:u64)->u32{let k=inv_word(a,m)as i64;weight(k).min(weight(k-m as i64))}

#[derive(Clone,Copy,Default,Debug,PartialEq,Eq)]pub struct Cost{pub ifma:u64,pub mullo:u64,pub add:u64,pub sub:u64,pub shift:u64,pub mask:u64,pub compare:u64,pub masked_sub:u64,pub select:u64,pub sparse_calls:u64,pub shoup_calls:u64}
impl Cost{pub fn total(&self)->u64{self.ifma+self.mullo+self.add+self.sub+self.shift+self.mask+self.compare+self.masked_sub+self.select}pub fn merge(&mut self,b:Self){self.ifma+=b.ifma;self.mullo+=b.mullo;self.add+=b.add;self.sub+=b.sub;self.shift+=b.shift;self.mask+=b.mask;self.compare+=b.compare;self.masked_sub+=b.masked_sub;self.select+=b.select;self.sparse_calls+=b.sparse_calls;self.shoup_calls+=b.shoup_calls;}}

// Split an exact positive constant product into radix-52 words, using
// signed NAF terms and explicit signed64 partial accumulators.
pub fn split_sparse(x:u64,k:u64,c:&mut Cost)->(u64,u64){
    assert!(x<B&&k<=B);if k==0{return(0,0)}if k==1{return(x,0)}
    let(mut lo,mut hi)=(0i64,0i64);for(e,s)in naf(k as i64){
        if e==0{lo+=s*x as i64;c.add+=1;}
        else if e==52{hi+=s*x as i64;c.add+=1;}
        else{assert!(e<52);lo+=s*((x<<e)&MASK)as i64;hi+=s*(x>>(52-e))as i64;c.shift+=2;c.mask+=1;c.add+=2;}
        assert!(lo.unsigned_abs()<1<<59&&hi.unsigned_abs()<1<<59);
    }
    hi+=lo>>52;lo&=MASK as i64;c.shift+=1;c.add+=1;c.mask+=1;
    assert!(hi>=0&&hi<B as i64);(lo as u64,hi as u64)
}
fn small_sparse(x:u64,k:u64,c:&mut Cost)->u64{assert!(x<=k&&k<1<<26);if k==0{return 0}if k==1{return x}let mut v=0i64;for(e,s)in naf(k as i64){v+=s*(x<<e)as i64;c.add+=1;if e!=0{c.shift+=1;}}assert!(v>=0&&v<B as i64);v as u64}
pub fn fold(lo:u64,hi:u64,m:u64,cost:&mut Cost)->u64{
    assert!(lo<B&&hi<B);let c=B-m;assert!(c<(1<<26)&&c*c+2*c<B);if c==0{return lo}
    let(l,h)=split_sparse(hi,c,cost);let sum=lo+l;let high=h+(sum>>52);let low=sum&MASK;cost.add+=2;cost.shift+=1;cost.mask+=1;assert!(high<=c);
    let mut v=low+small_sparse(high,c,cost);cost.add+=1;assert!(v<2*m);cost.compare+=1;cost.masked_sub+=1;if v>=m{v-=m}v
}
pub fn variable(x:u64,y:u64,m:u64,cost:&mut Cost)->u64{assert!(x<m&&y<m);let v=x as u128*y as u128;if m==B{cost.ifma+=1;return(v as u64)&MASK}cost.ifma+=2;fold(v as u64&MASK,(v>>52)as u64,m,cost)}
fn sparse_positive(x:u64,k:u64,m:u64,c:&mut Cost)->u64{let(lo,hi)=split_sparse(x,k,c);fold(lo,hi,m,c)}
pub fn sparse(x:u64,k:u64,m:u64,c:&mut Cost)->u64{
    assert!(x<m&&k<m);if k==0{return 0}if k==1{return x}
    let mut a=Cost::default();let pos=sparse_positive(x,k,m,&mut a);
    let mut b=Cost::default();let q=sparse_positive(x,m-k,m,&mut b);b.compare+=1;b.sub+=1;b.select+=1;let neg=if q==0{0}else{m-q};assert_eq!(pos,neg);
    let(v,mut cc)=if b.total()<a.total(){(neg,b)}else{(pos,a)};cc.sparse_calls+=1;c.merge(cc);v
}
pub fn shoup(x:u64,k:u64,m:u64,c:&mut Cost)->u64{
    assert!(x<m&&k<m);if k==0{return 0}if k==1{return x}
    if m==B{c.mullo+=1;c.mask+=1;c.shoup_calls+=1;return x.wrapping_mul(k)&MASK}
    let factor=((k as u128*B as u128)/m as u128)as u64;assert!(factor<B);
    let q=((x as u128*factor as u128)>>52)as u64;
    let mut r=x.wrapping_mul(k).wrapping_sub(q.wrapping_mul(m));assert!(r<2*m);
    c.ifma+=1;c.mullo+=2;c.sub+=1;c.compare+=1;c.masked_sub+=1;c.shoup_calls+=1;if r>=m{r-=m}r
}
pub fn constant(x:u64,k:u64,m:u64,hybrid:bool,c:&mut Cost)->u64{
    let mut a=Cost::default();let av=sparse(x,k,m,&mut a);if !hybrid{c.merge(a);return av}
    let mut b=Cost::default();let bv=shoup(x,k,m,&mut b);assert_eq!(av,bv);let(v,cc)=if b.total()<a.total(){(bv,b)}else{(av,a)};c.merge(cc);v
}

pub struct Basis{pub moduli:[u64;N],pub product:BigInt,pub cofactor:[BigInt;N],pub ci:[u64;N],pub inverses:[[u64;N];N]}
impl Basis{pub fn new(moduli:[u64;N])->Self{let product=moduli.iter().fold(BigInt::one(),|a,m|a*m);let cofactor=std::array::from_fn(|i|&product/moduli[i]);let ci=std::array::from_fn(|i|inverse(&cofactor[i],&moduli[i].into()).to_u64().unwrap());let mut inverses=[[0;N];N];for i in 0..N{for j in i+1..N{inverses[i][j]=inv_word(moduli[i],moduli[j]);}}Self{moduli,product,cofactor,ci,inverses}}
pub fn residues(&self,x:&BigInt)->[u64;N]{self.moduli.map(|m|rem(x.clone(),&m.into()).to_u64().unwrap())}
pub fn reconstruct(&self,x:&[u64;N])->BigInt{rem((0..N).fold(BigInt::zero(),|a,i|a+BigInt::from(x[i])*self.ci[i]*&self.cofactor[i]),&self.product)}
pub fn digits(&self,x:&[u64;N],hybrid:bool,c:&mut Cost)->[u64;N]{let mut a=*x;for i in 0..N{assert!(a[i]<self.moduli[i]);for j in i+1..N{let m=self.moduli[j];assert!(self.moduli[i]<2*m);let mut v=a[j]+2*m-a[i];assert!(v<3*m);c.add+=1;c.sub+=1;for _ in 0..2{c.compare+=1;c.masked_sub+=1;if v>=m{v-=m}}
    a[j]=constant(v,self.inverses[i][j],m,hybrid,c);}}a}
pub fn convert(&self,x:&[u64;N],target:&Basis,hybrid:bool,c:&mut Cost)->[u64;N]{let a=self.digits(x,hybrid,c);std::array::from_fn(|j|{let m=target.moduli[j];let mut y=a[N-1];assert!(y<2*m);c.compare+=1;c.masked_sub+=1;if y>=m{y-=m}
    for i in(0..N-1).rev(){y=constant(y,self.moduli[i]%m,m,hybrid,c);y+=a[i];assert!(y<3*m);c.add+=1;for _ in 0..2{c.compare+=1;c.masked_sub+=1;if y>=m{y-=m}}}y})}
}

#[derive(Clone,Debug)]pub struct State{pub left:[u64;N],pub right:[u64;N]}
pub struct Rns{pub p:BigInt,pub left:Basis,pub right:Basis,pub negative_inverse:[u64;N],pub p_right:[u64;N],pub m_inverse:[u64;N],pub offset_left:[u64;N],pub offset_right:[u64;N]}
impl Rns{pub fn new(p:BigInt,l:[u64;N],r:[u64;N])->Self{let left=Basis::new(l);let right=Basis::new(r);let all:Vec<_>=l.into_iter().chain(r).collect();for i in 0..all.len(){assert!(rem(p.clone(),&all[i].into())!=BigInt::zero());for j in 0..i{assert_eq!(gcd(all[i],all[j]),1);}}assert!(left.product>64u32*&p&&right.product>8u32*&p);
    let negative_inverse=l.map(|m|rem(-inverse(&p,&m.into()),&m.into()).to_u64().unwrap());let p_right=right.residues(&p);let m_inverse=r.map(|m|inverse(&left.product,&m.into()).to_u64().unwrap());let offset=64u32*&p*&p;let offset_left=left.residues(&offset);let offset_right=right.residues(&offset);
    Self{p,left,right,negative_inverse,p_right,m_inverse,offset_left,offset_right}}
pub fn state(&self,x:&BigInt)->State{assert!(x>=&BigInt::zero()&&x<&(8u32*&self.p));State{left:self.left.residues(x),right:self.right.residues(x)}}
pub fn integer(&self,s:&State)->BigInt{let v=self.right.reconstruct(&s.right);assert_eq!(self.left.residues(&v),s.left);v}
pub fn apply(&self,a:&State,b:&State,c:&State,d:&State,mode:u8,hybrid:bool,cost:&mut Cost)->State{assert!(mode<3);
    let numerator=|x:&[u64;N],y:&[u64;N],z:&[u64;N],w:&[u64;N],mods:&[u64;N],offset:&[u64;N],cost:&mut Cost|->[u64;N]{std::array::from_fn(|i|{let m=mods[i];let mut v=variable(x[i],y[i],m,cost);if mode!=0{let u=variable(z[i],w[i],m,cost);if mode==1{v+=u;cost.add+=1;}else{v+=m-u;cost.add+=1;cost.sub+=1;}cost.compare+=1;cost.masked_sub+=1;if v>=m{v-=m}if mode==2{v+=offset[i];cost.add+=1;cost.compare+=1;cost.masked_sub+=1;if v>=m{v-=m}}}v})};
    let tl=numerator(&a.left,&b.left,&c.left,&d.left,&self.left.moduli,&self.offset_left,cost);let tr=numerator(&a.right,&b.right,&c.right,&d.right,&self.right.moduli,&self.offset_right,cost);
    let ql=std::array::from_fn(|i|constant(tl[i],self.negative_inverse[i],self.left.moduli[i],hybrid,cost));let qr=self.left.convert(&ql,&self.right,hybrid,cost);
    let right=std::array::from_fn(|j|{let m=self.right.moduli[j];let mut v=tr[j]+constant(qr[j],self.p_right[j],m,hybrid,cost);cost.add+=1;cost.compare+=1;cost.masked_sub+=1;if v>=m{v-=m}constant(v,self.m_inverse[j],m,hybrid,cost)});
    let left=self.right.convert(&right,&self.left,hybrid,cost);State{left,right}
}
}

pub fn optimal_order(edge:[[u32;N];N],wc:[u32;N])->(u32,[usize;N]){let mut dp=[u32::MAX;1<<N];let mut path=[[0;N];1<<N];dp[0]=0;for mask in 0usize..1<<N{if dp[mask]==u32::MAX{continue}let len=mask.count_ones()as usize;for i in 0..N{if mask>>i&1!=0{continue}let mut cost=dp[mask];for j in 0..N{if mask>>j&1!=0{cost+=edge[j][i]+2*wc[i]+4;}}if len<N-1{cost+=5*wc[i];}let next=mask|1<<i;if cost<dp[next]{dp[next]=cost;path[next]=path[mask];path[next][len]=i;}}}let target:u32=wc.iter().map(|w|4*(2*w+2)).sum();(dp[(1<<N)-1]+target,path[(1<<N)-1])}
#[derive(Clone)]pub struct ScoredBasis{pub mask:u64,pub compatible:u64,pub score:u32,pub ids:[usize;N]}
pub struct Family{pub offsets:Vec<u64>,pub moduli:Vec<u64>,pub edges:Vec<Vec<u32>>,pub neighbors:Vec<u64>,pub wc:Vec<u32>}
impl Family{pub fn new()->Self{let mut offsets=vec![0,1];for t in 1..=25{offsets.extend([(1u64<<t)-1,(1u64<<t)+1]);}offsets.sort_unstable();offsets.dedup();let moduli:Vec<_>=offsets.iter().map(|c|B-c).collect();assert!(moduli.len()<64);let count=moduli.len();let mut edges=vec![vec![0;count];count];let mut neighbors=vec![0;count];for i in 0..count{for j in 0..count{if i!=j&&gcd(moduli[i],moduli[j])==1{neighbors[i]|=1<<j;edges[i][j]=inverse_weight(moduli[i],moduli[j]);}}}let wc=offsets.iter().map(|c|weight(*c as i64)).collect();Self{offsets,moduli,edges,neighbors,wc}}
pub fn enumerate(&self)->Vec<ScoredBasis>{fn walk(f:&Family,ids:&mut[usize;N],len:usize,available:u64,mask:u64,out:&mut Vec<ScoredBasis>){if len==N{let edge=std::array::from_fn(|i|std::array::from_fn(|j|f.edges[ids[i]][ids[j]]));let wc=ids.map(|i|f.wc[i]);let(score,order)=optimal_order(edge,wc);let ordered=order.map(|i|ids[i]);let compatible=ids.iter().fold((1<<f.moduli.len())-1,|a,i|a&f.neighbors[*i]);out.push(ScoredBasis{mask,compatible,score,ids:ordered});return}
    let mut available=available;while available!=0{let i=available.trailing_zeros()as usize;available&=available-1;if available.count_ones()as usize+1<N-len{break}ids[len]=i;walk(f,ids,len+1,available&f.neighbors[i],mask|1<<i,out);}}
    let mut out=Vec::new();walk(self,&mut[0;N],0,(1<<self.moduli.len())-1,0,&mut out);out.sort_unstable_by_key(|b|(b.score,b.mask));out}
}
pub fn best_pair(bases:&[ScoredBasis])->(usize,usize,u64){let mut best=u32::MAX;let mut pair=None;let mut checks=0;for i in 0..bases.len(){if i+1==bases.len()||bases[i].score+bases[i+1].score>=best{break}for j in i+1..bases.len(){let score=bases[i].score+bases[j].score;if score>=best{break}checks+=1;if bases[j].mask&!bases[i].compatible==0{best=score;pair=Some((i,j));}}}let(i,j)=pair.unwrap();(i,j,checks)}

#[cfg(test)]mod tests{use super::*;
#[test]fn naf_inverse_and_split_word_modular_recipes(){let f=Family::new();for k in 0..8192i64{let n=naf(k);let v:i128=n.iter().map(|(e,s)|(*s as i128)<<e).sum();assert_eq!(v,k as i128);assert!(n.windows(2).all(|p|p[1].0>p[0].0+1));}
    for &m in &f.moduli{for x in [0,1,m/2,m-2,m-1]{for k in [0,1,m/3,m-2,m-1]{let mut cc=Cost::default();let got=constant(x,k,m,true,&mut cc);assert_eq!(got,((x as u128*k as u128)%m as u128)as u64);let mut cc=Cost::default();assert_eq!(variable(x,k,m,&mut cc),got);}}}
    for i in 0..f.moduli.len(){for j in 0..f.moduli.len(){if f.neighbors[i]>>j&1!=0{assert_eq!((f.moduli[i]as u128*inv_word(f.moduli[i],f.moduli[j])as u128)%f.moduli[j]as u128,1);}}}
}
#[test]fn mixed_radix_conversion_small_integers_exhaustive(){let a=Basis::new(std::array::from_fn(|i|B-3-2520*i as u64));let b=Basis::new(std::array::from_fn(|i|B-3-2520*(i+5)as u64));for(source,target)in[(&a,&b),(&b,&a)]{for x in 0..4096u32{let n=BigInt::from(x);let input=source.residues(&n);for hybrid in [false,true]{let got=source.convert(&input,target,hybrid,&mut Cost::default());assert_eq!(got,target.residues(&n));}}}}
#[test]fn subset_optimizer_matches_all_permutations(){fn brute(edge:&[[u32;5];5],wc:&[u32;5],p:&mut[usize;5],start:usize,best:&mut u32){if start==5{let mut c=0;for i in 0..5{c+=4*(2*wc[p[i]]+2);if i<4{c+=5*wc[p[i]];}for j in i+1..5{c+=edge[p[i]][p[j]]+2*wc[p[j]]+4;}}*best=(*best).min(c);return}for i in start..5{p.swap(i,start);brute(edge,wc,p,start+1,best);p.swap(i,start);}}
    for seed in 0..8{let edge=std::array::from_fn(|i|std::array::from_fn(|j|((7*i+13*j+seed)%19)as u32));let wc=std::array::from_fn(|i|((i+seed)%3)as u32);let mut want=u32::MAX;brute(&edge,&wc,&mut[0,1,2,3,4],0,&mut want);assert_eq!(optimal_order(edge,wc).0,want);}}
}
