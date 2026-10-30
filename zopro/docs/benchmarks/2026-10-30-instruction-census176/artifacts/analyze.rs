use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::Path,
};

const R: &str = "/home/ubuntu/src/zopro-perf-work/instruction-census176";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn add(map: &mut BTreeMap<String, (u64, u64)>, key: &str, n: u64, period: u64) {
    let entry = map.entry(key.into()).or_default();
    entry.0 += n;
    entry.1 += period;
}
fn verify_bins(actual: &BTreeMap<String, (u64, u64)>, expected: &Value) {
    assert_eq!(actual.len(), expected.as_object().unwrap().len());
    for (key, (n, p)) in actual {
        assert_eq!(expected[key]["samples"], *n);
        assert_eq!(expected[key]["period"], *p);
    }
}
fn selected(names: &Value) -> Option<&'static str> {
    let names = names
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap())
        .collect::<Vec<_>>()
        .join(" ");
    if names.contains("::msm::stream_g1_impl") {
        Some("G1 stream")
    } else if names.contains("::msm::g2_borrowed::apply") {
        Some("G2 borrowed add")
    } else if names.contains("::msm::reduce_layer") && names.contains("G1Config") {
        Some("G1 tree reduce")
    } else if names.contains("::msm::reduce_layer") && names.contains("G2Config") {
        Some("G2 tree reduce")
    } else if names.contains("crossbeam_epoch::default::with_handle") {
        Some("Epoch pin context")
    } else {
        None
    }
}
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    assert!(!r.join("findings.json").exists());
    let config = read(r.join("analysis-config.json"));
    let base = read(p.join("analysis.json"));
    let manifest_path=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let manifest = read(&manifest_path);
    for (f, h) in config["pins"].as_object().unwrap() {
        assert_eq!(hash(f), h.as_str().unwrap());
    }
    for (f, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        manifest["source_files_sha256"].as_object().unwrap().len(),
        176
    );
    let mut total_n = 0;
    let mut total_p = 0;
    let mut all_runs = BTreeMap::new();
    for run in config["runs"].as_array().unwrap() {
        let name = run["name"].as_str().unwrap();
        let data = read(r.join(format!("analysis/{name}.json")));
        let workers = if name.contains("w16") { 16 } else { 32 };
        let variant = if run["binary"] == "current" {
            "ON"
        } else {
            "accepted"
        };
        let capture = &base["profiles"][format!("cycles-1x2-w{workers}-{variant}")];
        let mut raw = BTreeMap::<u64, (u64, u64)>::new();
        let mut tids = BTreeSet::new();
        for line in fs::read_to_string(run["samples"].as_str().unwrap())
            .unwrap()
            .lines()
            .map(str::trim)
            .filter(|l| !l.is_empty() && !l.starts_with('#'))
        {
            let fields = line.split_whitespace().collect::<Vec<_>>();
            let (pid, tid) = fields[0].split_once('/').unwrap();
            assert_eq!(
                pid.parse::<u64>().unwrap(),
                run["expected_pid"].as_u64().unwrap()
            );
            tids.insert(tid.parse::<u64>().unwrap());
            let period = fields[1].parse::<u64>().unwrap();
            assert_eq!(period, 16_000_000);
            let ip = u64::from_str_radix(fields[2].trim_start_matches("0x"), 16).unwrap();
            let v = raw.entry(ip).or_default();
            v.0 += 1;
            v.1 += period;
        }
        let expected_tids = capture["TIDs"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect::<BTreeSet<_>>();
        assert_eq!(tids, expected_tids);
        let mut family = BTreeMap::new();
        let mut memory = BTreeMap::new();
        let mut scopes = BTreeMap::new();
        let mut functions = BTreeMap::new();
        let mut executable = (0, 0);
        let mut outside = (0, 0);
        let mut observed = BTreeSet::new();
        for line in fs::read_to_string(r.join(format!("analysis/{name}-ips.jsonl")))
            .unwrap()
            .lines()
        {
            let row: Value = serde_json::from_str(line).unwrap();
            let ip = u64::from_str_radix(row["runtime_ip"].as_str().unwrap(), 16).unwrap();
            assert!(observed.insert(ip));
            let (n, period) = raw[&ip];
            assert_eq!(row["tally"]["samples"], n);
            assert_eq!(row["tally"]["period"], period);
            add(&mut family, row["family"].as_str().unwrap(), n, period);
            add(&mut memory, row["memory"].as_str().unwrap(), n, period);
            add(&mut scopes, row["scope"].as_str().unwrap(), n, period);
            if let Some(elf) = row["elf_ip"].as_str() {
                let elf = u64::from_str_radix(elf, 16).unwrap();
                let bias = u64::from_str_radix(data["summary"]["load_bias"].as_str().unwrap(), 16)
                    .unwrap();
                assert_eq!(ip - bias, elf);
                assert!(row["assembly"].as_str().is_some_and(|s| !s.is_empty()));
                let index = row["function"].as_u64().unwrap();
                add(&mut functions, &index.to_string(), n, period);
                executable.0 += n;
                executable.1 += period;
            } else {
                assert_eq!(row["family"], "outside_executable");
                outside.0 += n;
                outside.1 += period;
            }
        }
        assert_eq!(observed, raw.keys().copied().collect());
        verify_bins(&family, &data["all"]["families"]);
        verify_bins(&memory, &data["all"]["memory"]);
        for (scope, (n, period)) in scopes {
            assert_eq!(data["scopes"][&scope]["total"]["samples"], n);
            assert_eq!(data["scopes"][&scope]["total"]["period"], period);
        }
        for (index, (n, period)) in functions {
            let f = &data["functions"][index.parse::<usize>().unwrap()];
            assert_eq!(f["bins"]["total"]["samples"], n);
            assert_eq!(f["bins"]["total"]["period"], period);
        }
        let n = raw.values().map(|v| v.0).sum::<u64>();
        let period = raw.values().map(|v| v.1).sum::<u64>();
        assert_eq!(data["summary"]["total"]["samples"], n);
        assert_eq!(data["summary"]["total"]["period"], period);
        assert_eq!(n, run["expected_samples"].as_u64().unwrap());
        assert_eq!(period, run["expected_period"].as_u64().unwrap());
        assert_eq!(executable.0 + outside.0, n);
        assert_eq!(executable.1 + outside.1, period);
        let mut contexts = BTreeMap::new();
        for f in data["functions"].as_array().unwrap() {
            if let Some(label) = selected(&f["names"]) {
                assert!(!contexts.contains_key(label));
                let b = &f["bins"];
                let count = b["total"]["samples"].as_u64().unwrap();
                let ifma = b["families"]["ifma"]["samples"].as_u64().unwrap_or(0);
                let scalar = b["families"]["scalar_move"]["samples"]
                    .as_u64()
                    .unwrap_or(0);
                let vector = b["families"]["vector_move"]["samples"]
                    .as_u64()
                    .unwrap_or(0);
                let rsp = b["memory"]["rsp_operand"]["samples"].as_u64().unwrap_or(0);
                contexts.insert(label,json!({"names":f["names"],"bins":b,"samples":count,"share_of_all":count as f64/n as f64,"ifma_within":ifma as f64/count as f64,"scalar_move_within":scalar as f64/count as f64,"vector_move_within":vector as f64/count as f64,"rsp_within":rsp as f64/count as f64}));
            }
        }
        assert_eq!(contexts.len(), 5);
        all_runs.insert(name.to_string(),json!({"summary":data["summary"],"all_instruction_bins":data["all"],"selected_contexts":contexts}));
        total_n += n;
        total_p += period;
    }
    assert_eq!(total_n, 42495);
    assert_eq!(total_p, 679920000000);
    let counters = base["counters"]
        .as_object()
        .unwrap()
        .iter()
        .filter(|(k, _)| k.ends_with("-ON"))
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect::<BTreeMap<_, _>>();
    let findings = json!({"complete":true,"diagnostic_only":true,"source_files":176,"production_changed":false,"adopted_new_optimization":false,"new_proofs_or_timings_or_profiles_or_tests":0,"proof_ledger":27353,"samples":total_n,"period":total_p,"runs":all_runs,"inherited_current_counter_panels":counters,"all_current_local_rows":manifest["ms"],"all_eight_M5_comparisons":manifest["m5"],"goal_achieved":false,"original8_CPU10x_rows":0,"next_hypothesis":"Investigate direct output storage of canonical Fq2 coordinates in generic G2 tree reduction, keeping input packing, arithmetic, pair scheduling and topology unchanged. Audit against the rejected joint G1 input/output transpose and existing staged/outlined kernels before building. No native gain or sufficient 2x headroom is established.","interpretation":"Exact byte/IP/count attribution; sampled opcode and memory tags are overlapping observations subject to skid/inlining, not removable wall-time fractions, retired instruction counts, measured spills, memory traffic or a causal stall model."});
    save(r.join("findings.json"), &findings);
    save(
        r.join("independent-review.json"),
        &json!({"passed":true,"raw_samples_to_IPs_and_bins_independently_recomputed":true,"PID_and_actual_TID_sets_match_qualified_capture":true,"four_fixed_period_capture_totals_conserved":true,"all_input_hashes_and_source176_unchanged":true,"samples":total_n,"period":total_p,"production_changed":false,"new_proof_profile_timing_test_runs":0,"goal_achieved":false}),
    );
    println!("Independent offline audit PASS: four captures, 42495 samples, 679920000000 period; exact raw/IP/family/memory/scope/function conservation; source176 unchanged");
}
