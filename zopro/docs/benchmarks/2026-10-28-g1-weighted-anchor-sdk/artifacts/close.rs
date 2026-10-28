use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, io::Read, path::Path, process::Command};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor-sdk";
const P: &str = "/home/ubuntu/src/zopro-perf-work/g1-weighted-anchor";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-28-g1-weighted-anchor-sdk";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
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
fn copy(root: &Path, dir: &Path, out: &Path) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let n = e.file_name().to_string_lossy().to_string();
        let t = e.file_type().unwrap();
        if t.is_dir() {
            if n == "source"
                || n.starts_with("target-")
                || ["profiles", "frozen"].contains(&n.as_str())
            {
                continue;
            }
            copy(root, &p, out);
        } else if t.is_file()
            && ["rs", "md", "json", "jsonl", "log", "txt", "proof", "public"]
                .contains(&p.extension().and_then(|s| s.to_str()).unwrap_or(""))
        {
            let dest = out.join(p.strip_prefix(root).unwrap());
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(p, dest).unwrap();
        }
    }
}
fn inventory(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            inventory(root, &p, out)
        } else {
            let rel = p.strip_prefix(root).unwrap().display().to_string();
            if rel != "SHA256SUMS" {
                out.insert(rel, hash(p));
            }
        }
    }
}
fn archive() {
    assert!(!Path::new(D).exists());
    let root = Path::new(R);
    let a = read(root.join("analysis.json"));
    assert_eq!(a["independent_proof_count"], 13082);
    assert_eq!(a["SDK_requests"], 15396);
    let table = read(root.join("tables.json"));
    let pins = read(root.join("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m=read(format!("{Z}/docs/benchmarks/2026-10-27-clean-fft-adoption/artifacts/production-source-manifest.json"));
    let audit = read(root.join("source-audit.json"));
    let mut recovered =
        fs::read_to_string(root.join("source/crates/zopro-core/src/msm.rs")).unwrap();
    assert!(!recovered.contains("caller_probe"));
    let parent_audit = read(format!("{P}/source-audit.json"));
    let parent = fs::read_to_string(format!("{P}/source/crates/zopro-core/src/msm.rs")).unwrap();
    assert_eq!(
        recovered,
        parent
            .strip_suffix(parent_audit["msm_suffix"].as_str().unwrap())
            .unwrap()
    );
    for e in audit["msm_edits"].as_array().unwrap().iter().rev() {
        assert_eq!(recovered.matches(e["new"].as_str().unwrap()).count(), 1);
        recovered = recovered.replace(e["new"].as_str().unwrap(), e["old"].as_str().unwrap());
    }
    assert_eq!(
        recovered,
        fs::read_to_string(format!("{Z}/crates/zopro-core/src/msm.rs")).unwrap()
    );
    for (f, h) in m["source_files_sha256"].as_object().unwrap() {
        assert_eq!(hash(Path::new(Z).join(f)), h.as_str().unwrap());
        if f != "crates/zopro-core/src/msm.rs" {
            assert_eq!(hash(root.join("source").join(f)), h.as_str().unwrap());
        }
    }
    assert_eq!(
        hash("/home/ubuntu/src/zopro-perf-work/batch-local-steal/native/control-sdk"),
        "14da8231f5797db25d3f45e3570a73c38141bfb957295d2548b0e457676e612a"
    );
    for n in ["off", "on"] {
        assert_eq!(a["core_tests_per_configuration"][n], 119);
    }
    let gate = a["gate_passed"].as_bool().unwrap();
    let status = if gate {
        "passes the primary SDK screen; broader qualification is pending"
    } else {
        "does not pass the primary SDK screen and is not adopted"
    };
    let w32 = &a["pairs"]["w32/on/off"];
    let w32a = &a["pairs"]["w32/on/control"];
    let mut report=format!("# G1 weighted Y anchor in full SDK proofs\n\nThe candidate **{status}**. At 32 workers, ON/OFF paired median is **{:.9}**, marginal bounds [{:.9}, {:.9}], {}/15 wins. ON/accepted is **{:.9}**, bounds [{:.9}, {:.9}], {}/15 wins. All calibration and physical-16 nonregression checks remain in the decision. Production and the accepted executable are unchanged.\n\nThe [caller screen](../2026-10-28-g1-weighted-anchor/README.md) measured about 1.24% aggregate improvement, with larger borrowed-signed than owned-unsigned gains. This study carries the exact runtime formula into pure libraries without diagnostic adapters. With S=Ax+Bx, T=2Ax+Bx and U=lambda², store T in the existing Ax record slot and compute X=U−S, Y=lambda(T−U)−Ay. One extra general vector add per eight pairs shortens the Y dependency chain. No field arithmetic, heap buffer, key table, assembly, new unsafe operation or public runtime option is added.\n\n## Complete verified requests\n\nFifteen fixed rotated/reversed blocks compare accepted SDK14da, fresh ordinary OFF and weighted ON at 32 logical and 16 physical workers. Every latency process has three primes and 151 timed complete requests. Three four-worker preflights precede latency. Image loading is outside clocks; witness construction, constraint solving, fresh randomness, proving, internal verification and output serialization are included. Unrelated ceremony host work is recorded; no exclusive-host claim is made.\n\n| Workers | Variant | Median block ms | CPU ms/request | Median process peak MiB | Maximum process peak MiB |\n|---|---|---:|---:|---:|---:|\n{}\n| Workers | Paired ratio | Median | Marginal bounds | Wins | Check |\n|---|---|---:|---|---:|---|\n{}\nBounds are ranks 4–12 of 15 paired process medians, nominal 96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. Fresh OFF/accepted calibration must fit [1/1.03,1.03] for both worker counts. Primary ON upper bounds must be below parity with at least 11/15 wins against both controls; physical-16 ON upper bounds must be at most 1.02 against both. There is no gain floor, exclusion, schedule extension, successful-capture retry or unchanged reroll. Failed checks: `{}`.\n\n",w32["median"].as_f64().unwrap(),w32["low"].as_f64().unwrap(),w32["high"].as_f64().unwrap(),w32["wins"],w32a["median"].as_f64().unwrap(),w32a["low"].as_f64().unwrap(),w32a["high"].as_f64().unwrap(),w32a["wins"],table["latency"].as_str().unwrap(),table["pairs"].as_str().unwrap(),a["failed_checks"]);
    report.push_str("## Correctness and profiles\n\nBoth native core suites pass 119 tests, including required solver-prefix, refined-component and invalid-witness cases. Both configurations pass two raw-image tests and six cubic prover tests. The parent native 124-test suite covers the new vector formula and alternating reused scratch; field source is unchanged and its previous tests are inherited. Some optional external fixture tests return when fixtures are absent; those external cases are not claimed. The standard SDK and independent Gnark checks exercise the actual 1x2 solver/prover.\n\nThe private source has 171 exact accepted originals and only msm.rs runtime edits. Diagnostic adapters/reexports are absent. Reversing the edits recovers all 172 accepted bytes. OFF and ON share the exact same candidate source; only the private native compile cfg differs. Separate target roots, immediate immutable copies of all transitive rlibs/proc-macro objects, native flags/features and the standard eight-library xtask link including memmap2 are checked. Distinct field hashes reflect compile flags/metadata and are not a field-arithmetic change.\n\nAll 90 latency processes precede six FIFO-gated warm 199 Hz cycles:u profiles, each 251 requests after three primes. All 99 captures finish before all 99 independent Gnark verifications, followed by profile decoding, hashing and archive work. No owned build/helper/hash/archive/second benchmark overlaps capture. Profiles have one sampled SDK PID each and zero lost records. The point-caller matcher covers stream_g1 and BatchAdd symbols; it does not include all inlined point work and is not an exclusive opcode cost or removable proof wall fraction.\n\n| Workers | Variant | Samples | Lost | Matched point-caller period |\n|---|---|---:|---:|---:|\n");
    report.push_str(table["point_profiles"].as_str().unwrap());
    let samples: u64 = a["profiles"]
        .as_object()
        .unwrap()
        .values()
        .map(|p| p["samples"].as_u64().unwrap())
        .sum();
    writeln!(report,"\nTotal samples: {samples}. Raw perf.data stays local with hashes; decoded observations and full symbol weights are archived.\n").unwrap();
    report.push_str("All **15,396** SDK calls verify internally. All **99** fresh exported 164-byte proofs independently pass Gnark and image/public binding and are disjoint from 12,983 prior proof identities, advancing the ledger to **13,082**. Proof/public members are archived. Process peak RSS includes loading and primes; it is not minimum RAM or memory-pressure qualification.\n\n## Accepted comparisons (inherited)\n\nThese are accepted 172-source measurements, not this candidate's all-shape timings. The all-eight 10× CPU objective remains active and unmet, 0/8; primary target is 3.7 ms. Accepted FFT engineering exception and false full-performance/qualified-scope gates remain unchanged.\n\n| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU / accepted | Metal / accepted |\n|---|---:|---:|---:|---:|---:|\n");
    for shape in [
        "1x2",
        "2x2",
        "4x2",
        "8x2",
        "12x2",
        "16x2",
        "24x2",
        "merge-24x1",
    ] {
        let v = &m["m5"][shape];
        writeln!(
            report,
            "| {shape} | {:.6} | {:.0} | {:.0} | {:.3}× | {:.3}× |",
            v["accepted_ms"].as_f64().unwrap(),
            v["cpu_reference_ms"].as_f64().unwrap(),
            v["metal_reference_ms"].as_f64().unwrap(),
            v["cpu_speedup"].as_f64().unwrap(),
            v["metal_speedup"].as_f64().unwrap()
        )
        .unwrap();
    }
    report.push_str("\nHistorical accepted primary peak is 134.50 MiB; merge54 is 161.341835 ms / 1910.1875 MiB, with no supplied M5 reference. No new all-eight/54, portable/mobile/TEE, whole-proof concurrency, pressure, giant-circuit or AWS/Aeglos result is claimed. Latest AWS measurements predate the accepted FFT source. No cloud resource was created.\n\n[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [native qualification](artifacts/native/result.json) and raw logs are archived.\n\n");
    if gate {
        report.push_str("Next: all-eight/54 and portable/resource/concurrency qualification with the exact formula before adoption. This primary-only screen does not authorize a universal performance claim.\n");
    } else {
        report.push_str("The exact candidate is closed at the SDK gate. Retain the component gains, full-proof positive estimates and every failed check. Do not rerun unchanged arithmetic, flags or layout to search for a pass.\n");
    }
    save(
        root.join("closeout.json"),
        &json!({"gate_passed":gate,"adopted":false,"production172_unchanged":true,"accepted_SDK_unchanged":true,"no_diagnostic_adapter":true,"runtime_exact_parent_component":true,"core_tests_per_configuration":119,"new_Gnark_exports":99,"proof_ledger":13082,"profile_samples":samples,"goal_active_original8_achieved":0}),
    );
    fs::create_dir_all(Path::new(D).join("artifacts")).unwrap();
    copy(root, root, &Path::new(D).join("artifacts"));
    let dest = Path::new(D).join("artifacts/private-source/crates/zopro-core/src/msm.rs");
    fs::create_dir_all(dest.parent().unwrap()).unwrap();
    fs::copy(root.join("source/crates/zopro-core/src/msm.rs"), dest).unwrap();
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    let mut files = BTreeMap::new();
    inventory(Path::new(D), Path::new(D), &mut files);
    fs::write(
        Path::new(D).join("SHA256SUMS"),
        files
            .iter()
            .map(|(p, h)| format!("{h}  {p}\n"))
            .collect::<String>(),
    )
    .unwrap();
    let o = Command::new("sha256sum")
        .args(["--check", "--strict", "SHA256SUMS"])
        .current_dir(D)
        .output()
        .unwrap();
    fs::write(root.join("archive-verification.log"), o.stdout).unwrap();
    assert!(o.status.success());
    save(
        root.join("archive-summary.json"),
        &json!({"hashes":files.len(),"git_files":files.len()+1,"gate_passed":gate,"GNU_strict_passed":true}),
    );
    println!(
        "Archived{}hashes/{}Gitfiles;fullSDKgate={gate};ledger13082;notadopted",
        files.len(),
        files.len() + 1
    );
}
fn status(commit: &str) {
    let p = "/home/ubuntu/src/zopro-perf-work/latest-status.json";
    let mut s = read(p);
    let a = read(format!("{R}/analysis.json"));
    let summary = read(format!("{R}/archive-summary.json"));
    let previous = json!({"root":s["root"],"report":s["report"],"commit":s["commit"]});
    let gate = a["gate_passed"].as_bool().unwrap();
    let next = if gate {
        "All8/54,portable/resource/concurrency qualification of exact G1 weighted anchor before adoption"
    } else {
        "G1 weighted anchor closed at fullSDK gate; preserve gains and failed checks; choose distinct work reduction without unchanged reroll"
    };
    for (k, v) in [
        (
            "status",
            json!(format!(
                "G1 weighted-anchor fullSDK gate={gate}; committed/pushed/verified; not adopted"
            )),
        ),
        ("root", json!(R)),
        ("report", json!(format!("{D}/README.md"))),
        ("commit", json!(commit)),
        ("pushed", json!(true)),
        ("remote_verified", json!(true)),
        ("production_changed", json!(false)),
        ("adopted", json!(false)),
        ("last_goal_turn_classification", json!("PROGRESS")),
        ("SDK_requests", json!(15396)),
        ("new_independent_exports", json!(99)),
        ("independent_proof_count", json!(13082)),
        ("artifact_hashes", summary["hashes"].clone()),
        ("git_files_new_report", summary["git_files"].clone()),
        ("latest_full_sdk_measurement", a.clone()),
        ("previous_completed_experiment", previous),
        ("next_action", json!(next)),
        ("owned_live_jobs", json!([])),
        ("owned_cloud_resources", json!([])),
    ] {
        s[k] = v;
    }
    s.as_object_mut().unwrap().remove("active_experiment");
    save(p, &s);
    let hp = "/home/ubuntu/src/zopro-cpu-perf-handoff.md";
    let old = fs::read_to_string(hp).unwrap();
    let head=format!("CURRENT COMPLETE/PUSHED — G1 WEIGHTED Y ANCHOR FULL SDK SCREEN\n\nHEAD/remote verified {commit};root {R};report {D}/README.md. Gate={gate},NOadoption. {}strictverifiedhashes/{}Gitfiles. All172production/SDK14da unchanged,acceptedlocal1x2historical8.324853ms/RSS134.50MiB;goalACTIVEunmet0/8. CurrentgoalturnPROGRESS:positivecomponent pluspurefullSDKverifiedscreencompleted. ProofledgerNOW13082(12983prior+99new),noownedjobs/cloud.\n\nSeeanalysis.jsoncells/pairs for32/16times/CPU/RSS/boundsandfailedchecks. Purecandidate172files171originalsexact+onlymsmruntimeedits EXACTparentg1-weighted-anchor runtime;diagadapter/reexportremoved,fieldunchanged. GraphS=Ax+Bx,T=Ax+S,U=slopeSquared;cacheT existingAxslot,Y=slope*(T-U)-Ay;oneextraGENERALvectoraddper8,shorterYdependency,noaddedheap/table/unsafe/publicoption. Native119core/raw2/cubic6 OFF/ONPASSONCE;parent124includingnewgraph/reusedscratch inherited,fieldunchangedtests inherited. SeparateOFF/ONtargets,allrlibs/so frozenimmediatelyeachbuild;nativeflags/ifma8checked;8requiredSDKlibs includesmemmap2. No helper/build/hash/archiveoverlapcaptures.\n\n15blocks32/16accepted/OFF/ON151+3,3fourworkerpreflight1+3,THEN6warmFIFO199Hzprofiles251+3. All99capturesbeforeall99Gnarkverification+decode+hash/archive. Total15396SDK,99fresh164-byteexports198members,allverifiedbound/disjoint.6profilesoneSDKPIDzeroLost;pointmatcherstream_g1/BatchAdd includesonlynoninlinedmatches,notwallcost. No newall8/54/portable/resource/pressure/concurrency/giant/mobile/TEE/AWSqualification. PreserveacceptedFFTengineeringexceptionfalsefullperformancegates;latestAWSstale17137261af77. Parentcomponentdaf62e574alreadypushed,1.24%aggregatecallerpositive;donotconfusecomponentwithproofgain.\n\nNEXT:{next}. Forfuturepriorledger: g1-weighted-anchor-sdk/prior-proof-sha256.json12983 plus99proofmembersONLYfromproof-inventory.json yields13082;nevercountpublic/copies/subinventories. Allsessionsreaped;nextturninspectauthoritativefilesbeforeresuming. CommitANDpush everycompletedexperimentinclnegative,noquestions/noagents/Rustxtask/nohugetables/Casmrewrite. Olderheadershistorical.\n\n",summary["hashes"],summary["git_files"]);
    fs::write(hp, format!("{head}{old}")).unwrap();
    println!("State/handoffupdated;goalactive0/8;ledger13082");
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a[1].as_str() {
        "archive" => archive(),
        "status" => status(&a[2]),
        _ => panic!("mode"),
    }
}
