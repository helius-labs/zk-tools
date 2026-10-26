use serde_json::{json, Value};
use std::{fs, path::Path, process::Command};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn main() {
    let root = Path::new("/tmp/zopro-deferred-dif32-sdk");
    let mut records = Vec::new();
    let mut assembly = String::new();
    for (label, path) in [
        (
            "accepted",
            "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
        ),
        (
            "candidate",
            "/tmp/zopro-deferred-dif32-sdk/native/candidate/artifacts.json",
        ),
    ] {
        let a = read(path);
        let bin = a["binary"].as_str().unwrap();
        let out = Command::new("nm").args(["-S", "-C", bin]).output().unwrap();
        assert!(out.status.success());
        let text = String::from_utf8(out.stdout).unwrap();
        let matches = text
            .lines()
            .filter(|s| {
                s.contains("zopro_core::fft::pair::stored_column_leaf::<")
                    && s.contains("PackedIfma8<")
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 2);
        for line in matches {
            let f = line.split_whitespace().collect::<Vec<_>>();
            let symbol = f[3..].join(" ");
            let start = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            let argv = vec![
                "-d".to_owned(),
                "-C".into(),
                format!("--start-address=0x{start:x}"),
                format!("--stop-address=0x{:x}", start + size),
                bin.into(),
            ];
            let out = Command::new("objdump").args(&argv).output().unwrap();
            assert!(out.status.success());
            let body = String::from_utf8(out.stdout).unwrap();
            let lines = body
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
            records.push(json!({"build":label,"binary_sha256":a["binary_sha256"],"symbol":symbol,"size_bytes":size,"instructions":lines.len(),"ifma_sites":lines.iter().filter(|l|l.contains("vpmadd52")).count(),"rsp_operand_lines":lines.iter().filter(|l|l.contains("(%rsp)")).count(),"literal_rsp_subtractions":lines.iter().filter(|l|l.contains("sub")&&l.contains("%rsp")&&l.contains('$')).map(|l|l.to_string()).collect::<Vec<_>>(),"calls":calls,"objdump_argv":argv}));
            assembly.push_str(&format!("\n{label}: {symbol}\n{body}"));
        }
    }
    let rows = read(root.join("perf/rows.json"));
    let mut profiles = Vec::new();
    for r in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["phase"] == "profile")
    {
        let p = &r["profile"];
        let period = p["period"].as_f64().unwrap();
        let mut symbols = Vec::new();
        for (s, v) in p["symbol_periods"].as_object().unwrap() {
            if s.contains("stored_column_leaf") {
                symbols.push(
                    json!({"symbol":s,"period":v,"share_percent":v.as_f64().unwrap()/period*100.}),
                );
            }
        }
        profiles.push(json!({"case":r["case"],"variant":r["variant"],"cpu_ms_per_request":(r["summary"]["user_cpu_ms"].as_f64().unwrap()+r["summary"]["system_cpu_ms"].as_f64().unwrap())/r["samples_ms"].as_array().unwrap().len()as f64,"fft_sample_percent":p["category_percent"]["fft"],"stored_column_symbols":symbols}));
    }
    fs::write(root.join("codegen.txt"), assembly).unwrap();
    fs::write(root.join("codegen.json"),serde_json::to_vec_pretty(&json!({"symbols":records,"scope":"Actual linked packed stored-column wrappers, including scalar/header work and inlined branches. Candidate DIF wrapper retains OFF and ON and cached/uncached alternatives, so static IFMA sites are not executed counts. Stack-reference lines are not spill-cost or whole-stack bounds."})).unwrap()).unwrap();
    fs::write(root.join("profile-analysis.json"),serde_json::to_vec_pretty(&json!({"profiles":profiles,"scope":"One capture per declared tuple; user-cycle attribution and CPU means are not removable wall-time fractions or a causal decomposition. Euler path has one packed inverse and one packed forward transform, plus separate weighting/product passes; short expanded leaves do not call this changed column routine."})).unwrap()).unwrap();
    println!("Audited4actualSDK column wrappers and9declaredprofile observations");
}
