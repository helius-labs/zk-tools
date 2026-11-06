use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-queue-index-composition176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn digest(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn near(a: f64, b: f64) {
    assert!((a - b).abs() <= 1e-12 * a.abs().max(b.abs()).max(1.));
}
fn center(v: &mut [f64]) -> f64 {
    *v.select_nth_unstable_by(v.len() / 2, f64::total_cmp).1
}
fn collect(p: &Path, out: &mut Vec<PathBuf>) {
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            collect(&p, out)
        } else {
            out.push(p);
        }
    }
}
fn main() {
    assert!(!r("review.json").exists());
    let a = load(r("analysis.json"));
    let jobs = load(r("jobs.json"));

    let sourceaudit=load(r("source-audit.json"));
    let parentaudit=load("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/source-audit.json");
    let manifest=load(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    let mut files=vec![];collect(&r("source"),&mut files);assert_eq!(files.len(),180);
    let mut exact=0;
    for(p,h)in manifest["source_files_sha256"].as_object().unwrap(){let original=fs::read(Path::new(Z).join(p)).unwrap();assert_eq!(digest(Path::new(Z).join(p)),h.as_str().unwrap());let actual=fs::read(r("source").join(p)).unwrap();if actual==original{exact+=1;}let mut expected=original;if let Some(t)=parentaudit["suffixes"][p].as_str(){expected.extend_from_slice(t.as_bytes());}for e in sourceaudit["edits"].as_array().unwrap().iter().filter(|e|e["file"]==p.as_str()){if let Some(t)=e["suffix"].as_str(){expected.extend_from_slice(t.as_bytes());}else{let text=String::from_utf8(expected).unwrap();let old=e["old"].as_str().unwrap();assert_eq!(text.matches(old).count(),1);expected=text.replacen(old,e["new"].as_str().unwrap(),1).into_bytes();}}assert_eq!(actual,expected,"forward production reconstruction {p}");}assert_eq!(exact,170);
    for p in["crates/zopro-core/src/msm/contained_index.rs","crates/zopro-core/src/msm_affine/g2_phi/contained_indexed.rs"]{assert_eq!(fs::read(r("source").join(p)).unwrap(),fs::read(Path::new("/home/ubuntu/src/zopro-perf-work/g2-contained-index176/source").join(p)).unwrap());}
    assert_eq!(digest(r("source/crates/zopro-core/src/msm_affine/queue_drain.rs")),digest("/home/ubuntu/src/zopro-perf-work/queue-drain175/queue_drain.rs"));
    let queue=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/queue-drain175/g2-suffix.rs").unwrap().replace("cfg!(zopro_queue_drain)","(cfg!(zopro_queue_drain)&&crate::queue_index_composition::drain_enabled())");assert!(fs::read_to_string(r("source/crates/zopro-core/src/msm_affine/g2_borrowed.rs")).unwrap().ends_with(&queue));
    for(name,c)in parentaudit["clones"].as_object().unwrap(){let mut restored=c["candidate"].as_str().unwrap().to_owned();for change in c["edits"].as_array().unwrap().iter().rev(){restored=restored.replacen(change[1].as_str().unwrap(),change[0].as_str().unwrap(),1);}assert_eq!(restored,c["original"]);let(old,new)=match name.as_str(){"query"=>("crates/zopro-core/src/msm_affine.rs","crates/zopro-core/src/msm_affine.rs"),"old_global_packing"=>("docs/benchmarks/2026-10-29-g2-packed-index175/artifacts/source/crates/zopro-core/src/msm/packed_index.rs","crates/zopro-core/src/msm/contained_index.rs"),_=>("crates/zopro-core/src/msm_affine/g2_phi.rs","crates/zopro-core/src/msm_affine/g2_phi/contained_indexed.rs")};assert!(fs::read_to_string(Path::new(Z).join(old)).unwrap().contains(&restored));assert!(fs::read_to_string(r("source").join(new)).unwrap().contains(c["candidate"].as_str().unwrap()));}
    let d=load(r("driver-audit.json"));let mut driver=fs::read_to_string(r("driver/src/main.rs")).unwrap();assert_eq!(driver,fs::read_to_string(r("driver.rs")).unwrap());for e in d["edits"].as_array().unwrap().iter().rev(){let from=e["new"].as_str().unwrap();assert_eq!(driver.matches(from).count(),1);driver=driver.replacen(from,e["old"].as_str().unwrap(),1);}assert_eq!(driver,fs::read_to_string(d["parent"].as_str().unwrap()).unwrap());assert_eq!(format!("{:x}",Sha256::digest(driver.as_bytes())),d["parent_sha256"]);
    let ids=load(r("identities.json"));let binary_hash=digest(r("candidate-bin"));for arm in["ORIGINAL","DRAIN","PACK","BOTH"]{assert_eq!(ids[arm]["binary_sha256"],binary_hash);assert_eq!(ids[arm]["binary"],r("candidate-bin").display().to_string());let actual=ids[arm]["actual_rustc"].as_str().unwrap();assert!(fs::read_to_string(r("native/driver-build/stderr.log")).unwrap().lines().any(|l|l==actual));for flag in["-C opt-level=3","-C lto=fat","-C codegen-units=1","-C target-cpu=native","--cfg zopro_queue_drain","--cfg zopro_queue_index_composition","--cfg zopro_contained_index"]{assert!(actual.contains(flag));}}
    let mut pins=0;for f in["build-pins.json","capture-pins.json"]{for(p,h)in load(r(f)).as_object().unwrap(){assert_eq!(digest(p),h.as_str().unwrap());pins+=1;}}
    for line in fs::read_to_string(r("analysis-pins.sha256")).unwrap().lines(){let(h,p)=line.split_once("  ").unwrap();assert_eq!(digest(p),h);}
    let mut commands = 0;
    for (plan, out) in [
        ("build-plan.json", "native"),
        ("capture-plan.json", "runs"),
    ] {
        for s in load(r(plan))["steps"].as_array().unwrap() {
            let base = r(out).join(s["name"].as_str().unwrap());
            let e = load(base.join("exit.json"));
            assert_eq!(e["returncode"], 0);
            assert_eq!(e["success"], true);
            for f in ["stdout", "stderr"] {
                assert_eq!(
                    digest(base.join(format!("{f}.log"))),
                    e[format!("{f}_sha256")]
                );
            }
            let cmd = load(base.join("command.json"));
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                s["argv"].as_array().unwrap()
            );
            let mut expected_env = s.get("environment").cloned().unwrap_or(json!({}));
            // process::run records only config::managed variables. RUSTC is
            // passed to the process but omitted from this receipt projection.
            // The actual compiler argv is independently checked at freeze.
            expected_env.as_object_mut().unwrap().remove("RUSTC");
            assert_eq!(cmd["environment"], expected_env);
            commands += 1;
        }
    }

    for tag in["OFF","ON"]{let native=fs::read_to_string(r(format!("native/core-drain-{tag}/stdout.log"))).unwrap();assert!(native.contains("130 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));let chosen=native.lines().filter(|l|l.starts_with("test ")&&(l.contains("contained_tests::")||l.contains("queue_drain_tests::")||l.contains("queue_drain::tests::"))).collect::<Vec<_>>();assert_eq!(chosen.len(),5);assert!(chosen.iter().all(|l|l.ends_with(" ... ok")));}
    let mut callers = BTreeMap::new();
    let mut allcalls = 0;
    let mut reusedcalls = 0;
    for j in jobs.as_array().unwrap() {
        let base = j["reused_from"]
            .as_str()
            .map(PathBuf::from)
            .unwrap_or_else(|| r("runs").join(j["name"].as_str().unwrap()));
        let exit = load(base.join("exit.json"));
        assert_eq!(exit["success"], true);
        for f in ["stdout", "stderr"] {
            assert_eq!(
                digest(base.join(format!("{f}.log"))),
                exit[format!("{f}_sha256")]
            );
        }
        let text = fs::read_to_string(base.join("stdout.log")).unwrap();
        let lines = text
            .lines()
            .filter_map(|l| l.strip_prefix("CALLER "))
            .collect::<Vec<_>>();
        assert_eq!(lines.len(), 1);
        let v: Value = serde_json::from_str(lines[0]).unwrap();
        for k in [
            "every_output_matches_independent_full_MSM",
            "all_timed_outputs_checked",
            "point_inputs_unchanged",
            "normalization_inside_timer",
        ] {
            assert_eq!(v[k], true);
        }
        assert_eq!(v["query"], j["actual_query"]);
        assert_eq!(v["composition"]["drain_enabled"], ["DRAIN","BOTH"].contains(&j["variant"].as_str().unwrap()));
        assert_eq!(v["composition"]["pack_enabled"], ["PACK","BOTH"].contains(&j["variant"].as_str().unwrap()));
        assert_eq!(v["rounds"], j["rounds"]);
        assert_eq!(v["workers"], j["case"]["workers"]);
        assert_eq!(
            v["metadata"]["route"],
            if ["PACK","BOTH"].contains(&j["variant"].as_str().unwrap()) && j["case"]["query"] == "B" {
                336
            } else {
                0
            }
        );
        let n = j["rounds"].as_u64().unwrap() + 4;
        allcalls += n;
        if j.get("reused_from").is_some() {
            reusedcalls += n;
            let cmd = load(base.join("command.json"));
            assert_eq!(cmd["environment"], j["command"]["environment"]);
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                j["command"]["argv"].as_array().unwrap()
            );
        }
        callers.insert(j["name"].as_str().unwrap().to_owned(), v);
    }
    let mut failures = vec![];
    let mut ratios = BTreeMap::<(String, String), Vec<f64>>::new();
    for row in a["rows"].as_array().unwrap() {
        let case = row["case"].as_str().unwrap();
        let mut times = BTreeMap::<String, Vec<f64>>::new();
        for arm in ["ORIGINAL", "DRAIN", "PACK", "BOTH"] {
            let mut v = vec![];
            let mut rss = 0f64;
            let mut cpu = vec![];
            for b in 0..15 {
                let c = &callers[&format!("latency-b{b:02}-{case}-{arm}")];
                let mut samples = c["samples_ns"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_u64().unwrap() as f64)
                    .collect::<Vec<_>>();
                let rounds = if case.contains("-A-") { 47 } else { 17 };
                assert_eq!(samples.len(), rounds);
                v.push(center(&mut samples));
                rss = rss.max(c["usage"]["whole_process_maxrss_kib"].as_f64().unwrap() / 1024.);
                cpu.push(
                    (c["usage"]["user_cpu_ms"].as_f64().unwrap()
                        + c["usage"]["system_cpu_ms"].as_f64().unwrap())
                        / rounds as f64,
                );
            }
            let mut v2 = v.clone();
            near(
                center(&mut v2) / 1e6,
                row["median_ms"][arm].as_f64().unwrap(),
            );
            near(rss, row["max_process_RSS_MiB"][arm].as_f64().unwrap());
            near(
                center(&mut cpu),
                row["median_CPU_ms_per_query"][arm].as_f64().unwrap(),
            );
            times.insert(arm.into(), v);
        }
        for (num, den) in [("DRAIN","ORIGINAL"),("PACK","ORIGINAL"),("BOTH","ORIGINAL"),("PACK","DRAIN"),("BOTH","DRAIN"),("BOTH","PACK")] {
            let name = format!("{num}/{den}");
            let v = times[num]
                .iter()
                .zip(&times[den])
                .map(|(a, b)| a / b)
                .collect::<Vec<_>>();
            let wins = v.iter().filter(|&&r| r < 1.).count();
            let mut sorted = v.clone();
            sorted.sort_by(f64::total_cmp);
            let s = &row["comparisons"][&name];
            near(sorted[7], s["median"].as_f64().unwrap());
            near(sorted[3], s["low"].as_f64().unwrap());
            near(sorted[11], s["high"].as_f64().unwrap());
            assert_eq!(wins, s["wins"].as_u64().unwrap() as usize);
            let pass = if case.contains("-A-") {
                sorted[3] >= 1. / 1.03 && sorted[11] <= 1.03
            } else {
                sorted[11] <= 1.03
            };
            assert_eq!(s["declared_check_passed"], pass);
            let selected=case.contains("-A-")||(num=="BOTH"&&["ORIGINAL","DRAIN"].contains(&den));
            assert_eq!(s["selection_gate"], selected);
            if selected && !pass {
                failures.push(format!(
                    "{case}/{num}/{den}/{}",
                    if case.contains("-A-") {
                        "unchanged-A-stability"
                    } else {
                        "nonregression"
                    }
                ));
            }
            if num == "BOTH" && ["ORIGINAL","DRAIN"].contains(&den) && case == "1x2-B-w32" && (sorted[11] >= 1. || wins < 10) {
                failures.push(format!("{case}/{num}/{den}/primary-gain"));
            }
            ratios.insert((case.into(), name), v);
        }
    }

    for(num,den)in[("DRAIN","ORIGINAL"),("PACK","ORIGINAL"),("BOTH","ORIGINAL"),("PACK","DRAIN"),("BOTH","DRAIN"),("BOTH","PACK")]{let name=format!("{num}/{den}");let mut values=(0..15).map(|b|(ratios[&("1x2-B-w32".into(),name.clone())][b]*ratios[&("2x2-B-w32".into(),name.clone())][b]).sqrt()).collect::<Vec<_>>();let wins=values.iter().filter(|&&r|r<1.).count();values.sort_by(f64::total_cmp);let stat=&a["aggregates"][&name];near(values[7],stat["median"].as_f64().unwrap());near(values[3],stat["low"].as_f64().unwrap());near(values[11],stat["high"].as_f64().unwrap());assert_eq!(wins,stat["wins"].as_u64().unwrap()as usize);if num=="BOTH"&&["ORIGINAL","DRAIN"].contains(&den)&&(values[11]>=1.||wins<10){failures.push(format!("B32-two-shape/{num}/{den}/gain"));}}
    let mut samples_all = 0;
    for j in jobs
        .as_array()
        .unwrap()
        .iter()
        .filter(|j| j["panel"] == "stat" || j["panel"] == "cycles")
    {
        let name = j["name"].as_str().unwrap();
        if j["panel"] == "stat" {
            let mut seen = BTreeSet::new();
            for line in fs::read_to_string(j["data"].as_str().unwrap())
                .unwrap()
                .lines()
            {
                let f = line.split(';').collect::<Vec<_>>();
                if f.len() > 4 && a["counters"][name].get(f[2].trim()).is_some() {
                    let key = f[2].trim();
                    let count = f[0].trim().parse::<u64>().unwrap();
                    assert_eq!(count, a["counters"][name][key]["count"]);
                    near(
                        count as f64 / 200.,
                        a["counters"][name][key]["per_query"].as_f64().unwrap(),
                    );
                    assert!(f[4].trim().parse::<f64>().unwrap() >= 99.9);
                    seen.insert(key.to_owned());
                }
            }
            assert_eq!(seen.len(), 4);
            continue;
        }
        let v = &callers[name];
        let pid = v["pid"].as_u64().unwrap();
        let threads = v["worker_rows"]
            .as_array()
            .unwrap()
            .iter()
            .map(|r| (r["tid"].as_u64().unwrap(), r["cpu"].as_u64().unwrap()))
            .collect::<BTreeMap<_, _>>();
        let mut count = 0;
        let mut unknown = 0;
        for line in fs::read_to_string(r(format!("runs/decode-{name}/stdout.log")))
            .unwrap()
            .lines()
            .filter(|l| !l.starts_with('#') && !l.trim().is_empty())
        {
            let f = line.split_whitespace().collect::<Vec<_>>();
            let ids = f[0]
                .split('/')
                .map(|x| x.parse::<u64>().unwrap())
                .collect::<Vec<_>>();
            assert_eq!(ids[0], pid);
            let cpu = f[1].trim_matches(['[', ']']).parse::<u64>().unwrap();
            assert!(cpu < 32);
            if ids[1] != pid {
                assert_eq!(threads[&ids[1]], cpu);
            }
            assert_eq!(f[3], "16000000");
            if f[5] == "[unknown]" {
                unknown += 1;
            }
            count += 1;
        }
        assert_eq!(count, a["profiles"][name]["samples"].as_u64().unwrap());
        assert_eq!(unknown, a["profiles"][name]["unknown"].as_u64().unwrap());
        assert!(count >= 1000 && unknown as f64 / count as f64 <= 0.05);
        assert_eq!(
            digest(j["data"].as_str().unwrap()),
            a["profiles"][name]["raw_sha256"]
        );
        let mut child = Command::new("perf")
            .args(["script", "-D", "-i", j["data"].as_str().unwrap()])
            .stdout(Stdio::piped())
            .spawn()
            .unwrap();
        let mut lost = 0;
        for l in BufReader::new(child.stdout.take().unwrap()).lines() {
            if l.unwrap().contains("PERF_RECORD_LOST") {
                lost += 1;
            }
        }
        assert!(child.wait().unwrap().success());
        assert_eq!(lost, 0);
        samples_all += count;
    }
    assert_eq!(commands, 481);
    assert_eq!(pins, a["pin_occurrences_checked"]);
    assert_eq!(a["failed_checks"], json!(failures));
    assert_eq!(a["gate_passed"], false);
    assert_eq!(allcalls, 15688);
    assert_eq!(reusedcalls, 0);
    assert_eq!(samples_all, 77807);
    let result = json!({"passed":true,"original_production176_unchanged":true,"snapshot_files":180,"originals_exact":170,"all_originals_forward_reconstructed":true,"five_clones_inverse_and_actual_bodies_checked":true,"native_tests_each_drain_mode":130,"native_drain_modes":2,"new_native_tests":0,"pin_occurrences_checked":pins,"successful_workflow_commands":commands,"new_capture_commands":477,"reused_preflight_receipts":0,"reused_environment_and_argv_exact":true,"all_actual_command_argv_checked":true,"caller_rows":456,"complete_query_calls":allcalls,"new_query_calls":allcalls-reusedcalls,"all_timing_CPU_RSS_gates_reconstructed":true,"PMU_counts_reparsed":true,"profile_samples_reparsed":samples_all,"raw_loss_checks":4,"zero_lost":true,"component_gate_passed":false,"failed_checks":failures,"adopted":false,"new_proofs":0,"canonical_proof_ledger":30846,"goal_achieved":false});
    fs::write(
        r("review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("{result}");
}
