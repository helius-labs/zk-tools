use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-root-sharing-qualification176";
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
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let a = load(r.join("oracle.json"));
    let review = load(r.join("review.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(review["driver_linked_not_executed"], true);
    assert_eq!(
        hash(&fs::read(r.join("driver")).unwrap()),
        review["identities"]["driver"]["sha256"]
    );
    assert!(!r.join("capture-plan.json").exists());
    let art = d.join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Optimistic scalar-root sharing: arithmetic qualification\n\n**The bulk arithmetic passes correctness checks; performance is not measured.** The optimized driver is linked, but no request broker or production service is implemented. The next experiment asks whether a ready-root SIMD bulk can outperform independent inversions even with coordination costs removed.\n\n");
    report.push_str(&fs::read_to_string(r.join("decision.md")).unwrap());
    report.push_str("\n\n## Why the optimistic screen comes first\n\nCurrent workers can invert their own roots immediately. A coordinator can save summed arithmetic yet serialize independent work and lengthen the proof path. The current [workspace forecast](../../../crates/zopro-core/src/msm_affine/workspace.rs) assumes leaf kernels do not fork with live scratch. Making waiting workers execute arbitrary nested leaves would require a different lifetime/admission bound. Parking avoids that new retention, but workers stop progressing. No throughput improvement or twofold latency claim follows from the batch-inversion identity.\n\nAt 16 roots the wrapper multiplies two eight-lane vectors, uses the accepted lane inverse, then recovers both inverse vectors. At 32 it uses two vector pairs, a common root and a reverse tree. All roots are in the same BN254 Fq field, including G2 denominator norms; FFT Fr roots are outside scope. Root extraction and scalar inversion retain the accepted implementation, including canonicalization. No new scalar backend, IFMA multiplication, representation or key table is added.\n\n|Ready roots|Vector multiplications|Scalar root multiply|Scalar inversions|\n|---|---:|---:|---:|\n|1|0|0|1|\n|16|8|1|1|\n|32|14|1|1|\n\nThese are source operations, not instruction or latency predictions. The existing field library is immutable; the wrapper is a standalone feasibility consumer. Both independent controls will use the same native executable and identical individual-inversion code. They are repeated controls, not separately compiled accepted-SDK references. The shared arm runs one existing benchmark worker on CPU2 and parks the others, including its SMT sibling. It pays for arithmetic, packing/unpacking, output sums and start/end barriers; it pays no request publication, leader election, queue, polling or result handoff. That favors this implementation but is not a lower bound on every possible asynchronous strategy.\n\n## Independent arithmetic and state checks\n\nThe fixture contains 256 public rows of 32 canonical roots, 256 KiB of input data, generated from hashes and positive/negative boundary rows. It contains no precomputed inverse lookup used by timed code and is not a proposed prover table. Output qualification covers 12,544 active-root configurations at 1/16/32 roots, each checked for canonical form, multiplicative identity and equality with independent Arkworks inversion. All 49 possible invalid-root positions are rejected. A separate general batch model checks 776 cases and 37,248 root outputs across sizes 0 through 96 and eight patterns. It verifies the optimized 3(b−1) multiply count plus one inverse for nonempty batches. Empty batches perform neither. These are configurations, not a claim of 49,792 distinct root values or a new mathematical algorithm.\n\nThe conceptual state model is a sizing calculation only. A 64-byte aligned slot overlays input/result words and state/generation; cache-line pending-mask words and a leader header scale with worker count. No instance services requests, and no concurrency, safe reuse or cancellation protocol is qualified. The driver creates benchmark threads only, with 512 KiB configured stacks; that is not measured stack use or a production RAM bound.\n\n|Workers|Requested conceptual state bytes|Three Fq arrays + IDs payload bytes on a straightforward leader|\n|---|---:|---:|\n");
    for state in a["conceptual_state_only"].as_array().unwrap() {
        report.push_str(&format!(
            "|{}|{}|{}|\n",
            state["workers"],
            state["total_requested_bytes_without_allocator_rounding"],
            state["leader_arrays_payload_bytes_if_three_Fq_arrays_and_usize_ids"]
        ));
    }
    report.push_str("\nA single worker can omit the shared state entirely. The table excludes allocator rounding and compiler temporaries/spills and is not an actual service memory peak, full-proof RSS or minimum RAM. A real protocol must account for at least 16 payload loads/stores per request, release/acquire publication, shared atomic traffic, ownership, overlapping proofs and panic/cancellation. Those costs may erase any arithmetic saving.\n\n## Build and review evidence\n\nAll 176 accepted production source files remain exact. The accepted native field rlib SHA256 is `1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713`; it is reused, not rebuilt. Initial qualification pins 307 identities. Oracle build and execution succeed on first attempt. Driver linking initially fails because the ordinary helper serde/sha libraries contain no fat-LTO bitcode. Only those two library references change to the already frozen native dependencies; driver source, accepted field and O3/native/fatLTO/one-CGU flags remain exact. The repaired final link succeeds, with 317 repair pins. No successful oracle, field/core/prover suite or profile is repeated.\n\nThe first repair-plan helper also assumed a result file existed after failure. Its source/binary/logs are retained; the corrected helper binds the actual completed-step file. This changes metadata preparation only. Independent review checks all four native command attempts, actual argv/exits/stream hashes, source/field identities and the two library changes. It passes once. One unused fixture helper warning in the oracle is preserved without a cleanup rebuild. The final driver SHA256 is `40efd3811d9616e0f50ef46e5ada2b23b4622dedee51e82f4b9a2e9f7e0b96d3`. It is linked and has not been executed.\n\nNo timing protocol has been frozen yet. The [next stage](artifacts/next.md) specifies the prospective 1/16/32-worker contexts, identical controls, 15 paired blocks, all timing checks and automatic PMU/profiles to freeze before the first driver/preflight clock. A loss closes this ready-root implementation before broker engineering; a win selects further ownership/coordination feasibility, not production adoption or a full-proof speedup. Existing shared-norm, frontier/phase, chain and queue results remain distinct and closed.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-inverse-root-codec176/README.md").unwrap();
    let section = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&section[..section.find("\n## Continuation and artifacts").unwrap()]);
    report.push_str("\n\n## Artifacts\n\nAll new wrapper/fixture/driver/oracle sources and raw build/repair/review receipts are public as checked UTF8 members. Accepted sources and dependencies are bound by hashes. No executable/library blobs, private keys/images/witnesses/proofs, inverse-result table or third-party full text is embedded. There is no new performance, AWS/Aeglos/Gnark, huge/mobile/pressure/swap/minimum-RAM or throughput result. All eight CPU10× requirements remain active and unmet.\n\n[Design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [oracle result](artifacts/oracle.json), [review](artifacts/review.json), [bulk wrapper](artifacts/bulk.rs), [driver](artifacts/driver.rs), [fixture](artifacts/fixture.rs), [build plan](artifacts/build-plan.json), [build pins](artifacts/build-pins.json), [repair](artifacts/driver-repair.json), [repair plan](artifacts/driver-repair-plan.json), [repair pins](artifacts/driver-repair-pins.json), [helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).\n");
    fs::write(d.join("README.md"), report).unwrap();
    for f in [
        "design.md",
        "decision.md",
        "next.md",
        "oracle.json",
        "review.json",
        "bulk.rs",
        "fixture.rs",
        "oracle.rs",
        "driver.rs",
        "perf.rs",
        "prepare.rs",
        "review.rs",
        "publish.rs",
        "build-plan.json",
        "build-pins.json",
        "driver-repair.json",
        "driver-repair-plan.json",
        "driver-repair-pins.json",
        "repair-driver.rs",
    ] {
        fs::copy(r.join(f), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for p in walk(r) {
        if p.extension()
            .is_some_and(|s| ["rs", "md", "json", "log"].iter().any(|e| s == *e))
        {
            members.insert(p.strip_prefix(r).unwrap().display().to_string(), member(&p));
        }
    }
    let old_binary = r.join("repair-initial/repair-driver");
    let old_index = json!({"local_path":old_binary,"bytes":fs::metadata(&old_binary).unwrap().len(),"sha256":hash(&fs::read(&old_binary).unwrap()),"embedded":false});
    let bytes = serde_json::to_vec(&members).unwrap();
    let path = art.join("helpers-and-receipts.json.gz");
    let mut zip = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(&path).unwrap())
        .spawn()
        .unwrap();
    zip.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(zip.wait().unwrap().success());
    let back = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(&path)
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(back.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&back.stdout).unwrap();
    for e in decoded.as_object().unwrap().values() {
        let b = e["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), e["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), e["sha256"]);
    }
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"published":true,"qualification_only":true,"bulk_outputs":12544,"zero_rejections":49,"general_cases":776,"general_outputs":37248,"driver_linked_not_executed":true,"broker_implemented":false,"concurrency_qualified":false,"command_attempts":4,"successful_commands":3,"oracle_first_success_reused":true,"UTF8_archive_members":members.len(),"archive_roundtrip_checked":true,"archive_sha256":hash(&fs::read(&path).unwrap()),"initial_repair_binary_local_only":old_index,"new_proofs":0,"proof_ledger":30846,"adopted":false,"goal_achieved":false})).unwrap()).unwrap();
    let files = walk(d);
    let sums = files
        .iter()
        .map(|p| {
            format!(
                "{}  {}\n",
                hash(&fs::read(p).unwrap()),
                p.strip_prefix(d).unwrap().display()
            )
        })
        .collect::<String>();
    fs::write(d.join("SHA256SUMS"), sums).unwrap();
    println!(
        "Published{}files/{}UTF8members/hash-roundtripPASS; qualification only/no broker/clock/SDK",
        files.len() + 1,
        members.len()
    );
}
