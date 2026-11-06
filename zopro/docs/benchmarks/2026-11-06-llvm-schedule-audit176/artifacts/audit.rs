use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/llvm-schedule-audit176";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-llvm-schedule-audit176";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(D).exists());
    assert!(!Path::new(R).join("analysis.json").exists());
    let v = read(format!("{R}/coverage.json"));
    assert_eq!(v["TargetInfo"]["CPUName"], "znver5");
    assert_eq!(v["CodeRegions"].as_array().unwrap().len(), 1);
    let region = &v["CodeRegions"][0];
    let ins = region["Instructions"].as_array().unwrap();
    let info = region["InstructionInfoView"]["InstructionList"]
        .as_array()
        .unwrap();
    assert_eq!(ins.len(), 8);
    assert_eq!(info.len(), 8);
    assert!(fs::read_to_string(format!("{R}/coverage.stderr.log"))
        .unwrap()
        .is_empty());
    assert_eq!(region["SummaryView"]["Iterations"], 100);
    assert_eq!(region["SummaryView"]["Instructions"], 800);
    let mut instructions = vec![];
    for (i, (asm, row)) in ins.iter().zip(info).enumerate() {
        assert_eq!(row["Instruction"], i);
        assert_eq!(row["hasUnmodeledSideEffects"], false);
        if i < 4 {
            assert!(asm.as_str().unwrap().starts_with("vpmadd52"));
            assert_eq!(row["Latency"], if i < 2 { 4 } else { 10 });
            assert_eq!(row["RThroughput"], if i < 2 { 2 } else { 1 });
            assert_eq!(row["mayLoad"], i >= 2);
        }
        instructions.push(json!({"assembly":asm,"model":row}));
    }
    assert!(v["TargetInfo"]["Resources"]
        .as_array()
        .unwrap()
        .iter()
        .all(|s| s.as_str().unwrap().starts_with("Zn4")));
    let mut aliases = vec![];
    for (f, tag) in [
        ("X86.td", "llvmorg-20.1.2"),
        ("X86-22.td", "llvmorg-22.1.2"),
        ("X86-22.1.8.td", "llvmorg-22.1.8"),
    ] {
        let s = fs::read_to_string(format!("{R}/{f}")).unwrap();
        let lines=s.lines().enumerate().filter(|(_,s)|s.contains("ProcModel<\"znver5\", Znver4Model")).map(|(n,s)|json!({"line":n+1,"text":s,"tag":tag,"url":format!("https://github.com/llvm/llvm-project/blob/{tag}/llvm/lib/Target/X86/X86.td")})).collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        aliases.extend(lines);
    }
    let mut pins = BTreeMap::new();
    for f in [
        "audit.rs",
        "audit",
        "coverage.json",
        "coverage.stderr.log",
        "ifma-coverage.s",
        "version.txt",
        "cpus.stdout.txt",
        "cpus.stderr.txt",
        "X86.td",
        "X86-22.td",
        "X86-22.1.8.td",
        "package-sha256.txt",
        "package-expected-sha256.txt",
        "package-expected-epoch-sha256.txt",
        "download.stdout.log",
        "download.stderr.log",
        "packages/llvm-20_1%3a20.1.2-0ubuntu1~24.04.3_amd64.deb",
        "packages/libllvm20_1%3a20.1.2-0ubuntu1~24.04.3_amd64.deb",
        "tools/usr/lib/llvm-20/bin/llvm-mca",
        "tools/usr/lib/x86_64-linux-gnu/libLLVM.so.20.1",
    ] {
        let p = format!("{R}/{f}");
        pins.insert(p.clone(), hash(p));
    }
    assert_eq!(
        pins[&format!("{R}/packages/llvm-20_1%3a20.1.2-0ubuntu1~24.04.3_amd64.deb")],
        "b7ca50166943d94c786611ebe7a0dd8f5b9a9b1f81025620cf9dfcd8aac84563"
    );
    assert_eq!(
        pins[&format!("{R}/packages/libllvm20_1%3a20.1.2-0ubuntu1~24.04.3_amd64.deb")],
        "f018e71c72c7a4742fb1aff9dc98b45586f42e6ea464d4918af7e3772b7f0658"
    );
    let out = json!({"status":"pass","decision":"Do not use this inherited Zen4 model as calibrated Zen5/prover prediction; no full-consumer simulation or new arithmetic candidate selected","CPU_name":"znver5","resources_are_Zn4":true,"instruction_coverage":instructions,"upstream_aliases":aliases,"tool_version":fs::read_to_string(format!("{R}/version.txt")).unwrap(),"actual_prover_rustc":"1.98.1, LLVM22.1.8 per rustc -vV; custom Rust LLVM patchset not audited","production_changed":false,"new_native_execution":false,"new_performance_recordings":0,"new_decodes":0,"new_proofs":0,"proof_ledger":30846,"limits":["Instruction coverage is a parser/schedule check, not hardware validation.","Folded-memory IFMA model throughput1 vs register2 is uncalibrated; no hardware faster-memory conclusion.","Upstream22.1.8 source is version-matched, not proof of exact Rust LLVM fork contents.","LLVM MCA does not model instruction fetch/decode, branch prediction, caches or store-to-load forwarding.","No throughput/latency comparison with AWS, Aeglos or M5 follows."]});
    save(format!("{R}/pins.json"), &json!(pins));
    save(format!("{R}/analysis.json"), &out);
    let art = Path::new(D).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    let report = r#"# LLVM scheduling-model applicability audit

