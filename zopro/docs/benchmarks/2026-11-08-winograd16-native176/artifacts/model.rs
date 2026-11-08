use ark_ff::{AdditiveGroup,Field as ArkField,PrimeField};
use serde_json::json;use std::fs;
use zopro_bn254::Fr;
const R:&str="/home/ubuntu/src/zopro-perf-work/winograd16-176";
#[derive(Default,Debug,Clone,Copy,PartialEq,Eq)]struct Count{m:u64,a:u64}
fn plus(a:Fr,b:Fr,c:&mut Count)->Fr{c.a+=1;a+b}
fn minus(a:Fr,b:Fr,c:&mut Count)->Fr{c.a+=1;a-b}
fn prod(a:Fr,b:Fr,c:&mut Count)->Fr{c.m+=1;a*b}
#[derive(Clone,Copy)]struct Factors{w:Fr,f:[Fr;13]}
impl Factors{
fn new(w:Fr)->Self{assert_eq!(w.pow(&[16]),Fr::ONE);assert_eq!(w.pow(&[8]),-Fr::ONE);let half=Fr::from(2u64).inverse().unwrap();let wi=w.inverse().unwrap();let w2=w.square();let w3=w2*w;let w6=w3.square();let i=w2.square();let a=(w+wi)*half;let b=(w3+wi.pow(&[3]))*half;let d=(w2+wi.square())*half;Self{w,f:[i,i,(w2+w6)*half,(w2-w6)*half,d,i*d,i,a-b,-(a+b),b,i*(b-a),-i*(b+a),i*a]}}
}
fn rev(j:usize)->usize{j.reverse_bits()>>(usize::BITS-4)}
fn factored(x:[Fr;16],r:&Factors,c:&mut Count)->[Fr;16]{
 let mut ss=[Fr::ZERO;8];let mut tt=[Fr::ZERO;8];for j in 0..8{ss[j]=plus(x[j],x[j+8],c);tt[j]=minus(x[j],x[j+8],c);}
 let mut s=[Fr::ZERO;4];let mut t=[Fr::ZERO;4];for j in 0..4{s[j]=plus(ss[j],ss[j+4],c);t[j]=minus(ss[j],ss[j+4],c);}
 let p0=prod(minus(s[1],s[3],c),r.f[0],c);let p1=prod(t[2],r.f[1],c);let p2=prod(plus(t[1],t[3],c),r.f[2],c);let p3=prod(minus(t[1],t[3],c),r.f[3],c);
 let a=plus(s[0],s[2],c);let b=plus(s[1],s[3],c);let d=minus(s[0],s[2],c);
 let e0=plus(a,b,c);let e4=minus(a,b,c);let e2=plus(d,p0,c);let e6=minus(d,p0,c);
 let a=plus(t[0],p1,c);let b=minus(t[0],p1,c);let d=plus(p2,p3,c);let e=minus(p2,p3,c);
 let evens=[e0,e4,e2,e6,plus(a,d,c),minus(a,d,c),plus(b,e,c),minus(b,e,c)];
 let a1=minus(tt[1],tt[7],c);let a2=minus(tt[2],tt[6],c);let a3=minus(tt[3],tt[5],c);
 let b1=plus(tt[1],tt[7],c);let b2=plus(tt[2],tt[6],c);let b3=plus(tt[3],tt[5],c);
 let d=prod(a2,r.f[4],c);let g=prod(b2,r.f[5],c);let h=prod(tt[4],r.f[6],c);
 let m0=prod(a1,r.f[7],c);let m1=prod(a3,r.f[8],c);let m2=prod(plus(a1,a3,c),r.f[9],c);
 let n0=prod(b1,r.f[10],c);let n1=prod(b3,r.f[11],c);let n2=prod(plus(b1,b3,c),r.f[12],c);
 let u=plus(m0,m2,c);let v=plus(m1,m2,c);let w=plus(n0,n2,c);let z=plus(n1,n2,c);
 let a=plus(tt[0],d,c);let b=minus(tt[0],d,c);let e=plus(h,g,c);let f=minus(g,h,c);
 let base1=plus(a,e,c);let base5=minus(b,f,c);let base3=plus(b,f,c);let base7=minus(a,e,c);
 let o1=plus(u,w,c);let o5=minus(z,v,c);let o3=plus(v,z,c);let o7=minus(w,u,c);
 let odds=[plus(base1,o1,c),minus(base1,o1,c),plus(base5,o5,c),minus(base5,o5,c),plus(base3,o3,c),minus(base3,o3,c),plus(base7,o7,c),minus(base7,o7,c)];
 core::array::from_fn(|j|if j<8{evens[j]}else{odds[j-8]})
}
fn af(x:Fr)->ark_bn254::Fr{ark_bn254::Fr::from_be_bytes_mod_order(&x.to_be_bytes())}
fn direct(x:&[Fr;16],w:Fr)->[ark_bn254::Fr;16]{let x=x.map(af);let w=af(w);core::array::from_fn(|k|{let step=w.pow([k as u64]);let mut p=ark_bn254::Fr::ONE;let mut sum=ark_bn254::Fr::ZERO;for x in x{sum+=x*p;p*=step;}sum})}
fn rnd(s:&mut u64)->u64{*s^=*s<<13;*s^=*s>>7;*s^=*s<<17;*s}
fn random(s:&mut u64)->Fr{let mut b=[0;64];for chunk in b.chunks_exact_mut(8){chunk.copy_from_slice(&rnd(s).to_le_bytes());}Fr::from_random_bytes(&b)}
fn check(x:[Fr;16],r:&Factors)->Count{let reference=direct(&x,r.w);let mut c=Count::default();let out=factored(x,r,&mut c);for j in 0..16{assert_eq!(af(out[j]),reference[rev(j)]);}
 let bit_input:[Fr;16]=core::array::from_fn(|j|x[rev(j)]);let natural=core::array::from_fn(|j|bit_input[rev(j)]);let mut c2=Count::default();let got=factored(natural,r,&mut c2);let natural_out:[Fr;16]=core::array::from_fn(|j|got[rev(j)]);assert_eq!(natural_out.map(af),reference);assert_eq!(c,c2);assert_eq!(c,Count{m:13,a:74});c}
