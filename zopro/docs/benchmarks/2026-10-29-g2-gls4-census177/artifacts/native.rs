use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-census177";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
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
fn pin(p: &mut BTreeMap<String, String>, f: impl AsRef<Path>) {
    let f = f.as_ref();
    p.insert(f.display().to_string(), hash(f));
}
fn once(s: &mut String, a: &str, b: &str) {
    assert_eq!(s.matches(a).count(), 1, "{a}");
    *s = s.replace(a, b);
}
const PARENT: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-horner175";
fn setup() {
    assert!(!r("build-plan.json").exists());
    let parent = Path::new(PARENT);
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    let a = read(parent.join("source-audit.json"));
    assert_eq!(a["source_files"], 177);
    assert_eq!(a["field_source_files_exact"], 17);
    let oldpins = read(parent.join("build-pins.json"));
    let mut pins = BTreeMap::new();
    for p in [
        parent.join("source-audit.json"),
        parent.join("build-pins.json"),
        parent.join("analysis.json"),
    ] {
        pin(&mut pins, p);
    }
    assert_eq!(
        read(parent.join("analysis.json"))["independent_proof_count"],
        17551
    );
    let mut names: Vec<String> = m["source_files_sha256"]
        .as_object()
        .unwrap()
        .keys()
        .cloned()
        .collect();
    for file in a["new_files"].as_array().unwrap() {
        names.push(file.as_str().unwrap().into());
    }
    assert_eq!(names.len(), 177);
    for file in &names {
        let from = parent.join("source").join(file);
        assert_eq!(
            hash(&from),
            oldpins[from.display().to_string()].as_str().unwrap()
        );
        pin(&mut pins, &from);
        let to = r("source").join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
    }
    let file = "crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs";
    let p = r("source").join(file);
    let original = fs::read_to_string(&p).unwrap();
    let mut text = original.clone();
    let mut edits = vec![];
    let hook = "        size: usize,\n    ) {\n        for (p, s) in bases.iter().zip(short) {";
    let hooked="        size: usize,\n    ) {\n        #[cfg(zopro_gls4_census)]\n        if census::enabled(){return census::run_feed(self,bases,short,component,w,size);}\n        for (p, s) in bases.iter().zip(short) {";
    let chunk = "    let chunk = bases.len().div_ceil(PARTS);";
    let chunked="    let chunk = bases.len().div_ceil(PARTS);\n    #[cfg(zopro_gls4_census)]\n    let query_id=census::query();";
    let window = "                    window(&bases[start..end], &short[start..end], w, &x_factor)";
    let tagged="                    #[cfg(zopro_gls4_census)]\n                    let _context=census::context(query_id,w,part);\n                    window(&bases[start..end], &short[start..end], w, &x_factor)";
    for (old, new) in [(hook, hooked), (chunk, chunked), (window, tagged)] {
        once(&mut text, old, new);
        edits.push(json!({"old":old,"new":new}));
    }
    let suffix = "\n#[cfg(zopro_gls4_census)]\nmod census;\n";
    text.push_str(suffix);
    let mut reverse = text.clone();
    reverse.truncate(reverse.len() - suffix.len());
    for e in edits.iter().rev() {
        once(
            &mut reverse,
            e["new"].as_str().unwrap(),
            e["old"].as_str().unwrap(),
        );
    }
    assert_eq!(reverse, original);
    fs::write(&p, text).unwrap();
    let child = "crates/zopro-core/src/msm_affine/g2_borrowed/gls4/census.rs";
    fs::copy(r("census.rs"), r("source").join(child)).unwrap();
    let mut exact = 0;
    let mut field = 0;
    for name in &names {
        let p = r("source").join(name);
        let same = hash(&p) == hash(parent.join("source").join(name));
        exact += usize::from(same);
        if name.starts_with("crates/zopro-bn254/") {
            assert!(same);
            field += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 176);
    assert_eq!(field, 17);
    pin(&mut pins, r("source").join(child));
    save(
        r("source-audit.json"),
        &json!({"source_files":178,"parent_source_files":177,"parent_files_exact":176,"parent_recovered_by_inverse_edits":true,"changed":file,"edits":edits,"suffix":suffix,"new_file":child,"field_source_files_exact":17,"original_Window_methods_exact":true,"arithmetic_and_queue_order_unchanged":true,"no_production_edits":true,"native128core8prover33field_inherited":true}),
    );
    let oldflags = fs::read_to_string(parent.join("candidate-flags.txt")).unwrap();
    let flags = format!("{oldflags} --cfg zopro_gls4_census");
    fs::write(r("native-flags.txt"), oldflags).unwrap();
    fs::write(r("candidate-flags.txt"), &flags).unwrap();
    let mut env = m["environment"].clone();
    env["RUSTFLAGS"] = json!(flags);
    env["CARGO_TARGET_DIR"] = json!(r("target-candidate"));
    env["CARGO_BUILD_JOBS"] = json!("16");
    env["RAYON_NUM_THREADS"] = json!("4");
    env["ZOPRO_GLS4_CENSUS"] = json!("0");
    let cargo = r("source/Cargo.toml");
    let mut steps = vec![];
    steps.push(json!({"kind":"command","name":"census-tests","argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","msm_affine::g2_borrowed::gls4::census::tests","--","--test-threads=1"],"environment":env,"required_tests":["msm_affine::g2_borrowed::gls4::census::tests::observed_state_matches_original_horner_feeds","msm_affine::g2_borrowed::gls4::census::tests::explicit_exception_and_queue_counts"],"unfiltered":false,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":"libraries-candidate","argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
    steps.push(json!({"kind":"command","name":"freeze-candidate","argv":[r("native-helper"),"freeze","candidate"],"environment":env,"timeout_seconds":180}));
    steps.push(json!({"kind":"link-sdk","name":"candidate","artifacts":r("frozen-candidate-cargo.jsonl"),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));
    assert_eq!(steps.len(), 5);
    for name in [
        "design.md",
        "census.rs",
        "setup.rs",
        "bootstrap.rs",
        "code-audit.rs",
        "native.rs",
        "native-helper",
        "native-flags.txt",
        "candidate-flags.txt",
        "source-audit.json",
    ] {
        pin(&mut pins, r(name));
    }
    for name in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        pin(&mut pins, Path::new(Z).join(name));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared5steps;178source/176parentexact/reverse177;only2newobserver tests;no benchmark claim");
}

fn inspect() {
    use std::process::Command;
    let binary = r("native/candidate-sdk");
    let o = Command::new("nm")
        .args(["-S", "--size-sort", "--defined-only", "-C"])
        .arg(&binary)
        .output()
        .unwrap();
    assert!(o.status.success());
    let text = String::from_utf8(o.stdout).unwrap();
    fs::create_dir(r("codegen")).unwrap();
    let mut selected = vec![];
    for line in text.lines() {
        let w: Vec<_> = line.split_whitespace().collect();
        if w.len() < 4 || !line.contains("g2_borrowed::gls4::census") || !["t", "T"].contains(&w[2])
        {
            continue;
        }
        let address = u64::from_str_radix(w[0], 16).unwrap();
        let bytes = u64::from_str_radix(w[1], 16).unwrap();
        let o = Command::new("objdump")
            .args(["-d", "-C", "--no-show-raw-insn"])
            .arg(format!("--start-address=0x{address:x}"))
            .arg(format!("--stop-address=0x{:x}", address + bytes))
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let path = r(format!("codegen/census-{}.asm", selected.len()));
        fs::write(&path, o.stdout).unwrap();
        selected.push(json!({"symbol":w[3..].join(" "),"bytes":bytes,"assembly":path,"assembly_sha256":hash(&path)}));
    }
    assert!(!selected.is_empty());
    fs::write(r("codegen/symbols.txt"), text).unwrap();
    save(
        r("code-audit.json"),
        &json!({"linked_observer_present":true,"performance_gain_claimed":false,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}),
    );
    println!("Linkedobserver present; actual records/flows must validate after diagnostic");
}

fn freeze(label: &str) {
    assert!(["control", "candidate"].contains(&label));
    let flags = std::env::var("RUSTFLAGS").unwrap();
    assert_eq!(
        flags,
        fs::read_to_string(r(if label == "candidate" {
            "candidate-flags.txt"
        } else {
            "native-flags.txt"
        }))
        .unwrap()
    );
    let root = r("frozen").join(label);
    fs::create_dir_all(&root).unwrap();
    let mut all = BTreeMap::<String, BTreeSet<String>>::new();
    let mut pins = BTreeMap::new();
    for l in fs::read_to_string(r(format!("native/libraries-{label}/stdout.log")))
        .unwrap()
        .lines()
    {
        let a: Value = serde_json::from_str(l).unwrap();
        if a["reason"] != "compiler-artifact" {
            continue;
        }
        let n = a["target"]["name"].as_str().unwrap();
        let files = a["filenames"].as_array().unwrap();
        let metas: Vec<_> = files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rmeta") && p.contains("/deps/"))
            .collect();
        for f in files
            .iter()
            .filter_map(Value::as_str)
            .filter(|p| p.ends_with(".rlib") || p.ends_with(".so"))
        {
            let p = if f.ends_with(".rlib") && !f.contains("/deps/") && metas.len() == 1 {
                Path::new(metas[0]).with_extension("rlib")
            } else {
                PathBuf::from(f)
            };
            let h = hash(&p);
            let ext = p.extension().unwrap().to_str().unwrap();
            let to = root.join(format!("lib{n}-{h}.{ext}"));
            if to.exists() {
                assert_eq!(hash(&to), h)
            } else {
                fs::copy(&p, &to).unwrap();
            }
            pin(&mut pins, &to);
            if ext == "rlib" {
                all.entry(n.into())
                    .or_default()
                    .insert(to.display().to_string());
            }
        }
    }
    let mut lines = String::new();
    let mut libs = BTreeMap::new();
    for n in [
        "zopro_core",
        "zopro_bn254",
        "zopro_prover",
        "zopro_zolana",
        "libc",
        "memmap2",
        "rayon",
        "serde_json",
    ] {
        assert_eq!(all[n].len(), 1);
        let p = all[n].first().unwrap();
        libs.insert(n, p);
        lines.push_str(
            &serde_json::to_string(
                &json!({"reason":"compiler-artifact","target":{"name":n},"filenames":[p]}),
            )
            .unwrap(),
        );
        lines.push('\n');
    }
    let mut fingerprints = vec![];
    for e in fs::read_dir(r(format!("target-{label}/release/.fingerprint"))).unwrap() {
        let e = e.unwrap();
        for n in ["zopro-core", "zopro-bn254"] {
            let p = e.path().join(format!("lib-{}.json", n.replace('-', "_")));
            if e.file_name().to_string_lossy().starts_with(n) && p.exists() {
                let f = read(&p);
                assert_eq!(
                    f["rustflags"],
                    json!(flags.split_whitespace().collect::<Vec<_>>())
                );
                if n == "zopro-core" {
                    assert!(f["features"].as_str().unwrap().contains("ifma8"));
                }
                fingerprints.push(json!({"path":p,"receipt":f}));
            }
        }
    }
    assert!(fingerprints.len() >= 2);
    fs::write(r(format!("frozen-{label}-cargo.jsonl")), lines).unwrap();
    save(
        r(format!("frozen-{label}.json")),
        &json!({"rustflags":flags,"libraries":libs,"input_pins":pins,"fingerprints":fingerprints,"proc_macro_shared_objects_frozen":true}),
    );
    println!(
        "Frozen{label}{}rlib/sofiles; nativeflags/features checked",
        pins.len()
    );
}
fn main() {
    let a: Vec<_> = std::env::args().collect();
    match a.get(1).map(String::as_str) {
        Some("setup") => setup(),
        Some("freeze") => freeze(&a[2]),
        Some("inspect") => inspect(),
        _ => panic!("setup/freeze"),
    }
}
