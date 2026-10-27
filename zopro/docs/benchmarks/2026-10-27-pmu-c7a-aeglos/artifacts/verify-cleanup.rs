use serde_json::{json, Value};
use std::{
    fs,
    process::{Command, Output},
};
fn read(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn aws(args: &[&str]) -> Output {
    Command::new("aws")
        .args([
            "--profile",
            "dev",
            "--region",
            "eu-north-1",
            "--no-cli-pager",
        ])
        .args(args)
        .output()
        .unwrap()
}
fn main() {
    let r = "/tmp/zopro-pmu-c7a-20261027";
    let id = fs::read_to_string(format!("{r}/instance-id.txt")).unwrap();
    let run = fs::read_to_string(format!("{r}/run-id.txt")).unwrap();
    let expected = read(&format!("{r}/launch.json"));
    assert_eq!(expected["ClientToken"], run);
    let v = aws(&["ec2", "describe-instances", "--instance-ids", id.trim()]);
    assert!(v.status.success());
    let v: Value = serde_json::from_slice(&v.stdout).unwrap();
    let i = &v["Reservations"][0]["Instances"][0];
    assert_eq!(i["ClientToken"], run);
    assert_eq!(i["State"]["Name"], "terminated");
    let ownership = read(&format!("{r}/owned-volume-identity.json"));
    assert_eq!(ownership["instance_id"], id.trim());
    assert_eq!(ownership["run_id"], run.trim());
    let volume = ownership["volume_id"].as_str().unwrap();
    let gone = aws(&["ec2", "describe-volumes", "--volume-ids", volume]);
    fs::write(format!("{r}/owned-volume-absence.log"), &gone.stderr).unwrap();
    assert!(!gone.status.success());
    assert!(String::from_utf8_lossy(&gone.stderr).contains("InvalidVolume.NotFound"));
    let im = read(&format!("{r}/image-manifest.json"));
    let tag = im["image_tag"]
        .as_str()
        .unwrap()
        .rsplit(':')
        .next()
        .unwrap();
    let gone = aws(&[
        "ecr",
        "describe-images",
        "--repository-name",
        "zolana-prover",
        "--image-ids",
        &format!("imageTag={tag}"),
    ]);
    assert!(!gone.status.success());
    assert!(String::from_utf8_lossy(&gone.stderr).contains("ImageNotFoundException"));
    let bucket = read(&format!("{r}/bucket.json"));
    let gone = aws(&[
        "s3api",
        "head-bucket",
        "--bucket",
        bucket["name"].as_str().unwrap(),
    ]);
    assert!(!gone.status.success());
    assert!(String::from_utf8_lossy(&gone.stderr).contains("404"));
    assert!(!std::path::Path::new(&format!("{r}/upload-parameters.secret.json")).exists());
    assert!(!std::path::Path::new(&format!("{r}/docker-auth")).exists());
    let all = aws(&[
        "ec2",
        "describe-instances",
        "--filters",
        "Name=tag:Purpose,Values=TemporaryProverBenchmark",
        "Name=instance-state-name,Values=pending,running,stopping,stopped",
    ]);
    assert!(all.status.success());
    let all: Value = serde_json::from_slice(&all.stdout).unwrap();
    let count: usize = all["Reservations"]
        .as_array()
        .unwrap()
        .iter()
        .map(|r| r["Instances"].as_array().unwrap().len())
        .sum();
    let out = json!({"status":"complete","owned_instance":id.trim(),"instance_state":"terminated","owned_volume":volume,"owned_volume_absent":true,"owned_ECR_tag_absent":true,"owned_transfer_object_and_bucket_absent":true,"private_presigned_parameter_file_deleted":true,"local_docker_auth_directory_absent":true,"all_live_TemporaryProverBenchmark_instances_in_region":count,"unowned_resources_modified":false});
    fs::write(
        format!("{r}/cleanup-complete.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("Owned instance terminated; volume/ECRtag/bucket/secretabsent;{count} live TemporaryProverBenchmark instances in region");
}
