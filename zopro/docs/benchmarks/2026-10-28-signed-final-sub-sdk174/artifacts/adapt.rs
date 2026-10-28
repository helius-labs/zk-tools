use std::fs;
fn main(){
 let mut s=fs::read_to_string("/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174/capture.rs").unwrap();
 let changes=[
 ("/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174","/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174"),
 ("/home/ubuntu/src/zopro-perf-work/symmetric-norm174","/home/ubuntu/src/zopro-perf-work/unweighted-triple-census174"),
 ("zopro_g2_phased_inverse","zopro_signed_final_sub"),("phased_inverse_compile_cfg","signed_final_sub_compile_cfg"),
 ("let expected=read(MANIFEST)[\"rustflags\"].as_str().unwrap().to_string();","let expected=fs::read_to_string(\"/home/ubuntu/src/zopro-perf-work/signed-final-sub174/native-flags.txt\").unwrap();"),
 ("let cases = read(format!(\"{OLD}/cases.json\"));","let cases = read(\"/home/ubuntu/src/zopro-perf-work/g2-phased-inverse174/cases.json\");"),
 ("15541","15757"),("15433","15553"),("15325","15541"),("9672","18696"),
 ("for b in 0..7 {","for b in 0..15 {"),("for i in 0..7 {","for i in 0..15 {"),
 ("assert_eq!(jobs.len(), 96);","assert_eq!(jobs.len(), 192);"),
 ("assert_eq!(steps.len(), 192);","assert_eq!(steps.len(), 384);"),
 ("assert_eq!(jobs.len(), 108);","assert_eq!(jobs.len(), 204);"),
 ("assert_eq!(steps.len(), 234);","assert_eq!(steps.len(), 426);"),
 ("Prepared 234 steps","Prepared 426 steps"),("108 exports","204 exports"),
 ("assert_eq!(inventory.len(), 216);","assert_eq!(inventory.len(), 408);"),
 ("assert_eq!(rows.len(), 7);","assert_eq!(rows.len(), 15);"),
 ("s[3]","s[7]"),("s[0]","s[3]"),("s[6]","s[11]"),("wins >= 5","wins >= 10"),("0.984375","0.96484375"),
 ("completed phased four-chain G2 inverse private pilot; no adoption","completed signed final-carry subtraction SDK pilot;no adoption"),
 ("\"new_independent_exports\":108","\"new_independent_exports\":204"),
 ("\"native_field_tests\":\"not rerun; field source unchanged, inherited qualification\"","\"native_field_tests\":{\"reused_component_OFF\":33,\"reused_component_ON\":35}"),
 ("\"native_core_tests\":{\"OFF\":124,\"ON\":124}","\"native_core_tests\":{\"reused_component_OFF\":125,\"reused_component_ON\":125}"),
 ("four-chain norm inversion before G2 point finish pilot4cells; globalDEDUP1+forecasts unchanged, not all8/clean/resource qualification","post-REDC signed final-carry G1 x/y outputs;15blocks4cells;globalDEDUP1/forecasts unchanged;not all8/clean/resource qualification")
 ];
 for(b,a)in changes {assert!(s.contains(b),"missing exact adaptation anchor {b}");s=s.replace(b,a);}
 let before="    for label in [\"control\",\"candidate\"] {\n        for (package,n) in [(\"zopro-core\",124),(\"zopro-prover\",8)]{\n            let t=read(r(format!(\"native/{package}-{label}/tests.json\")));\n            assert_eq!(t[\"passed_tests\"].as_array().unwrap().len(),n);\n        }\n    }";
 let after="    for label in [\"control\",\"candidate\"] {\n        let t=read(r(format!(\"native/zopro-prover-{label}/tests.json\")));\n        assert_eq!(t[\"passed_tests\"].as_array().unwrap().len(),8);\n        let t=read(format!(\"/home/ubuntu/src/zopro-perf-work/signed-final-sub174/native/zopro-core-{label}/tests.json\"));\n        assert_eq!(t[\"passed_tests\"].as_array().unwrap().len(),125);\n    }";
 assert_eq!(s.matches(before).count(),1);s=s.replace(before,after);
 s=s.replace("\"accept\"","\"accepted\"").replace("\"offset\"","\"OFF\"").replace("\"bypass\"","\"ON\"");
 s=s.replace("108Gnark","204Gnark");
 fs::write("/home/ubuntu/src/zopro-perf-work/signed-final-sub-sdk174/capture.rs",s).unwrap();
}
