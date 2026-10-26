use crate::{
    config::{expand, Config},
    process::{fifo, json, run, sha256, HostLock, PinGuard},
    results::{markdown, paired_summary, perf_samples, profile_summary, sdk_output, Row},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json as value, Value};
use std::{
    collections::BTreeMap,
    path::{Path, PathBuf},
};

fn proof_files(directory: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
    for entry in std::fs::read_dir(directory)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            proof_files(&entry.path(), files)?;
        } else if entry.file_type()?.is_file()
            && entry.path().extension().is_some_and(|ext| ext == "proof")
        {
            files.push(entry.path());
        }
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Mode {
    Full,
    Screen,
}

impl Mode {
    fn phases(self) -> &'static [&'static str] {
        match self {
            Self::Full => &["latency", "profile"],
            Self::Screen => &["latency"],
        }
    }
}

fn check_execution(recorded: Option<Mode>, wanted: Mode) -> Result<()> {
    // Captures made before execution receipts existed are full experiments.
    if recorded.unwrap_or(Mode::Full) != wanted {
        return Err("screen and full experiments cannot resume each other's captures".into());
    }
    Ok(())
}

pub fn evaluate(config: Config, output: &Path, resume: bool) -> Result<Value> {
    evaluate_mode(config, output, resume, Mode::Full)
}

pub fn evaluate_screen(config: Config, output: &Path, resume: bool) -> Result<Value> {
    evaluate_mode(config, output, resume, Mode::Screen)
}

