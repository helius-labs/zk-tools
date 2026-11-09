use zopro_bn254::{Affine, CurveConfig, Field, Projective};

/// The accepted mixed addition with a standard Z1=Z2=1 specialization.
#[inline]
pub fn add<C: CurveConfig>(left: &Projective<C>, right: &Affine<C>) -> Projective<C> {
    if right.is_identity() {
        return *left;
    }
    if left.is_identity() {
        return right.to_projective();
    }
    if left.z != C::Base::ONE {
        return left.add_affine(right);
    }
    if left.x == right.x {
        return if left.y == right.y {
            left.double()
        } else {
            Projective::IDENTITY
        };
    }
    let h = right.x - left.x;
    let hh = h.square();
    let i = hh.double().double();
    let j = h * i;
    let r = (right.y - left.y).double();
    let v = left.x * i;
    let x = r.square() - j - v.double();
    let y = r * (v - x) - (left.y * j).double();
    Projective::new(x, y, h.double())
}

// Stable private diagnostic entrypoints in the actual qualification executable.
#[unsafe(no_mangle)]
#[inline(never)]
pub fn unit_z_mixed_g1(a: &zopro_bn254::G1Projective, b: &zopro_bn254::G1Affine) -> zopro_bn254::G1Projective {
    add(a, b)
}
#[unsafe(no_mangle)]
#[inline(never)]
pub fn unit_z_mixed_g2(a: &zopro_bn254::G2Projective, b: &zopro_bn254::G2Affine) -> zopro_bn254::G2Projective {
    add(a, b)
}
