use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/service-trace175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("flow.rs").exists());
    let n = fs::read_to_string(r.join("native.rs")).unwrap();
    let a = n.find("fn setup() {").unwrap();
    fs::write(
        r.join("flow.rs"),
        format!(
            "{}{}",
            &n[..a],
            fs::read_to_string(r.join("flow-body.rs")).unwrap()
        ),
    )
    .unwrap();
}
