pub use zopro_bn254::{CurveConfig, G2Config};
use zopro_bn254::{pairing, Fq, Fq12, Fq2, Fq6, G1Affine, G2Affine};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::{BTreeMap, BTreeSet}, fs, path::{Path, PathBuf}};
mod prepared;
mod packed;
const R: &str = "/home/ubuntu/src/zopro-perf-work/three-lane-final-exp176";
const OLD: &str = "/home/ubuntu/src/zopro-perf-work/three-lane-miller176";
const F: &str = "/home/ubuntu/src/zopro-perf-work/g2-vector-sign-default175/frozen/candidate";
fn r(p: &str) -> PathBuf { Path::new(R).join(p) }
fn read(p: impl AsRef<Path>) -> Value { serde_json::from_slice(&fs::read(p).unwrap()).unwrap() }
fn hash(p: impl AsRef<Path>) -> String { format!("{:x}", Sha256::digest(fs::read(p).unwrap())) }
fn save(p: impl AsRef<Path>, v: &Value) { assert!(!p.as_ref().exists()); fs::write(p, serde_json::to_vec_pretty(v).unwrap()).unwrap(); }
fn fq(v: &Value) -> Fq { Fq::from_canonical(serde_json::from_value(v.clone()).unwrap()).unwrap() }
fn fq12(v: &Value) -> Fq12 {
    let c: [Fq2; 6] = std::array::from_fn(|i| Fq2::new(fq(&v[2*i]), fq(&v[2*i+1])));
    Fq12::new(Fq6::new(c[0], c[2], c[4]), Fq6::new(c[1], c[3], c[5]))
}
fn words(v: Fq12) -> Vec<[u64; 4]> {
    [v.c0.c0, v.c1.c0, v.c0.c1, v.c1.c1, v.c0.c2, v.c1.c2].iter().flat_map(|q| [q.c0.to_canonical(), q.c1.to_canonical()]).collect()
}
struct Fixture { points: [G1Affine; 3], prepared: [prepared::G2Prepared; 3], miller: Fq12, final_value: Fq12 }
fn fixtures() -> Vec<Fixture> {
    read(Path::new(OLD).join("synthetic-fixtures.json"))["fixtures"].as_array().unwrap().iter().map(|row| {
        let points = std::array::from_fn(|i| G1Affine::new(fq(&row["points"][i]["g1"][0]), fq(&row["points"][i]["g1"][1])));
        let q: [G2Affine; 3] = std::array::from_fn(|i| G2Affine::new(
            Fq2::new(fq(&row["points"][i]["g2"][0]), fq(&row["points"][i]["g2"][1])),
            Fq2::new(fq(&row["points"][i]["g2"][2]), fq(&row["points"][i]["g2"][3]))));
        Fixture { points, prepared: q.map(|q| prepared::G2Prepared::new(&q)), miller: fq12(&row["miller"]), final_value: fq12(&row["final"]) }
    }).collect()
}
fn random(seed: &mut u64) -> Fq {
    let mut bytes = [0u8; 32];
    for out in bytes.chunks_exact_mut(8) { *seed ^= *seed << 13; *seed ^= *seed >> 7; *seed ^= *seed << 17; out.copy_from_slice(&seed.to_le_bytes()); }
    Fq::from_le_bytes_mod_order(&bytes)
}
fn raw(seed: &mut u64) -> Fq12 {
    let c: [Fq2; 6] = std::array::from_fn(|_| Fq2::new(random(seed), random(seed)));
    Fq12::new(Fq6::new(c[0], c[2], c[4]), Fq6::new(c[1], c[3], c[5]))
}
fn qualify() -> Value {
    let fixtures = fixtures();
    assert_eq!(fixtures.len(), 16);
    let mut checks = BTreeMap::<&str, usize>::new();
    let mut verify_batch = |values: [Fq12; 4], expected: Option<[Fq12; 4]>| {
        let t = packed::T::new(values);
        let inv = t.inverse().unwrap().get();
        assert_eq!(std::array::from_fn::<_, 4, _>(|i| values[i] * inv[i]), [Fq12::ONE; 4]);
        *checks.entry("inverse_identity_outputs").or_default() += 4;
        assert_eq!(t.conjugate().get(), values.map(|v| v.conjugate()));
        *checks.entry("conjugate_outputs").or_default() += 4;
        for power in 0..12 {
            assert_eq!(t.frobenius(power).get(), values.map(|v| v.frobenius(power)));
            *checks.entry("frobenius_outputs").or_default() += 4;
        }
        let easy = t.easy_part().unwrap();
        let e = easy.get();
        for v in e { assert_eq!(v * v.conjugate(), Fq12::ONE); assert_eq!(v.frobenius(4) * v, v.frobenius(2)); }
        *checks.entry("easy_subgroup_identities").or_default() += 8;
        assert_eq!(easy.cyclotomic_square().get(), e.map(|v| v.square()));
        *checks.entry("cyclotomic_vs_general_square_outputs").or_default() += 4;
        let expected = expected.unwrap_or_else(|| values.map(|v| pairing::final_exponentiation(v).unwrap()));
        assert_eq!(t.final_exponentiation().unwrap().get(), expected);
        *checks.entry("complete_final_exponentiation_outputs").or_default() += 4;
    };
    for base in (0..16).step_by(4) {
        verify_batch(std::array::from_fn(|i| fixtures[base+i].miller), Some(std::array::from_fn(|i| fixtures[base+i].final_value)));
    }
    let mut seed = 0xf1ca937d52b4e607;
    for _ in 0..16 { verify_batch(std::array::from_fn(|_| raw(&mut seed)), None); }
    let zero = Fq12::new(Fq6::ZERO, Fq6::ZERO);
    for mask in 1usize..16 {
        let values = std::array::from_fn(|i| if mask >> i & 1 != 0 { zero } else { fixtures[i+3].miller });
        assert!(packed::T::new(values).inverse().is_none());
        assert!(packed::T::new(values).easy_part().is_none());
        assert!(packed::T::new(values).final_exponentiation().is_none());
    }
    // Multiplicativity on new combined arguments, reusing the individual
    // cached final values rather than rerunning their successful oracles.
    let mut unique = BTreeMap::new();
    for i in 0..16 { for j in 0..16 {
        let k = (3*i+5*j+7)%16;
        let input = fixtures[i].miller * fixtures[j].miller * fixtures[k].miller;
        let expected = fixtures[i].final_value * fixtures[j].final_value * fixtures[k].final_value;
        if let Some(previous) = unique.insert(words(input), expected) { assert_eq!(previous, expected); }
    }}
    let known = fixtures.iter().map(|f| (words(f.miller), f.final_value)).collect::<BTreeMap<_, _>>();
    let mut new_combined_oracles = 0;
    let mut reused_combined_oracles = 0;
    for (input, expected) in &unique {
        if let Some(cached) = known.get(input) {
            assert_eq!(cached, expected);
            reused_combined_oracles += 1;
        } else {
            let fields = json!(input); assert_eq!(pairing::final_exponentiation(fq12(&fields)), Some(*expected));
            new_combined_oracles += 1;
        }
    }
    for f in &fixtures {
        let pairs = std::array::from_fn(|i| (f.points[i], &f.prepared[i]));
        assert_eq!(packed::fully_packed(&pairs), Some(f.final_value));
    }
    let mut pools = Vec::new();
    for workers in [1usize, 16, 32] {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(workers).stack_size(512*1024).build().unwrap();
        let outputs = pool.broadcast(|context| {
            let i = (context.index()+3)%16;
            let f = &fixtures[i];
            assert_eq!(rayon::current_num_threads(), workers);
            let pairs = std::array::from_fn(|j| (f.points[j], &f.prepared[j]));
            assert_eq!(packed::fully_packed(&pairs), Some(f.final_value));
            context.index()
        });
        assert_eq!(outputs.into_iter().collect::<BTreeSet<_>>(), (0..workers).collect());
        pools.push(json!({"workers":workers,"distinct_workers_checked":workers,"stack_bytes":512*1024}));
    }
    json!({"passed":true,"field_checks":checks,"zero_lane_masks":15,"zero_checks_per_mask":3,"cached_complete_Miller_finish_outputs":16,"new_combined_homomorphism_arguments":new_combined_oracles,"reused_combined_homomorphism_arguments":reused_combined_oracles,"homomorphism_individual_oracles_reused":true,"new_raw_final_exponentiation_oracles":64,"pools":pools,"all_checks_coordinator_stack_bytes":512*1024,"packed_payload_bytes":std::mem::size_of::<packed::T>(),"field_library_rebuilt":false,"old_native_suites_replayed":false,"new_SDK_requests":0,"new_proofs":0,"new_timings":0,"candidate_ready_for_component_timing":true})
}
fn prepare() {
    let mut pins = BTreeMap::new();
    for p in [r("design.md"), r("setup.rs"), r("source-copy-review.json"), r("prepared.rs"), r("packed.rs"), r("packed/finish.rs"), r("packed/frob6.rs"), r("packed/frob12.rs"), r("qualification.rs"), r("qualify"), Path::new(OLD).join("synthetic-fixtures.json")] { pins.insert(p.display().to_string(), hash(&p)); }
    for entry in fs::read_dir(F).unwrap() { let p = entry.unwrap().path(); if p.extension().and_then(|s| s.to_str()) == Some("rlib") { pins.insert(p.display().to_string(), hash(&p)); } }
    let old = read(Path::new(OLD).join("capture-pins.json"));
    for name in ["packed.rs", "prepared.rs", "synthetic-fixtures.json"] { let p = Path::new(OLD).join(name); assert_eq!(hash(&p), old[p.display().to_string()].as_str().unwrap()); pins.insert(p.display().to_string(), hash(&p)); }
    save(r("qualification-pins.json"), &json!(pins));
    save(r("qualification-plan.json"), &json!({"version":1,"source_pins":r("qualification-pins.json"),"steps":[{"kind":"command","name":"packed-complete-final-exponentiation-qualification","argv":[r("qualify"),PathBuf::from("check")],"timeout_seconds":300}]}));
    println!("Qualified-source and immutable cache/library pins prepared");
}
fn main() {
    match std::env::args().nth(1).as_deref() {
        Some("prepare") => prepare(),
        Some("check") => {
            for(p,h)in read(r("qualification-pins.json")).as_object().unwrap(){assert_eq!(hash(p),h.as_str().unwrap());}
            let result = std::thread::Builder::new().stack_size(512*1024).spawn(qualify).unwrap().join().unwrap();
            save(r("qualification.json"), &result); println!("{}", result);
        }, _ => panic!("prepare | check"),
    }
}
