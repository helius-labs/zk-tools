use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path, process::Command};
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-fused-product-overlap");
    let mut records = Vec::new();
    let mut asm = String::new();
    for (label, binary) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate-sdk",
        ),
        (
            "candidate",
            "/tmp/zopro-fused-product-overlap/native-v2/candidate-sdk",
        ),
    ] {
        let out = Command::new("nm")
            .args(["-S", "-C"])
            .arg(binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let nm = String::from_utf8(out.stdout).unwrap();
        let mut main = 0;
        let mut g2 = 0;
        for l in nm.lines() {
            let f = l.split_whitespace().collect::<Vec<_>>();
            if f.len() < 4 {
                continue;
            }
            let name = f[3..].join(" ");
            let selected = name == "<zopro_core::Key>::solve_canonical_wires"
                || name == "zopro_core::msm_affine::fused_kh::fused"
                || name.starts_with("zopro_core::msm::g2_prefix::apply::<");
            if !selected {
                continue;
            }
            if name == "<zopro_core::Key>::solve_canonical_wires" {
                main += 1;
            }
            if name.starts_with("zopro_core::msm::g2_prefix::apply::<") {
                g2 += 1;
            }
            let st = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            let argv = vec![
                "-d".to_owned(),
                "-C".into(),
                "--no-show-raw-insn".into(),
                format!("--start-address=0x{st:x}"),
                format!("--stop-address=0x{:x}", st + size),
                binary.into(),
            ];
            let o = Command::new("objdump").args(&argv).output().unwrap();
            assert!(o.status.success());
            let text = String::from_utf8(o.stdout).unwrap();
            let inst = text
                .lines()
                .filter_map(|l| {
                    let (a, b) = l.split_once(':')?;
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
            let sub = inst
                .iter()
                .filter_map(|l| {
                    let s = l.strip_prefix("sub")?.trim().strip_prefix("$0x")?;
                    let (v, reg) = s.split_once(',')?;
                    (reg.trim() == "%rsp")
                        .then(|| u64::from_str_radix(v, 16).ok())
                        .flatten()
                })
                .collect::<Vec<_>>();
            records.push(json!({"variant":label,"binary_sha256":hash(binary),"symbol":name,"size_bytes":size,"instructions":inst.len(),"ifma_sites":inst.iter().filter(|l|l.starts_with("vpmadd52")).count(),"rsp_operand_lines":inst.iter().filter(|l|l.contains("(%rsp)")).count(),"literal_rsp_subtractions":sub,"literal_rsp_subtraction_sum":sub.iter().sum::<u64>(),"calls":inst.iter().filter(|l|l.starts_with("call")).collect::<Vec<_>>(),"objdump_argv":argv}));
            asm.push_str(&format!("\n{label}:{name}\n{text}"));
        }
        assert_eq!(main, 1);
        assert_eq!(g2, 2);
    }
    fs::write(r.join("codegen.txt"), asm).unwrap();
    fs::write(r.join("codegen.json"),serde_json::to_vec_pretty(&json!({"symbols":records,"scope":"Actual complete-SDK canonical-wire caller and G2 prefix kernels; optional separately emitted fused helper. ON/OFF share one candidate executable. Static sites, caller sizes and RSP lines are not dynamic work or causal costs; literal prologue sum excludes pushes/alignment/callees."})).unwrap()).unwrap();
    let rows: Value = serde_json::from_slice(&fs::read(r.join("perf/rows.json")).unwrap()).unwrap();
    let mut env = Vec::new();
    for row in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|v| v["phase"] == "profile" && v["case"] == "1x2-w32")
    {
        let periods = row["profile"]["symbol_periods"].as_object().unwrap();
        let total = periods.values().map(|v| v.as_u64().unwrap()).sum::<u64>();
        let matches=periods.iter().filter(|(s,_)|s.contains("env")||s.contains("getenv")).map(|(s,p)|json!({"symbol":s,"period":p,"share_of_symbol_periods":p.as_u64().unwrap()as f64/total as f64})).collect::<Vec<_>>();
        env.push(json!({"variant":row["variant"],"environment_named_symbols":matches,"scope":"Existing captures only; inlining/unresolved symbols prevent an upper bound on environment/configuration cost."}));
    }
    fs::write(
        r.join("configuration-profile-notes.json"),
        serde_json::to_vec_pretty(&json!({"profiles":env,"new_captures":0,"new_experiment":false}))
            .unwrap(),
    )
    .unwrap();
    println!("Archived7actualSDKroutines andreadonlyexistingprofileconfigurationnotes");
}
