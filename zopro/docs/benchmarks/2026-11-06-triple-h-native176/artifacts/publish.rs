use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-triple-h-native176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut v = Vec::new();
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            v.extend(walk(&p))
        } else {
            v.push(p)
        }
    }
    v.sort();
    v
}
fn archive(path: &Path, members: &BTreeMap<String, Value>) {
    let bytes = serde_json::to_vec(members).unwrap();
    let mut child = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(path).unwrap())
        .spawn()
        .unwrap();
    child.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(child.wait().unwrap().success());
    let out = Command::new("gzip")
        .args(["-d", "-c"])
        .arg(path)
        .output()
        .unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, bytes);
    let decoded: Value = serde_json::from_slice(&out.stdout).unwrap();
    for v in decoded.as_object().unwrap().values() {
        let b = v["utf8"].as_str().unwrap().as_bytes();
        assert_eq!(b.len(), v["bytes"].as_u64().unwrap() as usize);
        assert_eq!(hash(b), v["sha256"]);
    }
}
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let text = std::str::from_utf8(&b)
        .expect("only explicitly selected UTF8 helpers/source delta/receipts");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
}
fn main() {
    assert!(!Path::new(D).exists());
    let review = read(format!("{R}/review.json"));
    let codegen = read(format!("{R}/codegen.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(codegen["passed"], true);
    assert_eq!(
        review["binaries_sha256"]["caller"],
        codegen["caller_sha256"]
    );
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# H-only native unweighted G1 correctness screen\n\nThe cancelled-slope three-point formula now has a native IFMA implementation. Its focused correctness tests, independent Arkworks checks and small-stack worker tests pass. **No latency measurement or production change is included.** Accepted local1×2 remains **7.928660ms**; all-eight CPU10× target remains unmet0/8.\n\n## Change\n\nThe [previous algebra screen](../2026-11-06-triple-h-cancel176/README.md) eliminates the first slope and uses H rather than dH as the shared denominator. This implementation retains just preceding-prefix/H for each eight-lane group and rereads original coordinates in the reverse pass. Pair remainders share the same inverse chain and also use two-vector records. The previous triple record used eight vectors. This record-size change does not establish a total-memory or speed improvement.\n\nAll regions are checked for matching lengths, bounds and disjoint order before use. Identity inputs and a zero product reject the layer before any point/count writes. Ordinary and exceptional input continue through the unchanged complete binary reducer as needed. Reverse outputs stay in their original disjoint first-input slots; forward compaction happens only after all original coordinates have been consumed. The copied checked-inverse method is existing prior art, not a new optimization. Weighted binary reductions are unchanged.\n\nThe actual candidate snapshot has178files;173accepted files are byte-identical and the remaining3recover exactly by removing cfg-only appendices. Both new modules are isolated behind the experiment cfg. All176production files remain exact. The accepted field1200 and coherent support libraries were reused without rebuild. The original complete-caller fixture, order, Work/reduction loop and external driver are byte-exact. No lookup table, key copy, extra production pool or field rewrite is added.\n\n## Validation\n\nThe six-command Rust/xtask workflow completed on its first attempt. Five new focused native tests pass;125original core tests were filtered and are inherited, not reported as freshly run. The independent Arkworks complete-reduction test was executed against the changed native kernel. It covers full-width scalar arrays, empty/multiple groups, tails and216boundary triples, both selection paths and fresh/reused buffer lifetimes.\n\nThe focused tests check48partial-lane/remainder layouts, all255nonempty zero-lane masks plus a nonzero inverse case,72identity/equality/cancellation/pair rejections with unchanged inputs/counts, eight admitted opposite-first-pair lanes, and seven malformed geometries rejected before mutation. Separate1/4/16/32test workers each use512KiB stacks, repeatedly grow/shrink private scratch, and compare complete reductions and forced fallback outputs with complete projective group arithmetic. No unsafe blocks were added.\n\nThese test workers validate independent reduction scratch and stack behavior. They are not concurrent-proof throughput measurements or a general resource bound.\n\n## Memory and emitted code\n\nThe representative grow/shrink sequence retained the same capacity vector in every tested worker:\n\n| Capacity category | Bytes per worker |\n|---|---:|\n");
    let labels = [
        "Common points/counts/output",
        "Binary fallback scratch",
        "Ternary indices and both record buffers",
        "Fixed Work struct",
    ];
    let b = review["per_worker_retained_capacity_bytes"]
        .as_array()
        .unwrap();
    let mut sum = 0;
    for (label, n) in labels.iter().zip(b) {
        let n = n.as_u64().unwrap();
        sum += n;
        report.push_str(&format!("|{label}|{n}|\n"));
    }
    assert_eq!(sum, 413136);
    report.push_str(&format!("|Total reported capacity|{sum}|\n\nThis is about403.45KiB per worker, including binary fallback scratch that coexists with about59.73KiB of ternary scratch. It is **not peak RSS, minimum RAM, total process memory or a universal bound**. No equivalent fresh measurement of the old eight-vector implementation was added; this screen makes no total-memory comparison. Future runtime routing must use existing resource admission.\n\nCore and final-consumer commands explicitly use O3/native/oneCGU; final test/caller consumers use fatLTO. The inspected final caller is `{}`. Its H-only layer contains native IFMA operations and subtracts4936bytes from RSP, with48bytes of pushed registers. This prologue is not whole-thread stack usage or a dynamic spill cost. The saved disassembly was inspected without redecoding.\n\n## Next and limits\n\nThe next stage is a separately declared complete-caller timing comparison against matching original FROZEN and fresh binary OFF controls. Preserve Tile384/Tile511/Tile2048/Group35973 as main complete cases and boundary/mixed-tail diagnostics. Freeze identities, worker contexts, exact round/block/order plan and all calibration/nonregression/gain/resource gates before capture. No favorable subset, layout/attribute/threshold sweep or extra blocks are selected from these correctness results.\n\nFewer shared inversions may help; coordinate rereads, extra linear operations, scratch, stack and fallback coexistence can offset that saving. Only credible complete-caller gain selects a separate verified all-eight+54/physical16/portable/concurrent/resource SDK trial. Passing these tests does not permit production adoption.\n\nThe [old dH complete-caller result](../2026-10-28-unweighted-triple-caller174/README.md) and [old mixed SDK result](../2026-10-28-mixed-sdk/README.md) remain negative. The new denominator/formula changes arithmetic; it does not rewrite those failed gates. No timing/PMU/profile, SDK/Gnark proof, current AWS/Aeglos comparison, huge-circuit, phone/ARM/TEE, pressure/swap, process peak/minimum RAM or new2×path is established. Canonical proof ledger remains30,846.\n\n",review["binaries_sha256"]["caller"].as_str().unwrap()));
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("## Accepted CPU and Metal comparisons, unchanged")
        .unwrap()..];
    let section = &section[..section.find("\n[Analysis]").unwrap()];
    report.push_str(section);
    report.push_str("\n\n[Review](artifacts/review.json), [design](artifacts/design.md), [source identity manifest](artifacts/source-manifest.json), [native prefix](artifacts/prefix.rs), [focused tests](artifacts/tests.rs), [unchanged external driver](artifacts/probe.rs), [codegen review](artifacts/codegen.json), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [continuation](artifacts/next-timing.md), [five exact source deltas](artifacts/source-deltas.json.gz), [exact helpers/receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Unchanged source files are bound by accepted hashes; full snapshot and binaries stay local. No private input, witness, key/image/proof bytes or executable/library blobs are included.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "review.json",
        "review.rs",
        "codegen.json",
        "codegen.rs",
        "design.md",
        "source-manifest.json",
        "prefix.rs",
        "tests.rs",
        "probe.rs",
        "prepare.rs",
        "plan.json",
        "pins.json",
        "next-timing.md",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut deltas = BTreeMap::new();
    for p in [
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/simd.rs",
        "crates/zopro-core/src/msm/h_only_probe.rs",
        "crates/zopro-core/src/simd/h_checked.rs",
    ] {
        deltas.insert(p.to_owned(), member(&Path::new(R).join("source").join(p)));
    }
    archive(&art.join("source-deltas.json.gz"), &deltas);
    let mut helpers = BTreeMap::new();
    for e in fs::read_dir(R).unwrap() {
        let p = e.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    for p in walk(&Path::new(R).join("runs")) {
        helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &helpers);
    let pubv = json!({"review_passed":true,"codegen_passed":true,"source_deltas":deltas.len(),"helper_receipt_UTF8_members":helpers.len(),"all_archive_hashes_and_roundtrips_verified":true,"six_workflow_commands_first_success":true,"new_native_tests":5,"inherited_filtered_core_tests":125,"independent_Arkworks_tests":1,"new_proofs":0,"latency_measured":false,"production_changed":false,"goal_status":"active/unmet0/8"});
    fs::write(
        art.join("publication.json"),
        serde_json::to_vec_pretty(&pubv).unwrap(),
    )
    .unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(&fs::read(&p).unwrap()),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published{}files/5exactUTF8deltas/{}helper-receipt members; hash+roundtripPASS; native qualification, no latency/adoption",walk(Path::new(D)).len(),helpers.len());
}
