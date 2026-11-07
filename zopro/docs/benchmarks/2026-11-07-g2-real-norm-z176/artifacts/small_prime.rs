// Independent F43[u]/(u²+1) rational oracle and direct all-other norm products.
// No native Field, prefix chains, product butterfly or group library is used.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct E(u64, u64);
impl E {
    fn add(self, b: Self) -> Self {
        Self((self.0 + b.0) % 43, (self.1 + b.1) % 43)
    }
    fn sub(self, b: Self) -> Self {
        Self((self.0 + 43 - b.0) % 43, (self.1 + 43 - b.1) % 43)
    }
    fn mul(self, b: Self) -> Self {
        Self(
            (self.0 * b.0 + 1849 - self.1 * b.1) % 43,
            (self.0 * b.1 + self.1 * b.0) % 43,
        )
    }
    fn scale(self, b: u64) -> Self {
        Self(self.0 * b % 43, self.1 * b % 43)
    }
    fn conj(self) -> Self {
        Self(self.0, (43 - self.1) % 43)
    }
    fn norm(self) -> u64 {
        (self.0 * self.0 + self.1 * self.1) % 43
    }
    fn inv(self) -> Self {
        let n = self.norm();
        assert!(n != 0);
        self.conj()
            .scale((1..43).find(|&v| v * n % 43 == 1).unwrap())
    }
}
fn affine_pair(a: (E, E), b: (E, E)) -> Option<(E, E)> {
    if a.0 == b.0 {
        return None;
    }
    let l = b.1.sub(a.1).mul(b.0.sub(a.0).inv());
    let x = l.mul(l).sub(a.0).sub(b.0);
    let y = l.mul(a.0.sub(x)).sub(a.1);
    Some((x, y))
}
fn small_prime() -> Value {
    let elems = (0..43)
        .flat_map(|x| (0..43).map(move |y| E(x, y)))
        .collect::<Vec<_>>();
    let mut roots = BTreeMap::new();
    for &y in &elems {
        roots.entry(y.mul(y)).or_insert(y);
    }
    let points = elems
        .iter()
        .filter_map(|&x| roots.get(&x.mul(x).mul(x).add(E(3, 1))).map(|&y| (x, y)))
        .collect::<Vec<_>>();
    assert!(points.len() > 500);
    for &a in &elems {
        assert_eq!(a.norm() == 0, a == E(0, 0));
    }
    let mut outputs = 0;
    let mut carry_outputs = 0;
    let mut rejected = 0;
    for m in 1..=16 {
        for shift in 0..12 {
            for z in [1u64, 7, 42] {
                let raw = (0..2 * m)
                    .map(|j| points[(shift * 37 + j * 11) % points.len()])
                    .collect::<Vec<_>>();
                let zs = z * z % 43;
                let zt = zs * z % 43;
                let scaled = raw
                    .iter()
                    .map(|&(x, y)| (x.scale(zs), y.scale(zt)))
                    .collect::<Vec<_>>();
                let denominators = (0..m)
                    .map(|j| scaled[2 * j + 1].0.sub(scaled[2 * j].0))
                    .collect::<Vec<_>>();
                if denominators.iter().any(|d| d.norm() == 0) {
                    rejected += 1;
                    continue;
                }
                let norms = denominators.iter().map(|d| d.norm()).collect::<Vec<_>>();
                let d = norms.iter().fold(1, |a, &b| a * b % 43);
                let s = d * d % 43;
                let t = s * d % 43;
                let zi = E(z * d % 43, 0).inv();
                let zi2 = zi.mul(zi);
                let zi3 = zi2.mul(zi);
                for j in 0..m {
                    let q = (0..m).filter(|&k| k != j).fold(1, |a, k| a * norms[k] % 43);
                    let (a, b) = (scaled[2 * j], scaled[2 * j + 1]);
                    let u = b.1.sub(a.1).mul(denominators[j].conj()).scale(q);
                    let x = u.mul(u).sub(a.0.add(b.0).scale(s));
                    let y = u.mul(a.0.scale(s).sub(x)).sub(a.1.scale(t));
                    assert_eq!(
                        Some((x.mul(zi2), y.mul(zi3))),
                        affine_pair(raw[2 * j], raw[2 * j + 1])
                    );
                    outputs += 1;
                }
                let carried = points[(shift * 17 + 9) % points.len()];
                let lifted = (carried.0.scale(zs * s % 43), carried.1.scale(zt * t % 43));
                assert_eq!((lifted.0.mul(zi2), lifted.1.mul(zi3)), carried);
                carry_outputs += 1;
            }
        }
    }
    json!({"pair_outputs":outputs,"carry_outputs":carry_outputs,"singular_levels_declined":rejected,"all1849_norm_zero_equivalences":true,"curve_B":"3+u","coordinate_field":"F43[u]/(u²+1)"})
}
