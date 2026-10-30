use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/r256-truncated-caller175";
fn main() {
    let r=Path::new(R);assert!(!r.join("field.rs").exists());
    let old=Path::new("/home/ubuntu/src/zopro-perf-work/early-x175");
    for f in ["caller_probe.rs","probe.rs","perf.rs"] { fs::copy(old.join(f),r.join(f)).unwrap(); }
    let patch=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-24-truncated-redc-component/evidence/source.patch").unwrap();
    let part=patch.split("+++ candidate/crates/zopro-bn254/src/ifma8.rs").nth(1).unwrap();
    let at=part.find("+    // -p^-1 modulo the full Montgomery radix").unwrap();
    let body:String=part[at..].lines().take_while(|l|l.starts_with('+')).map(|l|format!("{}\n",&l[1..])).collect();
    assert!(body.contains("fn product_truncated<const FUSED: bool>"));
    assert!(!body.contains("fn difference_truncated"));
    fs::write(r.join("inherited-helper.txt"),&body).unwrap();
    fs::write(r.join("field.rs"),format!("// Exact inherited arithmetic body; the new dispatch is in the parent.\nimpl<C:FpConfig,const MIXED:bool> Ifma8<C,MIXED> {{\n{body}}}\n{}",fs::read_to_string(r.join("field-tests.rs")).unwrap())).unwrap();
    let native=fs::read_to_string(old.join("native.rs")).unwrap().replace("/home/ubuntu/src/zopro-perf-work/early-x175",R).replace("zopro_early_x","zopro_r256_truncated");
    fs::write(r.join("native.rs"),native).unwrap();
}
