use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/three-lane-final-exp176";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/three-lane-miller176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn main() {
    let root = Path::new(R);
    let pins: Value = serde_json::from_slice(&fs::read(Path::new(OLD).join("capture-pins.json")).unwrap()).unwrap();
    for name in ["packed.rs", "prepared.rs", "synthetic-fixtures.json"] {
        let from = Path::new(OLD).join(name);
        assert_eq!(hash(&from), pins[from.display().to_string()].as_str().unwrap());
    }
    let mut source = fs::read_to_string(Path::new(OLD).join("packed.rs")).unwrap();
    let signature = "pub fn miller(pairs:&[(G1Affine,&crate::prepared::G2Prepared);3])->Fq12{";
    let tail = " let values=f.get();values[0]*values[1]*values[2]\n}";
    assert_eq!(source.matches(signature).count(), 1);
    assert_eq!(source.matches(tail).count(), 1);
    let suffix = "\nmod frob6;\nmod frob12;\nmod finish;\npub use finish::fully_packed;\n";
    source = source.replace(signature, "pub fn miller_values(pairs:&[(G1Affine,&crate::prepared::G2Prepared);3])->T{");
    source = source.replace(tail, " f\n}");
    source.push_str(suffix);
    let recovered = source.strip_suffix(suffix).unwrap().replace("pub fn miller_values(pairs:&[(G1Affine,&crate::prepared::G2Prepared);3])->T{", signature).replace(" f\n}", tail);
    assert_eq!(recovered, fs::read_to_string(Path::new(OLD).join("packed.rs")).unwrap());
    assert!(!root.join("packed.rs").exists());
    fs::write(root.join("packed.rs"), source).unwrap();
    fs::copy(Path::new(OLD).join("prepared.rs"), root.join("prepared.rs")).unwrap();
    let mut identities = BTreeMap::new();
    for (from, to, ty) in [("fq6.rs", "frob6.rs", "Fq6"), ("fq12.rs", "frob12.rs", "Fq12")] {
        let original_path = Path::new(Z).join("crates/zopro-bn254/src").join(from);
        let text = fs::read_to_string(&original_path).unwrap();
        let a = text.find("const fn fq2(").unwrap();
        let b = text.find(&format!("impl {ty} {{")).unwrap();
        let original = &text[a..b];
        let changed = original.replace("const FROBENIUS_", "pub(super) const FROBENIUS_");
        assert_eq!(changed.replace("pub(super) const FROBENIUS_", "const FROBENIUS_"), original);
        fs::write(root.join("packed").join(to), format!("use zopro_bn254::{{Fq,Fq2}};\n{changed}")).unwrap();
        identities.insert(from.to_string(), json!({"original_source":original_path,"source_sha256":hash(&original_path),"constants_exact_after_visibility_reversal":true}));
    }
    fs::write(root.join("source-copy-review.json"), serde_json::to_vec_pretty(&json!({"previous_root_untouched":true,"original_packed_source_exact_after_two_replacements_and_suffix_reversal":true,"prepared_source_exact":hash(root.join("prepared.rs"))==hash(Path::new(OLD).join("prepared.rs")),"frobenius_sources":identities})).unwrap()).unwrap();
    println!("New isolated packed Miller-return path and exact Frobenius constants prepared; previous qualified tree untouched");
}
