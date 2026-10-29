use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/radix-groups175";
fn replace(s: &mut String, a: &str, b: &str) {
    assert!(s.contains(a), "missing {a}");
    *s = s.replace(a, b);
}
fn main() {
    let root = Path::new(R);
    assert!(!root.join("capture.rs").exists());
    let old = "/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175";
    let source = fs::read_to_string(format!("{old}/capture.rs")).unwrap();
    let mut prep = fs::read_to_string(format!("{old}/capture-setup.rs")).unwrap();
    replace(
        &mut prep,
        "core_solver_key_scheduler_forecast_exact",
        "sample_cutoffs_reservation_canonicalization_unchanged",
    );
    replace(
        &mut prep,
        "[(\"control\", 124), (\"candidate\", 124)]",
        "[(\"control\", 128), (\"candidate\", 128)]",
    );
    let a = prep.find("        let field = read(").unwrap();
    let b = prep[a..].find("        for suffix in [").unwrap() + a;
    prep.replace_range(a..b, "");
    replace(
        &mut prep,
        "[\"field\", \"core\", \"prover\"]",
        "[\"core\", \"prover\"]",
    );
    replace(
        &mut prep,
        "actual_Fq_Fr_inverse_functions_changed",
        "actual_grouping_entry_changed",
    );
    replace(&mut prep, "direct_finish_cfg", "radix_group_cfg");
    replace(&mut prep, "18076", "18154");
    replace(&mut prep, "17830", "18076");
    replace(
        &mut prep,
        "let cases = json!([all[0]]);",
        "let cases = all.clone();",
    );
    let a = prep.find("    for b in 0..15 {").unwrap();
    let b = prep[a..].find("    assert_eq!(jobs.len(), 72);").unwrap() + a;
    prep.replace_range(
        a..b,
        r#"    for b in 0..15 {
        let mut contexts:Vec<_>=(0..9).collect();contexts.rotate_left(b%9);
        if b%2!=0 {contexts.reverse();}
        for (i,c) in contexts.into_iter().enumerate(){
            let mut order=VS;order.rotate_left((b+i)%3);if (b+i)%2!=0{order.reverse();}
            for v in order{push(&cases[c],v,b,false,&base,&ids,&mut jobs,&mut steps);}
        }
    }
"#,
    );
    replace(&mut prep, "jobs.len(), 72", "jobs.len(), 432");
    replace(&mut prep, "steps.len(), 144", "steps.len(), 864");
    replace(&mut prep, "jobs.len(), 78", "jobs.len(), 438");
    replace(&mut prep, "steps.len(), 165", "steps.len(), 885");
    replace(&mut prep, "7902", "20142");
    replace(&mut prep, "\"bounds.md\",", "\"memory-review.md\",");
    replace(&mut prep, "\"direct_finish.rs\",", "\"radix.rs\",");
    replace(&mut prep,"Prepared165steps/20142SDK/78freshGnark/ledger18154->18154;all9preflights before primary15blockpilot;noadoption","Prepared885steps/20142SDK/438freshGnark/ledger18154->18592;all9shapes15blocks;noadoption");
    fs::write(root.join("capture-setup.rs"), prep).unwrap();
    let a = source.find("fn finish() {").unwrap();
    let b = source.find("    let mut pairs =").unwrap();
    let c = source
        .find("    let diagnostics = analyze_diagnostics();")
        .unwrap();
    let head = &source[..a];
    let mut finish = source[a..b].to_owned();
    let pairbody = &source[b..c];
    replace(&mut finish, "18154", "18592");
    replace(&mut finish, "18076", "18154");
    replace(&mut finish, "7902", "20142");
    replace(&mut finish, "inventory.len(), 156", "inventory.len(), 876");
    let extra = r##"
    assert_eq!(cells.len(),27);assert_eq!(refs.len(),8);
    let cases=read(r("cases.json"));let mut aggregate=BTreeMap::new();
    let reference_cases:Vec<_>=cases.as_array().unwrap().iter().filter(|c|!c["reference"].is_null()).collect();assert_eq!(reference_cases.len(),8);
    for control in ["accepted","OFF"]{
        let ratios:Vec<_>=(0..15).map(|block|{
            let sum:f64=reference_cases.iter().map(|c|{
                let name=c["name"].as_str().unwrap();
                let on=cells[&format!("{name}/ON")][block]["median_ms"].as_f64().unwrap();
                let off=cells[&format!("{name}/{control}")][block]["median_ms"].as_f64().unwrap();(on/off).ln()
            }).sum();(sum/8.).exp()
        }).collect();
        let mut sorted=ratios.clone();sorted.sort_by(f64::total_cmp);let wins=ratios.iter().filter(|r|**r<1.).count();
        let pass=sorted[11]<1.&&wins>=10;let key=format!("original8/ON/{control}");checks.insert(key.clone(),pass);if !pass{failed.push(key.clone());}
        aggregate.insert(key,json!({"median":sorted[7],"low":sorted[3],"high":sorted[11],"wins":wins,"ratios":ratios,"nominal_coverage":0.96484375}));
    }
    let mut nonregression=BTreeMap::new();
    for c in cases.as_array().unwrap(){let name=c["name"].as_str().unwrap();for control in ["accepted","OFF"]{let key=format!("{name}/ON/{control}");nonregression.insert(key.clone(),pairs[&key]["high"].as_f64().unwrap()<=1.03);}}
    let diagnostics=analyze_diagnostics();
    for label in ["control","candidate"]{
        assert_eq!(read(r(format!("native/{label}-core/tests.json")))["passed_tests"].as_array().unwrap().len(),128);
        assert_eq!(read(r(format!("native/{label}-prover/tests.json")))["passed_tests"].as_array().unwrap().len(),8);
    }
    let candidate_rows=refs.values().filter(|r|r["variants"]["ON"]["CPU10x"]==true).count();
    let a=json!({"status":"completed exact radix-index grouping full SDK matrix;not adopted","gate_passed":failed.is_empty(),"failed_checks":failed,"checks":checks,"calibration_checks":calibration,"nonregression_checks":nonregression,"pairs":pairs,"cells":summaries,"original8_aggregate":aggregate,"diagnostics":diagnostics,"new_reference_comparisons":refs,"inherited_accepted_M5":read(MANIFEST)["m5"],"SDK_requests":requests,"new_independent_exports":438,"independent_proof_count":18592,"field_tests_inherited":33,"all17field_source_files_exact":true,"new_core_tests":4,"native_core_tests_each":128,"native_prover_tests_each":8,"production_changed":false,"adopted":false,"goal_status":"active","goal_achieved":false,"accepted_original8_CPU10x_rows":0,"candidate_fresh_reference_rows_measured":8,"candidate_fresh_CPU10x_rows":candidate_rows,"candidate_all8_native_latency_gate_passed":failed.is_empty(),"candidate_compatibility_resource_qualified":false,"accepted_full_performance_gate_passed":false,"accepted_failed_checks_retained":["1x2-w16/OFF/accepted"],"scope":"exact fingerprint radix grouping builder; all9shapes32workers15blocks; full8CPUandMetal comparisons; no further compatibility/resource/adoption qualification"});
    save(r("analysis.json"),&a);fs::write(r("host-after.txt"),output(&["ps","-eo","pid,ppid,comm,pcpu,pmem,etimes","--sort=-pcpu"])).unwrap();
    println!("Fullmatrix gate={} failures={};20142SDK/438Gnark/ledger18592",a["gate_passed"],a["failed_checks"]);
    for(k,v)in summaries{println!("{k} {:.6}ms CPU{:.3} RSS{:.2}/{:.2}",v["median_ms"].as_f64().unwrap(),v["CPU_ms"].as_f64().unwrap(),v["median_RSS_MiB"].as_f64().unwrap(),v["max_RSS_MiB"].as_f64().unwrap());}
    for(k,v)in aggregate{println!("{k} {}[{},{}]wins{}",v["median"],v["low"],v["high"],v["wins"]);}
}
"##;
    let main = &source[source.find("fn main() {").unwrap()..];
    let mut all = format!("{head}{finish}{pairbody}{extra}{main}");
    replace(&mut all, "inverse-direct-sdk175", "radix-groups175");
    replace(
        &mut all,
        "/home/ubuntu/src/zopro-perf-work/serial-phi175",
        "/home/ubuntu/src/zopro-perf-work/inverse-direct-sdk175",
    );
    fs::write(root.join("capture.rs"), all).unwrap();
    println!("Generated fixed all9shape matrix;405latencies27preflights6diagnostics/885workflowsteps;20142SDK438Gnark planned");
}
