use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs,
    path::Path,
    process::{Command, Stdio},
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-current-c7a-20261027");
    let complete = read(r.join("local-output/preflight/complete.json"));
    assert_eq!(complete["status"], "complete");
    assert_eq!(complete["sdk_requests"], 108);
    assert_eq!(complete["distinct_gnark_exports"], 27);
    let inventory = read(r.join("local-output/preflight/proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 54);
    let mut hashes = BTreeSet::new();
    for (p, h) in inventory.as_object().unwrap() {
        let local = r
            .join("local-output")
            .join(p.strip_prefix("/output/").unwrap());
        assert_eq!(hash(&local), h.as_str().unwrap());
        if p.ends_with(".proof") {
            assert!(hashes.insert(h.as_str().unwrap().to_owned()));
        }
    }
    assert_eq!(hashes.len(), 27);
    assert_eq!(complete["all_fixture_checks_passed"], true);
    let receipt = json!({"status":"complete","sdk_requests":108,"distinct_gnark_exports":27,"all_fixture_checks_passed":true,"new_proof_sha256":hashes,"source_manifest_sha256":hash(r.join("context/manifest.json")),"inventory_sha256":hash(r.join("local-output/preflight/proof-inventory.json")),"same_prover_binaries_in_final_image":true});
    fs::write(
        r.join("preflight-complete.json"),
        serde_json::to_vec_pretty(&receipt).unwrap(),
    )
    .unwrap();
    let ctx = r.join("final-context");
    fs::create_dir(&ctx).unwrap();
    fs::copy(
        r.join("preflight-complete.json"),
        ctx.join("preflight-complete.json"),
    )
    .unwrap();
    fs::write(ctx.join("Dockerfile"),"FROM zopro-current-c7a:preflight\nCOPY preflight-complete.json /cr/preflight-complete.json\n").unwrap();
    let run_id = Command::new("date")
        .args(["-u", "+%Y%m%d-%H%M%S"])
        .output()
        .unwrap();
    assert!(run_id.status.success());
    let id = format!(
        "cr-c7a-{}",
        String::from_utf8(run_id.stdout).unwrap().trim()
    );
    assert!(id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-'));
    let tag = format!("558215002830.dkr.ecr.eu-north-1.amazonaws.com/zolana-prover:bench-{id}");
    let log = fs::File::create(r.join("docker-final-build.log")).unwrap();
    let err = log.try_clone().unwrap();
    assert!(Command::new("docker")
        .args(["build", "--pull=false", "-t", &tag])
        .arg(&ctx)
        .stdout(Stdio::from(log))
        .stderr(Stdio::from(err))
        .status()
        .unwrap()
        .success());
    let o = Command::new("docker")
        .args(["image", "inspect", &tag, "--format", "{{.Id}}"])
        .output()
        .unwrap();
    assert!(o.status.success());
    let image_id = String::from_utf8(o.stdout).unwrap().trim().to_owned();
    fs::write(r.join("run-id.txt"), &id).unwrap();
    fs::write(r.join("image-manifest.json"),serde_json::to_vec_pretty(&json!({"status":"local exact-container preflight complete; not published or launched","image_tag":tag,"image_id":image_id,"run_id":id,"local_gnark_proofs":27,"local_preflight_sdk":108,"preflight_receipt_sha256":hash(r.join("preflight-complete.json")),"container_manifest_sha256":hash(r.join("context/manifest.json"))})).unwrap()).unwrap();
    println!(
        "Final image prepared: {tag};27Gnark/108SDK local preflight passed, no cloud resources"
    );
}
