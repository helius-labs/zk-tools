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
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-qualification";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-qualification";
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
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            if !["target-portable", "frozen-portable"]
                .contains(&e.file_name().to_string_lossy().as_ref())
            {
                walk(&e.path(), out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn report(a: &Value) -> String {
    let mut s=String::from("# Borrowed G2 compatibility and resource qualification\n\nThe exact candidate passes the added qualification:11 portable field tests,99 core tests,8 prover tests,248 complete SDK requests and89 independent Gnark exports. Eight cold/warmed memory cases finish under their declared limits without OOM/max events. Production remains unchanged pending an explicit adoption assessment. The original SDK performance screen remains **failed** at two physical16 bounds; its clear primary32 improvement is retained.\n\n");
    s.push_str("The [full SDK experiment](../2026-10-28-g2-borrowed-sdk/README.md) measured primary32 at8.226135ms versus accepted8.379021ms, paired2.13% faster with bounds1.53–2.61% and15/15wins. It removes40requests/1472000requestedbytes per primary proof, without a stable whole-process RSS reduction. Physical16 point estimates improve, but OFF/frozen calibration and ON/frozen nonregression missed required bounds. This qualification does not repeat latency, select another executable, exclude blocks or turn those failed checks into passes.\n\n## Coverage\n\n- The174-file private candidate snapshot matches the measured source. Portable x86-64/no-ASM/no-IFMA builds keep the private borrowed flag enabled while its native guards exclude the new path. All unfiltered suites pass, and native119/122 tests are inherited rather than rerun.\n\n- 13 portable real-proof cases cover all8 reference shapes plus54 with basis images, and primary/54 with raw and denied optional-preparation paths.\n\n- 24 native cases cover1/4workers,512KiB/1MiB worker stacks, primary/54, and basis/raw/denied paths.8 additional cases disable packed/direct/canonical-batch/vector-inverse on accepted and candidate SDKs.\n\n- 4 concurrent cases cover accepted/candidate × primary/54,4callers ×2requests. They check37 loaded TIDs and33 idle TIDs: one32worker global pool plus callers, without a replacement pool. All32 fresh exports pass independent verification.\n\n- 8 pressure cases cover cold/warm × accepted/candidate × default/retained allocator settings. The controller creates and charges a private image copy inside each owned transient service. Cold limits are192MiB; warmed cases prime at512MiB, add32MiB ballast, then lower high/maximum limits to128/192MiB before two fresh proofs. Swap is disabled for these cases. These are inherited validation conditions, not an architectural cap, minimum RAM or proof of swap behavior.\n\nRaw proving images are checked against historical hashes. The unchanged independent Gnark verifier obtains the matching verifying key from the corresponding historical Euler image; it is the same circuit/key in a different query representation. Both image identities and expected public hashes are retained. This checks raw-loader proof correctness without changing the verifier parser.\n\n## Resource observations\n\n| Case | Passed | Peak MiB | High events | Max events | OOM/kill | Swap limit |\n|---|---|---:|---:|---:|---|---|\n");
    for c in a["resources"].as_array().unwrap() {
        let peak = c["metrics"]["memory.peak"]
            .as_str()
            .unwrap()
            .parse::<u64>()
            .unwrap();
        writeln!(
            s,
            "| {} | {} | {:.2} | {} | {} | {}/{} | {} |",
            c["case"]["name"].as_str().unwrap(),
            c["passed"],
            peak as f64 / 1048576.,
            c["memory_events"]["high"],
            c["memory_events"]["max"],
            c["memory_events"]["oom"],
            c["memory_events"]["oom_kill"],
            c["metrics"]["memory.swap.max"].as_str().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nHigh events distinguish observed throttling from merely running under configured limits; complete current/peak/pressure/event/stat snapshots are retained. No larger-limit retry is permitted on failure. The service controller stops and resets only its uniquely prefixed units and removes their owned control directories. No global memory/kernel setting or unrelated process changes. Actual resource peaks include the private charged image, controller, SDK and any ballast; they are not a minimum-RAM guarantee.\n\n## Verification and adoption assessment\n\nAll248 SDK requests verify internally. All89 fresh164byte exports independently pass Gnark, expected public/key binding, member hashing and uniqueness against14462 prior identities. The independent ledger is14551;178 proof/public members are not178 proofs. There are45 functional processes,4 concurrent processes and8 resource cases. Builds finish before proof capture, and no owned build/hash/helper/archive/second benchmark overlaps capture. Unrelated host work is not excluded.\n\nThe candidate is eligible for an explicit engineering adoption assessment using the clear primary32 improvement and this added coverage, while accepting the documented physical16 uncertainty. The old performance gates remain false. Source promotion must still be concrete and reviewed; this report alone does not change production. No new mobile/TEE, AWS/Aeglos, swap,250/4999 or minimum-RAM result is claimed. Giant images from earlier qualification are absent. The original all8 CPU10x objective remains active/unmet0/8, with primary requiring3.7ms.\n\n## Inherited CPU and Metal comparisons\n\nThe following are the preceding full SDK measurements and supplied historical M5 references, not timings from these functional/resource checks.\n\n| Shape | Accepted ms | Candidate ms | M5 CPU ms | M5 Metal ms | CPU/candidate | Metal/candidate |\n|---|---:|---:|---:|---:|---:|---:|\n");
    let p = read("/home/ubuntu/src/zopro-perf-work/g2-borrowed-sdk/analysis.json");
    for (shape, v) in p["m5"].as_object().unwrap() {
        writeln!(
            s,
            "| {shape} | {:.6} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            v["variants"]["accepted"]["ms"].as_f64().unwrap(),
            v["variants"]["on"]["ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["variants"]["on"]["cpu_speedup"].as_f64().unwrap(),
            v["variants"]["on"]["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [build/test receipts](artifacts/build/result.json), [functional receipts](artifacts/functional/result.json), [resource receipts](artifacts/resource-result.json), [proof inventory](artifacts/proof-inventory.json). Executable/rlib payloads remain local with hashes and exact library receipts.\n");
    if let Ok(x) = fs::read_to_string(r("interpretation.md")) {
        s.push('\n');
        s.push_str(&x)
    }
    s
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(r("analysis.json"));
    assert_eq!(a["independent_proof_count"], 14551);
    for (p, h) in read(r("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    let out = Path::new(D);
    fs::create_dir_all(out.join("artifacts")).unwrap();
    let text = report(&a);
    fs::write(r("report.md"), &text).unwrap();
    let mut files = vec![];
    walk(Path::new(R), &mut files);
    let mut local = BTreeMap::new();
    for p in files {
        let rel = p.strip_prefix(R).unwrap();
        let name = p.file_name().unwrap().to_string_lossy();
        if name.starts_with("git-")
            || ["close.log", "archive-verification.log"].contains(&name.as_ref())
        {
            continue;
        }
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") || bytes.starts_with(b"!<arch>") {
            local.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let q = out.join("artifacts").join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::write(q, bytes).unwrap();
    }
    save(out.join("artifacts/local-binary-pins.json"), &json!(local));
    fs::write(out.join("README.md"), text).unwrap();
    let mut files = vec![];
    walk(out, &mut files);
    files.sort();
    let mut index = BTreeMap::new();
    let mut sums = String::new();
    for p in files {
        let key = p.strip_prefix(out).unwrap().display().to_string();
        let h = hash(p);
        sums.push_str(&format!("{h}  {key}\n"));
        index.insert(key, h);
    }
    save(out.join("artifact-index.json"), &json!(index));
    fs::write(out.join("SHA256SUMS"), sums).unwrap();
    println!("Archived{}hashes", index.len());
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
        json!({"commit":s["commit"],"root":s["root"],"report":s["report"]});
    s["commit"] = json!(head);
    s["root"] = json!(R);
    s["report"] = json!(format!("{D}/README.md"));
    s["status"]=json!("Borrowed G2 added qualification passed; prior performancegate false; adoption assessment pending");
    s["g2_borrowed_qualification"] = a;
    s["independent_proof_count"] = json!(14551);
    s["new_independent_exports"] = json!(89);
    s["SDK_requests"] = json!(248);
    s["owned_live_jobs"] = json!([]);
    s["last_goal_turn_classification"] = json!("progress");
    s["pushed"] = json!(true);
    s["remote_verified"] = json!(true);
    s["next_action"]=json!("Review/promote clean borrowed G2 production source as an explicit engineering exception if justified; preserve physical16 failed bounds, exact qualified behavior and full8 target. Inspect actual epoch-pin offsets before selecting next CPU-work change.");
    save(p, &s);
    let hand = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hand).unwrap();
    fs::write(hand,format!("CURRENT COMPLETE/PUSHED — BORROWED G2 ADDED QUALIFICATION\n\nHEAD/remoteverified{head};root{R};report{D}/README.md. Additionalqualpassed11field/99core/8proverportable;45functional+4concurrent+8resourcecases;248SDK/89freshGnark/ledger14551. Native119/122inherited/no newnative tests;exactcandidate notlatencyrerun. Primaryclear2.13%gainretained;priorperformancegateFALSE2physical16bounds;productionunchanged/notadopted.8cold/warm privatechargedcgroupcases192MiB/noSwap allpass, actualhigh events inanalysis;nativeoptouts/smallstacks/portableall8+54/raw/denied/concurrentchecked. No new250/4999/mobile/AWS/minRAM/swapclaim;goalACTIVE0/8. NextcleanproductionadoptionassessmentexplicitengineeringexceptionifjustifiedandepochhotIPinspection;neverhidefailedbounds/rerollunchangedlatency. Noownedjobs/cloud.\n\n{old}")).unwrap();
    println!("State/handoffupdated {head}");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("state") => state(),
        _ => panic!("archive/state"),
    }
}
