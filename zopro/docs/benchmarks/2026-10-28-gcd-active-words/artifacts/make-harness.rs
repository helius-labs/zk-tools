use std::fs;
fn replace(s: &mut String, a: &str, b: &str, n: usize) {
    assert_eq!(s.matches(a).count(), n, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let p = "/home/ubuntu/src/zopro-perf-work/concurrent-callers";
    let mut s = fs::read_to_string(format!("{p}/probe.rs")).unwrap();
    replace(&mut s, "f.set_weighted(mode == 2);", "", 2);
    replace(
        &mut s,
        "a[1] == \"g1-borrowed-signed\" && n == 352",
        "a[1] == \"g1-borrowed-signed\" && [200,352].contains(&n)",
        1,
    );
    replace(
        &mut s,
        "a[1] == \"g2-prefix\" && n == 352",
        "a[1] == \"g2-prefix\" && n == 200",
        1,
    );
    replace(&mut s, "(\"g2-prefix\", 352)", "(\"g2-prefix\", 200)", 1);
    replace(
        &mut s,
        "(\"g1-borrowed-signed\", 352),",
        "(\"g1-borrowed-signed\", 200),\n            (\"g1-borrowed-signed\", 352),",
        1,
    );
    replace(
        &mut s,
        "digests.insert(kind, h.clone())",
        "digests.insert((kind,n), h.clone())",
        1,
    );
    fs::write("probe.rs", s).unwrap();
    let s = fs::read_to_string(format!("{p}/driver.rs")).unwrap();
    let start = s.find("fn analyze()").unwrap();
    let end = s[start..]
        .find("    let mut cells = BTreeMap::new();")
        .unwrap()
        + start;
    let mut s = s[start..end].to_string();
    replace(&mut s,"fn analyze()","fn collect() -> (BTreeMap<String,Value>,BTreeMap<String,Value>,BTreeMap<String,Value>,BTreeMap<String,Value>,u64)",1);
    replace(
        &mut s,
        "format!(\"{}-{}\", j[\"kind\"], j[\"rounds\"])",
        "format!(\"{}-{}-{}\", j[\"kind\"], j[\"size\"], j[\"rounds\"])",
        1,
    );
    replace(
        &mut s,
        "let key = j[\"kind\"].as_str().unwrap();",
        "let key = format!(\"{}-{}\",j[\"kind\"],j[\"size\"]);",
        1,
    );
    s.push_str("    (results,stat,profiles,caps,total_calls)\n}\n");
    fs::write("collect.rs", s).unwrap();
    println!("Preparedsameconcurrencyharnesswithoutweightedsetter;frozen/OFF/ONexecontrols;collectorkeysincludesize");
}
