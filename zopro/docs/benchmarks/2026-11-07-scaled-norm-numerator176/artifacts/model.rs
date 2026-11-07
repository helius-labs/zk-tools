use ark_bn254::{Fq, Fq2, Fr, G2Affine, G2Projective};
use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, Field, PrimeField, Zero};
use num_bigint::BigUint;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
const R: &str = "/home/ubuntu/src/zopro-perf-work/scaled-norm-numerator176";
fn big(v: u64) -> BigUint {
    BigUint::from(v)
}
fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}
fn random(s: &mut u64, modulus: &BigUint) -> BigUint {
    let mut b = vec![];
    for _ in 0..4 {
        b.extend_from_slice(&next(s).to_le_bytes());
    }
    BigUint::from_bytes_le(&b) % modulus
}
struct Redc<F: PrimeField> {
    p: BigUint,
    twop: BigUint,
    r: BigUint,
    pinv: BigUint,
    hinv: F,
    sixteenth: F,
}
impl<F: PrimeField> Redc<F> {
    fn new() -> Self {
        let p = BigUint::from_bytes_le(&F::MODULUS.to_bytes_le());
        let r = big(1) << 260;
        let pinv = p.modpow(&((&r >> 1) - big(1)), &r);
        assert_eq!((&p * &pinv) % &r, big(1));
        assert!(&p * big(8) < r);
        let hinv = F::from(2).pow([256u64]).inverse().unwrap();
        Self {
            twop: &p * big(2),
            p,
            r,
            pinv,
            hinv,
            sixteenth: F::from(16).inverse().unwrap(),
        }
    }
    fn field(&self, x: &BigUint) -> F {
        F::from_le_bytes_mod_order(&x.to_bytes_le()) * self.hinv
    }
    fn evaluate(
        &self,
        a: &BigUint,
        b: &BigUint,
        c: &BigUint,
        d: &BigUint,
        negative: bool,
    ) -> BigUint {
        for v in [a, b, c, d] {
            assert!(v <= &self.twop);
        }
        let t = if negative {
            a * b + (&self.twop - c) * d
        } else {
            a * b + c * d
        };
        let rem = (&t * &self.pinv) % &self.r;
        let k = if rem == big(0) { big(0) } else { &self.r - rem };
        let cancelled = &t + &k * &self.p;
        assert_eq!(&cancelled % &self.r, big(0));
        let out = cancelled / &self.r;
        assert!(out < self.twop);
        let want = if negative {
            self.field(a) * self.field(b) - self.field(c) * self.field(d)
        } else {
            self.field(a) * self.field(b) + self.field(c) * self.field(d)
        } * self.sixteenth;
        assert_eq!(self.field(&out), want);
        out
    }
}
fn raw<F: PrimeField>(label: &str) -> Value {
    let ctx = Redc::<F>::new();
    let edges = [
        big(0),
        big(1),
        &ctx.p - big(1),
        ctx.p.clone(),
        &ctx.p + big(1),
        &ctx.twop - big(1),
        ctx.twop.clone(),
    ];
    let mut n = 0;
    let mut digest = Sha256::new();
    for a in &edges {
        for b in &edges {
            for c in &edges {
                for d in &edges {
                    for neg in [false, true] {
                        digest.update(ctx.evaluate(a, b, c, d, neg).to_bytes_le());
                        n += 1;
                    }
                }
            }
        }
    }
    let edge_outputs = n;
    let mut seed = if label == "Fq" {
        0x6451a9e3b20f7cd1
    } else {
        0xad3f620917e8b4c5
    };
    let domain = &ctx.twop + big(1);
    for _ in 0..2048 {
        let a = random(&mut seed, &domain);
        let b = random(&mut seed, &domain);
        let c = random(&mut seed, &domain);
        let d = random(&mut seed, &domain);
        for neg in [false, true] {
            digest.update(ctx.evaluate(&a, &b, &c, &d, neg).to_bytes_le());
            n += 1;
        }
    }
    let random_outputs = n - edge_outputs;
    let mut boundary_outputs = 0;
    for bits in [
        1usize, 51, 52, 53, 63, 64, 103, 104, 155, 156, 207, 208, 253, 254,
    ] {
        for offset in [0i32, 1, -1] {
            let power = big(1) << bits;
            let value = if offset < 0 {
                power - big(1)
            } else {
                power + big(offset as u64)
            };
            if value > ctx.twop {
                continue;
            }
            for neg in [false, true] {
                let a = &ctx.twop - &value;
                digest.update(
                    ctx.evaluate(&a, &value, &ctx.twop, &value, neg)
                        .to_bytes_le(),
                );
                n += 1;
                boundary_outputs += 1;
            }
        }
    }
    let mut a = ctx.p.clone();
    for round in 0..1024 {
        let b = random(&mut seed, &domain);
        let c = random(&mut seed, &domain);
        let d = random(&mut seed, &domain);
        a = ctx.evaluate(&a, &b, &c, &d, round % 2 != 0);
        digest.update(a.to_bytes_le());
        n += 1;
    }
    json!({"field":label,"outputs":n,"inclusive_corner_outputs":edge_outputs,"random_outputs":random_outputs,"radix_boundary_outputs":boundary_outputs,"changing_chain_outputs":1024,"range":"0<=input<=2p;0<=output<2p","scale":"R256logicalvalue of R260 REDC equals fusedresult/16","output_sha256":format!("{:x}",digest.finalize())})
}
fn norm(v: Fq2) -> Fq {
    v.c0.square() + v.c1.square()
}
fn conjugate(v: Fq2) -> Fq2 {
    Fq2::new(v.c0, -v.c1)
}
fn scale(v: Fq2, c: Fq) -> Fq2 {
    Fq2::new(v.c0 * c, v.c1 * c)
}
fn inverse_lanes(values: [Fq; 8]) -> [Fq; 8] {
    let mut pre = [Fq::from(1); 8];
    let mut product = Fq::from(1);
    for i in 0..8 {
        assert!(!values[i].is_zero());
        pre[i] = product;
        product *= values[i];
    }
    let mut inv = product.inverse().unwrap();
    let mut out = [Fq::from(1); 8];
    for i in (0..8).rev() {
        out[i] = inv * pre[i];
        inv *= values[i];
    }
    out
}
fn finish(a: G2Affine, b: G2Affine, slope: Fq2) -> G2Affine {
    let x = slope.square() - a.x - b.x;
    let y = slope * (a.x - x) - a.y;
    G2Affine::new_unchecked(x, y)
}
#[derive(Clone, Copy)]
struct Entry {
    a: G2Affine,
    b: G2Affine,
    pre: Fq,
    n: Fq,
    numerator: Fq2,
    index: usize,
}
fn apply(
    input: &[G2Affine],
    batch: &[(usize, G2Affine)],
    h: Fq,
) -> Result<Vec<G2Affine>, &'static str> {
    let mut seen = vec![false; input.len()];
    for &(i, b) in batch {
        if i >= input.len() {
            return Err("bounds");
        }
        if seen[i] {
            return Err("duplicate destination");
        }
        seen[i] = true;
        if input[i].is_zero() || b.is_zero() {
            return Err("identity");
        }
        if (b.x - input[i].x).is_zero() {
            return Err("zero denominator");
        }
    }
    let full = batch.len() / 8 * 8;
    let mut chain = [Fq::from(1); 8];
    let mut groups = Vec::<[Entry; 8]>::new();
    for k in (0..full).step_by(8) {
        let group = std::array::from_fn(|l| {
            let (i, b) = batch[k + l];
            let a = input[i];
            let dx = b.x - a.x;
            let n = norm(dx) * h;
            let entry = Entry {
                a,
                b,
                pre: chain[l],
                n,
                numerator: scale((b.y - a.y) * conjugate(dx), h),
                index: i,
            };
            chain[l] *= n;
            entry
        });
        groups.push(group);
    }
    let mut tails = vec![];
    for k in full..batch.len() {
        let l = k % 8;
        let (i, b) = batch[k];
        let a = input[i];
        let dx = b.x - a.x;
        let n = norm(dx);
        let numerator = scale((b.y - a.y) * conjugate(dx), chain[l]);
        tails.push(Entry {
            a,
            b,
            pre: chain[l],
            n,
            numerator,
            index: i,
        });
        chain[l] *= n;
    }
    let mut inverse = inverse_lanes(chain);
    let mut out = input.to_vec();
    for (offset, e) in tails.iter().enumerate().rev() {
        let l = (full + offset) % 8;
        let slope = scale(e.numerator, inverse[l]);
        inverse[l] *= e.n;
        out[e.index] = finish(e.a, e.b, slope);
    }
    for group in groups.iter().rev() {
        for l in 0..8 {
            let e = group[l];
            let factor = e.pre * inverse[l];
            inverse[l] *= e.n;
            let slope = scale(e.numerator, factor);
            assert_eq!(slope, (e.b.y - e.a.y) * (e.b.x - e.a.x).inverse().unwrap());
            out[e.index] = finish(e.a, e.b, slope);
        }
    }
    Ok(out)
}
fn scalar(label: &str) -> Fr {
    Fr::from_le_bytes_mod_order(&Sha256::digest(label.as_bytes()))
}
fn points(n: usize, label: &str) -> Vec<G2Affine> {
    let generator = G2Affine::generator();
    let mut p = generator
        .mul_bigint(scalar(&format!("scalednorm176/3bafe172/{label}/start")).into_bigint());
    let step =
        generator.mul_bigint(scalar(&format!("scalednorm176/3bafe172/{label}/step")).into_bigint());
    let mut out = vec![];
    for _ in 0..n {
        out.push(p);
        p += step;
    }
    G2Projective::normalize_batch(&out)
}
fn point_model() -> Value {
    let h = Fq::from(16).inverse().unwrap();
    let mut rows = vec![];
    let mut outputs = 0;
    let mut cases = 0;
    for n in [
        0usize, 1, 7, 8, 9, 15, 16, 31, 32, 63, 64, 199, 200, 201, 351, 352, 353, 511,
    ] {
        let size = (n + 1).next_power_of_two();
        let start = points(size, &format!("{n}/bucket"));
        let incoming = points(n, &format!("{n}/incoming"));
        for signs in 0..3 {
            let batch: Vec<_> = incoming
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    let p = if signs == 1 || signs == 2 && i % 3 == 1 {
                        -*p
                    } else {
                        *p
                    };
                    (i * 53 % size, p)
                })
                .collect();
            let mut original = start.clone();
            let mut candidate = start.clone();
            let mut expected = start.clone();
            for reuse in 0..3 {
                for &(i, b) in &batch {
                    expected[i] = (expected[i].into_group() + b).into_affine();
                }
                original = apply(&original, &batch, Fq::from(1)).unwrap();
                candidate = apply(&candidate, &batch, h).unwrap();
                assert_eq!(candidate, expected);
                assert_eq!(original, expected);
                outputs += n;
                cases += 1;
                rows.push(json!({"points":n,"signs":signs,"reuse":reuse,"full_vector_groups":n/8,"scalar_tail":n%8,"outputs_checked":n,"unchanged_bucket_slots_checked":size-n}));
            }
        }
    }
    let initial = points(16, "guards/bucket");
    let incoming = points(8, "guards/incoming");
    assert_eq!(
        apply(&initial, &[(0, incoming[0]), (0, incoming[1])], h).err(),
        Some("duplicate destination")
    );
    assert_eq!(
        apply(&initial, &[(16, incoming[0])], h).err(),
        Some("bounds")
    );
    assert_eq!(
        apply(&initial, &[(0, G2Affine::zero())], h).err(),
        Some("identity")
    );
    assert_eq!(
        apply(&initial, &[(0, initial[0])], h).err(),
        Some("zero denominator")
    );
    assert_eq!(
        apply(&initial, &[(0, -initial[0])], h).err(),
        Some("zero denominator")
    );
    let mut identity = initial.clone();
    identity[0] = G2Affine::zero();
    assert_eq!(
        apply(&identity, &[(0, incoming[0])], h).err(),
        Some("identity")
    );
    json!({"new_case_executions":cases,"new_point_outputs":outputs,"guard_checks":6,"rows":rows,"group_oracle":"independent complete Arkworks projective addition","model_scale_choices":["1","1/16"],"scalar_tails_unchanged":true,"one_shared_scalar_inverse_per_modeled_batch":true})
}
fn main() {
    let raw_results = json!([raw::<Fq>("Fq"), raw::<Fr>("Fr")]);
    let points = point_model();
    let result = json!({"mathematical_qualification_passed":true,"raw_scaled_fused":raw_results,"point_model":points,"native_selected_next":true,"native_qualified":false,"performance_qualified":false,"production_changed":false,"new_proofs":0,"new_timings":0,"canonical_proof_ledger_count":31543,"source_work":{"ordinary_vector_multiplies_per_fullgroup":7,"fused2_per_fullgroup":5,"scaled_fused2_per_fullgroup":3,"nominal_IFMA_source_proxy":1450,"new_input_output_codecs":0,"new_record_bytes":0},"limits":"Exact scalar/BigUint algebra, not execution of new SIMD wrappers or proof. Operation counts do not predict latency; final-consumer native code must confirm packing/correction removal."});
    let path = format!("{R}/analysis.json");
    assert!(!Path::new(&path).exists());
    fs::write(path, serde_json::to_vec_pretty(&result).unwrap()).unwrap();
    println!(
        "PASSscaledREDCandmatchedG2norm/numeratorwithtails;nativequalificationselected,notadopted"
    );
}
