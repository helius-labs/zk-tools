use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, io::Read, path::Path, process::Command};
const R: &str = "/tmp/zopro-g1-raw-scratch";
const P: &str = "/home/ubuntu/src/zk-tools/zopro";
const M: &str = "/tmp/zopro-clean-fft-adoption/production-source-manifest.json";
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
fn replace(s: &mut String, before: &str, after: &str) {
    assert_eq!(s.matches(before).count(), 1, "{before}");
    *s = s.replacen(before, after, 1);
}
fn main() {
    let r = Path::new(R);
    let p = Path::new(P);
    assert!(!r.join("native-plan.json").exists());
    let m = read(M);
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 172);
    for (f, h) in originals {
        assert_eq!(hash(p.join(f)), h.as_str().unwrap());
    }
    assert_eq!(
        hash(m["native_binary"].as_str().unwrap()),
        m["native_binary_sha256"].as_str().unwrap()
    );
    let mut edits = Vec::new();
    let file = "crates/zopro-core/src/msm.rs";
    let old = fs::read_to_string(p.join(file)).unwrap();
    let mut text = old.clone();
    let begin = text.find("struct FqAddGroup {").unwrap();
    let end = begin + text[begin..].find("\n}").unwrap() + 2;
    let before = text[begin..end].to_owned();
    let after = before.replace(": FqBatch", ": g1_storage::Stored");
    replace(&mut text, &before, &after);
    let begin = text.find("fn stream_g1_impl<").unwrap();
    let end = begin
        + text[begin..]
            .find("\n#[derive(Clone, Copy)]\nstruct Fq2AddGroup")
            .unwrap();
    let body = text[begin..end].to_owned();
    let mut next = body.clone();
    for name in ["ax", "ay", "numerator", "denominator"] {
        let a = format!("addr_of_mut!((*slot).{name}).write({name});");
        let b = format!("addr_of_mut!((*slot).{name}).write(g1_storage::Stored::new({name}));");
        replace(&mut next, &a, &b);
    }
    replace(&mut next,"addr_of_mut!((*slot).xsum).write(canonical_add::<FAST>(&ax, &bx));","addr_of_mut!((*slot).xsum).write(g1_storage::Stored::new(canonical_add::<FAST>(&ax, &bx)));");
    replace(&mut next,"                ax,\n                ay,\n                xsum: canonical_add::<FAST>(&ax, &bx),\n                denominator,\n                numerator,","                ax: g1_storage::Stored::new(ax),\n                ay: g1_storage::Stored::new(ay),\n                xsum: g1_storage::Stored::new(canonical_add::<FAST>(&ax, &bx)),\n                denominator: g1_storage::Stored::new(denominator),\n                numerator: g1_storage::Stored::new(numerator),");
    replace(
        &mut next,
        "g.numerator.mul(&inverse)",
        "g.numerator.expand().mul(&inverse)",
    );
    replace(
        &mut next,
        "inverse.mul(&g.denominator)",
        "inverse.mul(&g.denominator.expand())",
    );
    replace(
        &mut next,
        "slope.mul(&slope).sub(&g.xsum)",
        "slope.mul(&slope).sub(&g.xsum.expand())",
    );
    replace(
        &mut next,
        "slope.mul(&g.ax.sub(&x)).sub(&g.ay)",
        "slope.mul(&g.ax.expand().sub(&x)).sub(&g.ay.expand())",
    );
    replace(&mut text, &body, &next);
    replace(
        &mut text,
        "v::<FqAddGroup>(groups.checked_mul(2)?, false)?",
        "v::<[FqBatch; 5]>(groups.checked_mul(2)?, false)?",
    );
    text.push_str("\nmod g1_storage;\n");
    let out = r.join("overlay").join(file);
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::write(&out, &text).unwrap();
    edits.push(json!({"file":file,"before":old,"after":text}));
    let file = "crates/zopro-core/src/simd.rs";
    let old = fs::read_to_string(p.join(file)).unwrap();
    let mut text = old.clone();
    let before = "impl FqBatch {";
    let after="impl FqBatch {\n    #[inline(always)]\n    pub(crate) fn compact_storage(&self) -> zopro_bn254::ifma8::PackedIfma8<FqConfig, true> { self.0.compact() }\n    #[inline(always)]\n    pub(crate) fn expand_storage(value: &zopro_bn254::ifma8::PackedIfma8<FqConfig, true>) -> Self { Self(value.expand()) }";
    replace(&mut text, before, after);
    fs::write(r.join("overlay").join(file), &text).unwrap();
    edits.push(json!({"file":file,"before":old,"after":text}));
    let added = "crates/zopro-core/src/msm/g1_storage.rs";
    let out = r.join("overlay").join(added);
    fs::create_dir_all(out.parent().unwrap()).unwrap();
    fs::copy(r.join("storage.rs"), &out).unwrap();
    save(
        r.join("edits.json"),
        &json!({"edits":edits,"added_file":added,"added_sha256":hash(&out),"inverse_recovers_original":true,"workspace_forecast_unchanged":true}),
    );
    let mut pins = BTreeMap::new();
    for name in ["fresh", "candidate"] {
        let mut cmd = Command::new(X);
        cmd.arg("prepare").arg(M).arg(P).arg(r.join(name));
        if name == "candidate" {
            cmd.arg(r.join("overlay"));
        }
        let output = cmd.output().unwrap();
        fs::write(r.join(format!("prepare-{name}.log")), output.stdout).unwrap();
        fs::write(r.join(format!("prepare-{name}.stderr.log")), output.stderr).unwrap();
        assert!(output.status.success());
        for (f, h) in originals {
            let got = hash(r.join(name).join(f));
            if name == "candidate"
                && [
                    "crates/zopro-core/src/msm.rs",
                    "crates/zopro-core/src/simd.rs",
                ]
                .contains(&f.as_str())
            {
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
            pins.insert(f.clone(), h.as_str().unwrap().to_owned());
        }
    }
    for e in &edits {
        assert_eq!(
            fs::read_to_string(p.join(e["file"].as_str().unwrap())).unwrap(),
            e["before"].as_str().unwrap()
        );
    }
    save(
        r.join("source-audit.json"),
        &json!({"accepted_source_files":172,"fresh_exact_files":172,"candidate_exact_originals":170,"candidate_changed_files":["crates/zopro-core/src/msm.rs","crates/zopro-core/src/simd.rs"],"added_file":added,"candidate_source_files":173,"inverse_recovers_original":true,"no_new_unsafe_flag_buffer_or_key_table":true,"workspace_forecast_unchanged":true,"production_changed":false}),
    );
    for f in [
        r.join("design.md"),
        r.join("prepare-native.rs"),
        r.join("storage.rs"),
        r.join("edits.json"),
        r.join("source-audit.json"),
        Path::new(M).to_owned(),
        Path::new(X).to_owned(),
    ] {
        pins.insert(f.display().to_string(), hash(f));
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
    let raw = fs::read_to_string("/tmp/zopro-clean-fft-leaves/native-plan.json")
        .unwrap()
        .replace("/tmp/zopro-clean-fft-leaves", R);
    let mut plan: Value = serde_json::from_str(&raw).unwrap();
    for s in plan["steps"].as_array_mut().unwrap() {
        if s["name"] == "candidate-core" {
            for test in [
                "stored_values_match_scalar_arithmetic_across_lazy_chains",
                "record_size_and_conservative_forecast_contract",
            ] {
                s["required_tests"]
                    .as_array_mut()
                    .unwrap()
                    .push(json!(format!("msm::g1_storage::tests::{test}")));
            }
        }
    }
    save(r.join("native-plan.json"), &plan);
    println!("Prepared exact172fresh / candidate173; two changed originals and one private storage module; old forecast retained");
}
