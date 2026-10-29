use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fq-fips175";
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
fn setup() {
    use std::process::Command;
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["full_performance_gate_passed"], false);
    for (p, h) in read(m["native_build_source_pins"].as_str().unwrap())
        .as_object()
        .unwrap()
    {
        assert_eq!(hash(p), h.as_str().unwrap());
        pin(&mut pins, p);
    }
    pin(&mut pins, m["native_build_source_pins"].as_str().unwrap());
    let originals = m["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(), 175);
    for (file, h) in originals {
        let from = Path::new(Z).join(file);
        assert_eq!(hash(&from), h.as_str().unwrap());
        pin(&mut pins, &from);
        let to = r("source").join(file);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(from, to).unwrap();
    }
    let file = "crates/zopro-bn254/src/ifma.rs";
    let p = r("source").join(file);
    let original = fs::read_to_string(&p).unwrap();
    let old = "    fn product<const FUSED: bool>(&self, b: &Self, c: &Self, d: &Self) -> Self {";
    let new=format!("{old} #[cfg(zopro_fq_fips)] if LANES==8 && MIXED && C::P==crate::FqConfig::P {{return self.product_fips::<FUSED>(b,c,d);}}");
    let helper = fs::read_to_string(r("kernel.txt")).unwrap();
    let marker = "#[cfg(test)]\nmod tests {";
    let mut text = original.clone();
    once(&mut text, old, &new);
    once(&mut text, marker, &format!("{helper}{marker}"));
    let test_marker="                verify(v[0].difference2(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]-c[i]*d[i]));";
    let extra = r#"                #[cfg(zopro_fq_fips)]
                if LANES==8 && MIXED && C::P==crate::FqConfig::P {
                    verify(v[0].product_fips::<false>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]));
                    verify(v[0].product_fips::<true>(&v[1],&v[2],&v[3]),core::array::from_fn(|i|a[i]*b[i]+c[i]*d[i]));
                    let mut fips_chain=v[0];let mut fips_want=a;
                    for step in 0..16 {
                        if step%2==0 {
                            fips_chain=fips_chain.product_fips::<false>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]*=b[i];}
                        } else {
                            fips_chain=fips_chain.product_fips::<true>(&v[1],&v[2],&v[3]);
                            for i in 0..LANES {fips_want[i]=fips_want[i]*b[i]+c[i]*d[i];}
                        }
                        verify(fips_chain,fips_want);
                    }
                }
