use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

type Result<T> = std::result::Result<T, String>;

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Key {
    pid: u64,
    tid: u64,
    ns: u64,
    ip: u64,
}

#[derive(Clone, Debug)]
struct Raw {
    key: Key,
    ibs_ip: u64,
    data1: u64,
    data3: u64,
    addr: u64,
    weight: u64,
    source: u64,
}

#[derive(Debug)]
struct Sample {
    key: Key,
    weight: u64,
    source: u64,
    level: String,
    symbol: String,
    dso: String,
}

fn number(s: &str) -> Result<u64> {
    s.parse().map_err(|_| format!("invalid decimal: {s}"))
}

fn hex(s: &str) -> Result<u64> {
    u64::from_str_radix(s.trim_start_matches("0x"), 16)
        .map_err(|_| format!("invalid hexadecimal: {s}"))
}

fn pid_tid(s: &str) -> Result<(u64, u64)> {
    let (a, b) = s.trim_end_matches(':').split_once('/').ok_or("missing pid/tid")?;
    Ok((number(a)?, number(b)?))
}

fn ns(s: &str) -> Result<u64> {
    let (seconds, fraction) = s.trim_end_matches(':').split_once('.').ok_or("missing decimal time")?;
    if fraction.len() != 9 {
        return Err(format!("expected nanosecond precision: {s}"));
    }
    number(seconds)?.checked_mul(1_000_000_000)
        .and_then(|v| number(fraction).ok().and_then(|f| v.checked_add(f)))
        .ok_or_else(|| "invalid or overflowing timestamp".into())
}

fn valid_weight(data3: u64) -> u64 {
    if data3 & 1 != 0 && data3 & (1 << 7) != 0 {
        (data3 >> 32) & 0xffff
    } else {
        0
    }
}

fn parse_sample(line: &str) -> Result<Option<Sample>> {
    if line.trim().is_empty() || line.trim_start().starts_with('#') {
        return Ok(None);
    }
    let parts: Vec<_> = line.split('|').collect();
    if parts.len() != 7 {
        return Err(format!("unexpected sample columns: {line}"));
    }
    let first: Vec<_> = parts[0].split_whitespace().collect();
    if first.len() != 3 {
        return Err(format!("unexpected sample prefix: {}", parts[0]));
    }
    let (pid, tid) = pid_tid(first[0])?;
    let last = parts[6].trim_end();
    let (body, dso) = last.rsplit_once(" (").ok_or("missing DSO")?;
    let dso = dso.strip_suffix(')').ok_or("missing DSO end")?.to_owned();
    let tokens: Vec<_> = body.split_whitespace().collect();
    let at = (1..tokens.len().saturating_sub(2)).find(|&i| {
        tokens[i].bytes().all(|v| v.is_ascii_digit())
            && tokens[i + 1].bytes().all(|v| v.is_ascii_hexdigit())
    }).ok_or("missing weight/IP/symbol fields")?;
    Ok(Some(Sample {
        key: Key { pid, tid, ns: ns(first[1])?, ip: hex(tokens[at + 1])? },
        weight: number(tokens[at])?,
        source: hex(first[2])?,
        level: parts[2].strip_prefix("LVL ").ok_or("missing level")?.to_owned(),
        symbol: tokens[at + 2..].join(" "),
        dso,
    }))
}

fn register(line: &str, name: &str) -> Result<Option<u64>> {
    if let Some(rest) = line.strip_prefix(name) {
        return Ok(Some(hex(rest.split_whitespace().next().ok_or("missing register")?)?));
    }
    Ok(None)
}