The scheduling tool parses the tested IFMA forms, but **no calibrated Ryzen9950X performance model or new runtime candidate is selected**. The `znver5` CPU name in upstream LLVM20.1.2 and22.1.8 resolves to `Znver4Model`. This audit changes the next action: obtain independent hardware/resource evidence before relying on simulated costs to choose another kernel.

## Evidence

The benchmark Rust toolchain reports1.98.1 with LLVM22.1.8. The [matching upstream LLVM tag](https://github.com/llvm/llvm-project/blob/llvmorg-22.1.8/llvm/lib/Target/X86/X86.td#L1994) explicitly assigns `znver5` to `Znver4Model`, with separate Zen5 feature/tuning sets. The custom Rust LLVM patchset was not inspected, so this is an upstream-source finding rather than proof of exact compiler-fork contents. Shared scheduling classes do not imply identical Zen4/Zen5 generated code.

No llvm-mca binary was installed initially. Two Ubuntu packages, LLVM20.1.2 and its shared library, were downloaded into a private research directory, verified against apt SHA-256 metadata and extracted locally. No system package or production dependency changed. The running tool recognizes `znver5` and exposes only `Zn4` resource names in its model output. Downloaded packages, executable and library remain local with hashes.

One fixed model-only input contains eight instructions: four IFMA register/memory forms, vector addition, logical/arithmetic shifts and a vector stack store. All eight parse and receive scheduling data without unsupported-instruction skipping or stderr. These instructions are never assembled or executed as a benchmark. A100-iteration simulation describes800 modeled instructions, not measured CPU work.

|IFMA model form|Reported latency cycles|Reported reciprocal throughput|Reported model micro-ops|
|---|---:|---:|---:|
|Register low|4|2|1|
|Register high|4|2|1|
|Folded-memory low|10|1|1|
|Folded-memory high|10|1|1|

The faster modeled folded-memory throughput is not evidence that adding loads is faster on hardware. These are uncalibrated scheduling entries; no missing resource or actual timing error is proved. No model parameters were patched, tuned or swept, and no full-consumer simulation was run using this inherited model.

## Limits and next step

The [LLVM documentation](https://llvm.org/docs/CommandGuide/llvm-mca.html#instruction-flow) states that the model omits frontend fetch/decode and branch prediction. Its [memory-model limitations](https://llvm.org/docs/CommandGuide/llvm-mca.html#load-store-unit-and-memory-consistency-model) also exclude cache hierarchy and store-to-load forwarding. Those omissions matter for the large, stack-using G2 consumers. Recognizing an instruction or CPU name is insufficient to qualify a bottleneck diagnosis.

Next, use actual saved consumers and independently checked dependency/work accounting, with explicit hardware calibration where needed. Do not infer a compiler defect, an AWS/Aeglos explanation, a removable wall-time fraction or a new field algorithm from this audit. Earlier successful models/captures/decodes and rejected arithmetic/scheduling implementations remain closed.

Preparation failures are retained in the research notes: the initial process launch named a not-yet-created working directory and performed no work; the first package-path check omitted apt's encoded epoch and stopped before extraction. Correct filenames were then verified before one extraction. CPU-help intentionally exits1 after listing targets, which stopped that command sequence after successful extraction/version/help output; those successful steps were not repeated. An incorrectly named scheduling source and a nonexistent separate Zen5 source/processor file returned404; the actual model mapping was read from X86.td. No benchmark, proof, build or successful model run was repeated for these repairs.

## Accepted baseline

Source176/SDK6d83 remain unchanged. Accepted local32-worker1×2 is7.928660 ms and merge54 is161.883899 ms. Inherited132.75 MiB RSS is neither a new peak nor minimum RAM. Ledger30846 is unchanged. The all-eight CPU10× goal remains active and unmet(0/8); inherited [M5 CPU and Metal comparisons](../2026-11-06-g2-validated-capture176/README.md#accepted-proof-baseline-unchanged) remain current. No new proof timing, AWS/Aeglos, device, huge-circuit, pressure/swap or concurrency measurement is added.

[Analysis](artifacts/analysis.json), [model output](artifacts/coverage.json), [model input](artifacts/ifma-coverage.s), [audit source](artifacts/audit.rs), [local input hashes](artifacts/pins.json), [tool version](artifacts/version.txt) and [checksums](SHA256SUMS) preserve the evidence. Third-party LLVM source and binary packages are not redistributed; upstream URLs/tags and local hashes identify them.
"#;
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    for f in [
        "analysis.json",
        "pins.json",
        "audit.rs",
        "coverage.json",
        "coverage.stderr.log",
        "ifma-coverage.s",
        "version.txt",
        "cpus.stdout.txt",
        "cpus.stderr.txt",
    ] {
        fs::copy(Path::new(R).join(f), art.join(f)).unwrap();
    }
    let mut all = vec![Path::new(D).join("README.md")];
    all.extend(
        fs::read_dir(&art)
            .unwrap()
            .map(|e| e.unwrap().path())
            .collect::<Vec<PathBuf>>(),
    );
    all.sort();
    let sums = all
        .iter()
        .map(|p| format!("{}  {}\n", hash(p), p.strip_prefix(D).unwrap().display()))
        .collect::<String>();
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    println!("Audit PASS:8 instruction forms/model identities/pins; Zen5 applicability remains uncalibrated; no candidate or performance claim.");
}
