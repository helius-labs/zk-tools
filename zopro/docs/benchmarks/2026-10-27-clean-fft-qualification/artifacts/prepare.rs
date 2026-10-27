use std::{fs, path::Path};
fn main() {
    let r = Path::new("/tmp/zopro-clean-fft-qualification");
    let old = "/tmp/zopro-composed-qualification";
    for name in ["concurrent.rs", "warm.rs", "forester.rs"] {
        fs::copy(Path::new(old).join(name), r.join(name)).unwrap();
    }
    let mut build = fs::read_to_string(Path::new(old).join("build.rs"))
        .unwrap()
        .replace(old, r.to_str().unwrap());
    build = build.replace(
        "/tmp/zopro-composed-cpu/perf-plan.json",
        "/tmp/zopro-clean-fft-leaves/perf-plan.json",
    );
    build = build
        .replace(
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
            "/tmp/zopro-clean-fft-leaves/native/candidate/artifacts.json",
        )
        .replace(
            "/tmp/zopro-composed-cpu/native/on-libraries/stdout.log",
            "/tmp/zopro-clean-fft-leaves/native/candidate-libraries/stdout.log",
        );
    build = build
        .replace(
            "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
        )
        .replace(
            "/tmp/zopro-grouping-module/native/libraries/stdout.log",
            "/tmp/zopro-composed-cpu/native/on-libraries/stdout.log",
        );
    build = build.replace(
        "/tmp/zopro-composed-cpu/candidate/",
        "/tmp/zopro-clean-fft-leaves/candidate/",
    );
    fs::write(r.join("build.rs"), build).unwrap();
    let resource = fs::read_to_string(Path::new(old).join("resources.rs"))
        .unwrap()
        .replace(old, r.to_str().unwrap())
        .replace("zopro-composed-q-", "zopro-cleafft-q-");
    fs::write(r.join("resources.rs"), resource).unwrap();
    let mut qualify = fs::read_to_string(Path::new(old).join("qualify.rs"))
        .unwrap()
        .replace(old, r.to_str().unwrap());
    qualify = qualify
        .replace(
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
            "/tmp/zopro-clean-fft-leaves/native/candidate/artifacts.json",
        )
        .replace(
            "/tmp/zopro-grouping-module/native/candidate/artifacts.json",
            "/tmp/zopro-composed-cpu/native/composed/artifacts.json",
        );
    let begin = qualify
        .find("    let old = read(\"/tmp/zopro-composed-cpu/perf-plan.json\");")
        .unwrap();
    let end = begin
        + qualify[begin..]
            .find("    let base = old[\"environment\"].clone();")
            .unwrap();
    let prior = r#"    let old = read("/tmp/zopro-clean-fft-leaves/perf-plan.json");
    let mut seen: BTreeSet<String> = serde_json::from_value(old["prior_proof_sha256"].clone()).unwrap();
    assert_eq!(seen.len(),11119);let mut repeated=0;
    for(file,want)in read("/tmp/zopro-clean-fft-leaves/proof-inventory.json").as_object().unwrap(){assert_eq!(hash(file),want.as_str().unwrap());if file.ends_with(".proof"){assert_eq!(fs::metadata(file).unwrap().len(),164);if !seen.insert(want.as_str().unwrap().into()){repeated+=1;}}}
    assert_eq!(repeated,9);assert_eq!(seen.len(),11575);
"#;
    qualify.replace_range(begin..end, prior);
    qualify = qualify.replace("9069", "11575").replace("9154", "11660");
    fs::write(r.join("qualify.rs"), qualify).unwrap();
    println!("Prepared exactnative/portable qualification sources;accepted824/clean14da;prior11575->11660;uniqueownedunitprefix");
}
