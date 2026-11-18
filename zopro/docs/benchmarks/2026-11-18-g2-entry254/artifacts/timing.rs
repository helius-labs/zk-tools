use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, env, fs, path::Path};

fn events(dir: &Path) -> BTreeMap<String, Vec<(u64, i64)>> {
    let data = fs::read(dir.join("stdout.log")).unwrap();
    let receipt: Value = serde_json::from_slice(&fs::read(dir.join("exit.json")).unwrap()).unwrap();
    assert_eq!(receipt["success"], true);
    assert_eq!(receipt["stdout_sha256"], format!("{:x}", Sha256::digest(&data)));
    let mut events = BTreeMap::<String, Vec<_>>::new();
    for line in std::str::from_utf8(&data).unwrap().lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') { continue; }
        let fields: Vec<_> = line.split_whitespace().collect();
        let tid = fields[0].split_once('/').unwrap().1.parse().unwrap();
        let time = fields[2].trim_end_matches(':');
        let (sec, frac) = time.split_once('.').unwrap();
        assert!(frac.len() <= 9);
        let ns = sec.parse::<i64>().unwrap() * 1_000_000_000
            + frac.parse::<i64>().unwrap() * 10i64.pow(9 - frac.len() as u32);
        let name = fields[4].trim_end_matches(':').split_once(':').unwrap().1;
        events.entry(name.to_string()).or_default().push((tid, ns));
    }
    assert_eq!(events.len(), 12);
    assert!(events.values().all(|v| v.len() == 151));
    events
}

fn main() {
    let base = env::args().nth(1).unwrap();
    let mut rows = Vec::new();
    for workers in [32, 96] {
        for block in 0..3 {
            let dir = format!("b{block}-review/decode-main-{workers}-trace-b{block}");
            let data = events(&Path::new(&base).join(dir));
            let mut values = BTreeMap::<&str, Vec<i64>>::new();
            let mut g2_last = 0;
            let mut g2_enters_first = 0;
            for i in 0..151 {
                let span = |name: &str| {
                    let (tid, t0) = data[&format!("{name}_enter")][i];
                    let (u, t1) = data[&format!("{name}_exit")][i];
                    assert_eq!(tid, u);
                    assert!(t1 >= t0);
                    (t0, t1)
                };
                let (_, fft) = span("fft");
                let (a, ae) = span("a");
                let (g, ge) = span("g2");
                let (_, end) = span("canonical");
                assert!(a >= fft && g >= fft && ae <= end && ge <= end);
                let start_delta = g - a;
                let span_delta = (ge - g) - (ae - a);
                assert_eq!(ge - ae, start_delta + span_delta);
                g2_last += usize::from(ge > ae);
                g2_enters_first += usize::from(g < a);
                for (key, ns) in [
                    ("A_entry_after_FFT", a - fft), ("G2_entry_after_FFT", g - fft),
                    ("A_elapsed", ae - a), ("G2_elapsed", ge - g),
                    ("G2_minus_A_entry", start_delta), ("G2_minus_A_elapsed", span_delta),
                    ("G2_minus_A_exit", ge - ae), ("after_last_return", end - ae.max(ge)),
                ] { values.entry(key).or_default().push(ns); }
            }
            let means: BTreeMap<_, _> = values.into_iter().map(|(k,v)| (k,v.iter().sum::<i64>() as f64 / 151e6)).collect();
            rows.push(json!({"workers":workers,"block":block,"requests":151,"G2_returns_last":g2_last,"G2_enters_first":g2_enters_first,"mean_ms":means}));
        }
    }
    println!("{}", serde_json::to_string_pretty(&rows).unwrap());
}
