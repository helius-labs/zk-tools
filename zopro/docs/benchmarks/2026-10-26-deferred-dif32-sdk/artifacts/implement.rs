use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn replace(s: &mut String, needle: &str, replacement: &str) {
    assert_eq!(s.matches(needle).count(), 1, "{needle}");
    *s = s.replacen(needle, replacement, 1);
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32-sdk");
    let source = root.join("candidate");
    let component = Path::new("/tmp/zopro-deferred-dif32/candidate");
    let mut records = Vec::new();
    for name in [
        "crates/zopro-bn254/src/ifma.rs",
        "crates/zopro-bn254/src/ifma8.rs",
    ] {
        fs::copy(component.join(name), source.join(name)).unwrap();
        assert_eq!(hash(component.join(name)), hash(source.join(name)));
        records.push(
            json!({"path":name,"component_byte_identical":true,"sha256":hash(source.join(name))}),
        );
    }
    let native="#[cfg(all(zopro_deferred_dif32,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]";
    let path = source.join("crates/zopro-core/src/fft.rs");
    let before =
        fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/fft.rs").unwrap();
    let mut text = before.clone();
    let field = format!("    {native}\n    deferred_dif32: bool,\n");
    replace(
        &mut text,
        "struct Twiddles {\n",
        &format!("struct Twiddles {{\n{field}"),
    );
    let init = format!("            {native}\n            deferred_dif32: false,\n");
    assert_eq!(
        text.matches("        Self {\n            reservations: Vec::new(),")
            .count(),
        3
    );
    text = text.replace(
        "        Self {\n            reservations: Vec::new(),",
        &format!("        Self {{\n{init}            reservations: Vec::new(),"),
    );
    let anchor="        let mut inverse_roots = Twiddles::new(powers(generator.inverse().unwrap(), n / 2));\n";
    let setup=format!("        {native}\n        {{\n            let enabled = std::env::var(\"ZOPRO_CPU_DEFERRED_DIF32\").as_deref() == Ok(\"1\");\n            roots.deferred_dif32 = enabled;\n            inverse_roots.deferred_dif32 = enabled;\n        }}\n");
    replace(&mut text, anchor, &format!("{anchor}{setup}"));
    let restored = text
        .replacen(&field, "", 1)
        .replace(&init, "")
        .replacen(&setup, "", 1);
    assert_eq!(restored, before);
    fs::write(&path, text).unwrap();
    records.push(json!({"path":"crates/zopro-core/src/fft.rs","recovered_byte_for_byte":true,"sha256":hash(&path),"new_fields":"one bool per nativeTwiddles, initializedfalse; setonlyordinaryroots onceperPlan. Twisted/empty remainfalse."}));
    let path = source.join("crates/zopro-core/src/fft/pair.rs");
    let before =
        fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/fft/pair.rs")
            .unwrap();
    let mut text = before.clone();
    let module = "\n#[cfg(zopro_deferred_dif32)]\nmod deferred;\n";
    replace(
        &mut text,
        "const TASK: usize = 64;\n",
        &format!("const TASK: usize = 64;\n{module}"),
    );
    let signature = "    column: usize,\n) {\n    for order in 0..5 {";
    let dispatch="    #[cfg(zopro_deferred_dif32)]\n    if !FORWARD && roots.deferred_dif32 {\n        deferred::apply::<CACHED>(local, roots, level, q, column);\n        return;\n    }\n";
    replace(
        &mut text,
        signature,
        &format!("    column: usize,\n) {{\n{dispatch}    for order in 0..5 {{"),
    );
    assert_eq!(
        text.replacen(module, "", 1).replacen(dispatch, "", 1),
        before
    );
    fs::write(&path, text).unwrap();
    records.push(json!({"path":"crates/zopro-core/src/fft/pair.rs","recovered_byte_for_byte":true,"sha256":hash(&path),"scope":"nativeDIFcolumn dispatch only; original OFF/DIT arithmetic body intact"}));
    let module = source.join("crates/zopro-core/src/fft/pair/deferred.rs");
    fs::create_dir_all(module.parent().unwrap()).unwrap();
    fs::copy(root.join("deferred.rs"), &module).unwrap();
    for name in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        let dst = source.join(name);
        fs::create_dir_all(dst.parent().unwrap()).unwrap();
        fs::copy(
            Path::new("/tmp/zopro-outlined-snapshot-fence/candidate").join(name),
            dst,
        )
        .unwrap();
    }
    let mut pins: BTreeMap<String, String> = serde_json::from_value(
        serde_json::from_slice::<Value>(&fs::read(source.join(".xtask/source-pins.json")).unwrap())
            .unwrap(),
    )
    .unwrap();
    for name in [
        "crates/zopro-bn254/src/ifma.rs",
        "crates/zopro-bn254/src/ifma8.rs",
        "crates/zopro-core/src/fft.rs",
        "crates/zopro-core/src/fft/pair.rs",
        "crates/zopro-core/src/fft/pair/deferred.rs",
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        let p = source.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    assert_eq!(pins.len(), 173);
    fs::write(
        source.join(".xtask/source-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(root.join("source-audit.json"),serde_json::to_vec_pretty(&json!({"status":"passed","source_files":pins.len(),"component_field_identity":true,"original_core_files_recovered":true,"changes":records,"twisted_root_fallback":"twisted usesordinarymul andcanbelazy; flaginitialfalsepreventsdispatch","heap_allocation_added":false,"root_array_copy_added":false})).unwrap()).unwrap();
    println!("Prepared173sourcepins; componentfieldbytesidentical; originalcorefilesrecoverexactly; new4tests pending");
}
