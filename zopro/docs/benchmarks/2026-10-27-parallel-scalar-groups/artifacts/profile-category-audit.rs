use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new("/tmp/zopro-parallel-scalar-groups");
    let d = Path::new(
        "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-parallel-scalar-groups",
    );
    let p = read(r.join("performance/protocol.json"));
    let categories = p["symbol_categories"].as_array().unwrap();
    let rows = read(r.join("performance/rows.json"));
    let mut audit = Vec::new();
    for row in rows
        .as_array()
        .unwrap()
        .iter()
        .filter(|r| r["phase"] == "profile" && r["variant"] == "candidate")
    {
        let symbols = row["profile"]["symbol_periods"].as_object().unwrap();
        let mut total = 0u64;
        let mut matched = 0;
        for (symbol, period) in symbols
            .iter()
            .filter(|(symbol, _)| symbol.contains("grouping::parallel"))
        {
            let cat = categories
                .iter()
                .find(|cat| {
                    cat["contains"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|s| symbol.contains(s.as_str().unwrap()))
                })
                .map_or("other", |c| c["name"].as_str().unwrap());
            assert_eq!(cat, "other");
            total = total.checked_add(period.as_u64().unwrap()).unwrap();
            matched += 1;
        }
        assert!(matched > 0);
        let all = row["profile"]["period"].as_u64().unwrap();
        audit.push(json!({"case":row["case"],"variant":"candidate","matching_symbols":matched,"parallel_grouping_symbol_period":total,"total_user_cycle_period":all,"parallel_grouping_symbol_percent":100.*total as f64/all as f64,"original_category":"other","original_grouping_percent":row["profile"]["category_percent"]["grouping"],"not_exclusive_function_or_stage_cost":true}));
    }
    assert_eq!(audit.len(), 2);
    let result = json!({"stage":"offline after all captures terminal","original_profiles_and_categories_unchanged":true,"new_captures":0,"SDK_or_verifier_repeats":0,"scope":"Only explicit grouping::parallel symbol paths; inlining prevents a complete grouping-cost estimate","findings":audit});
    fs::write(
        r.join("profile-category-audit.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    let mut index = read(d.join("artifact-index.json"));
    for name in ["profile-category-audit.rs", "profile-category-audit.json"] {
        let source = r.join(name);
        let dest = d.join("artifacts").join(name);
        fs::copy(&source, &dest).unwrap();
        assert_eq!(hash(&source), hash(&dest));
        assert!(index
            .as_object_mut()
            .unwrap()
            .insert(name.into(), json!(hash(dest)))
            .is_none());
    }
    fs::write(
        d.join("artifact-index.json"),
        serde_json::to_vec_pretty(&index).unwrap(),
    )
    .unwrap();
    println!(
        "Added two offline category-audit artifacts; {} total hashes; no category/capture changed",
        index.as_object().unwrap().len()
    );
}
