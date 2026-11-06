use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};
const C: &str = "/home/ubuntu/src/zopro-perf-work/g2-validated-capture176";
const Q: &str = "/home/ubuntu/src/zopro-perf-work/g2-validated-batch176";
const R: &str = "/home/ubuntu/src/zopro-perf-work/accepted-dependencies176";
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
    blocks: BTreeMap<u64, Value>,
}
// This intentionally measures only connected chains of calibrated register-only
// instructions. Unknown writers terminate provenance; there are no guessed
// latency, memory-alias or inter-block edges and no scheduling/throughput model.
fn vector_reg(s: &str) -> Option<usize> {
    let s = s.trim();
    let t = s
        .strip_prefix("zmm")
        .or_else(|| s.strip_prefix("ymm"))
        .or_else(|| s.strip_prefix("xmm"))?;
    let n = t.chars().take_while(char::is_ascii_digit).count();
    if n == 0 {
        return None;
    }
    let i = t[..n].parse::<usize>().ok()?;
    if i < 32 {
        Some(i)
    } else {
        None
    }
}
fn calibrated(s: &str) -> Option<(usize, Vec<usize>, u64, u64)> {
    let s = s.split('#').next().unwrap().trim();
    let (m, a) = s.split_once(char::is_whitespace)?;
    let args = a.split(',').map(str::trim).collect::<Vec<_>>();
    if args.len() != 3
        || args.iter().any(|s| s.contains(['[', '{', '}']))
        || !args[0].starts_with("zmm")
    {
        return None;
    }
    let d = vector_reg(args[0])?;
    if m == "vpmadd52luq" || m == "vpmadd52huq" {
        if !args[1].starts_with("zmm") || !args[2].starts_with("zmm") {
            return None;
        }
        return Some((d, vec![d, vector_reg(args[1])?, vector_reg(args[2])?], 4, 4));
    }
    if m == "vpaddq" || m == "vpsubq" {
        if !args[1].starts_with("zmm") || !args[2].starts_with("zmm") {
            return None;
        }
        return Some((d, vec![vector_reg(args[1])?, vector_reg(args[2])?], 2, 1));
    }
    if (m == "vpsrlq" || m == "vpsraq") && args[1].starts_with("zmm") && args[2] == "0x34" {
        return Some((d, vec![vector_reg(args[1])?], 2, 1));
    }
    None
}
fn score(block: &[(u64, String)]) -> Value {
    let mut last = [None; 32];
    let mut nodes = vec![];
    let mut scores = vec![];
    let mut predecessors = Vec::<Option<usize>>::new();
    let mut old = vec![];
    let mut breaks = BTreeMap::<String, u64>::new();
    for (ip, s) in block {
        if let Some((d, rs, w, ow)) = calibrated(s) {
            let pred = rs
                .iter()
                .filter_map(|r| last[*r])
                .max_by_key(|j: &usize| (scores[*j], old[*j]));
            let cur = pred.map(|j| scores[j]).unwrap_or(0) + w;
            let prev = pred.map(|j| old[j]).unwrap_or(0) + ow;
            last[d] = Some(nodes.len());
            nodes.push(json!({"ip_hex":format!("{ip:#x}"),"instruction":s,"calibrated_weight":w,"inherited_model_weight":ow}));
            scores.push(cur);
            old.push(prev);
            predecessors.push(pred);
        } else {
            let (m, a) = s
                .split_once(char::is_whitespace)
                .unwrap_or((s.as_str(), ""));
            if m == "vzeroall" || m == "vzeroupper" {
                last = [None; 32];
            } else if m.starts_with('v') {
                if let Some(d) = a.split(',').next().and_then(vector_reg) {
                    last[d] = None;
                    *breaks.entry(m.into()).or_default() += 1;
                }
            }
        }
    }
    let best = scores
        .iter()
        .enumerate()
        .max_by_key(|(i, v)| (**v, old[*i]))
        .map(|(i, _)| i);
    let mut path = vec![];
    let mut at = best;
    while let Some(i) = at {
        path.push(nodes[i].clone());
        at = predecessors[i];
    }
    path.reverse();
    json!({"instruction_rows":block.len(),"calibrated_register_only_rows":nodes.len(),"unknown_vector_writer_boundaries":breaks,"max_connected_calibrated_score":best.map(|i|scores[i]).unwrap_or(0),"inherited_weights_on_same_path":best.map(|i|old[i]).unwrap_or(0),"selected_chain":path})
}
fn blocks(inst: &BTreeMap<u64, String>) -> BTreeMap<u64, Value> {
    let first = *inst.first_key_value().unwrap().0;
    let mut starts = BTreeSet::from([first]);
    let rows = inst.iter().collect::<Vec<_>>();
    for (i, (_, s)) in rows.iter().enumerate() {
        let (m, a) = s
            .split_once(char::is_whitespace)
            .unwrap_or((s.as_str(), ""));
        if m.starts_with('j') || m.starts_with("call") || m.starts_with("ret") || m == "ud2" {
            if let Some((ip, _)) = rows.get(i + 1) {
                starts.insert(**ip);
            }
            if m.starts_with('j') {
                if let Some(t) = a.split_whitespace().next() {
                    if let Ok(target) = u64::from_str_radix(t, 16) {
                        if inst.contains_key(&target) {
                            starts.insert(target);
                        }
                    }
                }
            }
        }
    }
    let starts = starts.into_iter().collect::<Vec<_>>();
    let mut out = BTreeMap::new();
    for (i, start) in starts.iter().enumerate() {
        let end = starts.get(i + 1).copied().unwrap_or(u64::MAX);
        let b = inst
            .range(*start..end)
            .map(|(a, s)| (*a, s.clone()))
            .collect::<Vec<_>>();
        let mut v = score(&b);
        v["start_hex"] = json!(format!("{start:#x}"));
        v["last_hex"] = json!(format!("{:#x}", b.last().unwrap().0));
        out.insert(*start, v);
    }
    assert_eq!(
        out.values()
            .map(|v| v["instruction_rows"].as_u64().unwrap())
            .sum::<u64>(),
        inst.len() as u64
    );
    out
}
fn parser_checks() {
    // Saved, already decoded calibration loops supply independent expected chain
    // geometry: serial recurrences vs 16 distinct accumulators and fixed sources.
    let root = "/home/ubuntu/src/zopro-perf-work/instruction-calibration176";
    let cases = read(format!("{root}/cases.json"));
    for c in cases.as_array().unwrap() {
        let id = c["id"].as_str().unwrap();
        let s = fs::read_to_string(format!("{root}/preparation/decode-{id}/stdout.log")).unwrap();
        let mut inst = BTreeMap::new();
        for l in s.lines() {
            if let Some((a, s)) = l.trim().split_once(':') {
                if let Ok(a) = u64::from_str_radix(a, 16) {
                    inst.insert(a, s.trim().into());
                }
            }
        }
        let bs = blocks(&inst);
        let best = bs
            .values()
            .map(|v| v["max_connected_calibrated_score"].as_u64().unwrap())
            .max()
            .unwrap();
        let expected = if id.contains("mem") {
            0
        } else if id.starts_with("ifma") {
            if id.ends_with("-tp") {
                64
            } else {
                1024
            }
        } else if id.ends_with("-tp") {
            2
        } else {
            512
        };
        assert_eq!(best, expected, "{id}: exact known chain geometry");
    }
    let b = vec![
        (0, "vpmadd52luq zmm0,zmm1,zmm2".into()),
        (1, "vpxor xmm0,xmm0,xmm0".into()),
        (2, "vpmadd52luq zmm0,zmm1,zmm2".into()),
    ];
    assert_eq!(score(&b)["max_connected_calibrated_score"], 4);
    assert!(calibrated("vpaddq zmm0{k1},zmm0,zmm1").is_none());
    assert!(calibrated("vpsrlq zmm0,zmm0,0x30").is_none());
    assert!(calibrated("vpmadd52luq zmm0,zmm1,ZMMWORD PTR [rsp]").is_none());
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
    assert!(!Path::new(R).join("analysis.json").exists());
    parser_checks();
    let reviewed = read(format!("{C}/review.json"));
    assert_eq!(reviewed["review_passed"], true);
    let codegen = read(format!("{Q}/qualification.json"));
    assert_eq!(codegen["status"], "pass");
    let identities = read(format!("{C}/identities.json"));
    let streams=read(format!("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-g2-validated-capture176/artifacts/profile-stream-index.json"));
    let mut pins = BTreeMap::new();
    for p in [
        format!("{C}/review.json"),
        format!("{Q}/qualification.json"),
        format!("{C}/identities.json"),
        format!("{R}/analyze.rs"),
        format!("{R}/analyze"),
        format!("{R}/protocol.md"),
        "/home/ubuntu/src/zopro-perf-work/instruction-calibration-native176/analysis.json".into(),
    ] {
        pin(&p, &mut pins);
    }
    let mut functions = BTreeMap::<String, Vec<Function>>::new();
    let mut static_rows = vec![];
    for row in codegen["rows"].as_array().unwrap() {
        let role = row["role"].as_str().unwrap();
        let kind = row["kind"].as_str().unwrap();
        let path = row["inherited_disassembly"]
            .as_str()
            .map(String::from)
            .unwrap_or_else(|| format!("{Q}/disassembly/{role}-{kind}.txt"));
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
        let graph = blocks(&inst);
        static_rows.push(json!({"role":role,"kind":kind,"rows":inst.len(),"categories":cats,"basic_blocks":graph.values().collect::<Vec<_>>() }));
        functions.entry(role.into()).or_default().push(Function {
            kind: kind.into(),
            start,
            size,
            inst,
            blocks: graph,
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
                    || sym.contains("msm::g2_validated::apply")
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
                        sym.contains(if arm == "ON" {
                            "msm::g2_validated::apply"
                        } else {
                            "msm::g2_borrowed::apply"
                        })
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
                        .entry((
                            fun.kind.clone(),
                            *fun.blocks.range(..=a).next_back().unwrap().0,
                        ))
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
            let top=hot.into_iter().take(12).map(|((kind,start),n)|{let fun=functions[role].iter().find(|f|f.kind==*kind).unwrap();json!({"function":kind,"offset_hex":format!("{:#x}",start-fun.start),"samples":n,"share_all":*n as f64/count as f64,"block":fun.blocks[start]})}).collect::<Vec<_>>();
            let mut hottest = addresses.into_iter().collect::<Vec<_>>();
            hottest.sort_by_key(|(_, n)| std::cmp::Reverse(*n));
            let ips=hottest.into_iter().take(20).map(|((kind,off,asm),n)|json!({"function":kind,"offset_hex":format!("{off:#x}"),"instruction":asm,"samples":n})).collect::<Vec<_>>();
            panels.push(json!({"profile":name,"samples":count,"normalized_instruction_samples":matched,"named_core_samples":named_kernel,"map_start_hex":format!("{map_start:#x}"),"map_offset_hex":format!("{map_off:#x}"),"ELF_vaddr_minus_offset":delta,"sampled_instruction_categories":categories,"sampled_mnemonics":mnemonics,"top_basic_blocks":top,"top_instruction_addresses":ips,"other_function_samples":other}));
            total += count;
        }
    }
    assert_eq!(total, reviewed["total_profile_samples"].as_u64().unwrap());
    assert_eq!(panels.len(), 18);
    let result = json!({"status":"pass","panels":panels,"samples":total,"static_function_categories":static_rows,"new_capture":false,"new_decode":false,"new_proofs":0,"production_changed":false,"adopted":false,"parser_checks":"16 saved calibration-loop geometries plus alias/mask/memory/shift exclusions passed without execution or decode","limits":"Exploratory basic-block/register-chain census, NOT cycle prediction or proof bottleneck attribution. Only unmasked register IFMA/add/sub/right-shift52 edges carry weights 4/2/2/2 (old 4/1/1/1). Unknown vector writers terminate chains; memory edges, other instructions, register copies and inter-block dependencies are deliberately omitted. No scheduling/throughput/frontend/cache/alias model. Flat cycle samples have skid and do not establish exclusive instruction latency, dependency stalls, spill costs, causal memory hierarchy or removable wall time. All18 panels retained; no clock-selected subset."});
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
