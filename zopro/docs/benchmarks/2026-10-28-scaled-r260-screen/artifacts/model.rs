use ark_ec::{AffineRepr, CurveGroup};
use ark_ff::{BigInteger, PrimeField};
use num_bigint::BigUint as U;
use serde_json::{json, Value};
use std::{cell::Cell, collections::BTreeMap};

fn u(x: u64) -> U {
    U::from(x)
}
fn low(x: &U) -> u64 {
    x.to_u64_digits().first().copied().unwrap_or(0)
}
fn random(seed: &mut u64) -> U {
    let mut b = [0u8; 32];
    for c in b.chunks_mut(8) {
        *seed ^= *seed << 13;
        *seed ^= *seed >> 7;
        *seed ^= *seed << 17;
        c.copy_from_slice(&seed.to_le_bytes());
    }
    U::from_bytes_le(&b)
}
type C = [U; 2];
#[derive(Clone, Debug, PartialEq, Eq)]
struct Point {
    x: C,
    y: C,
    infinity: bool,
}
struct Family {
    p: U,
    h: U,
    r: U,
    nprime: U,
    h_inv: U,
    r_inv: U,
    mul: Cell<u64>,
    sum2: Cell<u64>,
    inverse: Cell<u64>,
}
impl Family {
    fn new(p: U, hostbits: usize) -> Self {
        let h = u(1) << hostbits;
        let r = &h << 4usize;
        assert!((&p & u(1)) == u(1));
        assert!(&p * u(4) < h);
        assert!(&p * u(64) < r);
        let nprime = &r - p.modinv(&r).unwrap();
        let h_inv = h.modinv(&p).unwrap();
        let r_inv = r.modinv(&p).unwrap();
        Self {
            p,
            h,
            r,
            nprime,
            h_inv,
            r_inv,
            mul: Cell::new(0),
            sum2: Cell::new(0),
            inverse: Cell::new(0),
        }
    }
    fn bn() -> Self {
        let p = U::from_bytes_le(&ark_bn254::Fq::MODULUS.to_bytes_le());
        assert_eq!(
            p.to_string(),
            "21888242871839275222246405745257275088696311157297823662689037894645226208583"
        );
        Self::new(p, 256)
    }
    fn bound(&self, x: &U, k: u64) {
        assert!(x < &(&self.p * u(k)), "range{k}p");
        assert!(x < &self.r);
    }
    fn redc(&self, t: U, k: u64) -> U {
        assert!(t < &self.p * &self.r * u(k - 1));
        let q = ((&t % &self.r) * &self.nprime) % &self.r;
        let numerator = t + q * &self.p;
        assert_eq!(&numerator % &self.r, u(0));
        let result = numerator / &self.r;
        self.bound(&result, k);
        result
    }
    fn m(&self, a: &U, b: &U) -> U {
        self.mul.set(self.mul.get() + 1);
        self.redc(a * b, 2)
    }
    fn s(&self, a: &U, b: &U, c: &U, d: &U, k: u64) -> U {
        self.sum2.set(self.sum2.get() + 1);
        self.redc(a * b + c * d, k)
    }
    fn sub(&self, a: &U, b: &U, bias: u64) -> U {
        let t = a + &self.p * u(bias);
        assert!(&t >= b);
        t - b
    }
    fn encode(&self, x: &U) -> U {
        x * &self.h % &self.p
    }
    fn canonical_host(&self, x: &U) -> U {
        x * &self.h_inv % &self.p
    }
    fn import(&self, x: &U, bits: usize) -> U {
        self.bound(x, 1);
        x << bits
    }
    fn export(&self, x: &U, bits: usize) -> U {
        let mask = (1u64 << bits) - 1;
        let m = low(x).wrapping_mul(low(&self.nprime)) & mask;
        let n = x + &self.p * u(m);
        assert_eq!(&n % u(1 << bits), u(0));
        let mut z = n >> bits;
        self.bound(&z, 4);
        if z >= &self.p * u(2) {
            z -= &self.p * u(2);
        }
        if z >= self.p {
            z -= &self.p;
        }
        self.bound(&z, 1);
        assert_eq!((&z << bits) % &self.p, x % &self.p);
        z
    }
    fn inverse_tree(&self, chains: &[U; 8]) -> [U; 8] {
        let swapped: [U; 8] = core::array::from_fn(|i| chains[i ^ 1].clone());
        let pairs: [U; 8] = core::array::from_fn(|i| self.m(&chains[i], &swapped[i]));
        let others: [U; 8] = core::array::from_fn(|i| pairs[i ^ 2].clone());
        let quads: [U; 8] = core::array::from_fn(|i| self.m(&pairs[i], &others[i]));
        let root = self.m(&quads[0], &quads[4]);
        let inverse =
            ((&root % &self.p).modinv(&self.p).unwrap() * (&self.r * &self.r % &self.p)) % &self.p;
        self.inverse.set(self.inverse.get() + 1);
        core::array::from_fn(|i| {
            let a = self.m(&inverse, &quads[i ^ 4]);
            let b = self.m(&a, &others[i]);
            let c = self.m(&b, &swapped[i]);
            assert_eq!((&c * &chains[i] * &self.r_inv) % &self.p, &self.r % &self.p);
            c
        })
    }
    fn batch(&self, inputs: &[(Point, Point)]) -> Option<Vec<Point>> {
        if inputs.is_empty() {
            return Some(vec![]);
        }
        if inputs.len() % 8 != 0
            || inputs
                .iter()
                .any(|(a, b)| a.infinity || b.infinity || a.x == b.x)
        {
            return None;
        }
        let mut chains: [U; 8] = core::array::from_fn(|_| &self.r % &self.p);
        let mut records = Vec::new();
        for group in inputs.chunks_exact(8) {
            let prefix = chains.clone();
            let norms: [U; 8] = core::array::from_fn(|i| {
                let (a, b) = &group[i];
                let dx: C = core::array::from_fn(|j| {
                    self.sub(&self.import(&b.x[j], 2), &self.import(&a.x[j], 2), 4)
                });
                for x in &dx {
                    self.bound(x, 8);
                }
                let norm = self.s(&dx[0], &dx[0], &dx[1], &dx[1], 3);
                assert_ne!(&norm % &self.p, u(0));
                norm
            });
            for i in 0..8 {
                chains[i] = self.m(&chains[i], &norms[i]);
            }
            records.push((prefix, norms));
        }
        let mut inverse = self.inverse_tree(&chains);
        let mut out = inputs.iter().map(|(a, _)| a.clone()).collect::<Vec<_>>();
        for g in (0..records.len()).rev() {
            for i in 0..8 {
                let k = g * 8 + i;
                let (a, b) = &inputs[k];
                let ax: C = core::array::from_fn(|j| self.import(&a.x[j], 2));
                let bx: C = core::array::from_fn(|j| self.import(&b.x[j], 2));
                let ay: C = core::array::from_fn(|j| self.import(&a.y[j], 1));
                let by: C = core::array::from_fn(|j| self.import(&b.y[j], 1));
                let dx: C = core::array::from_fn(|j| self.sub(&bx[j], &ax[j], 4));
                let dy: C = core::array::from_fn(|j| self.sub(&by[j], &ay[j], 2));
                for d in &dy {
                    self.bound(d, 4);
                }
                let factor = self.m(&records[g].0[i], &inverse[i]);
                inverse[i] = self.m(&inverse[i], &records[g].1[i]);
                let negdy0 = &self.p * u(4) - &dy[0];
                let numerator = [
                    self.s(&dy[0], &dx[0], &dy[1], &dx[1], 2),
                    self.s(&dy[1], &dx[0], &negdy0, &dx[1], 2),
                ];
                let l: C = core::array::from_fn(|j| self.m(&numerator[j], &factor));
                let sq0 = self.m(&(&l[0] + &l[1]), &self.sub(&l[0], &l[1], 2));
                let sq1 = self.m(&l[0], &l[1]) * u(2);
                self.bound(&sq1, 4);
                let sq = [sq0, sq1];
                let x: C = core::array::from_fn(|j| {
                    let z = &sq[j] + &self.p * u(8);
                    assert!(z >= &ax[j] + &bx[j]);
                    let x = z - &ax[j] - &bx[j];
                    self.bound(&x, 12);
                    x
                });
                let d: C = core::array::from_fn(|j| {
                    let d = self.sub(&ax[j], &x[j], 12);
                    self.bound(&d, 16);
                    d
                });
                let negl1 = &self.p * u(2) - &l[1];
                let product = [
                    self.s(&l[0], &d[0], &negl1, &d[1], 2),
                    self.s(&l[0], &d[1], &l[1], &d[0], 2),
                ];
                let y: C = core::array::from_fn(|j| {
                    let z = self.sub(&product[j], &ay[j], 2);
                    self.bound(&z, 4);
                    z
                });
                out[k] = Point {
                    x: x.map(|v| self.export(&v, 2)),
                    y: y.map(|v| self.export(&v, 1)),
                    infinity: false,
                };
            }
        }
        assert!(inverse.iter().all(|x| x % &self.p == &self.r % &self.p));
        Some(out)
    }
    fn cadd(&self, a: &C, b: &C) -> C {
        core::array::from_fn(|i| (&a[i] + &b[i]) % &self.p)
    }
    fn csub(&self, a: &C, b: &C) -> C {
        core::array::from_fn(|i| (&a[i] + &self.p - &b[i]) % &self.p)
    }
    fn cmul(&self, a: &C, b: &C) -> C {
        [
            (&a[0] * &b[0] + &self.p * &self.p - &a[1] * &b[1]) % &self.p,
            (&a[0] * &b[1] + &a[1] * &b[0]) % &self.p,
        ]
    }
    fn toy_add(&self, a: &Point, b: &Point) -> Point {
        assert!(!a.infinity && !b.infinity && a.x != b.x);
        let dx = self.csub(&b.x, &a.x);
        let dy = self.csub(&b.y, &a.y);
        let norm = (&dx[0] * &dx[0] + &dx[1] * &dx[1]) % &self.p;
        let inverse = norm.modinv(&self.p).unwrap();
        let inv = [
            &dx[0] * &inverse % &self.p,
            (&self.p - &dx[1]) * &inverse % &self.p,
        ];
        let slope = self.cmul(&dy, &inv);
        let x = self.csub(&self.csub(&self.cmul(&slope, &slope), &a.x), &b.x);
        let y = self.csub(&self.cmul(&slope, &self.csub(&a.x, &x)), &a.y);
        Point {
            x,
            y,
            infinity: false,
        }
    }
    fn host_point(&self, p: &Point) -> Point {
        Point {
            x: core::array::from_fn(|j| self.encode(&p.x[j])),
            y: core::array::from_fn(|j| self.encode(&p.y[j])),
            infinity: p.infinity,
        }
    }
    fn canonical_point(&self, p: &Point) -> Point {
        Point {
            x: core::array::from_fn(|j| self.canonical_host(&p.x[j])),
            y: core::array::from_fn(|j| self.canonical_host(&p.y[j])),
            infinity: p.infinity,
        }
    }
}
fn ark_point(s: ark_bn254::Fr) -> Point {
    let p = (ark_bn254::G2Affine::generator() * s).into_affine();
    let cv = |v: ark_bn254::Fq| U::from_bytes_le(&v.into_bigint().to_bytes_le());
    Point {
        x: [cv(p.x.c0), cv(p.x.c1)],
        y: [cv(p.y.c0), cv(p.y.c1)],
        infinity: p.infinity,
    }
}
fn codec_checks(f: &Family) -> usize {
    let mut seed = 0x3487_1e57_8a53_f913;
    let mut values = vec![u(0), u(1), &f.p - u(1)];
    for bit in [1, 51, 52, 63, 64, 127, 128, 207, 208, 252] {
        let x: U = u(1) << bit as usize;
        values.extend([&x - u(1), x.clone(), x + u(1)]);
    }
    for _ in 0..2048 {
        values.push(random(&mut seed) % &f.p);
    }
    let mut count = 0;
    for a in values {
        let a = a % &f.p;
        assert_eq!((&a * u(4) * &f.r_inv * u(4)) % &f.p, &a * &f.h_inv % &f.p);
        assert_eq!((&a * u(2) * &f.r_inv * u(8)) % &f.p, &a * &f.h_inv % &f.p);
        for alias in 0..8 {
            let x = &a * u(4) + &f.p * u(alias);
            f.bound(&x, 12);
            assert_eq!(f.export(&x, 2), a);
            count += 1;
        }
        for alias in 0..2 {
            let y = &a * u(2) + &f.p * u(alias);
            f.bound(&y, 4);
            assert_eq!(f.export(&y, 1), a);
            count += 1;
        }
    }
    count
}
fn split52(x: &U) -> [u64; 5] {
    core::array::from_fn(|i| low(&(x >> (52 * i))) & ((1u64 << 52) - 1))
}
fn cios(f: &Family, a: &U, b: &U, c: &U, d: &U, fused: bool) -> U {
    assert_eq!(f.r, u(1) << 260usize);
    let [aa, bb, cc, dd, pp] = [a, b, c, d, &f.p].map(split52);
    let mask = (1u128 << 52) - 1;
    let inv = low(&f.nprime) & mask as u64;
    let mut t = [0u128; 6];
    for i in 0..5 {
        let mut carry = 0;
        for j in 0..5 {
            let z = t[j]
                + aa[j] as u128 * bb[i] as u128
                + if fused {
                    cc[j] as u128 * dd[i] as u128
                } else {
                    0
                }
                + carry;
            t[j] = z & mask;
            carry = z >> 52;
        }
        t[5] += carry;
        let q = (t[0] * inv as u128) & mask;
        carry = 0;
        for j in 0..5 {
            let z = t[j] + q * pp[j] as u128 + carry;
            if j > 0 {
                t[j - 1] = z & mask;
            }
            carry = z >> 52;
        }
        let z = t[5] + carry;
        t[4] = z & mask;
        t[5] = z >> 52;
    }
    assert_eq!(t[5], 0);
    let mut out = u(0);
    for j in (0..5).rev() {
        out = (out << 52usize) + U::from(t[j]);
    }
    out
}
fn cios_checks(f: &Family) -> Value {
    let mut seed = 0xa43d_7272_41f3_d8e1;
    let mut rows = vec![];
    for (name, limits, fused, bound) in [
        ("ordinary", [8, 8, 0, 0], false, 2),
        ("norm", [8, 8, 8, 8], true, 3),
        ("numerator", [4, 8, 4, 8], true, 2),
        ("finish", [2, 16, 2, 16], true, 2),
        ("prefix", [2, 3, 0, 0], false, 2),
    ] {
        let edge = |k: u64| {
            if k == 0 {
                vec![u(0)]
            } else {
                vec![
                    u(0),
                    u(1),
                    &f.p - u(1),
                    &f.p * u(k) / u(2),
                    &f.p * u(k) - u(1),
                ]
            }
        };
        let e = limits.map(edge);
        let mut count = 0;
        let mut check = |v: [U; 4]| {
            let t = &v[0] * &v[1] + if fused { &v[2] * &v[3] } else { u(0) };
            let want = f.redc(t, bound);
            assert_eq!(cios(f, &v[0], &v[1], &v[2], &v[3], fused), want);
            count += 1;
        };
        for a in &e[0] {
            for b in &e[1] {
                for c in &e[2] {
                    for d in &e[3] {
                        check([a.clone(), b.clone(), c.clone(), d.clone()]);
                    }
                }
            }
        }
        for _ in 0..8192 {
            let v = core::array::from_fn(|i| {
                if limits[i] == 0 {
                    u(0)
                } else {
                    random(&mut seed) % (&f.p * u(limits[i]))
                }
            });
            check(v);
        }
        rows.push(json!({"case":name,"reductions":count,"output_bound_p":bound,"limits_p":limits}));
    }
    json!(rows)
}
fn toy_checks() -> Value {
    let f = Family::new(u(7), 6);
    let mut points = vec![];
    for x0 in 0..7 {
        for x1 in 0..7 {
            let x = [u(x0), u(x1)];
            let rhs = f.cadd(&f.cmul(&f.cmul(&x, &x), &x), &[u(3), u(0)]);
            for y0 in 0..7 {
                for y1 in 0..7 {
                    let y = [u(y0), u(y1)];
                    if f.cmul(&y, &y) == rhs {
                        points.push(Point {
                            x: x.clone(),
                            y,
                            infinity: false,
                        });
                    }
                }
            }
        }
    }
    let mut ordinary = 0;
    let mut guards = 0;
    for a in &points {
        for b in &points {
            let input = (f.host_point(a), f.host_point(b));
            let before = input.clone();
            let got = f.batch(&vec![input.clone(); 8]);
            assert_eq!(input, before);
            if a.x == b.x {
                assert!(got.is_none());
                guards += 1;
            } else {
                let want = f.toy_add(a, b);
                for p in got.unwrap() {
                    assert_eq!(f.canonical_point(&p), want);
                }
                ordinary += 1;
            }
        }
    }
    json!({"field":"F49;u²=-1","curve":"y²=x³+3","finite_points":points.len(),"ordinary_pairs":ordinary,"equal_x_guards":guards,"repeated_lanes_per_pair":8,"SDK_requests":0})
}
fn bn_batches() -> Value {
    let f = Family::bn();
    let mut seed = 0x1974_9acf_7823_a241;
    let mut cases = vec![];
    let mut checked = 0;
    for n in [8, 16, 32, 64, 200] {
        let mut left = vec![];
        let mut right = vec![];
        for _ in 0..n {
            left.push(ark_bn254::Fr::from_le_bytes_mod_order(
                &random(&mut seed).to_bytes_le(),
            ));
            right.push(ark_bn254::Fr::from_le_bytes_mod_order(
                &random(&mut seed).to_bytes_le(),
            ));
        }
        let incoming = right
            .iter()
            .map(|&s| f.host_point(&ark_point(s)))
            .collect::<Vec<_>>();
        let mut current = left
            .iter()
            .map(|&s| f.host_point(&ark_point(s)))
            .collect::<Vec<_>>();
        for _ in 0..8 {
            let inputs = current
                .iter()
                .cloned()
                .zip(incoming.iter().cloned())
                .collect::<Vec<_>>();
            let got = f.batch(&inputs).unwrap();
            for i in 0..n {
                left[i] += right[i];
                assert_eq!(f.canonical_point(&got[i]), ark_point(left[i]));
                checked += 1;
            }
            current = got;
        }
        cases.push(json!({"batch":n,"changing_steps":8,"ordinary_outputs":n*8}));
    }
    let scalars = [
        ark_bn254::Fr::from(0u64),
        ark_bn254::Fr::from(1u64),
        -ark_bn254::Fr::from(1u64),
        ark_bn254::Fr::from(2u64),
        -ark_bn254::Fr::from(2u64),
    ];
    let mut guards = 0;
    let mut ordinary = 0;
    for &a in &scalars {
        for &b in &scalars {
            let pa = ark_point(a);
            let pb = ark_point(b);
            let pair = (f.host_point(&pa), f.host_point(&pb));
            let got = f.batch(&vec![pair; 8]);
            if pa.infinity || pb.infinity || pa.x == pb.x {
                assert!(got.is_none());
                guards += 1;
            } else {
                for p in got.unwrap() {
                    assert_eq!(f.canonical_point(&p), ark_point(a + b));
                }
                ordinary += 1;
            }
        }
    }
    let pair = (
        f.host_point(&ark_point(ark_bn254::Fr::from(3u64))),
        f.host_point(&ark_point(ark_bn254::Fr::from(7u64))),
    );
    for n in [1, 7, 9] {
        assert!(f.batch(&vec![pair.clone(); n]).is_none());
    }
    assert_eq!(f.batch(&[]), Some(vec![]));
    json!({"cases":cases,"ordinary_outputs":checked,"boundary_ordinary_pairs":ordinary,"boundary_whole_batch_guards":guards,"boundary_repeated_lanes":8,"tail_guards":3,"empty_case":1,"diagnostic_Montgomery_products":f.mul.get(),"diagnostic_fused_products":f.sum2.get(),"diagnostic_central_inversions":f.inverse.get()})
}
fn bounds(f: &Family) -> Value {
    let rows = [
        ("norm", 128, 3),
        ("numerator_and_finishing", 64, 2),
        ("prefix_and_reverse_norm", 6, 2),
        ("real_scale", 4, 2),
        ("square", 16, 2),
    ];
    let mut certificate = BTreeMap::new();
    for (name, k, bound) in rows {
        assert!(&f.p * u(k) < &f.r * u(bound - 1));
        certificate.insert(name,json!({"T_less_than_p_squared_times":k,"REDC_less_than_p_times":bound,"exact_sufficient_inequality":format!("{k}p < {}R",bound-1),"passed":true}));
    }
    json!({"p":f.p.to_string(),"H":f.h.to_string(),"R":f.r.to_string(),"64p_less_than_R":true,"4p_less_than_H":true,"nodes":certificate,"linear_bounds_p":{"import_x":4,"import_y":2,"dx":8,"dy":4,"slope":2,"square_real":2,"square_imaginary":4,"output_x_before_export":12,"finish_difference":16,"output_y_before_export":4,"export_before_correction":4},"export_corrections":["conditional subtract2p","conditional subtractp"],"key_point_bytes_unchanged":true,"native_implementation":false})
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn exact_bounds_and_scaled_coordinate_codec() {
        let f = Family::bn();
        bounds(&f);
        for a in [u(0), u(1), &f.p - u(1)] {
            assert_eq!(f.export(&(&a * u(4) + &f.p * u(7)), 2), a);
            assert_eq!(f.export(&(&a * u(2) + &f.p), 1), a);
        }
    }
    #[test]
    fn toy_curve_golden_addition_and_early_guard() {
        let f = Family::new(u(7), 6);
        let a = Point {
            x: [u(1), u(0)],
            y: [u(2), u(0)],
            infinity: false,
        };
        let b = Point {
            x: [u(2), u(0)],
            y: [u(2), u(0)],
            infinity: false,
        };
        let want = Point {
            x: [u(4), u(0)],
            y: [u(5), u(0)],
            infinity: false,
        };
        for p in f
            .batch(&vec![(f.host_point(&a), f.host_point(&b)); 8])
            .unwrap()
        {
            assert_eq!(f.canonical_point(&p), want);
        }
        let before = (f.mul.get(), f.sum2.get(), f.inverse.get());
        assert!(f
            .batch(&vec![(f.host_point(&a), f.host_point(&a)); 8])
            .is_none());
        assert_eq!(before, (f.mul.get(), f.sum2.get(), f.inverse.get()));
    }
    #[test]
    fn small_CIOS_matches_complete_REDC() {
        let f = Family::bn();
        for k in [2, 3, 8, 16] {
            let a = &f.p * u(k) - u(1);
            let b = &f.p * u(2) - u(1);
            assert_eq!(cios(&f, &a, &b, &u(0), &u(0), false), f.redc(&a * &b, 2));
        }
    }
}
fn main() {
    let f = Family::bn();
    let report = json!({"status":"exact model passed","bounds":bounds(&f),"codec_alias_checks":codec_checks(&f),"CIOS_domains":cios_checks(&f),"toy":toy_checks(),"BN254_G2":bn_batches(),"native_speedup_proven":false,"SDK_requests":0,"new_Gnark_exports":0,"proof_ledger":13181,"adopted":false,"production_changed":false,"goal_status":"active","CPU10x_rows":0});
    std::fs::write("analysis.json", serde_json::to_vec_pretty(&report).unwrap()).unwrap();
    println!("{}", serde_json::to_string_pretty(&report).unwrap());
}
