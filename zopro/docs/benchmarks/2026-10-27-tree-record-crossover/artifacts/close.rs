use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fmt::Write, fs, path::Path};
const R: &str = "/tmp/zopro-tree-record-crossover";
const D: &str = "/home/ubuntu/src/zk-tools/zopro/docs/benchmarks/2026-10-27-tree-record-crossover";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn save(p: impl AsRef<Path>, v: &Value) {
    fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn main() {
    let r = Path::new(R);
    let d = Path::new(D);
    let a = read(r.join("analysis.json"));
    assert_eq!(a["latency_stages"], 1176);
    assert_eq!(a["next_prior_count"], 7560);
    assert_eq!(a["SDK_requests"], 0);
    assert_eq!(a["Gnark_exports"], 0);
    let pins = read(r.join("measurement-pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(p), h.as_str().unwrap());
    }
    let m = read("/tmp/zopro-grouping-module/production-source-manifest.json");
    for (p, h) in m["source_files_sha256"].as_object().unwrap() {
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
    let audit = json!({"status":"G2 layer service and structural crossover component closed","component_advances":a["component_advances"],"training_threshold_actual_pairs":a["training_threshold_actual_pairs"],"source_originals":170,"production_sources_checked_unchanged":170,"native_core_tests":112,"new_core_tests":2,"raw_tests":2,"cubic_tests":6,"new_complete_buffer_mode_comparisons":448,"field_source_qualification_inherited":true,"no_new_production_dispatcher":true,"closing_pins_checked":pins.as_object().unwrap().len(),"latency_processes":56,"latency_stages":1176,"profile_processes":4,"timed_allocation_calls":0,"SDK_requests":0,"independent_gnark_exports":0,"next_prior_count":7560,"adopted":false,"goal_achieved":false,"next_action":"Assess training-only fit and heldout observations without selecting a circuit/core from earlier full-SDK rows. Preserve all controls and prior24x2/54x1 positives. A passing component supports only a distinct structural caller study, not prover integration; a failed component must not be rerolled or narrowed posthoc."});
    save(r.join("closeout.json"), &audit);
    let mut md=format!("# G2 tree-layer service crossover\n\nTraining threshold in actual ordinary pairs: **{}**. Component advances: **{}**. This is a serial arithmetic component, not whole-proof latency. No production dispatcher or accepted source changes; the all-eight CPU10× objective remains active and unmet.\n\nThe [preceding full SDK study](../2026-10-27-g2-tree-records/README.md) retained24×2 and54→1 gains but failed its global gate. This experiment asks whether the original and prefix-record layer costs cross with ordinary-pair count, before proposing any structural dispatch. It does not reclassify that failed study or choose a circuit/core selector from its rows.\n\nThe previous candidate's arithmetic is factored through a compile-time boolean. One instantiation executes the original record body and one executes prefix/norm records. The default wrapper selects the same path as the prior candidate. Reversing the factor and native cfg hooks recovers that candidate; its existing audit recovers the accepted source. Field primitives, tail formulas, exceptional-pair handling, point order, reduction topology and scheduling remain unchanged. The new fixture module/reexport is native-cfg-only.\n\n## Fixed component caller\n\nCPU2, one serial thread, no Rayon pool. Requested pair counts1/4/7/8/9/15/16/17/32/64/128/256/512/1024, either one dense bucket or seven ragged mixed buckets with identity/doubling/opposite cases. Seeds0/1 use distinct generator offsets and strides. Actual ordinary-pair counts exclude exceptional pairs and are computed before timing. These are synthetic curve-valid fixtures, not captured key points or witness distributions.\n\nAll56 cases run seven paired blocks in one fresh process each, with identical original controlsA/B and prefix mode in rotated/reversed order. Three primes per stage;iterations=max(64,16384/requestedpairs). The same fixture, point addresses and scratch serve all modes. Reset occurs before each kernel timer; complete storage/count validation occurs after each stage. Per-call Instant bracketing adds observer cost, which is retained. Caller time includes reset/clock/loop work. Both record vectors are warmed and retained, so this caller does not measure per-mode peak memory savings. All JSON/output follows all21 stages.\n\n|Split|Requested pairs|Mixed|Ordinary pairs|A kernel µs|B kernel µs|Prefix kernel µs|Prefix/A|Prefix/B|Wins/A|Wins/B|\n|---|---:|---|---:|---:|---:|---:|---:|---:|---:|---:|\n",a["training_threshold_actual_pairs"],a["component_advances"]);
    let cases = read(r.join("cases.json"));
    for config in cases.as_array().unwrap() {
        let c = &a["cases"][config["name"].as_str().unwrap()];
        let s = &c["resource_medians"];
        writeln!(
            md,
            "|{}|{}|{}|{}|{:.3}|{:.3}|{:.3}|{:.6}|{:.6}|{}|{}|",
            config["split"].as_str().unwrap(),
            config["requested_pairs"],
            config["mixed"],
            c["ordinary_pairs"],
            s["control-a"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            s["control-b"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            s["prefix"]["kernel_ns_per_layer"].as_f64().unwrap() / 1000.,
            c["pairs"]["control-a"]["geometric_ratio"].as_f64().unwrap(),
            c["pairs"]["control-b"]["geometric_ratio"].as_f64().unwrap(),
            c["pairs"]["control-a"]["wins"],
            c["pairs"]["control-b"]["wins"]
        )
        .unwrap();
    }
    md.push_str("\nMedians summarize stage averages; ratios pair blocks geometrically and need not equal ratios of displayed medians. All stage values, caller cost, process CPU/counters and retained record capacities are in [rows](artifacts/rows.json) and [analysis](artifacts/analysis.json). None is a predicted proof speedup.\n\n## Training fit and heldout decision\n\nCalibration requires every case's identical-control medianB/A within factor1.03 and geometricB/A within factor1.05. All blocks remain included; no individual block veto or minimum positive-gain floor. The frozen training algorithm selects the smallest threshold from8/16/32/64/128/256/512/1024 ordinary pairs for which both layouts are represented and every training case at or above it improves against both controls with at least4/7 wins. If none qualifies, no universal crossover is claimed.\n\nHeldout data cannot change the fitted threshold. Every selected heldout case must improve against both controls in at least4/7 blocks and stay within2% against both. The offline rule's aggregate over all heldout cases must improve against both with at least4/7 winning block aggregates. Unselected cases use the corresponding original control ratio1 in that offline model; actual dispatch/branch cost is unmeasured. The rule is not implemented in the prover.\n\n");
    writeln!(
        md,
        "Calibration failures: `{}`.\n\nHeldout/speed failures: `{}`.\n",
        a["calibration_failures"], a["speed_or_nonregression_failures"]
    )
    .unwrap();
    md.push_str("[Training-only rule](artifacts/training-rule.json) retains all threshold candidates and training failures. All heldout outcomes and offline aggregates remain in analysis. There are no repeated captures, dropped cases, threshold reselections or partial promotions. A passing result supports only a subsequent distinct caller experiment; a failure does not erase individually positive measurements.\n\n## Validation, profiles and scope\n\nAll112 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. Two new tests perform448 complete-storage comparisons across56 fixtures, two inverse modes and four mode orders, plus reset/address/geometry checks. Expected live outputs use independent complete scalar curve formulas; every physical point-storage slot is also checked against the original layer. The prior candidate's independent tree oracle is retained. Field source and accepted qualification are inherited unchanged.\n\nEvery one of1,180 stages passes complete storage/count checks and zero timed allocation calls. Process affinity and TID remain fixed. Usage counters cover the caller, not isolated field instructions. Whole-process RSS includes fixture/reference copies, both retained record buffers, allocator and executable mappings; it is not prover scratch, per-mode savings or minimum RAM.\n\nAll56 latency processes precede four separate warm-gated999Hz cycle profiles: dense seed1, requested16and1024, originalA/prefix. Profiles cover the complete caller, including resets and timing-loop work. Every capture has one own PID/TID and zero lost records. Sampled-cycle shares are descriptive and are not removable wall-time fractions.\n\n|Profile|Samples|Lost|\n|---|---:|---:|\n");
    for (name, p) in a["profiles"].as_object().unwrap() {
        writeln!(md, "|{name}|{}|{}|", p["samples"], p["lost_records"]).unwrap();
    }
    writeln!(md,"\nMaximum whole-process RSS across these component processes: **{:.2}MiB**. This includes all diagnostic storage. Raw root-owned recordings remain local and hashed; decoded records/receipts are archived.\n",a["whole_process_maxrss_kib"].as_f64().unwrap()/1024.).unwrap();
    md.push_str("No helper, build, hash or archive overlaps CPU captures. Sources, libraries, executable, tools and protocol pins are checked before and after. **Zero SDK requests and zero Gnark exports** are generated; the proof ledger remains **7,560**. All170 production files and the exact accepted SDK are unchanged. No new M5 CPU/Metal, AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM, pressure, concurrency or throughput qualification is claimed. The preceding full SDK report remains authoritative for those measured shape comparisons.\n\n");
    let binary = read(r.join("native/probe/artifacts.json"));
    writeln!(
        md,
        "Component executable: `{}`; SHA-256 `{}`.\n",
        binary["binary"].as_str().unwrap(),
        binary["binary_sha256"].as_str().unwrap()
    )
    .unwrap();
    md.push_str("[Design](artifacts/design.md), [source audit](artifacts/source-audit.json), [native tests](artifacts/native/core/tests.json), [affinity receipts](artifacts/affinity.json), [raw hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).\n");
    fs::create_dir_all(d).unwrap();
    fs::write(d.join("README.md"), md).unwrap();
    fs::copy(
        "/tmp/zopro-grouping-module/production-source-manifest.json",
        r.join("accepted-source-manifest.json"),
    )
    .unwrap();
    let mut index = BTreeMap::new();
    archive(r, r, &d.join("artifacts"), &mut index);
    for (p, h) in &index {
        assert_eq!(&hash(d.join("artifacts").join(p)), h);
    }
    save(d.join("artifact-index.json"), &json!(index));
    println!(
        "{}archived hashes;0SDK/Gnark;prior7560unchanged;noadoption",
        index.len()
    );
}
fn archive(root: &Path, here: &Path, dest: &Path, index: &mut BTreeMap<String, String>) {
    let mut paths: Vec<_> = fs::read_dir(here)
        .unwrap()
        .map(|e| e.unwrap().path())
        .collect();
    paths.sort();
    for p in paths {
        let rel = p.strip_prefix(root).unwrap();
        let s = rel.display().to_string();
        if s.starts_with("target-") {
            continue;
        }
        if p.is_dir() {
            archive(root, &p, dest, index);
            continue;
        }
        if s.starts_with("candidate/")
            && ![
                "candidate/crates/zopro-core/src/lib.rs",
                "candidate/crates/zopro-core/src/msm.rs",
                "candidate/crates/zopro-core/src/msm/layer_probe.rs",
            ]
            .contains(&s.as_str())
        {
            continue;
        }
        if !["json", "jsonl", "rs", "md", "txt", "log"]
            .iter()
            .any(|x| p.extension().is_some_and(|e| e == *x))
        {
            continue;
        }
        let q = dest.join(rel);
        fs::create_dir_all(q.parent().unwrap()).unwrap();
        fs::copy(&p, &q).unwrap();
        let h = hash(&q);
        assert_eq!(h, hash(&p));
        index.insert(s, h);
    }
}
