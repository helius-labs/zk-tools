use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fs,
    path::{Path, PathBuf},
    process::Command,
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/split-redc-native176";
const Z: &str = "/home/ubuntu/src/zk-tools/zopro";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-06-split-redc-native176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn walk(p: &Path) -> Vec<PathBuf> {
    let mut out = vec![];
    for e in fs::read_dir(p).unwrap() {
        let p = e.unwrap().path();
        if p.is_dir() {
            out.extend(walk(&p));
        } else {
            out.push(p);
        }
    }
    out.sort();
    out
}
fn extract(s: &str, name: &str) -> String {
    let start = s.find(name).unwrap();
    let b = s.as_bytes();
    let brace = start + s[start..].find('{').unwrap();
    let mut depth = 0;
    for i in brace..b.len() {
        if b[i] == b'{' {
            depth += 1;
        }
        if b[i] == b'}' {
            depth -= 1;
            if depth == 0 {
                return s[start..=i].to_string();
            }
        }
    }
    panic!("no method end")
}
fn main() {
    assert!(!Path::new(D).exists());
    assert!(!Path::new(R).join("qualification.json").exists());
    let result = read(format!("{R}/build/result.json"));
    assert_eq!(result["status"], "complete");
    assert_eq!(result["steps"].as_array().unwrap().len(), 9);
    let pins = read(format!("{R}/build-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(h, &json!(hash(p)));
    }
    let audit = read(format!("{R}/source-audit.json"));
    assert_eq!(audit["source_files"], 177);
    assert_eq!(audit["exact_originals"], 173);
    assert_eq!(audit["recoverable_originals"], 176);
    for (name, h) in audit["hashes"].as_object().unwrap() {
        assert_eq!(h, &json!(hash(format!("{R}/source/{name}"))));
    }
    let mut receipts = vec![];
    let plan = read(format!("{R}/build-plan.json"));
    for s in plan["steps"].as_array().unwrap() {
        let n = s["name"].as_str().unwrap();
        let d = format!("{R}/build/{n}");
        let exit = read(format!("{d}/exit.json"));
        assert_eq!(exit["success"], true);
        assert_eq!(exit["returncode"], 0);
        assert_eq!(exit["stdout_sha256"], hash(format!("{d}/stdout.log")));
        assert_eq!(exit["stderr_sha256"], hash(format!("{d}/stderr.log")));
        let cmd = read(format!("{d}/command.json"));
        assert_eq!(
            &cmd["argv"].as_array().unwrap()[4..],
            s["argv"].as_array().unwrap()
        );
        receipts.push(json!({"name":n,"exit_sha256":hash(format!("{d}/exit.json"))}));
    }
    let child = fs::read_to_string(format!(
        "{R}/source/crates/zopro-bn254/src/split_redc_tests.rs"
    ))
    .unwrap();
    let original = fs::read_to_string(format!("{Z}/crates/zopro-bn254/src/ifma8.rs")).unwrap();
    let refmethod = extract(&child, "    #[inline(always)]\n    fn retained<const").replacen(
        "fn retained",
        "fn product_cios",
        1,
    );
    let oldmethod = extract(&original, "    #[inline(always)]\n    fn product_cios");
    assert_eq!(refmethod, oldmethod);
    let reffield = extract(&child, "    #[inline(always)]\n    fn retained_difference2")
        .replacen("fn retained_difference2", "pub fn difference2", 1)
        .replace("self.retained::<true>", "self.product::<true>");
    assert_eq!(
        reffield,
        extract(&original, "    #[inline(always)]\n    pub fn difference2")
    );
    let mut fields = vec![];
    for role in ["OFF", "ON"] {
        let b = plan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == format!("build-tests-{role}"))
            .unwrap();
        let a = b["argv"].as_array().unwrap();
        assert!(a.iter().any(|v| v == "overflow-checks=yes"));
        assert_eq!(
            a.iter().filter(|v| *v == "zopro_split_redc_planes").count(),
            if role == "ON" { 1 } else { 0 }
        );
        let run = plan["steps"]
            .as_array()
            .unwrap()
            .iter()
            .find(|s| s["name"] == format!("tests-{role}"))
            .unwrap();
        assert_eq!(run["required_tests"].as_array().unwrap().len(), 4);
        let stdout = fs::read_to_string(format!("{R}/build/tests-{role}/stdout.log")).unwrap();
        let stderr = fs::read_to_string(format!("{R}/build/tests-{role}/stderr.log")).unwrap();
        assert_eq!(
            stderr
                .matches("SPLIT_REDC_NATIVE_LANES_PER_CONTEXT=156096")
                .count(),
            4
        );
        for name in ["fq_mixed", "fq_uniform", "fr_mixed", "fr_uniform"] {
            assert!(stdout.contains(&format!("test ifma8::split_redc_tests::{name} ... ok")));
        }
        if role == "ON" {
            assert_eq!(run["unfiltered"], true);
            assert!(stdout.contains("39 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out"));
        } else {
            assert!(stdout.contains("4 passed; 0 failed; 0 ignored; 0 measured; 35 filtered out"));
        }
        let f = read(format!("{R}/field-{role}.json"));
        for k in ["source", "frozen"] {
            assert_eq!(f["sha256"], hash(f[k].as_str().unwrap()));
        }
        fields.push(f);
    }
    let qualification = json!({"status":"pass","workflow_commands":9,"source_files":177,"exact_originals":173,"recoverable_originals":176,"OFF_new_tests_passed":4,"OFF_inherited_tests_filtered":35,"ON_unfiltered_tests_passed":39,"new_oracle_lanes_per_build":624384,"new_oracle_lanes_across_both_builds":1248768,"test_reference_bodies_original_byte_exact":true,"overflow_checks":true,"fields":fields,"receipts":receipts,"native_timing":false,"new_profiles":0,"new_decodes":0,"new_proofs":0,"proof_ledger":30846,"production_changed":false,"adopted":false,"limitations":"Field arithmetic/native tests only. No actual point caller/codegen/latency/resource/portable/SDK/Gnark/AWS qualification; no source adoption."});
    save(format!("{R}/qualification.json"), &qualification);
    let out = Path::new(D).join("artifacts");
    fs::create_dir_all(&out).unwrap();
    let mut files = BTreeMap::new();
    for p in walk(Path::new(R)) {
        let rel = p.strip_prefix(R).unwrap().to_str().unwrap();
        let keep = if rel.starts_with("source/") {
            [
                "source/crates/zopro-bn254/src/ifma.rs",
                "source/crates/zopro-bn254/src/ifma8.rs",
                "source/crates/zopro-bn254/src/split_redc_tests.rs",
                "source/scripts/ifma8.py",
            ]
            .contains(&rel)
        } else {
            p.extension()
                .is_some_and(|x| ["rs", "md", "json", "log"].iter().any(|s| x == *s))
        };
        if keep {
            files.insert(rel.to_string(), fs::read_to_string(p).unwrap());
        }
    }
    let archive = json!({"files":files});
    save(format!("{R}/source-and-receipts.json"), &archive);
    let f = fs::File::create(out.join("source-and-receipts.json.gz")).unwrap();
    assert!(Command::new("gzip")
        .args(["-n", "-c"])
        .arg(format!("{R}/source-and-receipts.json"))
        .stdout(f)
        .status()
        .unwrap()
        .success());
    let back = Command::new("gzip")
        .arg("-dc")
        .arg(out.join("source-and-receipts.json.gz"))
        .output()
        .unwrap();
    assert!(back.status.success());
    assert_eq!(
        back.stdout,
        fs::read(format!("{R}/source-and-receipts.json")).unwrap()
    );
    let index = archive["files"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                format!("{:x}", Sha256::digest(v.as_str().unwrap().as_bytes())),
            )
        })
        .collect::<BTreeMap<_, _>>();
    save(out.join("archive-index.json"), &json!(index));
    save(out.join("qualification.json"), &qualification);
    for f in [
        "setup.rs",
        "qualify.rs",
        "first-four-rows.rs",
        "tests-body.rs",
        "design.md",
        "source-audit.json",
        "build-plan.json",
        "build-pins.json",
    ] {
        fs::copy(format!("{R}/{f}"), out.join(f)).unwrap();
    }
    let report = r#"# Split REDC coefficient planes: native field qualification

