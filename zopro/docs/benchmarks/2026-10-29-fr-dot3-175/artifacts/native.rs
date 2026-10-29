use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/fr-dot3-175";
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
    assert!(!r("build-plan.json").exists());
    let manifest = Path::new(Z).join(
        "docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json",
    );
    let m = read(&manifest);
    assert_eq!(m["adopted"], true);
    assert_eq!(m["full_performance_gate_passed"], false);
    let mut pins = BTreeMap::new();
    pin(&mut pins, &manifest);
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
    let fp_old = "    /// self^e, e little-endian limbs.";
    let fp_new = r#"    /// Three canonical-input products with one reduction (private experiment).
    #[cfg(zopro_fr_dot3)]
    #[inline(always)]
    pub fn sum_of_three_products(a:&Self,b:&Self,c:&Self,d:&Self,e:&Self,f:&Self)->Self {
        Self(crate::sop3::sum3([a.0,c.0,e.0],[b.0,d.0,f.0],C::P,C::INV),PhantomData)
    }
    /// self^e, e little-endian limbs."#;
    let lib_old = "pub mod fp;";
    let lib_new = "pub mod fp;\n#[cfg(zopro_fr_dot3)]\nmod sop3;";
    let dense_old="                    next[i] = Fr::sum_of_products(&a[row], &s[0], &a[row + 1], &s[1])\n                        + a[row + 2] * s[2];";
    let dense_new=format!("                    #[cfg(zopro_fr_dot3)]\n                    {{ next[i]=Fr::sum_of_three_products(&a[row],&s[0],&a[row+1],&s[1],&a[row+2],&s[2]); }}\n                    #[cfg(not(zopro_fr_dot3))]\n                    {{\n{dense_old}\n                    }}");
    let mut changes = vec![];
    for (file, old, new) in [
        ("crates/zopro-bn254/src/fp.rs", fp_old, fp_new),
        ("crates/zopro-bn254/src/lib.rs", lib_old, lib_new),
        (
            "crates/zopro-core/src/solver/sparse_poseidon.rs",
            dense_old,
            dense_new.as_str(),
        ),
    ] {
        let p = r("source").join(file);
        let original = fs::read_to_string(&p).unwrap();
        let mut text = original.clone();
        once(&mut text, old, new);
        let mut reverse = text.clone();
        once(&mut reverse, new, old);
        assert_eq!(reverse, original);
        fs::write(&p, text).unwrap();
        pin(&mut pins, p);
        changes.push(json!({"file":file,"old":old,"new":new,"inverse_exact":true}));
    }
    let child = "crates/zopro-bn254/src/sop3.rs";
    assert!(!originals.contains_key(child));
    fs::copy(r("sop3.rs"), r("source").join(child)).unwrap();
    pin(&mut pins, r("source").join(child));
    let mut exact = 0;
    for (file, h) in originals {
        let p = r("source").join(file);
        if hash(&p) == h.as_str().unwrap() {
            exact += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 172);
    save(
        r("source-audit.json"),
        &json!({"source_files":176,"original_files":175,"originals_exact":172,"reverse175_by_inverse_edits":true,"changes":changes,"new_module":child,"key_scheduler_forecast_exact":true,"solver_dense_dot_only":true,"partial_rounds_exact":true,"IFMA_sources_exact":true,"production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    let candidate = format!("{flags} --cfg zopro_fr_dot3");
    fs::write(r("native-flags.txt"), flags).unwrap();
    fs::write(r("candidate-flags.txt"), &candidate).unwrap();
    let mut steps = vec![
        json!({"kind":"command","name":"generator-check","argv":["python3",r("source/scripts/ifma8.py"),"--check"],"timeout_seconds":180}),
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
        let required = if label == "candidate" {
            json!([
                "sop3::tests::small_radix_exact_cancellation",
                "sop3::tests::canonical_sum3_and_chains_match_arkworks"
            ])
        } else {
            json!(["ifma8::tests::cios_carry_matches_arkworks"])
        };
        steps.push(json!({"kind":"command","name":format!("{label}-field"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bn254","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1800}));
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
        "history-audit.md",
        "bounds.md",
        "sop3.rs",
        "bootstrap.rs",
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
    println!("Prepared14steps/176source172originalexact/reverse175;control33/candidate35field124core8provereach;densewidth3sum3only");
}

fn inspect() {
    use std::process::Command;
    let m=read(format!("{Z}/docs/benchmarks/2026-10-29-balanced-kh-adoption/artifacts/production-source-manifest.json"));
    fs::create_dir(r("codegen")).unwrap();
    let mut variants = vec![];
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
        for line in symbols.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let dense = name.contains("::run_poseidon::dense");
            let caller = name.contains("::run_poseidon_task");
            if !dense && !caller {
                continue;
            }
            let address = u64::from_str_radix(w[0], 16).unwrap();
            let bytes = u64::from_str_radix(w[1], 16).unwrap();
            let output = Command::new("objdump")
                .args(["-d", "-C", "--no-show-raw-insn"])
                .arg(format!("--start-address=0x{address:x}"))
                .arg(format!("--stop-address=0x{:x}", address + bytes))
                .arg(&binary)
                .output()
                .unwrap();
            assert!(output.status.success());
            let text = String::from_utf8(output.stdout).unwrap();
            let path = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&path, &text).unwrap();
            selected.push(json!({"symbol":name,"bytes":bytes,"dense":dense,"caller":caller,"dense_call_sites":text.lines().filter(|l|l.contains("call")&&l.contains("::run_poseidon::dense")).count(),"assembly":path,"assembly_sha256":hash(&path)}));
        }
        assert_eq!(
            selected.iter().filter(|x| x["dense"] == true).count(),
            1,
            "missing linked dense {label}"
        );
        assert_eq!(
            selected.iter().filter(|x| x["caller"] == true).count(),
            1,
            "missing caller {label}"
        );
        assert!(
            selected
                .iter()
                .map(|x| x["dense_call_sites"].as_u64().unwrap())
                .sum::<u64>()
                > 0
        );
        fs::write(r(format!("codegen/{label}-symbols.txt")), symbols).unwrap();
        variants.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    let size = |v: &Value| {
        v["selected_functions"]
            .as_array()
            .unwrap()
            .iter()
            .find(|x| x["dense"] == true)
            .unwrap()["bytes"]
            .as_u64()
            .unwrap()
    };
    assert_ne!(size(&variants[1]), size(&variants[2]));
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"solver_dense_changed_and_called":true,"source_cfg_and_fingerprints_checked":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":variants}),
    );
    println!("Verified actuallinkedsolver dense code changed and originaltask caller calls it; no speedclaim");
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
