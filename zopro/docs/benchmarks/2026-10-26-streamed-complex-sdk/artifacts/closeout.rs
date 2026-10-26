use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader, Read},
    path::Path,
};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn receipt(p: impl AsRef<Path>) {
    let p = p.as_ref();
    let e = read(p.join("exit.json"));
    assert_eq!(e["success"], true, "{}", p.display());
    assert_eq!(e["returncode"], 0);
    for s in ["stdout", "stderr"] {
        assert_eq!(
            hash(p.join(format!("{s}.log"))),
            e[format!("{s}_sha256")].as_str().unwrap()
        );
    }
}
fn check_pins(v: &Value) {
    for (p, h) in v.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap(), "{p}");
    }
}
fn pct(v: &Value, total: u64) -> f64 {
    v["period"].as_u64().unwrap_or(0) as f64 * 100. / total as f64
}
fn main() {
    let root = Path::new("/tmp/zopro-streamed-complex-sdk");
    let config = read(root.join("perf-plan.json"));
    assert_eq!(config["pins"].as_object().unwrap().len(), 394);
    check_pins(&config["pins"]);
    for f in [
        "build-input-pins.json",
        "input-pins.json",
        "instruction-analysis-pins.json",
    ] {
        check_pins(&read(root.join(f)));
    }
    let base = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (n, h) in base["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(n)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(
        hash("/tmp/zopro-grouping-module/native/candidate-sdk"),
        "03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d"
    );
    fs::copy(
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        root.join("accepted-manifest.json"),
    )
    .unwrap();
    let audit = read(root.join("source-audit.json"));
    for item in audit["changed"].as_array().unwrap() {
        let name = item["path"].as_str().unwrap();
        let mut text = fs::read_to_string(root.join("candidate").join(name)).unwrap();
        if name.contains("zopro-bn254") {
            let begin = "    // BEGIN STREAMED_COMPLEX_B\n";
            let end = "    // END STREAMED_COMPLEX_B\n\n";
            let start = text.find(begin).unwrap();
            let stop = text.find(end).unwrap() + end.len();
            let component =
                fs::read_to_string(Path::new("/tmp/zopro-streamed-complex-b/candidate").join(name))
                    .unwrap();
            let cs = component.find(begin).unwrap();
            let ce = component.find(end).unwrap() + end.len();
            assert_eq!(&text[start..stop], &component[cs..ce]);
            text.replace_range(start..stop, "");
        } else if name.ends_with("simd.rs") {
            let added = audit["core_edits"]
                .as_array()
                .unwrap()
                .iter()
                .find(|e| e["path"] == name && !e["added"].is_null())
                .unwrap()["added"]
                .as_str()
                .unwrap();
            assert_eq!(text.matches(added).count(), 1);
            text = text.replace(added, "");
        } else {
            for edit in audit["core_edits"]
                .as_array()
                .unwrap()
                .iter()
                .rev()
                .filter(|e| e["path"] == name && !e["to"].is_null())
            {
                let to = edit["to"].as_str().unwrap();
                assert_eq!(text.matches(to).count(), 1);
                text = text.replace(to, edit["from"].as_str().unwrap());
            }
        }
        assert_eq!(
            format!("{:x}", Sha256::digest(text.as_bytes())),
            base["source_files_sha256"][name].as_str().unwrap()
        );
    }
    assert_eq!(read(root.join("native/result.json"))["status"], "complete");
    for step in read(root.join("native-plan.json"))["steps"]
        .as_array()
        .unwrap()
    {
        if step["kind"] == "command" {
            receipt(root.join("native").join(step["name"].as_str().unwrap()));
        }
    }
    for label in ["off", "on"] {
        assert_eq!(
            read(root.join(format!("native/{label}-core/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            109
        );
        assert_eq!(
            read(root.join(format!("native/{label}-prover/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
    }
    let pre = read(root.join("preflight/result.json"));
    assert_eq!(pre["status"], "complete");
    assert_eq!(pre["steps"].as_array().unwrap().len(), 44);
    for step in pre["steps"].as_array().unwrap() {
        let name = step.as_str().unwrap();
        let dir = root.join("preflight").join(name);
        receipt(&dir);
        if name.ends_with("-verify") {
            assert!(fs::read_to_string(dir.join("stdout.log"))
                .unwrap()
                .contains("verified 1 distinct proofs across 1 pinned keys"));
        }
    }
    for mode in ["accepted", "off", "on"] {
        assert_eq!(
            fs::read_to_string(
                root.join(format!("preflight/{mode}-layout-observation/stdout.log"))
            )
            .unwrap()
            .trim(),
            "NATIVE_PLAN_BYTES=1728 ALIGN=32"
        );
    }
    let template = read(root.join("all-nine-template.json"));
    let prior: BTreeSet<String> =
        serde_json::from_value(template["prior_proof_sha256"].clone()).unwrap();
    let perf_prior: BTreeSet<String> =
        serde_json::from_value(config["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(prior.len(), 4567);
    assert_eq!(perf_prior.len(), 4585);
    let fixtures = template["cases"]
        .as_array()
        .unwrap()
        .iter()
        .map(|c| {
            (
                Path::new(c["image"].as_str().unwrap())
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_owned(),
                c["public_sha256"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<BTreeMap<_, _>>();
    let result = read(root.join("perf/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["distinct_verified_exports"], 219);
    let rows = read(root.join("perf/rows.json"));
    assert_eq!(rows.as_array().unwrap().len(), 219);
    let (mut sdk, mut latency, mut samples) = (0u64, 0usize, 0u64);
    let mut captures = BTreeSet::new();
    let mut profiles = Vec::new();
    for row in rows.as_array().unwrap() {
        let phase = row["phase"].as_str().unwrap();
        let case_name = row["case"].as_str().unwrap();
        let variant = row["variant"].as_str().unwrap();
        let block = row["block"].as_u64().unwrap();
        let dir = root.join(format!("perf/{phase}/b{block}/{case_name}/{variant}"));
        receipt(dir.join("run"));
        receipt(dir.join("verify"));
        assert!(fs::read_to_string(dir.join("verify/stdout.log"))
            .unwrap()
            .contains("verified 1 distinct proofs across 1 pinned keys"));
        let case = config["cases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|c| c["name"] == case_name)
            .unwrap();
        let count = if phase == "profile" {
            case["profile_rounds"].as_u64().unwrap()
        } else {
            case["rounds"].as_u64().unwrap()
        };
        assert_eq!(row["samples_ms"].as_array().unwrap().len(), count as usize);
        assert_eq!(row["summary"]["rounds"], count);
        assert_eq!(row["summary"]["priming"], 3);
        assert_eq!(row["summary"]["workers"], case["workers"]);
        sdk += count + 3;
        let cmd = read(dir.join("run/command.json"));
        let argv = cmd["argv"].as_array().unwrap();
        let binary = &config["variants"]
            .as_array()
            .unwrap()
            .iter()
            .find(|v| v["name"] == variant)
            .unwrap()["binary"];
        assert!(argv
            .windows(3)
            .any(|a| a[0] == *binary && a[1] == case["image"] && a[2] == case["request"]));
        assert!(argv
            .windows(2)
            .any(|a| a[0] == "-c" && a[1] == case["cpu_affinity"]));
        if phase == "profile" {
            receipt(dir.join("symbols"));
            assert_eq!(row["profile"]["lost_records"], 0);
            assert_eq!(row["profile"]["sampled_pids"].as_array().unwrap().len(), 1);
            samples += row["profile"]["samples"].as_u64().unwrap();
            assert!(captures.insert((case_name.to_owned(), variant.to_owned(), block)));
            assert!(argv.iter().any(|a| a == "--no-buildid-cache"));
            assert!(argv
                .windows(2)
                .any(|a| a[0] == "--clockid" && a[1] == "mono"));
            assert!(argv
                .iter()
                .any(|a| a.as_str().is_some_and(|s| s.starts_with("--control=fifo:"))));
            profiles.push(json!({"case":case_name,"variant":variant,"samples":row["profile"]["samples"],"period":row["profile"]["period"],"cpu_ms_per_request":(row["summary"]["user_cpu_ms"].as_f64().unwrap()+row["summary"]["system_cpu_ms"].as_f64().unwrap())/count as f64,"mean_wall_ms":row["summary"]["loop_wall_ms"].as_f64().unwrap()/count as f64,"category_percent":row["profile"]["category_percent"],"lost_records":0,"scope":"One warm user-cycle capture; sample categories are not exclusive costs or removable walltime."}));
        } else {
            assert_eq!(phase, "latency");
            latency += 1;
        }
    }
    assert_eq!(sdk, 9120);
    assert_eq!(latency, 210);
    assert_eq!(samples, 301177);
    assert_eq!(captures.len(), 9);
    for name in ["1x2-w32", "12x2-w32", "merge-54x1-w32"] {
        for variant in ["accepted", "off", "on"] {
            assert!(captures.contains(&(name.into(), variant.into(), 0)));
        }
    }
    let mut inventory = BTreeMap::new();
    for (p, h) in read(root.join("preflight-proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        assert!(inventory
            .insert(p.clone(), h.as_str().unwrap().to_owned())
            .is_none());
    }
    for (p, h) in read(root.join("perf/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        let p = root.join("perf").join(p);
        assert_eq!(hash(&p), h.as_str().unwrap());
        assert!(inventory
            .insert(p.to_str().unwrap().into(), h.as_str().unwrap().to_owned())
            .is_none());
    }
    assert_eq!(inventory.len(), 474);
    let mut unique = BTreeSet::new();
    for (path, digest) in &inventory {
        let p = Path::new(path);
        if p.extension().is_some_and(|e| e == "proof") {
            assert!(unique.insert(digest));
            assert!(!prior.contains(digest));
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
            assert!(inventory.contains_key(p.with_extension("public").to_str().unwrap()));
            if p.starts_with(root.join("perf")) {
                assert!(!perf_prior.contains(digest));
            }
        } else {
            assert_eq!(p.extension().unwrap(), "public");
            let stem = p.parent().unwrap().file_name().unwrap().to_str().unwrap();
            assert_eq!(&fixtures[stem], digest);
        }
    }
    assert_eq!(unique.len(), 237);
    let instruction_result = read(root.join("instructions/result.json"));
    assert_eq!(instruction_result["status"], "complete; diagnostic only");
    assert_eq!(instruction_result["total"]["samples"], 49666);
    let mut bins = Vec::new();
    let mut counted = 0u64;
    for label in ["accepted", "off", "on"] {
        let file = read(root.join(format!("instructions/primary-{label}.json")));
        let mut count = 0u64;
        let mut period = 0u64;
        for line in BufReader::new(
            fs::File::open(root.join(format!("instructions/primary-{label}-ips.jsonl"))).unwrap(),
        )
        .lines()
        {
            let row: Value = serde_json::from_str(&line.unwrap()).unwrap();
            count += row["tally"]["samples"].as_u64().unwrap();
            period += row["tally"]["period"].as_u64().unwrap();
        }
        assert_eq!(count, file["summary"]["total"]["samples"].as_u64().unwrap());
        assert_eq!(period, file["summary"]["total"]["period"].as_u64().unwrap());
        counted += count;
        for (scope, b) in file["scopes"].as_object().unwrap() {
            let total = b["total"]["period"].as_u64().unwrap();
            bins.push(json!({"variant":label,"scope":scope,"samples":b["total"]["samples"],"period":total,"share_of_capture_percent":total as f64*100./period as f64,"within_scope_percent":{"ifma":pct(&b["families"]["ifma"],total),"vector_add_sub":pct(&b["families"]["vector_add_sub"],total),"vector_move":pct(&b["families"]["vector_move"],total),"vector_logic":pct(&b["families"]["vector_logic"],total),"rsp_operand":pct(&b["memory"]["rsp_operand"],total),"rbp_operand":pct(&b["memory"]["rbp_operand"],total)}}));
        }
    }
    assert_eq!(counted, 49666);
    let pilot = read(root.join("pilot-result.json"));
    assert_eq!(pilot["performance_gate_passed"], false);
    assert_eq!(pilot["failed_checks"].as_array().unwrap().len(), 41);
    assert!(pilot["cases"]
        .as_object()
        .unwrap()
        .values()
        .all(|v| v["off_calibration_factor"].as_f64().unwrap() <= 1.03));
    save(
        root.join("instruction-summary.json"),
        &json!({"samples":counted,"all_sample_counts_and_periods_independently_conserve":true,"bins":bins,"scope":"All three retained primary captures after fullSDKscreen. Exact ELF/MMAP/bytes association by qualifiedRustreader, no new captures. Sampling may skid; operand tags are not traffic/spill counts or exclusivecosts."}),
    );
    save(
        root.join("profile-analysis.json"),
        &json!({"samples":samples,"lost_records":0,"profiles":profiles}),
    );
    save(root.join("proof-inventory.json"), &json!(inventory));
    let out = json!({"status":"closed_verified; fixed SDK speed gate failed; not adopted","adopted":false,"goal_achieved":false,"sdk_requests":sdk+72,"distinct_gnark_exports":237,"proof_public_members":474,"latency_processes":210,"profiles":9,"profile_samples":samples,"lost_records":0,"profile_tuples_rounds_clock_FIFO_flags_verified":true,"performance_pins":394,"core_tests_each_mode":109,"raw_tests_each_mode":2,"cubic_tests_each_mode":6,"inherited_field_oracles_byte_identical":true,"inherited_exact_field_outputs":91136,"inherited_ark_chain_outputs":729088,"changed_source_files":4,"unchanged_original_files":166,"all_four_whole_originals_recover_by_documented_inverse_edits":true,"accepted_170_files_and_sdk_unchanged":true,"workspace_key_plan_scheduler_solver_unchanged":true,"plan_abi_all_three":{"bytes":1728,"align":32},"prior_before_preflight":4567,"prior_before_perf":4585,"failed_checks":pilot["failed_checks"],"all_off_calibrations_pass":true,"primary32_pair_on_accepted":pilot["cases"]["1x2-w32"]["on_over_control"]["accepted"]["paired_median"],"primary32_pair_on_off":pilot["cases"]["1x2-w32"]["on_over_control"]["off"]["paired_median"],"original8_pairs":pilot["original8_aggregate"],"posthoc_primary_instruction_samples":counted,"no_new_pressure_portable_mobile_giant_AWS_Aeglos_or_minimumRAM_qualification":true,"owned_jobs":"native60659, preflight78823, performance98313, offlineinstructions94782 all terminal/reaped"});
    save(root.join("closeout.json"), &out);
    println!("CLOSED9192SDK237Gnark474members394pins9warmprofiles{samples}samples0lost;41speedfails;3primary49666instructions conserved; acceptedunchanged");
}
