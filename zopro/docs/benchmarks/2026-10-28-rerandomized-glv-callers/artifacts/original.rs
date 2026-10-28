use super::*;
pub(super) struct FixedBase<C:CurveConfig>(pub(super) Vec<Affine<C>>);
impl<C:CurveConfig> FixedBase<C> { const POINTS:usize=64*15;
pub(crate) fn new(base: Affine<C>) -> Self {
        let mut points = Vec::with_capacity(Self::POINTS);
        let mut shifted = base.to_projective();
        for _ in 0..64 {
            let mut multiple = shifted;
            for _ in 1..16 {
                points.push(multiple);
                multiple += shifted;
            }
            for _ in 0..4 {
                shifted = shifted.double();
            }
        }
        Self(Projective::batch_to_affine(&points))
    }
pub(crate) fn mul(&self, scalar: Fr) -> Projective<C> {
        let words = scalar.to_canonical();
        let mut result = Projective::IDENTITY;
        for window in 0..64 {
            let digit = ((words[window / 16] >> (4 * (window % 16))) & 15) as usize;
            if digit != 0 {
                result += self.0[window * 15 + digit - 1];
            }
        }
        result
    }
}
pub(super) fn blinding_digits<const W: usize>(scalar: zopro_bn254::Fr) -> ([i8; 256], usize) {
    let mut words = scalar.to_canonical();
    let mut digits = [0; 256];
    let mut used = 0;
    while words != [0; 4] {
        assert!(used < 256);
        if words[0] & 1 != 0 {
            let mut digit = (words[0] & ((1 << W) - 1)) as i16;
            if digit > (1 << (W - 1)) {
                digit -= 1 << W;
            }
            digits[used] = digit as i8;
            let mut carry = digit.unsigned_abs() as u64;
            for word in &mut words {
                let (v, next) = if digit > 0 {
                    word.overflowing_sub(carry)
                } else {
                    word.overflowing_add(carry)
                };
                *word = v;
                carry = u64::from(next);
            }
            assert_eq!(carry, 0);
        }
        for j in 0..3 {
            words[j] = (words[j] >> 1) | (words[j + 1] << 63);
        }
        words[3] >>= 1;
        used += 1;
    }
    (digits, used)
}
pub(super) fn odd_multiples<C: CurveConfig>(base: Projective<C>) -> Vec<Affine<C>> {
    let step = base.double();
    let mut p = base;
    let mut points = Vec::with_capacity(8);
    for _ in 0..8 {
        points.push(p);
        p += step;
    }
    Projective::batch_to_affine(&points)
}
pub(super) fn add_digit<C: CurveConfig>(out: &mut Projective<C>, table: &[Affine<C>], d: i8) {
    if d != 0 {
        let p = table[d.unsigned_abs() as usize / 2];
        *out += if d < 0 { -p } else { p };
    }
}
pub(super) fn pair_g1(base: G1Projective, left: Fr, right: Fr) -> (G1Projective, G1Projective) {
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
pub(super) fn joint_g2(
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
