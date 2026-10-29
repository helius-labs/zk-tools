fn inspect() {
    use std::process::Command;
    let binary = r("native/candidate-sdk");
    let o = Command::new("nm")
        .args(["-S", "--size-sort", "--defined-only", "-C"])
        .arg(&binary)
        .output()
        .unwrap();
    assert!(o.status.success());
    let text = String::from_utf8(o.stdout).unwrap();
    fs::create_dir(r("codegen")).unwrap();
    let mut selected = vec![];
    for line in text.lines() {
        let w: Vec<_> = line.split_whitespace().collect();
        if w.len() < 4 || !line.contains("g2_borrowed::gls4::census") || !["t", "T"].contains(&w[2])
        {
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
        let path = r(format!("codegen/census-{}.asm", selected.len()));
        fs::write(&path, o.stdout).unwrap();
        selected.push(json!({"symbol":w[3..].join(" "),"bytes":bytes,"assembly":path,"assembly_sha256":hash(&path)}));
    }
    assert!(!selected.is_empty());
    fs::write(r("codegen/symbols.txt"), text).unwrap();
    save(
        r("code-audit.json"),
        &json!({"linked_observer_present":true,"performance_gain_claimed":false,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}),
    );
    println!("Linkedobserver present; actual records/flows must validate after diagnostic");
}
