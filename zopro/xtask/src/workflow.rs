//! Audited build, focused test and frozen-library link steps.
use crate::{
    process::{check_pins, json, run, sha256, HostLock, PinGuard},
    Result,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::{Path, PathBuf},
};

#[derive(Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Plan {
    version: u32,
    source_pins: PathBuf,
    #[serde(default)]
    environment: BTreeMap<String, String>,
    steps: Vec<Step>,
}

#[derive(Deserialize, Serialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
enum Step {
    Command {
        name: String,
        argv: Vec<String>,
        #[serde(default)]
        environment: BTreeMap<String, String>,
        #[serde(default)]
        required_tests: Vec<String>,
        #[serde(default)]
        unfiltered: bool,
        #[serde(default = "timeout")]
        timeout_seconds: u64,
    },
    LinkSdk {
        name: String,
        artifacts: PathBuf,
        driver: PathBuf,
        #[serde(default)]
        additional_sources: Vec<PathBuf>,
        #[serde(default = "flags")]
        rustc_flags: Vec<String>,
        #[serde(default = "timeout")]
        timeout_seconds: u64,
    },
}
fn timeout() -> u64 {
    600
}
fn flags() -> Vec<String> {
    [
        "-O",
        "-C",
        "target-cpu=native",
        "-C",
        "lto=fat",
        "-C",
        "codegen-units=1",
    ]
    .map(String::from)
    .to_vec()
}
impl Step {
    fn name(&self) -> &str {
        match self {
            Self::Command { name, .. } | Self::LinkSdk { name, .. } => name,
        }
    }
}

fn successful_tests(text: &str) -> BTreeSet<String> {
    let mut passed = BTreeSet::new();
    let mut current = None;
    for line in text.lines() {
        if let Some((name, status)) = line
            .strip_prefix("test ")
            .and_then(|line| line.split_once(" ... "))
        {
            current = None;
            if status == "ok" {
                passed.insert(name.into());
            } else if !status.starts_with("ignored") && !status.starts_with("FAILED") {
                // With --nocapture, test output can split libtest's name and
                // completion marker. Sequential test execution keeps them paired.
                current = Some(name.to_owned());
            }
        } else if line == "ok" {
            if let Some(name) = current.take() {
                passed.insert(name);
            }
        } else if line == "FAILED" || line.starts_with("ignored") {
            current = None;
        }
    }
    passed
}

fn unfiltered_suites(text: &str) -> bool {
    let mut summaries = 0;
    for line in text.lines().filter(|line| line.starts_with("test result:")) {
        if !line.starts_with("test result: ok. ")
            || line
                .split("; ")
                .find_map(|field| field.strip_suffix(" filtered out"))
                != Some("0")
        {
            return false;
        }
        summaries += 1;
    }
    summaries != 0
}

fn libraries(text: &str) -> Result<BTreeMap<String, String>> {
    let mut all = BTreeMap::<String, BTreeSet<String>>::new();
    for line in text.lines() {
        let artifact: serde_json::Value = serde_json::from_str(line)?;
        if artifact["reason"] != "compiler-artifact" {
            continue;
        }
        let name = artifact["target"]["name"]
            .as_str()
            .ok_or("Cargo artifact has no target name")?;
        for file in artifact["filenames"]
            .as_array()
            .ok_or("Cargo artifact has no filenames")?
        {
            let file = file.as_str().ok_or("invalid Cargo artifact filename")?;
            if file.ends_with(".rlib") {
                all.entry(name.into()).or_default().insert(file.into());
            }
        }
    }
    let mut selected = BTreeMap::new();
    for name in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "memmap2",
        "serde_json",
        "libc",
        "rayon",
    ] {
        let candidates = all
            .get(name)
            .filter(|values| values.len() == 1)
            .ok_or_else(|| format!("expected one exact Cargo rlib for {name}"))?;
        selected.insert(name.into(), candidates.first().unwrap().clone());
    }
    Ok(selected)
}

fn build_variable(name: &str) -> bool {
    crate::config::managed(name)
        || name.starts_with("CARGO_")
        || matches!(name, "RUSTFLAGS" | "RUSTDOCFLAGS" | "RUST_MIN_STACK")
}

