//! Exact interpreter of top-limb cross-product sharing in six CIOS slots.
use num_bigint::{BigInt,BigUint};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use std::{collections::BTreeSet,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/top-cross52-model176";
const B:i128=1<<52;const MASK:u64=(1<<52)-1;
const PRIMES:[&str;2]=["21888242871839275222246405745257275088696311157297823662689037894645226208583","21888242871839275222246405745257275088548364400416034343698204186575808495617"];
type Digits=[u64;5];
fn u(n:u64)->BigUint{BigUint::from(n)}
fn integer(t:&[i128])->BigInt{t.iter().rev().fold(BigInt::from(0),|v,x|(v<<52usize)+BigInt::from(*x))}
fn digits(n:&BigUint)->Digits{let mut v=n.clone();std::array::from_fn(|_|{let x=(&v&u(MASK)).to_u64_digits().first().copied().unwrap_or(0);v>>=52usize;x})}
fn positive(n:BigInt,m:&BigInt)->BigInt{((n%m)+m)%m}
fn inverse(a:&BigUint,m:&BigUint)->BigUint{let(mut r,mut s)=(BigInt::from(m.clone()),BigInt::from(a.clone()));let(mut x,mut y)=(BigInt::from(0),BigInt::from(1));while s!=BigInt::from(0){let q=&r/&s;(r,s)=(s.clone(),r-&q*s);(x,y)=(y.clone(),x-q*y);}assert_eq!(r,BigInt::from(1));positive(x,&BigInt::from(m.clone())).to_biguint().unwrap()}
fn halves(a:u64,b:u64)->(i128,i128){assert!(a<=MASK&&b<=MASK);let v=a as i128*b as i128;(v&(B-1),v>>52)}
// Byte-exact arithmetic identity inherited from blocked-pair52-model176.
fn signed_pair(x: i128, y: i128, radix: i128) -> (i128, i128) {
    assert!((-radix..radix).contains(&x) && (-radix..radix).contains(&y));
    let a = x.abs(); let oriented = if x < 0 { -y } else { y };
    let p = a * oriented.rem_euclid(radix);
    (p % radix, p / radix - if oriented < 0 { a } else { 0 })
}
#[derive(Default)]struct Count{outputs:u64,steps:u64,raw:u64,reduction:u64,cross:u64,negative:u64,two_p:u64,max:i128,min:i128,mixed_max:i128,digest:Sha256}
fn observe(c:&mut Count,n:i128){assert!(n.abs()<64*B);assert!(i64::try_from(n).is_ok());c.max=c.max.max(n.abs());c.min=c.min.min(n);}
fn put(t:&mut[i128;6],i:usize,n:i128,c:&mut Count){t[i]+=n;observe(c,t[i]);}
fn raw(t:&mut[i128;6],i:usize,a:u64,b:u64,c:&mut Count){let(l,h)=halves(a,b);put(t,i,l,c);put(t,i+1,h,c);c.raw+=2;}
fn row(t:&mut[i128;6],i:usize,a:&Digits,b:&Digits,d4:(i128,i128),c:&mut Count){
 let d=halves(a[i],b[i]);c.raw+=2;
 put(t,i,d.0,c);put(t,i+1,d.1,c);
 for j in 0..4{if j!=i{raw(t,j,a[j],b[i],c);}}
 let dx=a[i] as i128-a[4] as i128;let dy=b[4] as i128-b[i] as i128;
 let p=signed_pair(dx,dy,B);c.raw+=2;c.negative+=u64::from(dx*dy<0);c.cross+=1;
 assert_eq!(p.0+B*p.1,dx*dy);
 let lo=d.0+d4.0+p.0;let hi=d.1+d4.1+p.1;observe(c,lo);observe(c,hi);
 assert_eq!(lo+B*hi,a[4] as i128*b[i] as i128+a[i] as i128*b[4] as i128);
 put(t,4,lo,c);put(t,5,hi,c);
}
#[derive(Clone,Copy)]struct Interval{lo:i128,hi:i128}
impl Interval{fn add(self,b:Self)->Self{Self{lo:self.lo+b.lo,hi:self.hi+b.hi}}}
fn bound(fused:bool,mixed:bool)->Value{
 let zero=Interval{lo:0,hi:0};let word=Interval{lo:0,hi:B-1};
 let cross_low=Interval{lo:0,hi:3*(B-1)};let cross_high=Interval{lo:1-B,hi:3*(B-1)};
 let mut t=[zero;6];let mut maximum=0i128;let mut trace=vec![];
 let mut add=|t:&mut[Interval;6],i:usize,x:Interval|{t[i]=t[i].add(x);maximum=maximum.max(t[i].lo.abs()).max(t[i].hi.abs());};
 let times=if fused{2}else{1};
 for i in 0..4{for _ in 0..times{for j in 0..4{add(&mut t,j,word);add(&mut t,j+1,word);}add(&mut t,4,cross_low);add(&mut t,5,cross_high);}
  let carry=Interval{lo:(t[0].lo+B-1).div_euclid(B),hi:(t[0].hi+B-1).div_euclid(B)}.add(word);
  for j in 1..5{add(&mut t,j,word);add(&mut t,j+1,word);}add(&mut t,1,carry);
  t=[t[1],t[2],t[3],t[4],t[5],zero];trace.push(json!({"row":i,"after_cancel_intervals":t.map(|x|[x.lo.to_string(),x.hi.to_string()])}));
 }
 for _ in 0..times{add(&mut t,4,word);add(&mut t,5,word);}
 let bits=if mixed{48}else{52};let radix=1i128<<bits;
 let carry=Interval{lo:(t[0].lo+radix-1).div_euclid(radix),hi:(t[0].hi+radix-1).div_euclid(radix)}.add(word);
 for j in 1..5{add(&mut t,j,word);add(&mut t,j+1,word);}
 let before=maximum;let scale=if mixed{16}else{1};let mut out=std::array::from_fn::<_,5,_>(|i|Interval{lo:t[i+1].lo*scale,hi:t[i+1].hi*scale});out[0]=out[0].add(carry);
 for j in 0..4{maximum=maximum.max(out[j].lo.abs()).max(out[j].hi.abs());let c=Interval{lo:out[j].lo.div_euclid(B),hi:out[j].hi.div_euclid(B)};out[j]=word;out[j+1]=out[j+1].add(c);}
 for x in out{maximum=maximum.max(x.lo.abs()).max(x.hi.abs());}
 assert!(before<64*B&&maximum<1024*B&&maximum<i64::MAX as i128);
 json!({"fused":fused,"mixed":mixed,"coefficient_slots":6,"prepacking_abs_bound_inclusive":before.to_string(),"including_packing_abs_bound_inclusive":maximum.to_string(),"signed64":true,"rows":trace})
}
struct Field{p:BigUint,two:BigUint,pd:Digits,radix:BigUint,pinv:BigUint,rinv:BigUint,inv:u64,mixed:bool}
impl Field{fn new(p:&BigUint,mixed:bool)->Self{let radix=u(1)<<if mixed{256usize}else{260};let wb=u(1)<<52usize;Self{p:p.clone(),two:p*u(2),pd:digits(p),pinv:inverse(p,&radix),rinv:inverse(&(&radix%p),p),inv:(&wb-inverse(&(p%&wb),&wb)).to_u64_digits()[0],radix,mixed}}}
fn execute(xs:&[BigUint;4],f:&Field,op:usize,c:&mut Count)->BigUint{
 for x in xs{assert!(x<=&f.two);c.two_p+=u64::from(x==&f.two);}
 let cc=if op==2{&f.two-&xs[2]}else{xs[2].clone()};
 let mut products=vec![(digits(&xs[0]),digits(&xs[1]))];if op!=0{products.push((digits(&cc),digits(&xs[3])));}
 let mut top=vec![];for(a,b)in &products{top.push(halves(a[4],b[4]));c.raw+=2;}
 let total=&xs[0]*&xs[1]+if op==0{u(0)}else{&cc*&xs[3]};
 let mut full_ref=total.clone();let mut partial=BigInt::from(0);let mut t=[0i128;6];let mut qs=vec![];
 for i in 0..5{
  for((a,b),d4)in products.iter().zip(&top){if i<4{row(&mut t,i,a,b,*d4,c);let alow=integer(&a[..4].iter().map(|x|*x as i128).collect::<Vec<_>>());let cross=a[4] as u128*b[i] as u128+a[i] as u128*b[4] as u128;partial+=alow*BigInt::from(b[i])+(BigInt::from(cross)<<208usize);}else{put(&mut t,4,d4.0,c);put(&mut t,5,d4.1,c);partial+=BigInt::from(a[4] as u128*b[4] as u128)<<208usize;}}
  assert_eq!(integer(&t),partial,"raw row {i}");
  let bits=if i==4&&f.mixed{48usize}else{52};let radix=1i128<<bits;
  let q=((t[0] as u64 as u128*f.inv as u128)&((radix-1)as u128))as u64;
  let mask=(u(1)<<bits)-u(1);
  let low=(&full_ref&mask).to_u64_digits().first().copied().unwrap_or(0);
  assert_eq!(q,((low as u128*f.inv as u128)&((radix-1)as u128))as u64);
  let exact=&partial+BigInt::from(q)*BigInt::from(f.p.clone());assert_eq!(&exact%BigInt::from(radix),BigInt::from(0));partial=exact>>bits;
  let m=if bits==48{q<<4}else{q};let(_,h)=halves(m,f.pd[0]);let carry=(t[0]+radix-1).div_euclid(radix)+h;observe(c,carry);
  for j in 1..5{let(l,h)=halves(q,f.pd[j]);put(&mut t,j,l,c);put(&mut t,j+1,h,c);}c.reduction+=10;c.steps+=1;qs.push(q);
  full_ref=(&full_ref+u(q)*&f.p)>>bits;
  if i<4{put(&mut t,1,carry,c);t=[t[1],t[2],t[3],t[4],t[5],0];assert_eq!(integer(&t),partial);}else{
   let scale=if f.mixed{16}else{1};let mut out=std::array::from_fn::<_,5,_>(|j|t[j+1]*scale);out[0]+=carry;
   for x in &out{assert!(x.abs()<1024*B);c.mixed_max=c.mixed_max.max(x.abs());}
   for j in 0..4{out[j+1]+=out[j].div_euclid(B);assert!(out[j+1].abs()<1024*B);c.mixed_max=c.mixed_max.max(out[j+1].abs());out[j]=out[j].rem_euclid(B);}assert_eq!(integer(&out),partial);assert!(out.iter().all(|x|*x>=0&&*x<B));
  }
 }
 assert_eq!(partial,BigInt::from(full_ref.clone()));
 let quotient=(&f.radix-(&total*&f.pinv)%&f.radix)%&f.radix;
 let mut qword=u(0);for(i,q)in qs.iter().enumerate(){qword+=u(*q)<<(52*i);}assert_eq!(qword,quotient);
 assert_eq!(full_ref,(&total+&quotient*&f.p)/&f.radix);
 let mut out=full_ref;if op!=0&&f.mixed&&out>=f.two{out-=&f.two;}assert!(out<f.two);
 assert_eq!(&out%&f.p,((&total%&f.p)*&f.rinv)%&f.p);
 let residue=match op{0=>(&xs[0]*&xs[1])%&f.p,1=>(&xs[0]*&xs[1]+&xs[2]*&xs[3])%&f.p,_=>positive(BigInt::from(&xs[0]*&xs[1])-BigInt::from(&xs[2]*&xs[3]),&BigInt::from(f.p.clone())).to_biguint().unwrap()};
 assert_eq!(&out%&f.p,(residue*&f.rinv)%&f.p);
 c.outputs+=1;let b=out.to_bytes_le();c.digest.update((b.len()as u64).to_le_bytes());c.digest.update(b);out
}
fn symbolic(){let mut count=[[0i32;5];5];for i in 0..4{for j in 0..4{count[j][i]+=1;}count[i][i]+=1;count[4][4]+=1;count[i][4]+=1;count[4][i]+=1;count[i][i]-=1;count[4][4]-=1;}count[4][4]+=1;assert!(count.iter().flatten().all(|x|*x==1));}
fn main(){assert!(!Path::new(&format!("{R}/result.json")).exists());symbolic();let bounds:Vec<_>=[false,true].into_iter().flat_map(|f|[false,true].into_iter().map(move|m|bound(f,m))).collect();
 let mut panels=vec![];for(tag,decimal)in["Fq","Fr"].into_iter().zip(PRIMES){let p=BigUint::parse_bytes(decimal.as_bytes(),10).unwrap();let two=&p*u(2);let limit=&two+u(1);let mut edges=BTreeSet::from([u(0),u(1),u(2),&p-u(1),p.clone(),&p+u(1),&two-u(1),two.clone()]);for bit in[26usize,48,51,52,53,104,105,156,157,208,209,253,254]{let a=u(1)<<bit;for x in[&a-u(1),a.clone(),&a+u(1)]{if x<=two{edges.insert(x);}}}let edges:Vec<_>=edges.into_iter().collect();
  for mixed in[true,false]{let f=Field::new(&p,mixed);let mut c=Count::default();let mut operations=[0u64;3];
   for i in 0..edges.len(){for j in 0..edges.len(){let xs=[edges[i].clone(),edges[j].clone(),edges[(i+7)%edges.len()].clone(),edges[(j+23)%edges.len()].clone()];for op in 0..3{execute(&xs,&f,op,&mut c);operations[op]+=1;}}}
   for case in 0u64..4096{let xs=std::array::from_fn(|slot|{let mut h=Sha256::new();h.update(b"top-cross52-model176/new-population");h.update(tag.as_bytes());h.update(case.to_le_bytes());h.update((slot as u64).to_le_bytes());BigUint::from_bytes_le(&h.finalize())%&limit});for op in 0..3{execute(&xs,&f,op,&mut c);operations[op]+=1;}}
   for op in 0..3{let mut xs=[&two-u(1),p.clone(),u(1),u(2)];for _ in 0..1024{let y=execute(&xs,&f,op,&mut c);xs=[xs[1].clone(),xs[2].clone(),xs[3].clone(),y];operations[op]+=1;}}
   assert_eq!(c.steps,c.outputs*5);assert_eq!(c.raw,42*operations[0]+84*(operations[1]+operations[2]));assert_eq!(c.reduction,50*c.outputs);assert_eq!(c.cross,4*operations[0]+8*(operations[1]+operations[2]));
   panels.push(json!({"field":tag,"radix_bits":if mixed{256}else{260},"boundary_values":edges.len(),"boundary_tuples":edges.len()*edges.len(),"random_tuples":4096,"chain_steps_per_operation":1024,"operations":operations,"outputs":c.outputs,"cancellation_steps":c.steps,"raw_IFMA_proxy":c.raw,"reduction_IFMA_proxy":c.reduction,"cross_identities":c.cross,"negative_pair_products":c.negative,"inclusive_two_p_input_occurrences":c.two_p,"max_observed_prepacking_abs":c.max.to_string(),"minimum_observed_coefficient":c.min.to_string(),"max_observed_packing_abs":c.mixed_max.to_string(),"output_digest":format!("{:x}",c.digest.finalize())}));
  }
 }
 let result=json!({"status":"pass","symbolic_coefficients":25,"coefficient_slots":6,"ordinary_IFMA_proxy":92,"fused_IFMA_proxy":134,"original_IFMA_proxy":[100,150],"interval_bounds":bounds,"panels":panels,"native_executed":false,"timings":0,"SDK_requests":0,"Gnark_exports":0,"canonical_proofs_unchanged":36477,"production_changed":false,"adopted":false,"goal_complete":false});fs::write(format!("{R}/result.json"),serde_json::to_vec_pretty(&result).unwrap()).unwrap();println!("Top-limb sharing exact model PASS; six slots, 92/134 IFMA recipe; no native or clocks");
}
