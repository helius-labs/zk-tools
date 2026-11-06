use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    io::{Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tree-production-coverage176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tree-production-coverage176";
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn hash_file(p: &Path) -> String {
    let mut file = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut buffer = [0; 65536];
    loop {
        let n = file.read(&mut buffer).unwrap();
        if n == 0 {
            break;
        }
        h.update(&buffer[..n]);
    }
    format!("{:x}", h.finalize())
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
    let text = std::str::from_utf8(&b).expect("only explicit UTF8 helper/delta/receipt members");
    json!({"bytes":b.len(),"sha256":hash(&b),"utf8":text})
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
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["complete"], true);
    assert_eq!(read(format!("{R}/runs/result.json"))["status"], "complete");
    let dir = Path::new(R).join("runs/accepted-coverage-audit");
    let e = read(dir.join("exit.json"));
    assert_eq!(e["returncode"], 0);
    assert_eq!(e["success"], true);
    for key in ["stdout", "stderr"] {
        assert_eq!(
            hash_file(&dir.join(format!("{key}.log"))),
            e[format!("{key}_sha256")]
        );
    }
    let cmd = read(dir.join("command.json"));
    let plan = read(format!("{R}/plan.json"));
    assert_eq!(
        &cmd["argv"].as_array().unwrap()[4..],
        plan["steps"][0]["argv"].as_array().unwrap().as_slice()
    );
    for (p, h) in read(format!("{R}/pins.json")).as_object().unwrap() {
        assert_eq!(hash_file(Path::new(p)), h.as_str().unwrap());
    }
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let mut report=String::from("# Production coverage of the G1 tree experiment\n\nThe [complete-caller experiment](../2026-11-06-g1-tree-callers176/README.md) reduced the cost and memory of large grouped reductions but did not pass its acceptance gate. This read-only audit checks which accepted proof paths it changes. It reuses existing accepted 1×2 profiles; no new proving time is measured.\n\n");
    report.push_str(&fs::read_to_string(format!("{R}/decision.md")).unwrap());
    report.push_str("\n\n## Exact source coverage\n\nThe accepted 176 production files and SDK6d83 are hash-checked. The separate G1 tree candidate retains the streaming kernel byte-for-byte. Source anchors are conditional paths, not captured execution counts.\n\n|Source|Line|Role|\n|---|---:|---|\n");
    for row in a["source_anchors"].as_array().unwrap() {
        report.push_str(&format!(
            "|[{}](../../../{}#L{})|{}|{}|\n",
            row["path"].as_str().unwrap(),
            row["path"].as_str().unwrap(),
            row["line"],
            row["line"],
            row["meaning"].as_str().unwrap()
        ));
    }
    report.push_str("\n## Existing accepted profile locations\n\nCounts and shares below are flat user-cycle samples from the accepted SDK, not the instrumented candidate. They conserve all saved bins. Ownership/period/loss validation is inherited from the [existing coarse-phase review](../2026-11-06-coarse-proof-phase-universal176/README.md); no raw capture or decode was repeated. The saved absolute-period-coverage limitation and failed observer gate remain visible in the analysis. These two profiles do not establish coverage for all shapes.\n\n|Location|16-worker samples / share|32-worker samples / share|\n|---|---:|---:|\n");
    for cat in [
        "G1 streamed add",
        "G2 streamed add",
        "G1 tree reduce_layer",
        "G2 tree reduce_layer",
        "shared scalar Fq inverse",
        "Crossbeam with_handle",
        "other",
        "unknown",
    ] {
        let value = |w| {
            let p = &a["profiles"][format!("cycles-1x2-w{w}-accepted")]["categories"][cat];
            format!(
                "{} / {:.3}%",
                p["samples"],
                p["share"].as_f64().unwrap() * 100.
            )
        };
        report.push_str(&format!("|{cat}|{}|{}|\n", value(16), value(32)));
    }
    report.push_str("\nHigh sampled Crossbeam use at 32 workers is useful evidence, but it does not establish a new scheduler optimization. The [window-local weighted](../2026-10-22-universal-window-local-weighted-sdk/README.md), [weighted task-grain](../2026-10-22-universal-weighted-task-grain/README.md) and [shared-frontier](../2026-10-31-weighted-frontier176/README.md) experiments already tested distinct attempts in that direction and remain rejected.\n\n## Accepted CPU and Metal comparisons, unchanged\n\n");
    let old=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g1-tagged-phi-review176/README.md").unwrap();
    let sec = &old[old
        .find("Inherited local 32-worker full-proof observations")
        .unwrap()..];
    report.push_str(&sec[..sec.find("\n[Analysis]").unwrap()]);
    report.push_str("\n\n## Continuation and artifacts\n\nThe [next step](artifacts/next.md) is a useful-work/dependency audit of the accepted streamed kernels before choosing another implementation. No new runtime candidate, full-SDK trial or adoption is selected. The full all-eight CPU10× goal remains active and unmet.\n\nThe one read-only Rust/xtask step passed on its first execution. All source pins, saved sample conservation checks and exact stream-body comparison passed. The helper archive contains the audit source, design, command/exit/output receipts and publication sources; it does not contain SDK/library binaries, private inputs, keys/witnesses/proofs or raw profiles. Existing profile binaries/recordings and their original ownership evidence remain at their source experiment.\n\n[Analysis](artifacts/analysis.json), [design](artifacts/design.md), [decision](artifacts/decision.md), [next](artifacts/next.md), [audit source](artifacts/audit.rs), [plan](artifacts/plan.json), [input pins](artifacts/pins.json), [helpers and receipts](artifacts/helpers-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS).\n");
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "design.md",
        "decision.md",
        "next.md",
        "audit.rs",
        "plan.json",
        "pins.json",
        "publish.rs",
    ] {
        fs::copy(format!("{R}/{f}"), art.join(f)).unwrap();
    }
    let mut members = BTreeMap::new();
    for entry in fs::read_dir(R).unwrap() {
        let p = entry.unwrap().path();
        if p.is_file()
            && p.extension()
                .is_some_and(|e| ["rs", "md", "json", "log"].iter().any(|s| e == *s))
        {
            members.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
        }
    }
    for p in walk(&Path::new(R).join("runs")) {
        members.insert(p.strip_prefix(R).unwrap().display().to_string(), member(&p));
    }
    archive(&art.join("helpers-and-receipts.json.gz"), &members);
    fs::write(art.join("publication.json"),serde_json::to_vec_pretty(&json!({"passed":true,"read_only_commands":1,"source_files_checked":176,"existing_samples_conserved":20837,"exact_stream_body_verified":true,"native_builds_tests_captures_or_decodes_rerun":false,"new_timing_observations":0,"helper_receipt_UTF8_members":members.len(),"archive_hashes_and_roundtrip_verified":true,"new_proofs":0,"production_changed":false,"adopted":false,"goal_status":"active/unmet0/8"})).unwrap()).unwrap();
    let mut sums = String::new();
    for p in walk(Path::new(D)) {
        sums.push_str(&format!(
            "{}  {}\n",
            hash_file(&p),
            p.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Published {} files/{} exactUTF8 members; archive/hash/receipt checks PASS; no runtime change",walk(Path::new(D)).len(),members.len());
}
