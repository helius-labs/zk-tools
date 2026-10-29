fn inspect() {
    use std::process::Command;
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    fs::create_dir(r("codegen")).unwrap();
    let mut results = vec![];
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
        let mut g2 = 0;
        let mut g1 = 0;
        for line in symbols.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let is_g2 = name.contains("zopro_core::msm::g2_borrowed::apply");
            let is_g1 = name.contains("stream_g1_impl") && name.contains(", true, true, &");
            if !is_g1 && !is_g2 {
                continue;
            }
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let bytes = u64::from_str_radix(w[1], 16).unwrap();
            let asm = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(asm.status.success());
            let text = String::from_utf8(asm.stdout).unwrap();
            let file = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&file, &text).unwrap();
            let ifma = text.lines().filter(|l| l.contains("vpmadd52")).count();
            assert!(ifma > 0);
            if is_g2 {
                g2 += 1;
            }
            if is_g1 {
                g1 += 1;
            }
            selected.push(json!({"symbol":name,"bytes":bytes,"ifma_sites":ifma,"assembly":file,"assembly_sha256":hash(&file),"G2":is_g2,"G1":is_g1}));
        }
        assert!(g2 > 0 && g1 > 0, "missing linked native callers: {label}");
        fs::write(r(format!("codegen/{label}-symbols.txt")), symbols).unwrap();
        results.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    // Source/flags/fingerprints and independent native tests qualify dispatch.
    // Selected linked caller bytes must change; instruction counts are static.
    for key in ["G1", "G2"] {
        let c = results[1]["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v[key] == true)
            .map(|v| v["bytes"].as_u64().unwrap())
            .sum::<u64>();
        let n = results[2]["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v[key] == true)
            .map(|v| v["bytes"].as_u64().unwrap())
            .sum::<u64>();
        assert_ne!(c, n, "actual caller code unchanged for {key}");
    }
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"selection":"Fq/R256/eight lanes ordinary and fused only","source_cfg_and_fingerprints_checked":true,"actual_G1_G2_IFMA_callers_changed":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":results}),
    );
    println!("Actual linked G1/G2 native IFMA caller code changed; source/cfg qualification separate; no speed claim");
}
