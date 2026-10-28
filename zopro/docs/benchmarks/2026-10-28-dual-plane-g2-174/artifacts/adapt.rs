use std::fs;
fn main(){
 let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174/capture.rs").unwrap();
 let edits=[
 ("const R: &str = \"/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174\";","const R: &str = \"/home/ubuntu/src/zopro-perf-work/dual-plane-g2-174\";"),
 ("const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/unweighted-triple-census174\";","const OLD: &str = \"/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174\";"),
 ("zopro_signed_final_sub","zopro_g2_dual_plane"),("signed_final_sub_compile_cfg","dual_plane_compile_cfg"),
 ("/home/ubuntu/src/zopro-perf-work/signed-final-sub174/native-flags.txt","/home/ubuntu/src/zopro-perf-work/dual-plane-g2-174/native-flags.txt"),
 ("let cases = read(\"/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174/cases.json\");","let all=read(\"/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174/cases.json\");let cases=json!([all[0].clone()]);"),
 ("assert_eq!(cases.as_array().unwrap().len(), 4);","assert_eq!(cases.as_array().unwrap().len(), 1);"),
 ("vec![0, 1, 2, 3]","vec![0]"),("order.rotate_left(b % 4);","order.rotate_left(0);"),
 ("15757","15811"),("15553","15757"),("15541","15553"),("18696","7806"),
 ("assert_eq!(jobs.len(), 192);","assert_eq!(jobs.len(), 48);"),("assert_eq!(steps.len(), 384);","assert_eq!(steps.len(), 96);"),
 ("assert_eq!(jobs.len(), 204);","assert_eq!(jobs.len(), 54);"),("assert_eq!(steps.len(), 426);","assert_eq!(steps.len(), 117);"),
 ("Prepared 426 steps","Prepared 117 steps"),("204 exports","54 exports"),("204Gnark","54Gnark"),
 ("assert_eq!(inventory.len(), 408);","assert_eq!(inventory.len(), 108);"),
 ("assert_eq!(counters.len(),6);","assert_eq!(counters.len(),3);"),("assert_eq!(profiles.len(),6);","assert_eq!(profiles.len(),3);"),
 ("\"new_independent_exports\":204","\"new_independent_exports\":54"),
 ("completed signed final-carry subtraction SDK pilot;no adoption","completed joint two-plane G2 primary SDK screen;no adoption"),
 ("\"native_field_tests\":{\"reused_component_OFF\":33,\"reused_component_ON\":35}","\"native_field_tests\":\"field source exact;inherited accepted33test qualification;not rerun\""),
 ("\"native_core_tests\":{\"reused_component_OFF\":125,\"reused_component_ON\":125}","\"native_core_tests\":{\"OFF\":122,\"ON\":124}"),
 ("post-REDC signed final-carry G1 x/y outputs;15blocks4cells;globalDEDUP1/forecasts unchanged;not all8/clean/resource qualification","joint G2 high-low bucket planes;15blocks primary32 only;one query resource reservation;not larger/all8/clean/resource qualification")
 ];
 for(b,a)in edits{assert!(s.contains(b),"missing {b}");s=s.replace(b,a);}
 let before="        let t=read(format!(\"/home/ubuntu/src/zopro-perf-work/signed-final-sub174/native/zopro-core-{label}/tests.json\"));\n        assert_eq!(t[\"passed_tests\"].as_array().unwrap().len(),125);";
 let after="        let t=read(r(format!(\"native/zopro-core-{label}/tests.json\")));\n        assert_eq!(t[\"passed_tests\"].as_array().unwrap().len(),if label==\"control\"{122}else{124});";
 assert_eq!(s.matches(before).count(),1);s=s.replace(before,after);
 fs::write("/home/ubuntu/src/zopro-perf-work/dual-plane-g2-174/capture.rs",s).unwrap();
}
