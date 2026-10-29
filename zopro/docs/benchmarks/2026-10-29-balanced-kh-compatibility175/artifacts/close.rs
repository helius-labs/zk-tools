use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-compatibility175";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-qualification175";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-compatibility175";
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
        let n = e.file_name().to_string_lossy().to_string();
        if e.file_type().unwrap().is_dir() {
            if !["target-portable", "frozen-portable"].contains(&n.as_str()) {
                walk(&e.path(), out)
            }
        } else if e.file_type().unwrap().is_file() {
            out.push(e.path());
        }
    }
}
fn report(a: &Value, full: &Value) -> String {
    let mut s=String::from("# Balanced K/H portability and resource checks\n\nThe measured 175-file candidate passes the additional compatibility and resource checks: 101 portable core tests, 8 portable prover tests, 248 internally verified SDK requests and 89 independently verified Gnark exports. All eight cold/warm cgroup cases finish under the declared limits without OOM or maximum-limit events. Production remains unchanged; the candidate is not adopted.\n\nThe [full performance qualification](../2026-10-28-balanced-kh-qualification175/README.md) remains **failed** solely on physical-16 identical-baseline calibration. At 32 workers it measured 8.039940 ms versus accepted 8.105814 ms, paired 0.813% faster. This study adds no latency measurements, repeats no successful capture and changes no performance gate.\n\n## Coverage\n\nThe source is copied byte for byte from the measured candidate: 173 of 174 original files are unchanged, one parent has the private balanced K/H hook, and one child implements the variable-width windows. No runtime arithmetic, scheduler, key, allocation forecast, native flag or library changes are made here. Native 122/124 core tests, 8 prover tests per variant and 33 field tests are inherited. Portable field source and manifests match the earlier qualified snapshot; its 11 field tests are inherited, not rerun. The new portable x86-64/no-IFMA/no-ASM build includes both balanced scalar/point oracle tests in its unfiltered 101-test core suite.\n\n- 13 portable processes cover all eight reference shapes plus 54→1 with basis images, and primary/54 with raw and denied optional-preparation paths.\n- 24 native candidate processes cover 1/4 workers, 512 KiB/1 MiB worker stacks, primary/54 and basis/raw/denied paths. Eight accepted/candidate processes disable packed, direct, canonical-batch or vector-inverse processing individually.\n- Four concurrent processes cover accepted/candidate × primary/54, with four callers making two proofs each. They check 33 idle and 37 loaded TIDs, consistent with the existing global 32-worker pool plus callers. All 32 exports verify independently.\n- Eight resource processes cover accepted/candidate × cold/warm × default/retained allocator settings. Each charges its own private image copy inside its owned transient service. Swap is disabled.\n\nResource maximum is derived from accepted primary peak RSS: rounding 1.25 × 133.5 MiB upward to 64 MiB gives 192 MiB. Warm high is 0.9 × 133.5 MiB rounded upward to 8 MiB, giving 128 MiB. Warm cases prime at 512 MiB, add 32 MiB ballast and lower high/maximum to 128/192 MiB before two fresh proofs. These are controlled test conditions, not an architectural cap or minimum-RAM estimate. No larger-limit retry is allowed.\n\nRaw and basis images retain their recorded identities and public bindings. Independent Gnark verification uses the corresponding Euler image for its verifying key; raw-loader correctness does not require changing the verifier parser.\n\n## Resource observations\n\n| Case | Passed | Cgroup peak MiB | High events | Max events | OOM/kill | Swap limit |\n|---|---|---:|---:|---:|---|---|\n");
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
    s.push_str("\nPeaks include the private image, controller, SDK and any ballast. High events identify observed throttling; complete pressure, event, CPU, memory and thread receipts are retained. The controller stops/resets only uniquely prefixed owned units and removes only their control directories. No global memory/kernel settings are changed.\n\n## Verification and limits\n\nAll 248 SDK requests verify internally. The 89 fresh 164-byte exports pass independent Gnark verification, expected public/key binding, member hashing and uniqueness against 16,411 previous proof identities. The ledger reaches 16,500; 178 proof/public members represent 89 proofs. All build, helper and source pins close. Helpers and consumer builds complete before proof/resource captures; no owned build, hash, archive or second workload overlaps them. Unrelated host work is not excluded.\n\nThis result does not authorize adoption or turn the failed full-performance gate into a pass. No fresh mobile, TEE, AWS/Aeglos, swap, 250/4999 or minimum-RAM claim is made. The giant images are unavailable here. There is no new lookup table, duplicate key plan or prover memory cap.\n\n## Inherited CPU and Metal comparisons\n\nThese are the preceding full performance qualification and supplied historical M5 references, not measurements from these functional/resource checks.\n\n| Shape | Accepted ms | Candidate ms | M5 CPU ms | M5 Metal ms | CPU/candidate | Metal/candidate |\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (shape, v) in full["new_reference_comparisons"].as_object().unwrap() {
        writeln!(
            s,
            "| {} | {:.6} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            shape,
            v["variants"]["accepted"]["ms"].as_f64().unwrap(),
            v["variants"]["ON"]["ms"].as_f64().unwrap(),
            v["reference"]["cpu_ms"].as_f64().unwrap(),
            v["reference"]["metal_ms"].as_f64().unwrap(),
            v["variants"]["ON"]["CPU_speedup"].as_f64().unwrap(),
            v["variants"]["ON"]["Metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    s.push_str("\nThe original all-eight CPU 10× goal remains active and unmet, **0/8**. Primary requires 3.7 ms. There is no supplied M5 reference for 54→1.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source reuse](artifacts/source-reuse.json), [build receipts](artifacts/build/result.json), [functional receipts](artifacts/functional/result.json), [resource receipts](artifacts/resource-result.json), [proof inventory](artifacts/proof-inventory.json). Native and portable binary payloads remain local with hashes and frozen-library receipts.\n");
    s
}
fn seal() {
    let d = Path::new(D);
    let mut paths = Vec::new();
    walk(d, &mut paths);
    let mut pins = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(d).unwrap().display().to_string();
        if ["SHA256SUMS", "artifact-index.json"].contains(&rel.as_str()) {
            continue;
        }
        pins.insert(rel, hash(p));
    }
    save(d.join("artifact-index.json"), &json!(pins));
    pins.insert(
        "artifact-index.json".into(),
        hash(d.join("artifact-index.json")),
    );
    fs::write(
        d.join("SHA256SUMS"),
        pins.iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    println!("Sealed {} hashes", pins.len());
}
fn archive() {
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let a = read(r.join("analysis.json"));
    let full = read(Path::new(Q).join("analysis.json"));
    assert_eq!(a["independent_proof_count"], 16500);
    assert_eq!(a["production_changed"], false);
    assert_eq!(full["gate_passed"], false);
    assert_eq!(full["failed_checks"], json!(["1x2-w16/OFF/accepted"]));
    for (p, h) in read(r.join("pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut paths = Vec::new();
    walk(r, &mut paths);
    let mut local = BTreeMap::new();
    for p in paths {
        let rel = p.strip_prefix(r).unwrap();
        let name = p.file_name().unwrap().to_str().unwrap();
        if name.starts_with("git-")
            || [
                "archive.log",
                "archive.stderr.log",
                "strict-hashes.log",
                "push.log",
                "commit.log",
            ]
            .contains(&name)
            || name.ends_with(".note")
        {
            continue;
        }
        let bytes = fs::read(&p).unwrap();
        if bytes.starts_with(b"\x7fELF") || bytes.starts_with(b"!<arch>") {
            local.insert(p.display().to_string(), hash(p));
            continue;
        }
        let to = d.join("artifacts").join(rel);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::write(&to, bytes).unwrap();
        assert_eq!(hash(&p), hash(to));
    }
    let mut source = Vec::new();
    walk(&d.join("artifacts/source"), &mut source);
    assert_eq!(source.len(), 175);
    save(d.join("artifacts/local-binary-pins.json"), &json!(local));
    fs::write(d.join("README.md"), report(&a, &full)).unwrap();
    seal();
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("archive") => archive(),
        Some("seal") => seal(),
        _ => panic!("archive/seal"),
    }
}
