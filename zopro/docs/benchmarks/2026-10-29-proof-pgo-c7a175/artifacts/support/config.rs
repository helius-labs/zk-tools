use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub version: u32,
    pub blocks: usize,
    #[serde(default = "profile_blocks")]
    pub profile_blocks: usize,
    #[serde(default)]
    pub profile_cases: Option<Vec<String>>,
    pub command: Vec<String>,
    pub variants: Vec<Variant>,
    pub cases: Vec<Case>,
    pub pins: BTreeMap<String, String>,
    pub verifier: Vec<String>,
    pub profile_environment: BTreeMap<String, String>,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
    #[serde(default)]
    pub checks: Vec<Vec<String>>,
    #[serde(default)]
    pub perf_prefix: Vec<String>,
    #[serde(default)]
    pub child_prefix: Vec<String>,
    #[serde(default)]
    pub symbol_categories: Vec<Category>,
    #[serde(default)]
    pub prior_proof_sha256: BTreeSet<String>,
    #[serde(default = "timeout")]
    pub timeout_seconds: u64,
    #[serde(default = "frequency")]
    pub frequency_hz: u32,
    #[serde(default = "perf")]
    pub perf: String,
    /// Match Linux CLOCK_MONOTONIC timestamps emitted by phase diagnostics.
    #[serde(default)]
    pub monotonic_clock: bool,
}
fn timeout() -> u64 {
    90
}
fn profile_blocks() -> usize {
    3
}
fn frequency() -> u32 {
    999
}
fn perf() -> String {
    "perf".into()
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Variant {
    pub name: String,
    pub binary: String,
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Case {
    pub name: String,
    pub shape: String,
    pub workers: usize,
    #[serde(default)]
    pub cpu_affinity: Option<String>,
    pub rounds: usize,
    pub profile_rounds: usize,
    pub public_sha256: String,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub request: Option<String>,
    #[serde(default)]
    pub reference: Option<Reference>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub cpu_ms: f64,
    pub metal_ms: f64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Category {
    pub name: String,
    pub contains: Vec<String>,
}

pub fn managed(key: &str) -> bool {
    ["ZOPRO_", "RAYON_", "MALLOC_", "CROSS_"]
        .iter()
        .any(|prefix| key.starts_with(prefix))
        || matches!(key, "RUSTFLAGS" | "RUSTDOCFLAGS" | "RUST_MIN_STACK")
        || key.starts_with("CARGO_PROFILE_")
        || matches!(key, "CARGO_TARGET_DIR" | "CARGO_BUILD_JOBS")
}

fn digest(value: &str) -> bool {
    value.len() == 64
        && value
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

fn cpu_affinity(value: &str) -> bool {
    !value.is_empty()
        && value.split(',').all(|span| {
            let parts: Vec<_> = span.split('-').collect();
            if !(1..=2).contains(&parts.len())
                || parts
                    .iter()
                    .any(|part| part.is_empty() || !part.bytes().all(|byte| byte.is_ascii_digit()))
            {
                return false;
            }
            let values: Option<Vec<u32>> = parts.iter().map(|part| part.parse().ok()).collect();
            values.is_some_and(|values| values.len() == 1 || values[0] <= values[1])
        })
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        if self.version != 1
            || self.blocks < 3
            || self.variants.len() < 2
            || self.cases.is_empty()
            || self.pins.is_empty()
            || self.command.is_empty()
            || self.verifier.is_empty()
            || self.profile_environment.is_empty()
            || self.timeout_seconds == 0
            || self.frequency_hz == 0
            || self.profile_blocks == 0
        {
            return Err("version 1, three blocks, two variants, cases, pins, commands and profiling are required".into());
        }
        for names in [
            self.variants.iter().map(|v| &v.name).collect::<Vec<_>>(),
            self.cases.iter().map(|c| &c.name).collect(),
        ] {
            let distinct: BTreeSet<_> = names.iter().collect();
            if distinct.len() != names.len()
                || names.iter().any(|name| {
                    name.is_empty()
                        || !name
                            .bytes()
                            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
                })
            {
                return Err("names must be unique safe path components".into());
            }
        }
        for (path, hash) in &self.pins {
            if !std::path::Path::new(path).is_absolute() || !digest(hash) {
                return Err("artifact pins need absolute paths and SHA-256 digests".into());
            }
        }
        for variant in &self.variants {
            if !self.pins.contains_key(&variant.binary) {
                return Err("each executable must be pinned".into());
            }
        }
        if self.command.first().map(String::as_str) != Some("{binary}")
            || !self.pins.contains_key(&self.verifier[0])
        {
            return Err(
                "the SDK command must start with {binary}; the independent verifier must be pinned"
                    .into(),
            );
        }
        for case in &self.cases {
            if case.workers == 0
                || case.rounds == 0
                || case.profile_rounds == 0
                || !digest(&case.public_sha256)
            {
                return Err("positive worker/round counts and established public-input digests are required".into());
            }
            if case
                .cpu_affinity
                .as_deref()
                .is_some_and(|s| !cpu_affinity(s))
            {
                return Err("CPU affinity needs ascending numeric CPU ranges".into());
            }
            for path in [case.image.as_ref(), case.request.as_ref()]
                .into_iter()
                .flatten()
            {
                if !self.pins.contains_key(path) {
                    return Err("case images and requests must be pinned".into());
                }
            }
        }
        if self
            .cases
            .iter()
            .filter_map(|case| case.reference.as_ref())
            .any(|r| {
                [r.cpu_ms, r.metal_ms]
                    .iter()
                    .any(|v| !v.is_finite() || *v <= 0.)
            })
        {
            return Err("CPU and Metal reference times must be positive and finite".into());
        }
        if let Some(names) = &self.profile_cases {
            let unique: BTreeSet<_> = names.iter().collect();
            if names.is_empty()
                || unique.len() != names.len()
                || names
                    .iter()
                    .any(|name| !self.cases.iter().any(|case| &case.name == name))
            {
                return Err(
                    "profile cases must be a nonempty unique subset of declared cases".into(),
                );
            }
        }
        for env in
            std::iter::once(&self.environment).chain(self.variants.iter().map(|v| &v.environment))
        {
            if env
                .keys()
                .any(|key| !managed(key) || self.profile_environment.contains_key(key))
            {
                return Err("benchmark variables must be explicit; profile controls cannot enter latency runs".into());
            }
        }
        if self.profile_environment.keys().any(|key| !managed(key)) {
            return Err("unmanaged profiler variable".into());
        }
        let mut names = BTreeSet::new();
        for category in &self.symbol_categories {
            if category.name == "other"
                || !names.insert(&category.name)
                || category.contains.is_empty()
                || category.contains.iter().any(String::is_empty)
            {
                return Err("categories need distinct names and nonempty substring rules; other is reserved".into());
            }
        }
        if self.checks.iter().any(Vec::is_empty) {
            return Err("empty check command".into());
        }
        Ok(())
    }
}

pub fn expand(value: &str, context: &BTreeMap<&str, String>) -> Result<String> {
    let mut result = String::new();
    let mut rest = value;
    while let Some(index) = rest.find('{') {
        result.push_str(&rest[..index]);
        rest = &rest[index + 1..];
        let end = rest.find('}').ok_or("unclosed command placeholder")?;
        result.push_str(
            context
                .get(&rest[..end])
                .ok_or("unknown command placeholder")?,
        );
        rest = &rest[end + 1..];
    }
    result.push_str(rest);
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;
    fn valid() -> serde_json::Value {
        let digest = "0".repeat(64);
        serde_json::json!({"version":1,"blocks":3,"command":["{binary}"],
            "variants":[{"name":"base","binary":"/tmp/base"},{"name":"candidate","binary":"/tmp/candidate"}],
            "cases":[{"name":"one","shape":"1x2","workers":4,"rounds":1,"profile_rounds":3,"public_sha256":digest}],
            "pins":{"/tmp/base":digest,"/tmp/candidate":digest,"/tmp/verifier":digest},"verifier":["/tmp/verifier"],
            "profile_environment":{"ZOPRO_BENCH_PERF_CTL":"{control}","ZOPRO_BENCH_PERF_ACK":"{ack}"}})
    }
    #[test]
    fn monotonic_clock_is_explicit_and_roundtrips_in_the_protocol() {
        let mut value = valid();
        let default: Config = serde_json::from_value(value.clone()).unwrap();
        assert!(!default.monotonic_clock);
        value["monotonic_clock"] = true.into();
        let configured: Config = serde_json::from_value(value).unwrap();
        configured.validate().unwrap();
        let encoded = serde_json::to_value(configured).unwrap();
        assert!(
            serde_json::from_value::<Config>(encoded)
                .unwrap()
                .monotonic_clock
        );
    }
    #[test]
    fn unknown_fields_and_unpinned_verifiers_fail() {
        let mut value = valid();
        let config: Config = serde_json::from_value(value.clone()).unwrap();
        config.validate().unwrap();
        value["surprise"] = true.into();
        assert!(serde_json::from_value::<Config>(value).is_err());
        let mut value = valid();
        value["pins"]
            .as_object_mut()
            .unwrap()
            .remove("/tmp/verifier");
        assert!(serde_json::from_value::<Config>(value)
            .unwrap()
            .validate()
            .is_err());
    }
    #[test]
    fn profile_controls_cannot_leak_into_latency_and_paths_cannot_escape() {
        let mut value = valid();
        value["environment"] = serde_json::json!({"ZOPRO_BENCH_PERF_CTL":"unwanted"});
        assert!(serde_json::from_value::<Config>(value)
            .unwrap()
            .validate()
            .is_err());
        let mut value = valid();
        value["cases"][0]["name"] = "../escape".into();
        assert!(serde_json::from_value::<Config>(value)
            .unwrap()
            .validate()
            .is_err());
    }
    #[test]
    fn profiling_subset_and_historical_references_are_validated() {
        let mut value = valid();
        value["profile_cases"] = serde_json::json!(["one"]);
        value["profile_blocks"] = 1.into();
        value["cases"][0]["reference"] = serde_json::json!({"cpu_ms":37.,"metal_ms":24.});
        serde_json::from_value::<Config>(value.clone())
            .unwrap()
            .validate()
            .unwrap();
        value["profile_cases"] = serde_json::json!(["missing"]);
        assert!(serde_json::from_value::<Config>(value)
            .unwrap()
            .validate()
            .is_err());
        let mut value = valid();
        value["profile_cases"] = serde_json::json!([]);
        assert!(serde_json::from_value::<Config>(value)
            .unwrap()
            .validate()
            .is_err());
        let mut value = valid();
        value["cases"][0]["reference"] = serde_json::json!({"cpu_ms":37.,"metal_ms":0.});
        assert!(serde_json::from_value::<Config>(value)
            .unwrap()
            .validate()
            .is_err());
    }
    #[test]
    fn cpu_affinity_accepts_explicit_lists_and_rejects_malformed_ranges() {
        let mut value = valid();
        value["cases"][0]["cpu_affinity"] = "0-15,24,28-31".into();
        serde_json::from_value::<Config>(value.clone())
            .unwrap()
            .validate()
            .unwrap();
        for affinity in [
            "",
            "all",
            "15-0",
            "0,,1",
            "0-1-2",
            "-1",
            "+1",
            "0 1",
            "4294967296",
        ] {
            value["cases"][0]["cpu_affinity"] = affinity.into();
            assert!(
                serde_json::from_value::<Config>(value.clone())
                    .unwrap()
                    .validate()
                    .is_err(),
                "{affinity}"
            );
        }
    }
    #[test]
    fn arguments_remain_literal_and_unknown_placeholders_fail() {
        let context = BTreeMap::from([("binary", "/tmp/a path/$(literal)".into())]);
        assert_eq!(
            expand("{binary}", &context).unwrap(),
            "/tmp/a path/$(literal)"
        );
        assert!(expand("{missing}", &context).is_err());
        assert!(expand("{binary", &context).is_err());
    }
    #[test]
    fn sha256_validation_rejects_non_hex_and_wrong_length() {
        assert!(digest(&"a0".repeat(32)));
        assert!(!digest(&"zz".repeat(32)));
        assert!(!digest("abc"));
    }
}
