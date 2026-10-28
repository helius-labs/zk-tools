use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g2-canonical-xsum";
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
fn main() {
    let mut rows = vec![];
    for variant in ["frozen", "off", "on"] {
        let p = format!("{R}/{variant}-probe");
        let out = Command::new("nm")
            .arg("--defined-only")
            .arg(&p)
            .output()
            .unwrap();
        assert!(out.status.success());
        for (i, line) in String::from_utf8(out.stdout)
            .unwrap()
            .lines()
            .filter(|s| s.contains("g2_prefix"))
            .enumerate()
        {
            let symbol = line.split_whitespace().last().unwrap();
            let out = Command::new("objdump")
                .args(["-d", "--no-show-raw-insn"])
                .arg(format!("--disassemble={symbol}"))
                .arg(&p)
                .output()
                .unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            let mut ops = BTreeMap::<String, u64>::new();
            for line in text.lines() {
                if let Some((address, rest)) = line.split_once(':') {
                    if !address.trim().is_empty()
                        && address.trim().chars().all(|c| c.is_ascii_hexdigit())
                    {
                        if let Some(op) = rest.split_whitespace().next() {
                            *ops.entry(op.to_owned()).or_default() += 1
                        }
                    }
                }
            }
            let file = format!("control-audit-{variant}-{i}.asm.txt");
            fs::write(format!("{R}/{file}"), &text).unwrap();
            rows.push(json!({"variant":variant,"symbol":symbol,"instructions":ops.values().sum::<u64>(),"opcodes":ops,"ifma_sites":text.matches("vpmadd52").count(),"executable_sha256":hash(&p),"disassembly":file}));
        }
    }
    let note = json!({"finding":"OFF preserves the generic operation but factors its original nested Xsum expression into a local variable; this does not meet the declared verbatim-OFF source invariant.","OFF_source_structure_changed":true,"OFF_executable_differs_from_frozen":hash(format!("{R}/off-probe"))!=hash(format!("{R}/frozen-probe")),"cause_of_control_gap_not_isolated":true,"existing_gates_unchanged":true,"adopted":false,"successful_capture_not_repeated":true,"next_control_requirement":"Preserve the accepted OFF expression byte-for-byte and audit control code generation before any future integration capture.","actual_symbols":rows});
    fs::write(
        format!("{R}/control-audit.json"),
        serde_json::to_vec_pretty(&note).unwrap(),
    )
    .unwrap();
    for f in ["control-audit.rs", "control-audit.json"] {
        fs::copy(format!("{R}/{f}"), format!("{D}/artifacts/{f}")).unwrap();
    }
    for row in note["actual_symbols"].as_array().unwrap() {
        let f = row["disassembly"].as_str().unwrap();
        fs::copy(format!("{R}/{f}"), format!("{D}/artifacts/{f}")).unwrap();
    }
    let p = format!("{D}/README.md");
    let mut s = fs::read_to_string(&p).unwrap();
    let old = "OFF retains its original expression.";
    assert_eq!(s.matches(old).count(), 1);
    s=s.replace(old,"OFF uses the original generic operation, with the Xsum expression factored into a local variable shared by both builds.");
    s.push_str("\n## Post-capture control audit\n\nThe OFF source preserves generic addition but factors the accepted nested Xsum expression into a local variable. This violates the declared verbatim-OFF source invariant. Its executable differs from the frozen control, so the fresh-control gain is relative to this refactored control; it cannot be attributed entirely to removing corrections. The source structure change is a possible contributor to the control gap, not an isolated cause. The unchanged gate failure and decision against adoption stand. Actual frozen/OFF/ON symbol counts and opcode inventories are in [control audit](artifacts/control-audit.json). No capture is repeated. Future integration controls must preserve the accepted OFF expression byte-for-byte and audit code generation before capture.\n");
    fs::write(p, s).unwrap();
    let mut files = vec![];
    walk(Path::new(D), &mut files);
    files.retain(|p| p.file_name().unwrap() != "SHA256SUMS");
    files.sort();
    let mut sums = String::new();
    for p in &files {
        writeln!(
            sums,
            "{}  {}",
            hash(p),
            p.strip_prefix(D).unwrap().display()
        )
        .unwrap();
    }
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    let check = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    assert!(check.status.success());
    fs::write(format!("{R}/archive-verification.log"), check.stdout).unwrap();
    let p = format!("{R}/archive-summary.json");
    let mut a: Value = serde_json::from_slice(&fs::read(&p).unwrap()).unwrap();
    a["artifact_hashes"] = json!(files.len());
    a["Git_files"] = json!(files.len() + 1);
    a["OFF_source_invariant_failed"] = json!(true);
    fs::write(p, serde_json::to_vec_pretty(&a).unwrap()).unwrap();
    println!(
        "Documented OFF source-structure deviation; all {} hashes verified",
        files.len()
    );
}