fn raw_records(reader: impl BufRead) -> Result<BTreeMap<Key, Raw>> {
    let mut records = BTreeMap::new();
    let (mut ip, mut data1, mut data3) = (None, None, None);
    let mut pending: Option<Raw> = None;
    let mut weight = None;
    for line in reader.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.contains("]: PERF_RECORD_LOST") {
            return Err("lost record found".into());
        }
        if let Some(v) = register(&line, "IbsOpRip:")? { ip = Some(v); }
        if let Some(v) = register(&line, "ibs_op_data:")? { data1 = Some(v); }
        if let Some(v) = register(&line, "ibs_op_data3:")? { data3 = Some(v); }
        if let Some((prefix, tail)) = line.split_once("]: PERF_RECORD_SAMPLE(IP,") {
            if pending.is_some() { return Err("incomplete previous raw sample".into()); }
            let (_, sample) = tail.split_once("): ").ok_or("invalid sample header")?;
            let words: Vec<_> = sample.split_whitespace().collect();
            let (pid, tid) = pid_tid(words.first().ok_or("missing sample process")?)?;
            let sample_ip = hex(words.get(1).ok_or("missing sample IP")?)?;
            let timestamp = prefix.split_whitespace().nth(1).ok_or("missing raw time")?;
            let address = words.windows(2).find_map(|w| (w[0] == "addr:").then_some(w[1])).ok_or("missing sample address")?;
            pending = Some(Raw {
                key: Key { pid, tid, ns: number(timestamp)?, ip: sample_ip },
                ibs_ip: ip.take().ok_or("missing raw IBS IP")?,
                data1: data1.take().ok_or("missing raw IBS data1")?,
                data3: data3.take().ok_or("missing raw IBS data3")?,
                addr: hex(address)?, weight: 0, source: 0,
            });
            weight = None;
        }
        if let Some(rest) = line.trim().strip_prefix("... weight:") {
            weight = Some(number(rest.trim())?);
        }
        if let Some(rest) = line.trim().strip_prefix(". data_src:") {
            let mut row = pending.take().ok_or("data source outside raw sample")?;
            row.source = hex(rest.trim())?;
            row.weight = weight.take().ok_or("missing perf sample weight")?;
            let op = if row.data3 & 1 != 0 { 2 } else if row.data3 & 2 != 0 { 4 } else { 1 };
            if row.source & 0x1f != op || row.weight != valid_weight(row.data3) {
                return Err(format!("raw/perf operation or weight mismatch at {:?}", row.key));
            }
            if records.insert(row.key, row).is_some() {
                return Err("duplicate raw sample identity".into());
            }
        }
    }
    if pending.is_some() || records.is_empty() { return Err("incomplete or empty raw stream".into()); }
    Ok(records)
}

fn category(symbol: &str) -> &'static str {
    if symbol.contains("g2_prefix") { "G2prefix" }
    else if symbol.contains("stream_g1_impl") { "G1stream" }
    else if symbol.contains("reduce_layer") { "affine_tree" }
    else if symbol.contains("inverse_nonzero") { "vector_inverse" }
    else if symbol.contains("inverse7inverse") { "scalar_inverse" }
    else if symbol.contains("crossbeam_epoch") { "epoch" }
    else if symbol.contains("rayon_core") || symbol.contains("crossbeam_deque") { "scheduler" }
    else if symbol.contains("g2_phi") { "G2dispatch" }
    else { "other" }
}

fn source_name(source: u64) -> &'static str {
    match (source >> 33) & 15 {
        1 => "L1", 2 => "L2", 3 => "L3", 13 => "RAM", 12 => "LFB",
        8 => "uncached", 11 => "other_cache", 15 | 0 => "N/A", _ => "other",
    }
}

#[derive(Default)]
struct Stats {
    samples: u64,
    positive: Vec<u64>,
    source_counts: BTreeMap<String, u64>,
    source_weights: BTreeMap<String, u64>,
    page_offsets: BTreeMap<u64, u64>,
    l1_miss: u64,
    l2_miss: u64,
    valid_addresses: u64,
}
impl Stats {
    fn add(&mut self, raw: &Raw) {
        self.samples += 1;
        if raw.weight != 0 { self.positive.push(raw.weight); }
        let source = source_name(raw.source);
        *self.source_counts.entry(source.into()).or_default() += 1;
        *self.source_weights.entry(source.into()).or_default() += raw.weight;
        self.l1_miss += u64::from(raw.data3 & (1 << 7) != 0);
        self.l2_miss += u64::from(raw.data3 & (1 << 7) != 0 && raw.data3 & (1 << 20) != 0);
        if raw.data3 & (1 << 17) != 0 {
            self.valid_addresses += 1;
            *self.page_offsets.entry((raw.addr >> 6) & 63).or_default() += 1;
        }
    }
    fn value(&mut self) -> Value {
        self.positive.sort_unstable();
        let n = self.positive.len();
        let sum: u64 = self.positive.iter().sum();
        let quantile = |p: usize| if n == 0 { None } else { Some(self.positive[(n * p).div_ceil(100).saturating_sub(1)]) };
        json!({"demand_load_samples": self.samples, "l1_miss_samples": self.l1_miss,
            "l2_miss_samples": self.l2_miss, "sources": self.source_counts,
            "source_weight_sums": self.source_weights, "positive_miss_samples": n,
            "miss_weight_sum": sum, "positive_weight_mean": (n != 0).then(|| sum as f64 / n as f64),
            "positive_weight_median_nearest_rank": quantile(50), "positive_weight_p90": quantile(90),
            "valid_data_addresses": self.valid_addresses, "address_page_line_bins": self.page_offsets})
    }
}

