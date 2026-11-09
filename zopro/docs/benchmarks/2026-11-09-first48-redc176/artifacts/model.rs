use ark_bn254::{Fq,Fr};
use ark_ff::{PrimeField,BigInteger,Field};
use num_bigint::BigUint as N;
use serde_json::{json,Value};
use sha2::{Sha256,Digest};
use std::fs;
const ROOT:&str="/home/ubuntu/src/zopro-perf-work/first48-redc176";
const B:u128=1<<52; const MASK:u128=B-1;
fn n(x:u64)->N{N::from(x)}
fn lo(x:&N)->u64{x.to_u64_digits().first().copied().unwrap_or(0)}
fn digits(x:&N)->[u128;5]{std::array::from_fn(|i|lo(&((x>>(52*i))&n(MASK as u64)))as u128)}
fn join(t:&[u128])->N{t.iter().enumerate().fold(n(0),|a,(i,&v)|a+(N::from(v)<<(52*i)))}
fn random(s:&mut u64,p:&N)->N{let mut v=n(0);for _ in 0..4{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;v=(v<<64)+n(*s);}v%p}
fn codec(t:[u128;5])->[u128;5]{let mask=(1u128<<48)-1;[t[0]&mask,(t[0]>>48)|((t[1]<<4)&MASK),(t[1]>>48)|((t[2]<<4)&MASK),(t[2]>>48)|((t[3]<<4)&MASK),(t[3]>>48)|(t[4]<<4)]}
fn codec_join(t:&[u128;5])->N{N::from(t[0])+(1..5).fold(n(0),|a,i|a+(N::from(t[i])<<(48+52*(i-1))))}
struct Ctx<F:PrimeField>{p:N,p2:N,r:N,neg_inv:N,inv:u128,fp_inv:F,max:u128,rows:u64,outputs:u64,hash:Sha256}
impl<F:PrimeField> Ctx<F>{fn new()->Self{let p=N::from_bytes_le(&F::MODULUS.to_bytes_le());let r=n(1)<<256;let inv=p.modpow(&((&r>>1)-n(1)),&r);assert_eq!((&p*&inv)%&r,n(1));let neg_inv=&r-inv;let p2=&p*n(2);let fp_inv=F::from(2u64).pow([256]).inverse().unwrap();Self{inv:lo(&neg_inv)as u128,p,p2,r,neg_inv,fp_inv,max:0,rows:0,outputs:0,hash:Sha256::new()}}
fn redc(&mut self,inputs:[&N;4],kind:usize,first:bool)->N{let [a,b,c,d]=inputs;for x in inputs{assert!(x<=&self.p2);}let complement=&self.p2-c;let c=if kind==2{&complement}else{c};let aa=digits(a);let cc=digits(c);let mut bb=digits(b);let mut dd=digits(d);if first{bb=codec(bb);dd=codec(dd);assert_eq!(codec_join(&bb),*b);assert_eq!(codec_join(&dd),*d);}for x in bb.into_iter().chain(dd){assert!(x<B);}
let pp=digits(&self.p);let mut t=[0u128;6];let mut done:usize=0;let mut qp=n(0);
for row in 0..5{let bits=if (first&&row==0)||(!first&&row==4){48}else{52};let mask=(1u128<<bits)-1;
for j in 0..5{let v=aa[j]*bb[row];t[j]+=v&MASK;t[j+1]+=v>>52;if kind!=0{let v=cc[j]*dd[row];t[j]+=v&MASK;t[j+1]+=v>>52;}}
let m=t[0].wrapping_mul(self.inv)&mask;qp+=N::from(m)<<done;
// Execute the canceled-low identity, without manufacturing the omitted low product.
let carry=(t[0]+mask)>>bits;let low=carry+((m*pp[0])>>bits);
for j in 1..5{let v=m*pp[j];t[j]+=v&MASK;t[j+1]+=v>>52;}
if bits==52{t[1]+=low;t=[t[1],t[2],t[3],t[4],t[5],0];}else{let old=t;t=[low+((old[1]&mask)<<4),(old[1]>>48)+((old[2]&mask)<<4),(old[2]>>48)+((old[3]&mask)<<4),(old[3]>>48)+((old[4]&mask)<<4),(old[4]>>48)+(old[5]<<4),0];}
done+=bits;self.max=self.max.max(*t.iter().max().unwrap());assert!(t.iter().all(|&v|v<(1u128<<60)));
let prefixmask=(n(1)<<done)-n(1);let mut original=a*(b.clone()&prefixmask.clone());if kind!=0{original+=c*(d.clone()&prefixmask.clone());}let numerator=original+&qp*&self.p;assert_eq!(&numerator&prefixmask,n(0));assert_eq!(join(&t),numerator>>done);self.rows+=1;
}
assert_eq!(done,256);let total=if kind==0{a*b}else{a*b+c*d};let q=(&total*&self.neg_inv)%&self.r;assert_eq!(qp,q);let raw=(&total+q*&self.p)/&self.r;assert_eq!(join(&t),raw);
for j in 0..4{t[j+1]+=t[j]>>52;t[j]&=MASK;}assert!(t.iter().all(|&v|v<B));let mut out=join(&t);if kind!=0&&out>=self.p2{out-=&self.p2;}assert!(out<self.p2);assert_eq!(&out%&self.p,(&total*&self.neg_inv*&self.p+&total)/&self.r%&self.p); // overwritten below by independent field semantics
let f=|x:&N|F::from_le_bytes_mod_order(&x.to_bytes_le())*self.fp_inv;
let want=match kind{0=>f(a)*f(b),1=>f(a)*f(b)+f(inputs[2])*f(d),_=>f(a)*f(b)-f(inputs[2])*f(d)};assert_eq!(f(&out),want);
self.hash.update(out.to_bytes_le());self.outputs+=1;out
}}
fn cohort<F:PrimeField>(ctx:&mut Ctx<F>,name:&str,cases:Vec<[N;4]>)->Value{let before=ctx.outputs;for v in &cases{for kind in 0..3{let refs=[&v[0],&v[1],&v[2],&v[3]];let left=ctx.redc(refs,kind,false);let right=ctx.redc(refs,kind,true);assert_eq!(left,right);}}json!({"name":name,"tuples":cases.len(),"outputs":ctx.outputs-before})}
fn field<F:PrimeField>(label:&str)->Value{let mut ctx=Ctx::<F>::new();let boundaries=[n(0),n(1),n(2),&ctx.p-n(1),ctx.p.clone(),&ctx.p+n(1),&ctx.p2-n(1),ctx.p2.clone()];let mut cohorts=Vec::new();
let mut corners=Vec::new();for a in &boundaries{for b in &boundaries{for c in &boundaries{for d in &boundaries{corners.push([a.clone(),b.clone(),c.clone(),d.clone()]);}}}}cohorts.push(cohort(&mut ctx,"inclusive_corners",corners));
let mut edges=Vec::new();for bit in 0usize..256{let power=(n(1)<<bit)%&(&ctx.p2+n(1));for value in [power.clone(),(&power+&ctx.p2)%(&ctx.p2+n(1)),(&power+n(1))%(&ctx.p2+n(1))]{edges.push([value.clone(),&ctx.p2-&value,ctx.p.clone(),n(1)]);}}cohorts.push(cohort(&mut ctx,"bit_boundaries_and_neighbors",edges));
let mut state=0x1847ef1973a81e07;let modulus=&ctx.p2+n(1);let randoms=(0..4096).map(|_|std::array::from_fn(|_|random(&mut state,&modulus))).collect();cohorts.push(cohort(&mut ctx,"full_width_deterministic",randoms));
let before=ctx.outputs;for chain in 0..256{let mut a=N::from(chain as u64+1);for round in 0..8{let b=random(&mut state,&modulus);let c=random(&mut state,&modulus);let d=random(&mut state,&modulus);let kind=round%3;let old=ctx.redc([&a,&b,&c,&d],kind,false);let new=ctx.redc([&a,&b,&c,&d],kind,true);assert_eq!(old,new);a=new;}}cohorts.push(json!({"name":"dependent_chains","chains":256,"steps":8,"outputs":ctx.outputs-before}));
json!({"field":label,"cohorts":cohorts,"outputs":ctx.outputs,"row_checks":ctx.rows,"max_observed_coefficient":ctx.max.to_string(),"declared_coefficient_bound":"2^60","result_digest":format!("{:x}",ctx.hash.finalize())})}
#[derive(Clone,Copy)]struct V{ready:u64}
struct Dag{ifmas:u64,alus:u64}
impl Dag{fn alu(&mut self,inputs:&[V])->V{self.alus+=1;V{ready:inputs.iter().map(|v|v.ready).max().unwrap_or(0)+2}}fn mac(&mut self,a:V,b:V,acc:V)->V{self.ifmas+=1;V{ready:a.ready.max(b.ready).max(acc.ready)+4}}
fn codec(&mut self,a:[V;5])->[V;5]{let mut out=a;out[0]=self.alu(&[a[0]]);for j in 1..5{let low=self.alu(&[a[j-1]]);let high=self.alu(&[a[j]]);let high=if j<4{self.alu(&[high])}else{high};out[j]=self.alu(&[low,high]);}out}
}
fn dag(first:bool,fused:bool)->Value{let z=V{ready:0};let mut d=Dag{ifmas:0,alus:0};let a=[z;5];let c=[z;5];let mut b=[z;5];let mut e=[z;5];if first{b=d.codec(b);if fused{e=d.codec(e);}}let p=[z;5];let mut t=[z;6];let mut frontier=Vec::new();
for row in 0..5{let bits=if(first&&row==0)||(!first&&row==4){48}else{52};for j in 0..5{t[j]=d.mac(a[j],b[row],t[j]);t[j+1]=d.mac(a[j],b[row],t[j+1]);}if fused{for j in 0..5{t[j]=d.mac(c[j],e[row],t[j]);t[j+1]=d.mac(c[j],e[row],t[j+1]);}}
let m=d.mac(t[0],z,z);let m=d.alu(&[m]);let carry=d.alu(&[t[0]]);let carry=d.alu(&[carry]);let mq=if bits==48{d.alu(&[m])}else{m};let low=d.mac(mq,p[0],carry);
for j in 1..5{t[j]=d.mac(m,p[j],t[j]);t[j+1]=d.mac(m,p[j],t[j+1]);}
if bits==52{t[1]=d.alu(&[t[1],low]);t=[t[1],t[2],t[3],t[4],t[5],z];}else{let old=t;let l=d.alu(&[old[1]]);let l=d.alu(&[l]);t[0]=d.alu(&[low,l]);for j in 1..4{let l=d.alu(&[old[j]]);let h=d.alu(&[old[j+1]]);let h=d.alu(&[h]);t[j]=d.alu(&[l,h]);}let l=d.alu(&[old[4]]);let h=d.alu(&[old[5]]);t[4]=d.alu(&[l,h]);t[5]=z;}
frontier.push(t.iter().map(|v|v.ready).max().unwrap());}
for j in 0..4{let carry=d.alu(&[t[j]]);t[j+1]=d.alu(&[t[j+1],carry]);t[j]=d.alu(&[t[j]]);}
json!({"first48":first,"fused":fused,"IFMA_sites":d.ifmas,"ALU_sites":d.alus,"row_frontier_scores":frontier,"output_ready_score":t.iter().map(|v|v.ready).max().unwrap()})}
fn main(){let fields=[field::<Fq>("Fq"),field::<Fr>("Fr")];let dags=[dag(false,false),dag(true,false),dag(false,true),dag(true,true)];let selected=(0..2).all(|i|{let off=&dags[i*2];let on=&dags[i*2+1];on["IFMA_sites"].as_u64()<=off["IFMA_sites"].as_u64()&&on["output_ready_score"].as_u64()<off["output_ready_score"].as_u64()});let out=json!({"math_pass":true,"fields":fields,"dependency_models":dags,"selects_native":selected,"measured_latency":false,"SDK_requests":0,"new_Gnark_exports":0});fs::write(format!("{ROOT}/result.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("{}",serde_json::to_string(&out).unwrap());}
