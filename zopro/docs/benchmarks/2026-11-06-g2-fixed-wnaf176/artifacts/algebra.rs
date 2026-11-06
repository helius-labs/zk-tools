// Independent integer construction of fixed-block odd digits with skew.
// Primary reference: bitcoin-core/secp256k1, secp256k1_wnaf_fixed.
// No curve code, lookup table, field implementation or external C is used.
const WIDTH:usize=12;
const WINDOWS:usize=126usize.div_ceil(WIDTH);
const RADIX:i128=1<<WIDTH;
#[derive(Debug,Clone,Copy)]
struct Encoding{digits:[i16;WINDOWS],skew:i16}
fn encode(m:u128)->Encoding{
 assert!(m<1u128<<126);
 let mut result=Encoding{digits:[0;WINDOWS],skew:0};
 if m==0{return result;}
 result.skew=i16::from(m&1==0);
 let chunk=|w:usize|((m>>(WIDTH*w))&4095)as i16;
 let highest=(0..WINDOWS).rev().find(|&w|chunk(w)!=0).unwrap();
 result.digits[0]=chunk(0)+result.skew;
 for w in 1..=highest{
  let next=chunk(w);
  if next&1==0{result.digits[w-1]-=RADIX as i16;result.digits[w]=next+1;}else{result.digits[w]=next;}
  if w>=2{
   let middle=result.digits[w-1];let lower=result.digits[w-2];
   if middle==1&&lower<0{result.digits[w-2]+=RADIX as i16;result.digits[w-1]=0;}
   else if middle== -1&&lower>0{result.digits[w-2]-=RADIX as i16;result.digits[w-1]=0;}
  }
 }
 result
}
fn check(m:u128){
 let value=encode(m);
 assert!(value.skew==0||value.skew==1);
 for d in value.digits{assert!(d==0||(d%2!=0&&d.unsigned_abs()<4096));}
 let sum=value.digits.iter().rev().try_fold(0i128,|a,&d|a.checked_mul(RADIX)?.checked_add(i128::from(d))).unwrap();
 assert_eq!(sum-i128::from(value.skew),m as i128);
 for sign in [-1i128,1]{let signed=value.digits.iter().enumerate().map(|(w,&d)|sign*i128::from(d)*(1i128<<(WIDTH*w))).sum::<i128>()-sign*i128::from(value.skew);assert_eq!(signed,sign*m as i128);}
 if m==0{assert_eq!(value.digits,[0;WINDOWS]);assert_eq!(value.skew,0);}
}
#[test]fn fixed_odd_exhaustive_small_and_all_component_boundaries(){
 for m in 0..65536{check(m);}
 for bit in 0..126{let p=1u128<<bit;for m in[p-1,p,(p+1).min((1<<126)-1)]{check(m);}}
 check((1<<126)-1);check((1<<126)-2);
}
#[test]fn fixed_odd_full_width_random_signs_and_carry_patterns(){
 let mut state=0x6a09e667f3bcc908bb67ae8584caa73bu128;
 for i in 0..50000{state^=state<<23;state^=state>>17;state^=state<<26;let m=state&((1<<126)-1);check(m);if i<11{let mut pattern=0u128;for w in 0..WINDOWS{let word=match(w+i)%4{0=>1,1=>4095,2=>0,_=>2};pattern|=(word as u128)<<(WIDTH*w);}check(pattern&((1<<126)-1));}}
}
