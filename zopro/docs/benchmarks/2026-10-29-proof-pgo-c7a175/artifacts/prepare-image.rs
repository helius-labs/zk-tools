use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-c7a175";
const PILOT: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo-qualification175";
const BASE: &str = "sha256:a24195e830e9becaa887ece0898c7c07b628edba3f365f439988e885c78bd1b5";
const OLD: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-pmu-c7a-aeglos/artifacts";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
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
fn run(argv: &[String], label: &str) {
    let f = fs::File::create(r(format!("{label}.log"))).unwrap();
    assert!(
        Command::new(&argv[0])
            .args(&argv[1..])
            .stdout(Stdio::from(f.try_clone().unwrap()))
            .stderr(Stdio::from(f))
            .status()
            .unwrap()
            .success(),
        "{label} failed"
    );
}
fn main() {
    assert!(!r("context").exists());
    for (flow, label) in [("native", "control"), ("native-use", "candidate")] {
        assert_eq!(read(r(format!("{flow}/result.json")))["status"], "complete");
        for (p, n) in [("field", 33), ("core", 124), ("prover", 8)] {
            assert_eq!(
                read(r(format!("{flow}/{label}-{p}/tests.json")))["passed_tests"]
                    .as_array()
                    .unwrap()
                    .len(),
                n
            );
        }
    }
    for (p, h) in read(r("build-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let old = read(Path::new(OLD).join("context/manifest.json"));
    let mut pins = old["container_pins"].as_object().unwrap().clone();
    for p in [
        "/usr/local/bin/cr-current",
        "/usr/local/bin/cr-previous",
        "/usr/local/bin/cr-runner",
        "/usr/local/bin/pm-runner",
        "/usr/local/bin/cr-xtask",
    ] {
        pins.remove(p);
    }
    let c = r("context");
    fs::create_dir(&c).unwrap();
    fs::create_dir(c.join("basis")).unwrap();
    for (flow, label, name) in [
        ("native", "control", "cr-previous"),
        ("native-use", "candidate", "cr-current"),
    ] {
        let a = read(r(format!("{flow}/{label}/artifacts.json")));
        let b = a["binary"].as_str().unwrap();
        assert_eq!(hash(b), a["binary_sha256"].as_str().unwrap());
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
        fs::copy(b, c.join(name)).unwrap();
        pins.insert(format!("/usr/local/bin/{name}"), json!(hash(b)));
        let symbols = Command::new("nm").args(["-a", b]).output().unwrap();
        assert!(symbols.status.success());
        assert!(!String::from_utf8_lossy(&symbols.stdout).contains("__llvm_profile_"));
        assert!(!String::from_utf8_lossy(&symbols.stdout).contains("__llvm_prf"));
        fs::write(r(format!("{label}-symbols.txt")), symbols.stdout).unwrap();
    }
    for (from, name) in [
        (r("runner"), "aws-runner"),
        (
            PathBuf::from("/home/ubuntu/src/zopro-perf-work/tool-target/release/xtask"),
            "cr-xtask",
        ),
    ] {
        fs::copy(&from, c.join(name)).unwrap();
        pins.insert(format!("/usr/local/bin/{name}"), json!(hash(from)));
    }
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(Path::new(PILOT).join("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 18847);
    for (p, h) in read(Path::new(PILOT).join("proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 19261);
    let local = read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");
    let mut cases = vec![];
    for original in local.as_array().unwrap() {
        let image = Path::new(original["image"].as_str().unwrap());
        let basename = image.file_name().unwrap().to_str().unwrap();
        let request = Path::new(original["request"].as_str().unwrap());
        let filename = request.file_name().unwrap().to_str().unwrap();
        assert_eq!(hash(image), original["image_sha256"].as_str().unwrap());
        fs::copy(image, c.join("basis").join(basename)).unwrap();
        pins.insert(format!("/cr-basis/{basename}"), json!(hash(image)));
        pins.insert(format!("/fixtures/{filename}"), json!(hash(request)));
        let mut case = original.clone();
        case["image"] = json!(format!("/cr-basis/{basename}"));
        case["request"] = json!(format!("/fixtures/{filename}"));
        cases.push(case);
    }
    assert_eq!(cases.len(), 9);
    let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json");
    let mut helpers = BTreeMap::new();
    for f in [
        "design.md",
        "native.rs",
        "build-setup.rs",
        "build-setup-body.rs",
        "runner.rs",
        "support/config.rs",
        "support/results.rs",
        "cloud.rs",
        "transfer.rs",
        "sigv4.rs",
        "prepare-image.rs",
        "launch-template.json",
        "user-data-template.sh",
    ] {
        helpers.insert(f, hash(r(f)));
    }
    let m = json!({"scope":"current175 znver4 plain, unchanged proof-PGO profile consumer, pinned Aeglos0290613Sorted; all9shapes32/64/96;15blocks","source_files":175,"source_audit":read(r("source-audit.json")),"container_pins":pins,"parent_image_id":BASE,"parent_report":OLD,"prior_proof_sha256":prior,"environment":manifest["environment"],"cases":cases,"symbol_categories":old["symbol_categories"],"helper_sha256":helpers,"mode_map":{"plain":"previous","PGO":"current","aeglos":"aeglos"},"production_changed":false,"adopted":false});
    save(c.join("manifest.json"), &m);
    fs::copy(
        Path::new(OLD).join("aeglos-source-identity.json"),
        r("aeglos-source-identity.json"),
    )
    .unwrap();
    let docker=format!("FROM {BASE}\nCOPY cr-previous cr-current aws-runner cr-xtask /usr/local/bin/\nCOPY basis/ /cr-basis/\nCOPY manifest.json /cr/manifest.json\nENTRYPOINT [\"/usr/local/bin/aws-runner\"]\n");
    fs::write(c.join("Dockerfile"), &docker).unwrap();
    run(
        &[
            "docker".into(),
            "build".into(),
            "-t".into(),
            "zopro-proof-pgo-c7a:preflight".into(),
            c.display().to_string(),
        ],
        "docker-preflight-build",
    );
    fs::create_dir(r("local-output")).unwrap();
    run(
        &[
            "docker".into(),
            "run".into(),
            "--rm".into(),
            "--network".into(),
            "none".into(),
            "--cpus".into(),
            "4".into(),
            "--memory".into(),
            "30g".into(),
            "--memory-swap".into(),
            "30g".into(),
            "--pids-limit".into(),
            "2048".into(),
            "-v".into(),
            format!("{R}/local-output:/output"),
            "zopro-proof-pgo-c7a:preflight".into(),
            "preflight".into(),
        ],
        "preflight",
    );
    let complete = read(r("local-output/preflight/complete.json"));
    assert_eq!(complete["sdk_requests"], 108);
    assert_eq!(complete["distinct_gnark_exports"], 27);
    assert_eq!(complete["proof_ledger"], 19288);
    fs::copy(
        r("local-output/preflight/complete.json"),
        c.join("preflight-complete.json"),
    )
    .unwrap();
    fs::write(
        c.join("Dockerfile"),
        format!("{docker}COPY preflight-complete.json /cr/preflight-complete.json\n"),
    )
    .unwrap();
    let time = Command::new("date")
        .args(["-u", "+%Y%m%d-%H%M%S"])
        .output()
        .unwrap();
    assert!(time.status.success());
    let id = format!("pgo-c7a-{}", String::from_utf8(time.stdout).unwrap().trim());
    fs::write(r("run-id.txt"), &id).unwrap();
    let tag = format!("558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-{id}");
    run(
        &[
            "docker".into(),
            "build".into(),
            "-t".into(),
            tag.clone(),
            c.display().to_string(),
        ],
        "docker-final-build",
    );
    let out = Command::new("docker")
        .args(["image", "inspect", &tag, "--format", "{{.Id}}"])
        .output()
        .unwrap();
    assert!(out.status.success());
    let imageid = String::from_utf8(out.stdout).unwrap().trim().to_owned();
    save(
        r("image-manifest.json"),
        &json!({"image_tag":tag,"local_image_id":imageid,"source_files":175,"local_preflight_sdk":108,"local_gnark_proofs":27,"local_proof_ledger":19288,"status":"local preflight complete; not published or launched","aeglos_pin":"0290613e8eeca17a4ce88f6f2d726e8e06a5385d","plain_sdk_sha256":pins["/usr/local/bin/cr-previous"],"PGO_sdk_sha256":pins["/usr/local/bin/cr-current"]}),
    );
    println!("Imageprepared/local108SDK27Gnark ledger19288;no cloud resources");
}
