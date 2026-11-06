use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const C: &str = "/home/ubuntu/src/zopro-perf-work/joint-pair52-capture176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/joint-pair52-callers176";
const R: &str = "/home/ubuntu/src/zopro-perf-work/joint-pair52-postmortem176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn pin(p: &str, pins: &mut BTreeMap<String, String>) {
    pins.insert(p.into(), hash(p));
}
fn u16le(b: &[u8], n: usize) -> usize {
    u16::from_le_bytes(b[n..n + 2].try_into().unwrap()) as usize
}
fn u32le(b: &[u8], n: usize) -> u32 {
    u32::from_le_bytes(b[n..n + 4].try_into().unwrap())
}
fn u64le(b: &[u8], n: usize) -> u64 {
    u64::from_le_bytes(b[n..n + 8].try_into().unwrap())
}
#[derive(Clone, Copy)]
struct Load {
    off: u64,
    va: u64,
    bytes: u64,
}
fn loads(b: &[u8]) -> Vec<Load> {
    assert_eq!(&b[..6], b"\x7fELF\x02\x01");
    let ph = u64le(b, 32) as usize;
    let stride = u16le(b, 54);
    let count = u16le(b, 56);
    let mut out = vec![];
    for i in 0..count {
        let n = ph + i * stride;
        if u32le(b, n) == 1 && u32le(b, n + 4) & 1 != 0 {
            out.push(Load {
                off: u64le(b, n + 8),
                va: u64le(b, n + 16),
                bytes: u64le(b, n + 32),
            });
        }
    }
    assert!(!out.is_empty());
    out
}
fn hex(t: &str) -> u64 {
    u64::from_str_radix(t.trim_start_matches("0x"), 16).unwrap()
}
struct Function {
    kind: String,
    start: u64,
    size: u64,
    inst: BTreeMap<u64, String>,
}
fn category(s: &str) -> String {
    let (m, args) = s.split_once(char::is_whitespace).unwrap_or((s, ""));
    if m.starts_with("vpmadd52") {
        return "IFMA".into();
    }
    if m.starts_with('v') && (args.contains("[rsp") || args.contains("[rbp")) {
        return if args.trim_start().starts_with(['Z', 'Y', 'X']) {
            "vector_stack_store"
        } else {
            "vector_stack_read_operand"
        }
        .into();
    }
    if m == "vpaddq" || m == "vpsubq" {
        return "vector_add_sub".into();
    }
    if m.starts_with("vpsr") || m.starts_with("vpsl") {
        return "vector_shift".into();
    }
    if m.starts_with('v') && (m.contains("perm") || m.contains("shuf") || m.contains("unpck")) {
        return "vector_permute".into();
    }
    if m.starts_with('v') {
        return "other_vector".into();
    }
    if m.starts_with('j') || m.starts_with("call") || m.starts_with("ret") {
        return "branch_call_return".into();
    }
    "other_scalar".into()
}
fn main() {
    assert!(!Path::new(R).exists());
    fs::create_dir(R).unwrap();
    let reviewed = read(format!("{C}/review.json"));
    assert_eq!(reviewed["review_passed"], true);
    let codegen = read(format!("{Q}/codegen.json"));
    assert_eq!(codegen["status"], "pass");
    let identities = read(format!("{C}/identities.json"));
    let streams=read(format!("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-joint-pair52-capture176/artifacts/profile-stream-index.json"));
    let mut pins = BTreeMap::new();
    for p in [
        format!("{C}/review.json"),
        format!("{Q}/codegen.json"),
        format!("{C}/identities.json"),
        format!("{C}/postmortem.rs"),
        format!("{C}/postmortem"),
    ] {
        pin(&p, &mut pins);
    }
    let mut functions = BTreeMap::<String, Vec<Function>>::new();
    let mut static_rows = vec![];
    for row in codegen["rows"].as_array().unwrap() {
        let role = row["role"].as_str().unwrap();
        let kind = row["kind"].as_str().unwrap();
        let path = format!("{Q}/disassembly/{role}-{kind}.txt");
        assert_eq!(hash(&path), row["disassembly_sha256"]);
        pin(&path, &mut pins);
        let start = row["start"].as_u64().unwrap();
        let size = row["size_bytes"].as_u64().unwrap();
        let mut inst = BTreeMap::new();
        let mut cats = BTreeMap::<String, u64>::new();
        for line in fs::read_to_string(path).unwrap().lines() {
            if let Some((a, s)) = line.trim().split_once(':') {
                if let Ok(a) = u64::from_str_radix(a, 16) {
                    let s = s.trim();
                    if a >= start && a < start + size && !s.is_empty() {
                        assert!(inst.insert(a, s.into()).is_none());
                        *cats.entry(category(s)).or_default() += 1;
                    }
                }
            }
        }
        assert_eq!(inst.len() as u64, row["instruction_rows"].as_u64().unwrap());
        assert_eq!(cats["IFMA"], row["ifma"].as_u64().unwrap());
        static_rows.push(json!({"role":role,"kind":kind,"rows":inst.len(),"categories":cats}));
        functions.entry(role.into()).or_default().push(Function {
            kind: kind.into(),
            start,
            size,
            inst,
        });
    }
    let mut panels = vec![];
    let mut total = 0u64;
    for (kind, size, workers) in [
        ("G1", 200, 1),
        ("G1", 200, 32),
        ("G1", 352, 32),
        ("G2", 200, 1),
        ("G2", 200, 32),
        ("G2", 352, 32),
    ] {
        for arm in ["frozen", "OFF", "ON"] {
            let role = if arm == "frozen" { "FROZEN" } else { arm };
            let binary = identities[arm]["binary"].as_str().unwrap();
            assert_eq!(hash(binary), identities[arm]["binary_sha256"]);
            pin(binary, &mut pins);
            let l = loads(&fs::read(binary).unwrap());
            let name = format!("cycles-{kind}-{size}-w{workers}-{arm}");
            let meta = format!("{C}/runs/metadata-{name}/stdout.log");
            let samples = format!("{C}/runs/decode-{name}/stdout.log");
            for p in [&meta, &samples] {
                let rel = p.strip_prefix(&format!("{C}/")).unwrap();
                assert_eq!(hash(p), streams[rel]["sha256"]);
                pin(p, &mut pins);
            }
            let mut mappings = vec![];
            for line in fs::read_to_string(&meta).unwrap().lines().filter(|s| {
                s.contains("PERF_RECORD_MMAP2") && s.contains("r-xp") && s.ends_with(binary)
            }) {
                let tail = line.split_once(": [").unwrap().1;
                let (base, tail) = tail.split_once('(').unwrap();
                let (length, tail) = tail.split_once(')').unwrap();
                let off = tail
                    .split_once('@')
                    .unwrap()
                    .1
                    .split_whitespace()
                    .next()
                    .unwrap();
                mappings.push((hex(base), hex(length), hex(off)));
            }
            assert_eq!(mappings.len(), 1);
            let (map_start, map_len, map_off) = mappings[0];
            let mut count = 0;
            let mut matched = 0;
            let mut named_kernel = 0;
            let mut categories = BTreeMap::<String, u64>::new();
            let mut mnemonics = BTreeMap::<String, u64>::new();
            let mut blocks = BTreeMap::<(String, u64), u64>::new();
            let mut addresses = BTreeMap::<(String, u64, String), u64>::new();
            let mut other = BTreeMap::<String, u64>::new();
            let mut delta = BTreeMap::<String, u64>::new();
            for line in fs::read_to_string(samples)
                .unwrap()
                .lines()
                .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
            {
                let f = line.split_whitespace().collect::<Vec<_>>();
                assert_eq!(f[3], "16000000");
                let ip = hex(f[4]);
                let raw = f[5..].join(" ");
                let sym = rustc_demangle::try_demangle(&raw)
                    .map(|s| format!("{s:#}"))
                    .unwrap_or(raw);
                count += 1;
                let normalized = if ip >= map_start && ip < map_start + map_len {
                    let file = map_off + (ip - map_start);
                    l.iter()
                        .find(|s| file >= s.off && file < s.off + s.bytes)
                        .map(|s| {
                            let address = s.va + (file - s.off);
                            let bias = map_start as i128
                                - (s.va as i128 + map_off as i128 - s.off as i128);
                            assert_eq!(ip as i128 - bias, address as i128);
                            *delta.entry(format!("{:#x}", s.va - s.off)).or_default() += 1;
                            address
                        })
                } else {
                    None
                };
                let found = normalized.and_then(|a| {
                    functions[role]
                        .iter()
                        .find(|x| a >= x.start && a < x.start + x.size)
                        .map(|x| (x, a))
                });
                let is_named = sym.contains("msm::stream_g1_impl")
                    || sym.contains("msm::g2_borrowed::apply")
                    || sym.contains("inverse_nonzero");
                if is_named {
                    named_kernel += 1;
                    assert!(
                        found.is_some(),
                        "symbol outside normalized consumer {name} {sym} {ip:x}"
                    );
                }
                if let Some((fun, a)) = found {
                    let s = fun
                        .inst
                        .get(&a)
                        .unwrap_or_else(|| panic!("non-instruction IP {name} {a:x}"));
                    assert!(is_named, "unexpected symbol inside consumer {name} {sym}");
                    let expected = if fun.kind == "G1" {
                        sym.contains("msm::stream_g1_impl")
                    } else if fun.kind == "G2" {
                        sym.contains("msm::g2_borrowed::apply")
                    } else {
                        sym.contains("inverse_nonzero")
                    };
                    assert!(expected);
                    matched += 1;
                    *categories.entry(category(s)).or_default() += 1;
                    *mnemonics
                        .entry(s.split_whitespace().next().unwrap().into())
                        .or_default() += 1;
                    *blocks
                        .entry((fun.kind.clone(), (a - fun.start) / 256 * 256))
                        .or_default() += 1;
                    *addresses
                        .entry((fun.kind.clone(), a - fun.start, s.clone()))
                        .or_default() += 1;
                } else {
                    *other.entry(sym).or_default() += 1;
                }
            }
            let expected = reviewed["profiles"][&name]["samples"].as_u64().unwrap();
            assert_eq!(count, expected);
            assert_eq!(matched, named_kernel);
            let mut hot = blocks.iter().collect::<Vec<_>>();
            hot.sort_by_key(|(_, n)| std::cmp::Reverse(**n));
            let top=hot.into_iter().take(12).map(|((kind,off),n)|json!({"function":kind,"offset_hex":format!("{off:#x}"),"window_bytes":256,"samples":n,"share_all":*n as f64/count as f64})).collect::<Vec<_>>();
            let mut hottest = addresses.into_iter().collect::<Vec<_>>();
            hottest.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            let ips=hottest.into_iter().take(20).map(|((kind,off,asm),n)|json!({"function":kind,"offset_hex":format!("{off:#x}"),"instruction":asm,"samples":n})).collect::<Vec<_>>();
            panels.push(json!({"profile":name,"samples":count,"normalized_instruction_samples":matched,"named_core_samples":named_kernel,"map_start_hex":format!("{map_start:#x}"),"map_offset_hex":format!("{map_off:#x}"),"ELF_vaddr_minus_offset":delta,"sampled_instruction_categories":categories,"sampled_mnemonics":mnemonics,"top_256_byte_windows":top,"top_instruction_addresses":ips,"other_function_samples":other}));
            total += count;
        }
    }
    assert_eq!(total, reviewed["total_profile_samples"].as_u64().unwrap());
    assert_eq!(panels.len(), 18);
    let result = json!({"status":"pass","panels":panels,"samples":total,"static_function_categories":static_rows,"new_capture":false,"new_decode":false,"new_proofs":0,"production_changed":false,"adopted":false,"limits":"Exploratory saved-data postmortem. Flat cycle samples have skid and do not establish exclusive instruction latency, dependency stalls, spill costs or causal memory hierarchy. All18 panels retained; no clock-selected subset."});
    fs::write(
        format!("{R}/analysis.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    fs::write(
        format!("{R}/pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    println!("PASS:18 saved panels/{total} samples; exact ELF/MMAP instruction mapping and counts reconciled; no recording or decode repeated.");
}
