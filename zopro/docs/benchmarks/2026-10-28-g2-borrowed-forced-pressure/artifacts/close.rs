use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-forced-pressure";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-borrowed-forced-pressure";
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
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        if e.file_type().unwrap().is_dir() {
            walk(&e.path(), out)
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path())
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let a = read(r("analysis.json"));
    let mut s=format!("# Borrowed G2 under forced high-limit pressure\n\n**{}**. {} complete SDK requests are known verified internally and{} fresh exports independently pass Gnark. The proof ledger is{}. Owned services are cleaned up. Production remains unchanged.\n\n",a["status"].as_str().unwrap(),a["known_verified_SDK_requests"],a["new_independent_exports"],a["independent_proof_count"]);
    s.push_str("The [preceding qualification](../2026-10-28-g2-borrowed-qualification/README.md) passed its bounded-memory cases, but every high-event counter was0. This new declared scenario holds112MiB of touched anonymous ballast, lowers memory.high to96MiB, retains the same192MiB hard limit and disables swap. Ballast alone exceeds the high limit. It tests actual kernel throttling on the exact accepted/candidate warm SDKs, with default/retained allocator settings. No old latency block or failed limit is retried.\n\nEach case primes3 times under512MiB, applies the declared limits, then makes2 fresh requests. The privately copied image is charged inside the owned service. Success requires positive high events, normal SDK completion, no OOM/kill/max events, expected key/public verification and cleanup. Intentional throttling gets a300second SDK-child deadline; these times are not pooled with latency measurements. Limits are validation conditions, not architecture caps or minimum RAM.\n\n| Case | Executed | Passed | High events | Peak MiB | Max events | OOM/kill |\n|---|---|---|---:|---:|---:|---|\n");
    for row in a["rows"].as_array().unwrap() {
        let res = &row["result"];
        let peak = res["metrics"]["memory.peak"]
            .as_str()
            .and_then(|s| s.parse::<f64>().ok())
            .map(|n| format!("{:.2}", n / 1048576.))
            .unwrap_or_else(|| "unobserved".into());
        writeln!(
            s,
            "| {} | {} | {} | {} | {peak} | {} | {}/{} |",
            row["name"].as_str().unwrap(),
            row["executed"],
            row["passed"],
            row["high_events"],
            res["memory_events"]["max"],
            res["memory_events"]["oom"],
            res["memory_events"]["oom_kill"]
        )
        .unwrap();
    }
    s.push_str("\nEvery available complete export is hashed,164bytes, unique against14551 prior identities, and independently checked against the historical primary image/public. If the controller stopped early, unexecuted cases stay unexecuted and any complete previously unverified export is checked afterward with a retained receipt; successful verification is not repeated. Known request counts do not invent unobserved partial work. No larger-limit restart or source/library/SDK change occurs. Service cleanup affects only the unique zopro-borrow-forced prefix; unrelated work and global kernel settings are unchanged. No owned build/hash/archive/second benchmark overlaps captures.\n\nThe primary32 performance gain remains the [preceding measured2.13% result](../2026-10-28-g2-borrowed-sdk/README.md), not a timing from this test. The two physical16 performance bounds remain failed. This pressure result informs an explicit adoption assessment and does not silently promote the old gate or satisfy the original10x objective, which stays active/unmet0/8. No new phone/TEE, AWS/Aeglos, swap, minimum-RAM or250/4999 result is claimed.\n\n## Inherited CPU and Metal comparisons\n\nAll8 rows below come from the completed full SDK study, with supplied historical M5 references; no latency rerun occurs here.\n\n| Shape | Accepted ms | Candidate ms | M5 CPU ms | M5 Metal ms | CPU/candidate | Metal/candidate |\n|---|---:|---:|---:|---:|---:|---:|\n");
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
    s.push_str("\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [case receipts](artifacts/runs), [cleanup](artifacts/cleanup-units.txt). Executables remain local with hashes.\n");
    if let Ok(x) = fs::read_to_string(r("interpretation.md")) {
        s.push('\n');
        s.push_str(&x)
    }
    let dst = Path::new(D);
    fs::create_dir_all(dst.join("artifacts")).unwrap();
    let mut paths = vec![];
    walk(Path::new(R), &mut paths);
    let mut binaries = BTreeMap::new();
    for p in paths {
        let name = p.file_name().unwrap().to_string_lossy();
        if name.starts_with("git-")
            || ["close.log", "archive-verification.log"].contains(&name.as_ref())
        {
            continue;
        }
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") {
            binaries.insert(p.display().to_string(), hash(&p));
            continue;
        }
        let dest = dst.join("artifacts").join(p.strip_prefix(R).unwrap());
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::write(dest, bytes).unwrap();
    }
    save(
        dst.join("artifacts/local-binary-pins.json"),
        &json!(binaries),
    );
    fs::write(dst.join("README.md"), s).unwrap();
    let mut files = vec![];
    walk(dst, &mut files);
    files.sort();
    let mut index = BTreeMap::new();
    let mut sums = String::new();
    for p in files {
        let name = p.strip_prefix(dst).unwrap().display().to_string();
        let h = hash(p);
        sums.push_str(&format!("{h}  {name}\n"));
        index.insert(name, h);
    }
    save(dst.join("artifact-index.json"), &json!(index));
    fs::write(dst.join("SHA256SUMS"), sums).unwrap();
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
    let path = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut v = read(path);
    v["previous_completed_experiment"] =
        json!({"commit":v["commit"],"root":v["root"],"report":v["report"]});
    v["commit"] = json!(head);
    v["root"] = json!(R);
    v["report"] = json!(format!("{D}/README.md"));
    v["status"] = a["status"].clone();
    v["g2_borrowed_forced_pressure"] = a.clone();
    v["independent_proof_count"] = a["independent_proof_count"].clone();
    v["new_independent_exports"] = a["new_independent_exports"].clone();
    v["SDK_requests"] = a["known_verified_SDK_requests"].clone();
    v["owned_live_jobs"] = json!([]);
    v["pushed"] = json!(true);
    v["remote_verified"] = json!(true);
    v["last_goal_turn_classification"] = json!("progress");
    v["next_action"] = json!(if a["qualified_pressure"] == true {
        "Complete concrete clean-source borrowed G2 adoption assessment as explicit engineering exception, retaining physical16 failed bounds; no old latency reroll. Then inspect empty-victim epoch work for next CPU reduction."
    } else {
        "Inspect forced-pressure failure and actual observed evidence without larger-budget retry; retain primary32 gain and old gates, do not promote source until justified."
    });
    save(path, &v);
    let hand = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hand).unwrap();
    fs::write(hand,format!("CURRENT COMPLETE/PUSHED — BORROWED G2 FORCED PRESSURE\n\nHEAD/remoteverified{head};root{R};report{D}/README.md;{};knownSDK{}/newGnark{}/ledger{}. Oldboundedqualification99384bb5fpassed248SDK89exportsbuthigh0;thisnewscenario112MiBballast/high96/max192/noSwap requiresactualhigh>0 plusnoOOM/max. No SDK/source/librarychange/no latencyreroll/no largerretry. Readanalysis/interpretation for eachcase;ownedservicesclean, noownedjobs/cloud. GoalACTIVE0/8;primary32clear2.13%gainretained/oldphysical16boundsFALSE/noadoptionyet. Nextsourceadoptionassessmentifpressurepassed, otherwiseinspectrealfailure;neverflipoldgates.\n\n{old}",a["status"],a["known_verified_SDK_requests"],a["new_independent_exports"],a["independent_proof_count"])).unwrap();
    println!("State/handoffupdated {head}");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("state") => state(),
        _ => panic!("archive/state"),
    }
}
