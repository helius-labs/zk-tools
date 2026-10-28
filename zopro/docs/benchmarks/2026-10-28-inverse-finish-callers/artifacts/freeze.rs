use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Write,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/inverse-finish-callers";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn freeze(p: &Path, root: &Path, name: &str) -> (PathBuf, String) {
    let bytes = fs::read(p).unwrap();
    let h = hash(&bytes);
    fs::create_dir_all(root).unwrap();
    let dest = root.join(format!("lib{name}-{h}.rlib"));
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&dest)
    {
        Ok(mut f) => f.write_all(&bytes).unwrap(),
        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {
            assert_eq!(fs::read(&dest).unwrap(), bytes)
        }
        Err(e) => panic!("{e}"),
    }
    assert_eq!(hash(&fs::read(&dest).unwrap()), h);
    (dest, h)
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    assert_eq!(a.len(), 3);
    let label = &a[1];
    assert!(["frozen", "fresh", "selected"].contains(&label.as_str()));
    let flags = std::env::var("RUSTFLAGS").unwrap();
    assert_eq!(
        flags
            .split_whitespace()
            .any(|s| s == "zopro_direct_finish_reduction"),
        label == "selected"
    );
    let log = fs::read_to_string(&a[2]).unwrap();
    let root = Path::new(R).join("frozen-libraries").join(label);
    let receipt = Path::new(R).join(format!("native/{label}-artifacts.json"));
    assert!(!receipt.exists());
    let mut libraries = BTreeMap::<String, BTreeSet<String>>::new();
    let mut pins = BTreeMap::new();
    let mut inputs = BTreeMap::new();
    for l in log.lines() {
        let v: Value = serde_json::from_str(l).unwrap();
        if v["reason"] != "compiler-artifact" {
            continue;
        }
        let name = v["target"]["name"].as_str().unwrap();
        for path in v["filenames"]
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rlib"))
        {
            let (dest, h) = freeze(Path::new(path), &root, name);
            libraries
                .entry(name.into())
                .or_default()
                .insert(dest.display().to_string());
            pins.insert(dest.display().to_string(), h.clone());
            inputs.insert(path.to_string(), h);
        }
    }
    let mut required = BTreeMap::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "serde_json",
        "sha2",
        "libc",
        "rayon",
    ] {
        let found = libraries.get(name).unwrap();
        assert_eq!(found.len(), 1, "{name}");
        required.insert(name, found.first().unwrap());
    }
    fs::write(receipt,serde_json::to_vec_pretty(&json!({"label":label,"rustflags":flags,"artifacts_sha256":hash(log.as_bytes()),"libraries":required,"all_libraries":libraries,"input_pins":pins,"original_library_hashes_at_freeze":inputs,"timing_started":false})).unwrap()).unwrap();
    println!("Froze{label}{}contentaddressedlibraries", pins.len());
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn mutable_cargo_root_overwrite_preserves_frozen_off_and_distinct_on() {
        let root =
            std::env::temp_dir().join(format!("inverse-finish-freezer-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let mutable = root.join("libfield.rlib");
        fs::write(&mutable, b"exact OFF library").unwrap();
        let (off, oh) = freeze(&mutable, &root.join("off"), "field");
        fs::write(&mutable, b"exact ON library").unwrap();
        let (on, ch) = freeze(&mutable, &root.join("on"), "field");
        assert_ne!(oh, ch);
        assert_eq!(fs::read(off).unwrap(), b"exact OFF library");
        assert_eq!(fs::read(on).unwrap(), b"exact ON library");
        fs::remove_dir_all(root).unwrap();
    }
}
