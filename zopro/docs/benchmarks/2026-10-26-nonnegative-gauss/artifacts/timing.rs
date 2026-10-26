use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}

fn normalize(lines: &[&str]) -> String {
    lines
        .iter()
        .map(|line| {
            let line = line.split('#').next().unwrap().trim();
            let line = line
                .replace("probe::run::<1>", "repeat")
                .replace("probe::run::<2>", "repeat");
            if let Some(at) = line.find("(%rip)") {
                let start = line[..at]
                    .rfind(|c: char| c.is_whitespace() || c == ',')
                    .map(|i| i + 1)
                    .unwrap_or(0);
                format!("{}<rip>{}", &line[..start], &line[at..])
            } else {
                line
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn main() {
    let r = Path::new("/tmp/zopro-nonnegative-gauss");
    assert_eq!(read(r.join("build-v2/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("build-v2/component-oracles/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        4
    );
    let accepted = r.join("accepted-probe");
    let fresh = r.join("target/release/probe");
    let variants = [
        ("accepted", &accepted, "old", 0),
        ("fresh", &fresh, "old", 0),
        ("streamed", &fresh, "streamed", 1),
        ("gauss", &fresh, "gauss", 2),
    ];
    let mut records = Vec::new();
    let mut sequences = BTreeMap::new();
    let mut assembly = String::new();
    for (label, binary, mode, strategy) in variants {
        let o = Command::new("nm")
            .args(["-S", "-C"])
            .arg(binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let nm = String::from_utf8(o.stdout).unwrap();
        let wrapper = match mode {
            "old" => "complex_old_codegen",
            "streamed" => "complex_streamed_codegen",
            _ => "complex_gauss_codegen",
        };
        for symbol in [wrapper.to_owned(), format!("probe::run::<{strategy}>")] {
            let matches = nm
                .lines()
                .filter(|l| l.ends_with(&symbol))
                .collect::<Vec<_>>();
            assert_eq!(matches.len(), 1);
            let words = matches[0].split_whitespace().collect::<Vec<_>>();
            let start = u64::from_str_radix(words[0], 16).unwrap();
            let len = u64::from_str_radix(words[1], 16).unwrap();
            let argv = vec![
                "-d".to_owned(),
                "-C".into(),
                "--no-show-raw-insn".into(),
                format!("--start-address=0x{start:x}"),
                format!("--stop-address=0x{:x}", start + len),
                binary.to_str().unwrap().into(),
            ];
            let o = Command::new("objdump").args(&argv).output().unwrap();
            assert!(o.status.success());
            let text = String::from_utf8(o.stdout).unwrap();
            let lines = text
                .lines()
                .filter_map(|l| {
                    let (a, b) = l.split_once(':')?;
                    if a.trim().chars().all(|c| c.is_ascii_hexdigit())
                        && !a.trim().is_empty()
                        && !b.trim().is_empty()
                    {
                        Some(b.trim())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            sequences.insert(
                format!(
                    "{label}:{}",
                    if symbol.starts_with("probe::run") {
                        "repeat"
                    } else {
                        "wrapper"
                    }
                ),
                normalize(&lines),
            );
            let subs = lines
                .iter()
                .filter_map(|l| {
                    let s = l.strip_prefix("sub")?.trim().strip_prefix("$0x")?;
                    let (hex, reg) = s.split_once(',')?;
                    if reg.trim() == "%rsp" {
                        u64::from_str_radix(hex, 16).ok()
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            records.push(json!({"variant":label,"binary_sha256":hash(binary),"symbol":symbol,"size_bytes":len,"instructions":lines.len(),"ifma_sites":lines.iter().filter(|l|l.starts_with("vpmadd52")).count(),"rsp_operand_lines":lines.iter().filter(|l|l.contains("(%rsp)")).count(),"literal_rsp_subtractions":subs,"literal_rsp_subtraction_sum":subs.iter().sum::<u64>(),"calls":lines.iter().filter(|l|l.starts_with("call")).collect::<Vec<_>>(),"objdump_argv":argv}));
            assembly.push_str(&format!("\n{label}:{symbol}\n{text}"));
        }
    }
    fs::write(r.join("codegen.txt"), assembly).unwrap();
    fs::write(r.join("codegen.json"),serde_json::to_vec_pretty(&json!({"symbols":records,"scope":"Actual component wrappers and full repeat functions. Static sites/stack references are not dynamic operation/spill counts. Literal prologue subtractions exclude pushes/alignment/callees. No actual SDK/G2 caller inspected here."})).unwrap()).unwrap();

    let candidate = records
        .iter()
        .find(|r| r["variant"] == "gauss" && r["symbol"] == "complex_gauss_codegen")
        .unwrap();
    let wrapper_same = sequences["fresh:wrapper"] == sequences["gauss:wrapper"];
    let repeat_same = sequences["fresh:repeat"] == sequences["gauss:repeat"];
    let codegen_pass = candidate["ifma_sites"] == 250 && !(wrapper_same && repeat_same);
    fs::write(r.join("codegen-screen.json"),serde_json::to_vec_pretty(&json!({"passed":codegen_pass,"standalone_expected_ifma":250,"standalone_actual_ifma":candidate["ifma_sites"],"ordinary_vs_gauss_wrapper_normalized_equal":wrapper_same,"ordinary_vs_gauss_repeat_normalized_equal":repeat_same,"normalization":"Remove instruction addresses, symbol-address comments and RIP-relative displacement only; map repeat strategy labels to same self name. Static equivalence screen only.","sequence_sha256":sequences.iter().map(|(k,v)|(k,format!("{:x}",Sha256::digest(v.as_bytes())))).collect::<BTreeMap<_,_>>(),"limitation":"Static instruction/stack counts are not dynamic spills or performance."})).unwrap()).unwrap();
    assert!(
        codegen_pass,
        "declared codegen screen fails; close without timing"
    );
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-input-pins-v2.json"))).unwrap();
    for name in [
        "accepted-probe",
        "target/release/probe",
        "build-v2/result.json",
        "build-v2/component-oracles/tests.json",
        "build-v2/component-oracles/exit.json",
        "build-v2/component-oracles/stdout.log",
        "build-v2/component-oracles/stderr.log",
        "build-plan-v2.json",
        "analyze.rs",
        "analyze",
        "timing.rs",
        "codegen.json",
        "codegen.txt",
        "codegen-screen.json",
        "adapt-runner.rs",
        "harness-repair.json",
        "repair-tests.rs",
    ] {
        let p = r.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
    }
    for entry in fs::read_dir(r.join("target/release/deps")).unwrap(){let p=entry.unwrap().path();let n=p.file_name().unwrap().to_str().unwrap();if n.starts_with("libzopro_bn254-")&&n.ends_with(".rlib"){pins.insert(p.to_str().unwrap().into(),hash(&p));}}
    let patterns = ["chain", "streams4", "scaled", "affine-expression"];
    let mut order = Vec::new();
    let mut steps = Vec::new();
    for block in 0..31 {
        let mut indices = (0..16).map(|i| (i + block) % 16).collect::<Vec<_>>();
        if block % 2 == 1 {
            indices.reverse();
        }
        for index in indices {
            let pi = index / 4;
            let vi = index % 4;
            let (label, binary, mode, strategy) = variants[vi];
            let pattern = patterns[pi];
            let name = format!("b{block:02}-{pattern}-{label}");
            steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","2",binary,mode,pattern,"100000"],"timeout_seconds":60}));
            order.push(json!({"name":name,"pattern":pattern,"block":block,"variant":label,"mode":mode,"strategy":strategy}));
        }
    }
    assert_eq!(steps.len(), 496);
    fs::write(
        r.join("timing-order.json"),
        serde_json::to_vec_pretty(&order).unwrap(),
    )
    .unwrap();
    pins.insert(
        r.join("timing-order.json").to_str().unwrap().into(),
        hash(r.join("timing-order.json")),
    );
    fs::write(
        r.join("timing-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(r.join("timing-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":r.join("timing-input-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    println!(
        "Frozen496processes{}pins,8exactlinkedroutines; latency only then profiles",
        pins.len()
    );
}
