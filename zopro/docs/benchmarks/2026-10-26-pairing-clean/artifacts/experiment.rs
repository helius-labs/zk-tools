use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/tmp/zopro-pairing-clean";
const TOOL: &str = "/tmp/zopro-xtask-fast-screen-target/release/xtask";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    pins.insert(p.display().to_string(), hash(p));
}
fn median(mut v: Vec<f64>) -> f64 {
    v.sort_by(f64::total_cmp);
    let n = v.len();
    assert!(n > 0);
    if n % 2 == 1 {
        v[n / 2]
    } else {
        (v[n / 2 - 1] + v[n / 2]) / 2.
    }
}
fn gm(v: &[f64]) -> f64 {
    (v.iter().map(|x| x.ln()).sum::<f64>() / v.len() as f64).exp()
}
fn prior() -> BTreeSet<String> {
    let mut set: BTreeSet<String> = serde_json::from_value(
        read("/tmp/zopro-g2-tree-sdk/context/manifest.json")["prior_proof_sha256"].clone(),
    )
    .unwrap();
    assert_eq!(set.len(), 5958);
    for p in [
        "/tmp/zopro-g2-tree-sdk/proof-inventory.json",
        "/tmp/zopro-g1-quarter-current/proof-inventory.json",
        "/tmp/zopro-pairing-x-chain-sdk/proof-inventory.json",
        "/tmp/zopro-control-rescheduling/all-proof-inventory.json",
    ] {
        for (file, h) in read(p).as_object().unwrap() {
            assert_eq!(hash(file), h.as_str().unwrap());
            if file.ends_with(".proof") {
                assert!(set.insert(h.as_str().unwrap().into()));
            }
        }
    }
    assert_eq!(set.len(), 6814);
    set
}
fn setup() {
    let r = Path::new(R);
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    let mut pins = BTreeMap::new();
    let mut changed = Vec::new();
    for (rel, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let production = Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel);
        assert_eq!(hash(&production), h.as_str().unwrap());
        add(&production, &mut pins);
        let baseline = r.join("baseline").join(rel);
        assert_eq!(hash(&baseline), h.as_str().unwrap());
        add(&baseline, &mut pins);
        let candidate = r.join("candidate").join(rel);
        if hash(&candidate) != h.as_str().unwrap() {
            changed.push(rel.clone());
        }
        add(candidate, &mut pins);
    }
    assert_eq!(changed, vec!["crates/zopro-bn254/src/pairing.rs"]);
    for sub in ["baseline", "candidate"] {
        for rel in [
            "crates/zopro-bench/examples/cpu-sdk.rs",
            "crates/zopro-bench/examples/support/perf.rs",
        ] {
            let p = r.join(sub).join(rel);
            assert_eq!(
                hash(&p),
                hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel))
            );
            add(p, &mut pins);
        }
    }
    let chain = r.join("candidate/crates/zopro-bn254/src/pairing/x_chain.rs");
    let text = fs::read_to_string(&chain).unwrap();
    assert!(
        !text.contains("OnceLock")
            && !text.contains("std::env")
            && !text.contains("pub(super) fn final_exponentiation")
    );
    let integration =
        fs::read_to_string(r.join("candidate/crates/zopro-bn254/src/pairing.rs")).unwrap();
    assert!(
        !integration.contains("zopro_pairing_x_chain")
            && !integration.contains("OnceLock")
            && !integration.contains("std::env")
    );
    add(chain, &mut pins);
    for p in [
        r.join("experiment.rs"),
        r.join("design.md"),
        PathBuf::from(TOOL),
        PathBuf::from("/usr/bin/perf"),
    ] {
        add(p, &mut pins);
    }
    save(r.join("source-pins.json"), &json!(pins));
    save(
        r.join("implementation-audit.json"),
        &json!({"original_files":170,"unchanged_candidate_originals":169,"changed":changed,"new_module":"pairing/x_chain.rs","canonical_final_exponentiation_body_unchanged":true,"experimental_runtime_selector":false,"new_heap_scratch":false,"new_input_cache_or_point_table":false,"production_sources_unchanged":true,"adopted":false}),
    );
    let old = read("/tmp/zopro-pairing-x-chain-sdk/native-plan.json");
    let text = serde_json::to_string(&old)
        .unwrap()
        .replace("/tmp/zopro-pairing-x-chain-sdk", R)
        .replace(" --cfg zopro_pairing_x_chain", "");
    let mut plan: Value = serde_json::from_str(&text).unwrap();
    plan["source_pins"] = json!(r.join("source-pins.json"));
    plan["environment"]
        .as_object_mut()
        .unwrap()
        .remove("ZOPRO_CPU_PAIRING_X_CHAIN");
    let mut fresh = plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "on-libraries")
        .unwrap()
        .clone();
    fresh["name"] = json!("fresh-libraries");
    let text = serde_json::to_string(&fresh)
        .unwrap()
        .replace(&format!("{R}/candidate"), &format!("{R}/baseline"))
        .replace("target-sdk-on", "target-sdk-fresh");
    fresh = serde_json::from_str(&text).unwrap();
    let mut fresh_link = plan["steps"]
        .as_array()
        .unwrap()
        .iter()
        .find(|s| s["name"] == "on")
        .unwrap()
        .clone();
    fresh_link["name"] = json!("fresh");
    fresh_link["artifacts"] = json!(r.join("native/fresh-libraries/stdout.log"));
    fresh_link["driver"] = json!(r.join("baseline/crates/zopro-bench/examples/cpu-sdk.rs"));
    fresh_link["additional_sources"] =
        json!([r.join("baseline/crates/zopro-bench/examples/support/perf.rs")]);
    let steps = plan["steps"].as_array_mut().unwrap();
    steps.insert(2, fresh);
    steps.insert(3, fresh_link);
    for s in steps {
        if s["name"] == "on" {
            s["name"] = json!("clean");
        } else if s["name"] == "on-core" {
            s["name"] = json!("core");
        } else if s["name"] == "on-prover" {
            s["name"] = json!("prover");
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!(
        "Pinned {} source inputs; separate fresh and clean native consumers",
        pins.len()
    );
}
fn prepare() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("native/field/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        35
    );
    assert_eq!(
        read(r.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        109
    );
    let mut c = read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json");
    let mut pins: BTreeMap<String, String> = serde_json::from_value(c["pins"].clone()).unwrap();
    pins.extend(
        serde_json::from_value::<BTreeMap<String, String>>(read(r.join("source-pins.json")))
            .unwrap(),
    );
    for name in ["fresh", "clean"] {
        let p = r.join(format!("native/{name}/artifacts.json"));
        let a = read(&p);
        add(p, &mut pins);
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            add(p, &mut pins);
        }
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        add(a["binary"].as_str().unwrap(), &mut pins);
    }
    for p in [
        r.join("experiment"),
        r.join("native-plan.json"),
        r.join("native/result.json"),
        r.join("implementation-audit.json"),
        PathBuf::from(TOOL),
        PathBuf::from("/tmp/zopro-control-rescheduling/all-proof-inventory.json"),
    ] {
        add(p, &mut pins);
    }
    for (p, h) in &pins {
        assert_eq!(&hash(p), h, "pin {p}");
    }
    c["variants"] = json!([{"name":"accepted","binary":"/tmp/zopro-grouping-module/native/candidate-sdk"},{"name":"fresh","binary":r.join("native/fresh-sdk")},{"name":"clean","binary":r.join("native/clean-sdk")}]);
    c["pins"] = json!(pins);
    c["prior_proof_sha256"] = json!(prior());
    save(r.join("perf-before-preflight.json"), &c);
    let mut jobs = Vec::new();
    let mut steps = Vec::new();
    for case in c["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|x| x["workers"] == 32)
    {
        for variant in ["fresh", "clean"] {
            let name = format!("{}-w4-{variant}", case["shape"].as_str().unwrap());
            let stem = Path::new(case["image"].as_str().unwrap())
                .file_stem()
                .unwrap();
            let d = r.join("preflight-proofs").join(&name).join(stem);
            let mut env = c["environment"].clone();
            env["RAYON_NUM_THREADS"] = json!("4");
            steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c","0-3",r.join(format!("native/{variant}-sdk")),case["image"],case["request"],"1",d],"environment":env,"timeout_seconds":600}));
            steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",d.parent().unwrap(),"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
            jobs.push(json!({"name":name,"directory":d,"public_sha256":case["public_sha256"]}));
        }
    }
    save(r.join("preflight-jobs.json"), &json!(jobs));
    save(r.join("preflight-pins.json"), &json!(pins));
    save(
        r.join("preflight-plan.json"),
        &json!({"version":1,"source_pins":r.join("preflight-pins.json"),"steps":steps}),
    );
    println!("Prepared all-nine preflight72SDK18exports; no performance captures yet");
}
fn qualify() {
    let r = Path::new(R);
    assert_eq!(read(r.join("preflight/result.json"))["status"], "complete");
    let mut c = read(r.join("perf-before-preflight.json"));
    let mut pins: BTreeMap<String, String> = serde_json::from_value(c["pins"].clone()).unwrap();
    let mut seen = prior();
    let mut inventory = BTreeMap::new();
    for job in read(r.join("preflight-jobs.json")).as_array().unwrap() {
        let d = Path::new(job["directory"].as_str().unwrap());
        let proof = d.join("0.proof");
        let public = d.join("0.public");
        assert_eq!(fs::metadata(&proof).unwrap().len(), 164);
        assert_eq!(hash(&public), job["public_sha256"].as_str().unwrap());
        assert!(seen.insert(hash(&proof)));
        for p in [proof, public] {
            add(&p, &mut pins);
            inventory.insert(p.display().to_string(), hash(p));
        }
        let name = job["name"].as_str().unwrap();
        let log = fs::read_to_string(r.join(format!("preflight/{name}/stdout.log"))).unwrap();
        let summary: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("SUMMARY "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(summary["rounds"], 1);
        assert_eq!(summary["priming"], 3);
        assert_eq!(summary["workers"], 4);
        let gnark =
            fs::read_to_string(r.join(format!("preflight/verify-{name}/stdout.log"))).unwrap();
        assert!(gnark.contains("verified 1 distinct proofs across 1 pinned keys"));
    }
    assert_eq!(seen.len(), 6832);
    assert_eq!(inventory.len(), 36);
    save(r.join("preflight-proof-inventory.json"), &json!(inventory));
    save(
        r.join("preflight-analysis.json"),
        &json!({"passed":true,"sdk_requests":72,"independent_exports":18,"next_prior_count":6832}),
    );
    for f in [
        "preflight-plan.json",
        "preflight/result.json",
        "preflight-analysis.json",
        "preflight-proof-inventory.json",
    ] {
        add(r.join(f), &mut pins);
    }
    c["pins"] = json!(pins);
    c["prior_proof_sha256"] = json!(seen);
    save(r.join("perf-plan-screen.json"), &c);
    println!("All-nine preflight passes72SDK18Gnark; ready for short screen");
}
fn metrics(rows: &[Value], config: &Value, blocks: usize) -> Value {
    let mut result = BTreeMap::new();
    for case in config["cases"].as_array().unwrap() {
        let name = case["name"].as_str().unwrap();
        let mut modes = BTreeMap::new();
        let mut pairs = BTreeMap::new();
        for variant in ["accepted", "fresh", "clean"] {
            let rs: Vec<_> = rows
                .iter()
                .filter(|r| r["phase"] == "latency" && r["case"] == name && r["variant"] == variant)
                .collect();
            assert_eq!(rs.len(), blocks);
            let ms = median(
                rs.iter()
                    .map(|r| r["median_ms"].as_f64().unwrap())
                    .collect(),
            );
            let rss = rs
                .iter()
                .map(|r| r["summary"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.)
                .fold(0., f64::max);
            let cpu = median(
                rs.iter()
                    .map(|r| {
                        (r["summary"]["user_cpu_ms"].as_f64().unwrap()
                            + r["summary"]["system_cpu_ms"].as_f64().unwrap())
                            / r["summary"]["rounds"].as_f64().unwrap()
                    })
                    .collect(),
            );
            modes.insert(
                variant,
                json!({"ms":ms,"maxrss_mib":rss,"cpu_ms_per_request":cpu}),
            );
        }
        for (a, b) in [
            ("fresh", "accepted"),
            ("clean", "accepted"),
            ("clean", "fresh"),
        ] {
            let mut savings = Vec::new();
            let ratios: Vec<_> = (0..blocks)
                .map(|block| {
                    let ms = |v: &str| {
                        rows.iter()
                            .find(|r| {
                                r["phase"] == "latency"
                                    && r["case"] == name
                                    && r["variant"] == v
                                    && r["block"] == block
                            })
                            .unwrap()["median_ms"]
                            .as_f64()
                            .unwrap()
                    };
                    savings.push((ms(b) - ms(a)) * 1000.);
                    ms(a) / ms(b)
                })
                .collect();
            pairs.insert(format!("{a}/{b}"),json!({"geometric_ratio":gm(&ratios),"median_ratio":median(ratios.clone()),"wins":ratios.iter().filter(|v|**v<1.).count(),"ratios":ratios,"saving_us":median(savings)}));
        }
        result.insert(name,json!({"shape":case["shape"],"workers":case["workers"],"reference":case["reference"],"modes":modes,"pairs":pairs}));
    }
    json!(result)
}
fn evaluate(stage: &str) {
    let r = Path::new(R);
    let dir = r.join(stage);
    let result = read(dir.join("result.json"));
    assert_eq!(result["status"], "complete");
    let config = read(dir.join("protocol.json"));
    let rows = read(dir.join("rows.json"));
    let rows = rows.as_array().unwrap();
    let screen = stage == "screen";
    let blocks = if screen { 3 } else { 7 };
    let cases = metrics(rows, &config, blocks);
    let mut calibration = Vec::new();
    let mut primary = Vec::new();
    let mut regressions = Vec::new();
    let mut original_a = Vec::new();
    let mut original_f = Vec::new();
    for (name, cell) in cases.as_object().unwrap() {
        let pairs = &cell["pairs"];
        let fresh = &pairs["fresh/accepted"];
        for (b, x) in fresh["ratios"].as_array().unwrap().iter().enumerate() {
            let x = x.as_f64().unwrap();
            if !(1. / 1.05..=1.05).contains(&x) {
                calibration.push(format!("{name} block{b} fresh/accepted={x}"));
            }
        }
        let med = fresh["median_ratio"].as_f64().unwrap();
        if !(1. / 1.03..=1.03).contains(&med) {
            calibration.push(format!("{name} median fresh/accepted={med}"));
        }
        for control in ["accepted", "fresh"] {
            let key = format!("clean/{control}");
            let p = &pairs[&key];
            let ratio = p["geometric_ratio"].as_f64().unwrap();
            if name == "1x2-w32"
                && (ratio >= 1. || p["wins"].as_u64().unwrap() < if screen { 2 } else { 4 })
            {
                primary.push(format!("{name} {key}={ratio},wins{}", p["wins"]));
            }
            if ratio > 1.02 {
                regressions.push(format!("{name} {key}={ratio}"));
            }
            if !screen && cell["workers"] == 32 && cell["shape"] != "merge-54x1" {
                if control == "accepted" {
                    original_a.push(ratio);
                } else {
                    original_f.push(ratio);
                }
            }
        }
    }
    let original_gm = if screen {
        Value::Null
    } else {
        assert_eq!(original_a.len(), 8);
        json!({"clean/accepted":gm(&original_a),"clean/fresh":gm(&original_f)})
    };
    let passes = calibration.is_empty()
        && primary.is_empty()
        && regressions.is_empty()
        && (screen || (gm(&original_a) <= 1. && gm(&original_f) <= 1.));
    let expected = if screen { 18 } else { 213 };
    assert_eq!(result["distinct_verified_exports"], expected);
    let sdk: u64 = rows
        .iter()
        .map(|r| r["summary"]["rounds"].as_u64().unwrap() + 3)
        .sum();
    assert_eq!(sdk, if screen { 360 } else { 10422 });
    if !screen {
        assert_eq!(result["profiles"], 3);
        for row in rows.iter().filter(|r| r["phase"] == "profile") {
            assert_eq!(row["profile"]["lost_records"], 0);
            assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
        }
    }
    let mut all: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("preflight-proof-inventory.json"))).unwrap();
    for sub in if screen {
        vec!["screen"]
    } else {
        vec!["screen", "performance"]
    } {
        let d = r.join(sub);
        for (rel, h) in read(d.join("proof-inventory.json")).as_object().unwrap() {
            let p = d.join(rel);
            assert_eq!(hash(&p), h.as_str().unwrap());
            assert!(all
                .insert(p.display().to_string(), h.as_str().unwrap().into())
                .is_none());
        }
    }
    let mut seen = prior();
    for (p, h) in &all {
        assert_eq!(hash(p), *h);
        if p.ends_with(".proof") {
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
            assert!(seen.insert(h.clone()));
        }
    }
    assert_eq!(seen.len(), if screen { 6850 } else { 7063 });
    save(r.join("proof-inventory.json"), &json!(all));
    if screen {
        save(r.join("screen-combined-proof-inventory.json"), &json!(all));
    }
    let a = json!({"stage":stage,"cases":cases,"calibration_failures":calibration,"primary_failures":primary,"regressions":regressions,"original8":original_gm,"screen_advances_to_full":screen&&passes,"full_gate_passed":!screen&&passes,"sdk_requests":sdk+72+if screen{0}else{360},"independent_exports":expected+18+if screen{0}else{18},"next_prior_count":seen.len(),"adopted":false,"goal_achieved":false,"candidate_retained":true});
    save(r.join(format!("{stage}-analysis.json")), &a);
    if screen && passes {
        let mut c = read(r.join("perf-plan-screen.json"));
        c["prior_proof_sha256"] = json!(seen);
        let mut pins: BTreeMap<String, String> = serde_json::from_value(c["pins"].clone()).unwrap();
        for f in [
            "screen-analysis.json",
            "screen/result.json",
            "screen/freshness.json",
            "screen-combined-proof-inventory.json",
        ] {
            add(r.join(f), &mut pins);
        }
        c["pins"] = json!(pins);
        save(r.join("perf-plan-full.json"), &c);
    }
    println!("{}",serde_json::to_string_pretty(&json!({"stage":stage,"passes":passes,"calibration":a["calibration_failures"],"primary":a["primary_failures"],"regressions":a["regressions"],"next_prior":seen.len()})).unwrap());
}
fn report() {
    let r = Path::new(R);
    let full = r.join("performance-analysis.json").exists();
    let a = read(r.join(if full {
        "performance-analysis.json"
    } else {
        "screen-analysis.json"
    }));
    let d = Path::new("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-26-pairing-clean");
    fs::create_dir_all(d).unwrap();
    let mut md=format!("# Clean fixed-parameter power integration\n\nThe {} is complete. Full qualification passed: {}. Production is unchanged; the candidate is retained. Small positive gains have no minimum improvement floor.\n\nThis Rust candidate keeps the previously tested fixed addition/subtraction schedule and uses it from the existing canonical final-exponentiation body. It removes the earlier runtime experiment selector, environment lookup, OnceLock policy and duplicate final-exponentiation implementation. The BN parameter constant is needed only by tests. Native scalar arithmetic removes nine cyclotomic squares and 33 full extension products per final exponentiation at source level. This is a source integration and compiler experiment, not a claim to isolate arithmetic alone.\n\nOnly pairing.rs and a new private x_chain.rs change; 169 original files remain byte-identical. No new point table, key/image copy, input cache, heap workspace, thread pool, field/MSM/solver change or universal memory cap. Existing general-power and Arkworks references are retained. [Schedule provenance and previous full SDK study](../2026-10-26-pairing-x-chain-sdk/README.md).\n\nThe exact accepted binary is compared with a fresh unmodified build and the clean candidate. Standard SDK driver, allocator policy and inputs are identical. Timers include witness construction, fresh blinded proving, verification and serialization; loading and three priming proofs are excluded. No scheduler trace or per-request observer clocks. Ratios pair process medians within rotated/reversed blocks; displayed medians are independent. CPU time includes scheduling. RSS includes loading and primes and is not minimum RAM or evidence of a memory saving.\n\n|Case|Accepted ms|Fresh ms|Clean ms|Clean/accepted|Clean/fresh|Paired saving vs fresh µs|Wins vs fresh|Clean RSS MiB|Clean CPU ms/request|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",if full{"full study"}else{"short screen"},a["full_gate_passed"]);
    for (name, cell) in a["cases"].as_object().unwrap() {
        let m = &cell["modes"];
        let p = &cell["pairs"];
        writeln!(
            md,
            "|{name}|{:.6}|{:.6}|{:.6}|{:.9}|{:.9}|{:.3}|{}|{:.2}|{:.3}|",
            m["accepted"]["ms"].as_f64().unwrap(),
            m["fresh"]["ms"].as_f64().unwrap(),
            m["clean"]["ms"].as_f64().unwrap(),
            p["clean/accepted"]["geometric_ratio"].as_f64().unwrap(),
            p["clean/fresh"]["geometric_ratio"].as_f64().unwrap(),
            p["clean/fresh"]["saving_us"].as_f64().unwrap(),
            p["clean/fresh"]["wins"],
            m["clean"]["maxrss_mib"].as_f64().unwrap(),
            m["clean"]["cpu_ms_per_request"].as_f64().unwrap()
        )
        .unwrap();
    }
    md.push_str("\n## Gate and limitations\n\nThe predeclared short screen uses three blocks and 17 timed requests plus three primes, explicitly selecting 1x2 with 16 and 32 workers. It does not qualify adoption. Advancement requires fresh/accepted calibration within factor 1.05 per block and 1.03 paired median, primary32 improvements against both controls with at least two wins, and both cases within 1.02 of both controls. No minimum gain cutoff. A non-advancing screen is not rerun until it passes. If advanced, the frozen full stage covers all nine shapes at32 workers plus primary16, seven blocks, then three warm-gated cycle profiles after every latency capture. Its full gates are in the frozen design.\n\n");
    for key in ["calibration_failures", "primary_failures", "regressions"] {
        writeln!(md, "{key}:\n").unwrap();
        let failures = a[key].as_array().unwrap();
        if failures.is_empty() {
            md.push_str("- None.\n");
        } else {
            for f in failures {
                writeln!(md, "- {}", f.as_str().unwrap()).unwrap();
            }
        }
        md.push('\n');
    }
    writeln!(md,"Native35 field/core109/raw2/cubic6/generator checks and portable/no-IFMA field/consumer checks pass. All-nine fresh/clean preflight produces 72 SDK requests and 18 distinct independently verified exports. Combined measured work: {} SDK requests, {} distinct Gnark exports, with pinned public-input binding, 164-byte proof length and freshness against 6,814 prior identities. Next prior ledger: {}. All source, exact-library, binary, input and tool pins are checked. Production's170 sources remain unchanged.\n",a["sdk_requests"],a["independent_exports"],a["next_prior_count"]).unwrap();
    if !full {
        md.push_str("No new cycle profile is collected by this short screen. The earlier failed full qualification remains failed, and its profiles do not characterize this integration. The previous full SDK report retains all supplied M5 CPU/Metal comparisons; this screen only supplies fresh primary timings. All eight CPU10× targets remain unmet.\n");
    } else {
        md.push_str("## Supplied M5 comparisons\n\nHistorical supplied references. Merge54 has no supplied reference.\n\n|Case|Clean ms|M5 CPU ms|M5 Metal ms|CPU speedup|Metal speedup|\n|---|---:|---:|---:|---:|---:|\n");
        for (name, cell) in a["cases"].as_object().unwrap() {
            if cell["workers"] != 32 {
                continue;
            }
            let ms = cell["modes"]["clean"]["ms"].as_f64().unwrap();
            if cell["reference"].is_object() {
                let cpu = cell["reference"]["cpu_ms"].as_f64().unwrap();
                let metal = cell["reference"]["metal_ms"].as_f64().unwrap();
                writeln!(
                    md,
                    "|{name}|{ms:.6}|{cpu:.0}|{metal:.0}|{:.3}×|{:.3}×|",
                    cpu / ms,
                    metal / ms
                )
                .unwrap();
            } else {
                writeln!(md, "|{name}|{ms:.6}|—|—|—|—|").unwrap();
            }
        }
    }
    md.push_str("\nNo new AWS/Aeglos, phone, TEE, giant-circuit, swap, pressure, minimum-RAM or throughput measurement. No successful SDK capture repeats or exclusions. Helper builds and hashes occur before SDK captures or after all captures. The original full CPU10× objective remains active and unproven.\n\n[Design](artifacts/design.md),[source audit](artifacts/implementation-audit.json),[screen analysis](artifacts/screen-analysis.json),[proof inventory](artifacts/proof-inventory.json),[artifact hashes](artifact-index.json).\n");
    fs::write(d.join("README.md"), md).unwrap();
    let mut artifacts = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut artifacts);
    save(d.join("artifact-index.json"), &json!(artifacts));
    println!("Archived {} files", artifacts.len());
}
fn archive(root: &Path, here: &Path, dest: &Path, pins: &mut BTreeMap<String, String>) {
    let mut es: Vec<_> = fs::read_dir(here)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    es.sort();
    for p in es {
        let rel = p.strip_prefix(root).unwrap();
        let s = rel.display().to_string();
        if p.is_dir() {
            if s.starts_with("target-") || s.starts_with("baseline") {
                continue;
            }
            archive(root, &p, dest, pins);
        } else {
            if !["json", "jsonl", "rs", "md", "txt", "log", "proof", "public"]
                .iter()
                .any(|e| p.extension().is_some_and(|x| x == *e))
            {
                continue;
            }
            let q = dest.join(rel);
            fs::create_dir_all(q.parent().unwrap()).unwrap();
            fs::copy(&p, &q).unwrap();
            let h = hash(&q);
            assert_eq!(h, hash(&p));
            pins.insert(s, h);
        }
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("setup") => setup(),
        Some("prepare") => prepare(),
        Some("qualify") => qualify(),
        Some("screen") => evaluate("screen"),
        Some("full") => evaluate("performance"),
        Some("report") => report(),
        _ => panic!("mode"),
    }
}
