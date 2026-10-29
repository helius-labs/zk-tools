use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175";
fn main(){
let root=Path::new(R);
let mut adapter=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/signed-final-sub174/caller_probe.rs").unwrap();
let anchor="impl CallerG2 {\n";
assert_eq!(adapter.matches(anchor).count(),1);
adapter=adapter.replace(anchor,&format!("{anchor}{}",r#"
    pub fn new_signed(n: usize, seed: u64) -> Self {
        let s = Fixture::new(n, true, seed);
        assert!(s.scratch.direct && s.scratch.fq2.canonical
            && s.scratch.fq2.vector_inverse && s.scratch.fq2.prefix_enabled);
        Self(s)
    }
    pub fn with_borrowed<R>(&mut self, action: impl FnOnce(&mut BorrowedG2<'_>) -> R) -> R {
        let f = &mut self.0;
        let mut caller = BorrowedG2 {
            source: &f.source, buckets: &mut f.buckets,
            batch: f.batch.iter().map(|(i,p)| (*i,p)).collect(),
            original: &f.batch, scratch: &mut f.scratch,
            coefficients: &f.coefficients, steps: &f.steps, signed: f.signed,
        };
        action(&mut caller)
    }
"#));
let anchor="#[cfg(test)]\nmod tests {";
assert_eq!(adapter.matches(anchor).count(),1);
adapter=adapter.replace(anchor,&format!("{}{anchor}",r#"
pub struct BorrowedG2<'a> {
    source: &'a [Affine<G2Config>], buckets: &'a mut [Affine<G2Config>],
    batch: Vec<(u32,&'a Affine<G2Config>)>, original: &'a [(u32,Affine<G2Config>)],
    scratch: &'a mut BatchAdd<G2Config>, coefficients: &'a [u64], steps: &'a [u64], signed: bool,
}
impl Caller for BorrowedG2<'_> {
    fn reset(&mut self) { self.buckets.copy_from_slice(self.source) }
    fn run(&mut self,n:usize) {
        for _ in 0..n {
            let s=black_box(&mut *self);
            s.scratch.apply_signed_borrowed_g2(s.buckets,&s.batch)
        }
    }
    fn check(&self,n:usize) { check(self.buckets,self.coefficients,self.steps,self.original,self.signed,n) }
    fn bytes(&self)->Vec<u8> { format!("{:?}",self.buckets).into_bytes() }
    fn capacity_bytes(&self)->usize {
        self.scratch.fq2.prefix_records.capacity()*std::mem::size_of::<[FqBatch;2]>()
        +self.scratch.fq2.tail.capacity()*std::mem::size_of::<[Fq2;5]>()
    }
    fn reference_bytes(&self)->usize { self.batch.capacity()*std::mem::size_of::<(u32,&Affine<G2Config>)>() }
}
"#));
let i=adapter.rfind('}').unwrap();
adapter.insert_str(i,r#"
    #[test]
    fn borrowed_signed_g2_generator_oracle_tails_and_reuse() {
        for n in LENGTHS {
            let mut f=CallerG2::new_signed(n,31);
            f.with_borrowed(|f| {
                for rounds in [1,3,17] { f.reset();f.run(rounds);f.check(rounds); }
                assert_eq!(f.reference_bytes(),n*std::mem::size_of::<(u32,&Affine<G2Config>)>());
            });
        }
    }
"#);
fs::write(root.join("caller_probe.rs"),adapter).unwrap();
let mut probe=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss/probe.rs").unwrap();
probe=probe.split("#[cfg(test)]\nmod tests {").next().unwrap().to_owned();
let anchor="                    \"g2-prefix\" => {";
assert_eq!(probe.matches(anchor).count(),1);
probe=probe.replace(anchor,&format!("{}{anchor}",r#"
                    "g2-borrowed-signed" => {
                        let mut f=CallerG2::new_signed(n,17);
                        f.with_borrowed(|b| worker(b,cpu,rounds,prime,g))
                    }
"#));
probe=probe.replace("[8192, 262144]","[32, 8192, 262144]");
let a=probe.find("    assert!(\n        (a[1]").unwrap();
let b=probe[a..].find("    let result").unwrap()+a;
probe.replace_range(a..b,r#"    assert!((a[1]=="g1-borrowed-signed" && n==200)
        || (a[1]=="g2-borrowed-signed" && [200,352].contains(&n)));
"#);
fs::write(root.join("probe.rs"),probe).unwrap();
fs::copy("/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss/perf.rs",root.join("perf.rs")).unwrap();
let path=root.join("setup.rs");let s=fs::read_to_string(&path).unwrap();
assert!(s.contains("[\"gate_passed\"],true"));
fs::write(path,s.replace("[\"gate_passed\"],true","[\"passed\"],true")).unwrap();
}
