use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs,
    io::Read,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fft-eight-columns175";
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
fn out(command: &mut Command) -> String {
    let o = command.output().unwrap();
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
    String::from_utf8(o.stdout).unwrap()
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("code-audit.json").exists());
    fs::create_dir(r.join("codegen")).unwrap();
    let mut variants = vec![];
    for label in ["accepted", "control", "candidate"] {
        let binary = if label == "accepted" {
            PathBuf::from(
                "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk",
            )
        } else {
            r.join(format!("native/{label}-sdk"))
        };
        let symbols = out(Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary));
        let mut selected = vec![];
        let mut helpers = vec![];
        let mut callers = vec![];
        let mut lines = String::new();
        for line in symbols.lines() {
            let f: Vec<_> = line.split_whitespace().collect();
            if f.len() < 4 {
                continue;
            }
            let name = f[3..].join(" ");
            let helper = name.contains("zopro_core::fft::pair::leaf::eight::");
            let caller = name.contains("zopro_core::fft::pair::leaf::dif_serial");
            if !helper && !caller {
                continue;
            }
            let address = u64::from_str_radix(f[0], 16).unwrap();
            let bytes = u64::from_str_radix(f[1], 16).unwrap();
            lines.push_str(line);
            lines.push('\n');
            let asm = out(Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address={address}"))
                .arg(format!("--stop-address={}", address + bytes))
                .arg(&binary));
            let path = r.join(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&path, &asm).unwrap();
            let instructions: Vec<_> = asm
                .lines()
                .filter_map(|l| {
                    let (a, b) = l.trim().split_once(':')?;
                    if !a.is_empty()
                        && a.chars().all(|c| c.is_ascii_hexdigit())
                        && !b.trim().is_empty()
                    {
                        Some(b.trim())
                    } else {
                        None
                    }
                })
                .collect();
            let calls = instructions
                .iter()
                .filter(|l| {
                    (l.starts_with("call") || l.starts_with("jmp")) && l.contains("eight::")
                })
                .count();
            let item = json!({"symbol":name,"bytes":bytes,"address":address,"assembly":path,"assembly_sha256":hash(&path),"instruction_lines":instructions.len(),"ifma_sites":instructions.iter().filter(|l|l.contains("vpmadd52")).count(),"column_call_or_jump_sites":calls});
            if helper {
                helpers.push(item.clone())
            }
            if caller {
                callers.push(item.clone())
            }
            selected.push(item);
        }
        fs::write(r.join(format!("codegen/{label}-symbols.txt")), lines).unwrap();
        assert!(!callers.is_empty());
        if label == "candidate" {
            assert!(helpers
                .iter()
                .any(|v| v["symbol"].as_str().unwrap().contains("eight::dif_serial")));
            assert!(helpers
                .iter()
                .any(|v| v["ifma_sites"].as_u64().unwrap() > 0));
            assert!(callers
                .iter()
                .any(|v| v["column_call_or_jump_sites"].as_u64().unwrap() > 0));
        } else {
            assert!(helpers.is_empty());
            assert!(callers.iter().all(|v| v["column_call_or_jump_sites"] == 0));
        }
        variants.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected,"column_functions":helpers,"DIF_leaf_callers":callers}));
    }
    let result: Value = json!({"caller_route_verified":true,"static_counts_not_dynamic_or_memory_bounds":true,"performance_gain_claimed":false,"variants":variants});
    fs::write(
        r.join("code-audit.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Actual linked FFT independent-column DIF leaf route verified; OFF absent; no performance claim");
}
