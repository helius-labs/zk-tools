// The BN254 quadratic extension, i^2 = -1, over the eight-limb Fp.
struct Fp2 { a0: Fp, a1: Fp, }
fn fp2_zero() -> Fp2 { return Fp2(fp_zero(), fp_zero()); }
fn fp2_one() -> Fp2 { return Fp2(fp_one(), fp_zero()); }
fn fp2_is_zero(a: Fp2) -> bool { return fp_is_zero(a.a0) && fp_is_zero(a.a1); }
fn fp2_add(a: Fp2, b: Fp2) -> Fp2 { return Fp2(fp_add(a.a0, b.a0), fp_add(a.a1, b.a1)); }
fn fp2_sub(a: Fp2, b: Fp2) -> Fp2 { return Fp2(fp_sub(a.a0, b.a0), fp_sub(a.a1, b.a1)); }
fn fp2_neg(a: Fp2) -> Fp2 { return Fp2(fp_neg(a.a0), fp_neg(a.a1)); }
fn fp2_mul(a: Fp2, b: Fp2) -> Fp2 {
    let p0 = fp_mul(a.a0, b.a0);
    let p1 = fp_mul(a.a1, b.a1);
    let p2 = fp_mul(fp_add(a.a0, a.a1), fp_add(b.a0, b.a1));
    return Fp2(fp_sub(p0, p1), fp_sub(fp_sub(p2, p0), p1));
}
fn fp2_square(a: Fp2) -> Fp2 {
    let real = fp_mul(fp_add(a.a0, a.a1), fp_sub(a.a0, a.a1));
    let product = fp_mul(a.a0, a.a1);
    return Fp2(real, fp_add(product, product));
}
