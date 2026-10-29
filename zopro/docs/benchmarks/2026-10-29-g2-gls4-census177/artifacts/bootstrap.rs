use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-census177";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let old = fs::read_to_string(format!("{P}/native.rs")).unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let mut prefix = old[..a].replace(P, R);
    prefix.push_str(&format!("const PARENT:&str=\"{P}\";\n"));
    let out = format!(
        "{}{}\n{}\n{}",
        prefix,
        fs::read_to_string(r.join("setup.rs")).unwrap(),
        fs::read_to_string(r.join("code-audit.rs")).unwrap(),
        &old[b..]
    );
    fs::write(r.join("native.rs"), out).unwrap();
}
