use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    let old = read("/tmp/zopro-complex-shared-product/native-plan.json");
    let mut plan: Value = serde_json::from_str(
        &serde_json::to_string(&old)
            .unwrap()
            .replace(
                "/tmp/zopro-complex-shared-product",
                "/tmp/zopro-streamed-complex-sdk",
            )
            .replace("sdk-candidate/", "candidate/")
            .replace("zopro_shared_complex_products", "zopro_streamed_complex_b"),
    )
    .unwrap();
    let steps = plan["steps"].as_array_mut().unwrap();
    steps.insert(1,json!({"kind":"command","name":"generator-check","argv":["python3",root.join("candidate/scripts/ifma8.py"),"--check"],"timeout_seconds":120}));
    for step in steps {
        if step["name"] == "off-core" || step["name"] == "on-core" {
            step["required_tests"] = json!([
                "msm::g2_prefix::tests::g2_prefix_complete_curve_tails_reuse",
                "msm::g2_prefix::tests::g2_prefix_dispatch_and_control",
                "msm::inplace_scratch_tests::inplace_g2_mixed_buckets_odd_counts_and_reused_layers",
                "msm::reused_reduction_tests::reused_weighted_g2_independent_oracle",
                "msm_affine::fused_kh::tests::combined_queries_generator_oracle"
            ]);
        }
        if step["kind"] == "link-sdk" {
            step["driver"] = json!(root.join("candidate/crates/zopro-bench/examples/cpu-sdk.rs"));
            step["additional_sources"] =
                json!([root.join("candidate/crates/zopro-bench/examples/support/perf.rs")]);
        }
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let mut pins = BTreeMap::new();
    for (n, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
        let p = root.join("candidate").join(n);
        pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    }
    for n in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        let p = root.join("candidate").join(n);
        pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    }
    assert_eq!(pins.len(), 172);
    for n in [
        "experiment.json",
        "prepare.json",
        "implement.rs",
        "source-audit.json",
        "prepare-build.rs",
    ] {
        let p = root.join(n);
        pins.insert(p.to_str().unwrap().to_owned(), hash(&p));
    }
    for p in [
        "/home/ubuntu/src/zk-tools/zopro/xtask/target/release/xtask",
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        "/tmp/zopro-streamed-complex-b/closeout.json",
        "/tmp/zopro-streamed-complex-b/component-result.json",
        "/tmp/zopro-streamed-complex-b/candidate/crates/zopro-bn254/src/ifma.rs",
        "/tmp/zopro-streamed-complex-b/candidate/crates/zopro-bn254/src/ifma8.rs",
        "/tmp/zopro-streamed-complex-b/probe-tests.rs",
        "/tmp/zopro-streamed-complex-b/build-v2/component-oracles/stdout.log",
        "/tmp/zopro-streamed-complex-b/build-v2/component-oracles/exit.json",
    ] {
        pins.insert(p.into(), hash(p));
    }
    assert_eq!(
        read("/tmp/zopro-streamed-complex-b/closeout.json")["performance_gate_passed"],
        true
    );
    plan["source_pins"] = json!(root.join("build-input-pins.json"));
    fs::write(
        root.join("build-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("native-plan.json"),
        serde_json::to_vec_pretty(&plan).unwrap(),
    )
    .unwrap();
    fs::write(root.join("work-state.json"),serde_json::to_vec_pretty(&json!({"phase":"native OFF/ON fullcore+raw/cubic builds; no timing yet","last_pushed_commit":"03aa2eb2470000a5fb5aa591e1722dc45430ec2f","root":root,"source_pins":172,"build_pins":pins.len(),"changed_files":4,"accepted_unchanged":true,"adopted":false,"goal_achieved":false})).unwrap()).unwrap();
    println!(
        "Frozen172sources{}buildpins; separate OFF/ON nativeSDK + fullcore/raw/cubic",
        pins.len()
    );
}
