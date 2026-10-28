use std::fs;
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/gcd-vector-callers";
    let new = "/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss";
    for file in ["probe.rs", "perf.rs", "collect.rs"] {
        fs::copy(format!("{old}/{file}"), file).unwrap();
    }
    let native = fs::read_to_string(format!("{old}/native.rs")).unwrap();
    let freeze = &native[native.find("fn freeze(").unwrap()..];
    let freeze = freeze
        .replace(old, new)
        .replace("zopro_gcd_vector_rows", "zopro_g2_reduced_gauss");
    fs::write(
        "native.rs",
        format!(
            "{}\n{freeze}",
            fs::read_to_string("native-prefix.rs").unwrap()
        ),
    )
    .unwrap();
    let mut d = fs::read_to_string(format!("{old}/driver.rs"))
        .unwrap()
        .replace(old, new)
        .replace("zopro_gcd_vector_rows", "zopro_g2_reduced_gauss");
    let start = d
        .find("    for name in [\"checked\", \"off\", \"on\"]")
        .unwrap();
    let end = start
        + d[start..]
            .find("    for name in [\"off\", \"on\"]")
            .unwrap();
    d.replace_range(start..end, "");
    d = d.replace(
        "test result: ok. 122 passed; 0 failed",
        "test result: ok. 123 passed; 0 failed",
    );
    let start = d.find("    let parent = ").unwrap();
    let end = start + d[start..].find("    for (p, h) in &pins").unwrap();
    d.replace_range(start..end,"    for p in [\"make-native.rs\",\"make-native\",\"native-prefix.rs\",\"source-audit.json\",\"oracle.rs\"] {pin(&mut pins,r(p));}\n");
    for line in [
        "        \"make-native-v1.rs\",\n",
        "        \"make-native-v1\",\n",
        "        \"preparation-repair.json\",\n",
        "        \"runtime-provenance.json\",\n",
        "        \"driver-v1.rs\",\n",
        "        \"driver-v1\",\n",
        "        \"link-preparation-repair.json\",\n",
    ] {
        d = d.replace(line, "");
    }
    once(
        &mut d,
        "Prepared9frozen/OFF/ONlink/harnessteststeps;field34x3/core122x2PASS",
        "Prepared9frozen/OFF/ONlink/harnessteststeps;inheritedfieldqualification/core123x2PASS",
    );
    let start = d.find("    let mut code = vec![];").unwrap();
    let end = start + d[start..].find("    let mut topology = ").unwrap();
    let code = r#"    let mut code=vec![];
    for variant in ["off","on"] {
        let binary=r(format!("{variant}-probe"));
        let out=Command::new("nm").arg("--defined-only").arg(&binary).output().unwrap();assert!(out.status.success());
        let symbols=String::from_utf8(out.stdout).unwrap();let mut count=0;
        for(i,line)in symbols.lines().filter(|s|s.contains("g2_prefix")).enumerate(){
            let symbol=line.split_whitespace().last().unwrap();let name=format!("{:#}",rustc_demangle::demangle(symbol));
            let out=Command::new("objdump").args(["-d","--no-show-raw-insn"]).arg(format!("--disassemble={symbol}")).arg(&binary).output().unwrap();assert!(out.status.success());
            let text=String::from_utf8(out.stdout).unwrap();let ifma=text.matches("vpmadd52").count();assert!(ifma>0);count+=1;
            let p=r(format!("codegen-{variant}-{i}.asm.txt"));fs::write(&p,&text).unwrap();pin(&mut pins,&p);
            code.push(json!({"variant":variant,"symbol":name,"ifma_sites":ifma,"instructions":text.lines().filter(|l|l.contains(':')&&l.split_once(':').unwrap().0.trim().chars().all(|c|c.is_ascii_hexdigit())).count(),"path":p}));
        }assert!(count>0);
    }
    save(r("codegen.json"),&json!({"read_only_objdump":true,"actual_G2_prefix_symbols":code,"static_counts_not_executed_work":true}));pin(&mut pins,r("codegen.json"));
"#;
    d.replace_range(start..end, code);
    let start = d.find("fn contexts()").unwrap();
    let end = start + d[start..].find("fn link()").unwrap();
    d.replace_range(start..end,"fn contexts()->Vec<(&'static str,usize)>{vec![(\"g2-prefix\",32),(\"g2-prefix\",200),(\"g1-borrowed-signed\",200)]}\n");
    once(&mut d, "c.rotate_left(b % 12);", "c.rotate_left(b % 9);");
    once(
        &mut d,
        "[(\"g1-borrowed-signed\", 352), (\"g2-prefix\", 200)]",
        "[(\"g1-borrowed-signed\", 200), (\"g2-prefix\", 200)]",
    );
    once(&mut d,"for (kind, n) in [\n                    (\"g1-borrowed-signed\", 200),\n                    (\"g1-borrowed-signed\", 352),\n                    (\"g2-prefix\", 200),\n                ]","for (kind, n) in [(\"g2-prefix\",200)]");
    once(&mut d, "(sum / 3.).exp()", "sum.exp()");
    d = d
        .replace("1124", "845")
        .replace("1116", "837")
        .replace("1136", "857")
        .replace("12contexts31blocks", "9contexts31blocks")
        .replace(
            "\"native_field_tests_each\":35",
            "\"native_field_tests_each\":0",
        )
        .replace(
            "\"native_field_configurations\":3",
            "\"native_field_configurations\":0",
        )
        .replace(
            "\"native_core_tests_each\":122",
            "\"native_core_tests_each\":123",
        )
        .replace(
            "\"independent_proof_count\":13082",
            "\"independent_proof_count\":13181",
        );
    fs::write("driver.rs", d).unwrap();
    println!("Generatednativefreezer/link/capture/analyzers;9contexts837latency+8PMU+12decode;noGCDexperimentruntime");
}
