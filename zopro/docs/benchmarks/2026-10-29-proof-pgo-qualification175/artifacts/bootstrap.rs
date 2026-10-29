use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-qualification175";
fn main() {
    let root = Path::new(R);
    assert!(!root.join("capture.rs").exists());
    let old =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/proof-pgo175/capture.rs").unwrap();
    let a = old.find("include!(\"capture-setup.rs\");").unwrap();
    let b = old.find("fn push(").unwrap();
    let c = old.find("fn finish() {").unwrap();
    let begin = old[..a]
        .replace(
            "/home/ubuntu/src/zopro-perf-work/proof-pgo175",
            "/home/ubuntu/src/zopro-perf-work/proof-pgo-qualification175",
        )
        .replace(
            "const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/radix-groups175\";",
            "const PILOT: &str = \"/home/ubuntu/src/zopro-perf-work/proof-pgo175\";",
        );
    fs::write(root.join("capture.rs"),format!("{begin}include!(\"qualification.rs\");\n{}\nfn main(){{match std::env::args().nth(1).as_deref(){{Some(\"prepare\")=>prepare(),Some(\"finish\")=>finish(),Some(\"loss\")=>loss(&std::env::args().nth(2).unwrap()),_=>panic!(\"prepare|finish|loss\")}}}}\n",&old[b..c])).unwrap();
}