fn analyze(raw: impl BufRead, samples: impl BufRead) -> Result<Value> {
    let mut raw = raw_records(raw)?;
    let raw_count = raw.len();
    let mut groups = BTreeMap::<String, Stats>::new();
    let mut human_sources = BTreeMap::<String, u64>::new();
    let mut symbols = BTreeMap::<String, u64>::new();
    let (mut count, mut substituted, mut invalid, mut stale) = (0, 0, 0, 0);
    let (mut prefetch, mut stores, mut uncached) = (0, 0, 0);
    let mut user_pids = BTreeSet::new();
    let mut all_pids = BTreeSet::new();
    for line in samples.lines() {
        let line = line.map_err(|e| e.to_string())?;
        if line.contains("PERF_RECORD_LOST") { return Err("lost sample in script".into()); }
        let Some(sample) = parse_sample(&line)? else { continue };
        let row = raw.remove(&sample.key).ok_or_else(|| format!("unmatched or duplicate script sample {:?}", sample.key))?;
        if sample.source != row.source || sample.weight != row.weight {
            return Err(format!("raw/script source or weight mismatch {:?}", sample.key));
        }
        count += 1;
        substituted += u64::from(row.ibs_ip != row.key.ip);
        invalid += u64::from(row.data1 & (1 << 38) != 0);
        all_pids.insert(row.key.pid);
        let scope = if row.key.ip >> 63 != 0 { "kernel" } else { user_pids.insert(row.key.pid); "user" };
        stores += u64::from(row.data3 & 2 != 0);
        if row.data3 & 1 == 0 { continue; }
        if row.data3 & (1 << 7) == 0 { stale += (row.data3 >> 32) & 0xffff; }
        if row.data3 & (1 << 21) != 0 { prefetch += 1; continue; }
        if row.data3 & (1 << 14) != 0 { uncached += 1; }
        groups.entry(format!("{scope}/all")).or_default().add(&row);
        groups.entry(format!("{scope}/{}", category(&sample.symbol))).or_default().add(&row);
        *human_sources.entry(format!("{scope}/{}", sample.level)).or_default() += 1;
        *symbols.entry(format!("{scope}/{} ({})", sample.symbol, sample.dso)).or_default() += 1;
    }
    if !raw.is_empty() || count != raw_count as u64 {
        return Err(format!("unconsumed raw samples: {}", raw.len()));
    }
    if user_pids.len() != 1 { return Err(format!("expected one user PID, got {user_pids:?}")); }
    let data: BTreeMap<_, _> = groups.iter_mut().map(|(k, v)| (k.clone(), v.value())).collect();
    Ok(json!({"samples": count, "raw_script_exact_identity_weight_source_match": true,
        "lost_records": 0, "user_pids": user_pids, "all_pids": all_pids,
        "raw_record_ip_mismatches": substituted, "raw_rip_invalid": invalid,
        "load_nonmiss_stale_weight_sum_ignored": stale,
        "software_prefetch_load_samples_excluded": prefetch,
        "raw_store_samples": stores, "uncached_demand_load_samples": uncached,
        "groups": data, "human_source_labels": human_sources, "demand_symbols": symbols,
        "limitations": "Miss weights and sample fractions are not exclusive wall time or bandwidth. Prefetch removed by individual raw flag; timestamp/thread/record-IP identities match. Page-line bins are virtual page offsets, not proof of cache conflicts. Kernel/source decoder agreement is checked; Ubuntu runtime kernel is not asserted identical to upstream reference."}))
}

