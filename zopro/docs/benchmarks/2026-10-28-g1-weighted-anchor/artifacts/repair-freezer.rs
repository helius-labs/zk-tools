use std::fs;
fn main() {
    let s=fs::read_to_string("native.rs").unwrap();
    let old="    assert_eq!(fingerprints.len(), 2);";
    assert_eq!(s.matches(old).count(),1);
    let new="    assert_eq!(fingerprints.len(), 3);\n    assert_eq!(fingerprints.iter().filter(|v| v[\"path\"].as_str().unwrap().contains(\"zopro-core-\")).count(), 1);\n    assert_eq!(fingerprints.iter().filter(|v| v[\"path\"].as_str().unwrap().contains(\"zopro-bn254-\")).count(), 2);\n    let field:Vec<_>=fingerprints.iter().filter(|v| v[\"path\"].as_str().unwrap().contains(\"zopro-bn254-\")).collect();\n    assert_eq!(field[0][\"receipt\"][\"features\"],field[1][\"receipt\"][\"features\"]);\n    assert_ne!(field[0][\"receipt\"][\"profile\"],field[1][\"receipt\"][\"profile\"]);";
    fs::write("native-v2.rs",s.replace(old,new)).unwrap();
    fs::write("freezer-repair.json",r#"{"failure":"freezer expected two fingerprints; native core tests and bench build generated two bn254 profile fingerprints plus one core fingerprint","scope":"metadata-only count correction; both field fingerprints have identical features and rustflags and different Cargo profiles; freeze uses only exact bench library artifacts","core_tests_passed":124,"successful_tests_or_builds_repeated":false,"timing_started":false,"original_failure_preserved":"native/failure.json"}"#).unwrap();
}
