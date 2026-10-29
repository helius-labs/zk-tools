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
    let c = r("context");
    assert!(!r("local-output").exists());
    let pins = read(c.join("manifest.json"))["container_pins"].clone();
    let docker = fs::read_to_string(c.join("Dockerfile")).unwrap();
    let actual=Command::new("docker").args(["image","inspect","558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-pmu-c7a-20261027-123202","--format","{{.Id}}"] ).output().unwrap();
    assert!(actual.status.success());
    assert_eq!(String::from_utf8(actual.stdout).unwrap().trim(), BASE);
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
