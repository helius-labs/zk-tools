use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::Path,
    process::{Command, Stdio},
};
const R: &str = "/tmp/zopro-current-c7a-20261027";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    let current = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    for (f, h) in current["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(r.join("current").join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        read(r.join("native/current-field/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        33
    );
    assert_eq!(
        read(r.join("native/current-core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        116
    );
    assert_eq!(
        read(r.join("native/current-prover/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        8
    );
    let c = r.join("context");
    fs::create_dir(&c).unwrap();
    fs::create_dir(c.join("basis")).unwrap();
    let mut pins = BTreeMap::new();
    for (mode, receipt) in [
        (
            "previous",
            "/tmp/zopro-ready-c7a/native/accepted/artifacts.json".to_owned(),
        ),
        (
            "current",
            r.join("native/current/artifacts.json")
                .display()
                .to_string(),
        ),
    ] {
        let a = read(&receipt);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
        let name = format!("cr-{mode}");
        fs::copy(a["binary"].as_str().unwrap(), c.join(&name)).unwrap();
        pins.insert(format!("/usr/local/bin/{name}"), hash(c.join(name)));
    }
    for (src, name, target) in [
        (r.join("adapter"), "cr-adapter", "/usr/local/bin/cr-adapter"),
        (r.join("runner"), "cr-runner", "/usr/local/bin/cr-runner"),
        (
            Path::new("/tmp/zopro-xtask-fast-screen-target/release/xtask").to_path_buf(),
            "cr-xtask",
            "/usr/local/bin/cr-xtask",
        ),
        (
            Path::new("/tmp/zopro-gnark-verify/verify").to_path_buf(),
            "gnark-verify",
            "/usr/local/bin/gnark-verify",
        ),
        (
            Path::new("/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock").to_path_buf(),
            "zopro-keys.lock",
            "/manifest/zopro-keys.lock",
        ),
    ] {
        fs::copy(&src, c.join(name)).unwrap();
        pins.insert(target.into(), hash(src));
    }
    let local = read("/tmp/zopro-grouping-prefix-cutoff/perf-before-preflight.json");
    let lock = read("/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock");
    let mut cases = Vec::new();
    for original in local["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let mut case = original.clone();
        let image = Path::new(original["image"].as_str().unwrap());
        let basename = image.file_name().unwrap().to_str().unwrap();
        let stem = image.file_stem().unwrap().to_str().unwrap();
        let request = Path::new(original["request"].as_str().unwrap());
        let fixture = request.file_name().unwrap().to_str().unwrap();
        fs::copy(image, c.join("basis").join(basename)).unwrap();
        pins.insert(format!("/cr-basis/{basename}"), hash(image));
        pins.insert(format!("/fixtures/{fixture}"), hash(request));
        let key = &lock["keys"][format!("{stem}.key")];
        pins.insert(
            format!("/keys/{}", key["file"].as_str().unwrap()),
            key["sha256"].as_str().unwrap().to_owned(),
        );
        pins.insert(
            format!("/gnark-keys/{stem}.key"),
            key["key_sha256"].as_str().unwrap().to_owned(),
        );
        case["image"] = json!(format!("/cr-basis/{basename}"));
        case["request"] = json!(format!("/fixtures/{fixture}"));
        cases.push(case);
    }
    assert_eq!(cases.len(), 9);
    let previous = read("/tmp/zopro-latest-c7a/context/current-aws.json");
    pins.insert(
        "/usr/local/bin/aeglos-ablation".into(),
        previous["binaries"]["aeglos-default"]["sha256"]
            .as_str()
            .unwrap()
            .into(),
    );
    pins.insert(
        "/usr/lib/linux-tools/6.8.0-147-generic/perf".into(),
        "2927eb0973fd937bdd417622531c95f1e6e19307a29c3e05a2c6198e12af6e56".into(),
    );
    let old = read("/tmp/zopro-grouping-prefix-cutoff/perf-plan.json");
    let mut prior: BTreeSet<String> =
        serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 9448);
    let mut duplicates = 0;
    for (p, h) in read("/tmp/zopro-grouping-prefix-cutoff/proof-inventory.json")
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") && !prior.insert(h.as_str().unwrap().into()) {
            duplicates += 1;
        }
    }
    assert_eq!(duplicates, 9);
    assert_eq!(prior.len(), 9904);
    let manifest = json!({"scope":"Current adopted171-file Zopro versus previous AWS baseline and pinned Aeglos0290613Sorted; all9shapes32/64/96workers","parent_image_id":"sha256:0b79be0b9b5b9675622282a950bd87c8bb98ed092a6605c3a1cb426f6848e66b","container_pins":pins,"prior_proof_sha256":prior,"environment":current["environment"],"cases":cases,"symbol_categories":old["symbol_categories"],"accepted_source_manifest_sha256":hash("/tmp/zopro-composed-adoption/production-source-manifest.json"),"source_files":171,"source_changed":false,"design_sha256":hash(r.join("design.md")),"current_rustflags":read(r.join("native-plan.json"))["steps"][2]["environment"]["RUSTFLAGS"],"current_field_tests":33,"current_core_tests":116,"raw_tests":2,"cubic_tests":6,"adopted_new_source":false});
    save(c.join("manifest.json"), &manifest);
    let base="558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-latest-c7a-profile-20261025";
    let out = Command::new("docker")
        .args(["image", "inspect", base, "--format", "{{.Id}}"])
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(
        String::from_utf8(out.stdout).unwrap().trim(),
        manifest["parent_image_id"].as_str().unwrap()
    );
    fs::write(c.join("Dockerfile"),format!("FROM {base}\nCOPY cr-previous cr-current cr-adapter cr-runner cr-xtask gnark-verify /usr/local/bin/\nCOPY zopro-keys.lock /manifest/zopro-keys.lock\nCOPY basis /cr-basis\nCOPY manifest.json /cr/manifest.json\nENTRYPOINT [\"/usr/local/bin/cr-runner\"]\n")).unwrap();
    let log = fs::File::create(r.join("docker-build.log")).unwrap();
    assert!(Command::new("docker")
        .args(["build", "--pull=false", "-t", "zopro-current-c7a:preflight"])
        .arg(&c)
        .stdout(Stdio::from(log.try_clone().unwrap()))
        .stderr(Stdio::from(log))
        .status()
        .unwrap()
        .success());
    save(
        r.join("image-prepared.json"),
        &json!({"status":"built; not published or launched","container_pins":pins.len(),"parent_image_checked":true,"manifest_sha256":hash(c.join("manifest.json"))}),
    );
    println!("Built current171/previous/Aeglos exact preflight image;prior9904;no AWS resources");
}
