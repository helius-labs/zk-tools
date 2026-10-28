use std::fs;
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
    let new = "/home/ubuntu/src/zopro-perf-work/scaled-r260-native";
    for f in ["probe.rs", "perf.rs", "collect.rs"] {
        fs::write(
            f,
            fs::read_to_string(format!("{old}/{f}"))
                .unwrap()
                .replace(old, new),
        )
        .unwrap();
    }
    let s = fs::read_to_string(format!("{old}/native.rs")).unwrap();
    let tail = s[s.find("fn freeze(").unwrap()..]
        .replace(old, new)
        .replace("zopro_g2_canonical_xsum", "zopro_g2_scaled_r260");
    fs::write(
        "native.rs",
        format!(
            "{}\n{tail}",
            fs::read_to_string("native-prefix.rs").unwrap()
        ),
    )
    .unwrap();
    let mut d = fs::read_to_string(format!("{old}/driver.rs"))
        .unwrap()
        .replace(old, new)
        .replace("zopro_g2_canonical_xsum", "zopro_g2_scaled_r260");
    once(&mut d,"        assert!(t.contains(\"test result: ok. 122 passed; 0 failed\"));","        let count=if name==\"on\"{124}else{122}; assert!(t.contains(&format!(\"test result: ok. {count} passed; 0 failed\")));\n        assert!(fs::read_to_string(r(\"native/field-checked/stdout.log\")).unwrap().contains(\"test result: ok. 33 passed; 0 failed\"));");
    once(
        &mut d,
        "    save(r(\"link-pins.json\"), &json!(pins));",
        r#"    let field=read(r("artifacts-on.json"));
    let flags=field["rustflags"].as_str().unwrap();let mut argv=vec!["rustc".to_string()];argv.extend(flags.split_whitespace().map(str::to_owned));argv.extend(["--edition=2021".into(),"-O".into(),"--test".into(),r("wide-check.rs").display().to_string(),"-o".into(),r("wide-tests").display().to_string(),"-L".into(),"dependency=/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps".into(),"--extern".into(),format!("zopro_bn254={}",field["libraries"]["zopro_bn254"].as_str().unwrap()),"--extern".into(),"num_bigint=/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps/libnum_bigint-158c502562b8bd18.rlib".into()]);
    pin(&mut pins,r("wide-check.rs"));pin(&mut pins,"/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps/libnum_bigint-158c502562b8bd18.rlib");
    for e in fs::read_dir("/home/ubuntu/src/zopro-perf-work/paired-callers/target/release/deps").unwrap(){let p=e.unwrap().path();if p.is_file()&&p.extension().is_some_and(|e|e=="rlib"||e=="so"){pin(&mut pins,p);}}
    steps.push(json!({"kind":"command","name":"build-wide-tests","argv":argv,"timeout_seconds":300}));
    steps.push(json!({"kind":"command","name":"wide-tests","argv":[r("wide-tests"),PathBuf::from("--test-threads=1"),PathBuf::from("--nocapture")],"required_tests":["native_cios_wide_domains_match_full_radix_redc","native_coordinate_codecs_match_host_raw","native_wide_exports_match_modular_division"],"unfiltered":true,"timeout_seconds":300}));
    save(r("link-pins.json"), &json!(pins));"#,
    );
    once(
        &mut d,
        "    pin(&mut pins, X);",
        r#"    let checked=fs::read_to_string(r("linked/wide-tests/stdout.log")).unwrap();assert!(checked.contains("test result: ok. 3 passed; 0 failed"));pin(&mut pins,r("linked/wide-tests/stdout.log"));pin(&mut pins,r("wide-tests"));
    let frozen=hash(r("frozen-probe"));let off=hash(r("off-probe"));save(r("control-preflight.json"),&json!({"frozen_sha256":frozen,"off_sha256":off,"ELF_byte_identical":frozen==off,"original_OFF_body_verbatim":true,"timing_not_started":true}));
    assert_eq!(frozen,off,"Control ELF differs: stop before timing and audit code generation");pin(&mut pins,r("control-preflight.json"));
    pin(&mut pins, X);"#,
    );
    once(&mut d,"\"native_core_tests_each\":122","\"native_core_tests_off\":122,\"native_core_tests_on\":124,\"native_wide_numeric_tests\":3,\"checked_field_tests\":33");
    fs::write("driver.rs", d).unwrap();
    println!("Generatednativefreeze/11linkteststeps/capture;preflight requiresaccepted/freshOFF ELF byteidentity");
}
