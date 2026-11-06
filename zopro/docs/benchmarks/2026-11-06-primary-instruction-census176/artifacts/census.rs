use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/primary-instruction-census176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const B: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/native/candidate-sdk";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn h(b: impl AsRef<[u8]>) -> String {
    format!("{:x}", Sha256::digest(b.as_ref()))
}
fn hash(p: impl AsRef<Path>) -> String {
    h(fs::read(p).unwrap())
}
fn save(p: &str, v: &Value) {
    fs::write(r(p), serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn u16le(b: &[u8], p: usize) -> u16 {
    u16::from_le_bytes(b[p..p + 2].try_into().unwrap())
}
fn u32le(b: &[u8], p: usize) -> u32 {
    u32::from_le_bytes(b[p..p + 4].try_into().unwrap())
}
fn u64le(b: &[u8], p: usize) -> u64 {
    u64::from_le_bytes(b[p..p + 8].try_into().unwrap())
}
fn hex(s: &str) -> u64 {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}
#[derive(Clone)]
struct Segment {
    off: u64,
    vaddr: u64,
    bytes: u64,
    mem: u64,
    flags: u32,
}
fn segments(b: &[u8]) -> Vec<Segment> {
    assert_eq!(&b[..6], b"\x7fELF\x02\x01");
    assert_eq!(u16le(b, 16), 3);
    assert_eq!(u16le(b, 18), 62);
    let start = u64le(b, 32) as usize;
    let size = u16le(b, 54) as usize;
    let count = u16le(b, 56) as usize;
    let mut out = vec![];
    for i in 0..count {
        let p = start + i * size;
        if u32le(b, p) == 1 {
            assert_eq!(u64le(b, p + 48), 4096);
            out.push(Segment {
                off: u64le(b, p + 8),
                vaddr: u64le(b, p + 16),
                bytes: u64le(b, p + 32),
                mem: u64le(b, p + 40),
                flags: u32le(b, p + 4),
            });
        }
    }
    out
}
fn mapping(text: &str, segs: &[Segment], pid: u32) -> Value {
    let mut maps = BTreeSet::new();
    for l in text
        .lines()
        .filter(|l| l.contains("PERF_RECORD_MMAP2") && l.contains("r-xp") && l.ends_with(B))
    {
        assert_eq!(
            l.split_whitespace()
                .next()
                .unwrap()
                .split_once('/')
                .unwrap()
                .0
                .parse::<u32>()
                .unwrap(),
            pid
        );
        let tail = l.split_once(": [").unwrap().1;
        let (start, tail) = tail.split_once('(').unwrap();
        let (len, tail) = tail.split_once(')').unwrap();
        let off = tail
            .split_once(" @ ")
            .unwrap()
            .1
            .split_whitespace()
            .next()
            .unwrap();
        maps.insert((hex(start), hex(len), hex(off)));
    }
    assert_eq!(maps.len(), 1);
    let (start, len, off) = *maps.first().unwrap();
    let s = segs
        .iter()
        .filter(|s| {
            s.flags & 1 != 0
                && off >= s.off / 4096 * 4096
                && off < (s.off + s.bytes).div_ceil(4096) * 4096
        })
        .collect::<Vec<_>>();
    assert_eq!(s.len(), 1);
    let seg = s[0];
    let vpage = seg.vaddr / 4096 * 4096 + (off - seg.off / 4096 * 4096);
    let bias = start.checked_sub(vpage).unwrap();
    json!({"runtime_start":start,"runtime_bytes":len,"file_page_offset":off,"ELF_virtual_page":vpage,"ELF_load_bias":bias,"file_and_virtual_pages_differ":off!=vpage})
}
#[derive(Clone)]
struct Sample {
    tid: u32,
    cpu: u32,
    ip: u64,
    virt: Option<u64>,
    symbol: String,
}
struct Profile {
    label: String,
    mapping: Value,
    samples: Vec<Sample>,
    pid: u32,
}
fn profile(w: usize, segs: &[Segment], producer: &Value) -> Profile {
    let name = format!("cycles-1x2-w{w}-accepted");
    let p = &producer["profiles"][&name];
    assert_eq!(p["qualified"], true);
    assert_eq!(p["period16M"], true);
    assert_eq!(p["zero_lost"], true);
    let loss = read(format!("{Q}/runs/loss-{name}/stdout.log"));
    assert_eq!(loss["zero_lost"], true);
    assert!(loss["records"].as_array().unwrap().is_empty());
    let pid = p["actual_pid"].as_u64().unwrap() as u32;
    let tids = p["tids"]
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_u64().unwrap() as u32)
        .collect::<BTreeSet<_>>();
    let meta = fs::read_to_string(format!("{Q}/runs/metadata-{name}/stdout.log")).unwrap();
    let m = mapping(&meta, segs, pid);
    let lo = m["runtime_start"].as_u64().unwrap();
    let hi = lo + m["runtime_bytes"].as_u64().unwrap();
    let bias = m["ELF_load_bias"].as_u64().unwrap();
    let mut samples = vec![];
    let mut unknown = 0;
    for line in fs::read_to_string(format!("{Q}/runs/decode-{name}/stdout.log"))
        .unwrap()
        .lines()
    {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') {
            continue;
        }
        let f = l.split_whitespace().collect::<Vec<_>>();
        assert!(f.len() >= 6);
        let (pp, tt) = f[0].split_once('/').unwrap();
        assert_eq!(pp.parse::<u32>().unwrap(), pid);
        let tid = tt.parse::<u32>().unwrap();
        assert!(tids.contains(&tid));
        let cpu = f[1].trim_matches(['[', ']']).parse::<u32>().unwrap();
        assert!(cpu < (w as u32));
        let time = f[2].trim_end_matches(':').parse::<f64>().unwrap();
        assert!(time.is_finite() && time > 0.);
        assert_eq!(f[3].parse::<u64>().unwrap(), 16000000);
        let ip = hex(f[4]);
        let symbol = f[5..].join(" ");
        if symbol.starts_with("[unknown]") {
            unknown += 1;
        }
        let virt = if lo <= ip && ip < hi {
            let v = ip - bias;
            assert!(segs
                .iter()
                .any(|s| s.flags & 1 != 0 && v >= s.vaddr && v < s.vaddr + s.mem));
            Some(v)
        } else {
            None
        };
        samples.push(Sample {
            tid,
            cpu,
            ip,
            virt,
            symbol,
        });
    }
    assert_eq!(samples.len() as u64, p["samples"].as_u64().unwrap());
    assert!(
        (unknown as f64 / samples.len() as f64 - p["unknown_share"].as_f64().unwrap()).abs()
            < 1e-12
    );
    Profile {
        label: format!("w{w}"),
        mapping: m,
        samples,
        pid,
    }
}
#[derive(Clone)]
struct Symbol {
    raw: String,
    name: String,
    pc: u64,
    size: u64,
}
fn command(name: &str, argv: &[String]) -> String {
    let out = Command::new(&argv[0]).args(&argv[1..]).output().unwrap();
    fs::write(r(&format!("{name}.stdout")), &out.stdout).unwrap();
    fs::write(r(&format!("{name}.stderr")), &out.stderr).unwrap();
    save(
        &format!("{name}.command.json"),
        &json!({"argv":argv,"success":out.status.success(),"returncode":out.status.code(),"stdout_sha256":h(&out.stdout),"stderr_sha256":h(&out.stderr)}),
    );
    assert!(out.status.success());
    String::from_utf8(out.stdout).unwrap()
}
fn class(m: &str, ops: &str) -> String {
    let mem = ops.contains('(');
    let rsp = mem && ops.contains("%rsp");
    let suffix = if rsp {
        "RSP_memory"
    } else if mem {
        "other_memory"
    } else {
        "register_or_immediate"
    };
    let family = if m.starts_with("vpmadd52") {
        "IFMA"
    } else if m.starts_with("vmov") {
        "vector_move"
    } else if m.starts_with('v')
        && (m.contains("perm") || m.contains("shuf") || m.contains("unpck") || m.contains("align"))
    {
        "vector_shuffle"
    } else if m.starts_with('v') {
        "vector_other"
    } else if m.starts_with('j') || m.starts_with("call") || m.starts_with("ret") {
        return "control".into();
    } else if m.starts_with("nop") {
        return "nop".into();
    } else if m.starts_with("lea") {
        return "scalar_address".into();
    } else if m.starts_with("mov") {
        "scalar_move"
    } else {
        "scalar_other"
    };
    format!("{family}_{suffix}")
}
fn prepare() {
    assert!(!r("pins.json").exists());
    let mut pins = BTreeMap::new();
    let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    for (p, sha) in read(&manifest)["source_files_sha256"].as_object().unwrap() {
        let p = format!("{Z}/{p}");
        assert_eq!(hash(&p), sha.as_str().unwrap());
        pins.insert(p, sha.as_str().unwrap().to_string());
    }
    assert_eq!(pins.len(), 176);
    let mut files = vec![
        manifest,
        B.into(),
        format!("{Q}/analysis.json"),
        format!("{Q}/review.json"),
        format!("{Q}/identities.json"),
        format!("{R}/census.rs"),
        format!("{R}/census"),
        format!("{R}/design.md"),
    ];
    for w in [16, 32] {
        let n = format!("cycles-1x2-w{w}-accepted");
        files.push(format!("{Q}/perf/{n}.data"));
        for kind in ["decode", "metadata", "events", "loss"] {
            for f in ["stdout.log", "stderr.log", "exit.json", "command.json"] {
                files.push(format!("{Q}/runs/{kind}-{n}/{f}"));
            }
        }
    }
    for file in files {
        pins.insert(file.clone(), hash(file));
    }
    save("pins.json", &json!(pins));
    save(
        "plan.json",
        &json!({"version":1,"source_pins":r("pins.json"),"steps":[{"kind":"command","name":"accepted-instruction-census","argv":[r("census"),"analyze"],"timeout_seconds":180}]}),
    );
    println!(
        "Prepared{}pins,2savedacceptedprofiles,176production files; no new capture",
        pins.len()
    );
}
fn analyze() {
    assert!(!r("analysis.json").exists());
    let pins = read(r("pins.json"));
    for (p, sha) in pins.as_object().unwrap() {
        assert_eq!(hash(p), sha.as_str().unwrap());
    }
    assert_eq!(
        hash(B),
        "6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661"
    );
    assert_eq!(read(format!("{Q}/review.json"))["passed"], true);
    let producer = read(format!("{Q}/analysis.json"));
    assert_eq!(producer["diagnostic_qualified"], false);
    let segs = segments(&fs::read(B).unwrap());
    let profiles = [profile(16, &segs, &producer), profile(32, &segs, &producer)];
    let n = command(
        "nm",
        &[
            "nm".into(),
            "-S".into(),
            "--size-sort".into(),
            "--defined-only".into(),
            B.into(),
        ],
    );
    let mut syms = BTreeMap::new();
    for l in n.lines() {
        let f = l.split_whitespace().collect::<Vec<_>>();
        if f.len() != 4 || !matches!(f[2], "t" | "T") {
            continue;
        }
        let pc = hex(f[0]);
        let size = hex(f[1]);
        if size == 0 {
            continue;
        }
        let raw = f[3].to_string();
        let name = rustc_demangle::try_demangle(&raw)
            .map(|d| format!("{d:#}"))
            .unwrap_or(raw.clone());
        assert!(syms
            .insert(
                raw.clone(),
                Symbol {
                    raw,
                    name,
                    pc,
                    size
                }
            )
            .is_none());
    }
    let mut counts = BTreeMap::<String, usize>::new();
    let mut profile_coverage = BTreeMap::new();
    for p in &profiles {
        let mut matched = 0;
        let mut inside = 0;
        for s in &p.samples {
            if let Some(pc) = s.virt {
                inside += 1;
                if let Some(sym) = syms.get(&s.symbol) {
                    assert!(
                        sym.pc <= pc && pc < sym.pc + sym.size,
                        "symbol range mismatch{} {:x} {} {:x}",
                        p.label,
                        pc,
                        s.symbol,
                        sym.pc
                    );
                    *counts.entry(sym.raw.clone()).or_default() += 1;
                    matched += 1;
                }
            }
        }
        profile_coverage.insert(p.label.clone(),json!({"pid":p.pid,"mapping":p.mapping,"total_samples":p.samples.len(),"own_executable_samples":inside,"raw_symbol_range_matched_samples":matched,"outside_executable_samples":p.samples.len()-inside,"in_executable_no_exact_defined_symbol":inside-matched}));
    }
    let mut ranked = counts.into_iter().collect::<Vec<_>>();
    ranked.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    assert!(ranked.len() >= 12);
    let selected = ranked[..12].to_vec();
    let mut functions = vec![];
    let mut all_class = BTreeMap::<String, BTreeMap<String, usize>>::new();
    let mut selected_samples = BTreeMap::<String, usize>::new();
    let mut exact_total = 0;
    let mut unresolved_total = 0;
    for (index, (raw, total)) in selected.iter().enumerate() {
        let sym = &syms[raw];
        let stem = format!("symbol-{index:02}");
        let asm = command(
            &stem,
            &[
                "objdump".into(),
                "-d".into(),
                "--no-show-raw-insn".into(),
                format!("--disassemble={raw}"),
                B.into(),
            ],
        );
        let mut instructions = BTreeMap::new();
        let mut static_classes = BTreeMap::<String, usize>::new();
        for l in asm.lines() {
            let Some((address, text)) = l.trim().split_once(':') else {
                continue;
            };
            let Ok(pc) = u64::from_str_radix(address, 16) else {
                continue;
            };
            assert!(sym.pc <= pc && pc < sym.pc + sym.size);
            let text = text.trim();
            let (m, ops) = text.split_once(char::is_whitespace).unwrap_or((text, ""));
            let ops = ops.split_once('#').map_or(ops, |v| v.0).trim();
            let c = class(m, ops);
            *static_classes.entry(c.clone()).or_default() += 1;
            assert!(instructions
                .insert(
                    pc,
                    json!({"offset":pc-sym.pc,"mnemonic":m,"operands":ops,"class":c})
                )
                .is_none());
        }
        assert!(!instructions.is_empty());
        let mut sampled = BTreeMap::<u64, Value>::new();
        let mut per_profile = BTreeMap::new();
        let mut total_check = 0;
        for p in &profiles {
            let mut bins = BTreeMap::<String, usize>::new();
            let mut hits = 0;
            let mut nonstarts = vec![];
            let mut tids = BTreeMap::<u32, usize>::new();
            for s in p
                .samples
                .iter()
                .filter(|s| s.symbol == *raw && s.virt.is_some())
            {
                let pc = s.virt.unwrap();
                assert!(sym.pc <= pc && pc < sym.pc + sym.size);
                hits += 1;
                *tids.entry(s.tid).or_default() += 1;
                if let Some(i) = instructions.get(&pc) {
                    let c = i["class"].as_str().unwrap().to_string();
                    *bins.entry(c.clone()).or_default() += 1;
                    *all_class
                        .entry(p.label.clone())
                        .or_default()
                        .entry(c)
                        .or_default() += 1;
                    let row = sampled
                        .entry(pc)
                        .or_insert_with(|| json!({"ELF_pc":pc,"instruction":i,"samples":{}}));
                    let prev = row["samples"][&p.label].as_u64().unwrap_or(0);
                    row["samples"][&p.label] = json!(prev + 1);
                    exact_total += 1;
                } else {
                    nonstarts.push(json!({"runtime_ip":s.ip,"ELF_pc":pc,"tid":s.tid,"cpu":s.cpu}));
                    unresolved_total += 1;
                }
            }
            let sum: usize = bins.values().sum();
            assert_eq!(sum + nonstarts.len(), hits);
            *selected_samples.entry(p.label.clone()).or_default() += hits;
            total_check += hits;
            per_profile.insert(p.label.clone(),json!({"selected_symbol_samples":hits,"exact_instruction_starts":sum,"unresolved_or_interior_IPs":nonstarts,"classes":bins,"TID_samples":tids}));
        }
        assert_eq!(total_check, *total);
        let mut rows = sampled.into_values().collect::<Vec<_>>();
        rows.sort_by(|a, b| {
            let sum = |v: &Value| {
                v["samples"]
                    .as_object()
                    .unwrap()
                    .values()
                    .map(|v| v.as_u64().unwrap())
                    .sum::<u64>()
            };
            sum(b)
                .cmp(&sum(a))
                .then(a["ELF_pc"].as_u64().cmp(&b["ELF_pc"].as_u64()))
        });
        functions.push(json!({"rank":index+1,"raw_symbol":sym.raw,"symbol":sym.name,"ELF_start":sym.pc,"bytes":sym.size,"instruction_sites":instructions.len(),"static_classes":static_classes,"profile_counts":per_profile,"all_sampled_instructions":rows,"objdump_stem":stem,"combined_samples":total}));
    }
    for p in &profiles {
        let selected = selected_samples[&p.label];
        profile_coverage.get_mut(&p.label).unwrap()["top12_samples"] = json!(selected);
        profile_coverage.get_mut(&p.label).unwrap()["all_other_samples"] =
            json!(p.samples.len() - selected);
        profile_coverage.get_mut(&p.label).unwrap()["top12_classes"] = json!(all_class[&p.label]);
        assert_eq!(
            all_class[&p.label].values().sum::<usize>()
                + functions
                    .iter()
                    .map(
                        |f| f["profile_counts"][&p.label]["unresolved_or_interior_IPs"]
                            .as_array()
                            .unwrap()
                            .len()
                    )
                    .sum::<usize>(),
            selected
        );
    }
    let result = json!({"scope":"read-only instruction census of saved accepted profiles; instruction samples are not dynamic instruction counts, stalls, spills, causal costs or removable wall time","binary":B,"binary_sha256":hash(B),"pins_verified":pins.as_object().unwrap().len(),"production_files_unchanged":176,"profile_coverage":profile_coverage,"selected_functions":functions,"exact_selected_instruction_samples":exact_total,"unresolved_selected_IPs":unresolved_total,"selection":"12 defined executable functions ranked by combined counts in two saved accepted profiles","producer_combined_observer_qualification_remains_failed":true,"new_recordings":0,"new_native_tests_or_proofs":0,"goal_status":"active/unmet0/8"});
    save("analysis.json", &result);
    println!("Census PASS{}pins/176originals/2profiles/12functions/{}exactselectedsamples/{}unresolved; no new recording",pins.as_object().unwrap().len(),exact_total,unresolved_total);
}
fn main() {
    match std::env::args().nth(1).unwrap().as_str() {
        "prepare" => prepare(),
        "analyze" => analyze(),
        _ => panic!(),
    }
}
