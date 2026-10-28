use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    io::Read,
    path::Path,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/euler-fused-columns";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-euler-fused-columns";
const PH: &str = "3f0d846c3905d11595d77dd988df829ddc51325eda2b93e4db8f0e17a5928e5b";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
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
fn median(v: &[f64]) -> f64 {
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    s[s.len() / 2]
}
fn interval(v: &[f64]) -> Value {
    assert_eq!(v.len(), 15);
    let mut s = v.to_vec();
    s.sort_by(f64::total_cmp);
    json!({"median":s[7],"low":s[3],"high":s[11],"wins":v.iter().filter(|&&r|r<1.).count(),"all_ratios":v,"nominal_coverage":0.96484375})
}
fn samples(name: &str) -> (Vec<f64>, Value) {
    let text = fs::read_to_string(format!("{R}/runs/{name}/stdout.log")).unwrap();
    let samples = text
        .lines()
        .filter_map(|s| s.strip_prefix("SAMPLE "))
        .map(|s| {
            serde_json::from_str::<Value>(s).unwrap()["ms"]
                .as_f64()
                .unwrap()
        })
        .collect();
    let summary = serde_json::from_str(
        text.lines()
            .find_map(|s| s.strip_prefix("SUMMARY "))
            .unwrap(),
    )
    .unwrap();
    (samples, summary)
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let name = e.file_name().to_string_lossy().to_string();
        if e.file_type().unwrap().is_dir() {
            if ["source", "target", "profiles"].contains(&name.as_str()) {
                continue;
            }
            archive(root, &p, out, index)
        } else if ["rs", "md", "json", "log", "txt", "proof", "public"]
            .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
            && ![
                "close.log",
                "close-error.log",
                "archive-verification.log",
                "push.log",
            ]
            .contains(&name.as_str())
        {
            if p.to_string_lossy().contains("/loss-") && name == "stdout.log" {
                continue;
            }
            let rel = p.strip_prefix(root).unwrap();
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            index.insert(format!("artifacts/{}", rel.display()), hash(dest));
        }
    }
}
fn main() {
    let r = Path::new(R);
    assert_eq!(read(r.join("runs/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("runs/result.json"))["steps"]
            .as_array()
            .unwrap()
            .len(),
        216
    );
    let pins = read(r.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    for mode in ["off", "on"] {
        let t = fs::read_to_string(r.join(format!("native/core-{mode}/stdout.log"))).unwrap();
        assert!(t.contains("test result: ok. 122 passed; 0 failed"));
        for name in [
            "fused_columns_match_scalar_products_for_prefixes_and_cached_roots",
            "padded_output_iterators_preserve_each_existing_row_once",
            "fused_columns_share_roots_on_small_worker_stacks",
        ] {
            assert!(t.contains(&format!(
                "test fft::pair::euler_output::tests::{name} ... ok"
            )));
        }
    }
    for (name, n) in [("raw-tests", 2), ("cubic-tests", 6)] {
        assert!(
            fs::read_to_string(r.join(format!("native/{name}/stdout.log")))
                .unwrap()
                .contains(&format!("test result: ok. {n} passed; 0 failed"))
        );
    }
    let trace = fs::read_to_string(r.join("runs/preflight-on/stderr.log")).unwrap();
    assert_eq!(
        trace
            .lines()
            .filter(|s| *s == "EULER_FUSED_COLUMNS n=32768 rows=30922")
            .count(),
        4
    );
    let mut cells = BTreeMap::new();
    let mut pairs = BTreeMap::new();
    let mut failed = vec![];
    let mut table = String::new();
    let mut pair_table = String::new();
    for w in [32, 16] {
        let mut medians = BTreeMap::new();
        for label in ["control", "off", "on"] {
            let (mut ms, mut cpu, mut rss) = (vec![], vec![], vec![]);
            for block in 0..15 {
                let (s, t) = samples(&format!("b{block}-w{w}-{label}"));
                assert_eq!(s.len(), 151);
                assert_eq!(t["workers"], w);
                assert_eq!(t["priming"], 3);
                ms.push(median(&s));
                cpu.push(
                    (t["user_cpu_ms"].as_f64().unwrap() + t["system_cpu_ms"].as_f64().unwrap())
                        / 151.,
                );
                rss.push(t["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
            }
            let v = json!({"median_ms":median(&ms),"cpu_ms_per_request":median(&cpu),"median_process_peak_RSS_MiB":median(&rss),"max_process_peak_RSS_MiB":rss.iter().copied().fold(0.,f64::max),"block_medians":ms});
            writeln!(
                table,
                "| {w} | {label} | {:.6} | {:.6} | {:.2} | {:.2} |",
                v["median_ms"].as_f64().unwrap(),
                v["cpu_ms_per_request"].as_f64().unwrap(),
                v["median_process_peak_RSS_MiB"].as_f64().unwrap(),
                v["max_process_peak_RSS_MiB"].as_f64().unwrap()
            )
            .unwrap();
            cells.insert(format!("w{w}/{label}"), v);
            medians.insert(label, ms);
        }
        for (a, b) in [("off", "control"), ("on", "control"), ("on", "off")] {
            let ratios: Vec<_> = medians[a]
                .iter()
                .zip(&medians[b])
                .map(|(a, b)| a / b)
                .collect();
            let v = interval(&ratios);
            let pass = if a == "off" {
                v["low"].as_f64().unwrap() >= 1. / 1.03 && v["high"].as_f64().unwrap() <= 1.03
            } else if w == 32 {
                v["high"].as_f64().unwrap() < 1. && v["wins"].as_u64().unwrap() >= 11
            } else {
                v["high"].as_f64().unwrap() <= 1.02
            };
            let key = format!("w{w}/{a}/{b}");
            if !pass {
                failed.push(key.clone())
            }
            writeln!(
                pair_table,
                "| {w} | {a}/{b} | {:.9} | [{:.9}, {:.9}] | {}/15 | {} |",
                v["median"].as_f64().unwrap(),
                v["low"].as_f64().unwrap(),
                v["high"].as_f64().unwrap(),
                v["wins"],
                if pass { "pass" } else { "fail" }
            )
            .unwrap();
            pairs.insert(key, v);
        }
    }
    let mut profiles = BTreeMap::new();
    let mut raw = BTreeMap::new();
    let mut profile_table = String::new();
    for w in [32, 16] {
        for label in ["control", "off", "on"] {
            let name = format!("profile-w{w}-{label}");
            let (s, summary) = samples(&name);
            assert_eq!(s.len(), 251);
            assert_eq!(summary["workers"], w);
            assert!(
                !fs::read_to_string(r.join(format!("runs/loss-{name}/stdout.log")))
                    .unwrap()
                    .contains("PERF_RECORD_LOST")
            );
            let text =
                fs::read_to_string(r.join(format!("runs/decode-{name}/stdout.log"))).unwrap();
            let mut pids = BTreeSet::new();
            let (mut n, mut period) = (0u64, 0u64);
            let mut symbols = BTreeMap::<String, u64>::new();
            for line in text.lines().filter(|s| !s.trim_start().starts_with('#')) {
                let f: Vec<_> = line.split_whitespace().collect();
                if f.len() < 4 {
                    continue;
                }
                let fields = if let Some((p, _)) = f[0].split_once('/') {
                    p.parse::<u64>()
                        .ok()
                        .zip(f[1].parse::<u64>().ok())
                        .map(|(p, t)| (p, t, 3))
                } else if f.len() >= 5 {
                    f[0].parse::<u64>()
                        .ok()
                        .zip(f[2].parse::<u64>().ok())
                        .map(|(p, t)| (p, t, 4))
                } else {
                    None
                };
                if let Some((p, t, start)) = fields {
                    pids.insert(p);
                    n += 1;
                    period += t;
                    let sym = format!("{:#}", rustc_demangle::demangle(&f[start..].join(" ")));
                    *symbols.entry(sym).or_default() += t;
                }
            }
            assert!(n > 0);
            assert_eq!(pids.len(), 1);
            let fused: u64 = symbols
                .iter()
                .filter(|(s, _)| s.contains("euler_output::consume_columns"))
                .map(|(_, n)| n)
                .sum();
            writeln!(
                profile_table,
                "| {w} | {label} | {n} | 0 | {:.3}% |",
                100. * fused as f64 / period as f64
            )
            .unwrap();
            raw.insert(
                r.join("profiles")
                    .join(&name)
                    .join("perf.data")
                    .display()
                    .to_string(),
                hash(r.join("profiles").join(&name).join("perf.data")),
            );
            profiles.insert(name,json!({"samples":n,"lost":0,"pids":pids,"period":period,"fused_symbol_period":fused,"demangled_symbols":symbols,"summary":summary}));
        }
    }
    let jobs = read(r.join("jobs.json"));
    assert_eq!(jobs.as_array().unwrap().len(), 99);
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(r.join("prior-proof-sha256.json"))).unwrap();
    assert_eq!(seen.len(), 12686);
    let mut inv = BTreeMap::new();
    let mut requests = 0u64;
    for job in jobs.as_array().unwrap() {
        let name = job["name"].as_str().unwrap();
        let (_, summary) = samples(name);
        assert_eq!(summary["rounds"], job["rounds"]);
        assert_eq!(summary["workers"], job["workers"]);
        requests += job["rounds"].as_u64().unwrap() + 3;
        let dir = Path::new(job["directory"].as_str().unwrap());
        for ext in ["proof", "public"] {
            let p = dir.join(format!("0.{ext}"));
            let h = hash(&p);
            if ext == "proof" {
                assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                assert!(seen.insert(h.clone()));
            } else {
                assert_eq!(h, PH)
            }
            inv.insert(p.display().to_string(), h);
        }
    }
    assert_eq!(requests, 15396);
    assert_eq!(seen.len(), 12785);
    save(r.join("proof-inventory.json"), &json!(inv));
    let gate = failed.is_empty();
    let status = if gate {
        "passed primary SDK screen; broader qualification required"
    } else {
        "rejected primary SDK screen"
    };
    let analysis = json!({"status":status,"gate_passed":gate,"failed_checks":failed,"cells":cells,"pairs":pairs,"profiles":profiles,"raw_profile_pins":raw,"SDK_requests":requests,"new_independent_exports":99,"independent_proof_count":12785,"new_native_tests":3,"core_tests_per_configuration":122,"source_files":172,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"original8_CPU10x_rows":0,"capture_pins":pins.as_object().unwrap().len(),"scope":"primary1x2 only; no new all8/54/resource/portable/mobile/TEE/AWS qualification"});
    save(r.join("analysis.json"), &analysis);
    let manifest=read("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json");
    let mut refs = String::new();
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let v = &manifest["m5"][shape];
        writeln!(
            refs,
            "| {shape} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    let report = format!(
        r#"# Consume Euler output in the final FFT columns

The candidate **{status}**. No source adoption or global-goal achievement is claimed. All positive estimates, calibration failures and regressions remain in the tables.

The [compaction trial](../2026-10-28-euler-compact/README.md) rejected checked lane gathers. Its faster old-arithmetic tile control prompted a history audit: the [earlier FFT-pass-grain trial](../2026-10-26-fft-pass-grain/README.md) already tested a related 128-row splitting floor and failed its full gate. This experiment changes dataflow rather than rerunning a grain parameter.

The existing Euler forward transform completes its same 32 child transforms. The new terminal helper expands two adjacent final columns, runs the unchanged column butterflies, and computes the accepted eight-row `DA*B+A*DB` directly from those results. It writes A without packing final derivative values back into the temporary vector and reading them in a separate output pass. Safe disjoint output stripes preserve prefixes and zero-padded B tails without extending A. There is no new unsafe code, field formula, key table, execution plan or input-sized buffer. Two 32-Pair columns require roughly 20 KiB of explicit arithmetic scratch per active terminal call, versus the old single-column 10 KiB; compiler frames are additional. The never-inline terminal helper keeps scratch outside recursive iterator frames. Domains at most 4096 fields, unsupported output mode and OFF preserve the old path. The private process flag defaults OFF.

Only one of 172 original source files changes reversibly, plus one new module. Both OFF and ON pass the complete 122-test native core suite serially, avoiding the previously observed global allocator-test interference. Three new cases check full-width scalar-product oracles after the retained forward transform across four domains, cached/uncached roots, short prefixes/B extents; padded output row conservation; and concurrent shared-root callers with one/two/four-worker pools and 512 KiB stacks. Existing independent polynomial derivative tests also pass. Raw-image two and cubic six prover tests pass with explicit required names. Field source and its qualification are inherited; no field-suite rerun is claimed.

Fifteen fixed rotated/reversed blocks compare the accepted SDK and the same fresh binary OFF/ON at 32 logical and 16 physical workers. Each latency process performs three primes then 151 measured verified SDK requests. Three four-worker preflights precede latency; ON confirms four actual `n=32768 rows=30922` fusion entries. Key loading is excluded; fresh solving, randomness, proving, SDK verification and serialization are included.

| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |
|---|---|---:|---:|---:|---:|
{table}
| Workers | Paired ratio | Median | Conditional interval | Wins | Check |
|---|---|---:|---:|---:|---|
{pair_table}
Ratios pair the 15 process medians. The 4th–12th ordered bounds have nominal 96.484375% coverage under independent exchangeable blocks, without a familywise guarantee. OFF/accepted calibration must fit [1/1.03,1.03]. Primary ON requires upper ratio below one and at least 11 wins against both controls; physical-16 upper ratios must be at most 1.02. There is no gain floor, exclusion, extension or unchanged reroll.

All 90 latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles with 251 requests after three primes. All 99 SDK captures precede independent Gnark verification, profile decoding, hashes and archive work. No owned helper, build or other CPU benchmark overlaps capture. Every profile has one sampled SDK PID and zero lost records; demangled symbols and raw profile hashes are retained. Fused-symbol share includes butterfly and output work and is not a removable wall-time fraction.

| Workers | Variant | Samples | Lost | Fused-column symbol period share |
|---|---|---:|---:|---:|
{profile_table}
All 15,396 SDK calls verify internally. The 99 exported fresh 164-byte proofs independently pass Gnark, public binding and disjointness against 12,686 prior identities; ledger 12,785. Full pins and every exported member are checked. RSS includes loading and primes and is not minimum RAM or memory-pressure qualification. Production and accepted SDK remain unchanged. No new all-eight/54, portable/mobile/TEE, concurrent whole-proof, pressure, large-circuit or AWS result is claimed. A positive primary screen still requires those broader applicable checks before adoption.

The original all-eight 10× M5 CPU objective remains active and unmet (0/8). The accepted [FFT adoption](../2026-10-27-clean-fft-adoption/README.md) retains its engineering exception and false full-performance/qualified-scope gates. These accepted local comparisons are inherited, not measurements from this primary-only experiment.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
{refs}
Accepted primary peak RSS remains historically 134.50 MiB. Merge 54→1 remains 161.341835 ms / 1,910.1875 MiB, with no supplied M5 reference. The latest AWS/Aeglos comparison predates the current accepted FFT source; no new cloud claim follows from this experiment.

[Protocol](artifacts/design.md), [analysis](artifacts/analysis.json), [proof inventory](artifacts/proof-inventory.json), [build](artifacts/native/result.json), [source audit](artifacts/source-audit.json).
"#
    );
    fs::create_dir_all(format!("{D}/artifacts")).unwrap();
    fs::write(format!("{D}/README.md"), report).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &Path::new(D).join("artifacts"), &mut index);
    for f in [
        "crates/zopro-core/src/fft/pair.rs",
        "crates/zopro-core/src/fft/pair/euler_output.rs",
    ] {
        let p = Path::new(D).join("artifacts/private-source").join(f);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(r.join("source").join(f), &p).unwrap();
        index.insert(format!("artifacts/private-source/{f}"), hash(p));
    }
    index.insert("README.md".into(), hash(format!("{D}/README.md")));
    let sums = index
        .iter()
        .map(|(p, h)| format!("{h}  {p}\n"))
        .collect::<String>();
    fs::write(format!("{D}/SHA256SUMS"), sums).unwrap();
    println!("Closed {status}; {} hashes; ledger12785", index.len());
}
