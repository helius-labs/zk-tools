use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g1-offset177";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/bounded-gather-sdk177";
const P: &str = "/home/ubuntu/src/zopro-perf-work/affine-redc-offset-mask-callers176";
const F: &str = "/home/ubuntu/src/zopro-perf-work/affine-redc-offset-mask176";
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn save(p: &str, v: &Value) { fs::write(format!("{R}/{p}"), serde_json::to_vec_pretty(v).unwrap()).unwrap(); }
fn append(rel: &str, suffix: &str) {
    let p = format!("{R}/source/{rel}");
    let mut s = fs::read_to_string(&p).unwrap(); s.push_str(suffix); fs::write(p,s).unwrap();
}
fn main() {
    assert!(!Path::new(&format!("{R}/source")).exists());
    let manifest_path = format!("{Z}/docs/benchmarks/2026-11-11-phi-grain-parent176/artifacts/production-source-manifest.json");
    let manifest = read(&manifest_path);
    let originals = manifest["source_files_sha256"].as_object().unwrap();
    assert_eq!(originals.len(),176);
    let mut restored = vec![];
    for (rel, h) in originals {
        let mut from = format!("{Z}/{rel}");
        if hash(&from) != h.as_str().unwrap() {
            assert!(["crates/zopro-core/src/lib.rs", "crates/zopro-core/build.rs"].contains(&rel.as_str()));
            from = format!("{OLD}/accepted-source/{rel}"); restored.push(rel.clone());
        }
        assert_eq!(hash(&from),h.as_str().unwrap());
        let to = format!("{R}/source/{rel}");
        fs::create_dir_all(Path::new(&to).parent().unwrap()).unwrap(); fs::copy(from,to).unwrap();
    }
    let qualification = read(format!("{F}/qualification.json"));
    assert_eq!(qualification["passed"],true);
    let parent_field = fs::read_to_string(format!("{F}/source-fixed/crates/zopro-bn254/src/ifma8.rs")).unwrap();
    let current_field = fs::read_to_string(format!("{R}/source/crates/zopro-bn254/src/ifma8.rs")).unwrap();
    let suffix = "\n// Private deferred affine-offset study; original methods stay byte-exact.\n#[cfg(all(zopro_affine_offset, zopro_cios_cancel_carry))]\nmod affine_offset;\n";
    assert_eq!(parent_field.strip_suffix(suffix).unwrap(),current_field);
    let module = "crates/zopro-bn254/src/ifma8/affine_offset.rs";
    fs::create_dir_all(format!("{R}/source/crates/zopro-bn254/src/ifma8")).unwrap();
    fs::copy(format!("{F}/source-fixed/{module}"),format!("{R}/source/{module}")).unwrap();
    append("crates/zopro-bn254/src/ifma8.rs", "\n#[cfg(all(zopro_g1_affine_offset, zopro_cios_cancel_carry))]\nmod affine_offset;\n");
    let edits = read(format!("{P}/source-audit.json"));
    let mut applied = vec![];
    for edit in edits["reversible_core_changes"].as_array().unwrap() {
        if edit["file"] == "crates/zopro-core/src/msm.rs" && edit["old"].is_string() {
            let p = format!("{R}/source/crates/zopro-core/src/msm.rs");
            let s = fs::read_to_string(&p).unwrap(); let old = edit["old"].as_str().unwrap();
            assert_eq!(s.matches(old).count(),1);
            let new = edit["new"].as_str().unwrap().replace("zopro_affine_offset","zopro_g1_affine_offset");
            fs::write(p,s.replace(old,&new)).unwrap();
            applied.push(json!({"file":edit["file"],"old":old,"new":new}));
        }
    }
    assert_eq!(applied.len(),4);
    append("crates/zopro-core/src/simd.rs", "\n#[cfg(all(zopro_g1_affine_offset,zopro_cios_cancel_carry,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nimpl FqBatch {\n #[inline(always)]pub(crate)fn affine_mul_minus(&self,b:&Self,x:&Self)->Self{Self(self.0.affine_mul_minus(&b.0,&x.0))}\n #[inline(always)]pub(crate)fn affine_mul_minus_sum(&self,b:&Self,x:&Self,y:&Self)->Self{Self(self.0.affine_mul_minus_sum(&b.0,&x.0,&y.0))}\n}\n");
    let tests = "crates/zopro-core/src/msm/affine_offset_caller.rs";
    fs::copy(format!("{P}/source/{tests}"),format!("{R}/source/{tests}")).unwrap();
    append("crates/zopro-core/src/msm.rs", "\n#[cfg(all(test,feature=\"ifma8\",target_arch=\"x86_64\",target_feature=\"avx512ifma\",target_feature=\"avx512vl\",target_feature=\"avx512dq\",not(no_ifma)))]\nmod affine_offset_caller;\n");
    // Qualification adapters are test-only: they cannot alter SDK code layout.
    for rel in ["crates/zopro-core/src/msm/g2_borrowed.rs","crates/zopro-core/src/msm/g2_prefix.rs"] {
        assert_eq!(hash(format!("{R}/source/{rel}")),originals[rel].as_str().unwrap());
    }
    let mut source_pins = BTreeMap::new(); let mut changed = vec![];
    for (rel,h) in originals {
        let p = format!("{R}/source/{rel}"); let actual = hash(&p);
        if actual != h.as_str().unwrap() { changed.push(rel); }
        source_pins.insert(p,actual);
    }
    assert_eq!(changed.len(),3);
    for rel in [module,tests] { let p=format!("{R}/source/{rel}");source_pins.insert(p.clone(),hash(p)); }
    save("source-audit.json",&json!({"base_manifest":manifest_path,"base_manifest_sha256":hash(&manifest_path),"base_files":176,"changed_originals":changed,"added_files":[module,tests],"restored_local_default_off_gather_files":restored,"G2_files_exact":true,"field_parent_body_exact":true,"field_module_sha256":hash(format!("{R}/source/{module}")),"test_adapter_sha256":hash(format!("{R}/source/{tests}")),"G1_edits":applied,"SDK_requests":0,"proofs":0,"production_changed":false,"adopted":false}));
    let flags = fs::read_to_string(format!("{OLD}/control-flags.txt")).unwrap().replace(" --check-cfg=cfg(zopro_bounded_gather)","") + " --check-cfg=cfg(zopro_g1_affine_offset) --check-cfg=cfg(zopro_affine_offset_diagnostics)";
    fs::write(format!("{R}/control-flags.txt"),&flags).unwrap();
    fs::write(format!("{R}/candidate-flags.txt"),format!("{flags} --cfg zopro_g1_affine_offset")).unwrap();
    for rel in ["driver.rs","support/perf.rs"] { let to=format!("{R}/{rel}");fs::create_dir_all(Path::new(&to).parent().unwrap()).unwrap();fs::copy(format!("{OLD}/{rel}"),to).unwrap(); }
    fs::write(format!("{R}/native.rs"),fs::read_to_string(format!("{OLD}/native.rs")).unwrap().replace(OLD,R)).unwrap();
    for label in ["control","candidate"] {
        let mut plan: Value = serde_json::from_str(&fs::read_to_string(format!("{OLD}/{label}-plan.json")).unwrap().replace(OLD,R)).unwrap();
        let newflags = fs::read_to_string(format!("{R}/{label}-flags.txt")).unwrap();
        for step in plan["steps"].as_array_mut().unwrap() {
            if step["environment"].is_object() { step["environment"]["RUSTFLAGS"]=json!(newflags); }
            if step["name"]=="candidate-core" {
                step["required_tests"]=json!(["msm::affine_offset_caller::affine_offset_integration::both_curves_signs_tails_storage_and_reuse","msm::affine_offset_caller::affine_offset_integration::owned_prefix_and_unsigned_g1_share_the_canonical_offset_contract","msm::affine_offset_caller::affine_offset_integration::every_small_stack_worker_executes_complete_callers"]);
            }
        }
        save(&format!("{label}-plan.json"),&plan);
    }
    for rel in ["prepare.rs","design.md","source-audit.json","driver.rs","support/perf.rs","native.rs","control-plan.json","candidate-plan.json","control-flags.txt","candidate-flags.txt"] {
        let p=format!("{R}/{rel}");source_pins.insert(p.clone(),hash(p));
    }
    save("build-pins.json",&json!(source_pins));
    println!("Prepared G1-only source: 173 exact originals, 3 edited originals, 2 added files. G2 exact; field body and module inherited exactly. No SDK execution.");
}
