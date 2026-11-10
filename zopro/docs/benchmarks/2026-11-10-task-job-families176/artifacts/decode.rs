use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
};
const ROOT: &str = "/home/ubuntu/src/zopro-perf-work/task-job-families176";
const INPUT: &str = "/home/ubuntu/src/zopro-perf-work/task-interval-c7a176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn graph_path(j: &Value) -> PathBuf {
    Path::new(INPUT)
        .join("remote-results")
        .join(j["proof_dir"].as_str().unwrap().strip_prefix('/').unwrap())
        .join("graphs.json")
}
fn main() {
    let r = Path::new(ROOT);
    let c = Path::new(INPUT);
    let jobs = read(c.join("remote-results/output/measure/jobs.json"));
    let jobs = jobs
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["arm"] == "ON" && j["panel"] == "latency")
        .collect::<Vec<_>>();
    assert_eq!(jobs.len(), 18);
    if std::env::args().nth(1).as_deref() == Some("prepare") {
        let mut pins = BTreeMap::new();
        for p in [
            r.join("decode.rs"),
            r.join("decode"),
            r.join("symbols.txt"),
            c.join("context/cr-current"),
            c.join("context/manifest.json"),
            c.join("remote-results/output/measure/jobs.json"),
            c.join("phase-summary.json"),
            c.join("target-analysis.json"),
        ]
        .into_iter()
        .chain(jobs.iter().map(|j| graph_path(j)))
        {
            pins.insert(p.display().to_string(), hash(&p));
        }
        save(r.join("pins.json"), &json!(pins));
        save(
            r.join("plan.json"),
            &json!({"version":1,"source_pins":r.join("pins.json"),"steps":[{"kind":"command","name":"decode-published-job-families","argv":[r.join("decode"),"decode"],"timeout_seconds":300}]}),
        );
        println!("Frozen18ONprocesses/54graphs; offline decode only; zero new proofs");
        return;
    }
    assert_eq!(
        hash(c.join("context/cr-current")),
        read(c.join("context/manifest.json"))["container_pins"]["/usr/local/bin/cr-current"]
    );
    let mut symbols = BTreeMap::<u64, (u64, Vec<String>)>::new();
    for l in fs::read_to_string(r.join("symbols.txt")).unwrap().lines() {
        let f = l.split_whitespace().collect::<Vec<_>>();
        if f.len() < 4 || !matches!(f[2], "t" | "T" | "w" | "W") {
            continue;
        }
        let addr = u64::from_str_radix(f[0], 16).unwrap();
        let size = u64::from_str_radix(f[1], 16).unwrap();
        let e = symbols.entry(addr).or_default();
        e.0 = e.0.max(size);
        e.1.push(f[3..].join(" "));
    }
    assert!(symbols.len() > 100);
    let phase = read(c.join("phase-summary.json"));
    let mut graph_rows = vec![];
    let mut translations = vec![];
    for j in jobs {
        let raw = read(graph_path(j));
        assert_eq!(raw.as_array().unwrap().len(), 3);
        let ptrs = raw
            .as_array()
            .unwrap()
            .iter()
            .flat_map(|g| g["threads"].as_array().unwrap())
            .flat_map(|t| t["events"].as_array().unwrap())
            .filter(|e| e[2] == 1)
            .map(|e| e[4].as_u64().unwrap())
            .collect::<BTreeSet<_>>();
        assert!(!ptrs.is_empty() && !ptrs.contains(&0));
        let first = *ptrs.first().unwrap();
        let mut candidates = symbols
            .keys()
            .filter_map(|s| first.checked_sub(*s))
            .filter(|bias| bias % 4096 == 0)
            .collect::<BTreeSet<_>>();
        for ptr in &ptrs {
            candidates.retain(|bias| {
                ptr.checked_sub(*bias)
                    .is_some_and(|a| symbols.contains_key(&a))
            });
        }
        assert_eq!(
            candidates.len(),
            1,
            "require unique common page-aligned translation for {}",
            j["name"]
        );
        let bias = *candidates.first().unwrap();
        translations.push(json!({"job":j["name"],"bias":bias,"candidate_count":1,"distinct_function_addresses":ptrs.len(),"all_addresses_match_defined_text":true}));
        for g in raw.as_array().unwrap() {
            for p in 2..=6 {
                let bounds = g["check"]["phases"][p.to_string()].as_array().unwrap();
                let lo = bounds[0].as_u64().unwrap();
                let hi = bounds[1].as_u64().unwrap();
                let mut counts = BTreeMap::<u64, u64>::new();
                for t in g["threads"].as_array().unwrap() {
                    for e in t["events"].as_array().unwrap() {
                        if e[2] != 1 {
                            continue;
                        }
                        let time = e[0].as_u64().unwrap();
                        if time >= lo && time < hi {
                            *counts.entry(e[4].as_u64().unwrap() - bias).or_default() += 1;
                        }
                    }
                }
                let expected = phase["rows"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .find(|row| {
                        row["job"] == j["name"] && row["round"] == g["round"] && row["phase"] == p
                    })
                    .unwrap();
                assert_eq!(
                    counts.values().sum::<u64>(),
                    expected["matched_publications_in_phase"].as_u64().unwrap()
                );
                graph_rows.push(json!({"job":j["name"],"round":g["round"],"workers":j["workers"],"phase":p,"counts_by_symbol_address":counts}));
            }
        }
        println!("matched {} unique functions{}", ptrs.len(), j["name"]);
    }
    assert_eq!(graph_rows.len(), 270);
    let mut summary = vec![];
    for workers in [32, 96] {
        for phase in 2..=6 {
            let rows = graph_rows
                .iter()
                .filter(|g| g["workers"] == workers && g["phase"] == phase)
                .collect::<Vec<_>>();
            assert_eq!(rows.len(), 27);
            let mut totals = BTreeMap::<u64, u64>::new();
            for row in &rows {
                for (addr, n) in row["counts_by_symbol_address"].as_object().unwrap() {
                    *totals.entry(addr.parse().unwrap()).or_default() += n.as_u64().unwrap();
                }
            }
            for (addr, total) in totals {
                let mut counts = rows
                    .iter()
                    .map(|g| {
                        g["counts_by_symbol_address"][addr.to_string()]
                            .as_u64()
                            .unwrap_or(0)
                    })
                    .collect::<Vec<_>>();
                counts.sort_unstable();
                let sym = &symbols[&addr];
                summary.push(json!({"workers":workers,"phase":phase,"symbol_address":format!("0x{addr:x}"),"symbol_size":sym.0,"symbol_aliases":sym.1,"total_publications_27_graphs":total,"median_publications_including_zero_graphs":counts[13]}));
            }
        }
    }
    summary.sort_by_key(|v| {
        (
            v["workers"].as_u64().unwrap(),
            v["phase"].as_u64().unwrap(),
            std::cmp::Reverse(v["total_publications_27_graphs"].as_u64().unwrap()),
        )
    });
    save(
        r.join("analysis.json"),
        &json!({"scope":"posthoc offline classification of all54 fixed retained main instrumented graphs","target_observer_qualified":false,"causal_or_uninstrumented_claim":false,"new_proofs":0,"canonical_ledger":50208,"unique_translation_required":true,"translations":translations,"summary":summary,"graph_rows":graph_rows}),
    );
    for w in [32, 96] {
        for p in [3, 5] {
            println!("TOP workers={w} phase={p}");
            for v in summary
                .iter()
                .filter(|v| v["workers"] == w && v["phase"] == p)
                .take(6)
            {
                println!(
                    "{} pubs={} {}",
                    v["symbol_address"], v["total_publications_27_graphs"], v["symbol_aliases"]
                );
            }
        }
    }
}
