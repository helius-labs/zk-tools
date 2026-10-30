use num_bigint::{BigInt,BigUint};use num_traits::{One,Zero,ToPrimitive};use serde_json::json;
type U=BigUint;
fn digits(n:&U)->[i128;5]{core::array::from_fn(|i|((n>>(52*i))&U::from((1u64<<52)-1)).to_i128().unwrap())}
fn integer(t:&[i128])->BigInt{t.iter().enumerate().map(|(i,&v)|BigInt::from(v)<<(52*i)).sum()}
fn run(decimal:&str,bits:usize)->serde_json::Value{
 let p=U::parse_bytes(decimal.as_bytes(),10).unwrap();let r=U::one()<<bits;let two=&p*2u32;assert!(&p*4u32<r);assert!(&p*&p*12u32<&p*&r*3u32);
 let mut inverse=U::one();for _ in 0..9{inverse=(&inverse*(&r+2u32-(&p*&inverse)%&r))%&r;}assert_eq!(&inverse*&p%&r,U::one());let ni=&r-inverse;let inv=(&ni&U::from((1u64<<52)-1)).to_i128().unwrap();let pd=digits(&p);let mask=(1i128<<52)-1;
 let mut seed=0x4c4f4f534544454cu64;let mut next=|limit:&U|{let mut b=[0u8;64];for c in b.chunks_mut(8){seed^=seed<<13;seed^=seed>>7;seed^=seed<<17;c.copy_from_slice(&seed.to_le_bytes());}U::from_bytes_le(&b)%(limit+1u32)};
 let anchor=[U::zero(),U::one(),&p-1u32,p.clone()];let normal=[U::zero(),U::one(),&p-1u32,p.clone(),&two-1u32,two.clone()];let mut cases=0usize;let mut maximum=0i128;
 let mut chain=[U::one(),&p-1u32];
 for i in 0usize..(4usize.pow(2)*6usize.pow(4)+4096+1024){
   let mut x:[U;6]=if i<4usize.pow(2)*6usize.pow(4){let mut index=i;core::array::from_fn(|slot|{let values=if slot==2||slot==3{anchor.as_slice()}else{normal.as_slice()};let out=values[index%values.len()].clone();index/=values.len();out})}else{core::array::from_fn(|slot|next(if slot==2||slot==3{&p}else{&two}))};
   if i>=4usize.pow(2)*6usize.pow(4)+4096{x[0]=chain[0].clone();x[1]=chain[1].clone();}
   let delta:[U;2]=core::array::from_fn(|j|{let expected=&two+&x[j+2]-&x[j+4];let aa=digits(&x[j+2]);let bb=digits(&x[j+4]);let pp=digits(&two);let mut d=[0i128;5];let mut carry=0;for k in 0..5{let v=pp[k]+aa[k]-bb[k]+carry;assert!(v.abs()<(1i128<<54));carry=v>>52;d[k]=v&mask;}assert_eq!(carry,0);assert_eq!(integer(&d),BigInt::from(expected.clone()));assert!(expected<=&p*3u32);expected});
   let negative=&two-&x[1];let operands=[[x[0].clone(),delta[0].clone(),negative,delta[1].clone()],[x[0].clone(),delta[1].clone(),x[1].clone(),delta[0].clone()]];
   let mut outputs=[U::zero(),U::zero()];
   for k in 0..2{let v=&operands[k];let wide=&v[0]*&v[1]+&v[2]*&v[3];assert!(wide<=&p*&p*12u32);let d=v.each_ref().map(digits);let mut t=[0i128;6];let mut q=U::zero();for row in 0..5{for j in 0..5{for product in[d[0][j]*d[1][row],d[2][j]*d[3][row]]{t[j]+=product&mask;t[j+1]+=product>>52;}}let width=if row==4&&bits==256{48}else{52};let rm=(1i128<<width)-1;let m=((t[0]&mask)*inv)&rm;q+=U::from(m as u128)<<(52*row);let low=((t[0]+rm)>>width)+(((m<<(52-width))*pd[0])>>52);for j in 1..5{let product=m*pd[j];t[j]+=product&mask;t[j+1]+=product>>52;}for &v in&t{assert!((0..1i128<<61).contains(&v));maximum=maximum.max(v);}if width==52{t[1]+=low;t=[t[1],t[2],t[3],t[4],t[5],0];}else{t=[low+((t[1]&rm)<<4),(t[1]>>48)+((t[2]&rm)<<4),(t[2]>>48)+((t[3]&rm)<<4),(t[3]>>48)+(t[4]<<4),0];}}
     assert_eq!(q,&wide*&ni%&r);let full=&wide+q*&p;assert!((&full%&r).is_zero());let exact=full>>bits;assert_eq!(integer(&t[..5]),BigInt::from(exact.clone()));assert!(exact<&p*4u32);if bits==260{assert!(exact<two);}for j in 0..4{t[j+1]+=t[j]>>52;t[j]&=mask;}assert_eq!(integer(&t[..5]),BigInt::from(exact.clone()));outputs[k]=if bits==256&&exact>=two{exact-&two}else{exact};assert!(outputs[k]<two);
     let signed=if k==0{BigInt::from(&x[0]*(&x[2]+&two-&x[4]))-BigInt::from(&x[1]*(&x[3]+&two-&x[5]))}else{BigInt::from(&x[0]*(&x[3]+&two-&x[5])+&x[1]*(&x[2]+&two-&x[4]))};let modulus=BigInt::from(p.clone());let target=((signed%&modulus)+&modulus)%&modulus;assert_eq!(BigInt::from((&outputs[k]*&r)%&p),target);
   }chain=outputs;cases+=1;
 }
 json!({"radix":bits,"cases":cases,"outputs":2*cases,"normalized_delta_bound_3p":true,"exact_unsigned_CIOS_and_REDC":true,"fused_REDC_below_4p":true,"returned_below_2p":true,"max_unsigned_coefficient":maximum.to_string()})
}
fn main(){let mut rows=Vec::new();for p in["21888242871839275222246405745257275088696311157297823662689037894645226208583","21888242871839275222246405745257275088548364400416034343698204186575808495617"]{for bits in[256,260]{rows.push(run(p,bits));}}let total:u64=rows.iter().map(|v|v["cases"].as_u64().unwrap()).sum();let v=json!({"status":"passed","cases":total,"outputs":2*total,"rows":rows,"new_proofs":0,"SDK_requests":0,"proof_ledger":22990});std::fs::write("/home/ubuntu/src/zopro-perf-work/loose-delta175/model.json",serde_json::to_vec_pretty(&v).unwrap()).unwrap();println!("{v}");}