pub fn evaluate(plan: Plan, output: &Path, resume: bool) -> Result<serde_json::Value> {
    if plan.version != 1 || plan.steps.is_empty() {
        return Err("workflow version 1 and nonempty steps are required".into());
    }
    let mut names = BTreeSet::new();
    for step in &plan.steps {
        if step.name().is_empty()
            || !step
                .name()
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
            || !names.insert(step.name())
        {
            return Err("workflow step names must be unique path components".into());
        }
    }
    let _lock = HostLock::acquire()?;
    if resume {
        let old: Plan = serde_json::from_slice(&std::fs::read(output.join("protocol.json"))?)?;
        if serde_json::to_value(old)? != serde_json::to_value(&plan)? {
            return Err("workflow resume requires the same protocol".into());
        }
    } else {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir(output)?;
        json(output.join("protocol.json"), &plan)?;
    }
    let pins: BTreeMap<String, String> =
        serde_json::from_slice(&std::fs::read(&plan.source_pins)?)?;
    if resume {
        let old: BTreeMap<String, String> =
            serde_json::from_slice(&std::fs::read(output.join("source-pins.json"))?)?;
        if old != pins {
            return Err("workflow source pins changed; completed commands cannot be reused".into());
        }
    }
    let pinned = PinGuard::new(&pins)?;
    json(output.join("source-pins.json"), &pins)?;
    let attempt = (0..)
        .find(|index| !output.join(format!("attempt-{index}.json")).exists())
        .unwrap();
    json(
        output.join(format!("attempt-{attempt}.json")),
        &serde_json::json!({"resume":resume,"orchestrator_sha256":sha256(std::env::current_exe()?)?}),
    )?;
    if output.join("failure.json").exists() {
        std::fs::rename(
            output.join("failure.json"),
            output.join(format!("previous-failure-{attempt}.json")),
        )?;
    }
    let mut env: BTreeMap<_, _> = std::env::vars()
        .filter(|(key, _)| !build_variable(key))
        .collect();
    env.extend(plan.environment.clone());
    let mut completed = Vec::new();
    let result = (|| -> Result<serde_json::Value> {
        for step in &plan.steps {
            pinned.check_identity()?;
            let directory = output.join(step.name());
            json(
                output.join("progress.json"),
                &serde_json::json!({"step":step.name(),"completed":completed.len(),"pid":std::process::id()}),
            )?;
            println!("RUN {}", step.name());
            match step {
                Step::Command {
                    argv,
                    environment,
                    required_tests,
                    unfiltered,
                    timeout_seconds,
                    ..
                } => {
                    if argv.is_empty() {
                        return Err("empty workflow command".into());
                    }
                    let mut env = env.clone();
                    env.extend(environment.clone());
                    run(argv, &directory, &env, *timeout_seconds, &[])?;
                    let stdout = std::fs::read_to_string(directory.join("stdout.log"))?;
                    let passed = successful_tests(&stdout);
                    if required_tests.iter().any(|test| !passed.contains(test)) {
                        return Err(format!("required tests did not run: {}", step.name()).into());
                    }
                    if *unfiltered && !unfiltered_suites(&stdout) {
                        return Err("full-suite check did not report unfiltered tests".into());
                    }
                    json(
                        directory.join("tests.json"),
                        &serde_json::json!({"passed_tests":passed,"required_tests":required_tests,"unfiltered":unfiltered}),
                    )?;
                }
                Step::LinkSdk {
                    artifacts,
                    driver,
                    additional_sources,
                    rustc_flags,
                    timeout_seconds,
                    ..
                } => {
                    let selected = libraries(&std::fs::read_to_string(artifacts)?)?;
                    let mut inputs = BTreeMap::new();
                    inputs.insert(
                        driver.to_str().ok_or("non-UTF-8 driver path")?.into(),
                        sha256(driver)?,
                    );
                    for source in additional_sources {
                        inputs.insert(
                            source.to_str().ok_or("non-UTF-8 source path")?.into(),
                            sha256(source)?,
                        );
                    }
                    for library in selected.values() {
                        inputs.insert(library.clone(), sha256(library)?);
                    }
                    let binary = output.join(format!("{}-sdk", step.name()));
                    if directory.join("artifacts.json").exists() {
                        let old: serde_json::Value = serde_json::from_slice(&std::fs::read(
                            directory.join("artifacts.json"),
                        )?)?;
                        if old["input_pins"] != serde_json::to_value(&inputs)?
                            || old["binary_sha256"].as_str() != Some(&sha256(&binary)?)
                        {
                            return Err("frozen SDK link inputs or executable changed".into());
                        }
                    }
                    let dependencies = Path::new(selected.values().next().unwrap())
                        .parent()
                        .ok_or("rlib without parent")?;
                    let mut argv = vec!["rustc".into(), "--edition=2021".into()];
                    argv.extend(rustc_flags.clone());
                    argv.extend([
                        "-L".into(),
                        format!("dependency={}", dependencies.display()),
                    ]);
                    for (name, path) in &selected {
                        argv.extend(["--extern".into(), format!("{name}={path}")]);
                    }
                    argv.extend([
                        driver.to_str().ok_or("non-UTF-8 driver")?.into(),
                        "-o".into(),
                        binary.to_str().ok_or("non-UTF-8 binary path")?.into(),
                    ]);
                    run(&argv, &directory, &env, *timeout_seconds, &[])?;
                    check_pins(&inputs)?;
                    json(
                        directory.join("artifacts.json"),
                        &serde_json::json!({"binary":binary,"binary_sha256":sha256(&binary)?,"libraries":selected,"input_pins":inputs,"cargo_artifacts_sha256":sha256(artifacts)?}),
                    )?;
                }
            }
            pinned.check_identity()?;
            completed.push(step.name());
            json(output.join("completed.json"), &completed)?;
            println!("DONE {}", step.name());
        }
        pinned.verify()?;
        let result =
            serde_json::json!({"status":"complete","steps":completed,"source_files":pins.len()});
        json(output.join("result.json"), &result)?;
        Ok(result)
    })();
    if let Err(error) = &result {
        json(
            output.join("failure.json"),
            &serde_json::json!({"completed":completed,"error":error.to_string()}),
        )?;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn required_tests_are_named_not_inferred_from_a_fixed_count() {
        let names=successful_tests("test grouping::one ... ok\ntest grouping::two ... ignored\ntest result: ok. 1 passed\n");
        assert_eq!(names, BTreeSet::from(["grouping::one".into()]));
    }
    #[test]
    fn named_tests_allow_uncaptured_diagnostics_and_require_completion() {
        let output = "test field::one ... LANES=64\nmore diagnostics\nok\ntest field::ignored ... ignored\ntest field::failed ... detail\nFAILED\ntest field::unfinished ... detail\ntest field::last ... ok\ntest result: ok. 2 passed; 0 filtered out\n";
        assert_eq!(
            successful_tests(output),
            BTreeSet::from(["field::one".into(), "field::last".into()])
        );
    }
    #[test]
    fn full_suite_check_requires_every_summary_to_have_exactly_zero_filtered_tests() {
        let full = "test result: ok. 109 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.1s\n";
        let filtered = full.replace("0 filtered out", "10 filtered out");
        assert!(unfiltered_suites(full));
        assert!(unfiltered_suites(&format!("{full}{full}")));
        assert!(!unfiltered_suites(&filtered));
        assert!(!unfiltered_suites(&format!("{full}{filtered}")));
        assert!(!unfiltered_suites("test grouping::one ... ok\n"));
        assert!(!unfiltered_suites(
            &full.replace("result: ok.", "result: FAILED.")
        ));
    }
    #[test]
    fn missing_or_ambiguous_cargo_libraries_are_rejected() {
        assert!(libraries("").is_err());
        let mut lines = String::new();
        for name in [
            "zopro_core",
            "zopro_bn254",
            "zopro_prover",
            "zopro_zolana",
            "memmap2",
            "serde_json",
            "libc",
            "rayon",
        ] {
            lines += &format!(
                "{}\n",
                serde_json::json!({"reason":"compiler-artifact","target":{"name":name},"filenames":[format!("/tmp/{name}.rlib")]})
            );
        }
        assert_eq!(libraries(&lines).unwrap().len(), 8);
        lines += &format!(
            "{}\n",
            serde_json::json!({"reason":"compiler-artifact","target":{"name":"rayon"},"filenames":["/tmp/other-rayon.rlib"]})
        );
        assert!(libraries(&lines).is_err());
    }
    #[test]
    fn resume_rejects_a_replaced_source_pin_set() {
        let _serial = crate::process::test_experiment_lock();
        let root = std::env::temp_dir().join(format!("zopro-workflow-test-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let first = root.join("first");
        let second = root.join("second");
        std::fs::write(&first, b"same").unwrap();
        std::fs::write(&second, b"same").unwrap();
        let pins = root.join("pins.json");
        json(
            &pins,
            &BTreeMap::from([(first.to_str().unwrap(), sha256(&first).unwrap())]),
        )
        .unwrap();
        let plan = Plan {
            version: 1,
            source_pins: pins.clone(),
            environment: BTreeMap::new(),
            steps: vec![Step::Command {
                name: "check".into(),
                argv: vec!["sh".into(), "-c".into(), "exit 0".into()],
                environment: BTreeMap::new(),
                required_tests: Vec::new(),
                unfiltered: false,
                timeout_seconds: 2,
            }],
        };
        let output = root.join("output");
        evaluate(plan, &output, false).unwrap();
        let receipt = std::fs::read(output.join("check/exit.json")).unwrap();
        json(
            output.join("failure.json"),
            &serde_json::json!({"error":"retained prior failure"}),
        )
        .unwrap();
        let same: Plan =
            serde_json::from_slice(&std::fs::read(output.join("protocol.json")).unwrap()).unwrap();
        evaluate(same, &output, true).unwrap();
        assert_eq!(
            std::fs::read(output.join("check/exit.json")).unwrap(),
            receipt
        );
        assert!(!output.join("failure.json").exists());
        assert!(output.join("previous-failure-1.json").exists());
        assert!(output.join("attempt-1.json").exists());
        let plan: Plan =
            serde_json::from_slice(&std::fs::read(output.join("protocol.json")).unwrap()).unwrap();
        json(
            &pins,
            &BTreeMap::from([(second.to_str().unwrap(), sha256(&second).unwrap())]),
        )
        .unwrap();
        assert!(evaluate(plan, &output, true).is_err());
        std::fs::remove_dir_all(root).unwrap();
    }
}
