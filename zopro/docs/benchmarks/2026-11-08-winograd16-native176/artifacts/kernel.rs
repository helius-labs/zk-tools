use zopro_bn254::{ifma8::Ifma8, Fr, FrConfig};

pub type Pair = Ifma8<FrConfig>;
include!("original-within.rs.inc");

pub struct Roots {
    pub w: Fr,
    factors: [Pair; 4],
    twiddle16: [Pair; 2],
    twiddle8: Pair,
    pair: Pair,
}

fn repeat4(v: [Fr; 4]) -> Pair {
    Pair::new(&core::array::from_fn(|j| v[j % 4]))
}

impl Roots {
    pub fn new(power: u64) -> Self {
        assert!(power < 16 && power % 2 == 1);
        let w = Fr::root_of_unity(16).unwrap().pow(&[power]);
        let half = Fr::from(2u64).inverse().unwrap();
        let wi = w.inverse().unwrap();
        let i = w.pow(&[4]);
        let a = (w + wi) * half;
        let b = (w.pow(&[3]) + wi.pow(&[3])) * half;
        let d = (w.square() + wi.square()) * half;
        Self {
            w,
            factors: [
                repeat4([i, i, (w.pow(&[2]) + w.pow(&[6])) * half,
                    (w.pow(&[2]) - w.pow(&[6])) * half]),
                repeat4([d, i * d, i, a - b]),
                repeat4([-(a + b), b, i * (b - a), -i * (b + a)]),
                repeat4([i * a, Fr::ZERO, Fr::ZERO, Fr::ZERO]),
            ],
            twiddle16: [
                repeat4(core::array::from_fn(|j| w.pow(&[j as u64]))),
                repeat4(core::array::from_fn(|j| w.pow(&[(j + 4) as u64]))),
            ],
            twiddle8: repeat4([Fr::ONE, w.square(), i, w.pow(&[6])]),
            pair: repeat4([Fr::ONE, i, Fr::ONE, i]),
        }
    }
}

#[inline(always)]
fn dif8(a: &mut Pair, b: &mut Pair, r: &Roots) {
    let t = a.sub(b).mul(&r.twiddle8);
    *a = a.add(b);
    *b = t;
    dif_within(a, b, &r.pair);
}

#[inline(always)]
fn dit8(a: &mut Pair, b: &mut Pair, r: &Roots) {
    dit_within(a, b, &r.pair, None);
    let t = b.mul(&r.twiddle8);
    *b = a.sub(&t);
    *a = a.add(&t);
}

#[inline(always)]
pub fn control<const FORWARD: bool>(values: &mut [Pair; 4], r: &Roots) {
    let [mut a, mut b, mut c, mut d] = *values;
    if FORWARD {
        dit8(&mut a, &mut b, r);
        dit8(&mut c, &mut d, r);
        let u = c.mul(&r.twiddle16[0]);
        let v = d.mul(&r.twiddle16[1]);
        *values = [a.add(&u), b.add(&v), a.sub(&u), b.sub(&v)];
    } else {
        let u = a.sub(&c).mul(&r.twiddle16[0]);
        let v = b.sub(&d).mul(&r.twiddle16[1]);
        a = a.add(&c);
        b = b.add(&d);
        c = u;
        d = v;
        dif8(&mut a, &mut b, r);
        dif8(&mut c, &mut d, r);
        *values = [a, b, c, d];
    }
}

#[inline(always)]
fn bit_reverse(values: [Pair; 4]) -> [Pair; 4] {
    let [a, b, c, d] = values;
    let u = Pair::shuffle_pair::<0, 4, 1, 5>(&a, &c);
    let v = Pair::shuffle_pair::<0, 4, 1, 5>(&b, &d);
    let s = Pair::shuffle_pair::<2, 6, 3, 7>(&a, &c);
    let t = Pair::shuffle_pair::<2, 6, 3, 7>(&b, &d);
    [
        Pair::shuffle_pair::<0, 1, 4, 5>(&u, &v),
        Pair::shuffle_pair::<0, 1, 4, 5>(&s, &t),
        Pair::shuffle_pair::<2, 3, 6, 7>(&u, &v),
        Pair::shuffle_pair::<2, 3, 6, 7>(&s, &t),
    ]
}