The candidate passes the native field correctness stage: **39 unfiltered ON tests**, four new OFF oracle tests, and generator consistency. Both field libraries are frozen by hash. **There is no caller or proof timing yet, and production is unchanged.**

The [mathematical screen](../2026-11-06-split-redc-planes176/README.md) motivates separating product and reduction coefficient accumulators for the first four eight-lane CIOS rows. Their low sum supplies the existing quotient/carry frontier; five coefficients merge before the original final row. The implementation retains the original final52/mixed48 step, normalization, fused correction, representation and all other runtime code. Four-lane and private-cfg OFF paths use the original rows. The six additional live vector coefficients may increase stack traffic; correctness does not establish their cost.

The isolated snapshot contains177 files:173 accepted originals are byte-exact and inverse edits recover all176 originals. Only the IFMA template/generated kernel and a generator test hook differ; one new test-only child is added. The existing official generator writes the eight-lane file and its check passes. The retained CIOS/difference test reference bodies are checked byte-for-byte against the originals. They are excluded from production libraries.

Each build checks624,384 native lane results across Fq/Fr, R256/R260, ordinary/sum2/original-complement difference2, all7^4 boundary tuples,4,096 random tuples, seven zero-padding lanes and eight-step chains. Every result agrees in its exact lazy limb representative with the retained implementation, independently agrees with Arkworks field arithmetic, has normalized digits and remains strictly below2p. These are the same oracle inputs under OFF and ON, not1,248,768 distinct mathematical cases. Both test builds enable overflow checks.

