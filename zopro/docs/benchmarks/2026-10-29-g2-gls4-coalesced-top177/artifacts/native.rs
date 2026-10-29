use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs,
    io::Read,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-gls4-coalesced-top177";
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
    let hook_old = "    hb_raw(bases, words, 12, 2)";
    let hook_new="    #[cfg(zopro_gls4_horner)]\n    if let Some(result)=super::g2_borrowed::gls4::try_raw(bases,words){return result;}\n    hb_raw(bases, words, 12, 2)";
    let module = "\n#[cfg(zopro_gls4_horner)]\npub(super) mod gls4;\n";
    let cost = r#"
#[cfg(zopro_gls4_horner)]
pub(crate) fn gls4_horner_peak(n:usize,workers:usize)->Option<usize> {
    // Six-output shape conservatively includes returned top pieces and all merge scratch.
    let p=shape::<G2Config>(1<<10,6,4,batch_size(11).max(32))?;
    let t=top::<G2Config>(n.div_ceil(4),11)?;
    // Two sign classes, split-off negative vector, and retained earlier component.
    let extra=mul(vector::<Affine<G2Config>>(1<<10,false)?,3)?;
    let held=add(p.held,mul(add(t.held,extra)?,4)?)?;
    // Four parts x eight signed 256-bucket groups have disjoint point ranges.
    // Keep the old conservative routing/output held bound, but charge the
    // collective point/pair/prefix scratch once across all 32 possible groups.
    let scratch=crate::msm::workspace_grouping::<G2Config>(n,32)?;
    let local_stack=mul(32,mul(512,size_of::<usize>())?)?;
    add(add(add(add(vector::<[u64;5]>(n,false)?,held)?,scratch)?,local_stack)?,mul(p.leaf,workers)?)
}
"#;
    let mut edits = vec![];
    for (file, old, new) in [
        (
            "crates/zopro-core/src/msm_affine/g2_phi.rs",
            hook_old,
            hook_new,
        ),
        (
            "crates/zopro-core/src/msm_affine/g2_borrowed.rs",
            "",
            module,
        ),
        ("crates/zopro-core/src/msm_affine/workspace.rs", "", cost),
    ] {
        let p = r("source").join(file);
        let original = fs::read_to_string(&p).unwrap();
        let mut text = original.clone();
        if old.is_empty() {
            text.push_str(new);
        } else {
            once(&mut text, old, new);
        }
        let mut back = text.clone();
        if old.is_empty() {
            assert!(back.ends_with(new));
            back.truncate(back.len() - new.len());
        } else {
            once(&mut back, new, old);
        }
        assert_eq!(back, original);
        fs::write(&p, &text).unwrap();
        edits.push(
            json!({"file":file,"old":old,"new":new,"append":old.is_empty(),"reverse_exact":true}),
        );
    }
    let child = "crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs";
    let lattice = "crates/zopro-core/src/msm_affine/g2_borrowed/gls4/lattice.rs";
    for (name, to) in [("gls4.rs", child), ("lattice.rs", lattice)] {
        let p = r("source").join(to);
        fs::create_dir_all(p.parent().unwrap()).unwrap();
        fs::copy(r(name), p).unwrap();
    }
    let mut exact = 0;
    let mut field = 0;
    for (file, h) in originals {
        let p = r("source").join(file);
        let unchanged = hash(&p) == h.as_str().unwrap();
        exact += usize::from(unchanged);
        if file.starts_with("crates/zopro-bn254/") {
            assert!(unchanged);
            field += 1;
        }
        pin(&mut pins, p);
    }
    assert_eq!(exact, 172);
    assert_eq!(field, 17);
    for p in [child, lattice] {
        pin(&mut pins, r("source").join(p));
    }
    save(
        r("source-audit.json"),
        &json!({"source_files":177,"original_files":175,"originals_exact":172,"reverse175_exact":true,"edits":edits,"new_files":[child,lattice],"field_source_files_exact":17,"native_field_tests_inherited":33,"solver_key_field_fft_exact":true,"original_borrowed_Window_methods_exact":true,"old_lattice_body_reused":true,"new_costs_charged_via_resource_analysis":true,"production_worktree_changed":false}),
    );
    let flags = m["rustflags"].as_str().unwrap();
    let candidate = format!("{flags} --cfg zopro_gls4_horner");
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
            json!(["msm_affine::g2_borrowed::gls4::tests::decomposition_and_digits_reconstruct_fullwidth","msm_affine::g2_borrowed::gls4::tests::affine_and_projective_horner_maps_match_scalars","msm_affine::g2_borrowed::gls4::tests::full_msm_matches_independent_generator_sums","msm_affine::g2_borrowed::gls4::tests::checked_admission_and_fallback_preserve_words","msm_affine::g2_borrowed::gls4::tests::indexed_top_signed_edges_match_generator_sum","msm_affine::g2_borrowed::gls4::tests::hybrid_msm_custom_rayon_pools_match_generator_sum","msm_affine::g2_borrowed::gls4::tests::coalesced_top_skewed_complete_curve_groups"])
        } else {
            json!(["msm_affine::exclusive_g2_tests::exclusive_queries_match_generator_oracle"])
        };
        steps.push(json!({"kind":"command","name":format!("{label}-core"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-core","--lib","--","--test-threads=1"],"environment":env,"required_tests":required,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("{label}-prover"),"argv":["cargo","test","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-prover","--lib","--test","cubic","--","--test-threads=1"],"environment":env,"unfiltered":true,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("libraries-{label}"),"argv":["cargo","build","--manifest-path",cargo,"--locked","--offline","--release","-p","zopro-bench","--lib","--message-format=json"],"environment":env,"timeout_seconds":1800}));
        steps.push(json!({"kind":"command","name":format!("freeze-{label}"),"argv":[r("native-helper"),"freeze",label],"environment":env,"timeout_seconds":180}));
        steps.push(json!({"kind":"link-sdk","name":label,"artifacts":r(format!("frozen-{label}-cargo.jsonl")),"driver":format!("{Z}/crates/zopro-bench/examples/cpu-sdk.rs"),"additional_sources":[format!("{Z}/crates/zopro-bench/examples/support/perf.rs")],"timeout_seconds":600}));
    }
    steps.push(json!({"kind":"command","name":"code-audit","argv":[r("native-helper"),"inspect"],"timeout_seconds":300}));
    assert_eq!(steps.len(), 12);
    for f in [
        "design.md",
        "history-audit.md",
        "gls4.rs",
        "lattice.rs",
        "historical-gls4.rs",
        "historical-integer-oracle.json",
        "bootstrap.rs",
        "top.rs",
        "top-tests.rs",
        "coalesced-tests.rs",
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
    println!("Prepared12steps;177source/172originalexact/reverse175;field17exact/33inherited;OFF124/ON131core+8provereach;newcoalescedtopGLS4");
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
        let o = Command::new("nm")
            .args(["-S", "--size-sort", "--defined-only", "-C"])
            .arg(&binary)
            .output()
            .unwrap();
        assert!(o.status.success());
        let text = String::from_utf8(o.stdout).unwrap();
        let mut selected = vec![];
        let mut new_count = 0;
        for line in text.lines() {
            let w: Vec<_> = line.split_whitespace().collect();
            if w.len() < 4 {
                continue;
            }
            let name = w[3..].join(" ");
            let new = name.contains("g2_borrowed::gls4");
            let caller = name.contains("g2_phi::raw_exclusive");
            if !new && !caller {
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
            let asm = String::from_utf8(o.stdout).unwrap();
            let path = r(format!("codegen/{label}-{}.asm", selected.len()));
            fs::write(&path, &asm).unwrap();
            new_count += usize::from(new);
            selected.push(json!({"symbol":name,"bytes":bytes,"new_GLS4":new,"existing_caller":caller,"assembly":path,"assembly_sha256":hash(&path)}));
        }
        if label == "candidate" {
            assert!(new_count > 0);
        } else {
            assert_eq!(new_count, 0);
        }
        fs::write(r(format!("codegen/{label}-symbols.txt")), text).unwrap();
        variants.push(json!({"variant":label,"binary":binary,"binary_sha256":hash(&binary),"selected_functions":selected}));
    }
    save(
        r("code-audit.json"),
        &json!({"structural_checks_passed":true,"linked_GLS4_present_only_ON":true,"activation_requires_separate_primary_preflight_trace":true,"source_cfg_and_fingerprints_checked":true,"static_counts_not_dynamic_or_stack_bounds":true,"performance_gain_claimed":false,"variants":variants}),
    );
    println!("LinkednewGLS4code onlyON; actualactivationmustpassseparateprimarypreflighttrace");
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