fn evaluate_mode(config: Config, output: &Path, resume: bool, mode: Mode) -> Result<Value> {
    config.validate()?;
    if !cfg!(target_os = "linux") {
        return Err("perf experiments currently require a Linux host".into());
    }
    let _lock = HostLock::acquire()?;
    if resume {
        let previous: Config =
            serde_json::from_slice(&std::fs::read(output.join("protocol.json"))?)?;
        if serde_json::to_value(previous)? != serde_json::to_value(&config)? {
            return Err("resume requires the identical declared protocol".into());
        }
        let receipt = output.join("execution.json");
        check_execution(
            if receipt.exists() {
                Some(serde_json::from_slice(&std::fs::read(receipt)?)?)
            } else {
                None
            },
            mode,
        )?;
    } else {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir(output)?;
        json(output.join("protocol.json"), &config)?;
        json(output.join("execution.json"), &mode)?;
    }
    let mut rows = Vec::<Row>::new();
    let mut inventory = if resume && output.join("proof-inventory.json").exists() {
        serde_json::from_slice::<BTreeMap<String, String>>(&std::fs::read(
            output.join("proof-inventory.json"),
        )?)?
    } else {
        BTreeMap::new()
    };
    let mut prior = config.prior_proof_sha256.clone();
    prior.extend(
        inventory
            .iter()
            .filter(|(path, _)| path.ends_with(".proof"))
            .map(|(_, digest)| digest.clone()),
    );
    let mut environment: BTreeMap<_, _> = std::env::vars()
        .filter(|(key, _)| !crate::config::managed(key))
        .collect();
    environment.extend(config.environment.clone());
    let mut phase = "tests";
    let result = (|| -> Result<Value> {
        let pinned = PinGuard::new(&config.pins)?;
        json(
            output.join("pin-policy.json"),
            &value!({"full_sha256":"before and after the complete experiment","per_process":"device/inode/size/mtime/ctime including nanoseconds; access time excluded"}),
        )?;
        for (path, digest) in &inventory {
            if &sha256(output.join(path))? != digest {
                return Err("retained proof changed before resume".into());
            }
        }
        let attempt = (0..)
            .find(|index| !output.join(format!("attempt-{index}.json")).exists())
            .unwrap();
        json(
            output.join(format!("attempt-{attempt}.json")),
            &value!({"resume":resume,"orchestrator_sha256":sha256(std::env::current_exe()?)?}),
        )?;
        if output.join("failure.json").exists() {
            std::fs::rename(
                output.join("failure.json"),
                output.join(format!("previous-failure-{attempt}.json")),
            )?;
        }
        for (index, command) in config.checks.iter().enumerate() {
            run(
                command,
                &output.join(format!("check-{index}")),
                &environment,
                config.timeout_seconds,
                &[],
            )?;
        }
        for &current_phase in mode.phases() {
            phase = current_phase;
            let blocks = if phase == "profile" {
                config.profile_blocks
            } else {
                config.blocks
            };
            for block in 0..blocks {
                let mut cases: Vec<_> = config
                    .cases
                    .iter()
                    .filter(|case| {
                        phase != "profile"
                            || config
                                .profile_cases
                                .as_ref()
                                .is_none_or(|names| names.contains(&case.name))
                    })
                    .collect();
                let shift = block % cases.len();
                cases.rotate_left(shift);
                for (ci, case) in cases.iter().enumerate() {
                    let mut variants: Vec<_> = config.variants.iter().collect();
                    let shift = (ci + block) % variants.len();
                    variants.rotate_left(shift);
                    if block % 2 == 1 {
                        variants.reverse();
                    }
                    for variant in variants {
                        pinned.check_identity()?;
                        let name = format!("{phase}/b{block}/{}/{}", case.name, variant.name);
                        let directory = output.join(&name);
                        if !directory.exists() {
                            std::fs::create_dir_all(directory.parent().unwrap())?;
                            std::fs::create_dir(&directory)?;
                            fifo(&directory.join("ctl"))?;
                            fifo(&directory.join("ack"))?;
                        }
                        let rounds = if phase == "profile" {
                            case.profile_rounds
                        } else {
                            case.rounds
                        };
                        let mut context = BTreeMap::from([
                            ("binary", variant.binary.clone()),
                            ("shape", case.shape.clone()),
                            ("rounds", rounds.to_string()),
                            ("workers", case.workers.to_string()),
                            (
                                "proof_dir",
                                directory
                                    .join("proofs")
                                    .to_str()
                                    .ok_or("non-UTF-8 output path")?
                                    .into(),
                            ),
                            (
                                "control",
                                directory
                                    .join("ctl")
                                    .to_str()
                                    .ok_or("non-UTF-8 control path")?
                                    .into(),
                            ),
                            (
                                "ack",
                                directory
                                    .join("ack")
                                    .to_str()
                                    .ok_or("non-UTF-8 ack path")?
                                    .into(),
                            ),
                        ]);
                        if let Some(image) = &case.image {
                            context.insert("image", image.clone());
                            context.insert(
                                "stem",
                                Path::new(image)
                                    .file_stem()
                                    .and_then(|s| s.to_str())
                                    .ok_or("image has no UTF-8 stem")?
                                    .into(),
                            );
                        }
                        if let Some(request) = &case.request {
                            context.insert("request", request.clone());
                        }
                        let argv = config
                            .command
                            .iter()
                            .map(|arg| expand(arg, &context))
                            .collect::<Result<Vec<_>>>()?;
                        let mut env = environment.clone();
                        env.extend(variant.environment.clone());
                        env.insert("RAYON_NUM_THREADS".into(), case.workers.to_string());
                        if phase == "profile" {
                            for (key, arg) in &config.profile_environment {
                                env.insert(key.clone(), expand(arg, &context)?);
                            }
                        }
                        // Explicit benchmark variables survive sudo's environment filtering.
                        let mut child = config.child_prefix.clone();
                        child.push("env".into());
                        child.extend(
                            env.iter()
                                .filter(|(key, _)| crate::config::managed(key))
                                .map(|(key, value)| format!("{key}={value}")),
                        );
                        if let Some(affinity) = &case.cpu_affinity {
                            child.extend(["taskset".into(), "-c".into(), affinity.clone()]);
                        }
                        child.extend(argv);
                        let (command, prefix) = if phase == "profile" {
                            let mut command = vec![
                                config.perf.clone(),
                                "record".into(),
                                "--no-buildid-cache".into(),
                                "-e".into(),
                                "cycles:u".into(),
                                "-F".into(),
                                config.frequency_hz.to_string(),
                                "-m".into(),
                                "512".into(),
                                "--delay=-1".into(),
                                format!("--control=fifo:{},{}", context["control"], context["ack"]),
                                "-o".into(),
                                directory
                                    .join("perf.data")
                                    .to_str()
                                    .ok_or("non-UTF-8 perf path")?
                                    .into(),
                                "--".into(),
                            ];
                            if config.monotonic_clock {
                                command.splice(2..2, ["--clockid".into(), "mono".into()]);
                            }
                            command.extend(child);
                            (command, &config.perf_prefix[..])
                        } else {
                            (child, &[][..])
                        };
                        json(
                            output.join("progress.json"),
                            &value!({"phase":phase,"process":name,"completed":rows.len()}),
                        )?;
                        println!("RUN {name}");
                        run(
                            &command,
                            &directory.join("run"),
                            &env,
                            config.timeout_seconds,
                            prefix,
                        )?;
                        let sdk = sdk_output(
                            &std::fs::read_to_string(directory.join("run/stdout.log"))?,
                            rounds,
                        )?;
                        if sdk.summary["workers"].as_u64() != Some(case.workers as u64) {
                            return Err("SDK worker count differs from the declared case".into());
                        }
                        let verifier = config
                            .verifier
                            .iter()
                            .map(|arg| expand(arg, &context))
                            .collect::<Result<Vec<_>>>()?;
                        run(
                            &verifier,
                            &directory.join("verify"),
                            &environment,
                            config.timeout_seconds,
                            &[],
                        )?;
                        let mut files = Vec::new();
                        proof_files(&directory.join("proofs"), &mut files)?;
                        if files.len() != 1 {
                            return Err("expected one final proof export per process".into());
                        }
                        let proof = &files[0];
                        if sha256(proof.with_extension("public"))? != case.public_sha256 {
                            return Err("public inputs differ from the established fixture".into());
                        }
                        let hash = sha256(proof)?;
                        let relative = proof
                            .strip_prefix(output)?
                            .to_str()
                            .ok_or("non-UTF-8 proof path")?;
                        let recorded = inventory.get(relative).is_some_and(|v| v == &hash);
                        if !recorded && !prior.insert(hash.clone())
                            || inventory.iter().any(|(path, value)| {
                                path != relative && path.ends_with(".proof") && value == &hash
                            })
                        {
                            return Err("proof export repeats an earlier proof".into());
                        }
                        for artifact in [proof.clone(), proof.with_extension("public")] {
                            inventory.insert(
                                artifact
                                    .strip_prefix(output)?
                                    .to_str()
                                    .ok_or("non-UTF-8 proof path")?
                                    .into(),
                                sha256(&artifact)?,
                            );
                        }
                        json(output.join("proof-inventory.json"), &inventory)?;
                        let profile = if phase == "profile" {
                            let decode = vec![
                                config.perf.clone(),
                                "script".into(),
                                "--show-lost-events".into(),
                                "--no-demangle".into(),
                                "-i".into(),
                                directory
                                    .join("perf.data")
                                    .to_str()
                                    .ok_or("non-UTF-8 perf path")?
                                    .into(),
                                "-F".into(),
                                "pid,tid,period,ip,sym".into(),
                            ];
                            run(
                                &decode,
                                &directory.join("symbols"),
                                &environment,
                                config.timeout_seconds,
                                &config.perf_prefix,
                            )?;
                            Some(perf_samples(
                                &std::fs::read_to_string(directory.join("symbols/stdout.log"))?,
                                &config.symbol_categories,
                            )?)
                        } else {
                            None
                        };
                        println!("DONE {name}: {:.6} ms", sdk.median_ms);
                        rows.push(Row {
                            case: case.name.clone(),
                            variant: variant.name.clone(),
                            phase: phase.into(),
                            block,
                            sdk,
                            profile,
                        });
                        json(output.join("rows.json"), &rows)?;
                        json(output.join("proof-inventory.json"), &inventory)?;
                        pinned.check_identity()?;
                    }
                }
            }
        }
        for (path, digest) in &inventory {
            if &sha256(output.join(path))? != digest {
                return Err("retained proof artifact changed".into());
            }
        }
        pinned.verify()?;
        json(
            output.join("freshness.json"),
            &value!({"prior_proof_sha256":prior}),
        )?;
        let latency = paired_summary(&rows, &config.variants[0].name)?;
        let mut references = Vec::new();
        for case in &config.cases {
            if let Some(reference) = &case.reference {
                for variant in &config.variants {
                    let ms = latency[&case.name][&variant.name]["median_ms"]
                        .as_f64()
                        .ok_or("reference case has no measured latency")?;
                    references.push(value!({"case":case.name,"shape":case.shape,"variant":variant.name,"ms":ms,
                    "cpu_ms":reference.cpu_ms,"metal_ms":reference.metal_ms,"cpu_speedup":reference.cpu_ms/ms,
                    "metal_speedup":reference.metal_ms/ms,"cpu_meets_10x":reference.cpu_ms/ms>=10.}));
                }
            }
        }
        let mut controls = serde_json::Map::new();
        if mode == Mode::Screen {
            for variant in &config.variants {
                controls.insert(variant.name.clone(), paired_summary(&rows, &variant.name)?);
            }
        }
        let result = value!({"status":"complete","latency":latency,"references":references,
            "execution":mode,"latency_by_control":controls,
            "freshness_file":"freshness.json",
            "profile_comparison":profile_summary(&rows,&config.variants[0].name)?,
            "processes":rows.len(),"distinct_verified_exports":inventory.len()/2,
            "profiles":rows.iter().filter(|row|row.phase=="profile").count(),"qualification":false});
        json(output.join("result.json"), &result)?;
        std::fs::write(
            output.join("report.md"),
            if mode == Mode::Screen {
                crate::screen::markdown(&result)
            } else {
                markdown(&result)
            },
        )?;
        Ok(result)
    })();
    if let Err(error) = &result {
        json(
            output.join("failure.json"),
            &value!({"phase":phase,"completed":rows.len(),"error":error.to_string()}),
        )?;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scope_separates_screens_from_full_and_legacy_captures() {
        assert_eq!(Mode::Screen.phases(), &["latency"]);
        assert_eq!(Mode::Full.phases(), &["latency", "profile"]);
        assert!(check_execution(None, Mode::Full).is_ok());
        assert!(check_execution(None, Mode::Screen).is_err());
        assert!(check_execution(Some(Mode::Screen), Mode::Full).is_err());
        assert!(check_execution(Some(Mode::Full), Mode::Screen).is_err());
        assert!(check_execution(Some(Mode::Screen), Mode::Screen).is_ok());
    }

    #[test]
    fn screen_keeps_verifier_and_proof_receipts_and_resumes_without_new_calls() {
        use std::os::unix::fs::PermissionsExt;
        let _serial = crate::process::test_experiment_lock();
        let root = std::env::temp_dir().join(format!("zopro-screen-mock-{}", std::process::id()));
        std::fs::create_dir(&root).unwrap();
        let sdk = root.join("sdk");
        let verifier = root.join("verifier");
        let fixture = root.join("fixture");
        let counter = root.join("calls");
        std::fs::write(&fixture, b"fixture").unwrap();
        std::fs::write(
            &sdk,
            r##"#!/bin/sh
set -eu
mkdir -p "$2"
head -c 164 /dev/urandom > "$2/0.proof"
printf fixture > "$2/0.public"
printf 'call\n' >> "$3"
i=0
while [ "$i" -lt "$1" ]; do
  printf 'SAMPLE {"round":%s,"verified_ms":2.0}\n' "$i"
  i=$((i+1))
done
printf 'SUMMARY {"workers":1,"rounds":%s,"priming":3}\n' "$1"
"##,
        )
        .unwrap();
        std::fs::write(&verifier, "#!/bin/sh\nset -eu\ntest \"$2\" = accept\ntest -s \"$1/0.proof\"\ntest -s \"$1/0.public\"\n").unwrap();
        for path in [&sdk, &verifier] {
            std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700)).unwrap();
        }
        let pins: BTreeMap<_, _> = [&sdk, &verifier]
            .into_iter()
            .map(|p| (p.display().to_string(), sha256(p).unwrap()))
            .collect();
        let config:Config=serde_json::from_value(value!({"version":1,"blocks":7,
            "command":["{binary}","{rounds}","{proof_dir}",counter],
            "variants":[{"name":"base","binary":sdk},{"name":"candidate","binary":sdk}],
            "cases":[{"name":"one","shape":"1x2","workers":1,"rounds":31,"profile_rounds":51,"public_sha256":sha256(&fixture).unwrap()}],
            "pins":pins,"verifier":[verifier,"{proof_dir}","accept"],
            "prior_proof_sha256":["f".repeat(64)],
            "profile_environment":{"ZOPRO_BENCH_PERF_CTL":"{control}"}})).unwrap();
        let config = crate::screen::prepare(config, &["one".into()]).unwrap();
        let output = root.join("screen");
        let result = evaluate_screen(config, &output, false).unwrap();
        assert_eq!(result["profiles"], 0);
        assert_eq!(result["distinct_verified_exports"], 6);
        assert_eq!(result["qualification"], false);
        assert_eq!(result["execution"], "screen");
        let before = std::fs::read(&counter).unwrap();
        assert_eq!(before, b"call\n".repeat(6));
        let freshness = std::fs::read(output.join("freshness.json")).unwrap();
        assert_eq!(
            serde_json::from_slice::<Value>(&freshness).unwrap()["prior_proof_sha256"]
                .as_array()
                .unwrap()
                .len(),
            7
        );
        let config: Config =
            serde_json::from_slice(&std::fs::read(output.join("protocol.json")).unwrap()).unwrap();
        evaluate_screen(config, &output, true).unwrap();
        assert_eq!(std::fs::read(&counter).unwrap(), before);
        assert_eq!(
            std::fs::read(output.join("freshness.json")).unwrap(),
            freshness
        );
        let config: Config =
            serde_json::from_slice(&std::fs::read(output.join("protocol.json")).unwrap()).unwrap();
        assert!(evaluate(config, &output, true)
            .unwrap_err()
            .to_string()
            .contains("cannot resume"));
        let mut config: Config =
            serde_json::from_slice(&std::fs::read(output.join("protocol.json")).unwrap()).unwrap();
        config.verifier[2] = "reject".into();
        let failed = root.join("failed");
        assert!(evaluate_screen(config, &failed, false).is_err());
        assert!(failed.join("failure.json").exists());
        assert!(!failed.join("result.json").exists());
        std::fs::remove_dir_all(root).unwrap();
    }
}
