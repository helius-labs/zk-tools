use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-g1-reverse-overlap";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-g1-reverse-overlap";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn archive(root: &Path, dir: &Path, out: &Path, index: &mut BTreeMap<String, String>) {
    for e in fs::read_dir(dir).unwrap() {
        let e = e.unwrap();
        let p = e.path();
        let rel = p.strip_prefix(root).unwrap();
        let n = e.file_name();
        let n = n.to_str().unwrap();
        let ty = e.file_type().unwrap();
        if ty.is_dir() {
            if n.starts_with("target") || rel == Path::new("candidate") || n == "profiles" {
                continue;
            }
            archive(root, &p, out, index);
        } else if ty.is_file() {
            let ext = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !["rs", "md", "json", "log", "txt", "proof", "public"].contains(&ext) {
                continue;
            }
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            index.insert(rel.display().to_string(), hash(dest));
        }
    }
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["next_prior_count"], 8310);
    assert_eq!(a["SDK_requests"], 0);
    assert_eq!(a["Gnark_exports"], 0);
    let pins = read(r.join("measurement-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let manifest = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in manifest["source_files_sha256"].as_object().unwrap() {
        assert_eq!(
            hash(Path::new("/home/ubuntu/src/zk-tools/zopro").join(p)),
            h.as_str().unwrap()
        );
    }
    assert_eq!(
        hash("/tmp/zopro-grouping-module/native/candidate-sdk"),
        "03e1260e89068c2bec37a2471c34e93628a9f081903e1b7eee9cab9c306aaf3d"
    );
    assert_eq!(
        read(r.join("native/core/tests.json"))["passed_tests"]
            .as_array()
            .unwrap()
            .len(),
        111
    );
    let closeout = json!({"status":"G1 streaming reverse-group overlap component complete","component_advances":a["component_advances"],"production_sources_checked_unchanged":170,"native_core_tests":111,"new_core_tests":2,"raw_tests":2,"cubic_tests":6,"field_source_qualification_inherited":true,"closing_pins_checked":pins.as_object().unwrap().len(),"latency_cases":40,"latency_stages":840,"profiles":4,"cycle_samples":a["cycle_samples"],"SDK_requests":0,"independent_gnark_exports":0,"next_prior_count":8310,"adopted":false,"goal_achieved":false});
    save(r.join("closeout.json"), &closeout);
    let mut md=format!("# Overlapping G1 streaming reverse groups\n\nComponent advancement: **{}**. Accepted production remains unchanged; no full-proof latency gain is claimed. The all-eight CPU10× objective remains active and unmet.\n\nThe candidate exposes the next group's slope before finishing the current group's point outputs in the actual G1 streaming reverse loop. It processes two consecutive groups in the original descending order and preserves the single inverse-chain order, all field formulas, forward records, scalar tails, output order, signed/canonical guards and scratch capacities. G2 and affine trees are unchanged. There is no second inverse chain, dual-CIOS helper, table, cache, execution plan, point copy, heap storage or annotation. Extra live vector temporaries may affect registers and stack. This tests instruction overlap rather than fewer mathematical additions.\n\n## Complete-batch caller\n\nRyzen9950X, CPU2, one serial thread, no Rayon pool. Forty cases: batch sizes8/9/16/17/24/25/200/203/352/355, unsigned/signed and two seed families. Each batch has unique scattered destinations. Repeated calls update real buckets, so denominators vary across calls. Independent expected outputs are initial+iterations×signed-input, including untouched buckets. Exact accepted copy controlsA/B and candidate execute in one native fat-LTO binary. Seven rotated/reversed paired blocks, three primes then reset,1024timed complete calls per stage. One timer brackets the entire stage; setup, reset, expected-output construction and checks are excluded. Mutable state is blackboxed per call. All modes use prewarmed scratch and identical fixture addresses. Timed allocations are zero.\n\n|Batch|Signed|Seed|A µs/batch|B µs/batch|Overlap µs/batch|Overlap/A|Overlap/B|Wins/A|Wins/B|\n|---|---|---:|---:|---:|---:|---:|---:|---:|---:|\n",a["component_advances"]);
    for case in read(r.join("cases.json")).as_array().unwrap() {
        let c = &a["cases"][case["name"].as_str().unwrap()];
        let m = &c["resource_medians"];
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.3}|{:.3}|{:.6}|{:.6}|{}|{}|",
            case["pairs"],
            case["signed"],
            case["seed"],
            m["control-a"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            m["control-b"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            m["overlap"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            c["pairs"]["control-a"]["geometric_ratio"].as_f64().unwrap(),
            c["pairs"]["control-b"]["geometric_ratio"].as_f64().unwrap(),
            c["pairs"]["control-a"]["wins"],
            c["pairs"]["control-b"]["wins"]
        )
        .unwrap();
    }
    md.push_str("\nTimes are medians of stage averages; ratios are geometric means of paired block ratios and need not equal ratios of displayed times. [Rows](artifacts/rows.json) retain every observation and CPU, faults, switches, lifetime RSS and record-capacity receipts. Whole-process component RSS is not incremental stack or minimum prover RAM. No successful capture was repeated or case excluded.\n\n## Frozen decision\n\nEvery identical-control B/A case must have median within factor1.03 and geometric ratio within factor1.05. Every candidate case must have geometric ratio≤1.02 against both controls. The fixed targeted signed200/203/352/355 aggregate over both seeds must improve against both controls with at least4/7 winning block aggregates. There is no minimum gain floor, per-block veto, phase-order sweep or partial promotion. A passing component supports a separate standard SDK trial, not adoption or proof-speed claims. A failure closes this candidate with all positives retained.\n\n");
    writeln!(md,"Calibration failures: `{}`.\n\nNonregression failures: `{}`.\n\nAggregate failures: `{}`.\n\nTargeted signed aggregates: `{}`.\n",a["calibration_failures"],a["nonregression_failures"],a["aggregate_failures"],a["targeted_signed_aggregate"]).unwrap();
    md.push_str("## Validation and limits\n\nAll111 native core tests pass, including two new independent complete-curve accumulation/reuse tests. They cover both canonical modes, signed/unsigned inputs, both inverse modes, all three record-storage modes, empty/one/two/odd/even full-group boundaries and scalar tails. Growing/shrinking batches retain bucket addresses and reuse scratch. All backing buckets match independent final curve points, including untouched destinations. Raw2/cubic6, generator and portable/no-IFMA consumer checks pass. Field primitives and accepted field qualification remain unchanged. Exact original function copies and fixture exports exist only under test/diagnostic cfg. Inverse source edits recover accepted files; all170 production sources and exact accepted SDK are unchanged.\n\nFour warmFIFO999Hzcycles:u profiles follow all840latency stages: signed200/352, seed1, originalA/candidate,131072calls each. Every profile samples one own PID/TID with zero lost records. They cover the full repeated caller. [Profile summaries](artifacts/profile-summary.json) retain every symbol period and sample count; [symbol listings](artifacts/codegen-symbols.txt) describe actual linked functions. Neither static code sizes nor sampled periods establish dynamic instruction counts, cache causes or removable proof walltime. Raw profiles remain local with [hashes](artifacts/raw-local-only-sha256.json). All source/library/consumer/tool/helper pins are frozen before capture and rechecked at close. No build, helper, hash, archive or verifier overlaps a capture.\n\nThis component produces0SDK/Gnark exports; the ledger stays8310. No new M5/AWS/Aeglos/phone/TEE/giant/swap/pressure/concurrency/throughput or minimum-RAM result is claimed. The [latest full SDK report](../2026-10-27-tree-kernel-split/README.md) retains all supplied M5 CPU/Metal comparisons for a different candidate, with0/8CPU10×rows. Earlier adaptive31.530µs9/9primary gains remain positive alongside unresolved qualification.\n\n[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [analysis](artifacts/analysis.json), [native tests](artifacts/native/core/tests.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    for rel in [
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm/stream_probe.rs",
    ] {
        let dest = d.join("artifacts/candidate").join(rel);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(r.join("candidate").join(rel), &dest).unwrap();
        index.insert(format!("candidate/{rel}"), hash(dest));
    }
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!(
        "Closed G1 reverse overlap;{}hashes;advances={};0SDK/Gnark;prior8310",
        index.len(),
        a["component_advances"]
    );
}
