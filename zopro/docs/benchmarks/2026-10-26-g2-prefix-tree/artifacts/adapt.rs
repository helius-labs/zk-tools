use std::{fs,path::Path};
const R:&str="/tmp/zopro-g2-prefix-tree";
fn main(){
 for name in ["setup.rs","run.rs"]{let p=Path::new(R).join(name);let mut s=fs::read_to_string(&p).unwrap().replace("/tmp/zopro-g2-level-tree",R);
 if name=="setup.rs"{
 s=s.replace("--cfg zopro_g2_level_tree\"","--cfg zopro_g2_level_tree --cfg zopro_g2_prefix_tree\"");
 s=s.replace("\"msm_affine::g2_level_tree::tests::checked_forecast_covers_empty_and_boundary_and_rejects_geometry\"", "\"msm_affine::g2_level_tree::tests::checked_forecast_covers_empty_and_boundary_and_rejects_geometry\",\"msm_affine::g2_prefix_tree::tests::prefix_query_complete_curve_generator_oracles\",\"msm_affine::g2_prefix_tree::tests::actual_size_prefix_matches_stream_and_generator\",\"msm_affine::g2_prefix_tree::tests::prefix_forecast_and_geometry_are_checked\",\"msm::g2_prefix_tree_tests::grouped_prefix_handles_signed_exceptions_odd_and_skewed_buckets\",\"msm::g2_prefix_tree_tests::prefix_group_cost_rejects_overflow\"");
 s=s.replace("\"candidate/crates/zopro-core/src/msm_affine/g2_level_tree.rs\",", "\"candidate/crates/zopro-core/src/msm_affine/g2_level_tree.rs\",\"candidate/crates/zopro-core/src/msm_affine/g2_prefix_tree.rs\",\"adapt.rs\",\"analyze.inc.rs\",");
 s=s.replace("\"new_module\":\"msm_affine/g2_level_tree.rs\"", "\"new_modules\":[\"msm_affine/g2_level_tree.rs\",\"msm_affine/g2_prefix_tree.rs\"]");
 }else{
 s=s.replace(".len(),112)",".len(),117)");
 s=s.replace("let mut modes=vec![\"stream\",\"tree\"];", "let mut modes=vec![\"frozen\",\"stream\",\"tree\",\"prefix\"];modes.rotate_left(block%4);");
 s=s.replace("let mut e=env.clone();", "let selected_binary=if *mode==\"frozen\"{\"/tmp/zopro-g2-level-tree/native/on-sdk\"}else{binary};let cli_mode=if *mode==\"frozen\"{\"stream\"}else{*mode};let mut e=env.clone();");
 s=s.replace("affinity,binary,r.join(\"cases.json\"),c[\"shape\"],mode,", "affinity,selected_binary,r.join(\"cases.json\"),c[\"shape\"],cli_mode,");
 s=s.replace("assert_eq!(jobs.len(),84)","assert_eq!(jobs.len(),168)");
 s=s.replace("for mode in [\"stream\",\"tree\"]{let d=", "for mode in [\"stream\",\"tree\",\"prefix\"]{let d=");
 s=s.replace("json!(\"--delay=-1\")", "json!(\"--no-buildid-cache\"),json!(\"--delay=-1\")");
 s=s.replace("pins.insert(binary.into(),hash(binary));", "pins.insert(binary.into(),hash(binary));let old=\"/tmp/zopro-g2-level-tree/native/on-sdk\";assert_eq!(hash(old),\"1f273ced307d9946ede09bd7622c654e5c91951d6c53d3190ade1ae23bde4a67\");pins.insert(old.into(),hash(old));");
 s=s.replace("Prepared84latenciesTHEN2gatedprofiles;974rawquerycallsincl86references","Prepared168latenciesTHEN3gatedprofiles;1881rawqueriesincl171references");
 let start=s.find("fn analyze()").unwrap();let end=s.find("fn main()").unwrap();let replacement=fs::read_to_string(Path::new(R).join("analyze.inc.rs")).unwrap();s.replace_range(start..end,&replacement);
 }
 fs::write(p,s).unwrap();
 }
}
