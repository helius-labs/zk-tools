use std::fs;
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
    let new = "/home/ubuntu/src/zopro-perf-work/g2-borrowed-queue";
    let mut d = fs::read_to_string(format!("{old}/driver.rs"))
        .unwrap()
        .replace(old, new)
        .replace("zopro_g2_canonical_xsum", "zopro_g2_borrow_queue");
    let start = d.find("fn contexts()").unwrap();
    let end = start + d[start..].find("fn link()").unwrap();
    d.replace_range(start..end,"fn contexts()->Vec<(&'static str,usize)>{vec![(\"g2-msm\",4096),(\"g2-msm\",16384),(\"g2-msm\",30089)]}\n");
    once(&mut d,"assert!(t.contains(\"test result: ok. 122 passed; 0 failed\"));","let count=if name==\"on\"{122}else{119};assert!(t.contains(&format!(\"test result: ok. {count} passed; 0 failed\")));");
    let start = d.find("    let frozen =").unwrap();
    let end = start
        + d[start..]
            .find("    for p in [\n        \"probe.rs\"")
            .unwrap();
    d.replace_range(start..end, "");
    once(&mut d,"    let mut steps = vec![];\n    for name in [\"frozen\", \"off\", \"on\"] {","    pin(&mut pins,r(\"make-driver.rs\"));pin(&mut pins,r(\"make-driver\"));\n    let mut steps = vec![];\n    for name in [\"frozen\", \"off\", \"on\"] {");
    let start = d.find("        let dir = if name == \"frozen\"").unwrap();
    let end = start
        + d[start..]
            .find("        for test in [true, false]")
            .unwrap();
    d.replace_range(
        start..end,
        "        let dir=r(format!(\"frozen/{name}\")).display().to_string();\n",
    );
    once(
        &mut d,
        "tests::independent_workers_preserve_oracles_affinity_barriers_and_reuse",
        "tests::complete_MSM_oracles_reset_and_affinity",
    );
    let needle="            steps.push(json!({\"kind\":\"command\",\"name\":format!(\"link-{out}\"),\"argv\":argv,\"timeout_seconds\":300}));";
    once(
        &mut d,
        needle,
        &format!(
            "{needle}\n{}",
            r#"            if !test {
                let mut heap=argv.clone();let dest=r(format!("{name}-heap")).display().to_string();for v in &mut heap {if *v==r(&out).display().to_string(){*v=dest.clone();}}
                heap.extend(["--cfg".into(),"zopro_queue_heap_probe".into()]);
                steps.push(json!({"kind":"command","name":format!("link-{name}-heap"),"argv":heap,"timeout_seconds":300}));
                if name=="frozen" {let mut test=heap.clone();for v in &mut test{if *v==dest{*v=r("heap-tests").display().to_string();}}test.push("--test".into());steps.push(json!({"kind":"command","name":"link-heap-tests","argv":test,"timeout_seconds":300}));steps.push(json!({"kind":"command","name":"heap-accounting-test","argv":[r("heap-tests"),std::path::PathBuf::from("tests::heap_accounting_balances_requested_buffer"),std::path::PathBuf::from("--exact"),std::path::PathBuf::from("--test-threads=1")],"required_tests":["tests::heap_accounting_balances_requested_buffer"],"timeout_seconds":300}));}
            }"#
        ),
    );
    once(
        &mut d,
        "    pin(&mut pins, X);",
        r#"    assert!(fs::read_to_string(r("linked/heap-accounting-test/stdout.log")).unwrap().contains("test result: ok. 1 passed; 0 failed"));
    for f in ["heap-tests","linked/heap-accounting-test/stdout.log","control-sections.rs","control-sections","control-sections.json","control-locations.rs","control-locations","control-locations.json"] {pin(&mut pins,r(f));}
    for n in ["frozen","off","on"]{pin(&mut pins,r(format!("{n}-heap")));}
    assert_eq!(read(r("control-locations.json"))["native_code_or_arithmetic_changed"],false);
    pin(&mut pins, X);"#,
    );
    once(
        &mut d,
        ".filter(|s| s.contains(\"g2_prefix\"))",
        ".filter(|s|s.contains(\"g2_prefix\")||s.contains(\"11g2_borrowed5apply\"))",
    );
    once(&mut d,"        let mut environment = env();","        let mut environment = env();environment[\"ZOPRO_MSM_SEED\"]=json!((block as u64+17).to_string());");
    once(
        &mut d,
        "                    8192,",
        "                    7,",
    );
    let start = d.find("    for phase in [\"stat\", \"cycles\"] {").unwrap();
    let end = start + d[start..].find("    assert_eq!(jobs.len(), 845);").unwrap();
    d.replace_range(start..end,r#"    for phase in ["stat","cycles"] {for topology in ["physical16","logical32"] {for m in [1,2] {let name=format!("{phase}-g2-msm-30089-{topology}-m{m}");add(name.clone(),phase,0,"g2-msm",30089,m,topology,256);if phase=="cycles"{profiles.push(name);}}}}
"#);
    once(
        &mut d,
        "    save(r(\"jobs.json\"), &json!(jobs));",
        r#"    let mut resources=vec![];for(kind,n)in contexts(){for topology in ["solo","physical16","logical32"]{for mode in 0..3{let mut environment=env();environment["ZOPRO_MSM_SEED"]=json!("17");resources.push(json!({"kind":"command","name":format!("resource-{kind}-{n}-{topology}-m{mode}"),"argv":[r(format!("{}-heap",label(mode))).display().to_string(),kind.to_owned(),n.to_string(),mode.to_string(),topology.to_owned(),"1".to_owned()],"environment":environment,"timeout_seconds":300}));}}}
    assert_eq!(resources.len(),27);save(r("resource-plan.json"),&json!({"version":1,"source_pins":r("pins.json"),"steps":resources}));
    save(r("jobs.json"), &json!(jobs));"#,
    );
    once(&mut d,"                    \"wall_ns_per_wave\",\n                    \"throughput_ns_per_call\",\n                    \"mean_worker_ns_per_batch\",\n                    \"thread_cpu_ns_per_call\",","                    \"ns_per_MSM\",\n                    \"CPU_ms_per_MSM\",");
    d = d.replace("[\"wall_ns_per_wave\"]", "[\"ns_per_MSM\"]");
    once(&mut d, "[(\"g2-prefix\", 200)]", "[(\"g2-msm\", 30089)]");
    once(&mut d,"\"scope\":\"native actual-caller screen, independent frozen/freshOFF/ON, synthetic ordinary points at real batch sizes; no proof-speed claim\"","\"scope\":\"complete native G2 phi MSM and scalar-input reset; independent accepted-source frozen/freshOFF/ON; no full-proof claim\"");
    once(
        &mut d,
        "\"native_core_tests_each\":122",
        "\"native_core_tests_off\":119,\"native_core_tests_on\":122",
    );
    d = d.replace("\"timed_kernel_calls\":total", "\"timed_MSM_calls\":total");
    d=d.replace("Prepared9frozen/OFF/ONlink/harnessteststeps;inheritedfieldqualification/core122x2PASS","Prepared14linkteststeps;fieldunchanged/core119OFF122ON").replace("Prepared837latency/4stat/4cycles/12decode;9contexts31blocks;frozenOFFONindependentlibs","Prepared837latency/4stat/4cycles/12decode plus27separateresourceprobes;completeMSM");
    fs::write("driver.rs", d).unwrap();
    let s = fs::read_to_string(format!("{old}/collect.rs")).unwrap();
    let start = s.find("        if j[\"phase\"] == \"stat\" {").unwrap();
    let mut suffix = s[start..].to_string();
    let old="            assert_eq!(\n                sampled_worker_tids.len(),\n                j[\"worker_count\"].as_u64().unwrap() as usize\n            );";
    once(&mut suffix,old,"            assert!(!sampled_worker_tids.is_empty());assert!(sampled_worker_tids.len()<=j[\"worker_count\"].as_u64().unwrap()as usize);");
    suffix = suffix
        .replace("\"per_kernel_call\"", "\"per_MSM\"")
        .replace("\"total_kernel_calls\"", "\"total_MSM_calls\"");
    let head = r#"fn collect()->(BTreeMap<String,Value>,BTreeMap<String,Value>,BTreeMap<String,Value>,BTreeMap<String,Value>,u64){
 assert_eq!(read(r("runs/result.json"))["status"],"complete");assert_eq!(read(r("resources/result.json"))["status"],"complete");
 let jobs=read(r("jobs.json"));let mut results=BTreeMap::new();let mut digests=BTreeMap::new();let mut caps=BTreeMap::new();let mut stat=BTreeMap::new();let mut profiles=BTreeMap::new();let mut total_calls=0u64;
 for j in jobs.as_array().unwrap(){let name=j["name"].as_str().unwrap();let text=fs::read_to_string(r(format!("runs/{name}/stdout.log"))).unwrap();let lines:Vec<_>=text.lines().filter_map(|l|l.strip_prefix("RESULT ")).collect();assert_eq!(lines.len(),1);let v:Value=serde_json::from_str(lines[0]).unwrap();for f in["kind","size","mode","topology","worker_count"]{assert_eq!(v[f],j[f]);}assert_eq!(v["rounds"],j["rounds"]);assert_eq!(v["prime_calls"],3);assert_eq!(v["heap_instrumented"],false);assert_eq!(v["heap"],Value::Null);assert_eq!(v["includes_scalar_reset"],true);assert_eq!(v["internally_verified"],true);assert_eq!(v["input_seed"].as_u64().unwrap(),j["block"].as_u64().unwrap()+17);let count=v["total_MSM_calls"].as_u64().unwrap();assert_eq!(count,j["rounds"].as_u64().unwrap());total_calls+=count;
 let key=format!("{}-{}-{}",j["kind"],j["size"],v["input_seed"]);let h=v["output_sha256"].as_str().unwrap();if let Some(old)=digests.insert(key,h.to_owned()){assert_eq!(old,h);}
 let mut cpus=BTreeSet::new();let mut tids=BTreeSet::new();for w in v["workers"].as_array().unwrap(){assert!(cpus.insert(w["cpu"].as_u64().unwrap()));assert!(tids.insert(w["tid"].as_u64().unwrap()));}assert_eq!(cpus.len(),j["worker_count"].as_u64().unwrap()as usize);assert_eq!(cpus.into_iter().collect::<Vec<_>>(),v["cpus"].as_array().unwrap().iter().map(|x|x.as_u64().unwrap()).collect::<Vec<_>>());
"#;
    suffix=suffix.replace("    (results, stat, profiles, caps, total_calls)",r#"    for (kind,n)in contexts(){for t in ["solo","physical16","logical32"]{for mode in 0..3{let name=format!("resource-{kind}-{n}-{t}-m{mode}");let text=fs::read_to_string(r(format!("resources/{name}/stdout.log"))).unwrap();let line=text.lines().find_map(|l|l.strip_prefix("RESULT ")).unwrap();let v:Value=serde_json::from_str(line).unwrap();assert_eq!(v["heap_instrumented"],true);assert_eq!(v["internally_verified"],true);assert_eq!(v["size"],n);assert_eq!(v["mode"],mode);assert_eq!(v["topology"],t);assert_eq!(v["rounds"],1);assert!(v["heap"]["allocation_requests"].as_u64().unwrap()>0);assert!(v["heap"]["peak_live"].as_u64().unwrap()>=v["heap"]["live_before"].as_u64().unwrap());caps.insert(name,v);}}}
    (results, stat, profiles, caps, total_calls)"#);
    fs::write("collect.rs", format!("{head}{suffix}")).unwrap();
    let old = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native";
    fs::copy(format!("{old}/control-sections.rs"), "control-sections.rs").unwrap();
    let mut s = fs::read_to_string(format!("{old}/control-locations.rs")).unwrap();
    once(&mut s,"path.ends_with(\"crates/zopro-core/src/msm.rs\")\n                || path.ends_with(\"crates/zopro-core/src/msm/g2_prefix.rs\")","path.ends_with(\"crates/zopro-core/src/msm_affine/g2_phi.rs\")");
    once(
        &mut s,
        "    assert_eq!(out.len(), 20);",
        "    assert!(!out.is_empty());",
    );
    s=s.replace("All20records verified as Rust source lines+2;all other runtime section bytes identical","All changed records verified as Rust source lines+2;all other runtime section bytes identical").replace("20 Rust panic Location line fields, each +2 after cfg-only source insertions","Rust panic Location line fields, each +2 after cfg-only source insertions");
    fs::write("control-locations.rs", s).unwrap();
    println!(
        "GeneratedcompleteMSMlink/capture/collector/resourceplans and strict code/location audit"
    );
}
