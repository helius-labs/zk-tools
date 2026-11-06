use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/instruction-calibration-native176";
const A: &str = "/home/ubuntu/src/zopro-perf-work/instruction-calibration176";
const D: &str =
    "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-instruction-calibration176";
fn p(s: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(s)
}
fn read(s: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(s).unwrap()).unwrap()
}
fn hash(s: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(s).unwrap()))
}
fn save(s: impl AsRef<Path>, v: &Value) {
    fs::write(s, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn near(a: f64, b: f64) {
    assert!(
        (a - b).abs() < 1e-12 * a.abs().max(b.abs()).max(1.0),
        "{a} != {b}"
    );
}
fn walk(d: &Path) -> Vec<PathBuf> {
    let mut v = vec![];
    for e in fs::read_dir(d).unwrap() {
        let q = e.unwrap().path();
        if q.is_dir() {
            v.extend(walk(&q));
        } else {
            v.push(q);
        }
    }
    v.sort();
    v
}
fn main() {
    assert!(!Path::new(D).exists());
    assert!(!p("review.json").exists());
    let plan = read(p("plan.json"));
    let analysis = read(p("analysis.json"));
    assert_eq!(analysis["status"], "pass");
    assert_eq!(analysis["timed_processes"], 240);
    assert_eq!(analysis["total_repetitions"], 1680);
    let cases = read(format!("{A}/cases.json"));
    let cs = cases.as_array().unwrap();
    assert_eq!(cs.len(), 16);
    let prep = read(format!("{A}/prepared.json"));
    assert_eq!(prep["status"], "pass");
    let prepresult = read(format!("{A}/preparation/result.json"));
    assert_eq!(prepresult["status"], "complete");
    let mut pins = BTreeMap::new();
    for file in [p("pins.json"), Path::new(A).join("capture-pins.json")] {
        for (s, h) in read(&file).as_object().unwrap() {
            assert_eq!(h, &json!(hash(s)), "{s}");
            pins.insert(s.clone(), h.as_str().unwrap().to_string());
        }
    }
    // Original preparation source has an explicit archived resolution for the three
    // files changed by the affinity capability repair. Every other pin stays exact.
    for (s, h) in read(format!("{A}/pins.json")).as_object().unwrap() {
        if h != &json!(hash(s)) {
            let name = Path::new(s).file_name().unwrap().to_str().unwrap();
            assert!(["campaign.rs", "campaign", "protocol.md"].contains(&name));
            let archived = format!("{A}/affinity-premeasurement-failure/{name}");
            assert_eq!(h, &json!(hash(&archived)));
            pins.insert(archived, h.as_str().unwrap().into());
        }
    }
    for (s, h) in prep["output_pins"].as_object().unwrap() {
        assert_eq!(h, &json!(hash(s)));
        pins.insert(s.clone(), h.as_str().unwrap().into());
    }
    let original = fs::read_to_string(p("campaign.rs")).unwrap();
    let offline = fs::read_to_string(p("offline-analysis.rs")).unwrap();
    let from="            assert_eq!(\n                mean,\n                v[\"mean_cycles_per_target_instruction\"].as_f64().unwrap()\n            );";
    let to="            let serialized = v[\"mean_cycles_per_target_instruction\"].as_f64().unwrap();\n            assert!((mean - serialized).abs() <= 1e-12 * mean.abs().max(1.0));";
    assert_eq!(original.matches(from).count(), 1);
    assert_eq!(original.replace(from, to), offline);
    let completed = read(p("runs/completed.json"));
    let names = completed
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_string())
        .collect::<Vec<_>>();
    let steps = plan["steps"].as_array().unwrap();
    assert_eq!(steps.len(), 241);
    assert_eq!(names.len(), 240);
    let mut seen = BTreeSet::new();
    let mut byid: BTreeMap<String, Vec<(u64, u64)>> = BTreeMap::new();
    let mut total = 0;
    let mut mincov = 1.0f64;
    for (b, block) in names.chunks(16).enumerate() {
        let mut order = (0..16).map(|j| (j + b) % 16).collect::<Vec<_>>();
        if b % 2 == 1 {
            order.reverse();
        }
        for (j, name) in block.iter().enumerate() {
            let c = &cs[order[j]];
            let id = c["id"].as_str().unwrap();
            assert_eq!(name, &format!("b{b:02}-{id}"));
            assert_eq!(steps[16 * b + j]["name"], *name);
            assert!(seen.insert(name.clone()));
            let d = p(format!("runs/{name}"));
            let actual = read(d.join("command.json"));
            let args = steps[16 * b + j]["argv"].as_array().unwrap();
            assert_eq!(&actual["argv"].as_array().unwrap()[4..], args);
            assert_eq!(
                args,
                &vec![
                    json!("sudo"),
                    json!("-n"),
                    json!("taskset"),
                    json!("-c"),
                    json!("2"),
                    json!(p("campaign")),
                    json!("measure"),
                    json!(id),
                    json!(b.to_string())
                ]
            );
            let e = read(d.join("exit.json"));
            assert_eq!(e["success"], true);
            assert_eq!(e["returncode"], 0);
            assert_eq!(e["stdout_sha256"], hash(d.join("stdout.log")));
            assert_eq!(e["stderr_sha256"], hash(d.join("stderr.log")));
            assert!(fs::read(d.join("stderr.log")).unwrap().is_empty());
            let v = read(d.join("stdout.log"));
            assert_eq!(v["id"], id);
            assert_eq!(v["block"], b);
            assert_eq!(v["status"], "pass");
            assert_eq!(v["event_type"], 0);
            assert_eq!(v["cycle_config"], 0);
            assert_eq!(v["instruction_config"], 1);
            assert_eq!(v["flags"], json!({"leader":101,"follower":96}));
            assert_eq!(v["read_format"], 15);
            assert_eq!(v["target_instructions"], 1048576);
            assert_eq!(v["scratch_bytes"], 1048576);
            assert_eq!(v["scratch_fill"], 90);
            assert_eq!(
                v["elf"],
                read(p("elf.json"))
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|e| e["object"] == format!("{A}/objects/{id}.o"))
                    .unwrap()
                    .clone()
            );
            let reps = v["repetitions"].as_array().unwrap();
            assert_eq!(reps.len(), 7);
            let mut sum = 0u64;
            let mut ids = None;
            for (k, r) in reps.iter().enumerate() {
                assert_eq!(r["repeat"], k);
                assert_eq!(r["cpu_before"], 2);
                assert_eq!(r["cpu_after"], 2);
                let this = (
                    r["cycle_event_id"].as_u64().unwrap(),
                    r["instruction_event_id"].as_u64().unwrap(),
                );
                assert_ne!(this.0, this.1);
                if let Some(old) = ids {
                    assert_eq!(old, this);
                }
                ids = Some(this);
                let en = r["enabled_ns"].as_u64().unwrap();
                let run = r["running_ns"].as_u64().unwrap();
                assert!(en > 0 && run > 0 && run <= en);
                let cov = run as f64 / en as f64;
                assert!(cov >= 0.99);
                near(cov, r["running_fraction"].as_f64().unwrap());
                mincov = mincov.min(cov);
                let ins = r["instructions"].as_u64().unwrap();
                assert!((1027604..=1080033).contains(&ins));
                let cy = r["cycles"].as_u64().unwrap();
                assert!(cy > 0);
                sum += cy;
                total += 1;
            }
            let m = sum as f64 / (7.0 * 1048576.0);
            near(m, v["mean_cycles_per_target_instruction"].as_f64().unwrap());
            byid.entry(id.into()).or_default().push((b as u64, sum));
            for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
                pins.insert(d.join(f).display().to_string(), hash(d.join(f)));
            }
        }
    }
    assert_eq!(total, 1680);
    near(
        mincov,
        analysis["minimum_running_fraction"].as_f64().unwrap(),
    );
    for (id, blocks) in &byid {
        assert_eq!(blocks.len(), 15);
        let a = analysis["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["id"] == *id)
            .unwrap();
        let xs = a["cycles_per_target_instruction"]["all_blocks"]
            .as_array()
            .unwrap();
        let mut sums = vec![];
        for (i, (b, sum)) in blocks.iter().enumerate() {
            assert_eq!(*b, i as u64);
            near(*sum as f64 / (7.0 * 1048576.0), xs[i].as_f64().unwrap());
            sums.push(*sum);
        }
        sums.sort();
        near(
            sums[7] as f64 / (7.0 * 1048576.0),
            a["cycles_per_target_instruction"]["median"]
                .as_f64()
                .unwrap(),
        );
    }
    for c in cs {
        let id = c["id"].as_str().unwrap();
        let asm = fs::read_to_string(format!("{A}/preparation/decode-{id}/stdout.log")).unwrap();
        assert!(asm.contains("movabs r8,0x1000"));
        assert_eq!(asm.lines().filter(|l| l.contains("\tjne ")).count(), 1);
        assert_eq!(asm.lines().filter(|l| l.ends_with("\tret")).count(), 1);
        let op = if id.starts_with("ifma-lo") {
            "vpmadd52luq"
        } else if id.starts_with("ifma-hi") {
            "vpmadd52huq"
        } else if id.starts_with("add") {
            "vpaddq"
        } else if id.starts_with("sub") {
            "vpsubq"
        } else if id.starts_with("shr52") {
            "vpsrlq"
        } else {
            "vpsraq"
        };
        assert_eq!(
            asm.lines()
                .filter(|l| l.contains(&format!("\t{op} ")))
                .count(),
            256
        );
    }
    for (rel, msg) in [
        (
            "runs",
            "inprocess execution mode does not support benchmark core pinning",
        ),
        (
            "runs-affinity-fixed",
            "Unable to create counter with name 'cycles_not_in_halt'",
        ),
    ] {
        let d = format!("{A}/{rel}/b00-ifma-lo-acc-lat");
        assert_eq!(read(format!("{d}/exit.json"))["success"], false);
        assert!(fs::read_to_string(format!("{d}/stderr.log"))
            .unwrap()
            .contains(msg));
        assert!(fs::read(format!("{d}/stdout.log")).unwrap().is_empty());
    }
    assert_eq!(read(p("runs/analyze/exit.json"))["success"], false);
    assert!(fs::read_to_string(p("runs/analyze/stderr.log"))
        .unwrap()
        .contains("assertion `left == right` failed"));
    assert!(fs::read(p("offline-analysis.stderr.log"))
        .unwrap()
        .is_empty());
    let review = json!({"status":"pass","timed_processes":seen.len(),"repetitions_reconstructed_from_raw_integers":total,"all_16_decodes_reused":true,"new_decodes":0,"native_counter_gates_unchanged":true,"mean_roundoff_only_repair":true,"minimum_running_fraction":mincov,"original_workflow_terminal":"failure in analyzer after all 240 native commands succeeded","new_proofs":0,"production_changed":false});
    save(p("review.json"), &review);
    let out = Path::new(D).join("artifacts");
    fs::create_dir_all(&out).unwrap();
    let mut members = BTreeMap::<String, String>::new();
    for root in [R, A] {
        for q in walk(Path::new(root)) {
            let rel = q.strip_prefix(root).unwrap().to_str().unwrap();
            let keep = if root == R {
                q.extension()
                    .is_some_and(|e| e == "rs" || e == "md" || e == "json" || e == "log")
            } else {
                rel.starts_with("snippets/")
                    || rel == "cases.json"
                    || rel == "prepared.json"
                    || rel == "plan.json"
                    || rel == "capture-plan.json"
                    || rel == "prepare-plan.json"
                    || rel == "pins.json"
                    || rel == "capture-pins.json"
                    || rel == "host.json"
                    || rel == "protocol.md"
                    || rel == "campaign.rs"
                    || rel.starts_with("preparation/")
                    || rel.starts_with("runs/")
                    || rel.starts_with("runs-affinity-fixed/")
                    || rel.starts_with("affinity-premeasurement-failure/")
                    || rel.ends_with(".log")
            };
            if keep {
                if let Ok(s) = fs::read_to_string(&q) {
                    members.insert(
                        format!(
                            "{}/{rel}",
                            if root == R {
                                "native"
                            } else {
                                "llvm-preparation"
                            }
                        ),
                        s,
                    );
                    pins.insert(q.display().to_string(), hash(&q));
                }
            }
        }
    }
    let mut hex = BTreeMap::new();
    for c in cs {
        let id = c["id"].as_str().unwrap();
        let b = fs::read(format!("{A}/objects/{id}.o")).unwrap();
        hex.insert(
            format!("{id}.o"),
            b.iter().map(|b| format!("{b:02x}")).collect::<String>(),
        );
    }
    members.insert(
        "frozen-objects-hex.json".into(),
        serde_json::to_string_pretty(&hex).unwrap(),
    );
    let members_json = json!({"format":"UTF-8 source and receipts, with frozen ELF objects as hex","files":members});
    let tar = p("source-and-receipts.json");
    save(&tar, &members_json);
    let f = fs::File::create(out.join("source-and-receipts.json.gz")).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&tar)
        .stdout(f)
        .status()
        .unwrap()
        .success());
    let back = Command::new("gzip")
        .arg("-dc")
        .arg(out.join("source-and-receipts.json.gz"))
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(back.stdout, fs::read(&tar).unwrap());
    let mut index = BTreeMap::new();
    for (k, v) in members_json["files"].as_object().unwrap() {
        index.insert(
            k.clone(),
            format!("{:x}", Sha256::digest(v.as_str().unwrap().as_bytes())),
        );
    }
    save(out.join("archive-index.json"), &json!(index));
    save(out.join("review.json"), &review);
    save(out.join("analysis.json"), &analysis);
    save(out.join("input-and-receipt-pins.json"), &json!(pins));
    for f in [
        "campaign.rs",
        "offline-analysis.rs",
        "review-publish.rs",
        "protocol.md",
        "offline-repair.md",
    ] {
        fs::copy(p(f), out.join(f)).unwrap();
    }
    let mut md=String::from("# Ryzen 9950X instruction calibration\n\nThe measured instruction costs differ materially from the inherited LLVM scheduling model. Independent register IFMA runs at about 0.50 cycles per instruction, while dependent vector add/subtract and carry shifts take about 2 cycles. **This is hardware calibration, not a new prover optimization or full-proof timing.**\n\nAll 240 native commands completed once: 15 blocks × 16 cases × 7 internal repetitions, using frozen LLVM-generated loops pinned to CPU 2. All 1,680 repetitions passed instruction-count and event-ID checks, and every counter group reports 100% running time. The original workflow ended at an offline floating-point equality failure after capture; the separately repaired analyzer and independent review pass without recapturing samples.\n\n## Hardware measurements\n\nValues include loop, register initialization and function-call user-space overhead divided by 1,048,576 target instructions. Each loop contains 256 target instructions repeated 4,096 times. Throughput cases use 16 independent outputs or accumulators. The host is not exclusive; CPU 18 is CPU 2’s SMT sibling. Bootstrap intervals describe block repeatability on this host.\n\n| Case | Median cycles/target instruction | 95% block-bootstrap interval |\n|---|---:|---:|\n");
    for c in analysis["cases"].as_array().unwrap() {
        let s = &c["cycles_per_target_instruction"];
        md.push_str(&format!(
            "| {} | {:.6} | {:.6}–{:.6} |\n",
            c["id"].as_str().unwrap(),
            s["median"].as_f64().unwrap(),
            s["bootstrap_95"][0].as_f64().unwrap(),
            s["bootstrap_95"][1].as_f64().unwrap()
        ));
    }
    md.push_str("\nThe accumulator/source-alias cases measure a combined recurrence, not an isolated multiply-input edge. Memory cases repeatedly read 1 KiB of prefaulted scratch filled with `0x5a`; they do not measure cold loads or DRAM. Both carry-shift immediates are 52. All raw repetitions, integer counters, commands, decodes, hashes and repaired-analysis provenance are archived below.\n\n## Implication for the next candidate\n\nThe [earlier model audit](../2026-11-06-llvm-schedule-audit176/README.md) gave register IFMA reciprocal throughput 2 cycles and vector add/shift latency 1 cycle. Hardware here instead gives approximately 0.50 and 2 cycles respectively. IFMA accumulator latency remains approximately 4 cycles. The hot folded-memory loops cost 17–19% more than the register loops in paired blocks; this does not establish that eliminating folded operands improves a full kernel, where register pressure and spill traffic differ.\n\nThe working hypothesis is therefore to examine dependency chains and carry work in the actual accepted consumer before selecting another arithmetic change. A lower IFMA count alone is insufficient evidence of a faster field operation. The calibration does not prove the full kernel is dependency-bound, quantify removable proof time, validate a complete Zen 5 model, or transfer to AWS/Aeglos/M5. Existing rejected arithmetic/layout/width/scheduler candidates remain closed.\n\n## Preparation and preserved failures\n\nThe automatic LLVM snippets were preparation only and were replaced before measurement by explicit fixed-register snippets. All 16 full objects were assembled and decoded once, then qualified without execution. The first LLVM measurement command rejected internal CPU pinning in in-process mode. The repaired command then failed because libpfm could not resolve `cycles_not_in_halt`. Both stopped before snippet execution.\n\nA Rust diagnostic reused the existing relocation-free `.text` functions and read Linux hardware cycles/instructions through a pinned perf-event group. It validates executable section/function bounds, affinity, event IDs, raw retirement counts and enabled/running time. No C/assembly production rewrite, system package installation, production dependency, new production buffer, field candidate or proof is introduced. The diagnostic scratch mapping is 1 MiB; no prover RSS or minimum-RAM conclusion follows.\n\nThe final analyzer initially compared a reconstructed and JSON-parsed floating mean with exact equality (one-rounding-unit difference). The preserved offline repair changes only that comparison to a relative `1e-12` serialization tolerance. Hardware gates, samples and the original capture source/binary remain unchanged.\n\n## Unchanged full-proof baseline and goal\n\nThese are inherited accepted-176 local 32-worker single-proof timings, **not measurements from this calibration**. The accepted qualification retains its previously documented physical-16 calibration engineering exception; this study does not reopen it. The original goal remains active and unmet on all eight shapes.\n\n| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | CPU 10× target ms |\n|---|---:|---:|---:|---:|\n");
    for (sh, ms, cpu, metal) in [
        ("1x2", 7.928660, 37., 24.),
        ("2x2", 11.151932, 59., 30.),
        ("4x2", 16.086386, 109., 48.),
        ("8x2", 24.179453, 189., 74.),
        ("12x2", 33.884455, 284., 101.),
        ("16x2", 40.077381, 335., 124.),
        ("24x2", 65.850777, 528., 176.),
        ("merge24", 65.060937, 507., 175.),
    ] {
        md.push_str(&format!(
            "| {sh} | {ms:.6} | {cpu:.0} | {metal:.0} | {:.1} |\n",
            cpu / 10.
        ));
    }
    md.push_str("\nInherited merge54 time is 161.883899 ms, without a supplied M5 reference. The inherited 132.75 MiB RSS is not a newly measured peak or minimum RAM requirement. Proof ledger stays at 30,846. No new AWS, Aeglos, Gnark, portability or memory-pressure result is claimed.\n\n## Artifacts\n\n[Analysis and every block](artifacts/analysis.json), [independent review](artifacts/review.json), [protocol](artifacts/protocol.md), [capture source](artifacts/campaign.rs), [offline analyzer](artifacts/offline-analysis.rs), [review/publication source](artifacts/review-publish.rs), [repair notes](artifacts/offline-repair.md), [source/receipt/object archive](artifacts/source-and-receipts.json.gz), [archive member hashes](artifacts/archive-index.json) and [local input/receipt pins](artifacts/input-and-receipt-pins.json). The archive includes exact frozen ELF objects encoded as hex; third-party LLVM executables/libraries and source are not redistributed. Retain the original failed workflow evidence rather than resuming or repeating successful measurements.\n");
    fs::write(Path::new(D).join("README.md"), md).unwrap();
    let files = walk(Path::new(D));
    let mut sums = String::new();
    for f in &files {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(f),
            f.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    save(
        p("publication.json"),
        &json!({"status":"pass","report":D,"files_before_checksum":files.len(),"archive_members":members_json["files"].as_object().unwrap().len(),"roundtrip_exact":true,"new_proofs":0}),
    );
    println!("independent review passed; 240 commands / 1680 repetitions; published {} files and {} archived members",files.len()+1,members_json["files"].as_object().unwrap().len());
}
