use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/stream-work-census176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-stream-work-census176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p));
        } else if p.is_file() {
            v.push(p);
        }
    }
    v.sort();
    v
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let s = std::str::from_utf8(&b).unwrap();
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":s})
}
fn main() {
    let root = Path::new(R);
    let dest = Path::new(D);
    assert!(!dest.exists());
    let a = load(root.join("result.json"));
    let review = load(root.join("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(
        review["result_sha256"],
        hash(&fs::read(root.join("result.json")).unwrap())
    );
    let art = dest.join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Production streaming-update arithmetic census\n\n**The mathematical census passes; no optimization or speedup is selected.** It binds the current accepted G1/G2 streaming bodies to a standalone Arkworks model, checks full batches and tails, and distinguishes multiply, square and fused-product work. The [preceding root-product experiment](../2026-11-06-inverse-root-callers176/README.md) remains rejected.\n\n");
    report.push_str(&fs::read_to_string(root.join("decision.md")).unwrap());
    report.push_str("\n\n## Model and independent checks\n\nOne normal Rust/xtask build and one execution complete successfully once. The model uses Arkworks field arithmetic to follow the source operation schedule, with independent projective addition as the point oracle. It is not an execution of the production SIMD kernel. Five accepted source files match the production176 manifest; the separate reviewer checks seven groups of exact source statements, 77 immutable pins, actual build/run argv/exits/stream hashes, all 168 case identities, count formulas and 72 inherited capacity observations. No original native test, proof or profile is rerun.\n\nSizes are 0, 1, 7, 8, 9, 15, 16, 199, 200, 201, 351, 352, 353 and 511 for both curves, all-positive/all-negative/mixed signs and two successive updates. Hashed full-width public scalar offsets and generator walks construct the points. All 26,676 ordinary point outputs equal Arkworks group addition. Eight explicit identity/equal/opposite classifications cover the precondition distinction; they do not qualify the native exception dispatcher. Scalar-tail field operations are counted separately from vector operations.\n\nFor g=floor(n/8), t=n%8, V=one eight-lane base-field multiply, F=one eight-lane fused sum/difference of two products, M/S/Fs=scalar multiply/square/fused2 and I=scalar inverse:\n\n|Curve|Vector work|Scalar work|Coordinate pack/export calls|\n|---|---|---|---|\n|G1|(6g+5)V|(5t+1)M+tS+I|4g / 2g|\n|G2|(7g+5)V+5gF|(8t+1)M+2tS+4tFs+I|12g / 4g|\n\nThe fixed five V plus M and I form the common lane-inverse bridge. Coordinate pack/export counts exclude that bridge and describe source calls, not machine memory operations. An IFMA-source proxy weights V=100 and F=150. It excludes scalar arithmetic, adds/carries/corrections, packing and control; it is not a runtime or stall model.\n\n|Curve / points|V|F|M|S|Fs|I|Vector IFMA proxy|Coordinate packs / exports|\n|---|---:|---:|---:|---:|---:|---:|---:|---|\n");
    for kind in ["G1", "G2"] {
        for n in [200, 201, 352, 353] {
            let key = format!("{kind}-{n}");
            let c = &a["source_operation_counts"][&key];
            report.push_str(&format!(
                "|{kind} / {n}|{}|{}|{}|{}|{}|{}|{}|{} / {}|\n",
                c["vector_mul"],
                c["vector_fused2"],
                c["scalar_mul"],
                c["scalar_square"],
                c["scalar_fused2"],
                c["scalar_inverse"],
                c["vector_IFMA_source_proxy"],
                c["coordinate_vector_packs"],
                c["coordinate_vector_exports"]
            ));
        }
    }
    report.push_str("\n## What this changes about the next experiment\n\nThe source model makes a proposed arithmetic saving checkable before another native trial. Moving the slope scale changes the dependency graph without reducing these products. Three separately reduced Karatsuba products and two fused2 outputs have equal 300-IFMA proxies; a raw-product/combined-reduction variant is a different circuit and already has recorded native negatives. Neither fewer mathematical products nor shorter source code establishes a machine gain. Known-unit endpoints and independent prefix chains have already been measured and stay closed.\n\nThe [Eisentraeger–Lauter–Montgomery paper](https://arxiv.org/abs/math/0208038v2) saves a multiplication for the repeated-input operation 2P+Q. [Barretenberg documents](https://barretenberg.aztec.network/api/namespacebb_1_1group__elements) its chord-only batch realization with two inversion stages and explicit nonexceptional preconditions. Those conditions do not hold for arbitrary bucket P+Q. Its [sequence reducer](https://barretenberg.aztec.network/api/classbb_1_1_batched_affine_addition) groups additions and shares denominators; corresponding mechanisms already exist here. No external performance figure or full paper/source is copied.\n\nThe history also already contains [streaming IBS operation/fetch evidence](../2026-10-29-stream-ibs175/README.md), [unit-prefix peeling](../2026-10-31-unit-prefix176/README.md) and [G1 prefix chains](../2026-10-22-universal-g1-prefix-chains-sdk/README.md). Their captures, mathematical qualifications, positive estimates and failed gates are inherited. No duplicate frontend/memory profile or unchanged small-field candidate follows from this census.\n\nThe next distinct feasibility question is opportunistic scalar-root inversion sharing across existing workers, subject to targeted history checks, exact arithmetic/canonical-output validation, fixed proof-owned state, ownership/coordination/cancellation costs and actual critical-path coverage. No new thread, key table or production broker is implemented or approved by this report. One batch replacing b inversions requires one inverse plus 3(b−1) multiplies; that identity alone does not establish a practical gain. The [next stage](artifacts/next.md) requires this screen before any native implementation or timing.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-root-codec176/README.md").unwrap();
    let s = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&s[..s.find("\n## Continuation and artifacts").unwrap()]);
    report.push_str("\n\n## Artifacts\n\nThe standalone model and complete raw build/run receipts are public. Accepted production bodies are bound by their existing source manifest and five hashes; no production source delta is introduced. All helper members are UTF8 and roundtrip/hash checked. No compiled libraries/binaries, key/image/witness/proof bytes or third-party full text is embedded. This census adds no performance, native-kernel, AWS/Aeglos/Gnark, device, low-memory or throughput qualification.\n\n[Result](artifacts/result.json), [independent review](artifacts/review.json), [model](artifacts/model.rs), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).\n");
    fs::write(dest.join("README.md"), report).unwrap();
    for f in [
        "result.json",
        "review.json",
        "model.rs",
        "review.rs",
        "prepare.rs",
        "publish.rs",
        "design.md",
        "decision.md",
        "next.md",
        "plan.json",
        "pins.json",
    ] {
        fs::copy(root.join(f), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for p in fs::read_dir(root).unwrap() {
        let p = p.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|s| ["rs", "md", "json", "log"].iter().any(|e| s == *e))
        {
            members.insert(
                p.strip_prefix(root).unwrap().display().to_string(),
                member(&p),
            );
        }
    }
    for p in walk(&root.join("runs")) {
        members.insert(
            p.strip_prefix(root).unwrap().display().to_string(),
            member(&p),
        );
    }
    let bytes = serde_json::to_vec(&members).unwrap();
    let path = art.join("helpers-and-receipts.json.gz");
    let mut gzip = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(&path).unwrap())
        .spawn()
        .unwrap();
    gzip.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(gzip.wait().unwrap().success());
    let back = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(back.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&back.stdout).unwrap();
    for e in decoded.as_object().unwrap().values() {
        let text = e["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(text.len(), e["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(text), e["sha256"]);
    }
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"published":true,"model_only":true,"native_kernel_executed":false,"new_performance_capture":false,"new_proofs":0,"ledger":30846,"batch_executions":168,"point_outputs_checked":26676,"guard_classifications":8,"source_bindings":7,"inherited_capacity_comparisons":72,"UTF8_archive_members":members.len(),"archive_roundtrip_checked":true,"archive_sha256":hash(&fs::read(&path).unwrap()),"adopted":false,"goal_achieved":false})).unwrap()).unwrap();
    let files = walk(dest);
    let sums = files
        .iter()
        .map(|p| {
            format!(
                "{}  {}\n",
                hash(&fs::read(p).unwrap()),
                p.strip_prefix(dest).unwrap().display()
            )
        })
        .collect::<String>();
    fs::write(dest.join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/{}UTF8members/archive roundtrip PASS; mathematical census only/no new native or speed claim",files.len()+1,members.len());
}
