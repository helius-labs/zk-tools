use std::fs;
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let path = "/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/msm/g2_prefix.rs";
    let mut s = fs::read_to_string(path).unwrap();
    s = s[..s.find("#[cfg(test)]").unwrap()].to_string();
    once(
        &mut s,
        "//! G2 inversion prefix/norm records; native experimental path.",
        "//! Signed borrowed G2 prefix, retaining accepted arithmetic and scratch.",
    );
    once(
        &mut s,
        "batch:&[(u32,Affine<C>)]",
        "batch:&[(u32,&Affine<C>)]",
    );
    assert_eq!(s.matches("batch[k+i].0 as usize").count(), 2);
    s = s.replace(
        "batch[k+i].0 as usize",
        "(batch[k+i].0&0x7fffffff) as usize",
    );
    once(&mut s,"            let (index, b) = batch[k];\n            let a = buckets[index as usize];","            let (encoded,b)=batch[k];let index=encoded&0x7fffffff;let b=if encoded>>31!=0{-*b}else{*b};\n            let a = buckets[index as usize];");
    once(
        &mut s,
        "            buckets[batch[k].0 as usize] = Affine::new(x, y);",
        "            buckets[(batch[k].0&0x7fffffff) as usize] = Affine::new(x, y);",
    );
    once(
        &mut s,
        "let b:[Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);",
        "let b:[&Affine<C>;LANES]=core::array::from_fn(|i|batch[k+i].1);",
    );
    once(&mut s,"let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(b.map(|p|p.y));","let bx=fq2_pack(b.map(|p|p.x));let by=fq2_pack(core::array::from_fn(|i|if batch[k+i].0>>31!=0{-b[i].y}else{b[i].y}));");
    let suffix = r#"
impl<C:CurveConfig<Base=Fq2>>BatchAdd<C>{
 pub(crate)fn borrowed_g2_enabled(&self)->bool{self.direct&&self.fq2.canonical&&self.fq2.vector_inverse&&self.fq2.prefix_enabled&&crate::simd::cpu_option("ZOPRO_CPU_PACKED")}
 pub(crate)fn apply_signed_borrowed_g2(&mut self,buckets:&mut[Affine<C>],batch:&[(u32,&Affine<C>)]){
  if self.borrowed_g2_enabled(){return apply::<C,true>(buckets,batch,&mut self.fq2)}
  let owned:Vec<_>=batch.iter().map(|&(id,p)|(id&0x7fffffff,if id>>31!=0{-*p}else{*p})).collect();self.apply(buckets,&owned);
 }
}
#[cfg(test)]mod tests{
 use super::*;use zopro_bn254::G2Config;
 #[test]fn signed_borrowed_prefix_tails_and_reuse(){
 let g=Projective::<G2Config>::generator();for n in[0usize,1,7,8,9,15,16,32,200,352,511]{let size=n.max(1).next_power_of_two();
 let initial=Projective::<G2Config>::batch_to_affine(&(0..size).map(|i|g*Fr::from((i+12345)as u64)).collect::<Vec<_>>());let incoming=Projective::<G2Config>::batch_to_affine(&(0..n).map(|i|g*Fr::from((i+900001)as u64)).collect::<Vec<_>>());
 for sign_mode in 0..3{let batch:Vec<_>=incoming.iter().enumerate().map(|(i,p)|(((i*37%size)as u32)|if sign_mode==1||sign_mode==2&&i%3==0{1<<31}else{0},p)).collect();let mut actual=initial.clone();let mut expected=initial.clone();let mut s=Fq2AddScratch{canonical:true,vector_inverse:true,..Default::default()};
 for _ in 0..3{for&(id,p)in &batch{let i=(id&0x7fffffff)as usize;let b=if id>>31!=0{-*p}else{*p};assert_ne!(expected[i].x,b.x);expected[i]=(expected[i]+b).to_affine();}apply::<G2Config,true>(&mut actual,&batch,&mut s);assert_eq!(actual,expected);}
 }}
 }
}
"#;
    s.push_str(suffix);
    fs::write("g2_borrowed_prefix.rs", s).unwrap();
    println!("Generatedborrowedprefixfromacceptedbodywithsignedreferences;fieldformulasunchanged");
}
