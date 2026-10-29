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
        let o = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let text = String::from_utf8(o.stdout).unwrap();
        let mut selected = vec![];
        let mut new_count = 0;
        for line in text.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let new = name.contains("msm::packed_index");
            let caller = name.contains("msm::reduce_indexed_buckets");
            if !new && !caller {
                continue;
            }
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
            let asm = String::from_utf8(o.stdout).unwrap();
            let path = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&path, &asm).unwrap();
            new_count += usize::from(new);
            selected.push(json!({"symbol":name,"bytes":bytes,"new_packed_index":new,"existing_caller":caller,"assembly":path,"assembly_sha256":hash(&path)}));
        }
        if label == "candidate" {
            assert!(new_count > 0);
        } else {
            assert_eq!(new_count, 0);
        }
        fs::write(r(format!("codegen/{label}-symbols.txt")), text).unwrap();
        variants.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"linked_packed_index_only_ON":true,"activation_requires_separate_primary_preflight_trace":true,"source_cfg_and_fingerprints_checked":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":variants}),
    );
    println!("LinkednewPackedIndexcode onlyON; actualactivationmustpassseparateprimarypreflighttrace");
}
