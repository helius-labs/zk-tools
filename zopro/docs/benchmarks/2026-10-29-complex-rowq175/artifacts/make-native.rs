use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/complex-rowq175";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let old =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/positive-gauss-caller175/native.rs")
            .unwrap();
    let a = old.find("fn freeze(label:").unwrap();
    let b = old.find("fn main()").unwrap();
    let mut freeze = old[a..b].replace("zopro_gauss_positive", "zopro_complex_rowq");
    let from="format!(\"{}/libraries-{label}/stdout.log\",if label==\"on\"{\"native-on\"}else{\"native\"})";
    assert!(freeze.contains(from));
    freeze = freeze.replace(from, "format!(\"native/libraries-{label}/stdout.log\")");
    let pre = old[..a].replace("positive-gauss-caller175", "complex-rowq175");
    fs::write(r.join("native.rs"),format!("{pre}{freeze}\nfn main(){{let a:Vec<_>=std::env::args().collect();match a[1].as_str(){{\"setup\"=>setup(),\"freeze\"=>freeze(&a[2]),_=>panic!(\"setup|freeze\")}}}}\n")).unwrap();
}
