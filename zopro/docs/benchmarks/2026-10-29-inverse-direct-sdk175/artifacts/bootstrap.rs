use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175";
fn main() {
    let root = Path::new(R);
    assert!(!root.join("native.rs").exists());
    let old = fs::read_to_string("/home/ubuntu/src/zopro-perf-work/fq-fips175/native.rs").unwrap();
    let pre = &old[..old.find("fn once(").unwrap()];
    let tail = &old[old.find("fn freeze(").unwrap()..];
    fs::write(
        root.join("native.rs"),
        format!("{pre}\ninclude!(\"setup.rs\");\ninclude!(\"code-audit.rs\");\n{tail}")
            .replace("fq-fips175", "inverse-direct-sdk175"),
    )
    .unwrap();
    println!("Generated native workflow helper; reused immutable artifact freezing with explicit cfg and identity checks");
}
