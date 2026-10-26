use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-phased-complex-products");
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
        ("shared", &fresh, "shared", 1),
        ("phased", &fresh, "phased", 2),
    ];
    let mut records = Vec::new();
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
            "shared" => "complex_shared_codegen",
            _ => "complex_phased_codegen",
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
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-input-pins-v2.json"))).unwrap();
    for name in [
        "accepted-probe",
        "target/release/probe",
        "target/release/deps/libzopro_bn254-fdaae4a8f3010cbc.rlib",
        "build-v2/result.json",
        "build-v2/component-oracles/tests.json",
        "build-v2/component-oracles/exit.json",
        "build-v2/component-oracles/stdout.log",
        "build-v2/component-oracles/stderr.log",
        "build-plan-v2.json",
        "repair.json",
        "analyze.rs",
        "analyze",
        "timing.rs",
        "codegen.json",
        "codegen.txt",
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
