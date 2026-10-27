use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/gaussian-fr-r130";
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"missing/duplicate {a}");*s=s.replacen(a,b,1);}
fn range(s:&mut String,start:&str,end:&str,replacement:&str){let a=s.find(start).unwrap();let b=a+s[a..].find(end).unwrap();s.replace_range(a..b,replacement);}
fn main(){let r=Path::new(R);let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-gaussian-fr/artifacts/src/native.rs").unwrap();let mut s=old.clone();
 replace(&mut s,"radix 2^128.","radix 2^130; canonical coordinates <=2^126+3b.");replace(&mut s,"const TOP:u64=(1u64<<24)-1;","const TOP:u64=(1u64<<26)-1;");replace(&mut s,"splat(1<<23)","splat(1<<25)");
 let a=s.find("#[inline(always)] unsafe fn high(").unwrap();let b=a+s[a..].find("#[inline(always)] unsafe fn greater(").unwrap();let body=s[a..b].to_owned();let next=body.replace("::<24>","::<26>").replace("::<28>","::<26>").replace("(1u64<<48)-1","MASK");s.replace_range(a..b,&next);
 range(&mut s,"#[inline(always)] unsafe fn greater(","impl G8 {","#[inline(always)] unsafe fn overflow(a:D)->(u8,u8){\n let q=_mm512_srai_epi64::<60>(_mm512_slli_epi64::<38>(a[2]));\n (_mm512_cmpgt_epi64_mask(q,zero()),_mm512_cmplt_epi64_mask(q,splat(u64::MAX)))\n}\n");
 range(&mut s," #[inline(always)] pub unsafe fn new("," #[inline(always)] unsafe fn balance(",r#" #[inline(always)] pub unsafe fn new(values:[(i128,i128);8])->Self{
  let load=|part:usize|{let words:[[u64;8];3]=std::array::from_fn(|j|std::array::from_fn(|k|{let n=if part==0{values[k].0}else{values[k].1};let v=n as u128;let w=(v>>(52*j))as u64;if j==2{(w&((1<<24)-1))|if n<0{3<<24}else{0}}else{w&MASK}}));words.map(|w|_mm512_loadu_si512(w.as_ptr().cast()))};Self{re:load(0),im:load(1)}
 }
 pub unsafe fn get(self)->[(i128,i128);8]{let unpack=|d:D|{let mut w=[[0u64;8];3];for j in 0..3{_mm512_storeu_si512(w[j].as_mut_ptr().cast(),d[j]);}std::array::from_fn::<_,8,_>(|k|{let x=(w[0][k]as u128|((w[1][k]as u128)<<52)|(((w[2][k]&((1<<24)-1))as u128)<<104))as i128;assert_eq!(w[2][k]>>24,if x<0{3}else{0},"not a signed128 coordinate");x})};let a=unpack(self.re);let b=unpack(self.im);std::array::from_fn(|i|(a[i],b[i]))}
"#);
 range(&mut s," #[inline(always)] unsafe fn balance("," #[inline(always)] pub unsafe fn rot(",r#" #[inline(always)] unsafe fn balance<const STEPS:usize>(mut self)->Self{
  let a=digits(A);let b=digits(B);
  for _ in 0..STEPS{let(pos,neg)=overflow(self.re);masked_sub(&mut self.re,a,pos);masked_add(&mut self.re,a,neg);masked_sub(&mut self.im,b,pos);masked_add(&mut self.im,b,neg);self.re=normalize3(self.re);}
  self.im=normalize3(self.im);
  for _ in 0..STEPS{let(pos,neg)=overflow(self.im);masked_add(&mut self.re,b,pos);masked_sub(&mut self.re,b,neg);masked_sub(&mut self.im,a,pos);masked_add(&mut self.im,a,neg);self.im=normalize3(self.im);}
  self.re=normalize3(self.re);self
 }
 #[inline(always)] pub unsafe fn add(self,b:Self)->Self{self.add_lazy(b).balance::<1>()}
 #[inline(always)] pub unsafe fn sub(self,b:Self)->Self{self.sub_lazy(b).balance::<1>()}
 /// Intermediate linear coordinates may be larger than signed128, bounded below R/2.
 #[inline(always)] pub unsafe fn add_lazy(self,b:Self)->Self{Self{re:add3(self.re,b.re),im:add3(self.im,b.im)}}
 #[inline(always)] pub unsafe fn sub_lazy(self,b:Self)->Self{Self{re:sub3(self.re,b.re),im:sub3(self.im,b.im)}}
 /// The complete all-sum plus recurrence constant is bounded by5H.
 #[inline(always)] pub unsafe fn finish_linear(self)->Self{self.balance::<3>()}
 /// Left coordinates<=4H; canonical twiddle coordinates<=H.
 #[inline(always)] pub unsafe fn mul_twiddle(self,b:Self)->Self{self.mul(b).balance::<1>()}
"#);
 replace(&mut s,"mul_low(tr,digits(KRE))","mul_low(tr,KRE.map(|v|splat(v)))");replace(&mut s,"mul_low(ti,digits(KIM))","mul_low(ti,KIM.map(|v|splat(v)))");replace(&mut s,"mul_low(add3(tr,ti),digits(KRE.wrapping_add(KIM)))","mul_low(add3(tr,ti),KSUM.map(|v|splat(v)))");
 replace(&mut s,"Self{re:add3(re,normalize3(c0)),im:add3(im,normalize3(c1))}.balance()","Self{re:add3(re,normalize3(c0)),im:add3(im,normalize3(c1))}");
 let test=r#"
 #[test]fn lazy_twiddle_and_linear_finish_match_exact_model(){let p=Params::new();let h=p.bound.to_i128().unwrap();let v=[0,1,-1,h,-h,HALF as i128,-(HALF as i128)];unsafe{for start in 0..v.len(){let inputs:[[(i128,i128);8];5]=std::array::from_fn(|j|std::array::from_fn(|k|(v[(start+j+k)%v.len()],v[(start+j*3+k+1)%v.len()])));let g=inputs.map(|x|G8::new(x));let linear=g[0].add_lazy(g[1]).add_lazy(g[2]).add_lazy(g[3]);let product=linear.mul_twiddle(g[4]).get();let finish=linear.add_lazy(g[4]).finish_linear().get();for k in 0..8{let z=(0..4).fold(Z::new(0,0),|z,j|z.add(&Z::new(inputs[j][k].0,inputs[j][k].1)));let b=Z::new(inputs[4][k].0,inputs[4][k].1);let expected=p.twiddle(&z,&b);assert_eq!(product[k],(expected.re.to_i128().unwrap(),expected.im.to_i128().unwrap()));let expected=p.finish(z.add(&b));assert_eq!(finish[k],(expected.re.to_i128().unwrap(),expected.im.to_i128().unwrap()));}}}}
"#;
 let end=s.rfind('}').unwrap();s.insert_str(end,test);fs::write(r.join("src/native.rs"),s).unwrap();println!("Prepared fixedR130 native invariant, power-of-two corrections and lazy APIs; priorR128source retained in Git");}
