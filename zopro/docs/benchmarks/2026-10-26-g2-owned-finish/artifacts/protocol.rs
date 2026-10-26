use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    let h = hash(p);
    if let Some(old) = pins.insert(p.to_str().unwrap().into(), h.clone()) {
        assert_eq!(old, h);
    }
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-g2-owned-finish");
    let parent = Path::new("/tmp/zopro-fused-product-overlap");
    assert_eq!(read(root.join("native/result.json"))["status"], "complete");
    let accepted = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let off = read(root.join("native/off/artifacts.json"));
    let on = read(root.join("native/on/artifacts.json"));
    let artifacts = [("accepted", &accepted), ("off", &off), ("on", &on)];
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("build-input-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "{p}");
    }
    for (label, artifact) in artifacts {
        add(artifact["binary"].as_str().unwrap(), &mut pins);
        assert_eq!(
            hash(artifact["binary"].as_str().unwrap()),
            artifact["binary_sha256"].as_str().unwrap()
        );
        for (p, h) in artifact["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            add(p, &mut pins);
        }
        if label != "accepted" {
            add(
                root.join(format!("native/{label}/artifacts.json")),
                &mut pins,
            );
        }
    }
    add(
        "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
        &mut pins,
    );
    let parent_config = read("/tmp/zopro-streamed-complex-sdk/perf-plan.json");
    for (p, h) in parent_config["pins"].as_object().unwrap() {
        if p.contains("/registry/")
            || p.starts_with("/tmp/zopro-keys/")
            || p.starts_with("/tmp/zopro-gnark-verify/")
        {
            assert_eq!(hash(p), h.as_str().unwrap());
            add(p, &mut pins);
        }
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(parent_config["prior_proof_sha256"].clone()).unwrap();
    let before = prior.len();
    let mut added = 0;
    let previous = read(Path::new("/tmp/zopro-streamed-complex-sdk/proof-inventory.json").to_path_buf());
    let mut previous_proofs = 0;
    for (p, h) in previous.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            previous_proofs += 1;
            if prior.insert(h.as_str().unwrap().into()) {
                added += 1;
            }
        }
    }
    assert_eq!(previous_proofs, 237);
    assert_eq!(added, 219);
    assert_eq!(prior.len(), before + 219);
    assert_eq!(prior.len(), 4804);
    let counters=read("/tmp/zopro-front-end-counters/proof-inventory.json");let mut added_counters=0;
    for(p,h)in counters.as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());add(p,&mut pins);if p.ends_with(".proof"){assert!(prior.insert(h.as_str().unwrap().into()));added_counters+=1;}}
    assert_eq!(added_counters,27);assert_eq!(prior.len(),4831);
    add("/tmp/zopro-front-end-counters/proof-inventory.json",&mut pins);
    add("/tmp/zopro-streamed-complex-sdk/perf-plan.json",&mut pins);
    save(
        root.join("prior-proof-audit.json"),
        &json!({"previous_perf_prior":before,"previous_experiment_exports":previous_proofs,"previous_exports_new_to_that_prior":added,"new_counter_exports":27,"prior_for_this_experiment":prior.len(),"source_inventory_sha256":hash(Path::new("/tmp/zopro-streamed-complex-sdk/proof-inventory.json").to_path_buf()),"all_source_proof_public_digests_verified":true}),
    );
    let mut template = read(parent.join("all-nine-template-v2.json"));
    let variants = json!([{"name":"accepted","binary":accepted["binary"]},{"name":"off","binary":off["binary"]},{"name":"on","binary":on["binary"]}]);
    template["variants"] = variants.clone();
    template["prior_proof_sha256"] = json!(prior);
    template["pins"] = json!({});
    template["checks"] = json!([]);
    save(root.join("all-nine-template.json"), &template);
    for case in template["cases"].as_array().unwrap() {
        add(case["image"].as_str().unwrap(), &mut pins);
        add(case["request"].as_str().unwrap(), &mut pins);
    }
    let inventory = fs::read_to_string(parent.join("inventory-preflight-v2.sh"))
        .unwrap()
        .replace(
            "/tmp/zopro-fused-product-overlap",
            "/tmp/zopro-g2-owned-finish",
        )
        .replace("preflight-proofs-v2", "preflight-proofs")
        .replace(
            "preflight-proof-inventory-v2.json",
            "preflight-proof-inventory.json",
        );
    fs::write(root.join("inventory-preflight.sh"), inventory).unwrap();
    fs::copy(
        parent.join("validate-preflight-v2.jq"),
        root.join("validate-preflight.jq"),
    )
    .unwrap();
    fs::copy(parent.join("layout.rs"), root.join("layout.rs")).unwrap();
    let mut steps = Vec::new();
    for (label, artifact) in artifacts {
        let core = artifact["libraries"]["zopro_core"].as_str().unwrap();
        let deps = Path::new(core).parent().unwrap();
        let exe = root.join(format!("{label}-layout"));
        steps.push(json!({"kind":"command","name":format!("{label}-layout-link"),"argv":["rustc","--edition=2021","-O","-C","target-cpu=native",root.join("layout.rs"),"--extern",format!("zopro_core={core}"),"-L",format!("dependency={}",deps.display()),"-o",exe],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("{label}-layout-observation"),"argv":[exe],"timeout_seconds":30}));
    }
    for (label, artifact) in [("off", &off), ("on", &on)] {
        for case in template["cases"].as_array().unwrap() {
            let image = case["image"].as_str().unwrap();
            let stem = Path::new(image).file_stem().unwrap().to_str().unwrap();
            let name = format!("{label}-w4-{}", case["shape"].as_str().unwrap());
            let proof = root.join("preflight-proofs").join(&name);
            steps.push(json!({"kind":"command","name":format!("{name}-prove"),"argv":[artifact["binary"],image,case["request"],"1",proof.join(stem)],"environment":{"RAYON_NUM_THREADS":"4"},"timeout_seconds":600}));
            steps.push(json!({"kind":"command","name":format!("{name}-verify"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",proof,"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":600}));
        }
    }
    let check = json!([
        "jq",
        "-n",
        "--slurpfile",
        "inventory",
        root.join("preflight-proof-inventory.json"),
        "--slurpfile",
        "template",
        root.join("all-nine-template.json"),
        "-f",
        root.join("validate-preflight.jq")
    ]);
    steps.push(json!({"kind":"command","name":"proof-inventory","argv":["bash",root.join("inventory-preflight.sh")],"timeout_seconds":30}));
    steps.push(
        json!({"kind":"command","name":"validate-inventory","argv":check,"timeout_seconds":30}),
    );
    assert_eq!(steps.len(), 44);
    let mut scheduler = BTreeMap::new();
    for label in ["off", "on"] {
        let log = root.join(format!("native/{label}-libraries/stdout.log"));
        for line in fs::read_to_string(&log).unwrap().lines() {
            if let Ok(v) = serde_json::from_str::<Value>(line) {
                if v["reason"] == "compiler-artifact" {
                    if let Some(name) = v["target"]["name"].as_str() {
                        if [
                            "crossbeam_deque",
                            "crossbeam_epoch",
                            "crossbeam_utils",
                            "rayon_core",
                        ]
                        .contains(&name)
                        {
                            for path in v["filenames"].as_array().unwrap() {
                                if path.as_str().unwrap().ends_with(".rlib") {
                                    let key = format!("{label}/{name}");
                                    assert!(scheduler
                                        .insert(key, path.as_str().unwrap().to_owned())
                                        .is_none());
                                    add(path.as_str().unwrap(), &mut pins);
                                }
                            }
                        }
                    }
                }
            }
        }
        add(&log, &mut pins);
    }
    assert_eq!(scheduler.len(), 8);
    save(
        root.join("linked-scheduler-libraries.json"),
        &json!(scheduler),
    );
    for n in [
        "protocol.rs",
        "native-plan.json",
        "native/result.json",
        "native/off-core/tests.json",
        "native/on-core/tests.json",
        "native/off-prover/tests.json",
        "native/on-prover/tests.json",
        "all-nine-template.json",
        "layout.rs",
        "inventory-preflight.sh",
        "validate-preflight.jq",
        "prior-proof-audit.json",
        "linked-scheduler-libraries.json",
        "analyze.rs",
        "codegen.rs",
        "codegen.json",
        "codegen.txt",
    ] {
        add(root.join(n), &mut pins);
    }
    for p in [
        parent.join("perf-plan-v2.json"),
        Path::new("/tmp/zopro-streamed-complex-sdk/proof-inventory.json").to_path_buf(),
        Path::new("/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock").into(),
    ] {
        add(p, &mut pins);
    }
    let mut perf = read(parent.join("perf-template-v2.json"));
    perf["variants"] = variants;
    perf["prior_proof_sha256"] = json!(prior);
    perf["pins"] = json!({});
    perf["checks"] = json!([check]);
    assert_eq!(perf["monotonic_clock"], true);
    let requests = perf["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["rounds"].as_u64().unwrap() + 3)
        .sum::<u64>()
        * 7
        * 3;
    assert_eq!(requests, 8610);
    save(root.join("perf-template.json"), &perf);
    add(root.join("perf-template.json"), &mut pins);
    save(root.join("input-pins.json"), &json!(pins));
    save(
        root.join("preflight-plan.json"),
        &json!({"version":1,"source_pins":root.join("input-pins.json"),"environment":perf["environment"],"steps":steps}),
    );
    println!(
        "Frozen44preflightsteps{}pins4831priorproofs; future8610latency510warmprofileSDK",
        pins.len()
    );
}
