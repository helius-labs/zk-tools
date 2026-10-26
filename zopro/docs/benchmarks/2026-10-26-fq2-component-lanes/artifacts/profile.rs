use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let root = Path::new("/tmp/zopro-fq2-component-lanes");
    let timing: Value =
        serde_json::from_slice(&fs::read(root.join("timing/result.json")).unwrap()).unwrap();
    assert_eq!(timing["status"], "complete");
    let nm = Command::new("nm")
        .args(["-S", "-C"])
        .arg(root.join("probe"))
        .output()
        .unwrap();
    assert!(nm.status.success());
    let nm = String::from_utf8(nm.stdout).unwrap();
    let mut records = Vec::new();
    let mut disassembly = String::new();
    for pattern in [
        "chain",
        "four_streams",
        "scaled_product",
        "affine_expression",
    ] {
        for mode in ["Split", "Interleaved"] {
            let symbol = format!("probe::{pattern}::<probe::{mode}>");
            let matches = nm
                .lines()
                .filter(|s| s.ends_with(&symbol))
                .collect::<Vec<_>>();
            assert_eq!(matches.len(), 1, "{symbol}");
            let f = matches[0].split_whitespace().collect::<Vec<_>>();
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            let argv = vec![
                "-d".into(),
                "-C".into(),
                format!("--start-address=0x{start:x}"),
                format!("--stop-address=0x{:x}", start + size),
                root.join("probe").to_str().unwrap().to_owned(),
            ];
            let out = Command::new("objdump").args(&argv).output().unwrap();
            assert!(out.status.success());
            let out = String::from_utf8(out.stdout).unwrap();
            let lines = out
                .lines()
                .filter(|l| {
                    l.split_once(':').is_some_and(|(a, b)| {
                        !a.trim().is_empty()
                            && a.trim().chars().all(|c| c.is_ascii_hexdigit())
                            && b.contains('\t')
                    })
                })
                .collect::<Vec<_>>();
            let calls = lines
                .iter()
                .filter(|l| l.contains("\tcall"))
                .map(|l| l.to_string())
                .collect::<Vec<_>>();
            records.push(json!({"symbol":symbol,"start":format!("0x{start:x}"),"size_bytes":size,"instructions":lines.len(),"ifma_sites":lines.iter().filter(|l|l.contains("vpmadd52")).count(),"rsp_operand_lines":lines.iter().filter(|l|l.contains("(%rsp)")).count(),"calls":calls,"objdump_argv":argv}));
            disassembly.push_str(&format!("\n{symbol}\n{out}"));
        }
    }
    fs::write(root.join("codegen.txt"), disassembly).unwrap();
    fs::write(root.join("codegen.json"),serde_json::to_vec_pretty(&json!({"binary_sha256":hash(root.join("probe")),"symbols":records,"scope":"Static complete outer functions including setup, timed loop and output. Nested calls retained explicitly. Counts are not hot-loop dynamic counts, spill counts, cache events or SDK effects."})).unwrap()).unwrap();
    let mut pins: BTreeMap<String, String> =
        serde_json::from_slice(&fs::read(root.join("timing-input-pins.json")).unwrap()).unwrap();
    for name in [
        "profile.rs",
        "component-result.json",
        "timing/result.json",
        "timing-plan.json",
        "timing-order.json",
        "codegen.json",
        "codegen.txt",
    ] {
        let p = root.join(name);
        pins.insert(p.to_str().unwrap().into(), hash(&p));
    }
    fs::write(
        root.join("profile-input-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    let mut steps = Vec::new();
    for mode in ["reference", "interleaved"] {
        let out = root.join(format!("{mode}.perf.data"));
        steps.push(json!({"kind":"command","name":format!("capture-{mode}"),"argv":["sudo","-n","perf","record","-e","cycles:u","-F","999","-o",out,"--","taskset","-c","2",root.join("probe"),mode,"affine-expression","20000000"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{mode}"),"argv":["sudo","-n","perf","script","--header","-i",out,"-F","comm,pid,tid,time,event,period,ip,sym,dso"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{mode}"),"argv":["sudo","-n","perf","script","-D","-i",out],"timeout_seconds":120}));
    }
    fs::write(root.join("profile-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":root.join("profile-input-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    println!("Exact 8 linked routines archived; froze two after-latency component captures");
}
