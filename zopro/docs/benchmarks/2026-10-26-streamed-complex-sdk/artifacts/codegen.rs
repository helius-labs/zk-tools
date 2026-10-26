use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, io::Read, path::Path, process::Command};
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
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    let result: Value =
        serde_json::from_slice(&fs::read(root.join("native/result.json")).unwrap()).unwrap();
    assert_eq!(result["status"], "complete");
    let mut records = Vec::new();
    let mut assembly = String::new();
    for (label, binary) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate-sdk",
        ),
        ("off", "/tmp/zopro-streamed-complex-sdk/native/off-sdk"),
        ("on", "/tmp/zopro-streamed-complex-sdk/native/on-sdk"),
    ] {
        let digest = hash(binary);
        let out = Command::new("nm")
            .args(["-S", "-C"])
            .arg(binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let nm = String::from_utf8(out.stdout).unwrap();
        fs::write(root.join(format!("{label}-symbols.txt")), &nm).unwrap();
        let (mut main, mut g2, mut reduction) = (0, 0, 0);
        for line in nm.lines() {
            let f = line.split_whitespace().collect::<Vec<_>>();
            if f.len() < 4 {
                continue;
            }
            let name = f[3..].join(" ");
            let is_g2 = name.starts_with("zopro_core::msm::g2_prefix::apply::<");
            let is_reduce = name == "zopro_core::msm::reduce_layer::<zopro_bn254::curve::G2Config>";
            let is_main = name == "<zopro_core::Key>::solve_canonical_wires";
            if !(is_g2 || is_reduce || is_main) {
                continue;
            }
            main += usize::from(is_main);
            g2 += usize::from(is_g2);
            reduction += usize::from(is_reduce);
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let len = u64::from_str_radix(f[1], 16).unwrap();
            let argv = vec![
                "-d".to_owned(),
                "-C".into(),
                "--no-show-raw-insn".into(),
                format!("--start-address=0x{start:x}"),
                format!("--stop-address=0x{:x}", start + len),
                binary.into(),
            ];
            let out = Command::new("objdump").args(&argv).output().unwrap();
            assert!(out.status.success());
            let text = String::from_utf8(out.stdout).unwrap();
            let instructions = text
                .lines()
                .filter_map(|line| {
                    let (a, b) = line.split_once(':')?;
                    if !a.trim().is_empty()
                        && a.trim().chars().all(|c| c.is_ascii_hexdigit())
                        && !b.trim().is_empty()
                    {
                        Some(b.trim())
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            let subs = instructions
                .iter()
                .filter_map(|line| {
                    let s = line.strip_prefix("sub")?.trim().strip_prefix("$0x")?;
                    let (hex, reg) = s.split_once(',')?;
                    if reg.trim() == "%rsp" {
                        u64::from_str_radix(hex, 16).ok()
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            records.push(json!({"variant":label,"binary_sha256":digest,"symbol":name,"size_bytes":len,"instructions":instructions.len(),"ifma_sites":instructions.iter().filter(|line|line.starts_with("vpmadd52")).count(),"rsp_operand_lines":instructions.iter().filter(|line|line.contains("(%rsp)")).count(),"literal_rsp_subtractions":subs,"literal_rsp_subtraction_sum":subs.iter().sum::<u64>(),"calls":instructions.iter().filter(|line|line.starts_with("call")).collect::<Vec<_>>(),"objdump_argv":argv}));
            assembly.push_str(&format!("\n{label}:{name}\n{text}"));
        }
        assert_eq!((main, g2, reduction), (1, 2, 1));
        assert_eq!(hash(binary), digest);
    }
    fs::write(root.join("codegen.txt"), assembly).unwrap();
    fs::write(root.join("codegen.json"),serde_json::to_vec_pretty(&json!({"symbols":records,"scope":"Actual linked canonical-wire, both G2-prefix specializations and G2 reduction in allthree separately compiled SDKs. Static counts are not executed operation counts, spills or causal speed. Literal stack sum excludes alignment/pushes/callees. No standalone-component prediction substitutes for these callers."})).unwrap()).unwrap();
    println!("Retained12 actual SDK routines fromthree distinct controls; no timing yet");
}
