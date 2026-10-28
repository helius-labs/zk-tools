use std::fs;
const R:&str="/home/ubuntu/src/zopro-perf-work/scalar-square-sdk";
fn main(){
    let old=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/inverse-finish-sdk/finish.rs").unwrap();
    let end=old.find("    let manifest=read(").unwrap();
    let mut s=old[..end].to_string().replace("/zopro-perf-work/inverse-finish-sdk","/zopro-perf-work/scalar-square-sdk").replace("2026-10-28-inverse-finish-sdk","2026-10-28-scalar-square-sdk");
    let start=s.find("    for (mode, variant) in").unwrap();let end=s[start..].find("    let off = read(").unwrap()+start;
    s.replace_range(start..end,r#"    let mut core_counts=BTreeMap::new();
    for mode in["off","on"] {
        let t=fs::read_to_string(r.join(format!("native/core-{mode}/stdout.log"))).unwrap();
        let line=t.lines().find_map(|l|l.strip_prefix("test result: ok. ")).unwrap();
        let count:usize=line.split_whitespace().next().unwrap().parse().unwrap();assert!(count>=100);
        assert!(line.contains("0 failed"));core_counts.insert(mode,count);
        for(name,n)in[("raw",2),("cubic",6)] {let text=fs::read_to_string(r.join(format!("native/{name}-{mode}/stdout.log"))).unwrap();assert!(text.contains(&format!("test result: ok. {n} passed; 0 failed")));}
    }
"#);
    s=s.replace("s.contains(\"zopro_bn254::inverse::inverse\")","s.contains(\"zopro_core::solver\")");
    s=s.replace("inverse_symbol_period","solver_symbol_period");
    s=s.replace("assert_eq!(seen.len(), 12884);","assert_eq!(seen.len(), 12983);");
    s=s.replace("assert_eq!(seen.len(), 12785);","assert_eq!(seen.len(), 12884);");
    s=s.replace("\"independent_proof_count\":12884","\"independent_proof_count\":12983");
    s=s.replace("\"inherited_adapter_tests\":3,\"core_tests_per_configuration\":122","\"inherited_adapter_tests\":0,\"core_tests_per_configuration\":core_counts");
    s.push_str(r#"
    save(r.join("tables.json"),&json!({"latency":table,"pairs":pair_table,"solver_profiles":profile_table}));
    println!("SDKscreen {status}; failed checks {:?}; ledger12983",failed);
    for(k,v)in &cells{println!("{k} {v}");}for(k,v)in &pairs{println!("{k} {v}");}
}
"#);
    fs::write(format!("{R}/finish.rs"),s).unwrap();println!("Prepared solver-profile/fullSDK analyzer; archive/report handled after result");
}
