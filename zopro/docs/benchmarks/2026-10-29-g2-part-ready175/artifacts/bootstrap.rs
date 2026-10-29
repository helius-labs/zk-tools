use std::{fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/g2-part-ready175";
fn main(){
 let r=Path::new(R); assert!(!r.join("native.rs").exists());
 let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/serial-phi175/native.rs").unwrap().replace("/home/ubuntu/src/zopro-perf-work/serial-phi175",R).replace("ZOPRO_CPU_SERIAL_PHI_PROBE","ZOPRO_CPU_PHI_PART_READY");
 let a=s.find("    let before=").unwrap(); let b=a+s[a..].find("    let p = r(\"source\").join(file);").unwrap();
 s.replace_range(a..b,r#"    let before="    hb_raw(bases, words, 12, 2)";
    let after="    #[cfg(zopro_phi_part_ready)]\n    if std::env::var(\"ZOPRO_CPU_PHI_PART_READY\").as_deref() == Ok(\"1\") {\n        if let Some(result) = part_ready::raw(bases, words) { return result; }\n    }\n    hb_raw(bases, words, 12, 2)";
"#);
 s=s.replace("    let next = text.replace(before, after);", "    let next = text.replace(before, after);\n    let next = format!(\"#[cfg(zopro_phi_part_ready)]\\nmod part_ready;\\n{}\", next);\n    let child = r(\"source/crates/zopro-core/src/msm_affine/g2_phi/part_ready.rs\");\n    fs::create_dir_all(child.parent().unwrap()).unwrap();\n    fs::copy(r(\"part_ready.rs\"), &child).unwrap();\n    pin(&mut pins, &child);");
 s=s.replace("assert_eq!(next.replace(after, before), text);", "assert_eq!(next.strip_prefix(\"#[cfg(zopro_phi_part_ready)]\\nmod part_ready;\\n\").unwrap().replace(after, before), text);");
 s=s.replace("let flags = m[\"rustflags\"].as_str().unwrap();", "let flags = format!(\"{} --cfg zopro_phi_part_ready\", m[\"rustflags\"].as_str().unwrap());");
 s=s.replace("fs::write(r(\"native-flags.txt\"), flags)","fs::write(r(\"native-flags.txt\"), &flags)");
 s=s.replace("\"source-audit.json\",", "\"part_ready.rs\",\"source-audit.json\",");
 s=s.replace("\"new_table_plan_unsafe\":false", "\"new_table_plan_unsafe\":false,\"candidate_files\":176,\"new_child\":\"crates/zopro-core/src/msm_affine/g2_phi/part_ready.rs\",\"field_qualification\":\"inherited33 exact17files\"");
 s=s.replace("\"msm_affine::g2_borrowed::tests::exceptional_keys_and_scalar_edges_match_generator_sum\"", "\"msm_affine::g2_borrowed::tests::exceptional_keys_and_scalar_edges_match_generator_sum\",\"msm_affine::g2_phi::part_ready::tests::slot_preserves_part_order_for_both_arrivals\",\"msm_affine::g2_phi::part_ready::tests::panic_releases_pending_and_consumed_inputs_without_waiting\",\"msm_affine::g2_phi::part_ready::tests::memory_bound_and_independent_generator_oracle_across_threads\"");
 fs::write(r.join("native.rs"),s).unwrap();
}
