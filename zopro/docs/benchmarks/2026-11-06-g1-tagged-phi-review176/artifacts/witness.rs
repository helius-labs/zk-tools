use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, sync::Arc};
#[global_allocator]
static ALLOCATOR: zopro_core::alloc::Allocator = zopro_core::alloc::Allocator;
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
fn main() {
    assert!(!std::path::Path::new(R).join("witness.bin").exists());
    let cases: Value =
        serde_json::from_slice(&fs::read(format!("{R}/cases.json")).unwrap()).unwrap();
    let c = &cases[0];
    assert_eq!(c["shape"], "1x2");
    let file = fs::File::open(c["image"].as_str().unwrap()).unwrap();
    let map = unsafe { memmap2::Mmap::map(&file).unwrap() };
    assert_eq!(format!("{:x}", Sha256::digest(&map[..])), c["image_sha256"]);
    let image: zopro_core::Image = Arc::new(map);
    let prover =
        zopro_zolana::Prover::new(zopro_prover::Prover::load_image(image).unwrap()).unwrap();
    let request = fs::read_to_string(c["request"].as_str().unwrap()).unwrap();
    let witness = prover.witness(&request).unwrap();
    let bytes = witness
        .iter()
        .flat_map(|x| x.0.into_iter().flat_map(u64::to_le_bytes))
        .collect::<Vec<_>>();
    fs::write(format!("{R}/witness.bin"), &bytes).unwrap();
    let out = json!({"witness_bytes":bytes.len(),"witness_sha256":format!("{:x}",Sha256::digest(&bytes)),"request_sha256":format!("{:x}",Sha256::digest(request.as_bytes())),"image_sha256":c["image_sha256"],"new_solver_calls":0,"new_proofs":0});
    fs::write(
        format!("{R}/witness-info.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!(
        "Accepted SDK witness prepared once;{}bytes; no solver/proof",
        bytes.len()
    );
}
