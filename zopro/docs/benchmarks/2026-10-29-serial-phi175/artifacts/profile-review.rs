use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write,
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/serial-phi175";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-29-serial-phi175";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn demangle(raw: &str) -> String {
    rustc_demangle::try_demangle(raw)
        .map(|v| format!("{v:#}"))
        .unwrap_or_else(|_| raw.to_string())
}
fn category(name: &str) -> &'static str {
    let definition = name.split('<').next().unwrap();
    if definition.starts_with("crossbeam_epoch::") {
        "epoch"
    } else if definition.starts_with("rayon_core::") || definition.starts_with("crossbeam_deque::")
    {
        "scheduler"
    } else if definition.starts_with("zopro_core::msm_affine::grouping::") {
        "grouping"
    } else if definition.contains("zopro_core::msm::stream_g1_impl") {
        "G1_feed"
    } else if definition.starts_with("zopro_core::") && definition.contains("g2_borrowed::") {
        "G2_feed"
    } else if definition == "zopro_core::msm_affine::g2_phi::phi_split" {
        "visible_phi_split"
    } else {
        "other"
    }
}
fn compress(v: &Value, p: &Path) {
    let b = serde_json::to_vec(v).unwrap();
    let raw = r("profile-symbols-v2.json");
    fs::write(&raw, &b).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(&raw)
        .stdout(Stdio::from(fs::File::create(p).unwrap()))
        .status()
        .unwrap()
        .success());
    let out = Command::new("gzip").arg("-dc").arg(p).output().unwrap();
    assert!(out.status.success());
    assert_eq!(out.stdout, b);
}
fn main() {
    let p = Path::new(D);
    assert!(!p.join("artifacts/profile-categories-v2.json").exists());
    for (f, h) in read(r("postcapture-pins.json")).as_object().unwrap() {
        assert_eq!(hash(&fs::read(f).unwrap()), h.as_str().unwrap());
    }
    let a = read(r("analysis.json"));
    let mut profiles = BTreeMap::new();
    let mut symbols = BTreeMap::new();
    let mut samples = 0;
    for (name, v) in a["diagnostics"]["profiles"].as_object().unwrap() {
        let mut cats = BTreeMap::<String, u64>::new();
        let mut rows = vec![];
        let total = v["period"].as_u64().unwrap();
        let mut ns = 0;
        for symbol in v["symbols"].as_array().unwrap() {
            let raw = symbol["symbol"].as_str().unwrap();
            let name = demangle(raw);
            let cat = category(&name);
            let weight = symbol["period"].as_u64().unwrap();
            ns += symbol["samples"].as_u64().unwrap();
            *cats.entry(cat.into()).or_default() += weight;
            rows.push(json!({"raw":raw,"demangled":name,"category":cat,"samples":symbol["samples"],"period":weight}));
        }
        assert_eq!(cats.values().sum::<u64>(), total);
        assert_eq!(ns, v["samples"]);
        samples += ns;
        let shares: BTreeMap<_, _> = cats
            .iter()
            .map(|(k, n)| (k.clone(), *n as f64 / total as f64))
            .collect();
        profiles.insert(name,json!({"samples":ns,"period":total,"CPU_period_shares":shares,"by_category_period":cats}));
        symbols.insert(name, rows);
    }
    assert_eq!(samples, 43651);
    let mut md = fs::read_to_string(p.join("README.md")).unwrap();
    let start = md.find("|Profile|Samples|Grouping %|").unwrap();
    let end = start + md[start..].find("\nThe six profiles contain").unwrap();
    let mut table=String::from("|Profile|Samples|Grouping %|Scheduler definitions %|Epoch definitions %|G1 feed %|G2 feed %|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (name, v) in &profiles {
        let c = &v["CPU_period_shares"];
        let n = |k: &str| 100. * c[k].as_f64().unwrap_or(0.);
        writeln!(
            table,
            "|{name}|{}|{:.3}|{:.3}|{:.3}|{:.3}|{:.3}|",
            v["samples"],
            n("grouping"),
            n("scheduler"),
            n("epoch"),
            n("G1_feed"),
            n("G2_feed")
        )
        .unwrap();
    }
    md.replace_range(start..end, &table);
    let anchor = "The six profiles contain 43651 samples in total.";
    assert!(md.contains(anchor));
    md=md.replace(anchor,"Postcapture classification review corrects the initial raw-name matcher: perf supplied Rust v0 mangled symbols, so grouping was missed, and embedded rayon_core instantiation references caused epoch functions to be counted as scheduler. The revised table demangles names, classifies only the defining path before generic arguments, and leaves trait/closure contexts in other. Three new focused classifier tests pass. Original profile-categories.json and all raw symbol/period data remain; no profile is rerun.\n\nThe six profiles contain 43651 samples in total.");
    fs::write(p.join("README.md"), md).unwrap();
    let result = json!({"status":"complete","original_categories_retained":"profile-categories.json","profiles":profiles,"total_samples":samples,"rule":"demangle v0/legacy symbols; category from defining path before first generic argument; trait/closure contexts conservatively other","new_classifier_tests":3,"new_capture_proof_test_calls":0,"postcapture_pins_closed":read(r("postcapture-pins.json")).as_object().unwrap().len(),"counter_capture":false,"partial_profile_visibility":true});
    save(r("profile-categories-v2.json"), &result);
    save(p.join("artifacts/profile-categories-v2.json"), &result);
    compress(
        &json!(symbols),
        &p.join("artifacts/profile-symbols-v2.json.gz"),
    );
    for f in ["profile-review.rs", "profile-review-tests.log"] {
        fs::copy(r(f), p.join("artifacts").join(f)).unwrap();
    }
    save(
        p.join("artifacts/publication-review.json"),
        &json!({"raw_symbols_are_v0_mangled":true,"classification_correction_disclosed":true,"old_classification_retained":true,"partial_CPU_symbol_attribution_not_latency_causality":true,"extra_decoder_workflow_complete":true,"original_498_successful_steps_retained":true,"new_18_owner_decode_steps_complete":true,"allocation_disclosure":"std::env::var transient allocation exists in both diagnostic modes; no loop storage added","new_classifier_tests":3,"profile_review_binary_sha256":hash(&fs::read(r("profile-review")).unwrap()),"profile_review_tests_binary_sha256":hash(&fs::read(r("profile-review-tests")).unwrap()),"production_changed":false,"adopted":false,"goal_achieved":false}),
    );
    println!(
        "Corrected symbol categories for43651samples;447postcapturepinsclose;no capture rerun"
    );
    for (name, v) in profiles {
        println!("{name} {}", v["CPU_period_shares"]);
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn v0_grouping_symbol_demangles_before_matching() {
        let raw = "_RNvNtNtCsHW5vo2gROj_10zopro_core10msm_affine8grouping13scalar_groups";
        assert_eq!(category(&demangle(raw)), "grouping");
    }
    #[test]
    fn epoch_definition_does_not_inherit_rayon_generic_argument() {
        let raw="_RINvNtCsbYa8AcIWZFT_15crossbeam_epoch7default11with_handleNCNvB2_3pin0NtNtB4_5guard5GuardECs8zaw6vLE3tn_10rayon_core";
        let d = demangle(raw);
        assert!(d.contains("crossbeam_epoch::default::with_handle"));
        assert!(raw.contains("rayon_core"));
        assert_eq!(category(&d), "epoch");
    }
    #[test]
    fn closure_context_stays_unknown_and_direct_feed_is_classified() {
        assert_eq!(category("<rayon_core::some::Closure<zopro_core::msm::stream_g1_impl> as core::ops::FnMut>::call_mut"),"other");
        assert_eq!(
            category("zopro_core::msm::stream_g1_impl<zopro_bn254::G1Config>"),
            "G1_feed"
        );
    }
}
