// G1 in extended Jacobian coordinates (x, y, zz, zzz): affine x is x / zz,
// affine y is y / zzz, zz = z^2, zzz = z^3; zz = 0 marks infinity. The
// affine identity is (0, 0), which is not on the curve.
struct Affine { x: Fp, y: Fp, }
struct Xyzz { x: Fp, y: Fp, zz: Fp, zzz: Fp, }

fn xyzz_infinity() -> Xyzz { return Xyzz(fp_zero(), fp_zero(), fp_zero(), fp_zero()); }
fn xyzz_is_infinity(p: Xyzz) -> bool { return fp_is_zero(p.zz); }
fn affine_is_infinity(p: Affine) -> bool { return fp_is_zero(p.x) && fp_is_zero(p.y); }
fn xyzz_from_affine(p: Affine) -> Xyzz {
    if (affine_is_infinity(p)) { return xyzz_infinity(); }
    return Xyzz(p.x, p.y, fp_one(), fp_one());
}
fn xyzz_double(p: Xyzz) -> Xyzz {
    if (xyzz_is_infinity(p) || fp_is_zero(p.y)) { return xyzz_infinity(); }
    let u = fp_add(p.y, p.y);
    let v = fp_square(u);
    let w = fp_mul(u, v);
    let s = fp_mul(p.x, v);
    let xx = fp_square(p.x);
    let m = fp_add(fp_add(xx, xx), xx);
    let x = fp_sub(fp_sub(fp_square(m), s), s);
    let y = fp_sub(fp_mul(fp_sub(s, x), m), fp_mul(w, p.y));
    return Xyzz(x, y, fp_mul(v, p.zz), fp_mul(w, p.zzz));
}
fn xyzz_add_mixed(p: Xyzz, q: Affine) -> Xyzz {
    if (affine_is_infinity(q)) { return p; }
    if (xyzz_is_infinity(p)) { return xyzz_from_affine(q); }
    let dx = fp_sub(fp_mul(q.x, p.zz), p.x);
    let dy = fp_sub(fp_mul(q.y, p.zzz), p.y);
    if (fp_is_zero(dx)) {
        if (fp_is_zero(dy)) { return xyzz_double(p); }
        return xyzz_infinity();
    }
    let pp = fp_square(dx);
    let ppp = fp_mul(dx, pp);
    let q2 = fp_mul(p.x, pp);
    let x = fp_sub(fp_sub(fp_sub(fp_square(dy), ppp), q2), q2);
    let y = fp_sub(fp_mul(fp_sub(q2, x), dy), fp_mul(p.y, ppp));
    return Xyzz(x, y, fp_mul(p.zz, pp), fp_mul(p.zzz, ppp));
}
// The first accumulated point has unit zz and zzz.
fn xyzz_add_affine_pair(p: Xyzz, q: Affine) -> Xyzz {
    if (affine_is_infinity(q)) { return p; }
    if (xyzz_is_infinity(p)) { return xyzz_from_affine(q); }
    let dx = fp_sub(q.x, p.x);
    let dy = fp_sub(q.y, p.y);
    if (fp_is_zero(dx)) {
        if (fp_is_zero(dy)) { return xyzz_double(p); }
        return xyzz_infinity();
    }
    let pp = fp_square(dx);
    let ppp = fp_mul(dx, pp);
    let q2 = fp_mul(p.x, pp);
    let x = fp_sub(fp_sub(fp_sub(fp_square(dy), ppp), q2), q2);
    let y = fp_sub(fp_mul(fp_sub(q2, x), dy), fp_mul(p.y, ppp));
    return Xyzz(x, y, pp, ppp);
}
fn xyzz_add(p: Xyzz, q: Xyzz) -> Xyzz {
    if (xyzz_is_infinity(q)) { return p; }
    if (xyzz_is_infinity(p)) { return q; }
    let u1 = fp_mul(p.x, q.zz);
    let s1 = fp_mul(p.y, q.zzz);
    let dx = fp_sub(fp_mul(q.x, p.zz), u1);
    let dy = fp_sub(fp_mul(q.y, p.zzz), s1);
    if (fp_is_zero(dx)) {
        if (fp_is_zero(dy)) { return xyzz_double(p); }
        return xyzz_infinity();
    }
    let pp = fp_square(dx);
    let ppp = fp_mul(dx, pp);
    let q2 = fp_mul(u1, pp);
    let x = fp_sub(fp_sub(fp_sub(fp_square(dy), ppp), q2), q2);
    let y = fp_sub(fp_mul(fp_sub(q2, x), dy), fp_mul(s1, ppp));
    return Xyzz(x, y, fp_mul(fp_mul(p.zz, q.zz), pp), fp_mul(fp_mul(p.zzz, q.zzz), ppp));
}
fn xyzz_shift(p: Xyzz, bits: u32) -> Xyzz {
    var r = p;
    for (var i = 0u; i < bits; i++) { r = xyzz_double(r); }
    return r;
}
