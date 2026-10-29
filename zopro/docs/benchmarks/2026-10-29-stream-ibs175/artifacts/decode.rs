#[allow(dead_code)]
mod old {
    include!("legacy-decoder.rs");
    pub fn operations(raw: &str, script: &str, asm: &str) -> Value {
        let mut records = raw_records(std::io::Cursor::new(raw)).unwrap();
        super::check_pretty(raw);
        let expected = records.len();
        let instructions = super::instructions(asm);
        let mut groups = BTreeMap::<String, super::OpStats>::new();
        let mut user = BTreeSet::new();
        let mut count = 0;
        let mut unmatched = BTreeMap::<String, u64>::new();
        for line in script.lines() {
            if line.contains("PERF_RECORD_LOST") {
                panic!("lost script record");
            }
            let Some(s) = parse_sample(line).unwrap() else {
                continue;
            };
            let row = records
                .remove(&s.key)
                .expect("unmatched/duplicate script sample");
            assert_eq!(row.weight, s.weight);
            assert_eq!(row.source, s.source);
            let scope = if row.key.ip >> 63 != 0 {
                "kernel"
            } else {
                user.insert(row.key.pid);
                "user"
            };
            let cat = super::category(&s.symbol);
            let precise = row.data1 & (1 << 38) == 0 && row.ibs_ip == Some(row.key.ip);
            let instruction = super::lookup(&instructions, &s.symbol, &s.dso);
            let (mnemonic, operand) = instruction
                .map(|s| super::classify(s))
                .unwrap_or(("unresolved".into(), "unknown".into()));
            if instruction.is_none() && s.dso == super::SDK {
                *unmatched.entry(s.symbol.clone()).or_default() += 1;
            }
            for key in [
                format!("{scope}/all"),
                format!("{scope}/{cat}"),
                format!("{scope}/{cat}/mnemonic/{mnemonic}"),
                format!("{scope}/{cat}/operand/{operand}"),
            ] {
                groups
                    .entry(key)
                    .or_default()
                    .add(row.data1, row.data3, row.source, precise);
            }
            count += 1;
        }
        assert!(records.is_empty());
        assert_eq!(count, expected);
        assert_eq!(user.len(), 1);
        let groups: BTreeMap<_, _> = groups.into_iter().map(|(k, v)| (k, v.value())).collect();
        json!({"kind":"op","samples":count,"lost_records":0,"raw_script_exact_identity_weight_source_match":true,"user_pids":user,"groups":groups,"unresolved_SDK_symbols":unmatched,"instruction_lookup":"Exact mangled symbol plus perf symoff into objdump VMA; only accepted SDK classified. Invalid/substituted RIP excluded from latency/mnemonic statistics."})
    }
}
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
};
const SDK: &str = "/home/ubuntu/src/zopro-perf-work/balanced-kh-production/native/candidate-sdk";
fn category(s: &str) -> &'static str {
    if s.contains("g2_borrowed") && s.contains("5apply") {
        "G2feed"
    } else if s.contains("stream_g1_impl") {
        "G1feed"
    } else if s.contains("reduce_layer") {
        "affine_tree"
    } else if s.contains("inverse_nonzero") {
        "vector_inverse"
    } else if s.contains("inverse7inverse") {
        "scalar_inverse"
    } else if s.contains("crossbeam_epoch") {
        "epoch"
    } else if s.contains("rayon_core") || s.contains("crossbeam_deque") {
        "scheduler"
    } else {
        "other"
    }
}
fn hex(s: &str) -> u64 {
    u64::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}
