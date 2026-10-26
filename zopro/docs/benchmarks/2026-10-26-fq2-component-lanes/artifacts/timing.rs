use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    let build: Value =
        serde_json::from_slice(&fs::read(root.join("build-v2/result.json")).unwrap()).unwrap();
    assert_eq!(build["status"], "complete");
    let patterns = [
        "chain",
        "four-streams",
        "scaled-product",
        "affine-expression",
    ];
    let modes = ["reference", "interleaved"];
    let mut steps = Vec::new();
    let mut order = Vec::new();
    for block in 0..31 {
        for pi in 0..4 {
            let pattern = patterns[(pi + block) % 4];
            let mut mi = [(pi + block) % 2, (pi + block + 1) % 2];
            if block % 2 == 1 {
                mi.reverse();
            }
            for m in mi {
                let mode = modes[m];
                let name = format!("b{block:02}-{pattern}-{mode}");
                let argv = json!([
                    "taskset",
                    "-c",
                    "2",
                    root.join("probe"),
                    mode,
                    pattern,
                    "1000000"
                ]);
                steps.push(json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":60}));
                order.push(json!({"name":name,"block":block,"pattern":pattern,"mode":mode}));
            }
        }
    }
    assert_eq!(steps.len(), 248);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(root.join("input-pins-v2.json")).unwrap()).unwrap();
    for name in [
        "probe",
        "oracle-tests",
        "timing.rs",
        "analyze.rs",
        "repair.json",
        "build-plan-v2.json",
        "build-v2/result.json",
        "build-v2/oracle-tests/stdout.log",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    fs::write(
        root.join("timing-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        root.join("timing-order.json"),
        serde_json::to_vec_pretty(&order).unwrap(),
    )
    .unwrap();
    fs::write(root.join("timing-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":root.join("timing-input-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    let heading=format!("# CURRENT — 2026-10-26: FQ2 COMPONENT-LANE EXPERIMENT RUNNING; LAST PUSH f30e95e9\n\nGoal ACTIVE/UNMET, no budget. New private component root {}. Frozen protocol experiment.json; four Fq2 elements packed into real/imaginary halves of one vector vs eight elements in two vectors. Existing accepted field kernels only; production170source snapshot and qualifiedSDK unchanged. No tables/ASM/C/prover dispatch. Four independent tests pass against frozen accepted BN254 library:212992 operation-input lanes (12 complex outputs +1 norm per input lane),864 corner checks,384 Arkworks curve additions,20 changing-state comparisons. Initial test compile missing AdditiveGroup import repaired before tests/timing, source/logs preserved. Build-v2 complete. Fixed31blocks4patterns2modes1million iterations each,248 processes then2profiles, exact-word comparisons all outputs. Running Rust xtask workflow; commit/push each finished experiment. No newAWS. This supersedes older CURRENT headers until closed.\n\n",root.display());
    let handoff = Path::new("/home/ubuntu/src/zopro-cpu-perf-handoff.md");
    let old = fs::read_to_string(handoff).unwrap();
    fs::write(handoff, heading + &old).unwrap();
    println!(
        "Frozen 248 process order and {} pins; component probe {}",
        pins.len(),
        hash(root.join("probe"))
    );
}
