// points.glsl over G2: the point operations the bucket kernels need beyond field.glsl.
// The first accumulated point of a run has unit zz and zzz.
xyzz2 xyzz2_add_affine_pair(xyzz2 p, affine2 q) {
  if (e2_is_zero(q.x) && e2_is_zero(q.y))
    return p;
  if (e2_is_zero(p.zz))
    return xyzz2_from_affine(q);
  const fp2 dx = e2_sub(q.x, p.x);
  const fp2 dy = e2_sub(q.y, p.y);
  if (e2_is_zero(dx))
    return e2_is_zero(dy) ? xyzz2_double(p) : xyzz2_infinity();
  const fp2 pp = e2_square(dx);
  const fp2 ppp = e2_mul(dx, pp);
  const fp2 q2 = e2_mul(p.x, pp);
  xyzz2 r;
  r.x = e2_sub(e2_sub(e2_sub(e2_square(dy), ppp), q2), q2);
  r.y = e2_sub(e2_mul(e2_sub(q2, r.x), dy), e2_mul(p.y, ppp));
  r.zz = pp;
  r.zzz = ppp;
  return r;
}
// k * p by double-and-add over k's bits.
xyzz2 xyzz2_mul_small(xyzz2 p, uint k) {
  xyzz2 r = xyzz2_infinity();
  if (k == 0u)
    return r;
  for (int bit = findMSB(k); bit >= 0; bit--) {
    r = xyzz2_double(r);
    if (((k >> uint(bit)) & 1u) == 1u)
      r = xyzz2_add(r, p);
  }
  return r;
}
