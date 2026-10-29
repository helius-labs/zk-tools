use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/proof-pgo175";
const M:&str="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json";
const LLVM:&str="/home/ubuntu/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-profdata";
const VERIFY: &str = "/home/ubuntu/src/zopro-perf-work/batch-local-steal/gnark/verify";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn pin(p: &mut BTreeMap<String, String>, file: impl AsRef<Path>) {
    let f = file.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn prepare() {
    assert!(!r("training-plan.json").exists());
    let result = read(r("native/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["steps"].as_array().unwrap().len(), 9);
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h);
    }
    for (name, n) in [("field", 33), ("core", 124), ("prover", 8)] {
        let t = read(r(format!("native/control-{name}/tests.json")));
        assert_eq!(t["unfiltered"], true);
        assert_eq!(t["passed_tests"].as_array().unwrap().len(), n);
        pin(&mut pins, r(format!("native/control-{name}/tests.json")));
    }
    let mut prior: BTreeSet<String> = serde_json::from_value(read(
        "/home/ubuntu/src/zopro-perf-work/balanced-phi175/prior-proof-sha256.json",
    ))
    .unwrap();
    assert_eq!(prior.len(), 18592);
    for (p, h) in read("/home/ubuntu/src/zopro-perf-work/balanced-phi175/proof-inventory.json")
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 18670);
    save(r("prior-proof-sha256.json"), &json!(prior));
    assert_eq!(fs::read_dir(r("raw")).unwrap().count(), 0);
    let frozen = read(r("frozen-generate.json"));
    for (p, h) in frozen["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    let gen = read(r("native/generate/artifacts.json"));
    assert_eq!(
        hash(gen["binary"].as_str().unwrap()),
        gen["binary_sha256"].as_str().unwrap()
    );
    let nm = Command::new("nm")
        .args(["--defined-only"])
        .arg(gen["binary"].as_str().unwrap())
        .output()
        .unwrap();
    assert!(nm.status.success());
    let text = String::from_utf8(nm.stdout).unwrap();
    assert!(text.contains("__llvm_profile_dump") && text.contains("__llvm_profile_reset_counters"));
    fs::write(r("generate-symbols.txt"), text).unwrap();
    let cases = read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");
    assert_eq!(cases.as_array().unwrap().len(), 9);
    save(r("training-cases.json"), &cases);
    let mut steps = vec![];
    let mut jobs = vec![];
    for c in cases.as_array().unwrap() {
        assert_eq!(c["workers"], 32);
        let name = format!("train-{}", c["name"].as_str().unwrap());
        let dir = r("training-proofs").join(&name);
        let mut env = read(M)["environment"].clone();
        env["RAYON_NUM_THREADS"] = json!("32");
        env["ZOPRO_CPU_DEDUP"] = json!("1");
        env["LLVM_PROFILE_FILE"] = json!(r(format!("raw/{name}-%p-%m.profraw")));
        steps.push(json!({"kind":"command","name":name,"argv":["taskset","-c",c["cpu_affinity"],gen["binary"],c["image"],c["request"],dir],"environment":env,"timeout_seconds":1800}));
        jobs.push(json!({"name":name,"case":c,"directory":dir}));
        for key in ["image", "request"] {
            pin(&mut pins, c[key].as_str().unwrap());
        }
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
    }
    for j in &jobs {
        let c = &j["case"];
        for i in 0..11 {
            let dir = Path::new(j["directory"].as_str().unwrap());
            steps.push(json!({"kind":"command","name":format!("verify-{}-{i}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join(format!("{i}.proof")),dir.join(format!("{i}.public")),c["public_sha256"]],"timeout_seconds":120}));
        }
    }
    steps.push(json!({"kind":"command","name":"merge-and-audit","argv":[r("train-helper"),"finish"],"timeout_seconds":600}));
    assert_eq!(steps.len(), 109);
    for e in
        fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap()
    {
        pin(&mut pins, e.unwrap().path());
    }
    for file in [LLVM, VERIFY, M] {
        pin(&mut pins, file);
    }
    for file in [
        "train.rs",
        "train-helper",
        "train-tests",
        "train-tests.log",
        "train-driver.rs",
        "generate-symbols.txt",
        "training-cases.json",
        "prior-proof-sha256.json",
        "native/result.json",
        "native/generate/artifacts.json",
        "frozen-generate.json",
    ] {
        pin(&mut pins, r(file));
    }
    pin(&mut pins, gen["binary"].as_str().unwrap());
    save(r("training-jobs.json"), &json!(jobs));
    pin(&mut pins, r("training-jobs.json"));
    save(r("training-pins.json"), &json!(pins));
    save(
        r("training-plan.json"),
        &json!({"version":1,"source_pins":r("training-pins.json"),"steps":steps}),
    );
    println!("Prepared109training steps:9captures99Gnark1mergeaudit;126SDK99exports;ledger18670->18769onlyverified");
}
fn counts(text: &str) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    let mut name = None;
    for line in text.lines() {
        if line.starts_with("  ") && !line.starts_with("    ") && line.ends_with(':') {
            name = Some(line.trim().trim_end_matches(':').to_string());
        }
        if let Some(data) = line.trim().strip_prefix("Block counts: ") {
            let raw = name.take().expect("counts without function");
            let c: Vec<u64> = serde_json::from_str(data).unwrap();
            assert!(!c.is_empty());
            let start = raw.find("_R").or_else(|| raw.find("_ZN")).unwrap_or(0);
            let demangled = rustc_demangle::try_demangle(&raw[start..])
                .map(|d| format!("{d:#}"))
                .unwrap_or_else(|_| raw.clone());
            let max = *c.iter().max().unwrap();
            assert!(out
                .insert(
                    raw,
                    json!({"demangled":demangled,"counts":c,"max_block_count":max})
                )
                .is_none());
        }
    }
    assert!(!out.is_empty());
    out
}
fn pick(c: &BTreeMap<String, Value>, module: &str, method: &str) -> Value {
    let matches: Vec<_> = c
        .iter()
        .filter(|(_, v)| {
            let n = v["demangled"].as_str().unwrap();
            n.contains(module) && n.contains("Prover") && n.ends_with(method)
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "ambiguous {module}/{method}:{}",
        json!(matches)
    );
    json!({"raw":matches[0].0,"profile":matches[0].1})
}
fn finish() {
    assert!(!r("training-audit.json").exists());
    let pins = read(r("training-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let mut closing: BTreeMap<String, String> = serde_json::from_value(pins).unwrap();
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(r("prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 18670);
    let jobs = read(r("training-jobs.json"));
    let mut inventory = BTreeMap::new();
    for j in jobs.as_array().unwrap() {
        let name = j["name"].as_str().unwrap();
        let log = fs::read_to_string(r(format!("training/{name}/stdout.log"))).unwrap();
        let t: Value = serde_json::from_str(
            log.lines()
                .find_map(|l| l.strip_prefix("TRAINING "))
                .unwrap(),
        )
        .unwrap();
        assert_eq!(t["SDK_requests"], 14);
        assert_eq!(t["proofs_after_reset"], 11);
        assert_eq!(t["dump_before_exports"], true);
        assert_eq!(t["workers"], 32);
        for i in 0..11 {
            let verified =
                fs::read_to_string(r(format!("training/verify-{name}-{i}/stdout.log"))).unwrap();
            assert!(verified.contains("VERIFIED proof_sha256="));
            let c = &j["case"];
            assert!(verified.contains(&format!(
                "image_sha256={}",
                c.get("verification_image_sha256")
                    .unwrap_or(&c["image_sha256"])
                    .as_str()
                    .unwrap()
            )));
            for ext in ["proof", "public"] {
                let p = Path::new(j["directory"].as_str().unwrap()).join(format!("{i}.{ext}"));
                let h = hash(&p);
                if ext == "proof" {
                    assert_eq!(fs::metadata(&p).unwrap().len(), 164);
                    assert!(prior.insert(h.clone()));
                } else {
                    assert_eq!(h, c["public_sha256"].as_str().unwrap());
                }
                assert!(verified.contains(&format!("{ext}_sha256={h}")));
                inventory.insert(p.display().to_string(), h);
            }
        }
    }
    assert_eq!(prior.len(), 18769);
    assert_eq!(inventory.len(), 198);
    save(r("training-proof-inventory.json"), &json!(inventory));
    let mut raw: Vec<PathBuf> = fs::read_dir(r("raw"))
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    raw.sort();
    assert_eq!(raw.len(), 9);
    for j in jobs.as_array().unwrap() {
        let prefix = format!("{}-", j["name"].as_str().unwrap());
        assert_eq!(
            raw.iter()
                .filter(|p| p
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .starts_with(&prefix))
                .count(),
            1
        );
    }
    for p in &raw {
        assert_eq!(p.extension().unwrap(), "profraw");
        pin(&mut closing, p);
    }
    let o = Command::new(LLVM)
        .arg("merge")
        .arg("-o")
        .arg(r("merged.profdata"))
        .args(&raw)
        .output()
        .unwrap();
    fs::write(r("profile-merge.stdout.log"), o.stdout).unwrap();
    fs::write(r("profile-merge.stderr.log"), &o.stderr).unwrap();
    assert!(o.status.success());
    let o = Command::new(LLVM)
        .args(["show", "--all-functions", "--counts"])
        .arg(r("merged.profdata"))
        .output()
        .unwrap();
    fs::write(r("profile-show.stderr.log"), &o.stderr).unwrap();
    assert!(o.status.success());
    let text = String::from_utf8(o.stdout).unwrap();
    fs::write(r("profile-counts.log"), &text).unwrap();
    let c = counts(&text);
    let prove = pick(&c, "zopro_zolana", "::prove");
    let load = pick(&c, "zopro_prover", "::load_image");
    let new = pick(&c, "zopro_zolana", "::new");
    assert_eq!(prove["profile"]["max_block_count"], 99);
    assert_eq!(load["profile"]["max_block_count"], 0);
    assert_eq!(new["profile"]["max_block_count"], 0);
    save(r("profile-functions.json"), &json!(c));
    for file in [
        "merged.profdata",
        "profile-counts.log",
        "profile-functions.json",
        "training-proof-inventory.json",
        "profile-merge.stdout.log",
        "profile-merge.stderr.log",
        "profile-show.stderr.log",
    ] {
        pin(&mut closing, r(file));
    }
    save(
        r("training-audit.json"),
        &json!({"passed":true,"SDK_requests":126,"priming_requests_excluded_from_profile":27,"SDK_prove_count":99,"SDK_load_count":0,"SDK_constructor_count":0,"SDK_prove":prove,"key_load":load,"SDK_constructor":new,"raw_profiles":raw,"raw_profile_count":9,"profile_function_count":c.len(),"atomic_generate_counters_only":true,"new_Gnark_exports":99,"proof_public_members":198,"proof_ledger":18769,"pins":closing,"no_successful_capture_or_verification_repeated":true,"training_timings_are_not_performance_evidence":true,"all175prover_sources_exact":true}),
    );
    println!("TRAININGAUDIT99prove/0load/0new;99freshGnark;ledger18769;9profiles;USE qualification permitted");
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parses_real_prior_ir_profile_without_using_it_for_training() {
        let text=fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-22-universal-proof-pgo/profile-counts.log").unwrap();
        let c = counts(&text);
        assert_eq!(
            pick(&c, "zopro_zolana", "::prove")["profile"]["max_block_count"],
            242
        );
        assert_eq!(
            pick(&c, "zopro_zolana", "::load")["profile"]["max_block_count"],
            0
        );
    }
    #[test]
    fn preserves_zero_and_full_width_counts() {
        let text="Counters:\n  raw:\n    Hash: 0x123\n    Counters: 2\n    Block counts: [0, 18446744073709551615]\n";
        let c = counts(text);
        assert_eq!(c["raw"]["counts"][0], 0);
        assert_eq!(c["raw"]["max_block_count"], u64::MAX);
    }
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("prepare|finish"),
    }
}
