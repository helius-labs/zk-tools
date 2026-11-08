use num_bigint::{BigInt, BigUint, Sign};
use serde_json::{json, Value};
use std::{fs, path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/pmns-toeplitz6-feasibility176";
const PARAMETERS:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-23-pmns-field-component/experiment/parameters.json";
const B:i128=1i128<<52;
#[derive(Default,Clone)]struct Work{products:u64,add_sub:u64}
fn add(a:i128,b:i128,w:&mut Work)->i128{w.add_sub+=1;a.checked_add(b).unwrap()}
fn sub(a:i128,b:i128,w:&mut Work)->i128{w.add_sub+=1;a.checked_sub(b).unwrap()}
fn mul(a:i128,b:i128,w:&mut Work)->i128{w.products+=1;a.checked_mul(b).unwrap()}
fn t3(t:[i128;5],v:[i128;3],w:&mut Work)->[i128;3]{
 let [c,b,a,d,e]=t;let [x,y,z]=v;
 let yz=sub(y,z,w);let xz=sub(x,z,w);let xy=sub(x,y,w);
 let ab=add(a,b,w);let abc=add(ab,c,w);let abd=add(ab,d,w);let ad=add(a,d,w);let ade=add(ad,e,w);
 let p=mul(abc,z,w);let q=mul(abd,y,w);let r=mul(ade,x,w);
 let s=mul(b,yz,w);let t=mul(a,xz,w);let u=mul(d,xy,w);
 let o0=add(p,s,w);let o0=add(o0,t,w);let o1=sub(q,s,w);let o1=add(o1,u,w);let o2=sub(r,t,w);let o2=sub(o2,u,w);[o0,o1,o2]
}
fn t6(t:[i128;11],v:[i128;6],w:&mut Work)->[i128;6]{
 let a:[i128;5]=t[3..8].try_into().unwrap();
 let c=core::array::from_fn(|i|sub(t[i+6],a[i],w));
 let b=core::array::from_fn(|i|sub(t[i],a[i],w));
 let sum=core::array::from_fn(|i|add(v[i],v[i+3],w));
 let p=t3(a,sum,w);let q=t3(c,v[..3].try_into().unwrap(),w);let r=t3(b,v[3..].try_into().unwrap(),w);
 core::array::from_fn(|i|if i<3{add(p[i],r[i],w)}else{add(p[i-3],q[i-3],w)})
}
fn dense(t:&[i128;11],v:&[i128;6])->[i128;6]{core::array::from_fn(|i|(0..6).map(|j|t[i+5-j]*v[j]).sum())}
fn product(a:[i128;6],b:[i128;6],c:i128,d:i128,w:&mut Work)->[i128;6]{
 assert!(c==0||c==1&&d==1);
 let mut t=[0;11];for i in 1..6{t[5+i]=a[i];t[5-i]=if c==0{d*a[6-i]}else{a[6-i]+a[5-i]};}t[5]=a[0]+c*a[5];
 let mut out=t6(t,b,w);if c==1{let mut extra=0;for j in 0..6{let p=mul(a[5-j],b[j],w);extra=if j==0{p}else{add(extra,p,w)};}out[0]=sub(out[0],extra,w);}out
}
fn convolution(a:[i128;6],b:[i128;6],c:i128,d:i128)->[i128;6]{let mut t=[0i128;11];for i in 0..6{for j in 0..6{t[i+j]+=a[i]*b[j];}}for i in(6..11).rev(){let v=t[i];t[i-6]+=d*v;t[i-5]+=c*v;}t[..6].try_into().unwrap()}
fn positive(x:BigInt,p:&BigInt)->BigInt{let x=x%p;if x.sign()==Sign::Minus{x+p}else{x}}
fn eval(a:[i128;6],g:&BigInt,p:&BigInt)->BigInt{a.iter().rev().fold(BigInt::from(0),|v,&x|positive(v*g+BigInt::from(x),p))}
fn power_mod(p:&BigInt)->BigInt{positive(BigInt::from(1u32)<<52,p)}
fn next(state:&mut u64)->u64{*state^=*state<<13;*state^=*state>>7;*state^=*state<<17;*state}
fn random(state:&mut u64,u:i128)->[i128;6]{core::array::from_fn(|_|(next(state)as i128%(2*u+1))-u)}
fn int_array(v:&Value)->[i128;6]{core::array::from_fn(|i|v[i].as_i64().map(i128::from).unwrap_or_else(||v[i].as_u64().unwrap()as i128))}
fn combine(a:[i128;6],b:[i128;6],sign:i128)->[i128;6]{core::array::from_fn(|i|a[i]+sign*b[i])}
fn reduce(t:[i128;6],m:[i128;6],n:[i128;6],c:i128,d:i128)->([i128;6],[i128;6]){
 let low=t.map(|x|x.rem_euclid(B));let q=product(n,low,c,d,&mut Work::default()).map(|x|x.rem_euclid(B));
 assert_eq!(q,convolution(n,low,c,d).map(|x|x.rem_euclid(B)));
 let qm=product(m,q,c,d,&mut Work::default());assert_eq!(qm,convolution(m,q,c,d));
 let out=core::array::from_fn(|i|{let v=t[i]+qm[i];assert_eq!(v.rem_euclid(B),0);v/B});(out,q)
}
fn ceil_sixth(p:&BigUint)->u64{let(mut lo,mut hi)=(0u64,1u64<<43);while lo+1<hi{let mid=(lo+hi)/2;if BigUint::from(mid).pow(6)>=*p{hi=mid}else{lo=mid}}assert!(BigUint::from(hi).pow(6)>=*p);assert!(BigUint::from(hi-1).pow(6)<*p);hi}
fn main(){assert!(!Path::new(&format!("{R}/analysis.json")).exists());let parameters:Value=serde_json::from_slice(&fs::read(PARAMETERS).unwrap()).unwrap();
 let mut basis=0;for a in 0..11{for b in 0..6{let mut t=[0;11];let mut v=[0;6];t[a]=1;v[b]=1;let mut w=Work::default();assert_eq!(t6(t,v,&mut w),dense(&t,&v));assert_eq!(w.products,18);assert_eq!(w.add_sub,61);basis+=1;}}
 let mut state=0x746f65706c69747au64;let mut matvecs=0;for i in 0..8256{let t=core::array::from_fn(|j|if i<64{if i>>j%6&1==0{-(B-1)}else{B-1}}else{(next(&mut state)as i128%(2*B-1))-(B-1)});let v=core::array::from_fn(|j|if i<64{if i>>j&1==0{-(B-1)}else{B-1}}else{(next(&mut state)as i128%(2*B-1))-(B-1)});assert_eq!(t6(t,v,&mut Work::default()),dense(&t,&v));matvecs+=1;}
 let mut polynomial_basis=0;for(c,d)in[(0,2),(1,1)]{for i in 0..6{for j in 0..6{let mut a=[0;6];let mut b=[0;6];a[i]=1;b[j]=1;let mut w=Work::default();assert_eq!(product(a,b,c,d,&mut w),convolution(a,b,c,d));assert_eq!(w.products,if c==0{18}else{24});polynomial_basis+=1;}}}
 let mut fields=vec![];let mut scan=vec![];
 for(name,prime)in[("Fq","21888242871839275222246405745257275088696311157297823662689037894645226208583"),("Fr","21888242871839275222246405745257275088548364400416034343698204186575808495617")]{
  let p=BigUint::parse_bytes(prime.as_bytes(),10).unwrap();let p_i=BigInt::from(p.clone());let root=ceil_sixth(&p);let exponent=(&p-BigUint::from(1u32))/BigUint::from(6u32);assert_eq!((&p-BigUint::from(1u32))%BigUint::from(6u32),BigUint::from(0u32));
  for lambda in -64i64..=64{if lambda==0{continue}let base=if lambda>0{BigUint::from(lambda as u64)}else{&p-BigUint::from(lambda.unsigned_abs())};let eligible=base.modpow(&exponent,&p)==BigUint::from(1u32);let abs=lambda.unsigned_abs();let square=lambda>0&&(1u64..=8).any(|x|x*x==abs);let cube=(1u64..=4).any(|x|x*x*x==abs);let w=1+5*abs;let lhs=32*w as u128*(B as u128-1)*root as u128;let rhs=(B as u128)*(B as u128);scan.push(json!({"field":name,"lambda":lambda,"sixth_root_exists":eligible,"obvious_rational_square_or_cube":square||cube,"ceil_p_sixth_root":root,"uniform_box_certificate_not_excluded":lhs<=rhs,"minimum_required_lhs":lhs.to_string(),"B_squared":rhs.to_string(),"w":w,"survives_both_necessary_screens":eligible&&!(square||cube)&&lhs<=rhs}));}
  let source=&parameters["selected"][name];let m=int_array(&source["M"]);let n=int_array(&source["N"]);let c=source["c"].as_i64().unwrap()as i128;let d=source["d"].as_i64().unwrap()as i128;let rho=source["rho"].as_u64().unwrap()as i128;let gtext=if name=="Fq"{"1052884307992789723965989692260553020571906542047308545697783291464326919813"}else{"13725448893911834185645684450194138098045799619100669937482123171837231674367"};let gamma=BigInt::parse_bytes(gtext.as_bytes(),10).unwrap();assert_eq!(eval(m,&gamma,&p_i),BigInt::from(0));assert_eq!(positive(gamma.pow(6)-&gamma*c-d,&p_i),BigInt::from(0));
  let inverse=convolution(m,n,c,d).map(|x|x.rem_euclid(B));assert_eq!(inverse,[B-1,0,0,0,0,0]);
  let mut column_sums=[0i128;6];let mut row_weights=[0i128;6];for j in 0..6{let mut b=[0;6];b[j]=1;let column=convolution(m,b,c,d);column_sums[j]=column.iter().map(|v|v.abs()).sum();let weights=convolution([1;6],b,c,d);for i in 0..6{row_weights[i]+=weights[i].abs();}}let s=*column_sums.iter().max().unwrap();let w=*row_weights.iter().max().unwrap();assert_eq!(2*s,rho);
  let normal_certificate=2*w*rho*rho+(B-1)*s<=B*rho;assert!(normal_certificate);let wide_certificate=8*w*rho*rho+(B-1)*s<=B*rho;
  let mut outputs=0u64;let mut corners=0u64;let mut wide_outputs=0u64;let mut wide_outside=0u64;let mut largest=0i128;let mut max_wide=0i128;
  for i in 0..8192u64{let a=if i<4096{core::array::from_fn(|j|if i>>j&1==0{-rho}else{rho})}else{random(&mut state,rho)};let b=if i<4096{core::array::from_fn(|j|if i>>(j+6)&1==0{-rho}else{rho})}else{random(&mut state,rho)};let cc=random(&mut state,rho);let dd=random(&mut state,rho);let ab=product(a,b,c,d,&mut Work::default());assert_eq!(ab,convolution(a,b,c,d));let cd=product(cc,dd,c,d,&mut Work::default());assert_eq!(cd,convolution(cc,dd,c,d));
   let ae=eval(a,&gamma,&p_i);let be=eval(b,&gamma,&p_i);let ce=eval(cc,&gamma,&p_i);let de=eval(dd,&gamma,&p_i);
   for sign in[0i128,1,-1]{let t=combine(ab,cd,sign);let(out,_)=reduce(t,m,n,c,d);assert!(out.iter().all(|x|x.abs()<=rho));largest=largest.max(out.iter().map(|v|v.abs()).max().unwrap());assert_eq!(positive(eval(out,&gamma,&p_i)*power_mod(&p_i),&p_i),positive(&ae*&be+sign*&ce*&de,&p_i));outputs+=1;if i<4096{corners+=1;}}
  }
  let mut last=random(&mut state,rho);for i in 0..1024{let b=random(&mut state,rho);let cc=random(&mut state,rho);let dd=random(&mut state,rho);let sign=[0i128,1,-1][i%3];let t=combine(product(last,b,c,d,&mut Work::default()),product(cc,dd,c,d,&mut Work::default()),sign);let(out,_)=reduce(t,m,n,c,d);assert!(out.iter().all(|x|x.abs()<=rho));let expected=positive(eval(last,&gamma,&p_i)*eval(b,&gamma,&p_i)+sign*eval(cc,&gamma,&p_i)*eval(dd,&gamma,&p_i),&p_i);assert_eq!(positive(eval(out,&gamma,&p_i)*power_mod(&p_i),&p_i),expected);last=out;outputs+=1;}
  for i in 0..4096u64{let a=core::array::from_fn(|j|if i>>j&1==0{-2*rho}else{2*rho});let b=core::array::from_fn(|j|if i>>(j+6)&1==0{-2*rho}else{2*rho});let t=combine(product(a,b,c,d,&mut Work::default()),product(a,b,c,d,&mut Work::default()),1);let(out,_)=reduce(t,m,n,c,d);let max=out.iter().map(|v|v.abs()).max().unwrap();max_wide=max_wide.max(max);wide_outside+=u64::from(max>rho);wide_outputs+=1;}
  let mut literal=Work::default();product([1;6],[1;6],c,d,&mut literal);fields.push(json!({"field":name,"old_public_parameters_reused_not_newly_generated":true,"c":c,"d":d,"rho":rho,"S":s,"w":w,"normalized_signed_box_fused_certificate_passes":normal_certificate,"wide_signed_2rho_fused_certificate_passes":wide_certificate,"new_factored_PMNS_outputs":outputs,"corner_outputs":corners,"output_fed_outputs":1024,"largest_normalized_output_coefficient":largest,"wide_corner_fused_outputs":wide_outputs,"wide_corner_outputs_outside_rho":wide_outside,"largest_wide_output_coefficient":max_wide,"coefficient_products_per_stage":literal.products,"matvec_add_sub_before_word_lowering":literal.add_sub,"nominal_IFMA_ordinary":5*literal.products,"nominal_IFMA_fused":7*literal.products}));
 }
 let survivors:Vec<_>=scan.iter().filter(|r|r["survives_both_necessary_screens"]==true).cloned().collect();
 let analysis=json!({"passed":true,"generic_Toeplitz_basis_pairs":basis,"generic_fullwidth_signed_matvecs":matvecs,"polynomial_basis_pairs":polynomial_basis,"Toeplitz6_coefficient_products":18,"Toeplitz6_integer_add_sub":61,"binomial_scan_rows":scan,"surviving_necessary_geometries":survivors,"fields":fields,"product_counts_not_native_instructions_or_cycles":true,"new_native_kernels":0,"new_timings":0,"new_profiles":0,"SDK_requests":0,"new_proofs":0,"canonical_proofs":37023,"production_changed":false,"adopted":false,"goal_complete":false});
 fs::write(format!("{R}/analysis.json"),serde_json::to_vec_pretty(&analysis).unwrap()).unwrap();println!("Toeplitz screen PASS:66basis/8256fullwidth matvecs/72polynomialbasis;51200newPMNS outputs+8192widecorners; surviving necessary geometries{}",analysis["surviving_necessary_geometries"].as_array().unwrap().len());
}
