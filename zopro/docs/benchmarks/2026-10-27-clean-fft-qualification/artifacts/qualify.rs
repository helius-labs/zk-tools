use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/tmp/zopro-clean-fft-qualification";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0u8; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn add(p: impl AsRef<Path>, pins: &mut BTreeMap<String, String>) {
    let p = p.as_ref();
    let h = hash(p);
    if let Some(old) = pins.insert(p.display().to_string(), h.clone()) {
        assert_eq!(old, h)
    }
}
fn prepare() {
    let r = Path::new(R);
    for n in ["native", "portable-linked"] {
        assert_eq!(read(r.join(n).join("result.json"))["status"], "complete")
    }
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r.join("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(&hash(p), h)
    }
    let portable = read(r.join("portable-linked/portable/artifacts.json"));
    let native = read("/tmp/zopro-clean-fft-leaves/native/candidate/artifacts.json");
    let accepted = read("/tmp/zopro-composed-cpu/native/composed/artifacts.json");
    for receipt in [
        r.join("portable-linked/portable/artifacts.json"),
        r.join("native/accepted-warm/artifacts.json"),
        r.join("native/accepted-concurrent/artifacts.json"),
        r.join("native/composed-warm/artifacts.json"),
        r.join("native/composed-concurrent/artifacts.json"),
        r.join("native/composed-forester/artifacts.json"),
    ] {
        let a = read(&receipt);
        assert_eq!(
            hash(a["binary"].as_str().unwrap()),
            a["binary_sha256"].as_str().unwrap()
        );
        for (p, h) in a["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            add(p, &mut pins)
        }
        add(a["binary"].as_str().unwrap(), &mut pins);
        add(receipt, &mut pins);
    }
    let old = read("/tmp/zopro-clean-fft-leaves/perf-plan.json");
    let mut seen: BTreeSet<String> = serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(),11119);let mut repeated=0;
    for(file,want)in read("/tmp/zopro-clean-fft-leaves/proof-inventory.json").as_object().unwrap(){assert_eq!(hash(file),want.as_str().unwrap());if file.ends_with(".proof"){assert_eq!(fs::metadata(file).unwrap().len(),164);if !seen.insert(want.as_str().unwrap().into()){repeated+=1;}}}
    assert_eq!(repeated,9);assert_eq!(seen.len(),11575);
    let base = old["environment"].clone();
    let cases: Vec<Value> = old["cases"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|c| c["workers"] == 32)
        .cloned()
        .collect();
    assert_eq!(cases.len(), 9);
    let primary = cases.iter().find(|c| c["shape"] == "1x2").unwrap();
    let big = cases.iter().find(|c| c["shape"] == "merge-54x1").unwrap();
    let mut functional = Vec::new();
    for c in &cases {
        functional.push(json!({"name":format!("portable-basis-{}",c["shape"].as_str().unwrap()),"backend":"portable","kind":"basis","case":c,"workers":4}));
    }
    for c in [primary, big] {
        for kind in ["raw", "denied"] {
            functional.push(json!({"name":format!("portable-{kind}-{}",c["shape"].as_str().unwrap()),"backend":"portable","kind":kind,"case":c,"workers":4}));
        }
    }
    for w in [1, 4] {
        for stack in [524288, 1048576] {
            for c in [primary, big] {
                for kind in ["basis", "raw", "denied"] {
                    functional.push(json!({"name":format!("native-w{w}-s{stack}-{kind}-{}",c["shape"].as_str().unwrap()),"backend":"native","kind":kind,"case":c,"workers":w,"stack":stack}));
                }
            }
        }
    }
    assert_eq!(functional.len(), 37);
    let mut steps = Vec::new();
    let mut jobs = Vec::new();
    let proofroot = r.join("proofs");
    fs::create_dir(&proofroot).unwrap();
    for c in &functional {
        let name = c["name"].as_str().unwrap();
        let case = &c["case"];
        let mut env = base.clone();
        env["RAYON_NUM_THREADS"] = json!(c["workers"].as_u64().unwrap().to_string());
        if let Some(stack) = c["stack"].as_u64() {
            env["RUST_MIN_STACK"] = json!(stack.to_string())
        }
        let original = Path::new(case["image"].as_str().unwrap());
        let image = if c["kind"] == "raw" {
            Path::new("/tmp/zopro-original-constants/small/candidate/raw")
                .join(original.file_name().unwrap())
        } else {
            original.to_owned()
        };
        if c["kind"] != "basis" {
            env["ZOPRO_CPU_PREPARE_MAX_BYTES"] = json!("0")
        }
        env["ZOPRO_BASIS_KEY_DIR"] = json!(image.parent().unwrap());
        add(&image, &mut pins);
        add(case["request"].as_str().unwrap(), &mut pins);
        let dir = proofroot.join(name);
        let stem = image.file_stem().unwrap();
        let sdk = if c["backend"] == "portable" {
            &portable
        } else {
            &native
        };
        steps.push(json!({"kind":"command","name":format!("prove-{name}"),"argv":["taskset","-c",if c["workers"]==1{"0"}else{"0-3"},sdk["binary"],image,case["request"],"1",dir.join(stem)],"environment":env,"timeout_seconds":1200}));
        steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",dir,"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
        jobs.push(json!({"name":name,"kind":"functional","directory":dir,"expected_exports":1,"public_sha256":case["public_sha256"],"SDK_requests":4}));
    }
    for (variant, receipt) in [("accepted", &accepted), ("composed", &native)] {
        for c in [primary, big] {
            let name = format!("concurrent-{variant}-{}", c["shape"].as_str().unwrap());
            let dir = proofroot.join(&name);
            let driver = read(r.join(format!("native/{variant}-concurrent/artifacts.json")));
            let mut env = base.clone();
            env["RAYON_NUM_THREADS"] = json!("32");
            assert_eq!(
                read(r.join(format!("native/{variant}-concurrent/artifacts.json")))["libraries"],
                receipt["libraries"]
            );
            steps.push(json!({"kind":"command","name":format!("prove-{name}"),"argv":["/usr/bin/time","-v",driver["binary"],c["shape"],"4","2",dir],"environment":env,"timeout_seconds":1200}));
            steps.push(json!({"kind":"command","name":format!("verify-{name}"),"argv":["/tmp/zopro-gnark-verify/verify","/tmp/zopro-keys",dir,"/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"],"timeout_seconds":120}));
            jobs.push(json!({"name":name,"kind":"concurrent","directory":dir,"expected_exports":8,"public_sha256":c["public_sha256"],"SDK_requests":8}));
        }
    }
    let smallverifier = json!([
        "/tmp/zopro-gnark-verify/verify",
        "/tmp/zopro-keys",
        "{proofs}",
        "/home/ubuntu/src/zk-tools/zopro/tools/zolana/zopro-keys.lock"
    ]);
    let mut resources = Vec::new();
    for kind in ["cold", "warm"] {
        for variant in ["accepted", "composed"] {
            for policy in ["default", "retained"] {
                let name = format!("{kind}-{variant}-{policy}");
                let binary = if kind == "warm" {
                    read(r.join(format!("native/{variant}-warm/artifacts.json")))["binary"].clone()
                } else if variant == "accepted" {
                    accepted["binary"].clone()
                } else {
                    native["binary"].clone()
                };
                let count = if kind == "warm" { 2 } else { 1 };
                resources.push(json!({"name":name,"kind":kind,"variant":variant,"allocator":policy,"binary":binary,"image":primary["image"],"request":primary["request"],"verifier":smallverifier}));
                jobs.push(json!({"name":name,"kind":kind,"directory":r.join("runs").join(&name).join("proofs"),"expected_exports":count,"public_sha256":primary["public_sha256"],"SDK_requests":if kind=="warm"{5}else{4}}));
            }
        }
    }
    for (batch, digest) in [
        (
            250,
            "44340a201fb843267ec048967779f1128f8ac7ebfe80ea4af0662bee4a1bf91b",
        ),
        (
            4999,
            "92d986baaa9d270c9a31e03a0cef78a002a3920eafac668edfdd91ca5904e8e0",
        ),
    ] {
        let image = format!("/tmp/zopro-original-constants/batch-{batch}.image");
        assert_eq!(hash(&image), digest);
        add(&image, &mut pins);
        let witnesses = format!("/tmp/zopro-forester-local/fixtures/{batch}");
        for i in 0..2 {
            add(
                Path::new(&witnesses).join(format!("{i}.witness")),
                &mut pins,
            )
        }
        let vk = format!("/tmp/zopro-forester-format-retry/dump-{batch}/vk.bin");
        add(&vk, &mut pins);
        add("/tmp/zopro-forester-local/go-driver", &mut pins);
        let name = format!("forester-{batch}");
        resources.push(json!({"name":name,"kind":"large","batch":batch,"allocator":"retained","binary":read(r.join("native/composed-forester/artifacts.json"))["binary"],"image":image,"image_sha256":digest,"witnesses":witnesses,"verifier":["/tmp/zopro-forester-local/go-driver","verify",vk,"{proofs}"]}));
        jobs.push(json!({"name":name,"kind":"large","directory":r.join("runs").join(&name).join("proofs"),"expected_exports":2,"witnesses":witnesses,"SDK_requests":0,"core_requests":2}));
    }
    assert_eq!(resources.len(), 10);
    assert_eq!(
        jobs.iter()
            .map(|j| j["expected_exports"].as_u64().unwrap())
            .sum::<u64>(),
        85
    );
    assert_eq!(
        jobs.iter()
            .map(|j| j["SDK_requests"].as_u64().unwrap())
            .sum::<u64>(),
        216
    );
    save(
        r.join("inputs.json"),
        &json!({"environment":base,"prior_proof_sha256":seen,"prior_count":11575,"planned_final_count":11660,"functional_cases":functional,"native_sdk":native,"accepted_sdk":accepted,"portable_sdk":portable}),
    );
    save(r.join("jobs.json"), &json!(jobs));
    save(r.join("resource-cases.json"), &json!(resources));
    for n in [
        "qualify.rs",
        "close.rs",
        "close",
        "prepare.rs",
        "prepare-close.rs",
        "preparation-failure.json",
        "engineering-review-plan.md",
        "qualify",
        "resources.rs",
        "resources",
        "resource-tests.log",
        "inputs.json",
        "jobs.json",
        "resource-cases.json",
        "native/result.json",
        "portable/failure.json",
        "portable-linked/result.json",
        "portable-link-repair.json",
        "portable-link-plan.json",
        "repair-portable-link.rs",
        "repair-portable-link-v1.rs",
        "portable/field/tests.json",
        "portable/core/tests.json",
        "portable/prover/tests.json",
    ] {
        add(r.join(n), &mut pins)
    }
    add("/usr/bin/time", &mut pins);
    save(r.join("qualification-pins.json"), &json!(pins));
    steps.push(json!({"kind":"command","name":"resources","argv":[r.join("resources"),"run"],"timeout_seconds":3600}));
    save(
        r.join("qualification-plan.json"),
        &json!({"version":1,"source_pins":r.join("qualification-pins.json"),"steps":steps}),
    );
    println!("Prepared37functional/4concurrent/10resourcecases;216SDK+4corecalls,85freshGnarkexports planned;prior11575");
}
fn proof_files(dir: &Path, v: &mut Vec<PathBuf>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        if e.file_type().unwrap().is_dir() {
            proof_files(&p, v)
        } else if p.extension().is_some_and(|x| x == "proof") {
            v.push(p)
        }
    }
}
fn finish() {
    let r = Path::new(R);
    assert_eq!(
        read(r.join("qualification/result.json"))["status"],
        "complete"
    );
    assert_eq!(read(r.join("resource-result.json"))["passed"], true);
    let mut seen: BTreeSet<String> =
        serde_json::from_value(read(r.join("inputs.json"))["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(), 11575);
    let mut inventory = BTreeMap::new();
    let mut rows = Vec::new();
    for job in read(r.join("jobs.json")).as_array().unwrap() {
        let dir = Path::new(job["directory"].as_str().unwrap());
        let mut files = Vec::new();
        proof_files(dir, &mut files);
        files.sort();
        assert_eq!(
            files.len() as u64,
            job["expected_exports"].as_u64().unwrap()
        );
        for p in &files {
            assert_eq!(fs::metadata(p).unwrap().len(), 164);
            let public = fs::read(p.with_extension("public")).unwrap();
            if job["kind"] == "large" {
                let n: usize = p.file_stem().unwrap().to_str().unwrap().parse().unwrap();
                assert!(n < 2);
                let witness = fs::read(
                    Path::new(job["witnesses"].as_str().unwrap()).join(format!("{n}.witness")),
                )
                .unwrap();
                let count = u32::from_be_bytes(witness[..4].try_into().unwrap());
                let mut expected = Vec::new();
                for w in [count, 0, count] {
                    expected.extend(w.to_be_bytes())
                }
                expected.extend(&witness[12..12 + 32 * count as usize]);
                assert_eq!(public, expected);
            } else {
                assert_eq!(
                    hash(p.with_extension("public")),
                    job["public_sha256"].as_str().unwrap()
                );
            }
            let h = hash(p);
            assert!(seen.insert(h));
            for member in [p.clone(), p.with_extension("public")] {
                inventory.insert(member.display().to_string(), hash(member));
            }
        }
        if job["kind"] == "large" || job["kind"] == "cold" || job["kind"] == "warm" {
            let out = r.join("runs").join(job["name"].as_str().unwrap());
            assert_eq!(read(out.join("result.json"))["passed"], true);
            assert_eq!(read(out.join("verifier-exit.json"))["success"], true);
            let log = fs::read_to_string(out.join("verifier.log")).unwrap();
            let n = if job["kind"] == "large" {
                log.lines()
                    .filter(|s| {
                        s.strip_prefix("EVENT ").is_some_and(|x| {
                            serde_json::from_str::<Value>(x).unwrap()["stage"] == "verified"
                        })
                    })
                    .count()
            } else {
                log.lines()
                    .filter(|s| s.starts_with("verified ") && s.contains(".proof "))
                    .count()
            };
            assert_eq!(n, files.len());
        } else {
            let log = fs::read_to_string(
                r.join("qualification")
                    .join(format!("verify-{}", job["name"].as_str().unwrap()))
                    .join("stdout.log"),
            )
            .unwrap();
            assert_eq!(
                log.lines()
                    .filter(|s| s.starts_with("verified ") && s.contains(".proof "))
                    .count(),
                files.len()
            );
        }
        rows.push(json!({"job":job,"passed":true,"proofs":files}));
    }
    assert_eq!(seen.len(), 11660);
    assert_eq!(inventory.len(), 170);
    for (p, h) in read(r.join("qualification-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap())
    }
    save(r.join("proof-inventory.json"), &json!(inventory));
    save(
        r.join("analysis.json"),
        &json!({"qualification_passed":true,"SDK_requests":216,"core_requests":4,"independent_exports":85,"proof_public_members":170,"prior_count":11575,"next_prior_count":11660,"rows":rows,"resources":read(r.join("resource-result.json")),"portable_field_tests":read(r.join("portable/field/tests.json"))["passed_tests"].as_array().unwrap().len(),"portable_core_tests":read(r.join("portable/core/tests.json"))["passed_tests"].as_array().unwrap().len(),"portable_prover_tests":read(r.join("portable/prover/tests.json"))["passed_tests"].as_array().unwrap().len(),"native_arithmetic_rebuilt":false,"accepted_source_changed":false,"adopted":false,"earlier_performance_full_gate_passed":false,"goal_achieved":false,"original8_CPU10x_rows":0}),
    );
    println!("All resource/compatibility qualification passed:216SDK+4forester,85Gnarkexports,proofledger11660;no changed latency verdict/adoption");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("finish") => finish(),
        _ => panic!("qualify prepare|finish"),
    }
}
