fn counts(text: &str) -> BTreeMap<String, Value> {
    let mut out = BTreeMap::new();
    let mut name = None;
    for line in text.lines() {
        if line.starts_with("  ") && !line.starts_with("    ") && line.ends_with(':') {
            name = Some(line.trim().trim_end_matches(':').to_string());
        }
        if let Some(data) = line.trim().strip_prefix("Block counts: ") {
            let raw = name.take().expect("counts without function");
            let c: Vec<u64> = serde_json::from_str(data).unwrap();
            assert!(!c.is_empty());
            let start = raw.find("_R").or_else(|| raw.find("_ZN")).unwrap_or(0);
            let demangled = rustc_demangle::try_demangle(&raw[start..])
                .map(|d| format!("{d:#}"))
                .unwrap_or_else(|_| raw.clone());
            let max = *c.iter().max().unwrap();
            assert!(out
                .insert(
                    raw,
                    json!({"demangled":demangled,"counts":c,"max_block_count":max})
                )
                .is_none());
        }
    }
    assert!(!out.is_empty());
    out
}
fn pick(c: &BTreeMap<String, Value>, module: &str, method: &str) -> Value {
    let matches: Vec<_> = c
        .iter()
        .filter(|(_, v)| {
            let n = v["demangled"].as_str().unwrap();
            n.contains(module) && n.contains("Prover") && n.ends_with(method)
        })
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "ambiguous {module}/{method}:{}",
        json!(matches)
    );
    json!({"raw":matches[0].0,"profile":matches[0].1})
}
