use ark_ff::{Field, PrimeField};
use serde_json::json;
use std::{
    fs,
    path::Path,
    sync::atomic::{AtomicBool, AtomicU64},
};
use zopro_bn254::Fq;
mod bulk;
mod fixture;
const R: &str = "/home/ubuntu/src/zopro-perf-work/root-sharing-feasibility176";
fn reference(x: Fq) -> ark_bn254::Fq {
    ark_bn254::Fq::from_le_bytes_mod_order(
        &x.to_canonical()
            .into_iter()
            .flat_map(u64::to_le_bytes)
            .collect::<Vec<_>>(),
    )
}
fn check(x: Fq, y: Fq) {
    assert_eq!(x * y, Fq::ONE);
    assert!(y.is_reduced());
    assert_eq!(
        y.to_canonical(),
        reference(x).inverse().unwrap().into_bigint().0
    );
}
fn scalar_batch(a: &[Fq]) -> (Vec<Fq>, usize, usize) {
    if a.is_empty() {
        return (vec![], 0, 0);
    }
    let mut products = vec![a[0]];
    let mut m = 0;
    for x in &a[1..] {
        products.push(*products.last().unwrap() * *x);
        m += 1;
    }
    let mut inverse = products.last().unwrap().inverse().unwrap();
    let mut out = vec![Fq::ZERO; a.len()];
    for i in (1..a.len()).rev() {
        out[i] = inverse * products[i - 1];
        inverse = inverse * a[i];
        m += 2;
    }
    out[0] = inverse;
    (out, m, 1)
}
#[repr(C, align(64))]
struct Slot {
    payload: [AtomicU64; 4],
    state_and_generation: AtomicU64,
}
#[repr(C, align(64))]
struct MaskWord(AtomicU64);
#[repr(C, align(64))]
struct Leader(AtomicBool);
#[repr(C)]
struct State {
    slots: Vec<Slot>,
    pending: Vec<MaskWord>,
    leader: Leader,
}
fn main() {
    let r = Path::new(R);
    assert!(!r.join("oracle.json").exists());
    let a = fixture::fixture();
    let mut configurations = 0;
    let mut zero_rejections = 0;
    for n in [1, 16, 32] {
        for row in &a {
            let inverse = bulk::bulk(row, n).unwrap();
            for i in 0..n {
                check(row[i], inverse[i]);
                configurations += 1;
            }
            assert!(inverse[n..].iter().all(|x| x.is_zero()));
        }
        // Every possible invalid-root position; zero never becomes an Fp inverse.
        for position in 0..n {
            let mut bad = [Fq::ONE; 32];
            bad[position] = Fq::ZERO;
            assert!(bulk::bulk(&bad, n).is_none());
            zero_rejections += 1;
        }
    }
    assert_eq!(configurations, 12544);
    assert_eq!(zero_rejections, 49);
    let mut general_cases = 0;
    let mut general_outputs = 0;
    let mut counts = vec![];
    for b in 0..=96 {
        for pattern in 0..8 {
            let inputs = (0..b)
                .map(|i| a[(b + 31 * pattern) % fixture::ROWS][i % 32])
                .collect::<Vec<_>>();
            let (outputs, mul, inversions) = scalar_batch(&inputs);
            assert_eq!(mul, if b == 0 { 0 } else { 3 * (b - 1) });
            assert_eq!(inversions, usize::from(b > 0));
            for (&x, y) in inputs.iter().zip(outputs) {
                check(x, y);
                general_outputs += 1;
            }
            general_cases += 1;
        }
        counts.push(json!({"roots":b,"scalar_multiplies":if b==0{0}else{3*(b-1)},"scalar_inversions":usize::from(b>0)}));
    }
    assert_eq!(general_cases, 776);
    assert_eq!(general_outputs, 37248);
    assert_eq!(std::mem::size_of::<Slot>(), 64);
    assert_eq!(std::mem::size_of::<MaskWord>(), 64);
    assert_eq!(std::mem::size_of::<State>(), 128);
    let conceptual=( [1usize,16,32,96] ).map(|workers|json!({"workers":workers,"slot_bytes":64*workers,"mask_bytes":64*workers.div_ceil(64),"inline_state_bytes":128,"total_requested_bytes_without_allocator_rounding":128+64*workers+64*workers.div_ceil(64),"leader_arrays_payload_bytes_if_three_Fq_arrays_and_usize_ids":104*workers,"solo_can_omit_state":workers==1}));
    let result = json!({"passed":true,"bulk_root_output_configurations":configurations,"bulk_zero_rejections":zero_rejections,"general_batch_cases":general_cases,"general_batch_root_outputs_checked":general_outputs,"general_batch_counts":counts,"fixture_rows":fixture::ROWS,"fixture_hash":fixture::hash(&a),"fixture_input_bytes":fixture::ROWS*32*32,"inverse_lookup_table":false,"native_field_rebuilt":false,"broker_implemented":false,"concurrency_or_cancellation_qualified":false,"conceptual_state_only":conceptual,"bulk_source_counts":{"1":{"vector_mul":0,"scalar_mul":0,"scalar_inverse":1},"16":{"vector_mul":8,"scalar_mul":1,"scalar_inverse":1},"32":{"vector_mul":14,"scalar_mul":1,"scalar_inverse":1}},"new_proofs":0,"proof_ledger":30846});
    fs::write(
        r.join("oracle.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("PASS12544bulk outputs/49zero rejections/776general batch cases/37248general outputs; state sizing only, no broker or timing");
}
