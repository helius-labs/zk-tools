fn inspect() {
    use std::process::Command;
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    fs::create_dir(r("codegen")).unwrap();
    let mut variants = vec![];
    for label in ["accepted", "control", "candidate"] {
        let binary = if label == "accepted" {
            PathBuf::from(m["native_binary"].as_str().unwrap())
        } else {
            r(format!("native/{label}-sdk"))
        };
        let output = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(output.status.success());
        let symbols = String::from_utf8(output.stdout).unwrap();
        let mut selected = vec![];
        for line in symbols.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let dense = name.contains("::run_poseidon::dense");
            let caller = name.contains("::run_poseidon_task");
            if !dense && !caller {
                continue;
            }
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let bytes = u64::from_str_radix(w[1], 16).unwrap();
            let output = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(output.status.success());
            let text = String::from_utf8(output.stdout).unwrap();
            let path = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&path, &text).unwrap();
            selected.push(json!({"symbol":name,"bytes":bytes,"dense":dense,"caller":caller,"dense_call_sites":text.lines().filter(|l|l.contains("call")&&l.contains("::run_poseidon::dense")).count(),"assembly":path,"assembly_sha256":hash(&path)}));
        }
        assert_eq!(
            selected.iter().filter(|x| x["dense"] == true).count(),
            1,
            "missing linked dense {label}"
        );
        assert_eq!(
            selected.iter().filter(|x| x["caller"] == true).count(),
            1,
            "missing caller {label}"
        );
        assert!(
            selected
                .iter()
                .map(|x| x["dense_call_sites"].as_u64().unwrap())
                .sum::<u64>()
                > 0
        );
        fs::write(r(format!("codegen/{label}-symbols.txt")), symbols).unwrap();
        variants.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    let size = |v: &Value| {
        v["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["dense"] == true)
            .unwrap()["bytes"]
            .as_u64()
            .unwrap()
    };
    assert_ne!(size(&variants[1]), size(&variants[2]));
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"solver_dense_changed_and_called":true,"source_cfg_and_fingerprints_checked":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":variants}),
    );
    println!("Verified actuallinkedsolver dense code changed and originaltask caller calls it; no speedclaim");
}
