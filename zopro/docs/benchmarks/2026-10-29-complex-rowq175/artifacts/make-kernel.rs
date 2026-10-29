use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/complex-rowq175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("kernel.rs").exists());
    let old =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/row-quotient175/kernel.rs").unwrap();
    let start = old
        .find("    #[inline(always)]\n    fn product_rowq<")
        .unwrap();
    let mut body = old[start..].to_owned();
    let from = "fn product_rowq<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self";
    let to="fn product_pairrow<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self, rowq:&[__m512i;5]) -> Self";
    assert_eq!(body.matches(from).count(), 1);
    body = body.replace(from, to);
    let a = body.find("            let a0i=").unwrap();
    let b = body[a..].find("            macro_rules! row {").unwrap() + a;
    body.replace_range(a..b, "");
    let prefix = fs::read_to_string(r.join("pair-prefix.rs")).unwrap();
    let tests = fs::read_to_string(r.join("field-tests.rs")).unwrap();
    fs::write(r.join("kernel.rs"), format!("{prefix}{body}\n{tests}")).unwrap();
    println!("Built joint quotient plan plus unchanged two fused CIOS reducers;315IFMA templates");
}
