// Four-query Groth16: A0=alpha+A, B0=beta2+B2, C0=K+H.
// For fresh uniform u != 0 and v in Fr, output
// A'=A0/u, B'=u*B0+u*v*delta2, C'=C0+v*A0.
// When A0 != 0, (A',B') is uniform on (G1\{0}) x G2. With
// nonzero delta2 and valid prime-order groups, the verification equation
// determines C', so this distribution is independent of the witness.
// It equals ordinary uniform-r/s proving conditioned on A' != 0, at
// statistical distance 1/|Fr| from the unconditioned distribution.
// If A0=0, Key::solve_rerandomized computes all five queries and samples
// ordinary proofs with the same condition. Never expose unblinded parts.
// Tables below are per-proof, constant-size, and retain no witness state.
fn odd_multiples<C: CurveConfig>(base: Projective<C>) -> Vec<Affine<C>> {
    let step = base.double();
    let mut p = base;
    let mut points = Vec::with_capacity(8);
    for _ in 0..8 {
        points.push(p);
        p += step;
    }
    Projective::batch_to_affine(&points)
}
fn add_digit<C: CurveConfig>(out: &mut Projective<C>, table: &[Affine<C>], d: i8) {
    if d != 0 {
        let p = table[d.unsigned_abs() as usize / 2];
        *out += if d < 0 { -p } else { p };
    }
}
fn pair_g1(base: G1Projective, left: Fr, right: Fr) -> (G1Projective, G1Projective) {
    let table = odd_multiples(base);
    let (ld, ln) = blinding_digits::<5>(left);
    let (rd, rn) = blinding_digits::<5>(right);
    let (mut l, mut r) = (G1Projective::IDENTITY, G1Projective::IDENTITY);
    for i in (0..ln.max(rn)).rev() {
        l = l.double();
        r = r.double();
        add_digit(&mut l, &table, ld[i]);
        add_digit(&mut r, &table, rd[i]);
    }
    (l, r)
}
fn joint_g2(
    base: G2Projective,
    u: Fr,
    delta: G2Affine,
    uv: Fr,
    prepared: Option<&FixedBase<G2Config>>,
) -> G2Projective {
    let table = odd_multiples(base);
    let delta_table = prepared
        .is_none()
        .then(|| odd_multiples(delta.to_projective()));
    let (ud, un) = blinding_digits::<5>(u);
    let (vd, vn) = blinding_digits::<5>(uv);
    let mut out = G2Projective::IDENTITY;
    for i in (0..un.max(vn)).rev() {
        out = out.double();
        add_digit(&mut out, &table, ud[i]);
        let d = vd[i];
        if d != 0 {
            // Prepared first window is delta,2delta,...,15delta.
            let p = if let Some(t) = prepared {
                t.0[d.unsigned_abs() as usize - 1]
            } else {
                delta_table.as_ref().unwrap()[d.unsigned_abs() as usize / 2]
            };
            out += if d < 0 { -p } else { p };
        }
    }
    out
}
impl PreparedFixedPoints {
    fn windowed_outputs(
        &self,
        a0: G1Projective,
        b0: G2Projective,
        c0: G1Projective,
        u: Fr,
        inverse: Fr,
        v: Fr,
    ) -> (G1Projective, G1Projective, G2Projective) {
        let ((a, add), b) = rayon::join(
            || pair_g1(a0, inverse, v),
            || {
                joint_g2(
                    b0,
                    u,
                    self.0.fixed.delta2,
                    u * v,
                    self.0.windows.as_ref().map(|(_, g2)| g2),
                )
            },
        );
        (a, c0 + add, b)
    }
}
