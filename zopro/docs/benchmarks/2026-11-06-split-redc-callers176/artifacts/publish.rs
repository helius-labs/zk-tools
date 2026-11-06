use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-callers176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-split-redc-callers176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else {
            out.push(p);
        }
    }
    out.sort();
    out
}
fn main() {
    assert!(!Path::new(D).exists());
    assert!(!Path::new(R).join("publication.json").exists());
    let q = read(format!("{R}/qualification.json"));
    let c = read(format!("{R}/consumer-chains.json"));
    assert_eq!(q["status"], "pass");
    assert_eq!(c["status"], "pass");
    assert_eq!(q["successful_unique_workflow_commands"], 12);
    assert_eq!(q["new_decodes"], 6);
    assert_eq!(q["core_ON_unfiltered_tests"], 126);
    let pins = read(format!("{R}/qualification-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(h, &json!(hash(p)));
    }
    let mut ids = q["identities"].clone();
    let f = read("/home/ubuntu/src/zopro-perf-work/g2-validated-capture176/identities.json");
    assert_eq!(
        f["frozen"]["binary_sha256"],
        hash(f["frozen"]["binary"].as_str().unwrap())
    );
    ids["frozen"] = f["frozen"].clone();
    save(format!("{R}/identities.json"), &ids);
    let audit = read(format!("{R}/source-audit.json"));
    save(
        format!("{R}/source-deltas.json"),
        &json!({"snapshot_files":178,"native_ancestor_files":177,"native_ancestor_unchanged":175,"core_runtime_arithmetic_original":true,"two_core_append_suffixes":audit["suffixes"],"new_adapter":audit["adapter_sha256"],"new_field_global_private_cfg":"zopro_split_redc_planes","field_qualification":"/home/ubuntu/src/zopro-perf-work/split-redc-native176/qualification.json","original176_inverse_recovery":"native field three inverse edits plus strip two core diagnostic suffixes","driver_only_cfg_edit":true,"no_rejected_core_kernel_inherited":true}),
    );
    let mut summary = vec![];
    for role in ["OFF", "ON"] {
        for kind in ["G1", "G2", "inverse"] {
            let r = q["rows"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["role"] == role && r["kind"] == kind)
                .unwrap();
            let g = c["functions"]
                .as_array()
                .unwrap()
                .iter()
                .find(|r| r["role"] == role && r["kind"] == kind)
                .unwrap();
            assert_eq!(r["disassembly_sha256"], g["source_sha256"]);
            assert_eq!(r["ifma"], r["expected_nominal_ifma"]);
            summary.push(json!({"role":role,"kind":kind,"instruction_rows":r["instruction_rows"],"ifma":r["ifma"],"add":r["add"],"sub":r["sub"],"vector_stack_operand_rows":r["vector_rsp_rbp_operand_rows"],"code_bytes":r["size_bytes"],"partial_connected_register_score":g["max_connected_calibrated_score"]}));
        }
    }
    let review = json!({"status":"pass","coherent_field_core_driver_pairs":true,"core_ON_unfiltered_tests":126,"core_OFF_new_test":1,"source_files":178,"twelve_unique_successful_stages":true,"failed_driver_link_preserved":true,"new_consumer_decodes":6,"reused_frozen_decodes":3,"new_profiles":0,"new_proofs":0,"codegen_summary":summary,"timing_claim":false,"production_changed":false,"proof_ledger":30846});
    save(format!("{R}/publication-review.json"), &review);
    let out = Path::new(D).join("artifacts");
    fs::create_dir_all(&out).unwrap();
    let changed = [
        "source/crates/zopro-bn254/src/ifma.rs",
        "source/crates/zopro-bn254/src/ifma8.rs",
        "source/crates/zopro-bn254/src/split_redc_tests.rs",
        "source/scripts/ifma8.py",
        "source/crates/zopro-core/src/lib.rs",
        "source/crates/zopro-core/src/msm.rs",
        "source/crates/zopro-core/src/msm/validated_caller.rs",
    ];
    let mut members = BTreeMap::new();
    for p in walk(Path::new(R)) {
        let rel = p.strip_prefix(R).unwrap().to_str().unwrap();
        let keep = if rel.starts_with("source/") {
            changed.contains(&rel)
        } else {
            p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log", "txt"].iter().any(|s| e == *s))
        };
        if keep {
            members.insert(rel.to_string(), fs::read_to_string(p).unwrap());
        }
    }
    let archive = json!({"files":members});
    save(format!("{R}/source-and-receipts.json"), &archive);
    let out_file = fs::File::create(out.join("source-and-receipts.json.gz")).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(format!("{R}/source-and-receipts.json"))
        .stdout(out_file)
        .status()
        .unwrap()
        .success());
    let round = Command::new("gzip")
        .arg("-dc")
        .arg(out.join("source-and-receipts.json.gz"))
        .output()
        .unwrap();
    assert!(round.status.success());
    assert_eq!(
        round.stdout,
        fs::read(format!("{R}/source-and-receipts.json")).unwrap()
    );
    let index = archive["files"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                format!("{:x}", Sha256::digest(v.as_str().unwrap().as_bytes())),
            )
        })
        .collect::<BTreeMap<_, _>>();
    save(out.join("archive-index.json"), &json!(index));
    save(out.join("review.json"), &review);
    save(out.join("qualification.json"), &q);
    for f in [
        "design.md",
        "qualify.rs",
        "publish.rs",
        "setup.rs",
        "consumer-chains.rs",
        "graph.rs",
        "identities.json",
        "source-audit.json",
        "source-deltas.json",
        "build-plan.json",
        "repair-plan.json",
        "qualification-pins.json",
        "repair-field-search.md",
        "prebuild-freeze-note.md",
    ] {
        fs::copy(format!("{R}/{f}"), out.join(f)).unwrap();
    }
    let mut md=String::from("# Split REDC planes: actual point-caller qualification\n\nThe fresh ON core passes **126 unfiltered tests**. The OFF core passes the shared signed-tail/reuse oracle, and matching OFF/ON field, core and driver identities are frozen and verified. The actual consumer code is emitted and inspected. **No caller timing, full-proof gain or production adoption is claimed.**\n\nThe [native field qualification](../2026-11-06-split-redc-native176/README.md) and [mathematical/dependency screen](../2026-11-06-split-redc-planes176/README.md) are reused without rerunning field tests or models. The 178-file snapshot copies the qualified native source, changes two core parents only by diagnostic module/export suffixes, and adds the existing caller adapter. Production core arithmetic remains byte-exact. The adapter calls the original G1 stream and signed borrowed G2 routine; no rejected validated-G2 kernel or dispatch is copied. Both eight-lane fields/radices retain the new global candidate scope.\n\n## Actual linked code\n\nSix newly decoded consumers cover G1, G2 and vector inversion in both drivers. Three frozen-control decodes are inherited unchanged. Nominal IFMA counts survive, while additional additions and vector stack operands remain visible. Counts are static instruction locations, not retired work, measured spills, memory peaks or minimum RAM.\n\n| Arm / consumer | Instruction rows | IFMA sites | Vector adds | Vector stack operands | Code bytes | Partial register-chain score |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for r in &summary {
        md.push_str(&format!(
            "| {} / {} | {} | {} | {} | {} | {} | {} |\n",
            r["role"].as_str().unwrap(),
            r["kind"].as_str().unwrap(),
            r["instruction_rows"],
            r["ifma"],
            r["add"],
            r["vector_stack_operand_rows"],
            r["code_bytes"],
            r["partial_connected_register_score"]
        ));
    }
    md.push_str("\nThe partial register-chain score uses the previously checked graph helper byte-for-byte with calibrated weights. It omits unknown/masked/memory/copy/inter-block edges. The lower ON scores may partly reflect extra memory boundaries breaking those graphs; they do **not** prove shorter whole-kernel hardware latency or compiler realization of every ideal source dependency. Actual matched measurements must decide whether the dependence change outweighs added work and stack traffic. No old profile is transferred to these binaries.\n\n## Validation and retained repair\n\nFour initial workflow stages succeeded once: OFF core build/freeze, test build and the shared tail/reuse test (one passed, 125 inherited tests filtered). Driver linking then failed because its inherited search path did not contain the new transitive field library. The failed command and original protocol are retained. A separate eight-stage continuation adds only the matching field search path and explicit field extern to the failed/unexecuted driver builds; all other remaining arguments stay identical. The four successful stages are reused. The continuation succeeds, including the full ON suite of 126 tests with zero filtered. All 12 unique successful stages and the one failed link are reconciled.\n\nA pre-build helper correction gave frozen core libraries valid `libzopro_core-HASH.rlib` names; no preparation or native work was repeated for it. Source inverse edits, actual command arguments, compiled-arm cfg, field/core/driver hashes and the inherited generator oracle are checked. Both caller executables are frozen immediately after compilation. The retained frozen accepted-source comparator is `/home/ubuntu/src/zopro-perf-work/word-export176/caller-frozen`; it is not an SDK or a claim of original field-library binary identity.\n\n## Next measurement\n\nDeclare a complete matched caller protocol before clocks: frozen accepted-source control, fresh OFF and ON; both curves; 200/201/352/353 points; 1/16/32 workers; fixed paired blocks with signed/tail/generator/output/capacity/zero-allocation guards. Retain every context and control-calibration failure. No unchanged width/layout/attribute or favorable-subset rescue. Only a positive complete-caller result permits SDK/all-shape/Gnark/resource/portable qualification.\n\nAccepted production remains unchanged: local 1x2 is 7.928660 ms and merge54 is 161.883899 ms. The inherited 132.75 MiB RSS is not a new measurement or minimum RAM. The [all-eight CPU/Metal comparison](../2026-11-06-split-redc-planes176/README.md#unchanged-full-proof-baseline) remains unchanged; the CPU 10× goal is active and unmet on all eight shapes. Proof ledger remains 30,846. There is no new AWS/Aeglos, SDK/Gnark proof, portability, pressure or mobile result.\n\n[Qualification and all static rows](artifacts/qualification.json), [publication review](artifacts/review.json), [design](artifacts/design.md), [qualifier/codegen source](artifacts/qualify.rs), [connected-chain analysis](artifacts/consumer-chains.rs), [reused graph helper](artifacts/graph.rs), [identities](artifacts/identities.json), [source audit](artifacts/source-audit.json), [source deltas](artifacts/source-deltas.json), [original build plan](artifacts/build-plan.json), [continuation plan](artifacts/repair-plan.json), [repair notes](artifacts/repair-field-search.md), [input/receipt pins](artifacts/qualification-pins.json), [source/receipt archive](artifacts/source-and-receipts.json.gz) and [member hashes](artifacts/archive-index.json). The archive includes all six new consumer decodes, full chain output, changed/added text sources and both successful and failed command records; binary/key/witness/proof blobs are not redistributed.\n");
    fs::write(Path::new(D).join("README.md"), md).unwrap();
    let published = walk(Path::new(D));
    let mut sums = String::new();
    for p in &published {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    save(
        format!("{R}/publication.json"),
        &json!({"status":"pass","report":D,"files":published.len()+1,"archive_members":members.len(),"roundtrip_exact":true,"new_proofs":0,"production_changed":false}),
    );
    println!(
        "published{}files/{}UTF8members; 12unique successful stages/6new decodes/ON126; no timing",
        published.len() + 1,
        members.len()
    );
}
