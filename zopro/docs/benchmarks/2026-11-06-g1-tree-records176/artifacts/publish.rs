use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-records176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tree-records176";
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
fn member(p: &Path) -> Value {
    let b = fs::read(p).unwrap();
    let s = std::str::from_utf8(&b).expect("explicit text members only");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":s})
}
fn archive(path: &Path, members: &BTreeMap<String, Value>) {
    let bytes = serde_json::to_vec(members).unwrap();
    let mut p = Command::new("gzip")
        .args(["-n", "-c"])
        .stdin(Stdio::piped())
        .stdout(fs::File::create(path).unwrap())
        .spawn()
        .unwrap();
    p.stdin.take().unwrap().write_all(&bytes).unwrap();
    assert!(p.wait().unwrap().success());
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
fn main() {
    assert!(!Path::new(D).exists());
    let review = read(format!("{R}/review.json"));
    let codegen = read(format!("{R}/codegen.json"));
    let repair = read(format!("{R}/review-metadata-repair.json"));
    assert_eq!(review["review_passed"], true);
    assert_eq!(codegen["passed"], true);
    assert_eq!(repair["passed"], true);
    assert_eq!(review["portable_original_tests_filtered"], 101);
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# G1 binary-tree prefix records: native qualification\n\nThe existing two-vector binary-tree prefix path is enabled for native G1 in a private candidate. **Correctness and capacity qualification passes; performance has not been measured.** Accepted local 1×2 remains **7.928660 ms** and production is unchanged. The CPU 10× objective remains unmet for all eight supplied shapes.\n\n## Change and rationale\n\nThe [preceding complete-caller study](../2026-11-06-triple-h-callers176/README.md) found a large working-set benefit but small-tile regressions for a ternary formula. That blanket ternary replacement remains closed. This candidate preserves binary pairs and uses the existing generic prefix/norm tree body, previously enabled only for G2. The earlier [G1 streaming-prefix result](../2026-10-25-prefix-records-component/README.md) and [old G2 tree confirmation](../2026-10-27-g2-tree-records/README.md) retain their original negative decisions. Actual inspected source distinguishes these placements; no exhaustive novelty claim is made.\n\nG1 selects prefix records for every full native eight-lane group and reserves exactly the required count. The legacy Group vector stays unallocated throughout grow/shrink and exceptional workloads. This is a fixed storage-lifetime design: an amortized reserve could allocate a four-element minimum for one 640-byte record pair, exceeding the original 1,600-byte single group. Exact reservation avoids that overhead and amortized growth. G2 retains its threshold of 32 ordinary pairs and its original reservation policy.\n\nThe scalar tail, pair selection, exceptional handling, reverse writes, post-reverse compaction, weighted copied-even-index identity, inversion bodies, streaming kernels, field arithmetic and existing forecast bodies are unchanged. Binary prefix/division work remains four vector multiplications per full ordinary pair; two x arrays are packed forward and four x/y arrays reverse, versus four arrays forward originally. Extra packing and stack/register effects remain costs. No source byte saving implies a latency gain.\n\nThe candidate has 177 source members: 174 originals exact, two reversible parent edits and one cfg-only caller/test child. Reversing the edits recovers all 176 accepted originals. The default trait selector/reservation preserve G2 behavior; only experimental native G1 enables the new policy. The portable x86-64/no-IFMA selector is false and keeps original records. No key table, point/key copy, plan, new production pool, new field primitive, unsafe block or arbitrary RAM cap is added. Existing native and portable field/support libraries are reused without rebuild.\n\n## Validation\n\nAll ten Rust/xtask workflow commands complete on their first attempt. Native executes **129 tests: 125 original and four new**, all passing. Portable executes four new focused tests, with 101 original tests filtered and 24 native-only originals absent from that executable. Each mode also passes two new independent Arkworks tests: 36 complete indexed/grouped fixtures including full 254-bit scalars, boundary values, mixed groups and fresh/reused buffers; plus 22 complete weighted cases including multiple and partial chunks.\n\nEach mode checks 780 single-layer ordinary/partial/tail/pattern combinations and 120 weighted copied-even-identity cases, both inversion settings, and repeated increasing/decreasing scratch sizes. Ordinary, identity, doubling, opposite and mixed inputs match complete scalar/projective references. Separate std workers and Rayon pools each cover 1/4/16/32 workers with 512 KiB stacks. Grouped/indexed/weighted public paths are exercised inside the fixed-stack pools. Existing native forecast bounds remain conservative for the tested requested heap capacities. These are correctness/resource observations on this host, not CPU scaling or concurrent-proof throughput results.\n\nThe no-IFMA build uses x86-64 and matching frozen portable dependencies. It validates that code path on this host; it is not an ARM, mobile or TEE execution. Field tests are inherited by exact source/library identity and were not rerun.\n\n## Capacity and emitted code\n\nRepresentative repeated-scratch observations are below. The two modes have different lane counts and representations, so these are **not an old/new RAM comparison**. The entries include fixed scratch but exclude live input/output arrays, allocator overhead and the rest of the process. They are not peak RSS, minimum RAM or whole-thread stack bounds.\n\n| Mode | Legacy record B | Prefix record B | Pair index B | Fixed scratch B |\n|---|---:|---:|---:|---:|\n");
    for mode in ["native", "portable"] {
        let v = review["std_worker_capacity_bytes"][mode]
            .as_array()
            .unwrap();
        report.push_str(&format!("|{mode}|{}|{}|{}|{}|\n", v[0], v[1], v[2], v[3]));
    }
    report.push_str("\nEnabled native G1 asserts zero legacy Group capacity and prefix capacity no larger than the largest full-group count seen. On this target a prefix/norm record is two 320-byte vectors instead of five, so the exact record bound is 640 versus 1,600 bytes per eight pairs. Retained common buffers and total process resources still need qualification. Existing admission estimates are unchanged.\n\nActual core commands use O3/oneCGU and explicit native or x86-64 targets. Final test/oracle consumers use fatLTO. Inspection of the native oracle consumer finds six selected functions with 1,200 static IFMA sites. Its G1 reduce_layer has 6,389 static instructions and a prologue with 3,840 bytes RSP subtraction plus 48 pushed-register bytes. These are emitted-code observations, not executed instruction counts, dynamic spill cost, total stack usage or timing results. No timing driver was built in this stage.\n\n## Review correction and next stage\n\nThe main independent review passed once, but a separate derived field mistakenly hardcoded 125 portable filtered tests while its own actual summary contained 101. A narrow repair checked the successful stdout hash, parsed the actual count and corrected only that metadata. Original reviewer/source/JSON/logs are retained; the successful main review, tests and builds were not repeated. The published reviewer source derives the count for reproducibility. See the [correction record](artifacts/review-metadata-repair.json).\n\nNext prepare matching frozen/fresh-OFF/new complete callers and a prospective performance protocol. Preserve all original unweighted cases and 1/16/32 contexts, and include weighted performance before SDK selection because the shared G1 tree affects weighted callers. Weighted point generation belongs outside the measured region. The existing weighted_words helper is a correctness boundary, not a ready timing caller. If an adapter extension is needed, keep qualified source/libraries immutable and qualify only the new wrapper/dependent artifacts.\n\nProfile counts should use the previous control evidence prospectively to target thousands of samples. The prior study's millions of large-group samples are already complete and must not be repeated or pushed into Git. All new latency must finish before PMU/profiles. There is no positive-gain floor, threshold/layout sweep, favorable-subset promotion, extension or unchanged reroll. A qualified complete-caller gain can select only a separate verified all-eight+54/physical16/portable/concurrent/resource/Gnark SDK trial. Passing this qualification does not permit adoption.\n\nNo latency/PMU/profile capture, new SDK/Gnark proof export, AWS/Aeglos comparison, huge-circuit, device/pressure/swap, minimum-RAM or concurrent-proof throughput result is added.\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let section = &old[old
        .find("## Accepted CPU and Metal comparisons, unchanged")
        .unwrap()..];
    report.push_str(&section[..section.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n[Review](artifacts/review.json), [codegen](artifacts/codegen.json), [design](artifacts/design.md), [source audit](artifacts/source-audit.json), [libraries](artifacts/libraries.json), [native/portable tests and caller](artifacts/probe.rs), [Arkworks oracle](artifacts/oracle.rs), [plan](artifacts/plan.json), [pins](artifacts/pins.json), [metadata repair](artifacts/review-metadata-repair.json), [next stage](artifacts/next.md), [three exact source deltas](artifacts/source-deltas.json.gz), [exact helpers/receipts and retained original review](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Unchanged source members are bound by their accepted hashes; full snapshots/binaries stay local. No private inputs, witness/key/image/proof bytes, executable/library blobs or external paper text are included.\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "review.json",
        "review.rs",
        "codegen.json",
        "codegen.rs",
        "design.md",
        "source-audit.json",
        "libraries.json",
        "probe.rs",
        "oracle.rs",
        "prepare.rs",
        "plan.json",
        "pins.json",
        "repair-review.rs",
        "review-metadata-repair.json",
        "review-metadata-repair.md",
        "next.md",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut deltas = BTreeMap::new();
    for p in [
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm/g1_tree_records_probe.rs",
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
    for dir in ["runs", "review-before-filter-count"] {
        for p in walk(&Path::new(R).join(dir)) {
            helpers.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &helpers);
    let publication = json!({"review_passed":true,"codegen_passed":true,"derived_metadata_repair_passed":true,"source_delta_UTF8_members":deltas.len(),"helper_receipt_UTF8_members":helpers.len(),"all_embedded_hashes_and_archive_roundtrips_verified":true,"workflow_commands_first_success":10,"native_tests":129,"portable_focused_tests":4,"portable_original_tests_filtered":101,"native_only_originals_absent_from_portable_executable":24,"Arkworks_tests_each_mode":2,"performance_measured":false,"new_proof_exports":0,"adopted":false,"goal_status":"active/unmet0/8"});
    fs::write(
        art.join("publication.json"),
        serde_json::to_vec_pretty(&publication).unwrap(),
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
    println!("Published{}files/3exactUTF8deltas/{}helper-receiptmembers; hash+roundtripPASS; native+portablequalification/no timing/adoption",walk(Path::new(D)).len(),helpers.len());
}