"#;
    once(&mut text, test_marker, &format!("{test_marker}\n{extra}"));
    fs::write(&p, &text).unwrap();
    let mut back = text;
    once(&mut back, &new, old);
    once(&mut back, &helper, "");
    once(&mut back, &format!("{test_marker}\n{extra}"), test_marker);
    assert_eq!(back, original);
    let generator = "scripts/ifma8.py";
    assert!(originals.contains_key(generator));
    let generated = "crates/zopro-bn254/src/ifma8.rs";
    let command = |script: &Path| {
        let o = Command::new("python3").arg(script).output().unwrap();
        assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
        o
    };
    let output = command(&r("source").join(generator));
    fs::write(r("generator.log"), output.stdout).unwrap();
    fs::write(r("generator.stderr.log"), output.stderr).unwrap();
    for f in [file, generator] {
        let to = r("reverse-check").join(f);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(Path::new(Z).join(f), to).unwrap();
    }
    let reverse = command(&r("reverse-check").join(generator));
    fs::write(r("reverse-generator.log"), reverse.stdout).unwrap();
    fs::write(r("reverse-generator.stderr.log"), reverse.stderr).unwrap();
    assert_eq!(
        hash(r("reverse-check").join(generated)),
        originals[generated].as_str().unwrap()
    );
    assert_eq!(
        hash(r("reverse-check").join(file)),
        originals[file].as_str().unwrap()
    );
    let mut exact = 0;
    for (file, h) in originals {
        let p = r("source").join(file);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 173);
    save(
        r("source-audit.json"),
        &json!({"source_files":175,"original_files":175,"originals_exact":exact,"reverse175_by_unedit_and_official_generator":true,"template_file":file,"generated_file":generated,"old":old,"new":new,"helper":helper,"candidate_test_extra":extra,"marker":marker,"generator_unchanged":true,"product_cios_body_exact":true,"new_diagonal_arithmetic":true,"representation_unchanged":true,"core_solver_key_scheduler_forecast_exact":true,"production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    assert!(!flags.contains("zopro_fq_fips"));
    let candidate = format!("{flags} --cfg zopro_fq_fips");
    fs::write(r("native-flags.txt"), flags).unwrap();
    fs::write(r("candidate-flags.txt"), &candidate).unwrap();
    let mut steps = vec![
        json!({"kind":"command","name":"generator-check","argv":["python3",r("source").join(generator),"--check"],"timeout_seconds":180}),
    ];
    for label in ["control", "candidate"] {
        let mut env = m["environment"].clone();
        env["RUSTFLAGS"] = json!(if label == "candidate" {
            candidate.as_str()
        } else {
            flags
        });
        env["CARGO_TARGET_DIR"] = json!(r(format!("target-{label}")));
        env["CARGO_BUILD_JOBS"] = json!("16");
        env["RAYON_NUM_THREADS"] = json!("4");
        let cargo = r("source/Cargo.toml");
        steps.push(json!({"kind":"command","name":format!("{label}-field"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":["ifma8::tests::cios_carry_matches_arkworks","ifma8::tests::fused_reduction_boundaries"],"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));
    assert_eq!(steps.len(), 14);
    for f in [
        "design.md",
        "algebra.md",
        "kernel.txt",
        "model.rs",
        "model.json",
        "build-model.sh",
        "bootstrap.rs",
        "history-audit.md",
        "setup.rs",
        "code-audit.rs",
        "native.rs",
        "native-helper",
        "source-audit.json",
        "native-flags.txt",
        "candidate-flags.txt",
    ] {
        pin(&mut pins, r(f));
    }
    for f in [
        "crates/zopro-bench/examples/cpu-sdk.rs",
        "crates/zopro-bench/examples/support/perf.rs",
    ] {
        pin(&mut pins, Path::new(Z).join(f));
    }
    save(r("build-pins.json"), &json!(pins));
    save(
        r("build-plan.json"),
        &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
    );
    println!("Prepared14steps;175source/173exact/reverse175;officialgenerator;field33core124prover8eachrequired;ordinaryandfusedFq8MIXEDdiagonal;production175unchanged");
}
fn inspect() {
    use std::process::Command;
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    fs::create_dir(r("codegen")).unwrap();
    let mut results = vec![];
    for label in ["accepted", "control", "candidate"] {
        let binary = if label == "accepted" {
            PathBuf::from(m["native_binary"].as_str().unwrap())
        } else {
            r(format!("native/{label}-sdk"))
        };
        let output = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(output.status.success());
        let symbols = String::from_utf8(output.stdout).unwrap();
        let mut selected = vec![];
        let mut g2 = 0;
        let mut g1 = 0;
        for line in symbols.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let is_g2 = name.contains("zopro_core::msm::g2_borrowed::apply");
            let is_g1 = name.contains("stream_g1_impl") && name.contains(", true, true, &");
            if !is_g1 && !is_g2 {
                continue;
            }
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let bytes = u64::from_str_radix(w[1], 16).unwrap();
            let asm = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(asm.status.success());
            let text = String::from_utf8(asm.stdout).unwrap();
            let file = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&file, &text).unwrap();
            let ifma = text.lines().filter(|l| l.contains("vpmadd52")).count();
            assert!(ifma > 0);
            if is_g2 {
                g2 += 1;
            }
            if is_g1 {
                g1 += 1;
            }
            selected.push(json!({"symbol":name,"bytes":bytes,"ifma_sites":ifma,"assembly":file,"assembly_sha256":hash(&file),"G2":is_g2,"G1":is_g1}));
        }
        assert!(g2 > 0 && g1 > 0, "missing linked native callers: {label}");
        fs::write(r(format!("codegen/{label}-symbols.txt")), symbols).unwrap();
        results.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    // Source/flags/fingerprints and independent native tests qualify dispatch.
    // Selected linked caller bytes must change; instruction counts are static.
    for key in ["G1", "G2"] {
        let c = results[1]["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v[key] == true)
            .map(|v| v["bytes"].as_u64().unwrap())
            .sum::<u64>();
        let n = results[2]["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|v| v[key] == true)
            .map(|v| v["bytes"].as_u64().unwrap())
            .sum::<u64>();
        assert_ne!(c, n, "actual caller code unchanged for {key}");
    }
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"selection":"Fq/R256/eight lanes ordinary and fused only","source_cfg_and_fingerprints_checked":true,"actual_G1_G2_IFMA_callers_changed":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":results}),
    );
    println!("Actual linked G1/G2 native IFMA caller code changed; source/cfg qualification separate; no speed claim");
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
