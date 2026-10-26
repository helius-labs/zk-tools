use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, format!("{}\n", serde_json::to_string_pretty(v).unwrap())).unwrap()
}
fn num(v: &Value, key: &str) -> f64 {
    v[key].as_f64().unwrap()
}
fn txt<'a>(v: &'a Value, key: &str) -> &'a str {
    v[key].as_str().unwrap()
}
fn median(mut v: Vec<f64>) -> f64 {
    assert!(!v.is_empty() && v.iter().all(|n| n.is_finite()));
    v.sort_by(f64::total_cmp);
    let m = v.len() / 2;
    if v.len() % 2 == 0 {
        (v[m - 1] + v[m]) / 2.
    } else {
        v[m]
    }
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn output(data: &Path, p: &str) -> PathBuf {
    data.join(Path::new(p).strip_prefix("/output").unwrap())
}
fn success(p: impl AsRef<Path>) {
    assert_eq!(read(p)["success"], true)
}
fn proof_files(dir: &Path, out: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(dir).unwrap() {
        let p = entry.unwrap().path();
        if p.is_dir() {
            proof_files(&p, out)
        } else if p.extension().is_some_and(|e| e == "proof") {
            out.push(p)
        }
    }
}
struct Audit {
    prior: BTreeSet<String>,
    seen: BTreeMap<String, String>,
    inventory: BTreeMap<String, String>,
}
impl Audit {
    fn new(context: &Path) -> Self {
        Self {
            prior: serde_json::from_value(read(context.join("prior-proofs.json"))).unwrap(),
            seen: BTreeMap::new(),
            inventory: BTreeMap::new(),
        }
    }
    fn export(&mut self, data: &Path, dir: &Path, public: &str) {
        let mut files = Vec::new();
        proof_files(dir, &mut files);
        assert_eq!(files.len(), 1, "one proof per process: {}", dir.display());
        let p = &files[0];
        let public_path = p.with_extension("public");
        assert_eq!(hash(&public_path), public, "fixture binding");
        let digest = hash(p);
        assert!(
            !self.prior.contains(&digest),
            "proof repeats a prior export"
        );
        let name = p.strip_prefix(data).unwrap().to_str().unwrap().to_owned();
        assert!(
            self.seen.insert(digest.clone(), name.clone()).is_none(),
            "duplicate proof"
        );
        self.inventory.insert(name, digest);
        self.inventory.insert(
            public_path
                .strip_prefix(data)
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned(),
            hash(public_path),
        );
    }
}
fn sdk(dir: &Path, rounds: usize, workers: usize, aeglos: bool) -> (Vec<f64>, Value) {
    success(dir.join("exit.json"));
    let text = fs::read_to_string(dir.join("stdout.log")).unwrap();
    let samples = text
        .lines()
        .filter_map(|l| l.strip_prefix("SAMPLE "))
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect::<Vec<_>>();
    let summaries = text
        .lines()
        .filter_map(|l| l.strip_prefix("SUMMARY "))
        .map(|l| serde_json::from_str::<Value>(l).unwrap())
        .collect::<Vec<_>>();
    assert_eq!(samples.len(), rounds);
    assert_eq!(summaries.len(), 1);
    let summary = summaries[0].clone();
    assert_eq!(summary["rounds"], rounds);
    assert_eq!(summary["priming"], 3);
    assert_eq!(summary[if aeglos { "threads" } else { "workers" }], workers);
    let times = samples
        .iter()
        .enumerate()
        .map(|(i, s)| {
            assert_eq!(s["round"], i);
            if aeglos {
                assert_eq!(s["cache_hit"], true);
            }
            let t = s["ms"]
                .as_f64()
                .or_else(|| s["verified_ms"].as_f64())
                .unwrap();
            assert!(t > 0. && t.is_finite());
            t
        })
        .collect();
    (times, summary)
}
fn preflight(context: &Path, data: &Path, audit: &mut Audit) -> Value {
    let manifest = read(context.join("preflight-jobs.json"));
    let jobs = manifest["jobs"].as_array().unwrap();
    assert_eq!(jobs.len(), 7);
    let mut requests = 0;
    for job in jobs {
        let name = txt(job, "name");
        let dir = data.join("preflight").join(name);
        let _ = sdk(&dir, 1, 4, job["engine"] == "aeglos");
        success(
            data.join("preflight")
                .join(format!("verify-{name}"))
                .join("exit.json"),
        );
        audit.export(
            data,
            &output(data, txt(job, "proof_dir")),
            txt(job, "public_sha256"),
        );
        requests += 4;
    }
    assert_eq!(requests, 28);
    json!({"status":"preflight verified","sdk":requests,"exports":7,"public_binding":true,"fresh_proofs":true})
}
fn counters(text: &str) -> BTreeMap<String, f64> {
    let mut result = BTreeMap::new();
    for line in text
        .lines()
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
    {
        let f = line.split(';').collect::<Vec<_>>();
        if f.len() < 5 {
            continue;
        }
        let event = f[2].trim();
        if ![
            "cycles:u",
            "instructions:u",
            "branches:u",
            "branch-misses:u",
        ]
        .contains(&event)
        {
            continue;
        }
        let value = f[0].trim().parse::<f64>().unwrap();
        let running = f[4].trim().trim_end_matches('%').parse::<f64>().unwrap();
        assert!(value.is_finite() && value >= 0.);
        assert!(
            (99.0..=100.0).contains(&running),
            "counter running percent {running}"
        );
        assert!(result.insert(event.to_owned(), value).is_none());
    }
    assert_eq!(result.len(), 4);
    assert!(result["cycles:u"] > 0. && result["instructions:u"] > 0. && result["branches:u"] > 0.);
    result
}
fn main() {
    let args = std::env::args().skip(1).collect::<Vec<_>>();
    assert!(args.len() == 2 || args.len() == 3);
    let context = PathBuf::from(
        std::env::var_os("ZOPRO_REVIEW_CONTEXT").unwrap_or_else(|| "/experiment".into()),
    );
    let data = Path::new(&args[1]);
    let mut audit = Audit::new(&context);
    let pref = preflight(&context, data, &mut audit);
    if args[0] == "preflight" {
        assert_eq!(args.len(), 2);
        println!("{}", serde_json::to_string_pretty(&pref).unwrap());
        return;
    }
    assert_eq!(args[0], "all");
    assert_eq!(args.len(), 3);
    let dest = Path::new(&args[2]);
    fs::create_dir(dest).unwrap();
    assert_eq!(
        read(data.join("preflight/result.json"))["status"],
        "complete"
    );
    let perf = read(data.join("perf/result.json"));
    assert_eq!(perf["status"], "complete");
    assert_eq!(perf["profiles"], 18);
    assert_eq!(perf["distinct_verified_exports"], 72);
    let rows = read(data.join("perf/rows.json"));
    let rows = rows.as_array().unwrap();
    assert_eq!(rows.len(), 72);
    let protocol = read(context.join("perf.json"));
    let cases = protocol["cases"].as_array().unwrap();
    let mut phase_sdk = 0usize;
    let mut samples = 0u64;
    let mut row_names = BTreeSet::new();
    for row in rows {
        let case = cases.iter().find(|c| c["name"] == row["case"]).unwrap();
        let phase = txt(row, "phase");
        assert!(["latency", "profile"].contains(&phase));
        let n = if phase == "latency" {
            case["rounds"].as_u64().unwrap()
        } else {
            case["profile_rounds"].as_u64().unwrap()
        };
        assert_eq!(row["samples_ms"].as_array().unwrap().len() as u64, n);
        assert_eq!(row["summary"]["workers"], case["workers"]);
        phase_sdk += n as usize + 3;
        let block = row["block"].as_u64().unwrap();
        assert!(block < if phase == "latency" { 3 } else { 1 });
        assert!(["accepted", "off", "on"].contains(&txt(row, "variant")));
        let dir = format!(
            "perf/{phase}/b{block}/{}/{}",
            txt(row, "case"),
            txt(row, "variant")
        );
        assert!(row_names.insert(dir.clone()));
        success(data.join(&dir).join("verify/exit.json"));
        audit.export(
            data,
            &data.join(&dir).join("proofs"),
            txt(case, "public_sha256"),
        );
        if phase == "profile" {
            assert_eq!(row["profile"]["lost_records"], 0);
            assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
            samples += row["profile"]["samples"].as_u64().unwrap();
        }
    }
    assert_eq!(phase_sdk, 2286);
    let latency = rows
        .iter()
        .filter(|r| r["phase"] == "latency")
        .collect::<Vec<_>>();
    assert_eq!(latency.len(), 54);
    let mut gate = BTreeMap::new();
    for workers in [32, 64, 96] {
        let mut failures = Vec::new();
        let mut values = BTreeMap::new();
        for shape in ["1x2", "merge-54x1"] {
            let case = format!("{shape}-w{workers}");
            let get = |variant: &str, b: usize| -> f64 {
                let selected = latency
                    .iter()
                    .filter(|r| r["case"] == case && r["variant"] == variant && r["block"] == b)
                    .collect::<Vec<_>>();
                assert_eq!(selected.len(), 1);
                num(selected[0], "median_ms")
            };
            let mut comparisons = BTreeMap::new();
            for control in ["accepted", "off"] {
                let ratios = (0..3)
                    .map(|b| get("on", b) / get(control, b))
                    .collect::<Vec<_>>();
                let ratio = median(ratios.clone());
                let wins = ratios.iter().filter(|r| **r < 1.).count();
                let pass = if shape == "1x2" {
                    ratio < 1. && wins >= 2
                } else {
                    ratio <= 1.03
                };
                if !pass || ratios.iter().any(|r| *r > 1.05) {
                    failures.push(format!("{case}/{control}: ratio{ratio}, wins{wins}"));
                }
                comparisons.insert(
                    control,
                    json!({"paired_median":ratio,"wins":wins,"block_ratios":ratios}),
                );
            }
            let calibration = median((0..3).map(|b| get("off", b) / get("accepted", b)).collect());
            let factor = calibration.max(1. / calibration);
            if factor > 1.03 {
                failures.push(format!("{case} OFF calibration {factor}"));
            }
            values.insert(
                shape,
                json!({"on_over_control":comparisons,"off_calibration_factor":factor}),
            );
        }
        gate.insert(
            workers,
            json!({"passed":failures.is_empty(),"failed_checks":failures,"cases":values}),
        );
    }
    for folder in ["comparison-latency", "comparison-stat"] {
        assert_eq!(
            read(data.join(folder).join("result.json"))["status"],
            "complete"
        );
    }
    let manifest = read(context.join("comparison-jobs.json"));
    let jobs = manifest["jobs"].as_array().unwrap();
    assert_eq!(jobs.len(), 72);
    let mut observations = Vec::new();
    let mut comparison_sdk = 0usize;
    let mut cells = BTreeSet::new();
    for job in jobs {
        let name = txt(job, "name");
        let folder = txt(job, "output");
        let rounds = job["rounds"].as_u64().unwrap() as usize;
        let workers = job["workers"].as_u64().unwrap() as usize;
        let engine = txt(job, "engine");
        let (times, s) = sdk(
            &data.join(folder).join(name),
            rounds,
            workers,
            engine == "aeglos",
        );
        success(
            data.join(folder)
                .join(format!("verify-{name}"))
                .join("exit.json"),
        );
        audit.export(
            data,
            &output(data, txt(job, "proof_dir")),
            txt(job, "public_sha256"),
        );
        comparison_sdk += rounds + 3;
        assert!(cells.insert((
            job["kind"].to_string(),
            job["block"].to_string(),
            workers,
            engine.to_string()
        )));
        let cpu = num(&s, "user_cpu_ms") + num(&s, "system_cpu_ms");
        let mut observation = json!({"name":name,"kind":job["kind"],"block":job["block"],"workers":workers,"engine":engine,"rounds":rounds,"samples_ms":times,"median_ms":median(times),"cpu_ms_per_proof":cpu/rounds as f64,"user_cpu_ms_per_proof":num(&s,"user_cpu_ms")/rounds as f64,"system_cpu_ms_per_proof":num(&s,"system_cpu_ms")/rounds as f64,"voluntary_switches_per_proof":num(&s,"voluntary_switches")/rounds as f64,"maxrss_mib":num(&s,"whole_process_maxrss_kib")/1024.,"mean_occupied_cpus":cpu/num(&s,"loop_wall_ms")});
        if job["kind"] == "stat" {
            let c = counters(
                &fs::read_to_string(data.join("counters").join(name).join("stat.csv")).unwrap(),
            );
            observation["ipc"] = json!(c["instructions:u"] / c["cycles:u"]);
            observation["cycles_per_proof"] = json!(c["cycles:u"] / rounds as f64);
            observation["instructions_per_proof"] = json!(c["instructions:u"] / rounds as f64);
            observation["estimated_user_ghz"] =
                json!(c["cycles:u"] / (num(&s, "user_cpu_ms") * 1e6));
            observation["counters"] = json!(c);
        }
        observations.push(observation);
    }
    assert_eq!(comparison_sdk, 8568);
    assert_eq!(audit.seen.len(), 151);
    assert_eq!(audit.inventory.len(), 302);
    let mut comparisons = BTreeMap::new();
    for workers in [32, 64, 96] {
        let mut engines = BTreeMap::new();
        for engine in ["accepted", "off", "on", "aeglos"] {
            let select = |kind: &str| {
                observations
                    .iter()
                    .filter(|o| {
                        o["workers"] == workers && o["engine"] == engine && o["kind"] == kind
                    })
                    .collect::<Vec<_>>()
            };
            let lat = select("latency");
            let stat = select("stat");
            assert_eq!(lat.len(), 3);
            assert_eq!(stat.len(), 3);
            let mut metrics = BTreeMap::new();
            metrics.insert(
                "latency_ms",
                json!(median(lat.iter().map(|r| num(r, "median_ms")).collect())),
            );
            metrics.insert(
                "latency_maxrss_mib",
                json!(
                    lat.iter()
                        .map(|r| num(r, "maxrss_mib"))
                        .max_by(f64::total_cmp)
                        .unwrap()
                ),
            );
            for field in [
                "cpu_ms_per_proof",
                "user_cpu_ms_per_proof",
                "system_cpu_ms_per_proof",
                "voluntary_switches_per_proof",
                "mean_occupied_cpus",
                "ipc",
                "cycles_per_proof",
                "instructions_per_proof",
                "estimated_user_ghz",
            ] {
                metrics.insert(
                    field,
                    json!(median(stat.iter().map(|r| num(r, field)).collect())),
                );
            }
            engines.insert(engine, json!(metrics));
        }
        comparisons.insert(workers, json!(engines));
    }
    let result = json!({"status":"complete and verified","adopted":false,"goal_achieved":false,"sdk":phase_sdk+comparison_sdk+28,"distinct_exports":151,"proof_public_members":302,"profiles":18,"profile_samples":samples,"lost_records":0,"target_gates":gate,"main_latency":perf["latency"],"profile_comparison":perf["profile_comparison"],"fresh_comparison":comparisons,"scope":"Warm complete SDK requests on one C7a24; no TEE/phone/large-circuit/minimumRAM or all-nine qualification. CPU shares are not removable wall time; occupied cores include spinning. EstimatedGHz derives from cycles/userCPU. No adoption."});
    save(dest.join("result.json"), &result);
    save(dest.join("comparison-rows.json"), &json!(observations));
    save(dest.join("proof-inventory.json"), &json!(audit.inventory));
    println!("REVIEW_VERIFIED SDK10882 EXPORTS151 PROFILES18 SAMPLES{samples}");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn counter_fields_require_complete_nonmultiplexed_set() {
        let text = "100;;cycles:u;1;100.00;;\n150;;instructions:u;1;100.00;;\n20;;branches:u;1;100.00;;\n0;;branch-misses:u;1;100.00;;\n";
        assert_eq!(counters(text)["branch-misses:u"], 0.);
        assert!(std::panic::catch_unwind(|| counters(&text.replace("100.00", "98.00"))).is_err());
        assert!(std::panic::catch_unwind(|| counters("100;;cycles:u;1;100.00;;\n")).is_err());
    }
    #[test]
    fn block_medians_reject_nonfinite_and_use_every_process() {
        assert_eq!(median(vec![3., 1., 2.]), 2.);
        assert_eq!(median(vec![2., 4.]), 3.);
        assert!(std::panic::catch_unwind(|| median(vec![f64::NAN])).is_err());
    }
}
