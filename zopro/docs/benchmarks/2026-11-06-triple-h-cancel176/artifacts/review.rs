use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/triple-h-cancel176";
fn read(p: impl AsRef<Path>) -> Value {
    serde_json::from_slice(&fs::read(p).unwrap()).unwrap()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
#[derive(Clone, Default, PartialEq, Eq)]
struct Poly(BTreeMap<[u8; 6], i128>);
impl Poly {
    fn var(i: usize) -> Self {
        let mut e = [0; 6];
        e[i] = 1;
        Self(BTreeMap::from([(e, 1)]))
    }
    fn add(&self, b: &Self) -> Self {
        let mut out = self.0.clone();
        for (k, v) in &b.0 {
            let n = out.entry(*k).or_default();
            *n = n.checked_add(*v).unwrap();
        }
        out.retain(|_, v| *v != 0);
        Self(out)
    }
    fn scale(&self, n: i128) -> Self {
        Self(
            self.0
                .iter()
                .map(|(k, v)| (*k, v.checked_mul(n).unwrap()))
                .filter(|(_, v)| *v != 0)
                .collect(),
        )
    }
    fn sub(&self, b: &Self) -> Self {
        self.add(&b.scale(-1))
    }
    fn mul(&self, b: &Self) -> Self {
        let mut out = BTreeMap::<[u8; 6], i128>::new();
        for (a, x) in &self.0 {
            for (b, y) in &b.0 {
                let e = core::array::from_fn(|i| a[i].checked_add(b[i]).unwrap());
                let v = out.entry(e).or_default();
                *v = v.checked_add(x.checked_mul(*y).unwrap()).unwrap();
            }
        }
        out.retain(|_, v| *v != 0);
        Self(out)
    }
    fn pow(&self, n: usize) -> Self {
        let mut p = Self(BTreeMap::from([([0; 6], 1)]));
        for _ in 0..n {
            p = p.mul(self);
        }
        p
    }
    fn seal(&self) -> Value {
        let terms = self
            .0
            .iter()
            .map(|(m, c)| json!([m, c]))
            .collect::<Vec<_>>();
        let bytes = serde_json::to_vec(&terms).unwrap();
        json!({"terms":terms.len(),"sha256":hash(&bytes)})
    }
}
fn main() {
    assert!(!Path::new(R).join("review.json").exists());
    let pins = read(format!("{R}/pins.json"));
    for (p, h) in pins.as_object().unwrap() {
        assert_eq!(hash(&fs::read(p).unwrap()), h.as_str().unwrap());
    }
    assert_eq!(
        fs::read(format!("{R}/original.rs")).unwrap(),
        fs::read("/home/ubuntu/src/zopro-perf-work/triple-affine-screen/model.rs").unwrap()
    );
    let plan = read(format!("{R}/plan.json"));
    for step in plan["steps"].as_array().unwrap() {
        let n = step["name"].as_str().unwrap();
        let base = format!("{R}/runs/{n}");
        let e = read(format!("{base}/exit.json"));
        assert_eq!(e["success"], true);
        assert_eq!(e["returncode"], 0);
        for f in ["stdout", "stderr"] {
            assert_eq!(
                hash(&fs::read(format!("{base}/{f}.log")).unwrap()),
                e[format!("{f}_sha256")]
            );
        }
        assert_eq!(
            &read(format!("{base}/command.json"))["argv"]
                .as_array()
                .unwrap()[4..],
            step["argv"].as_array().unwrap()
        );
    }
    let tests = fs::read_to_string(format!("{R}/runs/algebra-tests/stdout.log")).unwrap();
    assert!(tests.contains("2 passed; 0 failed; 0 ignored; 0 measured; 3 filtered out"));
    let log = fs::read_to_string(format!("{R}/runs/algebra-tests/stderr.log")).unwrap();
    assert!(log.contains("attempts=5832ordinary=4128opposite=272guards=[919, 785]"));
    // An independent exact polynomial proof. Variables are a,b,c,e,r,s.
    let [a, b, c, e, r, s] = core::array::from_fn(Poly::var);
    let d = c.sub(&a);
    let n = e.sub(&b);
    let total = a.add(&c).add(&r);
    let h = d.pow(2).mul(&total).sub(&n.pow(2));
    let u = d.mul(&b.add(&s)).add(&n.mul(&r.sub(&a)));
    let offset = a.add(&c).sub(&r);
    let h2 = h.pow(2);
    let h3 = h.pow(3);
    let d2 = d.pow(2);
    let x = d2
        .mul(&u.pow(2))
        .sub(&n.mul(&u).mul(&h).scale(2))
        .add(&offset.mul(&h2));
    let old_x = d2
        .mul(&u)
        .sub(&n.mul(&h))
        .pow(2)
        .sub(&n.pow(2).mul(&h2))
        .add(&offset.mul(&d2).mul(&h2));
    let new_x = d2.mul(&x);
    assert!(old_x.sub(&new_x).0.is_empty());
    let old_y = u
        .mul(&n.pow(2))
        .mul(&h2)
        .sub(&d2.mul(&u).mul(&a.add(&c)).mul(&h2))
        .sub(&d2.mul(&u).mul(&x))
        .add(&n.mul(&x).mul(&h))
        .sub(&n.mul(&a).mul(&h3))
        .add(&b.mul(&d).mul(&h3));
    let y = d
        .mul(&u)
        .mul(&r.sub(&total.scale(2)))
        .mul(&h2)
        .sub(&d.mul(&u).mul(&x))
        .add(&d.mul(&n).mul(&u.pow(2)).mul(&h))
        .add(&b.add(&e).add(&s).mul(&h3));
    let new_y = d.mul(&y);
    assert!(old_y.sub(&new_y).0.is_empty());
    let a = read(format!("{R}/analysis.json"));
    assert_eq!(a["BN254_attempts"], 11264);
    let mut ordinary = 0;
    let mut infinity = 0;
    let mut hz = 0;
    let mut opposite = 0;
    let mut batchtrips = 0;
    for (i, f) in a["fields"].as_array().unwrap().iter().enumerate() {
        assert_eq!(f["attempts"], 5632);
        let o = f["ordinary_results_checked"].as_u64().unwrap();
        let inf = f["fallback"]["infinity"].as_u64().unwrap();
        let z = f["fallback"]["H_zero"].as_u64().unwrap();
        let p = f["opposite_first_pair_computed"].as_u64().unwrap();
        assert_eq!((o, inf, z, p), (5296, 224, 112, 42));
        assert_eq!(o + inf + z, 5632);
        ordinary += o;
        infinity += inf;
        hz += z;
        opposite += p;
        for (row, m) in f["batches"]
            .as_array()
            .unwrap()
            .iter()
            .zip([8u64, 16, 32, 64, 128])
        {
            assert_eq!(row["triples"], m);
            batchtrips += m;
            for (name, coeff, inv) in if i == 0 {
                [
                    ("new_H_only", [7, 2, 2, 18], 1),
                    ("old_dH", [8, 3, 3, 14], 1),
                    ("two_binary", [10, 2, 0, 12], 2),
                ]
            } else {
                [
                    ("new_H_only", [11, 0, 15, 49], 1),
                    ("old_dH", [13, 0, 21, 46], 1),
                    ("two_binary", [14, 0, 10, 30], 2),
                ]
            } {
                let v = &row[name];
                let w = &v["work"];
                let [mm, ss, ff, ll] = coeff;
                for (k, unit) in [
                    ("ordinary_base_products", mm),
                    ("ordinary_base_squares", ss),
                    ("fused_two_products", ff),
                    ("explicit_linear_operations", ll),
                ] {
                    assert_eq!(w[k], unit * m);
                }
                assert_eq!(w["central_Fq_inversions"], inv);
                assert_eq!(w["elementary_products"], m * (mm + ss + 2 * ff));
                let sites = m * (100 * (mm + ss) + 150 * ff);
                assert_eq!(w["body_IFMA_template_sites_per_scalar_instance"], sites);
                assert_eq!(
                    v["native_vector_IFMA_template_sites"],
                    sites / 8 + 500 * inv
                );
                assert_eq!(v["native_scalar_products_in_root_bundle"], inv);
                assert_eq!(v["native_scalar_inversions"], inv);
            }
        }
    }
    assert_eq!(
        (ordinary, infinity, hz, opposite, batchtrips),
        (10592, 448, 224, 84, 496)
    );
    let result = json!({"review_passed":true,"pins_checked":pins.as_object().unwrap().len(),"workflow_commands":4,"all_first_success":true,"original_model_byte_exact":true,"new_focused_tests":2,"old_tests_filtered":3,"symbolic_polynomial_certificate":{"variables":["a","b","c","e","r","s"],"exact_i128_coefficients_and_checked_exponents":true,"old_x_cleared_d2H2":old_x.seal(),"new_x_cleared_d2H2":new_x.seal(),"old_y_cleared_dH3":old_y.seal(),"new_y_cleared_dH3":new_y.seal(),"both_polynomial_differences_exactly_zero":true,"domain":"finite inputs with d,H nonzero; opposite first pair extension checked separately by algebra/toy/BN oracles"},"toy_attempts":5832,"toy_ordinary":4128,"toy_opposite_computed":272,"toy_guards":[919,785],"BN254_attempts":11264,"BN254_ordinary_checked":ordinary,"BN254_fallbacks":infinity+hz,"BN254_opposite_computed":opposite,"extra_common_domain_batch_triples":batchtrips,"all_field_cost_ledgers_reproduced":true,"native_candidate_or_speed_claim":false,"next":"A separate native complete unweighted G1 reduction screen is justified; no G2 blanket switch or weighted-tree replacement. Implement exact H-only formula with checked inverse and deferred compaction, qualify all input/tail/exception/lifetime cases before performance.","new_proofs":0,"goal_status":"active/unmet0/8"});
    fs::write(
        format!("{R}/review.json"),
        serde_json::to_vec_pretty(&result).unwrap(),
    )
    .unwrap();
    println!("Independent review PASS: exact cleared-denominator polynomial identities/11264BNattempt receipts/5832toy/10work rows/4commands; no native performance claim");
}
