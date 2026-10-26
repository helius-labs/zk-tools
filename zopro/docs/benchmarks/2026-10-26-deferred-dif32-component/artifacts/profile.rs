use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
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
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32");
    assert_eq!(read(root.join("timing/result.json"))["status"], "complete");
    let variants = [
        ("accepted", root.join("accepted-probe"), "reference", false),
        (
            "fresh",
            root.join("target/release/probe"),
            "reference",
            false,
        ),
        (
            "deferred",
            root.join("target/release/probe"),
            "deferred",
            true,
        ),
    ];
    let mut records = Vec::new();
    let mut assembly = String::new();
    for (label, binary, _, mode) in &variants {
        let out = Command::new("nm")
            .args(["-S", "-C"])
            .arg(binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let nm = String::from_utf8(out.stdout).unwrap();
        let symbol = format!("probe::repeat::<{mode}>");
        let matches = nm
            .lines()
            .filter(|s| s.ends_with(&symbol))
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        let f = matches[0].split_whitespace().collect::<Vec<_>>();
        let start = u64::from_str_radix(f[0], 16).unwrap();
        let size = u64::from_str_radix(f[1], 16).unwrap();
        let argv = vec![
            "-d".to_owned(),
            "-C".into(),
            format!("--start-address=0x{start:x}"),
            format!("--stop-address=0x{:x}", start + size),
            binary.to_str().unwrap().into(),
        ];
        let out = Command::new("objdump").args(&argv).output().unwrap();
        assert!(out.status.success());
        let text = String::from_utf8(out.stdout).unwrap();
        let lines = text
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
        records.push(json!({"variant":label,"symbol":symbol,"binary_sha256":hash(binary),"size_bytes":size,"instructions":lines.len(),"ifma_sites":lines.iter().filter(|l|l.contains("vpmadd52")).count(),"rsp_operand_lines":lines.iter().filter(|l|l.contains("(%rsp)")).count(),"calls":calls,"objdump_argv":argv}));
        assembly.push_str(&format!("\n{label}:{symbol}\n{text}"));
    }
    fs::write(root.join("codegen.txt"), assembly).unwrap();
    fs::write(root.join("codegen.json"),serde_json::to_vec_pretty(&json!({"symbols":records,"scope":"Actual linked repeated-tile routines; static sites are not iteration/field-operation counts. Root callback and unchanged multiplication inline here. Inspect calls and samples before interpreting outer code size."})).unwrap()).unwrap();
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(root.join("timing-input-pins.json"))).unwrap();
    for name in [
        "profile.rs",
        "analyze.rs",
        "component-result.json",
        "timing/result.json",
        "timing-plan.json",
        "timing-order.json",
        "fresh-field-libraries.json",
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
    for (label, binary, mode, _) in variants {
        let data = root.join(format!("{label}.perf.data"));
        steps.push(json!({"kind":"command","name":format!("capture-{label}"),"argv":["sudo","-n","perf","record","--no-buildid-cache","-e","cycles:u","-F","999","-o",data,"--","taskset","-c","2",binary,mode,"256","173","5000000"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("decode-{label}"),"argv":["sudo","-n","perf","script","--header","-i",data,"-F","comm,pid,tid,time,event,period,ip,sym,dso"],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{label}"),"argv":["sudo","-n","perf","script","-D","-i",data],"timeout_seconds":120}));
    }
    fs::write(root.join("profile-plan.json"),serde_json::to_vec_pretty(&json!({"version":1,"source_pins":root.join("profile-input-pins.json"),"environment":{},"steps":steps})).unwrap()).unwrap();
    println!(
        "Three exact linked routines archived; frozen after-latency profiles{}pins",
        pins.len()
    );
}
