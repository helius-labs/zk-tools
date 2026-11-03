// G2 in extended Jacobian coordinates over Fp2: xyzz.wgsl with Fp2.
// G1 in extended Jacobian coordinates (x, y, zz, zzz): affine x is x / zz,
// affine y is y / zzz, zz = z^2, zzz = z^3; zz = 0 marks infinity. The
// affine identity is (0, 0), which is not on the curve.
struct Affine2 { x: Fp2, y: Fp2, }
struct Xyzz2 { x: Fp2, y: Fp2, zz: Fp2, zzz: Fp2, }

fn xyzz2_infinity() -> Xyzz2 { return Xyzz2(fp2_zero(), fp2_zero(), fp2_zero(), fp2_zero()); }
fn xyzz2_is_infinity(p: Xyzz2) -> bool { return fp2_is_zero(p.zz); }
fn affine2_is_infinity(p: Affine2) -> bool { return fp2_is_zero(p.x) && fp2_is_zero(p.y); }
fn xyzz2_from_affine(p: Affine2) -> Xyzz2 {
    if (affine2_is_infinity(p)) { return xyzz2_infinity(); }
    return Xyzz2(p.x, p.y, fp2_one(), fp2_one());
}
fn xyzz2_double(p: Xyzz2) -> Xyzz2 {
    if (xyzz2_is_infinity(p) || fp2_is_zero(p.y)) { return xyzz2_infinity(); }
    let u = fp2_add(p.y, p.y);
    let v = fp2_square(u);
    let w = fp2_mul(u, v);
    let s = fp2_mul(p.x, v);
    let xx = fp2_square(p.x);
    let m = fp2_add(fp2_add(xx, xx), xx);
    let x = fp2_sub(fp2_sub(fp2_square(m), s), s);
    let y = fp2_sub(fp2_mul(fp2_sub(s, x), m), fp2_mul(w, p.y));
    return Xyzz2(x, y, fp2_mul(v, p.zz), fp2_mul(w, p.zzz));
}
fn xyzz2_add_mixed(p: Xyzz2, q: Affine2) -> Xyzz2 {
    if (affine2_is_infinity(q)) { return p; }
    if (xyzz2_is_infinity(p)) { return xyzz2_from_affine(q); }
    let dx = fp2_sub(fp2_mul(q.x, p.zz), p.x);
    let dy = fp2_sub(fp2_mul(q.y, p.zzz), p.y);
    if (fp2_is_zero(dx)) {
        if (fp2_is_zero(dy)) { return xyzz2_double(p); }
        return xyzz2_infinity();
    }
    let pp = fp2_square(dx);
    let ppp = fp2_mul(dx, pp);
    let q2 = fp2_mul(p.x, pp);
    let x = fp2_sub(fp2_sub(fp2_sub(fp2_square(dy), ppp), q2), q2);
    let y = fp2_sub(fp2_mul(fp2_sub(q2, x), dy), fp2_mul(p.y, ppp));
    return Xyzz2(x, y, fp2_mul(p.zz, pp), fp2_mul(p.zzz, ppp));
}
// The first accumulated point has unit zz and zzz.
fn xyzz2_add_affine_pair(p: Xyzz2, q: Affine2) -> Xyzz2 {
    if (affine2_is_infinity(q)) { return p; }
    if (xyzz2_is_infinity(p)) { return xyzz2_from_affine(q); }
    let dx = fp2_sub(q.x, p.x);
    let dy = fp2_sub(q.y, p.y);
    if (fp2_is_zero(dx)) {
        if (fp2_is_zero(dy)) { return xyzz2_double(p); }
        return xyzz2_infinity();
    }
    let pp = fp2_square(dx);
    let ppp = fp2_mul(dx, pp);
    let q2 = fp2_mul(p.x, pp);
    let x = fp2_sub(fp2_sub(fp2_sub(fp2_square(dy), ppp), q2), q2);
    let y = fp2_sub(fp2_mul(fp2_sub(q2, x), dy), fp2_mul(p.y, ppp));
    return Xyzz2(x, y, pp, ppp);
}
fn xyzz2_add(p: Xyzz2, q: Xyzz2) -> Xyzz2 {
    if (xyzz2_is_infinity(q)) { return p; }
    if (xyzz2_is_infinity(p)) { return q; }
    let u1 = fp2_mul(p.x, q.zz);
    let s1 = fp2_mul(p.y, q.zzz);
    let dx = fp2_sub(fp2_mul(q.x, p.zz), u1);
    let dy = fp2_sub(fp2_mul(q.y, p.zzz), s1);
    if (fp2_is_zero(dx)) {
        if (fp2_is_zero(dy)) { return xyzz2_double(p); }
        return xyzz2_infinity();
    }
    let pp = fp2_square(dx);
    let ppp = fp2_mul(dx, pp);
    let q2 = fp2_mul(u1, pp);
    let x = fp2_sub(fp2_sub(fp2_sub(fp2_square(dy), ppp), q2), q2);
    let y = fp2_sub(fp2_mul(fp2_sub(q2, x), dy), fp2_mul(s1, ppp));
    return Xyzz2(x, y, fp2_mul(fp2_mul(p.zz, q.zz), pp), fp2_mul(fp2_mul(p.zzz, q.zzz), ppp));
}
fn xyzz2_shift(p: Xyzz2, bits: u32) -> Xyzz2 {
    var r = p;
    for (var i = 0u; i < bits; i++) { r = xyzz2_double(r); }
    return r;
}
