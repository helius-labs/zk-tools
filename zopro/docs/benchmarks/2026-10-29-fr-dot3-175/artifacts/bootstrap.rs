use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fr-dot3-175";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/fq-fips175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let old = fs::read_to_string(format!("{OLD}/native.rs")).unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let source = format!(
        "{}{}\n{}\n{}",
        &old[..a],
        fs::read_to_string(r.join("setup.rs")).unwrap(),
        fs::read_to_string(r.join("code-audit.rs")).unwrap(),
        &old[b..]
    )
    .replace(OLD, R);
    fs::write(r.join("native.rs"), source).unwrap();
}
