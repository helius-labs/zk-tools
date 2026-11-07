use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    fmt::Write as _,
    fs,
    path::{Path, PathBuf},
};
const R: &str = "/home/ubuntu/src/zopro-perf-work/g2-real-norm-z176";
const OUT: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-11-07-g2-real-norm-z176";
fn r(p: &str) -> PathBuf {
    Path::new(R).join(p)
}
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn save(p: impl AsRef<Path>, v: &Value) {
    assert!(!p.as_ref().exists());
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap();
}
fn prepare() {
    let mut pins = BTreeMap::new();
    for p in [
        "review-publish.rs",
        "review-publish",
        "analysis.json",
        "pins.json",
        "continuation-plan.json",
        "model.rs",
        "small_prime.rs",
        "design.md",
        "recipe.md",
        "recipe-final.md",
    ] {
        let p = r(p);
        pins.insert(p.display().to_string(), hash(&p));
    }
    for (name, out) in [
        ("compile-real-norm-model", "runs"),
        ("compile-real-norm-model", "continuation-runs"),
        ("qualify-and-count-real-norm", "continuation-runs"),
    ] {
        for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
            let p = r(&format!("{out}/{name}/{f}"));
            pins.insert(p.display().to_string(), hash(&p));
        }
    }
    save(r("review-pins.json"), &json!(pins));
    save(
        r("review-plan.json"),
        &json!({"version":1,"source_pins":r("review-pins.json"),"steps":[{"kind":"command","name":"review-counts-and-publish","argv":[r("review-publish"),PathBuf::from("run")],"timeout_seconds":300}]}),
    );
}
fn cost(vm: u64, vf: u64, sm: u64, ss: u64, sf: u64, i: u64) -> Value {
    json!({"VM":vm,"VF":vf,"SM":sm,"SS":ss,"SF":sf,"I":i,"vector_IFMA_proxy":100*vm+150*vf})
}
fn run() {
    for (p, h) in read(r("review-pins.json")).as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let pins = read(r("pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest="/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json";
    assert_eq!(
        hash(manifest),
        "3483b1e2d5655d5267bb7e2408f04ace3bbe9cd8e4ef492920f761a24d593318"
    );
    let source =
        fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-core/src/msm.rs").unwrap();
    for literal in [
        "let use_prefix = F::<C>::TREE_PREFIX && m >= 32;",
        "s.prefix_records.push([chains, norm]);",
        "let factor = record[0].mul(&inverses);",
        "inverses = inverses.mul(&record[1]);",
        "let norm = Fq::sum_of_products(&value.c0, &value.c0, &value.c1, &value.c1);",
    ] {
        assert!(source.contains(literal));
    }
    let fq2 = fs::read_to_string("/home/ubuntu/src/zk-tools/zopro/crates/zopro-bn254/src/fq2.rs")
        .unwrap();
    assert!(fq2.contains("Fq::sum_of_products(&a.c0, &b.c0, &(-a.c1), &b.c1)"));
    let a = read(r("analysis.json"));
    assert_eq!(a["algebra_passed"], true);
    assert_eq!(a["rows"].as_array().unwrap().len(), 22);
    let mut checked = 0;
    for row in a["rows"].as_array().unwrap() {
        let original = row["counts"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_u64().unwrap())
            .collect::<Vec<_>>();
        let max = *original.iter().max().unwrap();
        let k = if max <= 1 {
            0
        } else {
            64 - (max - 1).leading_zeros() as u64
        };
        let mut block_sum = 0;
        let mut carry_blocks = 0;
        let mut prefix_blocks = 0;
        let mut group_blocks = 0;
        let mut tail_sum = 0;
        let mut prefix_peak = 0;
        let mut group_peak = 0;
        let mut pair_peak = 0;
        let mut cofactor_peak = 0;
        for level in 0..k {
            let at = original
                .iter()
                .map(|&n| n.div_ceil(1u64 << level))
                .collect::<Vec<_>>();
            let pairs = at.iter().map(|n| n / 2).sum::<u64>();
            let full = pairs / 8;
            let tail = pairs % 8;
            block_sum += pairs.div_ceil(8);
            carry_blocks += (at.iter().filter(|&&n| n % 2 == 1).count() as u64).div_ceil(8);
            if pairs >= 32 {
                prefix_blocks += full;
                prefix_peak = prefix_peak.max(full);
            } else {
                group_blocks += full;
                group_peak = group_peak.max(full);
            }
            tail_sum += tail;
            pair_peak = pair_peak.max(pairs);
            cofactor_peak = cofactor_peak.max(pairs.div_ceil(8));
        }
        let out = original.iter().filter(|&&n| n != 0).count() as u64;
        let output_blocks = out.div_ceil(8);
        let real = if k == 0 {
            cost(0, 0, 0, 0, 0, 0)
        } else {
            cost(
                13 * block_sum + 8 * k - 1 + 4 * carry_blocks + 4 * output_blocks,
                6 * block_sum,
                1,
                1,
                0,
                1,
            )
        };
        let complex = if k == 0 {
            cost(0, 0, 0, 0, 0, 0)
        } else {
            cost(
                2 * block_sum + 2 * k,
                16 * block_sum + 14 * k - 2 + 4 * carry_blocks + 4 * output_blocks,
                4,
                2,
                2,
                1,
            )
        };
        let accepted = cost(
            7 * prefix_blocks + 8 * group_blocks + 5 * k,
            5 * (prefix_blocks + group_blocks),
            7 * tail_sum + k,
            0,
            5 * tail_sum,
            k,
        );
        assert_eq!(row["real_norm"], real);
        assert_eq!(row["generic_complex"], complex);
        assert_eq!(row["accepted_source"], accepted);
        let n = original.iter().sum::<u64>();
        let payload = &row["payload"];
        assert_eq!(payload["point_bytes"], 128 * n);
        assert_eq!(
            payload["source_required_retained_field_payload"],
            128 * n + 640 * prefix_peak + 2880 * group_peak
        );
        assert_eq!(payload["source_pair_index_logical_bytes"], 8 * pair_peak);
        assert_eq!(
            payload["real_point_cofactor_field_payload"],
            128 * n + 256 * cofactor_peak
        );
        assert_eq!(
            payload["complex_point_cofactor_field_payload"],
            128 * n + 512 * cofactor_peak
        );
        for (den, key) in [
            (&complex, "vector_IFMA_proxy_real_over_complex"),
            (&accepted, "vector_IFMA_proxy_real_over_source"),
        ] {
            let value = real["vector_IFMA_proxy"].as_f64().unwrap()
                / den["vector_IFMA_proxy"].as_f64().unwrap();
            assert!((row[key].as_f64().unwrap() - value).abs() < 1e-13);
        }
        assert!(
            real["vector_IFMA_proxy"].as_u64().unwrap()
                > accepted["vector_IFMA_proxy"].as_u64().unwrap()
        );
        assert!(
            real["vector_IFMA_proxy"].as_u64().unwrap()
                < complex["vector_IFMA_proxy"].as_u64().unwrap()
        );
        checked += 1;
    }
    let mut receipt_count = 0;
    let mut receipts = Vec::new();
    for (out, plan, limit) in [
        ("runs", "plan.json", 1),
        ("continuation-runs", "continuation-plan.json", 2),
    ] {
        for s in read(r(plan))["steps"]
            .as_array()
            .unwrap()
            .iter()
            .take(limit)
        {
            let name = s["name"].as_str().unwrap();
            let base = r(&format!("{out}/{name}"));
            let e = read(base.join("exit.json"));
            let cmd = read(base.join("command.json"));
            assert_eq!(
                &cmd["argv"].as_array().unwrap()[4..],
                s["argv"].as_array().unwrap().as_slice()
            );
            if out == "runs" {
                assert_eq!(e["success"], false);
            } else {
                assert_eq!(e["success"], true);
                assert_eq!(e["returncode"], 0);
            }
            for stream in ["stdout", "stderr"] {
                assert_eq!(
                    hash(base.join(format!("{stream}.log"))),
                    e[format!("{stream}_sha256")].as_str().unwrap()
                );
            }
            receipts.push(json!({"path":base,"command":cmd,"exit":e}));
            receipt_count += 1;
        }
    }
    assert_eq!(a["qualification"]["complete_outputs"], 580);
    assert_eq!(a["qualification"]["instrumented_level_checks"], 2658);
    assert_eq!(a["qualification"]["declines"], 14);
    assert_eq!(a["small_prime"]["pair_outputs"], 4896);
    assert_eq!(a["small_prime"]["carry_outputs"], 576);
    assert_eq!(a["small_prime"]["all1849_norm_zero_equivalences"], true);
    let decision = json!({"native_selected":false,"closed":true,"reason":"real-norm construction improves on the generic complex lift, but all22 vector-only work proxies exceed accepted arithmetic; the4096point case adds52.65% vector IFMA proxy for15% required retained field-payload saving, with unpriced scalar/inversion and conversion costs","not_a_measured_regression":true,"not_a_universal_lower_bound":true,"full_goal_achieved":false,"new_proofs":0});
    save(r("decision.json"), &decision);
    let review = json!({"passed":true,"count_and_payload_rows_reconstructed":checked,"pins_verified":pins.as_object().unwrap().len(),"command_receipts":receipts,"receipt_count":receipt_count,"source_branch_fragments_verified":true,"scope":"independent closed level geometry/count/payload and receipt/source checks; existing native-field and finite-prime arithmetic not rerun"});
    save(r("independent-review.json"), &review);
    assert!(!Path::new(OUT).exists());
    let art = Path::new(OUT).join("artifacts");
    fs::create_dir_all(&art).unwrap();
    for f in [
        "model.rs",
        "model-before-compile-repair.rs",
        "small_prime.rs",
        "design.md",
        "recipe.md",
        "recipe-final.md",
        "analysis.json",
        "pins.json",
        "bootstrap-pins.json",
        "plan.json",
        "continuation-pins.json",
        "continuation-plan.json",
        "review-publish.rs",
        "review-pins.json",
        "review-plan.json",
        "decision.json",
        "independent-review.json",
    ] {
        fs::copy(r(f), art.join(f)).unwrap();
    }
    for (out, name) in [
        ("runs", "compile-real-norm-model"),
        ("continuation-runs", "compile-real-norm-model"),
        ("continuation-runs", "qualify-and-count-real-norm"),
    ] {
        let dest = art.join(out).join(name);
        fs::create_dir_all(&dest).unwrap();
        for f in ["command.json", "exit.json", "stdout.log", "stderr.log"] {
            fs::copy(r(&format!("{out}/{name}/{f}")), dest.join(f)).unwrap();
        }
    }
    fs::copy(
        r("continuation-runs/result.json"),
        art.join("continuation-result.json"),
    )
    .unwrap();
    let mut report=String::from("# Real-norm common-scale G2 work screen\n\nThe algebra passes. This fixed recipe is **not selected for native implementation**: it improves on a generic complex-scale construction, but increases the vector arithmetic proxy against the accepted norm-based reducer in every declared case. This is an engineering decision from a work/resource screen, not a measured slowdown or a lower bound on other algorithms. Production is unchanged.\n\n## Construction and traversal\n\nFor dx_i in Fq2, let n_i=norm(dx_i), D=product(n_i) and q_i=product of the other norms, all in Fq. Then u_i=dy_i*conjugate(dx_i)*q_i satisfies u_i/D=dy_i/dx_i. The shared Jacobian lift is:\n\n    X'_i = u_i² − (X1_i+X2_i)D²\n    Y'_i = u_i(X1_i D²−X'_i) − Y1_i D³\n    Z'   = ZD\n\nReal Z stays in Fq. Cofactors and common squares/cubes are real; coordinate rescaling uses Fq2-by-Fq operations. The generic comparison uses complex D=product(dx_i), complex cofactors and the same traversal. No mathematical novelty or published CPU speed claim is asserted. The earlier [G1 caller experiment](../2026-11-07-common-z-callers176/README.md) remains closed; its failed gate is not rerun at a new endpoint.\n\nThe model uses eight prefix chains, a five-product butterfly, and one compact cofactor record per padded pair block. Its reverse pass recomputes denominators, completes cofactors and writes each pair to its original left slot. Pair inputs are disjoint. Odd carries rescale separately before forward compaction. This avoids a separate carry/cofactor-output buffer. Group boundaries, singleton survivors and final normalization are charged.\n\nAn earlier output-slot traversal was refined during source review, before any arithmetic or count execution; both designs are retained. Identity inputs, zero denominators/norms and late equal/opposite cases decline transactionally. The private inputs remain unchanged.\n\n## Units and accepted-source comparison\n\nVM is one logical eight-lane ordinary Fq product; VF is one fused sum/difference of two products. SM/SS/SF are scalar multiply/square/fused-two-product calls; I is one scalar inverse. **These units are kept separate.** The interpreter evaluates lanes using the unchanged scalar field library; it is not a new IFMA kernel or a latency benchmark.\n\nThe vector-only proxy100VM+150VF counts nominal IFMA sites for the accepted carry-cancel CIOS source. It excludes scalar work, inversion, shifts, correction, packing, memory traffic and scheduling. Its ratios are not speed ratios. The frozen field metadata confirms the carry-cancel flag.\n\nAccepted G2 already uses norm-based shared inversion. Full pair blocks cost7VM+5VF in prefix mode, or8VM+5VF below32pairs, plus the per-level bridge5VM+1SM+1I. Each scalar tail costs7SM+5SF. In particular, accepted x86 Fq2 scalar multiplication uses two fused products, not three ordinary multiplies. These are source-derived ordinary-pair counts with vector inversion enabled; the private production reducer and actual-witness branch populations are not executed here.\n\nThe following table retains all nineteen single-group lengths and three multiple-group populations. k is the accepted inversion count; the new recipe uses one inverse in every listed case. Scalar and inversion savings are fully listed in the JSON and are not priced as zero in a complete-work claim.\n\n|Case|Vector proxy: accepted / real norm / generic complex|Real/accepted proxy|I: accepted→real|Required field payload: accepted→real bytes|\n|---|---:|---:|---:|---:|\n");
    for row in a["rows"].as_array().unwrap() {
        writeln!(
            report,
            "|{}|{} / {} / {}|{:.4}|{}→{}|{}→{}|",
            row["name"].as_str().unwrap(),
            row["accepted_source"]["vector_IFMA_proxy"],
            row["real_norm"]["vector_IFMA_proxy"],
            row["generic_complex"]["vector_IFMA_proxy"],
            row["vector_IFMA_proxy_real_over_source"].as_f64().unwrap(),
            row["accepted_source"]["I"],
            row["real_norm"]["I"],
            row["payload"]["source_required_retained_field_payload"],
            row["payload"]["real_point_cofactor_field_payload"]
        )
        .unwrap();
    }
    report.push_str("\nAt4,096points the new recipe saves11inversions and reduces the generic complex vector proxy16.39%. Against accepted arithmetic its vector proxy rises52.65%, while required retained field buffers fall693,888→589,824bytes, about15.0%. That exchange does not justify a native whole-tree campaign for this recipe. It does not prove that inversions, scalar tails or cache effects could never change a particular timing. No selective small-size policy, threshold sweep or favorable native subset is selected.\n\n## Memory lifetimes\n\nNative source/type checks give32-byte Fq,64-byte Fq2 and128-byte point storage. There is no key-point layout saving. Accepted prefix records require640bytes per complete eight-pair block, and captured Group records require2,880bytes; their retained buffers coexist after different levels have used them. The new canonical record has256bytes, versus512for the generic complex record.\n\nTables include required point and retained prefix/Group/cofactor field payloads. The JSON separately reports pair-index and group metadata. Vec headers, allocator capacity rounding and stack/compiler temporaries are excluded. These are typed requirements and source lifetime counts, not measured native capacity, RSS, admission bounds or minimum RAM. Small and singleton-heavy populations can use more field-buffer space than accepted source. A shared-scale algorithm must also rescale untouched survivors; omitting that work would be incorrect.\n\n## Qualification and review\n\nThe new model checks580complete reductions and2,658instrumented level-count identities across real/complex constructions, four full-width-offset G2 walks, odd/even lengths, mixed/zero/singleton groups and three initial real scales. Complete generator-curve outputs agree, and inputs remain unchanged. Fourteen initial/late singular, identity and zero-scale cases decline. Native field arithmetic is shared with the complete group oracle; the model does not requalify that field.\n\nAn independent F43[u]/(u²+1) rational implementation checks4,896pair outputs and576carry identities for the real-norm construction. It uses direct all-other products, no native field/prefix/butterfly code, and exhausts all1,849norm-zero equivalences. It is a small-field algebra oracle, not BN254 SIMD or portable qualification.\n\nIndependent review reconstructs all22operation/payload rows from closed level geometry, checks source branch fragments, all176accepted source hashes and the three compile/model command receipts. An initial compile failed on a sum type annotation and iterator lifetimes; only that unexecuted model stage was repaired. The first successful arithmetic/count execution passes. Failed source/receipts are retained, and successful field, G1, core/prover, arithmetic, capture or decode stages are not rerun. No CPU profile or timing exists for this recipe.\n\n## Decision and accepted comparisons\n\nClose the fixed real-norm recipe without native integration. Retain its correct algebra and advantage over the generic complex lift. The next search should target substantial main-MSM work rather than treat inversion removal as a free reduction in total arithmetic. No key table/copy, duplicate solver plan, production pool, field rebuild or C/ASM backend is introduced.\n\nThe following latest accepted local32-worker full-proof observations and supplied historical M5 references are inherited. [Accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json), [full-proof evidence](../2026-11-06-window-batch-proof-capture176/README.md).\n\n|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|\n|---|---:|---:|---:|---:|---:|---:|\n");
    for (name, t, cpu, metal) in [
        ("1×2", 7.915095, 37., 24.),
        ("2×2", 11.169812, 59., 30.),
        ("4×2", 16.085882, 109., 48.),
        ("8×2", 24.517520, 189., 74.),
        ("12×2", 33.838626, 284., 101.),
        ("16×2", 40.083672, 335., 124.),
        ("24×2", 65.506198, 528., 176.),
        ("merge24", 65.200982, 507., 175.),
    ] {
        writeln!(
            report,
            "|{name}|{t:.6}|{cpu:.0}|{metal:.0}|{:.3}×|{:.3}×|{:.1}|",
            cpu / t,
            metal / t,
            cpu / 10.
        )
        .unwrap();
    }
    report.push_str("\nAccepted merge54 remains161.718956ms without a supplied M5 reference. All-eight CPU10× goal remains active and unmet,0/8. New SDK requests/proofs0; canonical distinct-proof ledger31,387unchanged. No AWS/Aeglos/Gnark comparison, portable/mobile, pressure, huge-circuit, swap, concurrency or minimum-RAM result is added.\n\n[Design](artifacts/design.md), [final traversal](artifacts/recipe-final.md), [interpreter](artifacts/model.rs), [independent finite-field oracle](artifacts/small_prime.rs), [counts and qualification](artifacts/analysis.json), [independent review](artifacts/independent-review.json), [decision](artifacts/decision.json), [source/library pins](artifacts/pins.json), [workflow](artifacts/continuation-plan.json), [checksums](SHA256SUMS). No actual point/witness/key/proof data or executable/library payload is published.\n");
    fs::write(Path::new(OUT).join("README.md"), report).unwrap();
    let mut members = Vec::new();
    fn walk(base: &Path, p: &Path, v: &mut Vec<String>) {
        for e in fs::read_dir(p).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(base, &p, v);
            } else {
                v.push(p.strip_prefix(base).unwrap().display().to_string());
            }
        }
    }
    walk(Path::new(OUT), Path::new(OUT), &mut members);
    members.sort();
    let sums = members
        .iter()
        .map(|p| format!("{}  {p}\n", hash(Path::new(OUT).join(p))))
        .collect::<String>();
    fs::write(Path::new(OUT).join("SHA256SUMS"), sums).unwrap();
    println!("PASS independent22count/payload rows,{}pins,3receipts; published{}files+SHA256SUMS; fixedrecipeclosed/no native timings/proofs",pins.as_object().unwrap().len(),members.len());
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("run") => run(),
        _ => panic!("prepare | run"),
    }
}
