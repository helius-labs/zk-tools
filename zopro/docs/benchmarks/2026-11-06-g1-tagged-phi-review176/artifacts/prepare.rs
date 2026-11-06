use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-tagged-phi-review176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const T: &str = "/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/rustc";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn save(p: &str, v: &Value) {
    fs::write(r(p), serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn function(s: &str, needle: &str) -> String {
    let a = s.find(needle).unwrap();
    let b = a + s[a..].find('{').unwrap();
    let mut level = 1;
    let mut e = b + 1;
    while level > 0 {
        match s.as_bytes()[e] {
            b'{' => level += 1,
            b'}' => level -= 1,
            _ => {}
        }
        e += 1;
    }
    s[a..e].to_owned()
}
fn main() {
    assert!(!r("plan.json").exists());
    assert!(!r("source").exists());
    let manifest=Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json");
    let m = read(&manifest);
    let infofile =
        "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen-candidate.json";
    let info = read(infofile);
    let mut pins = BTreeMap::<String, String>::new();
    for (p, h) in info["input_pins"].as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
        pins.insert(p.clone(), h.as_str().unwrap().into());
    }
    let suffixes=BTreeMap::from([("crates/zopro-core/src/lib.rs","\ninclude!(\"tagged_phi_diagnostic.rs\");\n"),("crates/zopro-core/src/msm_affine.rs","\npub(crate) fn tagged_phi_split_diagnostic(s:[u64;4])->[u64;4]{g2_phi::phi_split(s)}\n")]);
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
        let orig = Path::new(Z).join(p);
        assert_eq!(hash(&orig), h.as_str().unwrap());
        pins.insert(orig.display().to_string(), h.as_str().unwrap().into());
        let mut b = fs::read(orig).unwrap();
        if let Some(suffix) = suffixes.get(p.as_str()) {
            b.extend_from_slice(suffix.as_bytes());
        }
        let out = r("source").join(p);
        fs::create_dir_all(out.parent().unwrap()).unwrap();
        fs::write(&out, &b).unwrap();
        if let Some(suffix) = suffixes.get(p.as_str()) {
            b.truncate(b.len() - suffix.len());
        }
        assert_eq!(format!("{:x}", Sha256::digest(b)), h.as_str().unwrap());
        pins.insert(out.display().to_string(), hash(out));
    }
    let child = r("source/crates/zopro-core/src/tagged_phi_diagnostic.rs");
    fs::copy(r("diagnostic.rs"), &child).unwrap();
    pins.insert(child.display().to_string(), hash(child));
    save(
        "source-audit.json",
        &json!({"source_files":177,"originals_exact":174,"all176recover":true,"suffixes":suffixes,"original_function_bodies_and_all_field_files_exact":true,"no_runtime_candidate":true,"original_field_library_reused":info["libraries"]["zopro_bn254"]}),
    );
    let casesfile = "/home/ubuntu/src/zopro-perf-work/g2-queue-scan-count175/cases.json";
    let cases = read(casesfile);
    fs::copy(casesfile, r("cases.json")).unwrap();
    for (p, h) in [
        (
            cases[0]["image"].as_str().unwrap(),
            cases[0]["image_sha256"].as_str().unwrap(),
        ),
        (
            cases[0]["key"].as_str().unwrap(),
            cases[0]["key_sha256"].as_str().unwrap(),
        ),
    ] {
        assert_eq!(hash(p), h);
        pins.insert(p.into(), h.into());
    }
    let req = cases[0]["request"].as_str().unwrap();
    pins.insert(req.into(), hash(req));
    let balanced = fs::read_to_string(
        Path::new(Z).join("crates/zopro-core/src/msm_affine/borrowed_pending/balanced.rs"),
    )
    .unwrap();
    let digit = function(&balanced, "fn digit_at(");
    fs::write(r("digit.rs"), digit).unwrap();
    // Preserve exact existing CPU-basis parsing and Key construction, stopping before Prover assembly.
    let cpupath = Path::new(Z).join("crates/zopro-prover/src/cpu_basis.rs");
    let cpu = fs::read_to_string(&cpupath).unwrap();
    let prefix = &cpu[cpu.find("pub const FORMAT:").unwrap()..cpu.find("impl Prover {").unwrap()];
    let a = cpu.find("pub fn load_cpu_basis_image(").unwrap();
    let b = a + cpu[a..].find("        let names: Vec<String>").unwrap();
    let body = cpu[a..b].replacen(
        "pub fn load_cpu_basis_image(image: Image) -> Result<Self, String>",
        "pub fn load_key(image: Image) -> Result<Key, String>",
        1,
    );
    assert!(body.contains("pub fn load_key"));
    let raw = fs::read_to_string(Path::new(Z).join("crates/zopro-prover/src/raw.rs")).unwrap();
    let a = raw.find("struct Reader<'a>").unwrap();
    let b = a + raw[a..].find("\n#[cfg(").unwrap();
    let reader = &raw[a..b];
    let loader=format!("use zopro_core::{{Image,Points,Key,artifact::Artifact}};\nconst ALIGN:usize=64;\n{}\n{prefix}\n{reader}\n{body}\nOk(key)\n}}\n",function(&raw,"fn padding("));
    fs::write(r("loader.rs"), loader).unwrap();
    let mut env = m["environment"].clone();
    env["RAYON_NUM_THREADS"] = json!("32");
    let mut corecmd=read("/home/ubuntu/src/zopro-perf-work/word-export176/caller-native-repair/core-OFF/command.json");
    let mut argv = corecmd["argv"].as_array().unwrap()[4..]
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    for a in &mut argv {
        if a=="/home/ubuntu/src/zopro-perf-work/word-export176/caller-source/crates/zopro-core/src/lib.rs"{*a=r("source/crates/zopro-core/src/lib.rs").display().to_string();}else if a.starts_with("zopro_bn254="){*a=format!("zopro_bn254={}",info["libraries"]["zopro_bn254"].as_str().unwrap());}else if a=="/home/ubuntu/src/zopro-perf-work/word-export176/caller-libs/OFF/libzopro_core.rlib"{*a=r("native-libs/libzopro_core.rlib").display().to_string();}else if a=="dependency=/home/ubuntu/src/zopro-perf-work/word-export176/frozen"{*a=format!("dependency={}",Path::new(info["libraries"]["zopro_core"].as_str().unwrap()).parent().unwrap().display());}
    }
    corecmd["argv"] = json!(argv);
    env["CARGO_PKG_NAME"] = json!("zopro-core");
    env["CARGO_PKG_VERSION"] = json!("0.1.0");
    env["CARGO_MANIFEST_DIR"] = json!(r("source/crates/zopro-core"));
    fs::create_dir(r("native-libs")).unwrap();
    let depsdir = Path::new(info["libraries"]["zopro_core"].as_str().unwrap())
        .parent()
        .unwrap();
    let sha2 = fs::read_dir(depsdir)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("libsha2-")
                && p.extension().is_some_and(|e| e == "rlib")
        })
        .collect::<Vec<_>>();
    assert_eq!(sha2.len(), 1);
    let link = |witness: bool| {
        let mut a = vec![
            T.into(),
            "--edition=2021".into(),
            "-O".into(),
            "-C".into(),
            "target-cpu=native".into(),
            "-C".into(),
            "lto=fat".into(),
            "-C".into(),
            "codegen-units=1".into(),
            "-L".into(),
            format!("dependency={}", depsdir.display()),
        ];
        for n in ["zopro_bn254", "serde_json", "memmap2"] {
            a.extend([
                "--extern".into(),
                format!("{n}={}", info["libraries"][n].as_str().unwrap()),
            ]);
        }
        a.extend(["--extern".into(), format!("sha2={}", sha2[0].display())]);
        a.extend([
            "--extern".into(),
            format!(
                "zopro_core={}",
                if witness {
                    info["libraries"]["zopro_core"].as_str().unwrap().to_owned()
                } else {
                    r("native-libs/libzopro_core.rlib").display().to_string()
                }
            ),
        ]);
        if witness {
            for n in ["zopro_prover", "zopro_zolana"] {
                a.extend([
                    "--extern".into(),
                    format!("{n}={}", info["libraries"][n].as_str().unwrap()),
                ]);
            }
        }
        a.extend([
            r(if witness { "witness.rs" } else { "model.rs" })
                .display()
                .to_string(),
            "-o".into(),
            r(if witness { "witness" } else { "model" })
                .display()
                .to_string(),
        ]);
        a
    };
    let steps = json!([{"kind":"command","name":"diagnostic-core-build","argv":corecmd["argv"],"environment":env,"timeout_seconds":900},{"kind":"command","name":"witness-helper-build","argv":link(true),"environment":env,"timeout_seconds":600},{"kind":"command","name":"model-helper-build","argv":link(false),"environment":env,"timeout_seconds":600},{"kind":"command","name":"accepted-witness","argv":[r("witness")],"environment":env,"timeout_seconds":300},{"kind":"command","name":"primary-work-model","argv":[r("model")],"environment":env,"timeout_seconds":600}]);
    for p in[manifest.display().to_string(),infofile.into(),casesfile.into(),"/home/ubuntu/src/zopro-perf-work/word-export176/caller-native-repair/core-OFF/command.json".into()]{pins.insert(p.clone(),hash(p));}
    for f in [
        "prepare.rs",
        "prepare",
        "question.md",
        "design.md",
        "diagnostic.rs",
        "witness.rs",
        "model.rs",
        "loader.rs",
        "digit.rs",
        "cases.json",
        "source-audit.json",
    ] {
        pins.insert(r(f).display().to_string(), hash(r(f)));
    }
    save("pins.json", &json!(pins));
    save(
        "plan.json",
        &json!({"version":1,"source_pins":r("pins.json"),"steps":steps}),
    );
    println!("Frozen177source/174originalexact/all176recover;5step read-only primary model; accepted field reused; no runtime candidate/proofs/performance claim");
}
