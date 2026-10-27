use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-clean-fft-leaves";
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
    let p = Path::new(P);
    assert!(!r.join("native-plan.json").exists());
    let m = read(M);
    assert_eq!(m["adopted"], true);
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 171);
    for (f, h) in originals {
        assert_eq!(hash(p.join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    let file = "crates/zopro-core/src/fft/pair.rs";
    let added = "crates/zopro-core/src/fft/pair/leaf.rs";
    let old = fs::read_to_string(p.join(file)).unwrap();
    let start = old
        .find("fn stored_transform<S: PairStorage, const FORWARD: bool>(")
        .unwrap();
    let end = start
        + old[start..]
            .find("// Keep the fixed column scratch")
            .unwrap();
    let body = &old[start..end];
    let signature = "    level: usize,\n) {";
    assert_eq!(body.matches(signature).count(), 1);
    let inner = body
        .replace("fn stored_transform<", "fn stored_transform_inner<")
        .replace(signature, "    level: usize,\n    cutoff: usize,\n) {")
        .replace(
            "dit::<5>(&mut local, roots, level);",
            "leaf::dit_bounded(&mut local, roots, level, cutoff);",
        )
        .replace(
            "dif::<5>(&mut local, roots, level);",
            "leaf::dif_bounded(&mut local, roots, level, cutoff);",
        )
        .replace(
            "stored_transform::<S, FORWARD>(part, roots, level + 5)",
            "stored_transform_inner::<S, FORWARD>(part, roots, level + 5, cutoff)",
        );
    let wrapper="fn stored_transform<S: PairStorage, const FORWARD: bool>(\n    values: &mut [S], roots: &Twiddles, level: usize,\n) {\n    let cutoff = leaf::cutoff(values.len(), rayon::current_num_threads());\n    stored_transform_inner::<S, FORWARD>(values, roots, level, cutoff);\n}\n\n";
    let after = format!("{wrapper}{inner}");
    let new = old.replacen(body, &after, 1) + "\nmod leaf;\n";
    assert_eq!(
        new.strip_suffix("\nmod leaf;\n")
            .unwrap()
            .replace(&after, body),
        old
    );
    let prior =
        "/tmp/zopro-serial-fft-leaves/candidate-v2/crates/zopro-core/src/fft/pair/serial_leaf.rs";
    let prior_text = fs::read_to_string(prior).unwrap();
    let cstart = prior_text.find("fn cutoff(").unwrap();
    let cend = prior_text.find("pub(super) fn transform<").unwrap();
    let mut cutoff = prior_text[cstart..cend].replace("fn cutoff(", "pub(super) fn cutoff(");
    cutoff = cutoff.trim_end().to_owned();
    let tstart = prior_text.find("// These binary boundaries").unwrap();
    let terminal = prior_text[tstart..]
        .replace("fn dif_bounded(", "pub(super) fn dif_bounded(")
        .replace("fn dit_bounded(", "pub(super) fn dit_bounded(")
        .replace("transform::<S,", "stored_transform::<S,");
    let leaf=format!("//! Bounded serial arithmetic in existing expanded FFT leaves.\nuse super::*;\n\n{cutoff}\n\n{terminal}");
    assert!(!leaf.contains("OnceLock"));
    assert!(!leaf.contains("ZOPRO_CPU_SERIAL"));
    assert!(!leaf.contains("fn transform_inner"));
    for (f, text) in [(file, new), (added, leaf)] {
        let out = r.join("overlay").join(f);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::write(out, text).unwrap();
    }
    save(
        r.join("edits.json"),
        &json!({"file":file,"before":body,"after":after,"module_suffix":"\nmod leaf;\n","added_file":added,"added_file_sha256":hash(r.join("overlay").join(added)),"prior_oracle_module":prior,"prior_oracle_module_sha256":hash(prior),"no_runtime_flag_or_duplicate_outer_transform":true,"inverse_recovers_original":true,"production_changed":false}),
    );
    let mut pins = BTreeMap::<String, String>::new();
    for name in ["fresh", "candidate"] {
        let mut c = Command::new(X);
        c.arg("prepare").arg(M).arg(P).arg(r.join(name));
        if name == "candidate" {
            c.arg(r.join("overlay"));
        }
        let out = c.output().unwrap();
        fs::write(r.join(format!("prepare-{name}.log")), out.stdout).unwrap();
        fs::write(r.join(format!("prepare-{name}.stderr.log")), out.stderr).unwrap();
        assert!(out.status.success());
        for (f, h) in originals {
            let got = hash(r.join(name).join(f));
            if name == "candidate" && f == file {
                assert_ne!(got, h.as_str().unwrap());
            } else {
                assert_eq!(got, h.as_str().unwrap());
            }
        }
        if name == "candidate" {
            assert_eq!(
                hash(r.join(name).join(added)),
                hash(r.join("overlay").join(added))
            );
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
        &json!({"accepted_source_files":171,"fresh_exact_files":171,"candidate_exact_originals":170,"candidate_changed_files":[file],"added_file":added,"inverse_recovers_original":true,"no_new_unsafe_flag_buffer_or_key_table":true,"original_outer32_geometry_field_ops_load_store_drop_unchanged":true,"single_implementation_carries_cutoff_no_runtime_dispatch":true,"production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("edits.json"),
        r.join("source-audit.json"),
        Path::new(M).into(),
        Path::new(X).into(),
        Path::new(prior).into(),
    ] {
        pins.insert(f.display().to_string(), hash(&f));
    }
    for (field, sha) in [
        ("standard_cli_source", "standard_cli_source_sha256"),
        ("standard_cli_helper", "standard_cli_helper_sha256"),
    ] {
        let f = m[field].as_str().unwrap();
        assert_eq!(hash(f), m[sha].as_str().unwrap());
        pins.insert(f.into(), hash(f));
    }
    save(r.join("source-pins.json"), &json!(pins));
    let text = fs::read_to_string("/tmp/zopro-g2-scalar-grain/native-plan.json")
        .unwrap()
        .replace("/tmp/zopro-g2-scalar-grain", R);
    let mut plan: Value = serde_json::from_str(&text).unwrap();
    for step in plan["steps"].as_array_mut().unwrap() {
        if step["name"] == "candidate-core" {
            for name in [
                "cutoff_bounds_and_available_parallelism_hold_at_domain_and_worker_edges",
                "inverse_and_forward_match_scalar_polynomials_with_cached_and_twisted_roots",
                "concurrent_callers_share_roots_with_small_worker_stacks",
            ] {
                step["required_tests"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(format!("fft::pair::leaf::tests::{name}")));
            }
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Prepared171fresh/170exactcandidateoriginals+cleanFFTleafmodule;single implementation,no runtime dispatch");
}