The prospective design's inherited-suite count33 was stale. Actual source has35 inherited tests plus four new tests: ON passes all39 with zero filtered; OFF passes the four new tests and intentionally filters35 inherited tests. No test or gate was removed to reconcile the count. All nine workflow commands succeed on their first execution, including immediate library freezes. No correctness repair, mathematical model rerun, native timing capture, recording or decode occurs in this stage.

The next stage must link the exact new OFF/ON field libraries into fresh accepted-source point callers, verify core/curve outputs and cfg identity, and inspect the actual emitted consumer code before declaring matched measurements. The source DAG is not a speed result. A positive caller result would still require all-shape SDK/Gnark/resource/portable qualification before adoption. No arbitrary width/layout/attribute or favorable-subset rescue is selected.

Accepted176 remains7.928660ms local1x2 and161.883899ms merge54; the inherited132.75MiB RSS is not a newly measured peak or minimum RAM. The [all-eight CPU/Metal table](../2026-11-06-split-redc-planes176/README.md#unchanged-full-proof-baseline) remains unchanged. The original CPU10× goal is active and unmet0/8; primary target3.7ms. Proof ledger remains30,846. No fresh SDK/Gnark proof, AWS/Aeglos, portability, pressure or mobile result is claimed.

[Qualification and frozen library identities](artifacts/qualification.json), [native design](artifacts/design.md), [kernel branch](artifacts/first-four-rows.rs), [new oracle](artifacts/tests-body.rs), [preparation/source recovery](artifacts/setup.rs), [review/publication source](artifacts/qualify.rs), [source audit](artifacts/source-audit.json), [build plan](artifacts/build-plan.json), [input pins](artifacts/build-pins.json), [source/receipt archive](artifacts/source-and-receipts.json.gz) and [member hashes](artifacts/archive-index.json). The archive includes all four changed/added text sources and all nine command receipts; no key, witness, proof, executable or library blob is redistributed.
"#;
    fs::write(Path::new(D).join("README.md"), report).unwrap();
    let mut sums = String::new();
    let published = walk(Path::new(D));
    for f in &published {
        sums.push_str(&format!(
            "{}  {}\n",
            hash(f),
            f.strip_prefix(D).unwrap().display()
        ));
    }
    fs::write(Path::new(D).join("SHA256SUMS"), sums).unwrap();
    save(
        format!("{R}/publication.json"),
        &json!({"status":"pass","report":D,"files":published.len()+1,"archive_members":files.len(),"roundtrip_exact":true,"production_changed":false}),
    );
    println!("PASS 9 commands; OFF4/ON39 unfiltered; exact references and 2 frozen libraries; published {}files/{}UTF8 members",published.len()+1,files.len());
}
