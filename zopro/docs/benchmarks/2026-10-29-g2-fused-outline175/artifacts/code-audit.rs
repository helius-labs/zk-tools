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
        let text = String::from_utf8(output.stdout).unwrap();
        let mut selected = vec![];
        let mut outlined = vec![];
        let mut callers = vec![];
        let mut selection_lines = String::new();
        for line in text.lines() {
            let words: Vec<_> = line.split_whitespace().collect();
            if words.len() < 4 {
                continue;
            }
            let name = words[3..].join(" ");
            let helper = name.contains("product_cios_outlined");
            let g2 = name.contains("zopro_core::msm::g2_borrowed::apply");
            let g1 = name.contains("stream_g1_impl") && name.contains(", true, true, &");
            if !helper && !g2 && !g1 {
                continue;
            }
            let address = u64::from_str_radix(words[0], 16).unwrap();
            let bytes = u64::from_str_radix(words[1], 16).unwrap();
            selection_lines.push_str(line);
            selection_lines.push('\n');
            let assembly = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(assembly.status.success());
            let asm = String::from_utf8(assembly.stdout).unwrap();
            let path = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&path, &asm).unwrap();
            let instructions: Vec<_> = asm
                .lines()
                .filter_map(|l| {
                    let (a, b) = l.trim().split_once(':')?;
                    if !a.is_empty()
                        && a.chars().all(|c| c.is_ascii_hexdigit())
                        && !b.trim().is_empty()
                    {
                        Some(b.trim())
                    } else {
                        None
                    }
                })
                .collect();
            let calls = instructions
                .iter()
                .filter(|s| {
                    (s.starts_with("call") || s.starts_with("jmp"))
                        && s.contains("product_cios_outlined")
                })
                .count();
            let item = json!({"symbol":name,"address":address,"bytes":bytes,"instruction_lines":instructions.len(),"ifma_sites":instructions.iter().filter(|s|s.contains("vpmadd52")).count(),"stack_operand_lines":instructions.iter().filter(|s|s.contains("%rsp")||s.contains("%rbp")).count(),"outline_call_or_jump_sites":calls,"assembly":path,"assembly_sha256":hash(&path)});
            if helper {
                outlined.push(item.clone());
            }
            if g2 {
                callers.push(item.clone());
            }
            selected.push(item);
        }
        assert!(
            !callers.is_empty(),
            "missing actual G2 borrowed caller for {label}"
        );
        if label == "candidate" {
            assert!(!outlined.is_empty());
            for h in &outlined {
                let name = h["symbol"].as_str().unwrap().replace(' ', "");
                assert!(
                    name.contains("ifma8")
                        && name.contains("FqConfig,true>")
                        && name.contains("product_cios_outlined::<true>"),
                    "unexpected outlined mono {name}"
                );
            }
            assert!(
                callers
                    .iter()
                    .map(|v| v["outline_call_or_jump_sites"].as_u64().unwrap())
                    .sum::<u64>()
                    > 0,
                "G2 caller does not call outline helper"
            );
        } else {
            assert!(outlined.is_empty());
            assert!(callers.iter().all(|v| v["outline_call_or_jump_sites"] == 0));
        }
        fs::write(r(format!("codegen/{label}-symbols.txt")), selection_lines).unwrap();
        results.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected,"outlined_helpers":outlined,"G2_borrowed_callers":callers}));
    }
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"selection":"only8laneFqMixedFusedtrue","caller_route_verified":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":results}),
    );
    println!("Verified outlined Fq8MixedFusedtrue mono and actual G2 caller calls;static3artifact audit;no speed claim");
}
