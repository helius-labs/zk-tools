use std::{fs, path::Path};
fn main() {
    let r = Path::new("/home/ubuntu/src/zopro-perf-work/g2-gls4-census177");
    assert!(!r.join("flow.rs").exists());
    let source = fs::read_to_string(r.join("native.rs")).unwrap();
    let a = source.find("fn setup() {").unwrap();
    fs::write(
        r.join("flow.rs"),
        format!(
            "{}{}",
            &source[..a],
            fs::read_to_string(r.join("flow-body.rs")).unwrap()
        ),
    )
    .unwrap();
}
