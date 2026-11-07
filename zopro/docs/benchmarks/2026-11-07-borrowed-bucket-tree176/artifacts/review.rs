use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/borrowed-bucket-tree176";
fn load(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(p: impl AsRef<Path>) -> String {
    format!("{:x}", Sha256::digest(fs::read(p).unwrap()))
}
fn n(v: &Value, k: &str) -> usize {
    v[k].as_u64().unwrap() as usize
}
fn main() {
    let mut pins = 0;
    for file in ["preparation-pins.json", "pins.json"] {
        for (p, h) in load(format!("{R}/{file}")).as_object().unwrap() {
            assert_eq!(hash(p), h.as_str().unwrap());
            pins += 1;
        }
    }
    let mut receipts = 0;
    for (plan, dir) in [
        ("preparation-plan.json", "preparation-runs"),
        ("plan.json", "runs"),
    ] {
        let plan = load(format!("{R}/{plan}"));
        assert_eq!(load(format!("{R}/{dir}/result.json"))["status"], "complete");
        assert_eq!(
            load(format!("{R}/{dir}/completed.json"))
                .as_array()
                .unwrap()
                .len(),
            plan["steps"].as_array().unwrap().len()
        );
        for s in plan["steps"].as_array().unwrap() {
            let p = format!("{R}/{dir}/{}", s["name"].as_str().unwrap());
            let e = load(format!("{p}/exit.json"));
            assert_eq!(e["success"], true);
            assert_eq!(e["returncode"], 0);
            let c = load(format!("{p}/command.json"));
            let a = c["argv"].as_array().unwrap();
            let expected = s["argv"].as_array().unwrap();
            assert_eq!(&a[a.len() - expected.len()..], expected);
            for stream in ["stdout", "stderr"] {
                assert_eq!(
                    hash(format!("{p}/{stream}.log")),
                    e[format!("{stream}_sha256")].as_str().unwrap()
                );
            }
            receipts += 1;
        }
    }
    assert_eq!(receipts, 3);
    let analysis = load(format!("{R}/analysis.json"));
    assert_eq!(analysis["edge_cases"], 1157);
    assert_eq!(analysis["mathematical_checks_passed"], true);
    let mut rows = 0;
    let mut bands = 0;
    let mut levels = 0;
    let mut summaries = vec![];
    for fixture in analysis["fixtures"].as_array().unwrap() {
        assert_eq!(fixture["two_fullwidth_token_streams"], true);
        assert_eq!(fixture["whole_original_scalar_dot_product_matches"], true);
        assert_eq!(fixture["rows"].as_array().unwrap().len(), 20);
        let mut original_inv = 0;
        let mut candidate_inv = 0;
        let mut pairs = 0;
        let mut arrivals = 0;
        let mut baseline_tails = 0;
        let mut candidate_tails = 0;
        let mut pad_pairs = 0;
        let mut max_workspace = 0;
        let mut contexts = std::collections::BTreeSet::new();
        for row in fixture["rows"].as_array().unwrap() {
            assert!(contexts.insert((n(row, "window"), n(row, "part"))));
            assert!(n(row, "window") < 10 && n(row, "part") < 2);
            let old = &row["baseline"];
            let new = &row["candidate"];
            let mut direct_pairs = 0;
            let mut first_writes = 0;
            let mut population = 0;
            let mut expected_levels = vec![];
            for band in row["bands"].as_array().unwrap() {
                let id = n(band, "band");
                assert!(id < 8);
                let mut h = vec![];
                let mut l = vec![];
                let mut bucket_count = 0;
                for (k, v) in band["joint_high_low_population_histogram"]
                    .as_object()
                    .unwrap()
                {
                    let (a, b) = k.split_once(',').unwrap();
                    let a: usize = a.parse().unwrap();
                    let b: usize = b.parse().unwrap();
                    let f = v.as_u64().unwrap() as usize;
                    bucket_count += f;
                    direct_pairs += (a + b).saturating_sub(1) * f;
                    first_writes += usize::from(a + b > 0) * f;
                    population += (a + b) * f;
                    h.push((a, f));
                    l.push((b + usize::from(a > 0), f));
                }
                assert_eq!(bucket_count, 256);
                for (stage, lengths) in [("high", h), ("low+high-root", l)] {
                    let mut depth = 0;
                    loop {
                        let div = 1usize << depth;
                        let level_pairs = lengths
                            .iter()
                            .map(|(x, f)| x.div_ceil(div) / 2 * f)
                            .sum::<usize>();
                        if level_pairs == 0 {
                            break;
                        }
                        let slots = lengths
                            .iter()
                            .map(|(x, f)| x.div_ceil(div * 2) * f)
                            .sum::<usize>();
                        expected_levels.push(json!({"stage":stage,"band":id,"depth":depth,"pairs":level_pairs,"output_slots":slots,"full_groups":level_pairs/8,"tail_pairs":level_pairs%8}));
                        depth += 1;
                    }
                }
                bands += 1;
            }
            assert_eq!(row["bands"].as_array().unwrap().len(), 8);
            assert_eq!(Value::Array(expected_levels.clone()), row["levels"]);
            let inv = expected_levels.len();
            let total = expected_levels.iter().map(|v| n(v, "pairs")).sum::<usize>();
            let max_pairs = expected_levels
                .iter()
                .map(|v| n(v, "pairs"))
                .max()
                .unwrap_or(0);
            let max_slots = expected_levels
                .iter()
                .map(|v| n(v, "output_slots"))
                .max()
                .unwrap_or(0);
            let max_tail = expected_levels
                .iter()
                .map(|v| n(v, "tail_pairs"))
                .max()
                .unwrap_or(0);
            let workspace = 2 * max_slots * 128
                + (max_pairs / 8) * 640
                + max_tail * 320
                + max_pairs * 16
                + 256 * 128;
            assert_eq!(direct_pairs, total);
            assert_eq!(n(row, "direct_join_count"), total);
            assert_eq!(n(new, "generic_pairs"), total);
            assert_eq!(n(new, "inversion_requests"), inv);
            assert_eq!(n(new, "largest_level_pairs"), max_pairs);
            assert_eq!(n(new, "largest_compact_buffer_slots"), max_slots);
            assert_eq!(n(new, "largest_scalar_tail_pairs"), max_tail);
            assert_eq!(n(new, "conservative_group_payload_bytes"), workspace);
            let mut hist = BTreeMap::new();
            for (k, v) in old["batch_histogram"].as_object().unwrap() {
                hist.insert(k.parse::<usize>().unwrap(), v.as_u64().unwrap() as usize);
            }
            assert_eq!(hist.values().sum::<usize>(), n(old, "batches"));
            assert_eq!(
                hist.iter().map(|(k, v)| k * v).sum::<usize>(),
                n(old, "affine_updates")
            );
            assert_eq!(hist.get(&200).copied().unwrap_or(0), n(old, "full_batches"));
            assert_eq!(n(old, "arrivals"), population);
            assert_eq!(n(row, "arrivals"), population);
            assert_eq!(n(old, "first_writes"), first_writes);
            assert_eq!(
                n(old, "first_writes") + n(old, "affine_updates") + n(old, "projective_arrivals"),
                population
            );
            assert_eq!(
                n(old, "projective_accumulator_adds") + n(old, "finish_merges"),
                n(old, "projective_arrivals")
            );
            assert_eq!(
                n(old, "affine_updates")
                    + n(old, "projective_accumulator_adds")
                    + n(old, "finish_merges"),
                total
            );
            original_inv += n(old, "batches");
            candidate_inv += inv;
            pairs += total;
            arrivals += population;
            max_workspace = max_workspace.max(workspace);
            baseline_tails += hist.iter().map(|(k, v)| (k % 8) * v).sum::<usize>();
            candidate_tails += expected_levels
                .iter()
                .map(|v| n(v, "tail_pairs"))
                .sum::<usize>();
            pad_pairs += expected_levels
                .iter()
                .map(|v| (8 - n(v, "tail_pairs")) % 8)
                .sum::<usize>();
            levels += inv;
            rows += 1;
        }
        assert_eq!(n(fixture, "ordinary_baseline_inversions"), original_inv);
        assert_eq!(n(fixture, "ordinary_candidate_inversions"), candidate_inv);
        assert_eq!(n(fixture, "ordinary_baseline_generic_joins"), pairs);
        assert_eq!(n(fixture, "ordinary_candidate_generic_joins"), pairs);
        assert_eq!(n(fixture, "route_index_payload_bytes"), arrivals * 4);
        assert_eq!(
            n(fixture, "route_count_start_cursor_bytes_all10windows"),
            20 * (2 * 2048 * 3 + 1) * 8
        );
        let pass = candidate_inv < original_inv;
        assert_eq!(fixture["opportunity_passed"], pass);
        summaries.push(json!({"shape":fixture["shape"],"baseline_inversion_requests":original_inv,"candidate_inversion_requests":candidate_inv,"equal_real_generic_joins":pairs,"baseline_scalar_tail_pairs":baseline_tails,"candidate_scalar_tail_pairs":candidate_tails,"alternative_padded_vector_dummy_pairs":pad_pairs,"largest_group_requested_payload_bytes":max_workspace,"route_metadata_bytes":arrivals*4+20*(2*2048*3+1)*8,"necessary_opportunity_passed":pass}));
    }
    assert_eq!(rows, 40);
    assert_eq!(bands, 320);
    assert_eq!(
        analysis["necessary_opportunity_gate_passed"],
        summaries
            .iter()
            .all(|v| v["necessary_opportunity_passed"] == true)
    );
    let output = json!({"review_passed":true,"source_pin_occurrences":pins,"command_receipts":receipts,"fixture_rows":rows,"bands":bands,"levels":levels,"summaries":summaries,"new_proofs":0,"new_timings":0,"proof_ledger_count":31621,"production_changed":false,"limitations":"Separate histogram/ceil-division count implementation and receipt checks, not a second token, curve, Phi/Booth/grouping, native, timing or allocation oracle. Padding is a prospective native implementation cost, not executed code."});
    assert!(!Path::new(&format!("{R}/review.json")).exists());
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&output).unwrap(),
    )
    .unwrap();
    println!("review pass; {rows}rows/{bands}bands/{levels}levels/{receipts}receipts");
}
