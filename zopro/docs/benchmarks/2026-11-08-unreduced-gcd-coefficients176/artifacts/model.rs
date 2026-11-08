use num_bigint::{BigInt, BigUint, Sign};
use serde_json::{json, Value};
use std::{collections::BTreeMap, fs};
use zopro_bn254::{Field, Fp, FpConfig, FqConfig, FrConfig};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/unreduced-gcd-coefficients176";
mod matrix {
    use core::hint::select_unpredictable;
    const BITS: usize = 31;
    include!("accepted-matrix.rs.inc");
}
fn abs(x: &BigInt) -> BigInt { if x.sign()==Sign::Minus {-x} else {x.clone()} }
fn residue(x: BigInt, p:&BigInt)->BigInt { let r=x%p; if r.sign()==Sign::Minus {r+p} else {r} }
fn raw(w:[u64;4])->BigUint { BigUint::from_bytes_le(&w.into_iter().flat_map(u64::to_le_bytes).collect::<Vec<_>>()) }
fn words(x:&BigUint)->[u64;4] { let a=x.to_u64_digits(); assert!(a.len()<=4); std::array::from_fn(|i|a.get(i).copied().unwrap_or(0)) }
fn approximation(a:&BigUint,bits:u64)->u64 {
    let w=words(a);
    if bits<=64 {return w[0]}
    let shift=bits as usize-33; let (j,n)=(shift/64,shift%64);
    let mut top=w[j]>>n;
    if n!=0 && j<3 {top|=w[j+1]<<(64-n)}
    (top<<31)|(w[0]&((1<<31)-1))
}
fn normalized_row(a:&BigUint,b:&BigUint,f:&mut i64,g:&mut i64)->BigUint {
    let t=BigInt::from(a.clone())* *f+BigInt::from(b.clone())* *g;
    let divisor=BigInt::from(1u8)<<31usize;
    assert_eq!(&t%&divisor,BigInt::from(0));
    if t.sign()==Sign::Minus {*f= -*f; *g= -*g}
    (abs(&t)/divisor).to_biguint().unwrap()
}
#[derive(Default)]
struct Counts {
    rounds:u64, matrix_compositions:u64, old_rows:u64, old_finish_rows:u64,
    old_scale:u64,new_scale:u64,new_linear_products:u64,new_finish_products:u64,
    new_cancel_steps:u64,new_cancel_shifts:BTreeMap<u32,u64>,
    max_coefficient_bits:u64,max_finish_bits:u64,max_cancel_bits:u64,
    coefficient_word_hist:BTreeMap<usize,u64>, width_product_rows:u64,
    euclid_quotients:u64, finish_kind:String,
}
impl Counts {
    fn value(&self)->Value {json!({"rounds":self.rounds,"matrix_compositions":self.matrix_compositions,
      "old_modular_coefficient_rows":self.old_rows,"old_ordinary_finish_rows":self.old_finish_rows,
      "old_scale_field_multiplications":self.old_scale,"new_scale_field_multiplications":self.new_scale,
      "old_linear_word_products":8*(self.old_rows+self.old_finish_rows),
      "old_cancel_word_products":5*(self.old_rows+self.old_finish_rows),
      "new_coefficient_linear_word_products":self.new_linear_products,
      "new_finish_linear_word_products":self.new_finish_products,
      "new_cancel_word_products":5*self.new_cancel_steps,"new_cancel_steps":self.new_cancel_steps,
      "new_cancel_shifts":self.new_cancel_shifts,"max_coefficient_bits":self.max_coefficient_bits,
      "max_finish_bits":self.max_finish_bits,"max_cancel_intermediate_bits":self.max_cancel_bits,
      "coefficient_active_word_histogram":self.coefficient_word_hist,
      "active_width_row_visits":self.width_product_rows,"finish_Euclid_quotients":self.euclid_quotients,
      "finish_kind":self.finish_kind})}
}
fn old_update(t:BigInt,shift:u32,p:&BigInt,inv:u64)->BigInt {
    let radix=BigInt::from(1u8)<<shift as usize;
    let low=residue(t.clone(),&radix).to_u64_digits().1.first().copied().unwrap_or(0);
    let mask=if shift==64 {u64::MAX} else {(1u64<<shift)-1};
    let m=low.wrapping_mul(inv)&mask;
    let n=t+BigInt::from(m)*p;
    assert_eq!(&n%&radix,BigInt::from(0));
    let q=n/radix;
    assert!(q> -p && q<p*2);
    residue(q,p)
}
fn exact_update(c:&mut [BigInt;2],m:[i64;4],first:&mut bool,counts:&mut Counts) {
    if *first {
        assert_eq!(c[0],BigInt::from(1));assert_eq!(c[1],BigInt::from(0));
        *c=[BigInt::from(m[0]),BigInt::from(m[2])]; *first=false;
    } else {
        let widths=c.each_ref().map(|x|x.bits().div_ceil(64).max(1) as usize);
        counts.new_linear_products+=2*(widths[0]+widths[1]) as u64;
        counts.width_product_rows+=2;
        for n in widths {*counts.coefficient_word_hist.entry(n).or_default()+=2;}
        let old=c.clone();
        *c=[&old[0]*m[0]+&old[1]*m[1],&old[0]*m[2]+&old[1]*m[3]];
    }
    for a in c {counts.max_coefficient_bits=counts.max_coefficient_bits.max(a.bits());}
}
fn correct(c:&BigInt,k:u32,p:&BigInt,inv:u64,counts:&mut Counts)->BigInt {
    let mut a=abs(c);let mut left=k;
    assert!(a.bits()<=k as u64+64);
    let original=a.clone();
    while left>0 {
        let shift=left.min(64);let radix=BigInt::from(1u8)<<shift as usize;
        let low=(&a%&radix).to_u64_digits().1.first().copied().unwrap_or(0);
        let mask=if shift==64 {u64::MAX} else {(1u64<<shift)-1};
        let m=low.wrapping_mul(inv)&mask;
        let n=&a+BigInt::from(m)*p;
        counts.max_cancel_bits=counts.max_cancel_bits.max(n.bits());
        assert_eq!(&n%&radix,BigInt::from(0));
        a=n/radix;left-=shift;counts.new_cancel_steps+=1;
        *counts.new_cancel_shifts.entry(shift).or_default()+=1;
    }
    let radix=BigInt::from(1u8)<<k as usize;
    let difference=&a*&radix-original;
    assert_eq!(&difference%p,BigInt::from(0));
    let q=difference/p;assert!(q>=BigInt::from(0) && q<radix);
    assert!(a>=BigInt::from(0) && a<p*2);
    if a>=*p {a-=p;}
    if c.sign()==Sign::Minus && a!=BigInt::from(0) {a=p-a;}
    a
}
fn study<C:FpConfig>(x:&BigUint)->(Value,BigUint) {
    let prime=raw(C::P);let p=BigInt::from(prime.clone());let input=BigInt::from(x.clone());
    let mut a=x.clone();let mut b=prime.clone();let mut counts=Counts::default();
    let mut c=[BigInt::from(1),BigInt::from(0)];let mut d=[BigInt::from(0),BigInt::from(1)];
    let mut old=[BigInt::from(raw(C::R2)),BigInt::from(0)];
    let mut pending:Option<[i64;4]>=None;let mut first=true;let mut applied=0u32;
    let final_c;
    let old_result;
    loop {
        if a==BigUint::from(0u8) {
            assert_eq!(b,BigUint::from(1u8));
            if let Some(m)=pending.take() {
                old[1]=old_update(&old[0]*m[2]+&old[1]*m[3],31,&p,C::INV);
                counts.old_rows+=1;
                // Both exact rows are retained for identity checks; count only live final v.
                let before=counts.new_linear_products;exact_update(&mut c,m,&mut first,&mut counts);
                let charged=counts.new_linear_products-before;counts.new_linear_products-=charged/2;
                let previous=d.clone();d=[&previous[0]*m[0]+&previous[1]*m[1],&previous[0]*m[2]+&previous[1]*m[3]];
                applied+=31;
            }
            counts.finish_kind="outer_zero".into();final_c=c[1].clone();old_result=old[1].clone();break
        }
        let bits=a.bits().max(b.bits());
        if bits<=64 {
            if let Some(m)=pending.take() {
                let previous=old.clone();
                old=[old_update(&previous[0]*m[0]+&previous[1]*m[1],31,&p,C::INV),old_update(&previous[0]*m[2]+&previous[1]*m[3],31,&p,C::INV)];
                counts.old_rows+=2;exact_update(&mut c,m,&mut first,&mut counts);
                let previous=d.clone();d=[&previous[0]*m[0]+&previous[1]*m[1],&previous[0]*m[2]+&previous[1]*m[3]];applied+=31;
            }
            if b==BigUint::from(1u8) {
                counts.finish_kind="one_word_b_one".into();final_c=c[1].clone();old_result=old[1].clone();break
            }
            let(mut r0,mut r1)=(words(&a)[0],words(&b)[0]);
            let(mut f0,mut f1,mut g0,mut g1)=(1i128,0i128,0i128,1i128);
            loop {let q=r0/r1;let rem=r0%r1;counts.euclid_quotients+=1;
                if rem==0 {assert_eq!(r1,1);break}
                (r0,r1)=(r1,rem);(f0,f1)=(f1,f0-q as i128*f1);(g0,g1)=(g1,g0-q as i128*g1);
            }
            assert!(f1.unsigned_abs()<1u128<<63 && g1.unsigned_abs()<1u128<<63);
            assert_eq!(BigInt::from(a.clone())*f1+BigInt::from(b.clone())*g1,BigInt::from(1));
            let divisor=BigInt::from(1u8)<<applied as usize;
            final_c=&c[0]*f1+&c[1]*g1;
            let final_d=&d[0]*f1+&d[1]*g1;
            assert_eq!(&final_c*&input+final_d*&p,divisor);
            counts.new_finish_products=c.iter().map(|x|x.bits().div_ceil(64).max(1)).sum();
            counts.old_finish_rows=1;counts.old_scale=1;
            old_result=residue(old_update(&old[0]*f1+&old[1]*g1,64,&p,C::INV)*(BigInt::from(1u8)<<64usize),&p);
            counts.finish_kind="ordinary_Euclid".into();break
        }
        let (_,_,mut f,mut g,mut h,mut j)=matrix::zero_run_matrix(approximation(&a,bits),approximation(&b,bits));
        for (u,v) in [(f,g),(h,j)] {assert!(u.unsigned_abs()+v.unsigned_abs()<=1<<31);}
        let old_a=a.clone();a=normalized_row(&old_a,&b,&mut f,&mut g);b=normalized_row(&old_a,&b,&mut h,&mut j);
        assert!(a.bits().max(b.bits())<=bits);
        let det=f as i128*j as i128-g as i128*h as i128;assert_eq!(det.unsigned_abs(),1u128<<31);
        counts.rounds+=1;assert!(counts.rounds<=32);
        let m=if let Some(t)=pending.take() {
            counts.matrix_compositions+=1;
            Some([f*t[0]+g*t[2],f*t[1]+g*t[3],h*t[0]+j*t[2],h*t[1]+j*t[3]])
        }else{pending=Some([f,g,h,j]);None};
        if let Some(m)=m {
            for row in m.chunks(2) {assert!(row[0].unsigned_abs()+row[1].unsigned_abs()<=1<<62);}
            let previous=old.clone();old=[old_update(&previous[0]*m[0]+&previous[1]*m[1],62,&p,C::INV),old_update(&previous[0]*m[2]+&previous[1]*m[3],62,&p,C::INV)];
            counts.old_rows+=2;exact_update(&mut c,m,&mut first,&mut counts);
            let previous=d.clone();d=[&previous[0]*m[0]+&previous[1]*m[1],&previous[0]*m[2]+&previous[1]*m[3]];applied+=62;
            let scale=BigInt::from(1u8)<<applied as usize;
            assert_eq!(&c[0]*&input+&d[0]*&p,BigInt::from(a.clone())*&scale);
            assert_eq!(&c[1]*&input+&d[1]*&p,BigInt::from(b.clone())*&scale);
            for c in &c {assert!(abs(c)<=scale);}
        }
    }
    assert_eq!(applied as u64,31*counts.rounds);
    assert_eq!(residue(&final_c*&input-(BigInt::from(1u8)<<applied as usize),&p),BigInt::from(0));
    counts.max_finish_bits=final_c.bits();counts.new_scale=1;
    let inv=correct(&final_c,applied,&p,C::INV,&mut counts);
    assert_eq!(residue(&inv*&input,&p),BigInt::from(1));
    let r=BigInt::from(1u8)<<256usize;
    let result=residue(inv*&r*&r,&p).to_biguint().unwrap();
    assert_eq!(result,old_result.to_biguint().unwrap());
    let independent=x.modpow(&(&prime-BigUint::from(2u8)),&prime);
    assert_eq!(result,(&independent*(BigUint::from(1u8)<<512usize))%&prime);
    let native=Fp::<C>::from_montgomery(words(x)).inverse().unwrap();
    assert_eq!(raw(native.0),result);
    (counts.value(),result)
}
fn next(seed:&mut u64)->u64 {*seed^=*seed<<13;*seed^=*seed>>7;*seed^=*seed<<17;*seed}
fn toy()->usize {
    let mut cases=0;
    for p in [1019u64,1031] {
        let mut pinv=1u64;for _ in 0..6 {pinv=pinv.wrapping_mul(2u64.wrapping_sub(p.wrapping_mul(pinv)));}
        assert_eq!(p.wrapping_mul(pinv),1);let inv=pinv.wrapping_neg();
        for x in 1..p {
            let (mut a,mut b)=(x,p);let mut c=[BigInt::from(1),BigInt::from(0)];let mut k=0u32;
            while a!=0 {
                if a&1!=0 {
                    if a<b {std::mem::swap(&mut a,&mut b);c.swap(0,1);}
                    a-=b;let rhs=c[1].clone();c[0]-=rhs;
                }
                a>>=1;c[1]<<=1usize;k+=1;
                let pp=BigInt::from(p);let scale=BigInt::from(1u8)<<k as usize;
                assert_eq!(residue(&c[0]*x-BigInt::from(a)*&scale,&pp),BigInt::from(0));
                assert_eq!(residue(&c[1]*x-BigInt::from(b)*scale,&pp),BigInt::from(0));
            }
            assert_eq!(b,1);
            let actual=correct(&c[1],k,&BigInt::from(p),inv,&mut Counts::default());
            let(mut a,mut b,mut u,mut v)=(x as i128,p as i128,1i128,0i128);
            while b!=0 {let q=a/b;(a,b)=(b,a-q*b);(u,v)=(v,u-q*v);}
            assert_eq!(a,1);assert_eq!(actual,BigInt::from(u.rem_euclid(p as i128)));cases+=1;
        }
    }
    cases
}
fn field<C:FpConfig>(name:&str,seed:u64)->Value {
    let p=raw(C::P);let mut inputs=vec![("boundary",BigUint::from(1u8)),("boundary",&p-BigUint::from(1u8)),("boundary",&p-BigUint::from(2u8))];
    for bit in 1..254usize {let a=BigUint::from(1u8)<<bit;for x in [&a-BigUint::from(1u8),a.clone(),a+BigUint::from(1u8)] {if x<p {inputs.push(("power_neighbor",x))}}}
    let mut s=seed;for _ in 0..2048 {let x=raw([next(&mut s),next(&mut s),next(&mut s),next(&mut s)])%(&p-BigUint::from(1u8))+BigUint::from(1u8);inputs.push(("random",x));}
    let mut rows=Vec::new();let mut output=BigUint::from(0u8);
    for (i,(cohort,x)) in inputs.iter().enumerate() {let(c,r)=study::<C>(x);output^=r;rows.push(json!({"index":i,"cohort":cohort,"input_hex":x.to_str_radix(16),"counts":c}));}
    json!({"field":name,"inputs":inputs.len(),"seed":seed,"rows":rows,"output_xor_hex":output.to_str_radix(16)})
}
fn main() {
    assert!(!std::path::Path::new(&format!("{ROOT}/model.json")).exists());
    let toy_cases=toy();
    let fields=[field::<FqConfig>("Fq",0x6a34d059e02c179b),field::<FrConfig>("Fr",0x913bb2e6c047da51)];
    let v=json!({"mathematical_checks_passed":true,"new_native_backend_qualified":false,"timings_measured":false,"toy_dyadic_inverse_cases":toy_cases,"fields":fields,
    "limits":"Exact BigInt arithmetic and active-width product expressions are not native instructions, latency, fixed-stack qualification or a speed prediction. Coefficient companion d is oracle-only, not proposed runtime storage."});
    fs::write(format!("{ROOT}/model.json"),serde_json::to_vec_pretty(&v).unwrap()).unwrap();
    println!("Exact unit-seeded coefficient screen PASS: {} + {} new cases; original matrices and inverse outputs preserved",v["fields"][0]["inputs"],v["fields"][1]["inputs"]);
}
