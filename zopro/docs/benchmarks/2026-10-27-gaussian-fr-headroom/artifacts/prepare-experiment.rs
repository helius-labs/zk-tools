use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/gaussian-fr-r130";
fn replace(s:&mut String,a:&str,b:&str){assert_eq!(s.matches(a).count(),1,"{a}");*s=s.replacen(a,b,1);}
fn main(){let r=Path::new(R);let base=Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-gaussian-fr/artifacts/src");let mut s=fs::read_to_string(base.join("experiment.rs")).unwrap();
 let pos=s.find("pub struct Input {").unwrap();s.insert_str(pos,r#"
// Range contracts: canonical input/output coordinates<=H; inner linear
// operands<=4H, all-sum plus recurrence constant<=5H. See model130.rs.
#[inline(never)]unsafe fn fft_lazy(mut x:[G8;4],w:G8,w2:G8,w3:G8,c:G8,n:usize)->[G8;4]{
 for _ in 0..n {
  let s0=x[0].add_lazy(x[2]);let s1=x[1].add_lazy(x[3]);
  let d0=x[0].sub_lazy(x[2]);let d1=x[1].sub_lazy(x[3]).rot();
  let a=s0.add_lazy(s1).add_lazy(c).finish_linear();
  let b=s0.sub_lazy(s1).mul_twiddle(w2);
  let next_c=d0.add_lazy(d1).mul_twiddle(w);
  let d=d0.sub_lazy(d1).mul_twiddle(w3);
  x=[a,b,next_c,d];
 }
 std::hint::black_box(x)
}

"#);
 let begin=s.find("impl Gaussian {").unwrap();let end=begin+s[begin..].find("\ntype Matrix=").unwrap();let mut block=s[begin..end].to_owned();
 replace(&mut block,"direct:bool,n:usize)","direct:bool,n:usize,lazy:bool)");
 replace(&mut block,"let x=std::hint::black_box(self.x);match pattern", "let x=std::hint::black_box(self.x);if pattern==\"butterfly\"&&lazy{return unsafe{fft_lazy(x,self.w,self.w2,self.w3,self.c,n)}.into();}match pattern");s.replace_range(begin..end,&block);
 replace(&mut s,"g.execute(p,direct,n)","g.execute(p,direct,n,false)");
 replace(&mut s,"assert_eq!(gaussian,want,\"gaussian {p} n{n} direct{direct}\");", "assert_eq!(gaussian,want,\"gaussian {p} n{n} direct{direct}\");if p==\"butterfly\"{let lazy:Vec<_>=g.execute(p,true,n,true).into_iter().map(|v|i.decode(v)).collect();assert_eq!(lazy,want,\"lazy butterfly n{n}\");}");
 fs::write(r.join("src/experiment.rs"),s).unwrap();let mut s=fs::read_to_string(base.join("bin/probe.rs")).unwrap();
 replace(&mut s,"let direct=mode.ends_with(\"r4\");","let direct=mode.ends_with(\"r4\")||mode.contains(\"r4-\");let lazy=mode.ends_with(\"-lazy\");");
 replace(&mut s,"\"gaussian-r2\",\"gaussian-r4\"","\"gaussian-r2\",\"gaussian-r4\",\"gaussian-r4-eager\",\"gaussian-r4-lazy\"");
 replace(&mut s,"gaussian.execute(pattern,direct,2000)","gaussian.execute(pattern,direct,2000,lazy)");replace(&mut s,"gaussian.execute(pattern,direct,n)","gaussian.execute(pattern,direct,n,lazy)");
 fs::write(r.join("src/bin/probe.rs"),s).unwrap();println!("Prepared complete lazy butterfly and exact unchanged scalar/matrix oracle; ordinary arithmetic retained");}
