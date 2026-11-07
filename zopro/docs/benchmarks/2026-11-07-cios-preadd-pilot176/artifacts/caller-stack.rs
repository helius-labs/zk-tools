use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read};
const R: &str = "/home/ubuntu/src/zopro-perf-work/cios-preadd-sdk176/primary-pilot";
fn load(p: &str) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: &str) -> String {
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
    let plan = load(&format!("{R}/caller-codegen-plan.json"));
    assert_eq!(
        load(&format!("{R}/caller-codegen-runs/result.json"))["status"],
        "complete"
    );
    let mut rows = vec![];
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let base = format!("{R}/caller-codegen-runs/{n}");
        let cmd = load(&format!("{base}/command.json"));
        let a = cmd["argv"].as_array().unwrap();
        let b = s["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - b.len()..], b);
        let e = load(&format!("{base}/exit.json"));
        assert_eq!(e["success"], true);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(&format!("{base}/{stream}.log")),
                e[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
        let text = fs::read_to_string(format!("{base}/stdout.log")).unwrap();
        let lines: Vec<_> = text
            .lines()
            .filter_map(|l| {
                l.split_once(':').and_then(|(address, body)| {
                    u64::from_str_radix(address.trim(), 16)
                        .ok()
                        .map(|_| body.trim())
                })
            })
            .collect();
        assert!(lines.len() > 100);
        let mut sub = 0;
        let mut pushes = 0;
        let mut prologue = vec![];
        for l in &lines {
            if l.starts_with("vmov") {
                break;
            }
            prologue.push(l.to_string());
            if let Some(v) = l.strip_prefix("sub    rsp,0x") {
                sub += u64::from_str_radix(v, 16).unwrap();
            }
            if l.starts_with("push ") {
                pushes += 1;
            }
        }
        assert!(sub > 0);
        let ifma = lines.iter().filter(|l| l.starts_with("vpmadd52")).count();
        rows.push(json!({"name":n,"decoded_instructions":lines.len(),"static_IFMA_sites":ifma,"prologue_rsp_subtraction_bytes":sub,"pushed_registers":pushes,"prologue":prologue,"binary":b.last().unwrap(),"binary_sha256":hash(b.last().unwrap().as_str().unwrap()),"decoded_stream_sha256":hash(&format!("{base}/stdout.log"))}));
    }
    assert_eq!(rows.len(), 4);
    assert_eq!(rows[0]["prologue_rsp_subtraction_bytes"], 4288);
    assert_eq!(rows[1]["prologue_rsp_subtraction_bytes"], 4288);
    assert_eq!(rows[2]["prologue_rsp_subtraction_bytes"], 5952);
    assert_eq!(rows[3]["prologue_rsp_subtraction_bytes"], 6080);
    let out = format!("{R}/caller-stack-analysis.json");
    assert!(!std::path::Path::new(&out).exists());
    fs::write(out,serde_json::to_vec_pretty(&json!({"review_passed":true,"rows":rows,"limits":"Selected two emitted callers only. RSP subtraction excludes pushed registers, alignment loss, nested callee frames and concurrent workers; not a complete stack/live-memory bound. Static instruction/IFMA counts are not dynamic counts."})).unwrap()).unwrap();
    println!("4caller receipts/streams checked;G1prologue same,G2+128B");
}
