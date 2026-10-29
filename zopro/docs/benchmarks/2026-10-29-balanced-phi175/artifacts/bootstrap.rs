use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-phi175";
fn main() {
    let old = fs::read_to_string("/home/ubuntu/src/zopro-perf-work/balanced-ordinary175/native.rs")
        .unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(").unwrap();
    let source = format!("{}include!(\"setup.rs\");\n{}", &old[..a], &old[b..])
        .replace("balanced-ordinary175", "balanced-phi175");
    assert!(!Path::new(R).join("native.rs").exists());
    fs::write(Path::new(R).join("native.rs"), source).unwrap();
}
