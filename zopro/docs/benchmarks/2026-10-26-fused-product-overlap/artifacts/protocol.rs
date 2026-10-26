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
    fs::write(p, format!("{}\n", serde_json::to_string_pretty(v).unwrap())).unwrap();
}
fn main() {
    let root = Path::new("/tmp/zopro-fused-product-overlap");
    let parent = Path::new("/tmp/zopro-deferred-dif32-sdk");
    assert_eq!(read(root.join("native/result.json"))["status"], "complete");
    let artifact = read(root.join("native/candidate/artifacts.json"));
    let accepted = read("/tmp/zopro-grouping-module/native/candidate/artifacts.json");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("build-input-pins.json"))).unwrap();
    for a in [&artifact, &accepted] {
        add(a["binary"].as_str().unwrap(), &mut pins);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            add(p, &mut pins);
        }
    }
    add(root.join("native/candidate/artifacts.json"), &mut pins);
    add(
        "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
        &mut pins,
    );
    let oldpins = read(parent.join("input-pins.json"));
    for (p, h) in oldpins.as_object().unwrap() {
        if p.contains("/registry/")
            || p.starts_with("/tmp/zopro-keys/")
            || p.starts_with("/tmp/zopro-gnark-verify/")
        {
            assert_eq!(hash(p), h.as_str().unwrap());
            add(p, &mut pins);
        }
    }
    let mut template = read(parent.join("all-nine-template.json"));
    let mut prior: BTreeSet<String> =
        serde_json::from_value(template["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4084);
    for (p, h) in read(parent.join("proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 4321);
    let variants = json!([{"name":"accepted","binary":accepted["binary"]},{"name":"off","binary":artifact["binary"],"environment":{"ZOPRO_CPU_FUSED_PRODUCT_OVERLAP":"0"}},{"name":"on","binary":artifact["binary"],"environment":{"ZOPRO_CPU_FUSED_PRODUCT_OVERLAP":"1"}}]);
    template["variants"] = variants.clone();
    template["prior_proof_sha256"] = json!(prior);
    save(root.join("all-nine-template.json"), &template);
    let inventory = fs::read_to_string(parent.join("inventory-preflight.sh"))
        .unwrap()
        .replace(
            "/tmp/zopro-deferred-dif32-sdk",
            "/tmp/zopro-fused-product-overlap",
        );
    fs::write(root.join("inventory-preflight.sh"), inventory).unwrap();
    fs::copy(
        parent.join("validate-preflight.jq"),
        root.join("validate-preflight.jq"),
    )
    .unwrap();
    for case in template["cases"].as_array().unwrap() {
        add(case["image"].as_str().unwrap(), &mut pins);
        add(case["request"].as_str().unwrap(), &mut pins);
    }
    for name in [
        "experiment.json",
        "protocol.rs",
        "activation-audit.rs",
        "activation-audit",
        "layout.rs",
        "native-plan.json",
        "native/result.json",
        "native/core-off/tests.json",
        "native/core-on/tests.json",
        "native/core-on/stdout.log",
        "native/prover/tests.json",
        "all-nine-template.json",
        "inventory-preflight.sh",
        "validate-preflight.jq",
    ] {
        add(root.join(name), &mut pins);
    }
    add(
        "/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock",
        &mut pins,
    );
    let mut scheduler = BTreeMap::new();
    let log = root.join("native/libraries/stdout.log");
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
                        for p in v["filenames"].as_array().unwrap() {
                            if p.as_str().unwrap().ends_with(".rlib") {
                                assert!(scheduler
                                    .insert(name.to_owned(), p.as_str().unwrap().to_owned())
                                    .is_none());
                                add(p.as_str().unwrap(), &mut pins);
                            }
                        }
                    }
                }
            }
        }
    }
    assert_eq!(scheduler.len(), 4);
    save(
        root.join("linked-scheduler-libraries.json"),
        &json!(scheduler),
    );
    add(root.join("linked-scheduler-libraries.json"), &mut pins);
    add(log, &mut pins);
    let mut steps = Vec::new();
    for (label, a) in [("accepted", &accepted), ("candidate", &artifact)] {
        let core = a["libraries"]["zopro_core"].as_str().unwrap();
        let deps = Path::new(core).parent().unwrap();
        let exe = root.join(format!("{label}-layout"));
        steps.push(json!({"kind":"command","name":format!("{label}-layout-link"),"argv":["rustc","--edition=2021","-O","-C","target-cpu=native",root.join("layout.rs"),"--extern",format!("zopro_core={core}"),"-L",format!("dependency={}",deps.display()),"-o",exe],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("{label}-layout-observation"),"argv":[exe],"timeout_seconds":30}));
    }
    for mode in ["off", "on"] {
        for case in template["cases"].as_array().unwrap() {
            let shape = case["shape"].as_str().unwrap();
            let image = case["image"].as_str().unwrap();
            let stem = Path::new(image).file_stem().unwrap().to_str().unwrap();
            let name = format!("{mode}-w4-{shape}");
            let proof = root.join("preflight-proofs").join(&name);
            steps.push(json!({"kind":"command","name":format!("{name}-prove"),"argv":[artifact["binary"],image,case["request"],"1",proof.join(stem)],"environment":{"RAYON_NUM_THREADS":"4","ZOPRO_CPU_FUSED_PRODUCT_OVERLAP":if mode=="on"{"1"}else{"0"},"ZOPRO_FUSED_PRODUCT_OVERLAP_TRACE":"1"},"timeout_seconds":600}));
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
    assert_eq!(steps.len(), 42);
    steps.push(json!({"kind":"command","name":"validate-activation","argv":[root.join("activation-audit")],"timeout_seconds":30}));
    let mut perf = read(parent.join("perf-template.json"));
    perf["variants"] = variants;
    perf["prior_proof_sha256"] = json!(prior);
    perf["pins"] = json!({});
    perf["checks"] = json!([check]);
    let sdk = perf["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| c["rounds"].as_u64().unwrap() + 3)
        .sum::<u64>()
        * 7
        * 3;
    assert_eq!(sdk, 8610);
    save(root.join("perf-template.json"), &perf);
    add(root.join("perf-template.json"), &mut pins);
    save(root.join("input-pins.json"), &json!(pins));
    save(
        root.join("preflight-plan.json"),
        &json!({"version":1,"source_pins":root.join("input-pins.json"),"environment":perf["environment"],"steps":steps}),
    );
    println!(
        "FROZEN3variants8610latencySDK510profileSDK18preflightexports;43steps{}pins4321priorproofs",
        pins.len()
    );
}
