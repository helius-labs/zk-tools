// The point operations the bucket kernels need beyond field.glsl.
// The first accumulated point of a run has unit zz and zzz.
xyzz xyzz_add_affine_pair(xyzz p, affine q) {
  if (fp_is_zero(q.x) && fp_is_zero(q.y))
    return p;
  if (fp_is_zero(p.zz))
    return xyzz_from_affine(q);
  const fp dx = fp_sub(q.x, p.x);
  const fp dy = fp_sub(q.y, p.y);
  if (fp_is_zero(dx))
    return fp_is_zero(dy) ? xyzz_double(p) : xyzz_infinity();
  const fp pp = fp_square(dx);
  const fp ppp = fp_mul(dx, pp);
  const fp q2 = fp_mul(p.x, pp);
  xyzz r;
  r.x = fp_sub(fp_sub(fp_sub(fp_square(dy), ppp), q2), q2);
  r.y = fp_sub(fp_mul(fp_sub(q2, r.x), dy), fp_mul(p.y, ppp));
  r.zz = pp;
  r.zzz = ppp;
  return r;
}
// k * p by double-and-add over k's bits.
xyzz xyzz_mul_small(xyzz p, uint k) {
  xyzz r = xyzz_infinity();
  if (k == 0u)
    return r;
  for (int bit = findMSB(k); bit >= 0; bit--) {
    r = xyzz_double(r);
    if (((k >> uint(bit)) & 1u) == 1u)
      r = xyzz_add(r, p);
  }
  return r;
}
