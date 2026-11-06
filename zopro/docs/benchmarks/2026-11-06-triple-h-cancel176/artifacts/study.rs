mod cancel;
mod original;
use ark_bn254::{Fr, G1Projective, G2Projective};
use ark_ec::{AffineRepr, CurveGroup, PrimeGroup};
use ark_ff::{AdditiveGroup, Field, PrimeField};
use original::*;
use serde_json::{json, Value};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-cancel176";
fn word(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
fn scalar(s: &mut u64) -> Fr {
    let mut bytes = [0; 32];
    for b in bytes.chunks_exact_mut(8) {
        b.copy_from_slice(&word(s).to_le_bytes());
    }
    Fr::from_le_bytes_mod_order(&bytes)
}
fn point<G: CurveGroup<ScalarField = Fr>>(s: Fr) -> Point<G::BaseField>
where
    G::BaseField: PointField,
{
    let a = G::generator().mul_bigint(s.into_bigint()).into_affine();
    match a.xy() {
        Some((x, y)) => Point::finite(x, y),
        None => Point::infinity(),
    }
}
fn costs(c: Cost) -> Value {
    json!({"ordinary_base_products":c.mul,"ordinary_base_squares":c.square,"fused_two_products":c.fused,"explicit_linear_operations":c.linear,"central_Fq_inversions":c.inverse,"elementary_products":c.elementary(),"body_IFMA_template_sites_per_scalar_instance":c.ifma_sites()})
}
fn field<G: CurveGroup<ScalarField = Fr>>(name: &str) -> Value
where
    G::BaseField: PointField,
{
    let edges = [
        Fr::ZERO,
        Fr::ONE,
        -Fr::ONE,
        Fr::from(2u64),
        -Fr::from(2u64),
        Fr::from(3u64),
        -Fr::from(3u64),
        Fr::from(5u64),
    ];
    let mut pool = edges.to_vec();
    let mut seed = 0x7d8e5e48f4578abfu64;
    for _ in 0..256 {
        pool.push(scalar(&mut seed));
    }
    let points = pool.iter().map(|&s| point::<G>(s)).collect::<Vec<_>>();
    let mut ordinary = 0;
    let mut guards = [0; 2];
    let mut opposite = 0;
    let mut check = |scalars: [Fr; 3], input: [Point<G::BaseField>; 3]| {
        let expected = point::<G>(scalars.into_iter().sum());
        let op = Ops::new();
        match cancel::batch(&[input], &op) {
            Ok(out) => {
                assert_eq!(out, [expected]);
                ordinary += 1;
                if input[0].x == input[1].x {
                    assert_eq!(scalars[0] + scalars[1], Fr::ZERO);
                    opposite += 1;
                }
            }
            Err(Guard::Infinity) => {
                assert!(input.iter().any(|p| p.infinity));
                assert_eq!(op.cost.get().inverse, 0);
                guards[0] += 1;
            }
            Err(Guard::SecondDenominator) => {
                let d = input[1].x - input[0].x;
                let n = input[1].y - input[0].y;
                assert_eq!(
                    d.square() * (input[0].x + input[1].x + input[2].x) - n.square(),
                    G::BaseField::ZERO
                );
                assert_eq!(op.cost.get().inverse, 0);
                guards[1] += 1;
            }
            Err(Guard::FirstDenominator) => panic!("new formula has no first-denominator guard"),
        }
    };
    for i in 0..8 {
        for j in 0..8 {
            for k in 0..8 {
                check(
                    [pool[i], pool[j], pool[k]],
                    [points[i], points[j], points[k]],
                );
            }
        }
    }
    for _ in 0..4096 {
        let ids: [usize; 3] = std::array::from_fn(|_| word(&mut seed) as usize % pool.len());
        check(ids.map(|i| pool[i]), ids.map(|i| points[i]));
    }
    let mut current = Fr::ONE;
    for _ in 0..1024 {
        let j = word(&mut seed) as usize % pool.len();
        let k = word(&mut seed) as usize % pool.len();
        let values = [current, pool[j], pool[k]];
        check(values, [point::<G>(current), points[j], points[k]]);
        current = values.into_iter().sum();
    }
    assert!(ordinary > 0 && opposite > 0 && guards.iter().all(|&n| n > 0));
    let mut batches = Vec::new();
    for n in [8usize, 16, 32, 64, 128] {
        let input = (0..n)
            .map(|i| {
                [
                    point::<G>(Fr::from((10 * i + 1) as u64)),
                    point::<G>(Fr::from((10 * i + 2) as u64)),
                    point::<G>(Fr::from((10 * i + 4) as u64)),
                ]
            })
            .collect::<Vec<_>>();
        let expected = (0..n)
            .map(|i| point::<G>(Fr::from((30 * i + 7) as u64)))
            .collect::<Vec<_>>();
        let new = Ops::new();
        assert_eq!(cancel::batch(&input, &new).unwrap(), expected);
        let old = Ops::new();
        assert_eq!(direct_batch(&input, &old).unwrap(), expected);
        let binary = Ops::new();
        assert_eq!(two_binary(&input, &binary).unwrap(), expected);
        let row = |op: &Ops<G::BaseField>| {
            let c = op.cost.get();
            assert_eq!(c.ifma_sites() % 8, 0);
            json!({"work":costs(c),"native_vector_IFMA_template_sites":c.ifma_sites()/8+500*c.inverse,"native_scalar_products_in_root_bundle":c.inverse,"native_scalar_inversions":c.inverse})
        };
        batches.push(json!({"triples":n,"new_H_only":row(&new),"old_dH":row(&old),"two_binary":row(&binary)}));
    }
    json!({"curve":name,"fixed_pool_scalars":pool.len(),"attempts":5632,"ordinary_results_checked":ordinary,"opposite_first_pair_computed":opposite,"fallback":{"infinity":guards[0],"H_zero":guards[1]},"batches":batches})
}
fn main() {
    assert!(!Path::new(R).join("analysis.json").exists());
    let fields = [
        field::<G1Projective>("G1/Fq"),
        field::<G2Projective>("G2/Fq2"),
    ];
    let out = json!({"formula":"H-only single denominator; cancel first slope and reconstruct final y algebraically","fields":fields,"BN254_attempts":11264,"extra_common_domain_batch_triples":496,"old_tests_filtered_inherited":3,"new_focused_tests":2,"new_native_IFMA_candidate":false,"new_performance_measurements":0,"new_proofs":0,"accepted_source_changed":false,"proof_ledger":30846,"limits":"Exact scalar field/curve oracles and chosen-template work counts only. No native SIMD kernel, record/packing/stack/workspace/caller/latency/resource eligibility.","goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/analysis.json"),
        serde_json::to_vec_pretty(&out).unwrap(),
    )
    .unwrap();
    println!("H-only formula verified11264BN254attempts plus496common-domain batch triples; no native candidate, timing or proofs");
}
