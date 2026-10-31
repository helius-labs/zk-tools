#[cfg(test)]mod semantic_tests{
use super::*;
fn f(n:u64)->Fr{Fr::from(n)}
#[test]fn monic_commutative_products_match_field_valuations(){
 let mut e=Engine::new(16);let x=e.input(1);let y=e.input(2);let a=x.scale(f(3)).plus(&y.scale(f(2))).plus(&Form::constant(f(7)));let b=x.scale(f(2)).minus(&y).minus(&Form::constant(f(3)));let p=e.mul(&a,&b);let q=e.mul(&b,&a);let r=e.mul(&a.scale(f(7)),&b.scale(f(7).inverse().unwrap()));assert_eq!(p,q);assert_eq!(p,r);assert_eq!(e.stats.reused_products,2);
 for seed in 0..128{let v=[Fr::ONE,f(seed),f(seed*seed+19)];let expected=(f(3)*v[1]+f(2)*v[2]+f(7))*(f(2)*v[1]-v[2]-f(3));assert_eq!(e.evaluate(&p,&v).unwrap(),expected);}
}
#[test]fn deterministic_hints_scaled_inverse_involution_and_bit_prefix(){
 let mut e=Engine::new(32);let x=e.input(1);let inverse=e.inv0(&x);let inverse_again=e.inv0(&x);let scaled=e.inv0(&x.scale(f(7)));assert_eq!(inverse,inverse_again);assert_eq!(scaled,inverse.scale(f(7).inverse().unwrap()));let twice=e.inv0(&inverse);assert_eq!(twice,x);
 let bits=(0..256).map(|i|e.bit(&x,i)).collect::<Vec<_>>();for i in 0..64{assert_eq!(e.bit(&x,i),bits[i]);}assert_eq!(bits[254].value(),Some(Fr::ZERO));assert_eq!(bits[255].value(),Some(Fr::ZERO));
 let mut values=vec![Fr::ZERO,Fr::ONE,-Fr::ONE];for i in[1u64,63,127,191,253]{values.push(f(2).pow(&[i]));}for seed in 1..=24{let mut a=f(seed);for _ in 0..9{a=a.square()+f(seed*31+7);}values.push(a);}
 for a in values{let v=[Fr::ONE,a];assert_eq!(e.evaluate(&inverse,&v).unwrap(),a.inverse().unwrap_or(Fr::ZERO));assert_eq!(e.evaluate(&scaled,&v).unwrap(),(a*f(7)).inverse().unwrap_or(Fr::ZERO));let words=a.to_canonical();for i in 0..256{assert_eq!(e.evaluate(&bits[i],&v).unwrap(),if words[i/64]>>(i%64)&1==1{Fr::ONE}else{Fr::ZERO});}}
}
#[test]fn boolean_rules_include_zero_complements_and_scalars(){
 let mut e=Engine::new(32);let x=e.input(1);let b=e.bit(&x,0);let minus_one=b.minus(&Form::constant(Fr::ONE));let square=e.mul(&b,&b);assert_eq!(square,b);let zero=e.mul(&b,&minus_one);assert_eq!(zero.value(),Some(Fr::ZERO));let c=e.mul(&minus_one,&minus_one);assert_eq!(c,minus_one.scale(-Fr::ONE));
 for scale in[-Fr::ONE,f(2),f(17)]{let a=minus_one.scale(scale);let inverse=e.inv0(&a);let outputs=(0..8).map(|i|e.bit(&a,i)).collect::<Vec<_>>();for bit in[0u64,1]{let v=[Fr::ONE,f(bit)];let a=(f(bit)-Fr::ONE)*scale;assert_eq!(e.evaluate(&inverse,&v).unwrap(),a.inverse().unwrap_or(Fr::ZERO));let words=a.to_canonical();for i in 0..8{assert_eq!(e.evaluate(&outputs[i],&v).unwrap(),if words[0]>>i&1==1{Fr::ONE}else{Fr::ZERO});}}}
}
#[test]fn guarded_division_does_not_cancel_zero_case(){
 let mut e=Engine::new(16);let o=e.input(1);let r=e.input(2);let own=Form::constant(f(17));let d=e.assign(0,&[own.clone(),o.clone(),r.clone()],f(13));let swapped=e.assign(1,&[o.clone(),own.clone(),r.clone()],f(13));let scaled=e.assign(0,&[own,o.scale(f(7)),r.scale(f(7))],f(13));assert_eq!(d,swapped);assert_eq!(d,scaled);
 for other in[Fr::ZERO,Fr::ONE,-Fr::ONE,f(7)]{for result in[Fr::ZERO,Fr::ONE,f(19)]{let actual=e.evaluate(&d,&[Fr::ONE,other,result]);if other==Fr::ZERO&&result!=Fr::ZERO{assert!(actual.is_err());}else{let expected=if other==Fr::ZERO{Fr::ZERO}else{(result*other.inverse().unwrap()-f(17))*f(13)};assert_eq!(actual.unwrap(),expected);}}}
 let mut e=Engine::new(8);let x=e.input(1);let z=Form::constant(Fr::ZERO);let quotient=e.assign(0,&[z,x.clone(),x],Fr::ONE);assert!(quotient.value().is_none());assert_eq!(e.evaluate(&quotient,&[Fr::ONE,Fr::ZERO]).unwrap(),Fr::ZERO);assert_eq!(e.evaluate(&quotient,&[Fr::ONE,Fr::ONE]).unwrap(),Fr::ONE);
}
fn semantic_image()->Vec<u8>{
 let mut b=vec![];fn words(b:&mut Vec<u8>,a:&[u32]){for w in a{b.extend_from_slice(&w.to_le_bytes());}}fn field(b:&mut Vec<u8>,v:Fr){for w in v.0{b.extend_from_slice(&w.to_le_bytes());}}
 words(&mut b,&[0x3150535a,2,15,3,1,4,15,8,5,1,0,0,8]);for i in 0..15{words(&mut b,&[i]);}words(&mut b,&[3,4,5,8,11,12,13,14]);for c in[Fr::ZERO,Fr::ONE,f(2),-Fr::ONE,-f(2)]{field(&mut b,c);}words(&mut b,&[1,1]);for(kind,a,z)in[(0,3,4),(0,4,5),(1,5,8),(1,8,11)]{words(&mut b,&[1,kind,0,1,a,z]);}
 for(row,l,r,w,side)in[(0,1,2,11,2),(1,2,1,12,2),(2,5,5,13,2),(3,0,1,14,0)]{words(&mut b,&[0,row,0,0,0]);if side==2{words(&mut b,&[1,l,0,1,r,0,0,0,0]);}else{words(&mut b,&[0,0,0,1,1,0,1,1,0]);}words(&mut b,&[1,side,w,1]);field(&mut b,Fr::ONE);}
 b
}
#[test]fn complete_parser_finds_known_aliases_and_rejects_corruption(){
 let b=semantic_image();let v=census(&b).unwrap();assert_eq!(v["semantic_query_classes"]["A"]["proportional_nonzero_classes"],9);assert_eq!(v["semantic_query_classes"]["A"]["potential_terms_removed_proportional_or_zero"],6);assert_eq!(v["semantic_query_classes"]["B"]["proportional_nonzero_classes"],4);assert_eq!(v["semantic_engine"]["reused_products"],1);assert_eq!(v["semantic_engine"]["reused_inverse_hints"],1);assert_eq!(v["semantic_engine"]["reused_bit_outputs"],3);assert_eq!(v["semantic_engine"]["Boolean_square_rules"],1);assert_eq!(v["semantic_engine"]["guarded_division_requests"],1);
 for cut in[0,4,51,b.len()-1]{assert!(census(&b[..cut]).is_err());}let mut bad=b;bad.push(0);assert!(census(&bad).is_err());
}
}
