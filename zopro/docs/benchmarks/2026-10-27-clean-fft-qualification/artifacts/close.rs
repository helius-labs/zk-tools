use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path};
const R: &str = "/tmp/zopro-clean-fft-qualification";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-qualification";
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
    assert_eq!(a["next_prior_count"], 11660);
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
    let manifest = read("/tmp/zopro-composed-adoption/production-source-manifest.json");
    let files = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(files.len(), 171);
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
    let mut md=String::from("# Clean FFT compatibility and memory qualification\n\nThe exact clean FFT candidate passes the declared portable, fallback, small-stack, concurrent-call and constrained-memory cases. The large table below gives the actual Forester250/4999 load, cold/warm proof and memory observations under a95GiB hard limit with swap disabled. This establishes feasibility in these scenarios, not minimum RAM or a matched speed comparison. No source is adopted by this resource study.\n\nThe [complete latency study](../2026-10-27-clean-fft-leaves/README.md) retains its original failed full gate: primary strict-gain uncertainty and an8x2 candidate/fresh nonregression upper bound above1.02. All calibration and aggregate checks pass, with original-eight paired gains0.856%/0.419% against accepted/fresh. This qualification does not change that verdict. Any later adoption requires a separate explicit engineering review. Original all-eight supplied M5 CPU10x goal remains active0/8.\n\n## Sources and validation\n\nNative resource drivers link the exact measured clean14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a and current accepted824164 libraries; no native arithmetic is rebuilt. The legacy resource label composed denotes this clean candidate and its exact receipts. A separate fresh portable x86-64/noIFMA/noASM target uses the identical source and passes field/core/prover suites recorded in analysis.json. Native116fresh/119candidate core plus raw2/cubic6 qualification is inherited from the exact latency-study libraries, not a new execution here.\n\nRust xtask runs frozen cases serially. Two controller checks cover memory-event parsing and a nonfaulting mincore residency probe. Every request, verifier, source/library/input pin and lifecycle receipt remains in the archive. No unrelated owned workload overlaps qualification requests. All171 accepted production files remain unchanged. Kernel and scenario checks do not establish physical-phone/ARM/TEE or universal memory guarantees.\n\n## Functional and concurrent cases\n\nAll37 functional processes pass: nine portable standard shapes; portable raw/preparation-denied primary/54; native1/4workers x512KiB/1MiB requested worker stacks xprimary/54 xbasis/raw/preparation-denied. Every process makes three primes and one fresh internally verified request, then independently verifies its final export. RUST_MIN_STACK requests a stack size; no watermark is measured. Raw/denied cases exercise the existing fallback.\n\nPortable standard-image diagnostics below are one request after three primes at four workers. They are correctness/resource observations without acceptance confidence intervals; RSS includes loading and primes.\n\n|Shape|Observed request ms|Process peak RSS MiB|Independent proof|\n|---|---:|---:|---|\n");
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
    md.push_str("\nFour separate concurrent processes compare exact accepted/clean libraries for primary and54. Four callers share one immutable mapped image and the existing32-worker pool, two requests each; all32 exports independently verify. Assertions observe32 workers+four callers+main while active and32+main afterward, without a hidden pool. These are concurrency correctness checks, not throughput or scheduling-fairness qualification.\n\n|Variant|Shape|Callers / proofs|Process peak RSS MiB|\n|---|---|---:|---:|\n");
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
                "clean"
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
    let high_total:u64=resources["rows"].as_array().unwrap().iter()
        .filter(|r|r["case"]["kind"]!="large").map(|r|r["memory_events"]["high"].as_u64().unwrap()).sum();
    writeln!(md,"\nSmall-case high-limit events total{high_total}; zero means the soft threshold did not trigger reclaim. The actual table and events retain the distinction between successful limit transitions and active pressure. Cgroup peaks include controller, copied image pages, SDK and warm ballast; they differ from standalone RSS and do not establish minimum RAM.\n\n## Forester250 and4999\n\nThe original pinned solver images are unchanged. Before each launch, a bounded streaming hash verifies the image, then fsync/POSIX_FADV_DONTNEED acts only on that owned image. A PROT_NONE mincore probe confirms zero residency without faulting pages. No global cache drop, key conversion, image copy or larger retry occurs. One cold and one warm core-prover call use different original witnesses; proofs and exact public bindings independently verify.\n\n|Batch|Load s|Cold proof s|Warm proof s|Process peak RSS GiB|Cgroup peak GiB|High / max events|OOM / kill|\n|---|---:|---:|---:|---:|---:|---:|---:|").unwrap();
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
    md.push_str("\nBoth large cases use95GiB hard limits and memory.swap.max=0. Actual memory peak, swap and reclaim/OOM events remain in the table and raw receipts. Max/high event counters are not page counts or removable wall time. These single cold/warm observations are not matched accepted/Aeglos/Gnark timing and do not establish minimum RAM or a large-circuit speed gain. No lower-cap search occurs.\n\n## Counts and scope\n\nAll216SDK calls plus four Forester core calls complete. All85distinct new exports independently verify, satisfy164-byte length/exact public binding/freshness against11575prior identities: ledger11575->11660,170proof/public members. Inventories/copies/reverification are not counted twice. The frozen workflow/finish helper streams image hashes before/after execution; closing rehashes remaining inputs and exports without an extra giant-image pass. Owned cgroup services and runtime overrides are removed; no cloud resource is created.\n\nThe [integration review](../2026-10-27-clean-fft-leaves/artifacts/integration-review.md) describes unchanged allocation geometry. These scenarios complement it without proving a universal allocator peak, physical phone/ARM/TEE behavior,193GiB key feasibility, giant concurrent proofs or minimumRAM. No new lookup table, key copy or duplicate execution plan is introduced. Small image copies belong only to the owned qualification harness.\n\nThe parent full performance gate remains false. A separate explicit engineering decision may weigh reliable aggregate/larger-case benefits against primary and8x2bound uncertainty after these resource results. This report does not adopt code, waive a gate, repeat acceptance timing or claim new AWS/M5/Aeglos performance. The all-eight CPU10x goal remains active0/8.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [jobs](artifacts/jobs.json), [resources](artifacts/resource-result.json), [portable tests](artifacts/portable/core/tests.json), [proof inventory](artifacts/proof-inventory.json), [artifact hashes](artifact-index.json).\n");
    save(
        r.join("closeout.json"),
        &json!({"status":"complete clean FFT resource/compatibility qualification","qualification_passed":true,"SDK_requests":216,"forester_core_requests":4,"independent_gnark_exports":85,"proof_public_members":170,"next_prior_count":11660,"input_pins":pins.as_object().unwrap().len(),"production_sources_checked_unchanged":171,"native_arithmetic_rebuilt":false,"portable_field_tests":a["portable_field_tests"],"portable_core_tests":a["portable_core_tests"],"portable_prover_tests":a["portable_prover_tests"],"adopted":false,"earlier_performance_full_gate_passed":false,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    fs::create_dir_all(d).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!("Closed resource qualification: {} archive hashes;216SDK+4core,85Gnarkexports;proofledger11660;noadoption",index.len());
}
