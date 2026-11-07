use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176/native";
fn p(s: &str) -> String {
    format!("{R}/{s}")
}
fn load(s: &str) -> Value {
    serde_json::from_slice(&fs::read(s).unwrap()).unwrap()
}
fn save(s: &str, v: &Value) {
    assert!(!Path::new(s).exists());
    fs::write(s, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(s: &str) -> String {
    let mut f = fs::File::open(s).unwrap();
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
fn command(name: &str, argv: Value) -> Value {
    json!({"kind":"command","name":name,"argv":argv,"timeout_seconds":120})
}
fn prepare() {
    let mut pins = load(&p("sdk-build-fixed-pins.json"));
    let mut steps = vec![];
    for arm in ["OFF", "ON"] {
        let a = load(&p(&format!("sdk-build/{arm}/artifacts.json")));
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (k, v) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(k), v.as_str().unwrap());
            pins[k] = v.clone();
        }
        let c = load(&p(&format!("sdk-build/{arm}/command.json")));
        let args = c["argv"].as_array().unwrap();
        assert!(args.iter().any(|v| v == "-O"));
        for opt in ["target-cpu=native", "lto=fat", "codegen-units=1"] {
            assert!(args.windows(2).any(|w| w[0] == "-C" && w[1] == opt));
        }
        steps.push(command(
            &format!("nm-{arm}"),
            json!(["/usr/bin/nm", "-S", "--defined-only", a["binary"]]),
        ));
    }
    for f in [
        p("codegen.rs"),
        p("codegen"),
        p("codegen-design.md"),
        p("native-review.json"),
        "/usr/bin/nm".into(),
        "/usr/bin/objdump".into(),
    ] {
        pins[&f] = hash(&f).into();
    }
    save(&p("codegen-pins.json"), &pins);
    save(
        &p("nm-plan.json"),
        &json!({"version":1,"source_pins":p("codegen-pins.json"),"steps":steps}),
    );
    println!("SDKidentities/optimizedlinkflags checked;2nm commands ready");
}
fn receipts(stage: &str) -> usize {
    let plan = load(&p(&format!("{stage}-plan.json")));
    assert_eq!(
        load(&p(&format!("{stage}-runs/result.json")))["status"],
        "complete"
    );
    for step in plan["steps"].as_array().unwrap() {
        let n = step["name"].as_str().unwrap();
        let base = p(&format!("{stage}-runs/{n}"));
        let e = load(&format!("{base}/exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        let c = load(&format!("{base}/command.json"));
        let a = c["argv"].as_array().unwrap();
        let b = step["argv"].as_array().unwrap();
        assert_eq!(&a[a.len() - b.len()..], b);
        for stream in ["stdout", "stderr"] {
            assert_eq!(
                hash(&format!("{base}/{stream}.log")),
                e[format!("{stream}_sha256")].as_str().unwrap()
            );
        }
    }
    plan["steps"].as_array().unwrap().len()
}
fn select() {
    assert_eq!(receipts("nm"), 2);
    let mut jobs = vec![];
    let mut steps = vec![];
    for arm in ["OFF", "ON"] {
        let text = fs::read_to_string(p(&format!("nm-runs/nm-{arm}/stdout.log"))).unwrap();
        let mut found = BTreeMap::<String, Value>::new();
        for l in text.lines() {
            let f: Vec<_> = l.split_whitespace().collect();
            if f.len() != 4 {
                continue;
            }
            let symbol = f[3];
            let name = format!("{:#}", rustc_demangle::demangle(symbol));
            let kind = if name
                == "zopro_core::msm::g2_borrowed::apply::<zopro_bn254::curve::G2Config, true>"
            {
                "G2"
            } else if name.starts_with(
                "zopro_core::msm::stream_g1_impl::<zopro_bn254::curve::G1Config, true, true, &",
            ) {
                "G1"
            } else {
                continue;
            };
            let address = u64::from_str_radix(f[0], 16).unwrap();
            let size = u64::from_str_radix(f[1], 16).unwrap();
            assert!(size > 1000);
            let job = json!({"arm":arm,"kind":kind,"symbol":symbol,"demangled":name,"address":address,"size_bytes":size,"binary":p(&format!("sdk-build/{arm}-sdk")),"name":format!("decode-{kind}-{arm}")});
            assert!(
                found.insert(kind.into(), job).is_none(),
                "multiple{kind}{arm}"
            );
        }
        assert_eq!(found.len(), 2, "missingselectedcaller {arm}");
        for job in found.into_values() {
            steps.push(command(
                job["name"].as_str().unwrap(),
                json!([
                    "/usr/bin/objdump",
                    "-d",
                    "-M",
                    "intel",
                    "--no-show-raw-insn",
                    format!("--disassemble={}", job["symbol"].as_str().unwrap()),
                    job["binary"]
                ]),
            ));
            jobs.push(job);
        }
    }

    let mut tree_index = 0usize;
    for arm in ["OFF", "ON"] {
        let text = fs::read_to_string(p(&format!("nm-runs/nm-{arm}/stdout.log"))).unwrap();
        for line in text.lines() {
            let f = line.split_whitespace().collect::<Vec<_>>();
            if f.len() != 4 || !matches!(f[2], "t" | "T") {
                continue;
            }
            let symbol = f[3];
            let name = format!("{:#}", rustc_demangle::demangle(symbol));
            if !name.contains("::borrowed_tree::") {
                continue;
            }
            assert_eq!(arm, "ON", "OFF contains live new tree code");
            if !(name.starts_with("zopro_core::msm_affine::g2_phi::borrowed_tree::")
                || name.starts_with("rayon::iter::plumbing::bridge_producer_consumer::helper::"))
            {
                continue;
            }
            let kind = format!("Tree{tree_index:03}");
            tree_index += 1;
            let job = json!({"arm":arm,"kind":kind,"symbol":symbol,"demangled":name,
                "address":u64::from_str_radix(f[0],16).unwrap(),"size_bytes":u64::from_str_radix(f[1],16).unwrap(),
                "binary":p(&format!("sdk-build/{arm}-sdk")),"name":format!("decode-{kind}-{arm}")});
            steps.push(command(
                job["name"].as_str().unwrap(),
                json!([
                    "/usr/bin/objdump",
                    "-d",
                    "-M",
                    "intel",
                    "--no-show-raw-insn",
                    format!("--disassemble={symbol}"),
                    job["binary"]
                ]),
            ));
            jobs.push(job);
        }
    }
    assert!(tree_index > 0, "missing actual tree consumers");
    save(&p("codegen-jobs.json"), &json!(jobs));
    save(
        &p("decode-plan.json"),
        &json!({"version":1,"source_pins":p("codegen-pins.json"),"steps":steps}),
    );
    println!("4exactdemangledcallersselected;no decode executed yet");
}
fn analyze() {
    assert_eq!(receipts("nm"), 2);
    assert_eq!(
        receipts("decode"),
        load(&p("codegen-jobs.json")).as_array().unwrap().len()
    );
    let jobs = load(&p("codegen-jobs.json"));
    let mut rows = BTreeMap::new();
    for j in jobs.as_array().unwrap() {
        let stream = p(&format!(
            "decode-runs/{}/stdout.log",
            j["name"].as_str().unwrap()
        ));
        let text = fs::read_to_string(&stream).unwrap();
        let start = j["address"].as_u64().unwrap();
        let end = start + j["size_bytes"].as_u64().unwrap();
        let mut ins = vec![];
        for l in text.lines() {
            if let Some((a, b)) = l.split_once(':') {
                if let Ok(ip) = u64::from_str_radix(a.trim(), 16) {
                    if ip >= start && ip < end {
                        ins.push(b.trim().to_string());
                    }
                }
            }
        }
        assert!(!ins.is_empty());
        let ifma = ins.iter().filter(|s| s.starts_with("vpmadd52")).count();
        let left4 = ins
            .iter()
            .filter(|s| s.starts_with("vpsllq") && s.ends_with(",0x4"))
            .count();
        let right48 = ins
            .iter()
            .filter(|s| s.starts_with("vpsrlq") && s.ends_with(",0x30"))
            .count();
        let calls: Vec<_> = ins
            .iter()
            .filter(|s| s.starts_with("call "))
            .cloned()
            .collect();
        let wrapper_calls = calls
            .iter()
            .filter(|s| {
                s.contains("sum2_div16")
                    || s.contains("difference2_div16")
                    || s.contains("g2_scaled")
            })
            .count();
        let stacks = ins
            .iter()
            .filter(|s| s.contains("ZMMWORD PTR [rsp") || s.contains("ZMMWORD PTR [rbp"))
            .count();
        let stores = ins
            .iter()
            .filter(|s| {
                s.split_once(',')
                    .map(|(dst, _)| {
                        dst.contains("ZMMWORD PTR [rsp") || dst.contains("ZMMWORD PTR [rbp")
                    })
                    .unwrap_or(false)
            })
            .count();
        let mut sub = 0u64;
        let mut push = 0;
        let mut prologue = vec![];
        let mut stack_probe = 0u64;
        let mut skip_probe_page = false;
        for l in &ins {
            if l.starts_with("vmov") {
                break;
            }
            prologue.push(l.clone());
            if let Some(v) = l.strip_prefix("sub    r11,0x") {
                stack_probe = u64::from_str_radix(v, 16).unwrap();
                sub += stack_probe;
                skip_probe_page = true;
            }
            if let Some(v) = l.strip_prefix("sub    rsp,0x") {
                let bytes = u64::from_str_radix(v, 16).unwrap();
                if skip_probe_page && bytes == 4096 {
                    skip_probe_page = false;
                } else {
                    sub += bytes;
                }
            }
            if l.starts_with("push ") {
                push += 1;
            }
        }
        let key = format!(
            "{}-{}",
            j["kind"].as_str().unwrap(),
            j["arm"].as_str().unwrap()
        );
        rows.insert(key,json!({"job":j,"instructions":ins.len(),"IFMA_sites":ifma,"left4_shifts":left4,"right48_shifts":right48,"calls":calls,"new_wrapper_calls":wrapper_calls,"ZMM_stack_operand_rows":stacks,"ZMM_stack_store_rows":stores,"prologue_rsp_subtraction_bytes":sub,"stack_probe_bytes":stack_probe,"pushed_registers":push,"prologue":prologue,"decode_sha256":hash(&stream),"binary_sha256":hash(j["binary"].as_str().unwrap())}));
    }
    let off = &rows["G2-OFF"];
    let on = &rows["G2-ON"];
    let tree = rows
        .iter()
        .filter(|(k, _)| k.starts_with("Tree"))
        .collect::<Vec<_>>();
    let gate = !tree.is_empty()
        && tree
            .iter()
            .all(|(_, v)| v["prologue_rsp_subtraction_bytes"].as_u64().unwrap() < 512 * 1024);
    let out = json!({"codegen_review_passed":true,"native_structure_gate_passed":gate,"rows":rows,"native_correctness_inherited":load(&p("native-review.json"))["review_passed"],"new_timings":0,"new_proofs":0,"adopted":false,"limits":"Static final SDK kernels and selected direct/recursive tree consumers. Individual frames below512KiB are necessary, not a formal complete-stack bound. Prologues exclude alignment/pushes/callees; no speed or dynamic spill inference."});
    save(&p("codegen-analysis.json"), &out);
    println!(
        "{}",
        json!({"structure_gate_passed":gate,"G2_OFF":off,"G2_ON":on})
    );
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("select") => select(),
        Some("analyze") => analyze(),
        _ => panic!("prepare|select|analyze"),
    }
}
