use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/solver-a-stream176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
fn r(p: impl AsRef<Path>) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn function(text: &str, signature: &str) -> String {
    let a = text.find(signature).unwrap();
    let start = a + text[a..].find('{').unwrap();
    let mut end = start + 1;
    let mut depth = 1;
    while depth > 0 {
        match text.as_bytes()[end] {
            b'{' => depth += 1,
            b'}' => depth -= 1,
            _ => {}
        }
        end += 1;
    }
    text[a..end].into()
}
fn main() {
    assert!(!r("source").exists());
    let manifest = read(Path::new(Z).join("docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json"));
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let bytes = fs::read(Path::new(Z).join(p)).unwrap();
        assert_eq!(hash(&bytes), h.as_str().unwrap());
        let to = r("source").join(p);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::write(to, bytes).unwrap();
    }
    let solver = fs::read_to_string(r("source/crates/zopro-core/src/solver.rs")).unwrap();
    let original = function(&solver, "pub(crate) fn solve_in_with_slots(");
    let signature = original.split_once('{').unwrap().0;
    let changed_signature = "fn solve_with_a_feed<F>(&self, witness: &[Fr], domain: usize, slots: &[usize], materialize_c: bool, delivery: &mut Delivery<'_, '_, F>) -> Result<Solution, String>\n    where F: FnMut(Arc<Chunk>) -> Result<(), String> ";
    let mut changes = vec![(signature.to_string(), changed_signature.to_string())];
    changes.push(("        if witness.len() != self.n_inputs - 1 {".into(), "        if !std::ptr::eq(self, delivery.ready.plan) { return Err(\"mismatched A readiness plan\".into()); }\n        if witness.len() != self.n_inputs - 1 {".into()));
    changes.push(("        let epochs = solve_epochs();".into(), "        // SAFETY: Ready's epoch zero contains only initialized input wires.\n        unsafe { delivery.publish(0, false, view)?; }\n        let epochs = solve_epochs();".into()));
    changes.push((
        "                for epoch in &schedule.epochs {".into(),
        "                for (epoch_number, epoch) in schedule.epochs.iter().enumerate() {".into(),
    ));
    changes.push(("                    })?;\n                }\n            } else {".into(), "                    })?;\n                    // SAFETY: the full epoch joined and Ready proves unique\n                    // persistent writers; consumers own copies, never w.\n                    unsafe { delivery.publish(epoch_number + 1, false, view)?; }\n                }\n            } else {".into()));
    changes.push(("        if uninitialized {\n            // SAFETY: the cached".into(), "        // SAFETY: every solver branch succeeded and joined all operations.\n        // Covers serial/refined runtime fallbacks before slot compaction.\n        unsafe { delivery.publish(0, true, view)?; }\n        if uninitialized {\n            // SAFETY: the cached".into()));
    let mut changed = original.clone();
    for (a, b) in &changes {
        assert_eq!(changed.matches(a).count(), 1, "{a}");
        changed = changed.replacen(a, b, 1);
    }
    let mut recovered = changed.clone();
    for (a, b) in changes.iter().rev() {
        assert_eq!(recovered.matches(b).count(), 1);
        recovered = recovered.replacen(b, a, 1);
    }
    assert_eq!(recovered, original);
    let ready = format!(
        "{}\nimpl Plan {{\n{}\n}}\n",
        fs::read_to_string(r("ready.rs")).unwrap(),
        changed
    );
    let new_files = [
        (
            "stream.rs",
            "crates/zopro-core/src/msm_affine/borrowed_pending/early_stream.rs",
        ),
        ("trial.rs", "crates/zopro-core/src/early_a_trial.rs"),
    ];
    for (from, to) in new_files {
        let to = r("source").join(to);
        fs::create_dir_all(to.parent().unwrap()).unwrap();
        fs::copy(r(from), to).unwrap();
    }
    let to = r("source/crates/zopro-core/src/solver/early_feed.rs");
    fs::create_dir_all(to.parent().unwrap()).unwrap();
    fs::write(to, ready).unwrap();
    let cfg="#[cfg(all(zopro_early_a, target_arch = \"x86_64\", target_os = \"linux\", target_feature = \"avx512ifma\", target_feature = \"avx512vl\", target_feature = \"avx512dq\", feature = \"ifma8\", not(no_ifma)))]";
    let prover_cfg="#[cfg(all(zopro_early_a, target_arch = \"x86_64\", target_os = \"linux\", target_feature = \"avx512ifma\", target_feature = \"avx512vl\", target_feature = \"avx512dq\", not(no_ifma)))]";
    let suffixes = BTreeMap::from([
        ("crates/zopro-core/src/msm_affine/borrowed_pending.rs",format!("\n{cfg}\npub(crate) mod early_stream;\n")),
        ("crates/zopro-core/src/msm_affine/workspace.rs",format!("\n{cfg}\npub(crate) fn early_window_forecast(half: usize, batch: usize) -> Option<usize> {{ window::<zopro_bn254::G1Config>(half, batch) }}\n")),
        ("crates/zopro-core/src/solver.rs",format!("\n{cfg}\npub(crate) mod early_feed;\n")),
        ("crates/zopro-core/src/lib.rs",format!("\n{cfg}\npub mod early_a_trial;\n{cfg}\nimpl Key {{ pub fn prepare_early_a(&self) -> Option<early_a_trial::Prepared<'_>> {{ early_a_trial::Prepared::new(self) }} }}\n")),
        ("crates/zopro-prover/src/lib.rs",format!("\n{prover_cfg}\nimpl Prover {{ pub fn prepare_early_a(&self) -> Option<zopro_core::early_a_trial::Prepared<'_>> {{ self.key.prepare_early_a() }} }}\n")),
    ]);
    for (p, s) in &suffixes {
        let p = r("source").join(p);
        let old = fs::read_to_string(&p).unwrap();
        fs::write(p, format!("{old}{s}")).unwrap();
    }
    let mut exact = 0;
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        let bytes = fs::read(r("source").join(p)).unwrap();
        if hash(&bytes) == h.as_str().unwrap() {
            exact += 1;
        }
        let stripped = if let Some(s) = suffixes.get(p.as_str()) {
            String::from_utf8(bytes)
                .unwrap()
                .strip_suffix(s)
                .unwrap()
                .as_bytes()
                .to_vec()
        } else {
            bytes
        };
        assert_eq!(hash(&stripped), h.as_str().unwrap());
    }
    assert_eq!(exact, 171);
    fs::write(r("original-solver.rs"), original).unwrap();
    fs::write(r("source-audit.json"),serde_json::to_vec_pretty(&json!({"originals":176,"originals_exact":171,"snapshot_files":179,"all_original_bytes_recovered":true,"original_solver_and_msm_unchanged":true,"solver_clone_exact_after_hook_recovery":true,"suffixes":suffixes,"solver_clone_edits":changes,"new_focused_tests":5,"production_changed":false})).unwrap()).unwrap();
    fs::create_dir_all(r("driver/src")).unwrap();
    fs::copy(r("driver.rs"), r("driver/src/main.rs")).unwrap();
    fs::copy(
        Path::new(Z).join("crates/zopro-bench/examples/support/perf.rs"),
        r("driver/src/perf.rs"),
    )
    .unwrap();
    fs::write(r("driver/Cargo.toml"),"[package]\nname=\"zopro-solver-a-stream\"\nversion=\"0.0.0\"\nedition=\"2021\"\npublish=false\n[workspace]\n[dependencies]\nzopro-core={path=\"../source/crates/zopro-core\"}\nzopro-prover={path=\"../source/crates/zopro-prover\"}\nzopro-zolana={path=\"../source/crates/zopro-zolana\"}\nzopro-bn254={path=\"../source/crates/zopro-bn254\"}\nserde_json=\"=1.0.149\"\nsha2=\"=0.10.9\"\nrayon=\"1.10\"\nlibc=\"0.2\"\nmemmap2=\"0.9\"\n[profile.release]\nopt-level=3\nlto=\"fat\"\ncodegen-units=1\n").unwrap();
    let mut env =
        read("/home/ubuntu/src/zopro-perf-work/coarse-proof-phase-universal176/native-plan.json")
            ["steps"][0]["environment"]
            .clone();
    env["RUSTFLAGS"] = json!(format!(
        "{} --cfg zopro_early_a",
        manifest["rustflags"].as_str().unwrap()
    ));
    env["CARGO_TARGET_DIR"] = json!(r("target").display().to_string());
    env["CARGO_BUILD_JOBS"] = json!("16");
    fs::write(
        r("environment.json"),
        serde_json::to_vec_pretty(&env).unwrap(),
    )
    .unwrap();
    let cargo = "/home/ubuntu/.rustup/toolchains/1.98.1-x86_64-unknown-linux-gnu/bin/cargo";
    let steps = vec![
        json!({"kind":"command","name":"core-tests","argv":[cargo,"test","--locked","--offline","--release","--manifest-path",r("source/Cargo.toml"),"-p","zopro-core","--features","ifma8","--lib","--","--test-threads=1"],"environment":env,"timeout_seconds":1800,"unfiltered":true}),
        json!({"kind":"command","name":"driver-lock","argv":[cargo,"generate-lockfile","--offline","--manifest-path",r("driver/Cargo.toml")],"environment":env,"timeout_seconds":120}),
        json!({"kind":"command","name":"driver-build","argv":[cargo,"build","--locked","--offline","--release","-vv","--message-format=json","--manifest-path",r("driver/Cargo.toml")],"environment":env,"timeout_seconds":1800}),
    ];
    let mut pins = BTreeMap::new();
    fn scan(p: &Path, pins: &mut BTreeMap<String, String>) {
        for e in fs::read_dir(p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                scan(&p, pins)
            } else {
                pins.insert(p.display().to_string(), hash(&fs::read(p).unwrap()));
            }
        }
    }
    scan(&r("source"), &mut pins);
    scan(&r("driver"), &mut pins);
    for p in [
        "setup.rs",
        "design.md",
        "source-audit.json",
        "environment.json",
        "stream.rs",
        "ready.rs",
        "trial.rs",
        "driver.rs",
    ] {
        let p = r(p);
        pins.insert(p.display().to_string(), hash(&fs::read(p).unwrap()));
    }
    fs::write(
        r("build-pins.json"),
        serde_json::to_vec_pretty(&pins).unwrap(),
    )
    .unwrap();
    fs::write(
        r("build-plan.json"),
        serde_json::to_vec_pretty(
            &json!({"version":1,"source_pins":r("build-pins.json"),"steps":steps}),
        )
        .unwrap(),
    )
    .unwrap();
    println!("Prepared179files/171exact/all176recovered; original solver+MSM exact; five new focused tests. No measurements.");
}
