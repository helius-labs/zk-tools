use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-g2-scalar-grain";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const M: &str = "/tmp/zopro-composed-adoption/production-source-manifest.json";
const X: &str = "/tmp/zopro-xtask-fast-screen-target/release/xtask";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    let mut f = fs::File::open(p).unwrap();
    let mut h = Sha256::new();
    let mut b = [0; 65536];
    loop {
        let n = f.read(&mut b).unwrap();
        if n == 0 {
            break;
        }
        h.update(&b[..n]);
    }
    format!("{:x}", h.finalize())
}
fn main() {
    let r = Path::new(R);
    assert!(!r.exists());
    fs::create_dir(r).unwrap();
    fs::copy("/tmp/zopro-g2-scalar-grain-design.md", r.join("design.md")).unwrap();
    fs::copy(
        "/tmp/zopro-g2-scalar-grain-prepare-native.rs",
        r.join("prepare-native.rs"),
    )
    .unwrap();
    let m = read(M);
    assert_eq!(m["adopted"], true);
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 171);
    for (f, h) in originals {
        assert_eq!(hash(Path::new(P).join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    let file = "crates/zopro-core/src/msm_affine/g2_phi.rs";
    let before = "    short.par_iter_mut().for_each(|s| *s = phi_split(*s));";
    let after = "    short.par_iter_mut().with_min_len(128).for_each(|s| *s = phi_split(*s));";
    let old = fs::read_to_string(Path::new(P).join(file)).unwrap();
    assert_eq!(old.matches(before).count(), 1);
    let new = old.replace(before, after);
    assert_eq!(new.replace(after, before), old);
    let out = r.join("overlay").join(file);
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::write(out, new).unwrap();
    save(
        r.join("edits.json"),
        &json!({"file":file,"before":before,"after":after,"inverse_recovers_original":true,"production_changed":false}),
    );
    let mut pins = BTreeMap::<String, String>::new();
    for name in ["fresh", "candidate"] {
        let mut c = Command::new(X);
        c.arg("prepare").arg(M).arg(P).arg(r.join(name));
        if name == "candidate" {
            c.arg(r.join("overlay"));
        }
        let o = c.output().unwrap();
        fs::write(r.join(format!("prepare-{name}.log")), o.stdout).unwrap();
        fs::write(r.join(format!("prepare-{name}.stderr.log")), o.stderr).unwrap();
        assert!(o.status.success());
        for (f, h) in originals {
            let got = hash(r.join(name).join(f));
            if name == "candidate" && f == file {
                assert_ne!(got, h.as_str().unwrap());
            } else {
                assert_eq!(got, h.as_str().unwrap());
            }
        }
        for (f, h) in read(r.join(name).join(".xtask/source-pins.json"))
            .as_object()
            .unwrap()
        {
            assert_eq!(hash(f), h.as_str().unwrap());
            pins.insert(f.clone(), h.as_str().unwrap().into());
        }
    }
    save(
        r.join("source-audit.json"),
        &json!({"accepted_source_files":171,"fresh_exact_files":171,"candidate_exact_files":170,"candidate_changed_files":[file],"inverse_recovers_original":true,"no_new_unsafe_or_flag":true,"no_new_buffer_or_arithmetic":true,"production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("edits.json"),
        r.join("source-audit.json"),
        Path::new(M).into(),
        Path::new(X).into(),
    ] {
        pins.insert(f.display().to_string(), hash(&f));
    }
    for (field, sha) in [
        ("standard_cli_source", "standard_cli_source_sha256"),
        (
            "standard_cli_support_source",
            "standard_cli_support_source_sha256",
        ),
    ] {
        let f = m[field].as_str().unwrap();
        assert_eq!(hash(f), m[sha].as_str().unwrap());
        pins.insert(f.into(), hash(f));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let text = fs::read_to_string("/tmp/zopro-grouping-prefix-cutoff/native-plan-v2.json")
        .unwrap()
        .replace("/tmp/zopro-grouping-prefix-cutoff", R)
        .replace("/native-v2/", "/native/")
        .replace("source-pins-v2.json", "source-pins.json");
    let mut plan: Value = serde_json::from_str(&text).unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["kind"] == "command" {
            if let Some(tests) = step
                .get_mut("required_tests")
                .and_then(|v| v.as_array_mut())
            {
                tests.retain(|t| {
                    t!="msm_affine::grouping::tests::prefix_cutoff_matches_independent_ordered_map"
                });
            }
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Prepared171fresh/170unchangedcandidate;sole phi_split iterator minimum128; native workflow ready");
}
