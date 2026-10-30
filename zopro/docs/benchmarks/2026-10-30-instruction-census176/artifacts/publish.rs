use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/instruction-census176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-instruction-census176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn walk(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let e = e.unwrap();
        let path = e.path();
        let ty = e.file_type().unwrap();
        assert!(!ty.is_symlink());
        if ty.is_dir() {
            walk(&path, out)
        } else {
            assert!(ty.is_file());
            out.push(path);
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    assert!(!d.exists());
    let f = read(r.join("findings.json"));
    let review = read(r.join("independent-review.json"));
    assert_eq!(review["passed"], true);
    assert_eq!(f["samples"], 42495);
    assert_eq!(f["period"], 679920000000u64);
    assert_eq!(f["production_changed"], false);
    assert_eq!(f["proof_ledger"], 27353);
    let out = d.join("artifacts");
    fs::create_dir_all(&out).unwrap();
    let mut archive_hashes = BTreeMap::new();
    for name in ["extraction", "analysis", "analysis-workflow"] {
        let mut files = vec![];
        walk(&r.join(name), &mut files);
        for p in files {
            archive_hashes.insert(p.strip_prefix(r).unwrap().display().to_string(), hash(p));
        }
    }
    save_json(out.join("archive-file-hashes.json"), &json!(archive_hashes));
    for n in [
        "findings.json",
        "independent-review.json",
        "design.md",
        "history-audit.md",
        "prepare.rs",
        "analyze.rs",
        "publish.rs",
        "analysis-config.json",
        "analysis-pins.json",
        "analysis-plan.json",
        "analysis-template.json",
        "extraction-plan.json",
        "extraction-pins.json",
        "prepare-extraction.log",
        "prepare-analysis.log",
        "extraction.log",
        "analysis.log",
        "independent-review.log",
    ] {
        fs::copy(r.join(n), out.join(n)).unwrap();
    }
    for name in ["extraction", "analysis", "analysis-workflow"] {
        let status = Command::new("tar")
            .args(["-czf"])
            .arg(out.join(format!("{name}.tar.gz")))
            .arg("-C")
            .arg(r)
            .arg(name)
            .status()
            .unwrap();
        assert!(status.success());
        let status = Command::new("gzip")
            .args(["-t"])
            .arg(out.join(format!("{name}.tar.gz")))
            .status()
            .unwrap();
        assert!(status.success());
    }
    let mut md=String::from("# Current accepted CPU instruction attribution\n\nOffline analysis of four existing qualified primary captures completes with **42,495 samples**, weighted period **679,920,000,000**, and exact sample conservation. These are the 16/32-worker captures for the accepted 176-file SDK and its previous accepted control. No new proof, test suite, native prover build, timing or profile recording occurs. Production remains unchanged; no new optimization is adopted. The distinct verified-export ledger remains 27,353.\n\nThe existing Rust xtask reader binds disassembly bytes to each executable, translates recorded MMAP2 addresses through ELF load segments and requires exact sampled instruction boundaries. A separate audit independently reconciles every raw sampled IP, PID, actual TID set, family/memory/scope/function bin and total. There are 41,709 executable samples and 786 outside the executable; external samples remain explicit. Each decoded sample retains the fixed 16M period. All 15 extraction commands and the attribution command complete without a native capture or successful-test rerun.\n\n|Capture|All samples|Executable samples|IFMA % of all|Scalar move % of all|RSP-operand % of all|\n|---|---:|---:|---:|---:|---:|\n");
    for name in [
        "1x2-w16-prior",
        "1x2-w16-current",
        "1x2-w32-prior",
        "1x2-w32-current",
    ] {
        let run = &f["runs"][name];
        let n = run["summary"]["total"]["samples"].as_f64().unwrap();
        let bins = &run["all_instruction_bins"];
        writeln!(
            md,
            "|{name}|{:.0}|{}|{:.2}|{:.2}|{:.2}|",
            n,
            run["summary"]["executable"]["samples"],
            100. * bins["families"]["ifma"]["samples"].as_f64().unwrap() / n,
            100. * bins["families"]["scalar_move"]["samples"].as_f64().unwrap() / n,
            100. * bins["memory"]["rsp_operand"]["samples"].as_f64().unwrap() / n
        )
        .unwrap();
    }
    md.push_str("\n## Current 32-worker hot contexts\n\n|Context|Samples|% of all|IFMA % within context|Scalar move % within|RSP-operand % within|\n|---|---:|---:|---:|---:|---:|\n");
    for label in [
        "G1 stream",
        "G2 borrowed add",
        "G1 tree reduce",
        "G2 tree reduce",
        "Epoch pin context",
    ] {
        let row = &f["runs"]["1x2-w32-current"]["selected_contexts"][label];
        writeln!(
            md,
            "|{label}|{}|{:.2}|{:.2}|{:.2}|{:.2}|",
            row["samples"],
            row["share_of_all"].as_f64().unwrap() * 100.,
            row["ifma_within"].as_f64().unwrap() * 100.,
            row["scalar_move_within"].as_f64().unwrap() * 100.,
            row["rsp_within"].as_f64().unwrap() * 100.
        )
        .unwrap();
    }
    md.push_str("\nInstruction families and memory tags are overlapping views, not additive costs. Sampling can skid; inlining and aliases affect context labels. RSP operands do not establish spills, actual traffic or removable latency. Scalar-move samples can include register moves and memory accesses. The lower scalar-move share in current borrowed G2 is consistent with the adopted removal of scalar Y-sign work, but these single profile observations do not isolate that effect or replace the measured full-proof result.\n\nThe generic G2 tree reducer has more RSP-tagged samples within its own context than borrowed G2 addition. Source inspection finds output returned through canonical component arrays, interleaved into point arrays and scattered into the tree. This motivates inspecting a direct output-store path while keeping input packing, arithmetic, pair scheduling and topology unchanged. That path is not implemented or timed here. The G2 reducer is only 5.93% of this profile, so its stack-tag percentage is not a credible 2× whole-proof opportunity. A native candidate needs an actual distinct mechanism and complete threaded-caller evidence before any SDK promotion.\n\n## History and papers\n\nThe [history audit](artifacts/history-audit.md) keeps prior failures closed: direct linear-wire/query-rank reductions, bounded/global/partial scalar-window reuse, queue replacement, partition/arena changes, joint G1 coordinate I/O, staged kernels and tested R260 layouts. [SRE](https://arxiv.org/abs/2610.27452) and the [Waterloo thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403) were already considered in recorded local experiments. Their results do not establish a gain for this native prover. The rejected joint G1 input/output transpose is distinct from an output-only G2-tree hypothesis, but remains a required negative reference. No unchanged candidate is rerun.\n\n## Accepted proof results, unchanged\n\nThese are the prior measured accepted 32-worker full-proof results and supplied historical M5 references, not timings from this diagnostic. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) records the native flags, actual SDK, correctness and resource scope.\n\n|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|\n|---|---:|---:|---:|---:|\n");
    let cases = read("/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/cases.json");
    for c in cases
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
    {
        let shape = c["shape"].as_str().unwrap();
        let ms = f["all_current_local_rows"][c["name"].as_str().unwrap()]
            .as_f64()
            .unwrap();
        if let Some(cpu) = c["reference"]["cpu_ms"].as_f64() {
            let metal = c["reference"]["metal_ms"].as_f64().unwrap();
            writeln!(
                md,
                "|{shape}|{ms:.6}|{cpu:.0} / {:.3}×|{metal:.0} / {:.3}×|{:.1}|",
                cpu / ms,
                metal / ms,
                cpu / 10.
            )
            .unwrap();
        } else {
            writeln!(md, "|{shape}|{ms:.6}|—|—|—|").unwrap();
        }
    }
    md.push_str("\nThe original all-eight 10× CPU objective remains active and unmet, **0/8**. Primary needs 3.7 ms versus accepted 7.928660 ms. This diagnostic adds no AWS/Aeglos, phone/ARM/TEE, huge-circuit, swap, minimum-RAM or concurrent-throughput result. It introduces no lookup table, retained point copy, execution plan or prover allocation.\n\n[Design](artifacts/design.md), [findings](artifacts/findings.json), [independent review](artifacts/independent-review.json), [extraction plan](artifacts/extraction-plan.json), [attribution configuration](artifacts/analysis-config.json), [extraction receipts](artifacts/extraction.tar.gz), [attribution results and per-IP bins](artifacts/analysis.tar.gz), [attribution workflow](artifacts/analysis-workflow.tar.gz), [archive file hashes](artifacts/archive-file-hashes.json), [checksums](SHA256SUMS).\n");
    fs::write(d.join("README.md"), md).unwrap();
    println!("Published offline instruction census176;42495 samples;0 new proofs/timings;source and accepted results unchanged");
}
fn save_json(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
