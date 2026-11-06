use crate::original::{Guard, Ops, Point, PointField};
use ark_bn254::Fq;
use ark_ff::Field;

pub fn batch<F: PointField>(input: &[[Point<F>; 3]], op: &Ops<F>) -> Result<Vec<Point<F>>, Guard> {
    let mut chain = Fq::ONE;
    let mut records = Vec::with_capacity(input.len());
    for &[p, q, r] in input {
        if p.infinity || q.infinity || r.infinity {
            return Err(Guard::Infinity);
        }
        let d = op.sub(q.x, p.x);
        let n = op.sub(q.y, p.y);
        let s = op.add(op.add(p.x, q.x), r.x);
        let h = op.minus_square(op.sq(d), s, n);
        if h == F::ZERO {
            return Err(Guard::SecondDenominator);
        }
        let u = op.sop(d, op.add(p.y, r.y), n, op.sub(r.x, p.x));
        let prefix = chain;
        let norm = op.norm(h);
        chain = op.base_mul(chain, norm);
        records.push(([p, q, r], d, n, s, h, u, prefix, norm));
    }
    if records.is_empty() {
        return Ok(Vec::new());
    }
    let mut inverse = op.base_inv(chain);
    let mut out = vec![Point::infinity(); input.len()];
    for (i, ([p, q, r], d, n, s, h, u, prefix, norm)) in records.into_iter().enumerate().rev() {
        let factor = op.base_mul(prefix, inverse);
        inverse = op.base_mul(inverse, norm);
        let inv = op.denominator_inverse(h, factor);
        let ratio = op.mul(u, inv);
        let w = op.mul(d, ratio);
        let v = op.mul(n, ratio);
        let x = op.sub(
            op.add(op.sub(op.sq(w), op.add(v, v)), op.add(p.x, q.x)),
            r.x,
        );
        let inner = op.add(op.sub(op.sub(r.x, x), op.add(s, s)), v);
        let y = op.add(op.mul(w, inner), op.add(op.add(p.y, q.y), r.y));
        out[i] = Point::finite(x, y);
    }
    Ok(out)
}

#[cfg(test)]
mod cancel_tests {
    use super::*;
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    struct P {
        x: i64,
        y: i64,
    }
    fn f(x: i64) -> i64 {
        x.rem_euclid(17)
    }
    fn inv(x: i64) -> i64 {
        assert_ne!(f(x), 0);
        (1..17).find(|i| f(x * i) == 1).unwrap()
    }
    fn add(a: Option<P>, b: Option<P>) -> Option<P> {
        let (a, b) = match (a, b) {
            (None, b) => return b,
            (a, None) => return a,
            (Some(a), Some(b)) => (a, b),
        };
        if a.x == b.x && f(a.y + b.y) == 0 {
            return None;
        }
        let l = if a == b {
            f(3 * a.x * a.x * inv(2 * a.y))
        } else {
            f((b.y - a.y) * inv(b.x - a.x))
        };
        let x = f(l * l - a.x - b.x);
        Some(P {
            x,
            y: f(l * (a.x - x) - a.y),
        })
    }
    #[test]
    fn exhaustive_toy_h_only_and_opposite_domain() {
        let mut points = vec![None];
        for x in 0..17 {
            for y in 0..17 {
                if f(y * y) == f(x * x * x + 3) {
                    points.push(Some(P { x, y }));
                }
            }
        }
        assert_eq!(points.len(), 18);
        let mut ordinary = 0;
        let mut opposite = 0;
        let mut guards = [0; 2];
        for a in &points {
            for b in &points {
                for c in &points {
                    let want = add(add(*a, *b), *c);
                    let (p, q, r) = match (a, b, c) {
                        (Some(p), Some(q), Some(r)) => (*p, *q, *r),
                        _ => {
                            guards[0] += 1;
                            continue;
                        }
                    };
                    let d = f(q.x - p.x);
                    let n = f(q.y - p.y);
                    let s = f(p.x + q.x + r.x);
                    let h = f(d * d * s - n * n);
                    if h == 0 {
                        guards[1] += 1;
                        continue;
                    }
                    let u = f(d * (p.y + r.y) + n * (r.x - p.x));
                    let ratio = f(u * inv(h));
                    let w = f(d * ratio);
                    let v = f(n * ratio);
                    let x = f(w * w - 2 * v + p.x + q.x - r.x);
                    let y = f(w * (r.x - x - 2 * s + v) + p.y + q.y + r.y);
                    assert_eq!(Some(P { x, y }), want);
                    ordinary += 1;
                    if d == 0 {
                        assert_eq!(f(p.y + q.y), 0);
                        assert_eq!(Some(P { x, y }), Some(r));
                        opposite += 1;
                    }
                }
            }
        }
        assert!(ordinary > 0 && opposite > 0 && guards.iter().all(|&v| v > 0));
        eprintln!(
            "H_CANCEL_TOY attempts={}ordinary={ordinary}opposite={opposite}guards={guards:?}",
            18usize.pow(3)
        );
    }
    #[test]
    fn changed_formula_field_ledgers() {
        fn check<F: PointField>(input: [Point<F>; 3], expected: [u64; 4]) {
            let op = Ops::new();
            let new = batch(&[input], &op).unwrap();
            let c = op.cost.get();
            assert_eq!([c.mul, c.square, c.fused, c.inverse], expected);
            let control = Ops::new();
            assert_eq!(
                crate::original::direct_batch(&[input], &control).unwrap(),
                new
            );
            let binary = Ops::new();
            assert_eq!(crate::original::two_binary(&[input], &binary).unwrap(), new);
        }
        let p = [
            Point::finite(Fq::from(1u64), Fq::from(2u64)),
            Point::finite(Fq::from(4u64), Fq::from(7u64)),
            Point::finite(Fq::from(9u64), Fq::from(13u64)),
        ];
        check(p, [7, 2, 2, 1]);
        check(
            p.map(|p| {
                Point::finite(
                    ark_bn254::Fq2::new(p.x, Fq::ONE),
                    ark_bn254::Fq2::new(p.y, Fq::from(3u64)),
                )
            }),
            [11, 0, 15, 1],
        );
    }
}
