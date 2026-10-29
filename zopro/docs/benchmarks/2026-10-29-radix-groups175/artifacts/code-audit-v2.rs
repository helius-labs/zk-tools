fn inspect() {
    use std::process::Command;
    fs::create_dir(r("codegen-v2")).unwrap();
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    let mut variants = vec![];
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
                    .contains("--cfg zopro_radix_groups"),
                label == "candidate"
            );
        }
        let o = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let symbols = String::from_utf8(o.stdout).unwrap();
        fs::write(r(format!("codegen-v2/{label}-symbols.txt")), &symbols).unwrap();
        let mut selected = vec![];
        let mut child = 0;
        for line in symbols.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let entry = name == "zopro_core::msm_affine::grouping::scalar_groups";
            let radix = name.contains("zopro_core::msm_affine::grouping::radix::");
            if !entry && !radix {
                continue;
            }
            if radix {
                child += 1;
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
            let p = r(format!("codegen-v2/{label}-{}.asm", selected.len()));
            fs::write(&p, o.stdout).unwrap();
            selected.push(json!({"symbol":name,"bytes":bytes,"grouping_entry":entry,"collision_sort_instantiation":radix,"assembly":p,"assembly_sha256":hash(&p)}));
        }
        assert_eq!(child > 0, label == "candidate");
        assert_eq!(
            selected
                .iter()
                .filter(|s| s["grouping_entry"] == true)
                .count(),
            1
        );
        variants.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    let size = |i: usize| {
        variants[i]["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|f| f["grouping_entry"] == true)
            .unwrap()["bytes"]
            .as_u64()
            .unwrap()
    };
    assert_eq!(size(0), size(1));
    assert_ne!(size(1), size(2));
    let off = read(r("frozen-control.json"));
    let on = read(r("frozen-candidate.json"));
    for lib in ["zopro_core", "zopro_prover"] {
        assert_ne!(
            hash(off["libraries"][lib].as_str().unwrap()),
            hash(on["libraries"][lib].as_str().unwrap())
        );
    }
    assert_ne!(variants[1]["binary_sha256"], variants[2]["binary_sha256"]);
    save(
        r("code-audit-v2.json"),
        &json!({"structural_checks_passed":true,"actual_grouping_entry_changed":true,"collision_sort_instantiation_code_only_in_ON":true,"OFF_entry_size_equal_accepted":true,"OFF_byte_identical_accepted":variants[0]["binary_sha256"]==variants[1]["binary_sha256"],"distinct_OFF_ON_core_prover_SDK_identities":true,"static_code_not_speed_evidence":true,"variants":variants}),
    );
    println!("Audited inlined grouping entry and ON-only collision-sort specializations; standalone radix symbol not required");
}
