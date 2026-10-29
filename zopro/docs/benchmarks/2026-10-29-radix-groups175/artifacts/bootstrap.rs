use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/radix-groups175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let old =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175/native.rs")
            .unwrap();
    let begin = &old[..old.find("include!(\"setup.rs\");").unwrap()];
    let end = &old[old.find("fn freeze(").unwrap()..];
    fs::write(
        r.join("native.rs"),
        format!("{begin}include!(\"setup.rs\");\ninclude!(\"code-audit.rs\");\n{end}")
            .replace("inverse-direct-sdk175", "radix-groups175"),
    )
    .unwrap();
    println!("Generated native grouping workflow helper");
}
