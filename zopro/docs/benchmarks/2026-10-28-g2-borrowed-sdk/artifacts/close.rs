use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-sdk";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn n(v: &Value, k: &str) -> f64 {
    v[k].as_f64().unwrap()
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let q = e.path();
        if e.file_type().unwrap().is_dir() {
            if e.file_name() != "profiles" {
                walk(&q, out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(q)
        }
    }
}
fn report(a: &Value) -> String {
    let mut s = String::from("# Borrowed G2 pending queue in SDK proofs\n\n");
    let cell = &a["cells"]["1x2-w32/on"];
    let p = &a["pairs"]["1x2-w32/on/accepted"];
    let off = &a["pairs"]["1x2-w32/on/off"];
    writeln!(s,"The candidate **{}**. Primary32 is **{:.6} ms**, ON/accepted paired median **{:.9}**, bounds[{:.9},{:.9}], {}/15wins. ON/OFF is **{:.9}**, bounds[{:.9},{:.9}], {}/15wins. All positive estimates and failed checks are retained. Production remains unchanged.",if a["gate_passed"]==true{"passes the declared SDK screen; broader qualification is required before adoption"}else{"fails the declared SDK screen and is not adopted"},n(cell,"median_ms"),n(p,"median"),n(p,"low"),n(p,"high"),p["wins"],n(off,"median"),n(off,"low"),n(off,"high"),off["wins"]).unwrap();
    s.push_str("\nThe exact private candidate from the [complete G2 MSM study](../2026-10-28-g2-borrowed-queue/README.md) is linked into the real SDK for the first time. Signed 16-byte references replace 136-byte copied G2 points in proof-local batch/collision queues. Signs are materialized at bucket insertion/projective fallback or Y packing; two unused scalar buffers are omitted. Field arithmetic, phi split, bucket representation, width/parts, immutable key, top windows, exception handling and unsupported-path fallback are unchanged. There is no persistent point/key table, recoded-key copy or expanded execution plan. The component's positive16-worker estimate, measured extra-live reduction and failed gate are retained within their original scopes.\n\n");
    s.push_str("## Complete verified request latency\n\nRyzen9950X,15 fixed rotated/reversed process blocks, accepted/frozen/freshOFF/ON at32 workers for all eight supplied shapes and merge54, plus primary16 physical workers. Each process primes3 times, then151 primary or31 other requests. Witness solving, fresh randomized proving, SDK verification and serialization are timed; loading is excluded. CPU includes coordination; RSS includes load/primes. These are warm single-request times, not parallel-proof throughput. Values are medians of block medians; every block is retained.\n\n| Cell | Variant | Median ms | CPU ms/request | Median peak MiB | Max peak MiB |\n|---|---|---:|---:|---:|---:|\n");
    for c in read(r("cases.json")).as_array().unwrap() {
        let case = c["name"].as_str().unwrap();
        for v in ["accepted", "frozen", "off", "on"] {
            let x = &a["cells"][format!("{case}/{v}")];
            writeln!(
                s,
                "| {case} | {v} | {:.6} | {:.3} | {:.2} | {:.2} |",
                n(x, "median_ms"),
                n(x, "cpu_ms"),
                n(x, "median_RSS_MiB"),
                n(x, "max_RSS_MiB")
            )
            .unwrap();
        }
    }
    s.push_str("\n## Paired checks\n\nBounds use ranks4..12 of15, nominal96.484375% under independent exchangeable blocks, without familywise guarantees. Frozen/accepted and OFF/frozen bounds must fit[1/1.03,1.03] in all10cells. Primary32 ON requires upper<1 and at least11wins against all three controls. Every other ON/control cell requires upper<=1.02. No minimum gain floor, exclusion, extension, fastest-file selection or successful-capture repeat is applied. Failed bounds do not by themselves establish an exclusive causal slowdown.\n\n| Cell/ratio | Paired median | Lower | Upper | Wins | Required check |\n|---|---:|---:|---:|---:|---|\n");
    for (k, v) in a["pairs"].as_object().unwrap() {
        let check = a["checks"]
            .get(k)
            .map(|x| if x == true { "pass" } else { "fail" })
            .unwrap_or("descriptive");
        writeln!(
            s,
            "| {k} | {:.9} | {:.9} | {:.9} | {}/15 | {check} |",
            n(v, "median"),
            n(v, "low"),
            n(v, "high"),
            v["wins"]
        )
        .unwrap();
    }
    writeln!(s,"\nFailed checks: `{}`. Passing this screen would not establish the original10x goal or qualify every device/resource regime.",a["failed_checks"]).unwrap();
    s.push_str("\n## Separate requested-memory observations\n\nThree rotated observations per cell/variant, one complete SDK request after3primes, using a separate instrumented executable. The exact parent allocation wrapper delegates to the same native allocator and snapshots counters before JSON formatting. Its accounting test is inherited, not newly run. Allocation requests/bytes count requested payloads, including realloc requests; extra live is peak requested payload above the pre-request live level. These are whole-process request observations, not per-worker capacities, actual resident bytes or minimum RAM. Instrumented times are excluded from latency tables. All three observations are retained.\n\n| Cell/variant | Allocation requests | Requested MiB | Extra live peak MiB | Live before MiB | Live after MiB |\n|---|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["resources"].as_object().unwrap() {
        let h = &v["median_heap"];
        writeln!(
            s,
            "| {k} | {:.0} | {:.3} | {:.3} | {:.3} | {:.3} |",
            n(h, "allocation_requests"),
            n(h, "requested_bytes") / 1048576.,
            n(h, "peak_extra") / 1048576.,
            n(h, "live_before") / 1048576.,
            n(h, "live_after") / 1048576.
        )
        .unwrap();
    }
    s.push_str("\n## Counters and execution profiles\n\nCounters follow all latency captures:3 blocks × primary16/32 and merge54/32 ×4variants ×2 grouped userland panels,31 requests after3primes. All four counters per panel must count with at least99.9% running. One shared FIFO pair gates the measured request loop. Values below are median per-request counts; cycles/instructions/L1I/iTLB come from the fetch panel, frontend/L1D from separate stall-panel runs. This does not create a combined cycle decomposition or removable wall-time estimate.\n\n| Cell/variant | Cycles M | Instructions M | L1I misses M | iTLB misses | Frontend stall M | L1D misses M |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (k, v) in a["counters"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("/fetch"))
    {
        let key = k.strip_suffix("/fetch").unwrap();
        let f = &v["median_events_per_request"];
        let t = &a["counters"][format!("{key}/stall")]["median_events_per_request"];
        writeln!(
            s,
            "| {key} | {:.3} | {:.3} | {:.6} | {:.1} | {:.3} | {:.3} |",
            n(f, "cycles") / 1e6,
            n(f, "instructions") / 1e6,
            n(f, "L1-icache-load-misses") / 1e6,
            n(f, "iTLB-load-misses"),
            n(t, "stalled-cycles-frontend") / 1e6,
            n(t, "L1-dcache-load-misses") / 1e6
        )
        .unwrap();
    }
    s.push_str("\nSix separate FIFO-gated199Hz cycles profiles cover primary16/32 accepted/OFF/ON,251 requests after3primes. Decode runs after all proof/resource captures. All profiles must have one sampled SDK PID, no lost records and sampled TID coverage at least the intended worker count. TID counts are observed SDK threads, not a census assigning every TID to a particular Rayon task. ON must show the selected borrowed-module path; controls must not. Symbol period shares include called/inlined work and are not removable proof latency.\n\n| Profile | Samples | Lost | Sampled TIDs | Borrowed symbol period |\n|---|---:|---:|---:|---:|\n");
    for (k, v) in a["profiles"].as_object().unwrap() {
        writeln!(
            s,
            "| {k} | {} | {} | {} | {:.3}% |",
            v["samples"],
            v["lost"],
            v["sampled_TID_count"],
            100. * n(v, "borrowed_period_fraction")
        )
        .unwrap();
    }
    s.push_str("\nRaw profiles and executable payloads remain local with hashes. All six complete loss dumps have verified deterministic-gzip round trips. Demangled maps, TID sets, counts, summaries and every counter observation are retained. Aggregate event changes do not establish a sole cause or equate frontend events to all CPU cost.\n\n");
    s.push_str("## Supplied M5 CPU and Metal references\n\nAll eight reference shapes are newly measured here. M5 numbers are supplied historical cross-machine references, not newly remeasured M5 performance. Accepted and candidate times are shown separately; the candidate is not adopted. Merge54 has no supplied reference.\n\n| Shape | Variant | Local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local |\n|---|---|---:|---:|---:|---:|---:|\n");
    for c in read(r("cases.json")).as_array().unwrap().iter().take(9) {
        if c["reference"].is_null() {
            continue;
        }
        let shape = c["shape"].as_str().unwrap();
        let refs = &a["m5"][shape];
        for v in ["accepted", "on"] {
            let x = &refs["variants"][v];
            writeln!(
                s,
                "| {shape} | {v} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
                n(x, "ms"),
                n(refs, "cpu_reference_ms"),
                n(refs, "metal_reference_ms"),
                n(x, "cpu_speedup"),
                n(x, "metal_speedup")
            )
            .unwrap();
        }
    }
    writeln!(s,"\nThe original eight-shape CPU10x objective remains **active and unmet**, {}/8 candidate point estimates at the target in this capture. Primary requires3.7ms. Component improvements and screen outcomes do not redefine completion. The prior accepted FFT engineering exception and its false full-performance/qualified-scope gates remain unchanged.",a["original8_CPU10x_rows"]).unwrap();
    s.push_str("\n## Provenance and validation\n\nAccepted production has172 original files and SDK SHA256 `14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a`.168 private original files remain byte-exact; reversing the four suffix/dispatch edits recovers all172. Two new modules implement the borrowed queue/prefix. Field source and arithmetic are unchanged. The original accepted executable is hardlinked; the three ordinary SDK links and four resource SDK links use exact coherent previously frozen native rlibs/proc macros. Actual native flags/features and transitive library hashes are checked; no native library or component test is rebuilt/repeated.\n\nParent native OFF119 / ON122 core tests are inherited, including signed borrowed-prefix tails/reuse, full-width phi MSMs and adversarial identity/equal/opposite/scalar-edge cases. Prior field qualification is inherited. Optional missing-fixture scenarios are not claimed. These are not newly executed test counts. Original parent diagnostic exports are unused by the SDK; ordinary latency executables have the unchanged standard SDK driver/global allocator and no heap observer.\n\nBefore capture, frozen/OFF complete .text/.rodata and every other runtime section byte are identical except individually decoded g2_phi Rust panic source-line fields+2 and build ID. Executable identities and names of equal length are fixed before timing; all outcomes are kept. The preceding same-byte SDK study did not show a large full-proof backing effect, but does not establish universal equivalence; measured control calibration remains required here.\n\nThe9 images are the preceding diagnostic's restored historical Euler images, each checked against expected image/public hashes and requests.36 preflights,600 latency,72 counter,6 profile and36 separate resource processes complete. All **39,060 complete SDK requests verify internally**; all **750 fresh164-byte exports independently pass Gnark**, expected public/image binding, member hashing and uniqueness against13,712 prior identities. The independent ledger is **14,462**. The1,500 proof/public members are not counted as1,500 proofs. All independent verification and decoding occur after capture.\n\nNo owned build/helper/hash/archive or second benchmark overlaps capture. Unrelated host work remains untouched; host snapshots are not an exclusive-use claim. Initial metadata preparation corrected the heap-test receipt path before SDK linking; original helper and failure are retained, with no successful test/link/capture repeat. ELF audit templates are retargeted to the actual native-links directory before execution. No global ASLR/cache/kernel change occurs. Broader portable/resource/device/250/4999 qualification remains necessary before adoption; this screen does not claim AWS/Aeglos, phone/TEE, pressure/swap, minimum RAM or giant-circuit performance. Resource forecasts retain their conservative existing capacity estimates.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native links](artifacts/native-links/result.json), [qualification inheritance](artifacts/qualification-inheritance.json), [control audit](artifacts/control-locations.json), [all750 captures/verifiers](artifacts/runs/result.json), [proof inventory](artifacts/proof-inventory.json).\n");
    if let Ok(x) = fs::read_to_string(r("interpretation.md")) {
        s.push('\n');
        s.push_str(&x)
    }
    s
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(r("analysis.json"));
    assert_eq!(a["independent_proof_count"], 14462);
    for (f, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap())
    }
    let text = report(&a);
    fs::write(r("report.md"), &text).unwrap();
    let dest = Path::new(D);
    fs::create_dir_all(dest.join("artifacts")).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    let mut compression = vec![];
    for p in files {
        let rel = p.strip_prefix(R).unwrap();
        let name = p.file_name().unwrap().to_string_lossy();
        if name.starts_with("git-")
            || ["close.log", "archive.log", "archive-verification.log"].contains(&name.as_ref())
        {
            continue;
        }
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let q = dest.join("artifacts").join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        if rel.to_string_lossy().contains("runs/loss-") && rel.ends_with("stdout.log") {
            let o = Command::new("gzip")
                .args(["-n", "-c"])
                .arg(&p)
                .output()
                .unwrap();
            assert!(o.status.success());
            let z = q.with_extension("log.gz");
            fs::write(&z, o.stdout).unwrap();
            let back = Command::new("gzip").args(["-dc"]).arg(&z).output().unwrap();
            assert!(back.status.success());
            assert_eq!(back.stdout, bytes);
            compression.push(json!({"path":z.strip_prefix(dest).unwrap(),"original_sha256":hash(p),"compressed_sha256":hash(z)}));
        } else {
            fs::write(q, bytes).unwrap();
        }
    }
    assert_eq!(compression.len(), 6);
    save(
        dest.join("artifacts/local-executable-pins.json"),
        &json!(local),
    );
    save(
        dest.join("artifacts/profile-compression.json"),
        &json!(compression),
    );
    fs::write(dest.join("README.md"), text).unwrap();
    let mut files = vec![];
    walk(dest, &mut files);
    files.sort();
    let mut index = BTreeMap::new();
    let mut sums = String::new();
    for p in files {
        let key = p.strip_prefix(dest).unwrap().display().to_string();
        let h = hash(&p);
        sums.push_str(&format!("{h}  {key}\n"));
        index.insert(key, h);
    }
    save(dest.join("artifact-index.json"), &json!(index));
    fs::write(dest.join("SHA256SUMS"), sums).unwrap();
    save(
        r("archive-summary.json"),
        &json!({"hashes":index.len(),"git_files":index.len()+2,"compressed_loss_dumps":6}),
    );
    println!(
        "Archived{}hashes/{}files;6lossroundtrips",
        index.len(),
        index.len() + 2
    );
}
fn state() {
    let o = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();
    assert!(o.status.success());
    let head = String::from_utf8(o.stdout).unwrap().trim().to_string();
    let o = Command::new("git")
        .args([
            "-c",
            "url.https://github.com/.insteadOf=git@github.com:",
            "-c",
            "credential.helper=!gh auth git-credential",
            "ls-remote",
            "origin",
            "refs/heads/sergei/zopro-cpu-perf",
        ])
        .output()
        .unwrap();
    assert!(o.status.success());
    assert!(String::from_utf8(o.stdout).unwrap().starts_with(&head));
    let a = read(r("analysis.json"));
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    s["previous_completed_experiment"] =
        json!({"commit":s["commit"],"root":s["root"],"report":s["report"],"status":s["status"]});
    s["commit"] = json!(head);
    s["root"] = json!(R);
    s["report"] = json!(format!("{D}/README.md"));
    s["status"] = a["status"].clone();
    s["g2_borrowed_sdk"] = a.clone();
    s["latest_full_sdk_measurement"] = json!({"root":R,"report":format!("{D}/README.md"),"status":a["status"],"gate_passed":a["gate_passed"],"production_changed":false,"adopted":false,"SDK_requests":39060,"new_independent_exports":750,"independent_proof_count":14462,"scope":a["scope"]});
    for k in [
        "SDK_requests",
        "new_independent_exports",
        "independent_proof_count",
        "original8_CPU10x_rows",
        "goal_status",
        "goal_achieved",
        "production_changed",
        "adopted",
    ] {
        s[k] = a[k].clone();
    }
    s["owned_live_jobs"] = json!([]);
    s["pushed"] = json!(true);
    s["remote_verified"] = json!(true);
    s["last_goal_turn_classification"] = json!("progress");
    s["next_action"] = json!(if a["gate_passed"] == true {
        "Full SDK screen passed: complete remaining portable/resource qualification before any adoption; retain all8 CPU10x objective"
    } else {
        "Use SDK profiles and failed-check/positive estimates to select a different CPU-work reduction; no unchanged borrowed-queue reroll or favorable executable selection"
    });
    save(p, &s);
    let hand = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hand).unwrap();
    fs::write(hand,format!("CURRENT COMPLETE/PUSHED — BORROWED G2 FULL SDK\n\nHEAD/remoteverified{head};root{R};report{D}/README.md;status{};gate{};productionunchanged/noadoption. First fullSDKintegration exactparentborrowedqueue. Frozen/OFFallcode/constants exact+decodedpanicline+2/buildid. Native119/122inherited/newtests0/no nativerebuild;7SDKlinks.750captures/750Gnark/18decode allcomplete,39060internallyverifiedrequests/750fresh164Bexports/ledger14462;all8+54at32/primary16;heap36/counters72/profiles6 separate. Original172/SDK14da immutable;goalACTIVEunmet. Read analysis/interpretation for findings/nextdecision. No ownedjobs/cloud. CommitANDpushcompletedexperiment;no unchangedreroll/noquestions/subagents/Casm/hugecache. Olderheaders historical.\n\n{old}",a["status"],a["gate_passed"])).unwrap();
    println!("State/handoffupdated {head}");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("state") => state(),
        _ => panic!("archive/state"),
    }
}