// Linear even-half stage. The four scalar products per sequence share one
// existing field-vector multiplication. This is an algebraic subexpression
// of the new global DFT16, not another standalone DFT8 timing experiment.
#[inline(always)]
fn even_half(a: Pair, b: Pair, factor: &Pair) -> [Pair; 2] {
    let s = a.add(&b);
    let t = a.sub(&b);
    let sr = Pair::shuffle_pair::<2, 3, 0, 1>(&s, &s);
    let sd = s.sub(&sr);
    let se = s.add(&sr);
    let tr = Pair::shuffle_pair::<2, 3, 0, 1>(&t, &t);
    let tu = t.add(&tr);
    let tv = t.sub(&tr);
    let p0 = Pair::shuffle_pair::<1, 6, 0, 0>(&sd, &t);
    let p1 = Pair::shuffle_pair::<1, 5, 1, 5>(&tu, &tv);
    let p = Pair::shuffle_pair::<0, 1, 4, 5>(&p0, &p1).mul(factor);
    let ea = Pair::shuffle_pair::<0, 0, 4, 4>(&se, &sd);
    let eb = Pair::shuffle_pair::<1, 1, 4, 4>(&se, &p);
    let out0 = Pair::shuffle_pair::<0, 5, 2, 7>(&ea.add(&eb), &ea.sub(&eb));
    let b0 = Pair::shuffle_pair::<0, 0, 0, 0>(&t, &t);
    let p1 = Pair::shuffle_pair::<1, 1, 1, 1>(&p, &p);
    let base = Pair::shuffle_pair::<0, 1, 6, 7>(&b0.add(&p1), &b0.sub(&p1));
    let p2 = Pair::shuffle_pair::<2, 2, 2, 2>(&p, &p);
    let p3 = Pair::shuffle_pair::<3, 3, 3, 3>(&p, &p);
    let term = Pair::shuffle_pair::<0, 1, 6, 7>(&p2.add(&p3), &p2.sub(&p3));
    let out1 = Pair::shuffle_pair::<0, 5, 2, 7>(&base.add(&term), &base.sub(&term));
    [out0, out1]
}

#[inline(always)]
fn factored_dif(values: [Pair; 4], r: &Roots) -> [Pair; 4] {
    let [x0, x1, x2, x3] = values;
    let s0 = x0.add(&x2);
    let s1 = x1.add(&x3);
    let t0 = x0.sub(&x2);
    let t1 = x1.sub(&x3);
    let [e0, e1] = even_half(s0, s1, &r.factors[0]);

    let reverse = Pair::shuffle_pair::<0, 3, 2, 1>(&t1, &t1);
    let a = t0.sub(&reverse);
    let b = t0.add(&reverse);
    let ab = Pair::shuffle_pair::<1, 3, 5, 7>(&a, &b);
    let abr = Pair::shuffle_pair::<1, 0, 3, 2>(&ab, &ab);
    let sums = ab.add(&abr);

    let u = Pair::shuffle_pair::<2, 6, 0, 5>(&a, &b);
    let v = Pair::shuffle_pair::<0, 5, 0, 0>(&t1, &a);
    let p = Pair::shuffle_pair::<0, 1, 4, 5>(&u, &v).mul(&r.factors[1]);
    let u = Pair::shuffle_pair::<3, 4, 0, 0>(&a, &sums);
    let q = Pair::shuffle_pair::<0, 1, 5, 7>(&u, &b).mul(&r.factors[2]);
    let z = Pair::zero();
    let n = Pair::shuffle_pair::<2, 4, 5, 6>(&sums, &z).mul(&r.factors[3]);

    // [U,V,W,Z], reconstructed from the six rotation products.
    let left = Pair::shuffle_pair::<3, 4, 6, 7>(&p, &q);
    let right = Pair::shuffle_pair::<1, 1, 4, 4>(&q, &n);
    let rotations = left.add(&right);
    let left = Pair::shuffle_pair::<0, 3, 1, 2>(&rotations, &rotations);
    let right = Pair::shuffle_pair::<2, 1, 3, 0>(&rotations, &rotations);
    let terms = Pair::shuffle_pair::<0, 5, 2, 7>(&left.add(&right), &left.sub(&right));

    // Four-point Hadamard on [T0,D,H,G], with output order 1,5,3,7.
    let base = Pair::shuffle_pair::<0, 4, 6, 5>(&t0, &p);
    let swap = Pair::shuffle_pair::<1, 0, 3, 2>(&base, &base);
    let base = Pair::shuffle_pair::<0, 4, 2, 6>(&base.add(&swap), &base.sub(&swap));
    let swap = Pair::shuffle_pair::<2, 3, 0, 1>(&base, &base);
    let base = Pair::shuffle_pair::<0, 1, 5, 4>(&base.add(&swap), &base.sub(&swap));
    let sum = base.add(&terms);
    let difference = base.sub(&terms);
    let o0 = Pair::shuffle_pair::<0, 4, 1, 5>(&sum, &difference);
    let o1 = Pair::shuffle_pair::<2, 6, 3, 7>(&sum, &difference);
    [e0, e1, o0, o1]
}

#[inline(always)]
pub fn candidate<const FORWARD: bool>(values: &mut [Pair; 4], r: &Roots) {
    if FORWARD {
        // Both permutations are charged to the future complete caller. A
        // distinct transposed circuit is not silently substituted after timing.
        *values = bit_reverse(factored_dif(bit_reverse(*values), r));
    } else {
        *values = factored_dif(*values, r);
    }
}

pub fn pack(x: &[Fr; 32]) -> [Pair; 4] {
    core::array::from_fn(|q| Pair::new(&core::array::from_fn(|j| x[16 * (j / 4) + 4 * q + j % 4])))
}

pub fn unpack(v: &[Pair; 4]) -> [Fr; 32] {
    let lanes = v.map(|p| p.get());
    core::array::from_fn(|j| lanes[(j % 16) / 4][4 * (j / 16) + j % 4])
}