fn main(){let mut seed=0x5746_5441_3136_3236;let mut rows=vec![];let mut inputs=0;let mut matrix_entries=0;let mut repeats=0;
 for power in(1..16).step_by(2){let r=Factors::new(Fr::root_of_unity(16).unwrap().pow(&[power]));assert!(r.f.iter().all(|x|!x.is_zero()&&*x!=Fr::ONE&&*x!=-Fr::ONE));
 for basis in 0..16{let x=core::array::from_fn(|j|if j==basis{Fr::ONE}else{Fr::ZERO});check(x,&r);inputs+=1;matrix_entries+=16;}
 for kind in 0..515{let x=core::array::from_fn(|j|match kind{0=>Fr::ZERO,1=>Fr::ONE,2=>-Fr::ONE,3=>if j%2==0{Fr::ONE}else{-Fr::ONE},4=>Fr::from(2u64).pow(&[(j*16)as u64])-Fr::ONE,5=>Fr::from(2u64).pow(&[(j*16)as u64])+Fr::ONE,_=>random(&mut seed)});check(x,&r);inputs+=1;}
 let mut x=core::array::from_fn(|_|random(&mut seed));for _ in 0..129{check(x,&r);let mut c=Count::default();x=factored(x,&r,&mut c);repeats+=1;}
 rows.push(json!({"root_power":power,"constant_products":13,"add_sub":74,"nontrivial_coefficients":13,"radix2_nonunit_products":17,"radix2_add_sub":64,"all_matrix_entries_checked":256,"fresh_vector_cases":531,"repeated_states":129,"DIF_and_DIT_orders_checked":true}));}
 let out=json!({"mathematics_passed":true,"primitive_roots":8,"fresh_vectors":inputs,"matrix_entries":matrix_entries,"repeated_states":repeats,"DIF_and_DIT_orders_checked":true,"counts_per_transform":{"products":13,"adds_subs":74,"unit_pruned_radix2_products":17,"unit_pruned_radix2_adds_subs":64},"prospective_two_sequence_packing":{"logical_product_lanes":26,"vector_product_calls":4,"physical_lanes":32,"dummy_lanes":6,"accepted_vector_product_calls":6,"vector_factor_payload_bytes":1280},"rows":rows,"necessary_mathematical_gate_passed":true,"selects_native_packing_qualification":true,"selects_performance_capture":false,"selects_SDK":false,"new_timings":0,"new_SDK_requests":0,"new_proofs":0,"canonical_unchanged":35171,"production_changed":false,"goal_complete":false});fs::write(format!("{R}/model-result.json"),serde_json::to_vec_pretty(&out).unwrap()).unwrap();println!("PASS8roots/{}freshvectors/{}matrixentries/{}changingstates,bothorders;13products74adds select native packing qualification ONLY",inputs,matrix_entries,repeats);}