fn main() {
    let args: Vec<_> = std::env::args().collect();
    assert_eq!(args.len(), 4, "decoder RAW_DUMP PERF_SCRIPT OUTPUT_JSON");
    let raw = BufReader::new(File::open(Path::new(&args[1])).unwrap());
    let script = BufReader::new(File::open(Path::new(&args[2])).unwrap());
    let value = analyze(raw, script).unwrap();
    std::fs::write(&args[3], serde_json::to_vec_pretty(&value).unwrap()).unwrap();
    println!("Decoded {} samples; exact raw/script identity/weight/source agreement; zero lost", value["samples"]);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    fn fixture(data3: u64, actual_ip: u64, raw_ip: u64, ns: u64) -> (String, String) {
        let source = if data3 & 1 != 0 { (1u64 << 33) | 2 } else { 1 };
        let raw = format!("IbsOpRip: {raw_ip:x}\nibs_op_data: {:x}\nibs_op_data3: {data3:x}\n2 {ns} 0x10 [0x88]: PERF_RECORD_SAMPLE(IP, 0x4002): 7/8: 0x{actual_ip:x} period: 524288 addr: 0x1234\n... weight: {}\n . data_src: 0x{source:x}\n", if raw_ip != actual_ip { 1u64 << 38 } else { 0 }, valid_weight(data3));
        let script = format!("7/8 {}.{:09}: {source:x} |OP LOAD|LVL L1 hit|SNP N/A|TLB L1 hit|LCK N/A|BLK N/A {} {actual_ip:x} a symbol with spaces (a binary with spaces)\n", ns / 1_000_000_000, ns % 1_000_000_000, valid_weight(data3));
        (raw, script)
    }

    #[test]
    fn nanoseconds_are_exact_and_invalid_precision_rejected() {
        assert_eq!(ns("33566.401331287:").unwrap(), 33_566_401_331_287);
        assert!(ns("33566.401331:").is_err());
        assert!(ns("x.401331287:").is_err());
    }

    #[test]
    fn stale_nonmiss_weights_do_not_count() {
        assert_eq!(valid_weight(1 | (999 << 32)), 0);
        assert_eq!(valid_weight(1 | (1 << 7) | (321 << 32)), 321);
        assert_eq!(valid_weight((1 << 7) | (321 << 32)), 0);
    }

    #[test]
    fn same_ip_demand_and_prefetch_are_individually_matched() {
        let (a, x) = fixture(1, 0x500, 0x500, 1_000_000_001);
        let (b, y) = fixture(1 | (1 << 21), 0x500, 0x500, 1_000_000_002);
        let value = analyze(Cursor::new(a + &b), Cursor::new(y + &x)).unwrap();
        assert_eq!(value["samples"], 2);
        assert_eq!(value["software_prefetch_load_samples_excluded"], 1);
        assert_eq!(value["groups"]["user/all"]["demand_load_samples"], 1);
    }

    #[test]
    fn record_ip_controls_scope_after_raw_rip_substitution() {
        let (a, b) = fixture(1, 0x500, u64::MAX, 1);
        let value = analyze(Cursor::new(a), Cursor::new(b)).unwrap();
        assert_eq!(value["raw_record_ip_mismatches"], 1);
        assert_eq!(value["raw_rip_invalid"], 1);
        assert_eq!(value["user_pids"], json!([7]));
    }

    #[test]
    fn duplicate_and_missing_records_fail() {
        let (a, b) = fixture(1, 0x500, 0x500, 1);
        assert!(analyze(Cursor::new(a.clone() + &a), Cursor::new(&b)).is_err());
        assert!(analyze(Cursor::new(&a), Cursor::new(b.clone() + &b)).is_err());
        assert!(analyze(Cursor::new(a), Cursor::new("")).is_err());
    }

    #[test]
    fn names_with_spaces_and_weight_source_mismatches() {
        let (a, b) = fixture(1, 0x500, 0x500, 1);
        let s = parse_sample(&b).unwrap().unwrap();
        assert_eq!(s.symbol, "a symbol with spaces");
        assert_eq!(s.dso, "a binary with spaces");
        assert!(analyze(Cursor::new(a), Cursor::new(b.replace("BLK N/A 0", "BLK N/A 7"))).is_err());
    }

    #[test]
    fn incomplete_and_lost_raw_records_fail() {
        assert!(raw_records(Cursor::new("IbsOpRip: 123\n")).is_err());
        assert!(raw_records(Cursor::new("0 1 0x10 [0x88]: PERF_RECORD_LOST 4\n")).is_err());
    }
}
