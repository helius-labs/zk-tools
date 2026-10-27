use std::fs;
fn main() {
    let old = fs::read_to_string("/tmp/zopro-active-lane-inverse/analyze.rs").unwrap();
    assert_eq!(old.matches("if latency { 4096 } else { 65536 }").count(), 1);
    assert_eq!(
        old.matches(".filter(|c| c[\"case\"][\"pairs\"].as_u64().unwrap() < 8)")
            .count(),
        1
    );
    let new=old.replace("/tmp/zopro-active-lane-inverse","/tmp/zopro-g1-reverse-overlap")
        .replace("if latency { 4096 } else { 65536 }","if latency { 1024 } else { 131072 }")
        .replace(".filter(|c| c[\"case\"][\"pairs\"].as_u64().unwrap() < 8)",".filter(|c| c[\"case\"][\"signed\"] == true && c[\"case\"][\"pairs\"].as_u64().unwrap() >= 200)")
        .replace("changed_short_aggregate","targeted_signed_aggregate").replace("shortaggregate","targeted_signed_aggregate")
        .replace("\"short\"","\"overlap\"").replace("{name} short/","{name} overlap/");
    fs::write("/tmp/zopro-g1-reverse-overlap/analyze.rs", new).unwrap();
}
