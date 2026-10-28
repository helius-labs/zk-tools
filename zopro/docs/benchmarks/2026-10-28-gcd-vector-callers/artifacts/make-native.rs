use serde_json::Value;
use std::fs;
fn main() {
    let p = "/home/ubuntu/src/zopro-perf-work/gcd-vector-rows";
    let mut paired = fs::read_to_string(format!("{p}/paired.rs")).unwrap();
    let split = paired.find("#[cfg(test)]").unwrap();
    let runtime = paired[..split].to_owned();
    paired = paired
        .replace(
            "crate::ordinary::reference_pair",
            "super::super::reference_pair",
        )
        .replace(
            "crate::ordinary::reference_matrix",
            "super::super::reference_matrix",
        );
    assert_eq!(&paired[..split], runtime);
    fs::write("paired.rs", paired).unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/gcd-active-words/probe.rs",
        "probe.rs",
    )
    .unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/gcd-active-words/perf.rs",
        "perf.rs",
    )
    .unwrap();
    fs::copy(
        "/home/ubuntu/src/zopro-perf-work/gcd-active-words/collect.rs",
        "collect.rs",
    )
    .unwrap();
    let mut native =
        fs::read_to_string("/home/ubuntu/src/zopro-perf-work/gcd-active-words/native.rs").unwrap();
    native = native.replace(
        "/zopro-perf-work/gcd-active-words",
        "/zopro-perf-work/gcd-vector-callers",
    );
    let guard = r#"all(zopro_gcd_vector_rows,target_arch="x86_64",target_feature="avx512f",target_feature="avx2",not(no_ifma))"#;
    let new=format!("        #[cfg(not({guard}))]\n        {{a = integer_update(&old_a, &b, &mut f0, &mut g0);\n        b = integer_update(&old_a, &b, &mut f1, &mut g1);}}\n        #[cfg({guard})]\n        {{let mut rows=[f0,g0,f1,g1];let outputs=paired::update(&old_a,&b,&mut rows);\n        a=outputs.0;b=outputs.1;(f0,g0,f1,g1)=(rows[0],rows[1],rows[2],rows[3]);}}");
    let start = native.find("    let new=").unwrap();
    let end = native[start..]
        .find("    assert_eq!(s.matches(old)")
        .unwrap()
        + start;
    native.replace_range(
        start..end,
        &format!("    let new = {};\n", serde_json::to_string(&new).unwrap()),
    );
    let audit: Value =
        serde_json::from_slice(&fs::read(format!("{p}/source-audit.json")).unwrap()).unwrap();
    let suffix=format!("{}\n#[cfg(all(any(test,zopro_gcd_vector_rows),target_arch=\"x86_64\",target_feature=\"avx512f\",target_feature=\"avx2\",not(no_ifma)))]\n#[path=\"inverse_vector_rows.rs\"]\nmod paired;\n",audit["reference_test_suffix"].as_str().unwrap());
    let start = native.find("    let suffix=").unwrap();
    let end = native[start..].find("    fs::write(&p,").unwrap() + start;
    native.replace_range(
        start..end,
        &format!(
            "    let suffix = {};\n",
            serde_json::to_string(&suffix).unwrap()
        ),
    );
    native=native.replace("zopro_gcd_active_words","zopro_gcd_vector_rows").replace("zopro_active_words_caller_probe","zopro_vector_rows_caller_probe").replace("active_words.rs","paired.rs").replace("inverse_active_words.rs","inverse_vector_rows.rs").replace("inverse_paired.rs","inverse_vector_rows.rs").replace("inverse::active_words::tests::shortened_integer_updates_match_unsigned_oracle_and_original","inverse::paired::tests::paired_rows_match_original_and_independent_unsigned_magnitudes").replace("onlyGCDactiveintegerwords","onlypairedSIMD31bitintegerrows");
    fs::write("native.rs", native).unwrap();
    let driver = fs::read_to_string("/home/ubuntu/src/zopro-perf-work/gcd-active-words/driver.rs")
        .unwrap()
        .replace(
            "/zopro-perf-work/gcd-active-words",
            "/zopro-perf-work/gcd-vector-callers",
        )
        .replace("zopro_gcd_active_words", "zopro_gcd_vector_rows")
        .replace(
            "test result: ok. 34 passed; 0 failed",
            "test result: ok. 35 passed; 0 failed",
        )
        .replace(
            "\"native_field_tests_each\":34",
            "\"native_field_tests_each\":35",
        );
    fs::write("driver.rs", driver).unwrap();
    println!("Preparedguardednativepairintegration;runtimehelperexactprototype;testpathsONLYadapted;noactivewordsruntime");
}
