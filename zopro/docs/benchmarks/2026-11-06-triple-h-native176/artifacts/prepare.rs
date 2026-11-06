use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-native176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const BASE: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175";
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn main() {
    assert!(!Path::new(R).join("plan.json").exists());
    let mut pins = BTreeMap::new();
    let manifest=format!("{Z}/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let mut source = BTreeMap::new();
    for (p, h) in read(&manifest)["source_files_sha256"].as_object().unwrap() {
        let from = Path::new(Z).join(p);
        assert_eq!(hash(&from), h.as_str().unwrap());
        let to = Path::new(R).join("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(&from, &to).unwrap();
        source.insert(p.clone(), h.as_str().unwrap().to_owned());
        pins.insert(from.display().to_string(), hash(from));
    }
    assert_eq!(source.len(), 176);
    let old = "/home/ubuntu/src/zopro-perf-work/unweighted-triple-caller174/reduction_probe.rs";
    let oldtext = fs::read_to_string(old).unwrap();
    let tail = oldtext.split_once("struct Work {").unwrap().1;
    let tail = tail.split_once("#[cfg(test)]").unwrap().0;
    let module = format!(
        "{}\nstruct Work {{{tail}\n{}",
        fs::read_to_string(format!("{R}/prefix.rs")).unwrap(),
        fs::read_to_string(format!("{R}/tests.rs")).unwrap()
    );
    let target = format!("{R}/source/crates/zopro-core/src/msm/h_only_probe.rs");
    fs::create_dir_all(Path::new(&target).parent().unwrap()).unwrap();
    fs::write(target, module).unwrap();
    let checked="/home/ubuntu/src/zopro-perf-work/deferred-equality176/candidate-fixed/source/crates/zopro-core/src/simd/checked_inverse.rs";
    let to = format!("{R}/source/crates/zopro-core/src/simd/h_checked.rs");
    fs::create_dir_all(Path::new(&to).parent().unwrap()).unwrap();
    fs::copy(checked, &to).unwrap();
    for (file, extra) in [
        (
            "msm.rs",
            "\n#[cfg(zopro_h_only_probe)]\npub mod h_only_probe;\n",
        ),
        ("simd.rs", "\n#[cfg(zopro_h_only_probe)]\nmod h_checked;\n"),
        (
            "lib.rs",
            "\n#[cfg(zopro_h_only_probe)]\npub use msm::h_only_probe::ReductionCaller;\n",
        ),
    ] {
        let p = format!("{R}/source/crates/zopro-core/src/{file}");
        let mut text = fs::read_to_string(&p).unwrap();
        text.push_str(extra);
        fs::write(p, text).unwrap();
    }
    let driver = "/home/ubuntu/src/zopro-perf-work/unweighted-triple-caller174/probe.rs";
    fs::copy(driver, format!("{R}/probe.rs")).unwrap();
    fs::create_dir_all(format!("{R}/libs")).unwrap();
    let frozen = read(format!("{BASE}/frozen-candidate.json"));
    for (p, h) in frozen["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().to_owned());
    }
    let field = frozen["libraries"]["zopro_bn254"].as_str().unwrap();
    assert!(field.contains("1200e63547baa41c299a48e3221f1cef85990b0369f17452dc12eb17045f9713"));
    let previous="/home/ubuntu/src/zopro-perf-work/word-export176/caller-native-repair/core-OFF/command.json";
    let receipt = read(previous);
    let mut core: Vec<String> = receipt["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|s| s.as_str().unwrap().to_owned())
        .collect();
    for a in &mut core {
        if a=="/home/ubuntu/src/zopro-perf-work/word-export176/caller-source/crates/zopro-core/src/lib.rs"{*a=format!("{R}/source/crates/zopro-core/src/lib.rs");}
        else if a.starts_with("zopro_bn254="){*a=format!("zopro_bn254={field}");}
        else if a=="dependency=/home/ubuntu/src/zopro-perf-work/word-export176/frozen"{*a=format!("dependency={R}/libs");}
        else if a=="/home/ubuntu/src/zopro-perf-work/word-export176/caller-libs/OFF/libzopro_core.rlib"{*a=format!("{R}/libs/libzopro_core.rlib");}
    }
    core.extend(["--cfg".into(), "zopro_h_only_probe".into()]);
    let mut env = receipt["environment"].as_object().unwrap().clone();
    for (k, v) in [
        ("CARGO_PKG_NAME", "zopro-core"),
        ("CARGO_PKG_VERSION", "0.1.0"),
        (
            "CARGO_MANIFEST_DIR",
            &format!("{R}/source/crates/zopro-core"),
        ),
    ] {
        env.insert(k.into(), json!(v));
    }
    let mut test = core.clone();
    let p = test.iter().position(|s| s == "--crate-type").unwrap();
    test.drain(p..p + 2);
    let p = test.iter().position(|s| s == "-o").unwrap();
    test[p + 1] = format!("{R}/core-tests");
    test.extend(["--test".into(), "-C".into(), "lto=fat".into()]);
    let deps = "/home/ubuntu/src/zopro-perf-work/word-export176/target-candidate/release/deps";
    let tool = "/home/ubuntu/src/zopro-perf-work/tool-target/release/deps";
    let mut consumer = vec![
        core[0].clone(),
        "--edition=2021".into(),
        "-O".into(),
        "-C".into(),
        "target-cpu=native".into(),
        "-C".into(),
        "lto=fat".into(),
        "-C".into(),
        "codegen-units=1".into(),
        "-C".into(),
        "debuginfo=1".into(),
        "-L".into(),
        format!("dependency={BASE}/frozen/candidate"),
        "-L".into(),
        format!("dependency={R}/libs"),
        "-L".into(),
        format!("dependency={deps}"),
        "-L".into(),
        format!("dependency={tool}"),
    ];
    consumer.extend([
        "--extern".into(),
        format!("zopro_core={R}/libs/libzopro_core.rlib"),
    ]);
    for name in ["rayon", "serde_json"] {
        let lib = frozen["libraries"][name].as_str().unwrap();
        consumer.extend(["--extern".into(), format!("{name}={lib}")]);
    }
    for (name, file) in [
        ("ark_bn254", "libark_bn254-d28868a71f06f5b5.rlib"),
        ("ark_ec", "libark_ec-59ebd5b49f9b3011.rlib"),
        ("ark_ff", "libark_ff-3a8e68dc8045d2bd.rlib"),
    ] {
        consumer.extend(["--extern".into(), format!("{name}={deps}/{file}")]);
        pins.insert(format!("{deps}/{file}"), hash(format!("{deps}/{file}")));
    }
    let mut oracle = consumer.clone();
    oracle.extend([
        "--test".into(),
        format!("{R}/probe.rs"),
        "-o".into(),
        format!("{R}/oracle-tests"),
    ]);
    consumer.extend([format!("{R}/probe.rs"), "-o".into(), format!("{R}/caller")]);
    for p in [old, checked, driver, previous, &manifest] {
        pins.insert(p.to_owned(), hash(p));
    }
    for entry in fs::read_dir(deps).unwrap() {
        let p = entry.unwrap().path();
        if p.extension().is_some_and(|s| s == "rlib" || s == "so") {
            pins.insert(p.display().to_string(), hash(p));
        }
    }
    fn walk(dir: &Path, out: &mut Vec<PathBuf>) {
        for e in fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out)
            } else {
                out.push(p)
            }
        }
    }
    let mut files = Vec::new();
    walk(&Path::new(R).join("source"), &mut files);
    let mut actual = BTreeMap::new();
    for p in files {
        let h = hash(&p);
        actual.insert(
            p.strip_prefix(Path::new(R).join("source"))
                .unwrap()
                .display()
                .to_string(),
            h.clone(),
        );
        pins.insert(p.display().to_string(), h);
    }
    assert_eq!(actual.len(), 178);
    for p in [
        "design.md",
        "prepare.rs",
        "prepare",
        "prefix.rs",
        "tests.rs",
        "probe.rs",
    ] {
        let path = format!("{R}/{p}");
        pins.insert(path.clone(), hash(path));
    }
    save(
        format!("{R}/source-manifest.json"),
        &json!({"accepted":source,"candidate":actual,"accepted_count":176,"candidate_count":178,"originals_modified_only_by_appendices":["crates/zopro-core/src/lib.rs","crates/zopro-core/src/msm.rs","crates/zopro-core/src/simd.rs"],"field_reused":field}),
    );
    save(format!("{R}/pins.json"), &json!(pins));
    let steps = json!([
        {"kind":"command","name":"build-core","argv":core,"environment":env,"timeout_seconds":900},
        {"kind":"command","name":"build-core-tests","argv":test,"environment":env,"timeout_seconds":900},
        {"kind":"command","name":"native-focused-tests","argv":[format!("{R}/core-tests"),"msm::h_only_probe::h_only_tests::","--test-threads=1","--nocapture"],"environment":env,"timeout_seconds":600,"unfiltered":false,"required_tests":["msm::h_only_probe::h_only_tests::every_partial_lane_and_remainder_matches_complete_one_layer","msm::h_only_probe::h_only_tests::checked_inverse_rejects_all_255_zero_masks","msm::h_only_probe::h_only_tests::exceptional_lanes_guard_before_writes_and_opposites_are_admitted","msm::h_only_probe::h_only_tests::malformed_geometry_rejected_before_mutation","msm::h_only_probe::h_only_tests::reused_complete_scratch_on_small_stack_workers"]},
        {"kind":"command","name":"build-independent-oracle","argv":oracle,"environment":env,"timeout_seconds":900},
        {"kind":"command","name":"independent-oracle","argv":[format!("{R}/oracle-tests"),"--test-threads=1","--nocapture"],"environment":env,"timeout_seconds":600},
        {"kind":"command","name":"build-final-caller","argv":consumer,"environment":env,"timeout_seconds":900}
    ]);
    save(
        format!("{R}/plan.json"),
        &json!({"version":1,"source_pins":format!("{R}/pins.json"),"steps":steps}),
    );
    println!("Frozen178-source snapshot / accepted field+support /6 commands /5new native tests +1independent Arkworks / final O3nativefatLTOoneCGU; NOT timing");
}
