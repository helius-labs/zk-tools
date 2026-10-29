use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-cancel175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let m:Value=serde_json::from_slice(&fs::read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let mut pins = BTreeMap::new();
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        if let Some(tail) = f.strip_prefix("crates/zopro-bn254/") {
            let p = Path::new(Z).join(f);
            assert_eq!(hash(&p), h.as_str().unwrap());
            let q = Path::new(R).join("bn254").join(tail);
            fs::create_dir_all(q.parent().unwrap()).unwrap();
            fs::copy(&p, &q).unwrap();
            pins.insert(q.display().to_string(), hash(q));
        }
    }
    assert_eq!(pins.len(), 17);
    fs::write(
        format!("{R}/field-original-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let text = fs::read_to_string(format!("{R}/bn254/src/ifma8.rs")).unwrap();
    let start = text
        .find("    #[inline(always)]\n    fn product_cios<const FUSED: bool>")
        .unwrap();
    let end = start
        + text[start..]
            .find("    /// Inputs must each be below p.")
            .unwrap();
    let mut body = text[start..end].replace("fn product_cios<", "fn product_triple<");
    for (a, b) in [
        (
            "let mut t = [zero; 6];",
            "let mut t = self.prefix_three::<FUSED>(b,c,d);",
        ),
        (
            "            row!(0);\n            row!(1);\n            row!(2);\n",
            "",
        ),
    ] {
        assert_eq!(body.matches(a).count(), 1);
        body = body.replace(a, b);
    }
    let prefix = fs::read_to_string(format!("{R}/prefix.rs")).unwrap();
    fs::write(format!("{R}/kernel.rs"), format!("{prefix}\n{body}\n}}\n")).unwrap();
    let probe =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/mask-carry175/src/bin/probe.rs")
            .unwrap()
            .replace("mask_carry_kernel", "triple_cancel_kernel")
            .replace("mul_masked", "mul_triple")
            .replace("sum2_masked", "sum2_triple")
            .replace("\"masked\"", "\"triple\"");
    fs::write(format!("{R}/src/bin/probe.rs"), probe).unwrap();
    println!("Copied17acceptedfieldfiles; prefix3insteadrow0..2;last2rowsunchanged");
}
