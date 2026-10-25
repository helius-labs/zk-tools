use crate::{
    config::{expand, Config},
    process::{fifo, json, run, sha256, HostLock, PinGuard},
    results::{markdown, paired_summary, perf_samples, profile_summary, sdk_output, Row},
    Result,
};
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

pub fn evaluate(config: Config, output: &Path, resume: bool) -> Result<Value> {
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
    } else {
        if let Some(parent) = output.parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::create_dir(output)?;
        json(output.join("protocol.json"), &config)?;
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
        for current_phase in ["latency", "profile"] {
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
        let result = value!({"status":"complete","latency":latency,"references":references,
            "profile_comparison":profile_summary(&rows,&config.variants[0].name)?,
            "processes":rows.len(),"distinct_verified_exports":inventory.len()/2,
            "profiles":rows.iter().filter(|row|row.phase=="profile").count(),"qualification":false});
        json(output.join("result.json"), &result)?;
        std::fs::write(output.join("report.md"), markdown(&result))?;
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
