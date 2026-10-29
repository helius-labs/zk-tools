fn inspect() {
    use std::process::Command;
    fs::create_dir(r("codegen")).unwrap();
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    let mut results = vec![];
    for label in ["accepted", "control", "candidate"] {
        let binary = if label == "accepted" {
            PathBuf::from(m["native_binary"].as_str().unwrap())
        } else {
            r(format!("native/{label}-sdk"))
        };
        if label != "accepted" {
            let frozen = read(r(format!("frozen-{label}.json")));
            assert_eq!(
                frozen["rustflags"]
                    .as_str()
                    .unwrap()
                    .contains("--cfg zopro_direct_finish_reduction"),
                label == "candidate"
            );
        }
        let out = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(out.status.success());
        let symbols = String::from_utf8(out.stdout).unwrap();
        fs::write(r(format!("codegen/{label}-symbols.txt")), &symbols).unwrap();
        let mut functions = vec![];
        for line in symbols
            .lines()
            .filter(|l| l.contains("zopro_bn254::inverse::inverse::<"))
        {
            let w: Vec<_> = line.split_whitespace().collect();
            let name = w[3..].join(" ");
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let bytes = u64::from_str_radix(w[1], 16).unwrap();
            let o = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(o.status.success());
            let path = r(format!("codegen/{label}-{}.asm", functions.len()));
            fs::write(&path, o.stdout).unwrap();
            functions.push(
                json!({"symbol":name,"bytes":bytes,"assembly":path,"assembly_sha256":hash(&path)}),
            );
        }
        for config in ["FqConfig", "FrConfig"] {
            assert!(functions
                .iter()
                .any(|f| f["symbol"].as_str().unwrap().contains(config)));
        }
        results.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"inverse_functions":functions}));
    }
    for config in ["FqConfig", "FrConfig"] {
        let sizes = |i: usize| {
            let mut v: Vec<_> = results[i]["inverse_functions"]
                .as_array()
                .unwrap()
                .iter()
                .filter(|f| f["symbol"].as_str().unwrap().contains(config))
                .map(|f| f["bytes"].as_u64().unwrap())
                .collect();
            v.sort();
            v
        };
        assert_eq!(
            sizes(0),
            sizes(1),
            "fresh OFF inverse changed relative to accepted"
        );
        assert_ne!(sizes(1), sizes(2), "ON inverse code unchanged");
    }
    let off = read(r("frozen-control.json"));
    let on = read(r("frozen-candidate.json"));
    for lib in ["zopro_bn254", "zopro_core", "zopro_prover"] {
        assert_ne!(
            hash(off["libraries"][lib].as_str().unwrap()),
            hash(on["libraries"][lib].as_str().unwrap())
        );
    }
    assert_ne!(results[1]["binary_sha256"], results[2]["binary_sha256"]);
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"actual_Fq_Fr_inverse_functions_changed":true,"OFF_inverse_function_sizes_equal_accepted":true,"distinct_OFF_ON_libraries_and_executables":true,"source_cfg_and_fingerprints_checked":true,"static_counts_not_speed_evidence":true,"variants":results}),
    );
    println!("Audited distinct immutable OFF/ON libraries and SDKs; actual Fq/Fr inverse bodies differ, OFF sizes equal accepted");
}
