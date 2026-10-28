use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/direct-ref-pack";
fn main(){
    let original=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/inverse-finish-callers/experiment.rs").unwrap();
    let mut s=original.replace("/zopro-perf-work/inverse-finish-callers","/zopro-perf-work/direct-ref-pack").replace("2026-10-28-inverse-finish-callers","2026-10-28-direct-ref-pack").replace("zopro_direct_finish_reduction","zopro_direct_ref_pack");
    let start=s.find("    let m=read(").unwrap();let end=s[start..].find("    for label in [\"frozen\", \"fresh\", \"selected\"]").unwrap()+start;
    s.replace_range(start..end,r#"    let audit=Command::new(r.join("native-helper")).arg("audit").output().unwrap();
    assert!(audit.status.success());fs::write(r.join("pre-capture-source-audit.log"),audit.stdout).unwrap();
    for label in["fresh","selected"] {
        let file=r.join(format!("native/{label}-field-tests/stdout.log"));
        let text=fs::read_to_string(&file).unwrap();
        assert!(text.contains("test result: ok. 34 passed; 0 failed"));
        assert!(text.contains("test direct_input::tests::direct_reference_words_match_split_for_unaligned_permuted_and_aliased_inputs ... ok"));
        pin(&mut pins,file);
    }
"#);
    for f in["freeze-test.log","freeze-macros.rs","freeze-macros","freeze-macros.log"]{s=s.replace(&format!("        \"{f}\",\n"),"");}
    let anchor="        \"control-identity-audit.json\",\n";assert!(s.contains(anchor));s=s.replace(anchor,&format!("{anchor}        \"compiler.txt\",\n        \"machine.txt\",\n        \"host-workload.txt\",\n        \"pre-capture-source-audit.log\",\n        \"make-experiment.rs\",\n"));
    s=s.replace("for (kind, n) in [(\"g1-borrowed-signed\", 352), (\"g2-prefix\", 200)]", "for (kind, n) in [(\"g1-borrowed-signed\", 352), (\"g1-owned\", 200)]");
    s=s.replace("for kind in [\"g1-borrowed-signed\", \"g2-prefix\"]", "for kind in [\"g1-borrowed-signed\", \"g1-owned\"]");
    s=s.replace("if a == \"selected\" {", "if a == \"selected\" && kind.starts_with(\"g1-\") {");
    s=s.replace("+= x.ln() / 6.;", "+= x.ln() / 4.;");
    s=s.replace("\"independent_proof_count\":12785","\"independent_proof_count\":12983");
    let start=s.find("    let m=read(").unwrap();let end=s[start..].find("fn main()").unwrap()+start;
    s.replace_range(start..end,r#"    save(r.join("tables.json"),&json!({"latency":table,"pairs":pair_table,"profiles":profile_table}));
    println!("Direct referenced input screen {status}; failed checks {:?}",failed);
    for(k,v)in cells {println!("{k} {}",v["median_process_mean_ns_per_batch"]);}
    for(k,v)in agg {println!("aggregate/{k} {v}");}
}
"#);
    fs::write(format!("{R}/experiment.rs"),s).unwrap();println!("Preparedcallerharnesswith4G1aggregateand2G2controls;noinversecandidatecode");
}
