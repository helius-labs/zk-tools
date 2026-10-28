use std::fs;
fn main() {
    let old = "/home/ubuntu/src/zopro-perf-work/g2-reduced-gauss";
    let new = "/home/ubuntu/src/zopro-perf-work/g2-canonical-xsum";
    for f in ["probe.rs", "perf.rs", "collect.rs"] {
        let s = fs::read_to_string(format!("{old}/{f}"))
            .unwrap()
            .replace(old, new);
        fs::write(f, s).unwrap();
    }
    let s = fs::read_to_string(format!("{old}/native.rs")).unwrap();
    let tail = s[s.find("fn freeze(").unwrap()..]
        .replace(old, new)
        .replace("zopro_g2_reduced_gauss", "zopro_g2_canonical_xsum");
    fs::write(
        "native.rs",
        format!(
            "{}\n{tail}",
            fs::read_to_string("native-prefix.rs").unwrap()
        ),
    )
    .unwrap();
    let d = fs::read_to_string(format!("{old}/driver.rs"))
        .unwrap()
        .replace(old, new)
        .replace("zopro_g2_reduced_gauss", "zopro_g2_canonical_xsum")
        .replace("123", "122")
        .replace("        \"oracle.rs\",\n", "");
    fs::write("driver.rs", d).unwrap();
    println!(
        "Generated independent canonicalXsum study;repaired32/200 harness;no Gauss runtime/oracle"
    );
}
