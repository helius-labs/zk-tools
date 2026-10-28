use serde_json::{json,Value};use std::{collections::BTreeMap,fs,path::Path};
const R:&str="/home/ubuntu/src/zopro-perf-work/msm-geometry";
fn main() {
    let r=Path::new(R);assert!(!r.join("interpretation.json").exists());
    let v:Value=serde_json::from_slice(&fs::read(r.join("analysis.json")).unwrap()).unwrap();let p=v["processes"].as_array().unwrap();assert_eq!(p.len(),3);
    for other in &p[1..] {assert_eq!(other["counts"],p[0]["counts"]);}
    let mut sums=BTreeMap::<String,BTreeMap<String,u64>>::new();
    for (key,s) in p[0]["streams"].as_object().unwrap() {
        let curve=key.split('/').nth(1).unwrap().to_owned();let out=sums.entry(curve).or_default();
        assert_eq!(s["pending_pushes"],s["executed_entries"]);
        assert!(s["queued_arrivals"].as_u64().unwrap()>=s["projective_queue_entries"].as_u64().unwrap());
        for field in ["windows","execute_calls","executed_entries","queued_arrivals","matched_destinations_over_flushes","matched_entries_over_flushes","free_entries_over_flushes","projective_queue_entries","projective_queue_flushes","projective_updates","queue_take_blocked","flush_length_below64","empty_bucket_assignments"] {
            *out.entry(field.into()).or_default()+=s[field].as_u64().unwrap_or(0);
        }
    }
    fs::write(r.join("interpretation.json"),serde_json::to_vec_pretty(&json!({
        "all_three_observed_count_maps_identical":true,
        "all_pending_pushes_executed":true,
        "per_proof_stream_totals":sums,
        "small_uniform_swap_supported":false,
        "reason":"Actual streams batch200/352; only26of7234flushes have fewer than64entries. Prior uniform8/16triple caller gains cannot be directly mapped to them.",
        "mixed_flush_research_supported":true,
        "qualification":"Queue matches occur often inside large ordinary batches, but snapshots revisit entries. A mixed formula must share the existing inversion and reduce actual later flushes/projective work after charging extra arithmetic/indexing/queue compaction/scratch. No speed or avoided-inversion count follows from this census.",
        "tree_limit":"Census includes weighted_sum and plain reductions. Weighted_sum requires binary halving and uses copied even-index points; substituting ternary reduce_layer changes its identity. Ideal triple slots are not interchangeable or validated opportunities.",
        "next_action":"Design and qualify mixed signed-borrowed binary/triple flush on actual200/352batch geometries, with sparse collisions and bounded resource-accounted metadata; require caller and fullSDK proof gates before adoption.",
        "SDK_requests":12,"independent_exports":3,"proof_ledger":12527,"production_changed":false,"adopted":false,"goal_achieved":false
    })).unwrap()).unwrap();
}
