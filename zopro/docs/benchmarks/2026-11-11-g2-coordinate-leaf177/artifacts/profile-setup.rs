use std::{fs,path::Path};
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/g1-offset-profile177";
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf-profile177";
fn main() {
    assert!(!Path::new(&format!("{R}/profile.rs")).exists());
    for name in ["profile.rs","common.rs","sampling.rs","metadata.rs","counter.rs"] {
        let s = fs::read_to_string(format!("{OLD}/{name}")).unwrap()
            .replace(OLD,R)
            .replace("/home/ubuntu/src/zopro-perf-work/g1-offset177", "/home/ubuntu/src/zopro-perf-work/g2-coordinate-leaf177")
            .replace("52927","53899").replace("52945","53917");
        assert!(!s.contains("g1-offset"));
        fs::write(format!("{R}/{name}"),s).unwrap();
    }
    println!("Reused qualified profile panel with new exact consumers and ledger; no SDK recording.");
}
