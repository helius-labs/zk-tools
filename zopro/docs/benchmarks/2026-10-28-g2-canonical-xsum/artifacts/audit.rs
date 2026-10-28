use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
    let z = "/home/ubuntu/src/zk-tools/zopro";
    let m:Value=serde_json::from_slice(&fs::read(format!("{z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json")).unwrap()).unwrap();
    let audit: Value =
        serde_json::from_slice(&fs::read(format!("{root}/source-audit.json")).unwrap()).unwrap();
    let mut exact = 0;
    let mut recovered = 0;
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(format!("{z}/{f}")), h.as_str().unwrap());
        let p = format!("{root}/source/{f}");
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
            recovered += 1;
            continue;
        }
        let s = fs::read_to_string(&p).unwrap();
        let original = fs::read_to_string(format!("{z}/{f}")).unwrap();
        let back = match f.as_str() {
            "crates/zopro-core/src/msm.rs" => s
                .strip_suffix(audit["msm_suffix"].as_str().unwrap())
                .unwrap()
                .to_string(),
            "crates/zopro-core/src/lib.rs" => s
                .strip_suffix(audit["core_suffix"].as_str().unwrap())
                .unwrap()
                .to_string(),
            "crates/zopro-core/src/msm/g2_prefix.rs" => {
                let new = audit["new"].as_str().unwrap();
                assert_eq!(s.matches(new).count(), 1);
                s.replace(new, audit["old"].as_str().unwrap())
            }
            _ => panic!("unexpected changed file {f}"),
        };
        assert_eq!(back, original);
        recovered += 1;
    }
    assert_eq!(exact, 169);
    assert_eq!(recovered, 172);
    assert_eq!(
        hash(format!(
            "{root}/source/crates/zopro-core/src/msm/caller_probe.rs"
        )),
        hash("/home/ubuntu/src/zopro-perf-work/direct-ref-pack/caller_probe.rs")
    );
    fs::write(format!("{root}/runtime-audit.json"),serde_json::to_vec_pretty(&json!({"production_sources_unchanged":172,"private_originals_exact":exact,"reverse_edits_recover_accepted":recovered,"original_adapter_byte_exact":true,"audit_source":"audit.rs"})).unwrap()).unwrap();
    println!("169privateoriginalsexact;3editsreversed;172productionverified;adapterexact");
}