fn check_pretty(raw: &str) {
    for l in raw.lines() {
        let w: Vec<_> = l.split_whitespace().collect();
        if w.first() == Some(&"ibs_op_data:") {
            let d = hex(w[1]);
            let get = |key| {
                w.iter()
                    .position(|v| *v == key)
                    .map(|i| w[i + 1].parse::<u64>().unwrap())
            };
            assert_eq!(get("CompToRetCtr"), Some(d & 65535));
            assert_eq!(get("TagToRetCtr"), Some((d >> 16) & 65535));
        }
        if w.first() == Some(&"ibs_fetch_ctl:") {
            let d = hex(w[1]);
            for (key, shift, mask) in [
                ("Lat", 32, 65535),
                ("Comp", 50, 1),
                ("IcMiss", 51, 1),
                ("L1TlbMiss", 55, 1),
                ("L2TlbMiss", 56, 1),
                ("L2Miss", 58, 1),
                ("FetchOcMiss", 60, 1),
                ("FetchL3Miss", 61, 1),
            ] {
                if key == "L2Miss" && d & (1 << 50) == 0 {
                    continue;
                }
                let i = w.iter().position(|v| *v == key).unwrap();
                assert_eq!(w[i + 1].parse::<u64>().unwrap(), (d >> shift) & mask);
            }
        }
    }
}
fn instructions(asm: &str) -> BTreeMap<(String, u64), String> {
    let mut out = BTreeMap::new();
    let mut symbol = None;
    for l in asm.lines() {
        let s = l.trim();
        if let Some((address, name)) = s.split_once(" <") {
            if let Some(name) = name.strip_suffix(">:") {
                symbol = Some((name.to_owned(), hex(address)));
                continue;
            }
        }
        if let Some((a, body)) = s.split_once(':') {
            if !a.is_empty() && a.bytes().all(|b| b.is_ascii_hexdigit()) {
                if let Some((name, base)) = &symbol {
                    let address = hex(a);
                    if address >= *base {
                        out.insert((name.clone(), address - base), body.trim().to_owned());
                    }
                }
            }
        }
    }
    out
}
fn lookup<'a>(
    map: &'a BTreeMap<(String, u64), String>,
    symbol: &str,
    dso: &str,
) -> Option<&'a str> {
    if dso != SDK {
        return None;
    }
    let (name, offset) = symbol
        .rsplit_once("+0x")
        .map(|(s, o)| (s, hex(o)))
        .unwrap_or((symbol, 0));
    map.get(&(name.to_owned(), offset)).map(String::as_str)
}
fn classify(s: &str) -> (String, String) {
    let mnemonic = s.split_whitespace().next().unwrap_or("unknown").to_owned();
    let operand = if s.contains('[') {
        if s.contains("[rsp") || s.contains("[esp") {
            "stack-addressed"
        } else {
            "other-memory"
        }
    } else {
        "no-memory-operand"
    };
    (mnemonic, operand.into())
}
fn distribution(mut values: Vec<u64>) -> Value {
    values.sort_unstable();
    let n = values.len();
    let quant = |p: usize| {
        if n == 0 {
            None
        } else {
            Some(values[(n * p).div_ceil(100).saturating_sub(1)])
        }
    };
    json!({"n":n,"sum":values.iter().sum::<u64>(),"mean":if n==0{None}else{Some(values.iter().sum::<u64>() as f64/n as f64)},"median":quant(50),"p90":quant(90),"p99":quant(99),"max":values.last()})
}
fn latencies(data: u64) -> Option<(u64, u64, u64)> {
    let complete = data & 65535;
    let tag = (data >> 16) & 65535;
    if complete > tag || complete == 65535 || tag == 65535 {
        None
    } else {
        Some((tag, complete, tag - complete))
    }
}
#[derive(Default)]
struct OpStats {
    n: u64,
    imprecise: u64,
    invalid_or_saturated: u64,
    loads: u64,
    stores: u64,
    prefetch: u64,
    l1miss: u64,
    l2miss: u64,
    ram: u64,
    tag: Vec<u64>,
    complete: Vec<u64>,
    ready: Vec<u64>,
    miss: Vec<u64>,
}
impl OpStats {
    fn add(&mut self, data1: u64, data3: u64, source: u64, precise: bool) {
        self.n += 1;
        if !precise {
            self.imprecise += 1;
            return;
        }
        self.loads += u64::from(data3 & 1 != 0);
        self.stores += u64::from(data3 & 2 != 0);
        self.prefetch += u64::from(data3 & (1 << 21) != 0);
        if data3 & 1 != 0 && data3 & (1 << 21) == 0 {
            self.l1miss += u64::from(data3 & (1 << 7) != 0);
            self.l2miss += u64::from(data3 & (1 << 7) != 0 && data3 & (1 << 20) != 0);
            self.ram += u64::from((source >> 33) & 15 == 13);
            if data3 & (1 << 7) != 0 && data3 & (1 << 14) == 0 {
                self.miss.push((data3 >> 32) & 65535);
            }
        }
        if let Some((t, c, r)) = latencies(data1) {
            self.tag.push(t);
            self.complete.push(c);
            self.ready.push(r);
        } else {
            self.invalid_or_saturated += 1;
        }
    }
    fn value(self) -> Value {
        json!({"samples":self.n,"imprecise_excluded":self.imprecise,"invalid_or_saturated_latency_pairs":self.invalid_or_saturated,"load_ops":self.loads,"store_ops":self.stores,"software_prefetch":self.prefetch,"demand_L1_misses":self.l1miss,"demand_L2_misses":self.l2miss,"demand_RAM":self.ram,"tag_to_retire":distribution(self.tag),"complete_to_retire":distribution(self.complete),"tag_to_complete":distribution(self.ready),"valid_demand_miss_latency":distribution(self.miss)})
    }
}
#[derive(Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
struct Key {
    pid: u64,
    tid: u64,
    ns: u64,
    ip: u64,
}
fn pidtid(s: &str) -> (u64, u64) {
    let (a, b) = s.trim_end_matches(':').split_once('/').unwrap();
    (a.parse().unwrap(), b.parse().unwrap())
}
fn nanos(s: &str) -> u64 {
    let (a, b) = s.trim_end_matches(':').split_once('.').unwrap();
    assert_eq!(b.len(), 9);
    a.parse::<u64>().unwrap() * 1_000_000_000 + b.parse::<u64>().unwrap()
}
fn fetch_records(raw: &str) -> BTreeMap<Key, u64> {
    check_pretty(raw);
    let mut out = BTreeMap::new();
    let (mut ctl, mut ip) = (None, None);
    for l in raw.lines() {
        assert!(!l.contains("]: PERF_RECORD_LOST"));
        if let Some(s) = l.strip_prefix("ibs_fetch_ctl:") {
            assert!(ctl
                .replace(hex(s.split_whitespace().next().unwrap()))
                .is_none());
        }
        if let Some(s) = l.strip_prefix("IbsFetchLinAd:") {
            ip = Some(hex(s.trim()));
        }
        if let Some((prefix, tail)) = l.split_once("]: PERF_RECORD_SAMPLE(IP,") {
            let (_, s) = tail.split_once("): ").unwrap();
            let w: Vec<_> = s.split_whitespace().collect();
            let (pid, tid) = pidtid(w[0]);
            let actual = hex(w[1]);
            assert_eq!(ip.take(), Some(actual));
            let d = ctl.take().unwrap();
            assert_ne!(d & (1 << 49), 0);
            let key = Key {
                pid,
                tid,
                ns: prefix.split_whitespace().nth(1).unwrap().parse().unwrap(),
                ip: actual,
            };
            assert!(out.insert(key, d).is_none());
        }
    }
    assert!(ctl.is_none());
    assert!(!out.is_empty());
    out
}
#[derive(Default)]
struct FetchStats {
    n: u64,
    complete: u64,
    ic: u64,
    oc: u64,
    l1tlb: u64,
    l2tlb: u64,
    l2miss: u64,
    l3miss: u64,
    lat: Vec<u64>,
}
impl FetchStats {
    fn add(&mut self, d: u64) {
        self.n += 1;
        self.ic += u64::from(d & (1 << 51) != 0);
        self.oc += u64::from(d & (1 << 60) != 0);
        self.l1tlb += u64::from(d & (1 << 55) != 0);
        self.l2tlb += u64::from(d & (1 << 56) != 0);
        if d & (1 << 50) != 0 {
            self.complete += 1;
            self.lat.push((d >> 32) & 65535);
            self.l2miss += u64::from(d & (1 << 58) != 0);
            self.l3miss += u64::from(d & (1 << 61) != 0);
        }
    }
    fn value(self) -> Value {
        json!({"samples":self.n,"complete":self.complete,"icache_misses":self.ic,"opcache_misses":self.oc,"L1_ITLB_misses":self.l1tlb,"L2_ITLB_misses":self.l2tlb,"complete_L2_misses":self.l2miss,"complete_L3_misses":self.l3miss,"complete_fetch_latency":distribution(self.lat)})
    }
}
fn fetch(raw: &str, script: &str) -> Value {
    let mut rows = fetch_records(raw);
    let expected = rows.len();
    let mut groups = BTreeMap::<String, FetchStats>::new();
    let mut user = BTreeSet::new();
    let mut count = 0;
    for l in script.lines() {
        if l.trim().is_empty() || l.starts_with('#') {
            continue;
        }
        assert!(!l.contains("PERF_RECORD_LOST"));
        let (body, _dso) = l.rsplit_once(" (").unwrap();
        let w: Vec<_> = body.split_whitespace().collect();
        assert!(w.len() >= 4);
        let (pid, tid) = pidtid(w[0]);
        let key = Key {
            pid,
            tid,
            ns: nanos(w[1]),
            ip: hex(w[2]),
        };
        let d = rows.remove(&key).expect("unmatched/duplicate fetch");
        let scope = if key.ip >> 63 != 0 {
            "kernel"
        } else {
            user.insert(pid);
            "user"
        };
        let symbol = w[3..].join(" ");
        for name in [
            format!("{scope}/all"),
            format!("{scope}/{}", category(&symbol)),
        ] {
            groups.entry(name).or_default().add(d);
        }
        count += 1;
    }
    assert!(rows.is_empty());
    assert_eq!(count, expected);
    assert_eq!(user.len(), 1);
    let groups: BTreeMap<_, _> = groups.into_iter().map(|(k, v)| (k, v.value())).collect();
    json!({"kind":"fetch","samples":count,"lost_records":0,"raw_script_exact_identity_match":true,"user_pids":user,"groups":groups})
}
fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 6, "decode MODE RAW SCRIPT ASSEMBLY OUT");
    let raw = fs::read_to_string(&args[2]).unwrap();
    let script = fs::read_to_string(&args[3]).unwrap();
    let value = match args[1].as_str() {
        "op" => old::operations(&raw, &script, &fs::read_to_string(&args[4]).unwrap()),
        "fetch" => fetch(&raw, &script),
        _ => panic!("op|fetch"),
    };
    fs::write(&args[5], serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    println!(
        "Decoded {} {} samples; exact raw/script join, zero lost",
        args[1], value["samples"]
    );
}
#[cfg(test)]
mod extra_tests {
    use super::*;
    #[test]
    fn exact_symbol_offset_and_dso_match() {
        let asm="0000000000001000 <_Rfoo>:\n1000: vpmadd52luq zmm0,zmm1,zmm2\n1006: vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x40]\n";
        let m = instructions(asm);
        assert_eq!(
            lookup(&m, "_Rfoo+0x6", SDK),
            Some("vmovdqa64 zmm1,ZMMWORD PTR [rsp+0x40]")
        );
        assert!(lookup(&m, "_Rfoo+0x5", SDK).is_none());
        assert!(lookup(&m, "_Rfoo+0x6", "libc").is_none());
        assert_eq!(
            classify(lookup(&m, "_Rfoo+0x6", SDK).unwrap()).1,
            "stack-addressed"
        );
    }
    #[test]
    fn latency_pair_bounds_and_precision() {
        assert_eq!(latencies((50 << 16) | 20), Some((50, 20, 30)));
        assert_eq!(latencies((20 << 16) | 50), None);
        assert_eq!(latencies(65535 << 16), None);
        let mut s = OpStats::default();
        s.add((50 << 16) | 20, 0, 0, true);
        s.add((50 << 16) | 20, 0, 0, false);
        s.add((20 << 16) | 50, 0, 0, true);
        let v = s.value();
        assert_eq!(v["tag_to_complete"]["n"], 1);
        assert_eq!(v["imprecise_excluded"], 1);
        assert_eq!(v["invalid_or_saturated_latency_pairs"], 1);
    }
    #[test]
    fn incomplete_fetch_never_counts_latency_or_l2miss() {
        check_pretty("ibs_fetch_ctl: 0233000080008000 Lat 0 Comp 0 IcMiss 0 L1TlbMiss 0 L2TlbMiss 0 FetchOcMiss 0 FetchL3Miss 0\n");
        let mut s = FetchStats::default();
        s.add((99 << 32) | (1 << 58) | (1 << 60));
        s.add((8 << 32) | (1 << 50) | (1 << 51));
        let v = s.value();
        assert_eq!(v["complete_fetch_latency"]["n"], 1);
        assert_eq!(v["complete_L2_misses"], 0);
        assert_eq!(v["opcache_misses"], 1);
        assert_eq!(v["icache_misses"], 1);
    }
    #[test]
    fn raw_fetch_rejects_mismatched_ip_and_lost() {
        let raw="ibs_fetch_ctl: 6000000000000\nIbsFetchLinAd: 1000\n1 99 0x100 [0x50]: PERF_RECORD_SAMPLE(IP, 0x4002): 7/8: 0x1001 period: 524288 addr: 0\n";
        assert!(std::panic::catch_unwind(|| fetch_records(raw)).is_err());
        assert!(std::panic::catch_unwind(|| fetch_records(
            "1 99 0x100 [0x50]: PERF_RECORD_LOST 1"
        ))
        .is_err());
    }
}
