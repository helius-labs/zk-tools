fn prepare() {
    assert!(!r("run-plan.json").exists());
    assert_eq!(read(r("native/result.json"))["status"], "complete");
    let mut pins: BTreeMap<String, String> =
        serde_json::from_value(read(r("build-pins.json"))).unwrap();
    for (p, h) in &pins {
        assert_eq!(hash(p), *h, "{p}");
    }
    let audit = read(r("source-audit.json"));
    assert_eq!(audit["source_files"], 176);
    assert_eq!(audit["originals_exact"], 174);
    assert_eq!(audit["reverse175_exact"], true);
    assert_eq!(audit["accepted_balanced13_byte_exact"], true);
    assert_eq!(hash(ACCEPTED), ACCEPTED_HASH);
    pin(&mut pins, ACCEPTED);
    let required=["msm::g2_borrowed::norm_heap::tests::arbitrary_heap_lengths_match_scalar_inverses_and_reuse","msm::g2_borrowed::norm_heap::tests::signed_complete_batches_tails_disabled_and_scratch_reuse"];
    for (label, count) in [("control", 124), ("candidate", 126)] {
        let t = read(r(format!("native/{label}-core/tests.json")));
        assert_eq!(t["passed_tests"].as_array().unwrap().len(), count);
        assert_eq!(t["unfiltered"], true);
        if label == "candidate" {
            for name in required {
                assert!(t["passed_tests"].as_array().unwrap().contains(&json!(name)));
            }
        }
        assert_eq!(
            read(r(format!("native/{label}-prover/tests.json")))["passed_tests"]
                .as_array()
                .unwrap()
                .len(),
            8
        );
        let receipt = read(r(format!("native/{label}/artifacts.json")));
        assert_eq!(
            hash(receipt["binary"].as_str().unwrap()),
            receipt["binary_sha256"].as_str().unwrap()
        );
        assert_eq!(receipt["libraries"].as_object().unwrap().len(), 8);
        for (p, h) in receipt["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
        let frozen = read(r(format!("frozen-{label}.json")));
        let flags = fs::read_to_string(r(if label == "candidate" {
            "candidate-flags.txt"
        } else {
            "native-flags.txt"
        }))
        .unwrap();
        assert_eq!(frozen["rustflags"], flags);
        for (p, h) in frozen["input_pins"].as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pin(&mut pins, p);
        }
    }
    assert_eq!(audit["field_source_files_exact"], 17);
    let code = read(r("code-audit.json"));
    assert_eq!(code["caller_route_verified"], true);
    pin(&mut pins, r("code-audit.json"));
    pin(&mut pins, r("code-audit.rs"));
    for e in fs::read_dir(r("codegen")).unwrap() {
        let p = e.unwrap().path();
        assert!(p.is_file());
        pin(&mut pins, p);
    }
    for label in ["control", "candidate"] {
        for suffix in ["core", "prover"] {
            pin(&mut pins, r(format!("native/{label}-{suffix}/tests.json")));
        }
    }
    let mut ids = BTreeMap::new();
    for v in VS {
        let binary = if v == "accepted" {
            PathBuf::from(ACCEPTED)
        } else {
            r(if v == "OFF" {
                "native/control-sdk"
            } else {
                "native/candidate-sdk"
            })
        };
        pin(&mut pins, &binary);
        ids.insert(
            v,
            json!({"binary":binary,"sha256":hash(&binary),"g2_norm_heap_cfg":v=="ON"}),
        );
    }
    save(r("identities.json"), &json!(ids));
    let mut prior: BTreeSet<String> =
        serde_json::from_value(read(format!("{OLD}/prior-proof-sha256.json"))).unwrap();
    assert_eq!(prior.len(), 17083);
    for (p, h) in read(format!("{OLD}/proof-inventory.json"))
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
        if p.ends_with(".proof") {
            assert!(prior.insert(h.as_str().unwrap().into()));
        }
    }
    assert_eq!(prior.len(), 17161);
    save(r("prior-proof-sha256.json"), &json!(prior));
    let all = read("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/cases.json");
    assert_eq!(all.as_array().unwrap().len(), 9);
    save(r("preflight-cases.json"), &all);
    let cases = json!([all[0]]);
    save(r("cases.json"), &cases);
    let base = read(MANIFEST)["environment"].clone();
    assert_eq!(base["ZOPRO_CPU_DEFER_SIGN_WIDTH"], "12");
    assert_eq!(base["ZOPRO_CPU_DEFER_SIGN_PARTS"], "1");
    let mut jobs = vec![];
    let mut steps = vec![];
    let verifier = |j: &Value| {
        let c = &j["case"];
        let dir = Path::new(j["directory"].as_str().unwrap());
        json!({"kind":"command","name":format!("verify-{}",j["name"].as_str().unwrap()),"argv":[VERIFY,c["image"],c["image_sha256"],dir.join("0.proof"),dir.join("0.public"),c["public_sha256"]],"timeout_seconds":120})
    };
    for c in all.as_array().unwrap() {
        for v in VS {
            let j = push(c, v, 0, true, &base, &ids, &mut jobs, &mut steps);
            steps.push(verifier(&j));
        }
    }
    assert_eq!(jobs.len(), 27);
    assert_eq!(steps.len(), 54);
    for b in 0..15 {
        let mut order = VS;
        order.rotate_left(b % 3);
        if b % 2 != 0 {
            order.reverse();
        }
        for v in order {
            push(&cases[0], v, b, false, &base, &ids, &mut jobs, &mut steps);
        }
    }
    assert_eq!(jobs.len(), 72);
    for j in jobs.iter().filter(|j| j["preflight"] != true) {
        steps.push(verifier(j));
    }
    assert_eq!(steps.len(), 144);
    for name in ["ctl", "ack"] {
        assert!(Command::new("mkfifo")
            .arg(r(name))
            .status()
            .unwrap()
            .success());
    }
    for panel in ["arith", "cycles"] {
        for v in VS {
            diagnostic(&cases[0], v, panel, &base, &ids, &mut jobs, &mut steps);
        }
    }
    assert_eq!(jobs.len(), 78);
    for j in jobs.iter().filter(|j| j["diagnostic"] == true) {
        steps.push(verifier(j));
    }
    for j in jobs.iter().filter(|j| j["panel"] == "cycles") {
        let name = j["name"].as_str().unwrap();
        steps.push(json!({"kind":"command","name":format!("decode-{name}"),"argv":["sudo","-n","perf","script","--header","--show-lost-events","-i",j["data"],"-F","pid,tid,period,ip,sym"],"timeout_seconds":180}));
        steps.push(json!({"kind":"command","name":format!("evlist-{name}"),"argv":["sudo","-n","perf","evlist","-v","-i",j["data"]],"timeout_seconds":120}));
        steps.push(json!({"kind":"command","name":format!("loss-{name}"),"argv":[r("capture-helper"),"loss",j["data"]],"timeout_seconds":180}));
    }
    assert_eq!(steps.len(), 165);
    assert_eq!(
        jobs.iter()
            .map(|j| j["rounds"].as_u64().unwrap() + 3)
            .sum::<u64>(),
        7902
    );
    save(r("jobs.json"), &json!(jobs));
    for c in all.as_array().unwrap() {
        assert_eq!(
            hash(c["image"].as_str().unwrap()),
            c["image_sha256"].as_str().unwrap()
        );
        pin(&mut pins, c["image"].as_str().unwrap());
        pin(&mut pins, c["request"].as_str().unwrap());
    }
    for e in
        fs::read_dir("/home/ubuntu/src/zopro-perf-work/sdk-control-copies/lagrange-cache").unwrap()
    {
        pin(&mut pins, e.unwrap().path());
    }
    for p in [VERIFY, X, MANIFEST] {
        pin(&mut pins, p);
    }
    for name in [
        "design.md",
        "history-audit.md",
        "native.rs",
        "native-helper",
        "setup.rs",
        "norm_heap.rs",
        "capture-setup.rs",
        "capture.rs",
        "capture-helper",
        "report.rs",
        "report-helper",
        "close.rs",
        "close-helper",
        "pmu-feasibility.log",
        "source-audit.json",
        "identities.json",
        "prior-proof-sha256.json",
        "jobs.json",
        "cases.json",
        "preflight-cases.json",
        "native/result.json",
        "native/control/artifacts.json",
        "native/candidate/artifacts.json",
        "frozen-control.json",
        "frozen-candidate.json",
    ] {
        pin(&mut pins, r(name));
    }
    save(r("pins.json"), &json!(pins));
    save(
        r("run-plan.json"),
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    fs::write(
        r("host-before.txt"),
        output(&[
            "ps",
            "-eo",
            "pid,ppid,comm,pcpu,pmem,etimes",
            "--sort=-pcpu",
        ]),
    )
    .unwrap();
    println!("Prepared165steps/7902SDK/78freshGnark/ledger17161->17239;all9preflights before primary15blockpilot;noadoption");
}
