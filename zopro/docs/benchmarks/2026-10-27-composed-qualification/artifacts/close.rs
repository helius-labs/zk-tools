use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-composed-qualification";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-composed-qualification";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
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
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let ty = e.file_type().unwrap();
        if ty.is_dir() {
            if !e.file_name().to_str().unwrap().starts_with("target") {
                archive(root, &p, out, index)
            }
        } else if ty.is_file()
            && ["rs", "md", "json", "jsonl", "log", "txt", "proof", "public"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let rel = p.strip_prefix(root).unwrap();
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            assert_eq!(hash(&p), hash(&dest));
            index.insert(rel.display().to_string(), hash(&dest));
        }
    }
}
fn summary(path: impl AsRef<Path>) -> Value {
    serde_json::from_str(
        fs::read_to_string(path)
            .unwrap()
            .lines()
            .find_map(|s| s.strip_prefix("SUMMARY "))
            .unwrap(),
    )
    .unwrap()
}
fn events(path: impl AsRef<Path>) -> Vec<Value> {
    fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|s| s.strip_prefix("EVENT "))
        .map(|s| serde_json::from_str(s).unwrap())
        .collect()
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["qualification_passed"], true);
    assert_eq!(a["next_prior_count"], 9154);
    assert_eq!(a["independent_exports"], 85);
    assert_eq!(
        read(r.join("qualification/result.json"))["status"],
        "complete"
    );
    let inventory = read(r.join("proof-inventory.json"));
    assert_eq!(inventory.as_object().unwrap().len(), 170);
    for (p, h) in inventory.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let files = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 170);
    for (p, h) in files {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    let pins = read(r.join("qualification-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        if !p.ends_with(".image") {
            assert_eq!(hash(p), h.as_str().unwrap());
        }
    }
    // Workflow PinGuard and the frozen finish helper already streamed every
    // image before/after execution. Do not add another giant-key hash pass.
    let resources = read(r.join("resource-result.json"));
    let jobs = read(r.join("jobs.json"));
    let inputs = read(r.join("inputs.json"));
    let mut md=String::from("# Combined CPU candidate memory and compatibility qualification\n\nThe combined borrowed-G1/adaptive-G2/Poseidon candidate passes the declared portable, fallback, small-stack, concurrent-call and constrained-memory cases. Both 4,999-batch proofs independently verify under a **95 GiB cgroup limit with swap disabled**: **166.524 s cold / 164.686 s warm**, with **94.684 GiB peak process RSS**. This establishes feasibility for the tested cases and capacities, not minimum RAM or a new speed comparison.\n\nThe [complete latency study](../2026-10-27-composed-cpu-sdk/README.md) retains its primary32 **3.13%** and original-eight aggregate **1.98%** paired gains. Its original full performance gate remains false solely for physical16 control calibration. This resource study does not change that verdict or adopt source. Supplied CPU10× targets remain **0/8**.\n\n## Sources and validation\n\nNative qualification drivers link the exact measured accepted and combined libraries; native arithmetic is not rebuilt. The standard combined SDK retains SHA-256 `8241648f298893953bcacac515d3d52a7f2d363cb264e08d232af3037decb265`. A separate fresh portable target builds the identical combined source with x86-64/noIFMA/noASM flags and passes **11 field, 99 core and eight prover tests** (raw-image two/cubic six), including adaptive retained-capacity/tree boundaries and all-width Poseidon comparisons. Native116/raw2/cubic6/generator qualification is inherited from the exact tested source/library receipts. No new native test execution is claimed here.\n\nRust xtask runs all cases serially from a protocol frozen before requests. New Rust cgroup/cache controllers replace the earlier qualification's Python runners. Two controller checks cover memory-event parsing and nonfaulting mincore; a separate non-SDK systemd true smoke checks the memory controller. One build-plan-helper digest-format compilation failure preceded all native links and portable builds; the original helper/type note and repair are retained. No prover build/test/request failure or successful-process repeat occurred. All 170 production source files remain unchanged.\n\n## Functional and concurrent cases\n\nAll **37 functional processes** pass: nine portable standard shapes; portable raw/preparation-denied primary and54; native1/4 workers × requested512KiB/1MiB worker stacks × primary/54 × basis/raw/preparation-denied. Each performs three primes plus one fresh verified SDK request and independently verifies its final export. Worker stacks are requested through RUST_MIN_STACK; no stack watermark is measured. Raw images and preparation limit zero exercise the declared fallback configurations.\n\nPortable standard-image diagnostics below are one request after three primes at four workers. They are correctness/resource observations without latency confidence intervals or acceptance claims. RSS includes loading and primes.\n\n|Shape|Observed request ms|Process peak RSS MiB|Independent proof|\n|---|---:|---:|---|\n");
    for c in inputs["functional_cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["backend"] == "portable" && c["kind"] == "basis")
    {
        let name = c["name"].as_str().unwrap();
        let s = summary(r.join(format!("qualification/prove-{name}/stdout.log")));
        writeln!(
            md,
            "|{}|{:.3}|{:.2}|pass|",
            c["case"]["shape"].as_str().unwrap(),
            s["loop_wall_ms"].as_f64().unwrap(),
            s["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.
        )
        .unwrap();
    }
    md.push_str("\nFour separate concurrent processes compare exact accepted/combined libraries for primary and54. Four callers share one immutable mapped image and the existing32-worker pool, two requests each; all32 exports independently verify. Assertions observe32 workers+four callers+main while active and32+main afterward, without a hidden pool. These are concurrency correctness checks, not throughput or scheduling-fairness qualification.\n\n|Variant|Shape|Callers / proofs|Process peak RSS MiB|\n|---|---|---:|---:|\n");
    for j in jobs
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["kind"] == "concurrent")
    {
        let name = j["name"].as_str().unwrap();
        let stderr =
            fs::read_to_string(r.join(format!("qualification/prove-{name}/stderr.log"))).unwrap();
        let peak: f64 = stderr
            .lines()
            .find_map(|s| s.trim().strip_prefix("Maximum resident set size (kbytes):"))
            .unwrap()
            .trim()
            .parse()
            .unwrap();
        writeln!(
            md,
            "|{}|{}|4 / 8|{:.2}|",
            if name.contains("-accepted-") {
                "accepted"
            } else {
                "combined"
            },
            if name.ends_with("-1x2") {
                "1x2"
            } else {
                "54→1"
            },
            peak / 1024.
        )
        .unwrap();
    }
    md.push_str("\n## Primary memory limits\n\nCold cases start entirely under192MiB with swap disabled. The controller copies the owned primary image using a bounded buffer inside the cgroup so its new file pages are charged there. Both allocator policies perform three primes plus one request. Warm cases start at512MiB, prime three times, retain/touch32MiB ballast in the same cgroup, then lower MemoryHigh to128MiB and MemoryMax to192MiB before two further independently verified requests. Every case exits normally with exact declared limits and no max/oom/oom_kill events.\n\n|Case|Allocator|Cgroup lifetime peak MiB|High / max events|Independent exports|\n|---|---|---:|---:|---:|\n");
    for row in resources["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["case"]["kind"] != "large")
    {
        let c = &row["case"];
        let peak: f64 = row["metrics"]["memory.peak"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        writeln!(
            md,
            "|{} / {}|{}|{:.2}|{} / {}|{}|",
            c["kind"].as_str().unwrap(),
            c["variant"].as_str().unwrap(),
            c["allocator"].as_str().unwrap(),
            peak / 1048576.,
            row["memory_events"]["high"],
            row["memory_events"]["max"],
            row["expected_exports"]
        )
        .unwrap();
    }
    md.push_str("\nThese small cases remain below the128MiB soft limit: **they do not trigger high-limit reclamation**. They establish the requested limit transitions and successful bounded launches, not active reclaim stress at that size. The large4999 case below supplies actual hard-limit reclaim evidence. Peaks include controller, copied image pages, SDK and warm ballast; they are not comparable to standalone unconstrained process RSS. Resource scheduling also sees the cgroup limit. Do not infer a universal memory saving or minimum RAM from these values.\n\n## Forester250 and4999\n\nOriginal pinned solver images are6,451,333,588 and124,707,841,608 bytes. Before each launch, a bounded streaming hash checks the image, then fsync/POSIX_FADV_DONTNEED acts only on that owned image. A PROT_NONE mincore mapping confirms zero resident pages without faulting them in. No global cache flush or key/image conversion occurs. One cold and one warm core-prover call use different original witnesses; both public bindings and proofs independently verify.\n\n|Batch|Load s|Cold proof s|Warm proof s|Process peak RSS GiB|Cgroup peak GiB|High / max events|OOM / kill|\n|---|---:|---:|---:|---:|---:|---:|---:|\n");
    for row in resources["rows"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["case"]["kind"] == "large")
    {
        let name = row["case"]["name"].as_str().unwrap();
        let ev = events(r.join(format!("runs/{name}/sdk.log")));
        let load = ev.iter().find(|v| v["stage"] == "loaded").unwrap()["seconds"]
            .as_f64()
            .unwrap();
        let proofs: Vec<_> = ev.iter().filter(|v| v["stage"] == "proof").collect();
        let rss = ev
            .iter()
            .map(|v| v["process_peak_rss_kib"].as_f64().unwrap())
            .max_by(f64::total_cmp)
            .unwrap();
        let peak: f64 = row["metrics"]["memory.peak"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        writeln!(
            md,
            "|{}|{:.3}|{:.6}|{:.6}|{:.3}|{:.3}|{} / {}|{} / {}|",
            row["case"]["batch"],
            load,
            proofs[0]["verified_seconds"].as_f64().unwrap(),
            proofs[1]["verified_seconds"].as_f64().unwrap(),
            rss / 1048576.,
            peak / 1073741824.,
            row["memory_events"]["high"],
            row["memory_events"]["max"],
            row["memory_events"]["oom"],
            row["memory_events"]["oom_kill"]
        )
        .unwrap();
    }
    md.push_str("\nBoth use95GiB hard limits and memory.swap.max=0; recorded swap peak is zero. For4999 the cgroup reaches its cap and records128,010 max events and128 high events, while both proofs complete without OOM or kill. Those event counters are not page counts or exclusive wall-time costs. Process RSS and cgroup accounting cover different memory. These are single cold/warm observations, not matched accepted/Aeglos/Gnark comparisons; they do not establish a large-circuit speed improvement. A lower feasible capacity was not searched.\n\n## Counts, integrity and remaining work\n\nAll **216 SDK calls plus four forester core calls** complete. All **85 distinct fresh exports** pass independent Gnark verification,164-byte proof length, exact public binding and freshness against the checked9,069 prior identities: ledger **9,069→9,154**,170 proof/public members. No intermediate inventory is counted twice. All640 input pins are checked by the workflow and frozen finish helper, including bounded streaming hashes of the large images before/after execution. Closing checks rehash source/driver/binary inputs and all proof/public members; no extra giant-image hash pass is needed. All owned cgroup services are stopped and their runtime property overrides removed. No cloud machine is created.\n\nThe retained-capacity/forecast argument is in the [combined implementation report](../2026-10-27-composed-cpu-sdk/README.md#implementation-and-memory). These runtime cases complement that argument; they do not prove transient allocation bounds for every input, physical phone/ARM, TEE,193GiB key behavior, giant concurrent proofs or minimum RAM. There is no new lookup table, key copy or persistent solver plan in the implementation. Owned test image copies belong only to the cgroup harness.\n\nThe candidate now has positive complete-proof evidence and the declared resource/compatibility checks. Review adoption with the physical16 control-calibration limitation explicitly recorded; do not discard its gains, silently rewrite the failed frozen gate, or repeat unchanged acceptance timing. The all-shape historical M5 CPU/Metal comparisons remain in the [complete latency report](../2026-10-27-composed-cpu-sdk/README.md#supplied-m5-references); no new Apple/AWS/Aeglos measurements are claimed. The all-eight CPU10× goal remains active and unmet.\n\n[Design](artifacts/design.md), [qualification analysis](artifacts/analysis.json), [per-case jobs](artifacts/jobs.json), [resource results](artifacts/resource-result.json), [portable tests](artifacts/portable/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).\n");
    save(
        r.join("closeout.json"),
        &json!({"status":"complete combined resource/compatibility qualification","qualification_passed":true,"SDK_requests":216,"forester_core_requests":4,"independent_gnark_exports":85,"proof_public_members":170,"next_prior_count":9154,"input_pins":pins.as_object().unwrap().len(),"production_sources_checked_unchanged":170,"native_arithmetic_rebuilt":false,"portable_field_tests":11,"portable_core_tests":99,"portable_prover_tests":8,"adopted":false,"earlier_performance_full_gate_passed":false,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    fs::create_dir_all(d).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!("Closed resource qualification: {} archive hashes;216SDK+4core,85Gnarkexports;proofledger9154;noadoption",index.len());
}
