use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-active-lane-inverse";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-active-lane-inverse";
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
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let p = entry.path();
        let rel = p.strip_prefix(root).unwrap();
        let name = entry.file_name();
        let name = name.to_str().unwrap();
        let ty = entry.file_type().unwrap();
        if ty.is_dir() {
            if name.starts_with("target") || rel == Path::new("candidate") || name == "profiles" {
                continue;
            }
            archive(root, &p, out, index);
        } else if ty.is_file() {
            let extension = p.extension().and_then(|e| e.to_str()).unwrap_or("");
            if !["rs", "md", "json", "log", "txt", "proof", "public"].contains(&extension) {
                continue;
            }
            let dest = out.join(rel);
            fs::create_dir_all(dest.parent().unwrap()).unwrap();
            fs::copy(&p, &dest).unwrap();
            index.insert(rel.display().to_string(), hash(&dest));
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
        112
    );
    let closeout = json!({"status":"active-chain short-layer inverse component complete","component_advances":a["component_advances"],"production_sources_checked_unchanged":170,"native_core_tests":112,"new_core_tests":3,"raw_tests":2,"cubic_tests":6,"field_source_qualification_inherited":true,"closing_pins_checked":pins.as_object().unwrap().len(),"latency_cases":40,"latency_stages":840,"profiles":4,"cycle_samples":a["cycle_samples"],"SDK_requests":0,"independent_gnark_exports":0,"next_prior_count":8310,"adopted":false,"goal_achieved":false});
    save(r.join("closeout.json"), &closeout);
    let mut md=format!("# Active-chain inversion in short addition trees\n\nComponent advancement: **{}**. Production remains unchanged. This measures serial arithmetic layers, not proof latency; the all-eight CPU10× objective remains active and unmet. Small reliable gains have no minimum cutoff.\n\nThe candidate starts from accepted source and changes only the ordinary affine-tree inverse when no full SIMD group exists and one to seven lanes are active. A scalar Montgomery batch inverse processes the active values using3(m−1)field multiplications and one existing field inverse. Inactive chains have value one and inverse one. Empty layers, full groups, scalar point formulas, exceptional compaction, streaming kernels, reduction topology and scheduling remain unchanged. Only fixed stack arrays are used; no table, cache, new execution plan or heap storage is added.\n\n## Actual-layer measurement\n\nRyzen9950X, CPU2, one serial thread and no Rayon pool. Both G1/G2; requested pairs1/2/3/4/5/6/7/8/9/16 and two seed families. Every fresh process cycles16 distinct dense curve-valid fixtures so inversion inputs vary across calls. Identical controlsA/B call an exact accepted layer copy; candidate calls the changed real layer. Seven rotated/reversed paired blocks, three primes per stage and4096calls per stage. Reset is outside each kernel timer; caller time includes reset, clock and loop overhead. All modes share prewarmed fixture addresses and scratch. Per-call timing overhead is retained. All40cases/840stages are included, with no successful rerun or posthoc count selection.\n\n|Curve|Pairs|Seed|A µs/layer|B µs/layer|Short µs/layer|Short/A|Short/B|Wins/A|Wins/B|\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n",a["component_advances"]);
    for case in read(r.join("cases.json")).as_array().unwrap() {
        let c = &a["cases"][case["name"].as_str().unwrap()];
        let m = &c["resource_medians"];
        writeln!(
            md,
            "|{}|{}|{}|{:.3}|{:.3}|{:.3}|{:.6}|{:.6}|{}|{}|",
            case["curve"].as_str().unwrap(),
            case["pairs"],
            case["seed"],
            m["control-a"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            m["control-b"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            m["short"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            c["pairs"]["control-a"]["geometric_ratio"].as_f64().unwrap(),
            c["pairs"]["control-b"]["geometric_ratio"].as_f64().unwrap(),
            c["pairs"]["control-a"]["wins"],
            c["pairs"]["control-b"]["wins"]
        )
        .unwrap();
    }
    md.push_str("\nTimes are medians of stage averages, ratios are geometric means of paired block ratios and can differ from ratios of displayed times. [Rows](artifacts/rows.json) retain kernel/caller time, CPU, faults, context switches, RSS and scratch capacities. Component RSS is not whole-prover memory or minimum RAM. Timed allocations are zero.\n\n## Frozen decision\n\nEvery identical-control B/A case must have median within factor1.03 and geometric ratio within factor1.05. Candidate geometric ratio must be≤1.02 against both controls in every case, including unchanged full cases. The changed short-case within-block aggregate must improve against both controls with at least4/7 winning block aggregates. There is no minimum gain floor, per-block veto, exclusion or threshold reselection. A component pass supports a separate standard SDK study; it cannot establish end-to-end speed. A failure closes this implementation.\n\n");
    writeln!(md,"Control failures: `{}`.\n\nNonregression failures: `{}`.\n\nAggregate failures: `{}`.\n\nChanged-short aggregates: `{}`.\n",a["calibration_failures"],a["nonregression_failures"],a["aggregate_failures"],a["changed_short_aggregate"]).unwrap();
    md.push_str("## Correctness and profiling\n\nAll112 native core tests pass, including three new tests. Inverse checks use independent per-element inversions across1024boundary/random samples for every active count. Complete-curve checks cover G1/G2, both inverse modes, dense and ragged exceptional cases, empty/short/full boundaries and scratch resets. Every backing point and count matches the accepted reference; live outputs also match independent scalar complete-curve additions. Raw2/cubic6, generator and portable/no-IFMA consumer checks pass. Field source and its accepted qualification are unchanged. The reference copy and public fixture are diagnostic-cfg-only. Initial overlay verification caught one stale copied source, reconciled against the current170-file manifest before candidate edits and native testing; no capture used that stale copy.\n\nAfter all latency, four separate warmFIFO999Hz cycles:u profiles capture candidate G1/G2 at one and seven pairs,65536calls each. They cover the full caller, not isolated timer brackets. Each capture samples one own PID/TID with zero lost records. [Profiles](artifacts/profile-summary.json) retain every symbol period and sample count. No sampled fraction is a removable proof walltime or causal cache measurement. No helper, build, hash, archive or verifier overlaps a capture. Closing checks rehash all pins and accepted production sources.\n\nNo new SDK/Gnark proof is produced: the independent proof ledger remains8310. No new AWS/Aeglos, Apple, phone, TEE, giant-circuit, swap, pressure, minimum-RAM, concurrency or throughput measurement is claimed. The [latest full SDK comparison](../2026-10-27-tree-kernel-split/README.md) contains all supplied M5 CPU/Metal shapes and remains0/8at CPU10×; it measures a different candidate. Its earlier adaptive31.530µs9/9positive is retained with its unresolved qualification.\n\n[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [edits](artifacts/edits.json), [analysis](artifacts/analysis.json), [native tests](artifacts/native/core/tests.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d.join("artifacts")).unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    for rel in [
        "crates/zopro-core/src/msm.rs",
        "crates/zopro-core/src/lib.rs",
        "crates/zopro-core/src/msm/layer_probe.rs",
    ] {
        let dest = d.join("artifacts/candidate").join(rel);
        fs::create_dir_all(dest.parent().unwrap()).unwrap();
        fs::copy(r.join("candidate").join(rel), &dest).unwrap();
        index.insert(format!("candidate/{rel}"), hash(dest));
    }
    save(d.join("artifact-index.json"), &json!(index));
    fs::write(d.join("README.md"), md).unwrap();
    println!("Closed active-lane inverse component, {} archived hashes; advances={};0SDK/Gnark;prior8310.",index.len(),a["component_advances"]);
}
