use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fmt::Write,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/tmp/zopro-poseidon-fused-dot";
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
fn prior()->BTreeSet<String>{let mut seen:BTreeSet<String>=serde_json::from_value(read("/tmp/zopro-g2-tree-sdk/context/manifest.json")["prior_proof_sha256"].clone()).unwrap();assert_eq!(seen.len(),5958);for p in ["/tmp/zopro-g2-tree-sdk/proof-inventory.json","/tmp/zopro-g1-quarter-current/proof-inventory.json","/tmp/zopro-pairing-x-chain-sdk/proof-inventory.json","/tmp/zopro-control-rescheduling/all-proof-inventory.json","/tmp/zopro-pairing-clean/proof-inventory.json","/tmp/zopro-borrowed-pending/proof-inventory.json","/tmp/zopro-borrowed-g1/proof-inventory.json","/tmp/zopro-poseidon-frontier/proof-inventory.json"]{for(file,h)in read(p).as_object().unwrap(){assert_eq!(hash(file),h.as_str().unwrap());if file.ends_with(".proof"){assert!(seen.insert(h.as_str().unwrap().into()));}}}assert_eq!(seen.len(),6946);seen}
fn setup(){let r=Path::new(R);let manifest=read("/tmp/zopro-grouping-module/production-source-manifest.json");let mut pins=BTreeMap::new();let mut changed=Vec::new();for(rel,h)in manifest["source_files_sha256"].as_object().unwrap(){let original=Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel);assert_eq!(hash(&original),h.as_str().unwrap());add(&original,&mut pins);let baseline=r.join("baseline").join(rel);assert_eq!(hash(&baseline),h.as_str().unwrap());add(&baseline,&mut pins);let candidate=r.join("candidate").join(rel);if hash(&candidate)!=h.as_str().unwrap(){changed.push(rel.clone());let text=fs::read_to_string(&candidate).unwrap().replace("mod dot;\n","").replace("                next[i] = dot::dot(&a[offset + i * t..offset + (i + 1) * t], &s[..t]);","                for j in 0..t {\n                    next[i] += a[offset + i * t + j] * s[j];\n                }").replace("            rest += dot::dot(&a[offset + 1..offset + t], &s[1..t]);","            for j in 1..t {\n                rest += a[offset + j] * s[j];\n            }");assert_eq!(text,fs::read_to_string(&original).unwrap());}add(candidate,&mut pins);}assert_eq!(changed,vec!["crates/zopro-core/src/solver/sparse_poseidon.rs"]);
 for sub in ["baseline","candidate"]{for rel in ["crates/zopro-bench/examples/cpu-sdk.rs","crates/zopro-bench/examples/support/perf.rs"]{let p=r.join(sub).join(rel);assert_eq!(hash(&p),hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(rel)));add(p,&mut pins);}}
 for p in [r.join("candidate/crates/zopro-core/src/solver/sparse_poseidon/dot.rs"),r.join("experiment.rs"),r.join("adapt.rs"),r.join("design.md"),PathBuf::from(TOOL),PathBuf::from("/usr/bin/perf")]{add(p,&mut pins);}save(r.join("source-pins.json"),&json!(pins));save(r.join("implementation-audit.json"),&json!({"original_files":170,"unchanged_candidate_originals":169,"changed":changed,"new_module":"solver/sparse_poseidon/dot.rs","all_original_bytes_recovered_except2dot_substitutions_and_module_declaration":true,"field_sources_unchanged":true,"new_arithmetic_primitive":false,"new_heap_scratch":false,"new_table_or_plan":false,"new_pool_or_scheduler":false,"experimental_selector":false,"production_changed":false}));
 let text=serde_json::to_string(&read("/tmp/zopro-pairing-clean/native-plan.json")).unwrap().replace("/tmp/zopro-pairing-clean",R).replace("clean","fused");let mut plan:Value=serde_json::from_str(&text).unwrap();let steps=plan["steps"].as_array_mut().unwrap();steps.retain(|step|!["field","portable-consumers-field","no-ifma-consumers-field"].contains(&step["name"].as_str().unwrap()));for step in steps{if step["name"]=="core"{step["required_tests"]=json!(["solver::sparse_poseidon::dot::tests::odd_even_empty_and_boundary_inputs_match_separate_products","solver::sparse_poseidon::dot::tests::width_three_dense_and_partial_products_preserve_values","solver::sparse_poseidon::tests::every_width_exact_rows_and_adversarial_rejection","solver::sparse_poseidon::tests::raw_operation_indices_above_tag_limit_fall_back"]);}}save(r.join("native-plan.json"),&plan);println!("Frozen scalar fused dot; one original file/one module; native111expected; fresh and candidate consumers");}
fn prepare() {
    let r = Path::new(R);
    assert_eq!(read(r.join("native/result.json"))["status"], "complete");
    assert_eq!(
        read(r.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        111
    );
    let mut c = read("/tmp/zopro-pairing-x-chain-sdk/perf-plan.json");
    let mut pins: BTreeMap<String, String> = serde_json::from_value(c["pins"].clone()).unwrap();
    pins.extend(
        serde_json::from_value::<BTreeMap<String, String>>(read(r.join("source-pins.json")))
            .unwrap(),
    );
    for name in ["fresh", "fused"] {
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
    c["variants"] = json!([{"name":"accepted","binary":"/tmp/zopro-grouping-module/native/candidate-sdk"},{"name":"fresh","binary":r.join("native/fresh-sdk")},{"name":"fused","binary":r.join("native/fused-sdk")}]);
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
        for variant in ["fresh", "fused"] {
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
    assert_eq!(seen.len(), 6964);
    assert_eq!(inventory.len(), 36);
    save(r.join("preflight-proof-inventory.json"), &json!(inventory));
    save(
        r.join("preflight-analysis.json"),
        &json!({"passed":true,"sdk_requests":72,"independent_exports":18,"next_prior_count":6964}),
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
        for variant in ["accepted", "fresh", "fused"] {
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
            ("fused", "accepted"),
            ("fused", "fresh"),
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
            let key = format!("fused/{control}");
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
        json!({"fused/accepted":gm(&original_a),"fused/fresh":gm(&original_f)})
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
    assert_eq!(seen.len(), if screen { 6982 } else { 7195 });
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
fn main(){match std::env::args().nth(1).as_deref(){Some("setup")=>setup(),Some("prepare")=>prepare(),Some("qualify")=>qualify(),Some("screen")=>evaluate("screen"),Some("full")=>evaluate("performance"),_=>panic!("mode")}}
