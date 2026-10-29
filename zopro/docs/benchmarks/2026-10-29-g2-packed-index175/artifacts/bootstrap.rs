use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-packed-index175";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177";
fn main() {
    let r = Path::new(R);
    assert!(!r.join("native.rs").exists());
    let audit = fs::read_to_string(Path::new(P).join("code-audit.rs"))
        .unwrap()
        .replace("g2_borrowed::gls4", "msm::packed_index")
        .replace("g2_phi::raw_exclusive", "msm::reduce_indexed_buckets")
        .replace("new_GLS4", "new_packed_index")
        .replace("linked_GLS4_present_only_ON", "linked_packed_index_only_ON")
        .replace("newGLS4", "newPackedIndex");
    fs::write(r.join("code-audit.rs"), &audit).unwrap();
    let old = fs::read_to_string(Path::new(P).join("native.rs")).unwrap();
    let a = old.find("fn setup() {").unwrap();
    let b = old.find("fn freeze(label:").unwrap();
    let source = format!(
        "{}{}\n{}\n{}",
        &old[..a],
        fs::read_to_string(r.join("setup.rs")).unwrap(),
        audit,
        &old[b..]
    )
    .replace(P, R);
    fs::write(r.join("native.rs"), source).unwrap();
}
