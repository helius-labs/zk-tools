use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-g1-reference-staging";
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
    let mut b = [0u8; 65536];
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
    let file = "crates/zopro-core/src/msm.rs";
    let original = fs::read_to_string(p.join(file)).unwrap();
    let begin = original.find("fn stream_g1_impl<").unwrap();
    let end = begin + original[begin..].find("\n#[cfg(all(").unwrap();
    let body = &original[begin..end];
    let a="let a: [Affine<C>; LANES] = core::array::from_fn(|i| buckets[batch_index::<SIGNED>(batch[k + i].0)]);";
    let b = "let b: [Affine<C>; LANES] = core::array::from_fn(|i| *batch[k + i].1.borrow());";
    let aa="let a: [&Affine<C>; LANES] = core::array::from_fn(|i| &buckets[batch_index::<SIGNED>(batch[k + i].0)]);";
    let bb = "let b: [&Affine<C>; LANES] = core::array::from_fn(|i| batch[k + i].1.borrow());";
    assert_eq!(body.matches(a).count(), 2);
    assert_eq!(body.matches(b).count(), 2);
    let edited = body.replace(a, aa).replace(b, bb);
    assert_eq!(edited.replace(aa, a).replace(bb, b), body);
    let text = format!("{}{}{}", &original[..begin], edited, &original[end..]);
    let overlay = r.join("overlay").join(file);
    fs::create_dir_all(overlay.parent().unwrap()).unwrap();
    fs::write(&overlay, text.as_bytes()).unwrap();
    save(
        r.join("edits.json"),
        &json!({"file":file,"old_a":a,"new_a":aa,"old_b":b,"new_b":bb,"each_occurrences":2,"before_sha256":hash(p.join(file)),"after_sha256":hash(&overlay),"same_Borrow_calls_per_lane":true,"scope":"stream_g1_impl only"}),
    );
    let mut pins = BTreeMap::<String, String>::new();
    for name in ["fresh", "candidate"] {
        let mut cmd = Command::new(X);
        cmd.arg("prepare").arg(M).arg(P).arg(r.join(name));
        if name == "candidate" {
            cmd.arg(r.join("overlay"));
        }
        let out = cmd.output().unwrap();
        fs::write(r.join(format!("prepare-{name}.log")), &out.stdout).unwrap();
        fs::write(r.join(format!("prepare-{name}.stderr.log")), &out.stderr).unwrap();
        assert!(out.status.success());
        let tree = r.join(name);
        for (f, h) in originals {
            let got = hash(tree.join(f));
            if name == "candidate" && f == file {
                assert_ne!(&got, h.as_str().unwrap());
            } else {
                assert_eq!(got, h.as_str().unwrap());
            }
        }
        for (f, h) in read(tree.join(".xtask/source-pins.json"))
            .as_object()
            .unwrap()
        {
            assert_eq!(hash(f), h.as_str().unwrap());
            pins.insert(f.clone(), h.as_str().unwrap().to_owned());
        }
    }
    let actual = fs::read_to_string(r.join("candidate").join(file)).unwrap();
    let undo = actual.replace(aa, a).replace(bb, b);
    assert_eq!(undo, original);
    save(
        r.join("source-audit.json"),
        &json!({"accepted_originals":171,"fresh_exact_files":171,"candidate_exact_files":170,"candidate_changed_files":[file],"reverse_recovers_all171accepted_bytes":true,"no_new_heap_field_primitive_ISA_flags_unsafe_or_pool":true,"field_source_unchanged":true,"production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("prepare-native"),
        r.join("edits.json"),
        r.join("source-audit.json"),
    ] {
        pins.insert(f.display().to_string(), hash(&f));
    }
    pins.insert(M.into(), hash(M));
    pins.insert(X.into(), hash(X));
    save(r.join("source-pins.json"), &json!(pins));
    let old = read("/tmp/zopro-composed-cpu/native-plan.json");
    let mut steps = Vec::new();
    for name in ["fresh", "candidate"] {
        for oldname in ["core", "prover", "on-libraries"] {
            let mut s = old["steps"]
                .as_array()
                .unwrap()
                .iter()
                .find(|s| s["name"] == oldname)
                .unwrap()
                .clone();
            let label = if oldname == "on-libraries" {
                "libraries"
            } else {
                oldname
            };
            s["name"] = json!(format!("{name}-{label}"));
            for v in s["argv"].as_array_mut().unwrap() {
                if let Some(x) = v.as_str() {
                    *v = json!(x.replace(
                        "/tmp/zopro-composed-cpu/candidate",
                        &r.join(name).display().to_string()
                    ));
                }
            }
            s["environment"]["CARGO_TARGET_DIR"] = json!(r.join(format!("target-{name}")));
            assert_eq!(s["environment"]["RUSTFLAGS"], m["rustflags"]);
            steps.push(s);
        }
        steps.push(json!({"kind":"link-sdk","name":name,"artifacts":r.join(format!("native/{name}-libraries/stdout.log")),"driver":r.join(name).join("crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[r.join(name).join("crates/zopro-bench/examples/support/perf.rs")]}));
    }
    for oldname in ["portable-consumers", "no-ifma-consumers"] {
        let mut s = old["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == oldname)
            .unwrap()
            .clone();
        for v in s["argv"].as_array_mut().unwrap() {
            if let Some(x) = v.as_str() {
                *v = json!(x.replace(
                    "/tmp/zopro-composed-cpu/candidate",
                    &r.join("candidate").display().to_string()
                ));
            }
        }
        s["environment"]["CARGO_TARGET_DIR"] = json!(r.join(format!("target-{oldname}")));
        steps.push(s);
    }
    save(
        r.join("native-plan.json"),
        &json!({"version":1,"source_pins":r.join("source-pins.json"),"environment":old["environment"],"steps":steps}),
    );
    println!("Prepared exact171acceptedfresh/170candidate+oneedit;native116/raw2/cubic6 each; no field source/primitive change; no proof capture yet");
}
