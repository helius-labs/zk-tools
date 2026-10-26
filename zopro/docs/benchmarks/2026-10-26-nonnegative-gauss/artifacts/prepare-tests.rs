use std::{fs,path::Path};
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"missing/duplicate {a}");*s=s.replace(a,b);}
fn main(){let r=Path::new("/tmp/zopro-nonnegative-gauss");let mut s=fs::read_to_string("/tmp/zopro-streamed-complex-b/probe-tests.rs").unwrap();
 let a=s.find("    fn model(").unwrap();let b=s.find("    macro_rules! lane_tests").unwrap();s.replace_range(a..b,&fs::read_to_string(r.join("row-model.rs.txt")).unwrap());
 s=s.replace("complex_product_streamed","complex_product_gauss").replace("EXACT_STREAMED","EXACT_GAUSS").replace("ARK_STREAMED","ARK_GAUSS").replace("exact_signed_products_and_coefficients","exact_positive_products_and_coefficients");
 replace(&mut s,"let previous=Vector::complex_product_shared(&v[0],&v[1],&v[2],&v[3]);assert_eq!(gd,previous.each_ref().map(raw));","");
 replace(&mut s,"let real=BigInt::from(&a0*&b0)-BigInt::from(&a1*&b1);let imag=&a0*&b1+&a1*&b0;","let common=(&a0+&a1)*&b0;let real=&common+((&p<<1)-&a1)*(&b0+&b1);let imag=&common+&a0*(&b1+(&p<<1)-&b0);assert_eq!(BigInt::from(real.clone()),BigInt::from(&a0*&b0)-BigInt::from(&a1*&b1)+BigInt::from((&p<<1)*(&b0+&b1)));assert_eq!(imag,&a0*&b1+&a1*&b0+(&p<<1)*&a0);for t in [&real,&imag]{assert!(*t<=BigUint::from(12u8)*&p*&p);}");
 replace(&mut s,"[(0,real,true),(1,BigInt::from(imag),false)]","[(0,BigInt::from(real),false),(1,BigInt::from(imag),false)]");
 replace(&mut s,"        let mut state = 0x706861736564334du64;","        for bits in 0..16 {out.push(core::array::from_fn(|j|if (bits>>j)&1==0{BigUint::from(0u8)}else{p<<1}));}\n        let mut state = 0x706861736564334du64;");
 fs::write(r.join("probe-tests.rs"),s).unwrap();println!("Prepared independent positive integer/CIOS and Arkworks oracles with16additional0/2p corners perfield");}
