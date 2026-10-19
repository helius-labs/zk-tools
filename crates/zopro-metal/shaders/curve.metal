inline fp_t e_add(fp_t a, fp_t b) { return fp_add(a, b); }
inline fp2_t e_add(fp2_t a, fp2_t b) { return fp2_add(a, b); }
inline fp_t e_sub(fp_t a, fp_t b) { return fp_sub(a, b); }
inline fp2_t e_sub(fp2_t a, fp2_t b) { return fp2_sub(a, b); }
inline fp_t e_mul(fp_t a, fp_t b) { return fp_mul_g1(a, b); }
inline fp2_t e_mul(fp2_t a, fp2_t b) { return fp2_mul(a, b); }
inline fp_t e_square(fp_t a) { return fp_square_g1(a); }
inline fp2_t e_square(fp2_t a) { return fp2_square(a); }
inline fp_t e_neg(fp_t a) { return fp_neg(a); }
inline fp2_t e_neg(fp2_t a) { return fp2_neg(a); }
inline fp_t e_inv(fp_t a) { return fp_inv(a); }
inline fp2_t e_inv(fp2_t a) { return fp2_inv(a); }
inline bool e_is_zero(thread const fp_t &a) { return fp_is_zero(a); }
inline bool e_is_zero(thread const fp2_t &a) { return fp2_is_zero(a); }

template <typename E> E e_zero();
template <> inline fp_t e_zero<fp_t>() { return fp_zero(); }
template <> inline fp2_t e_zero<fp2_t>() { return fp2_zero(); }
template <typename E> E e_one();
template <> inline fp_t e_one<fp_t>() { return fp_one(); }
template <> inline fp2_t e_one<fp2_t>() { return fp2_one(); }

template <typename E> struct affine_t {
  E x;
  E y;
};

template <typename E> struct jacobian_t {
  E x;
  E y;
  E z;
};

// 1. Extended Jacobian coordinates bind affine x to X over ZZ and affine y to
//    Y over ZZZ. Zero ZZ marks infinity.
template <typename E> struct xyzz_t {
  E x;
  E y;
  E zz;
  E zzz;
};

template <typename E> inline xyzz_t<E> xyzz_infinity() {
  return xyzz_t<E>{e_zero<E>(), e_zero<E>(), e_zero<E>(), e_zero<E>()};
}

template <typename E> inline bool xyzz_is_infinity(thread const xyzz_t<E> &p) {
  return e_is_zero(p.zz);
}

template <typename E>
inline bool affine_is_infinity(thread const affine_t<E> &p) {
  return e_is_zero(p.x) && e_is_zero(p.y);
}

template <typename E> inline xyzz_t<E> xyzz_from_affine(affine_t<E> p) {
  if (affine_is_infinity(p))
    return xyzz_infinity<E>();
  return xyzz_t<E>{p.x, p.y, e_one<E>(), e_one<E>()};
}

template <typename E> inline xyzz_t<E> xyzz_double(xyzz_t<E> p) {
  if (xyzz_is_infinity(p) || e_is_zero(p.y))
    return xyzz_infinity<E>();
  const E u = e_add(p.y, p.y);
  const E v = e_square(u);
  const E w = e_mul(u, v);
  const E s = e_mul(p.x, v);
  const E xx = e_square(p.x);
  const E m = e_add(e_add(xx, xx), xx);
  xyzz_t<E> r;
  r.x = e_sub(e_sub(e_square(m), s), s);
  r.y = e_sub(e_mul(e_sub(s, r.x), m), e_mul(w, p.y));
  r.zz = e_mul(v, p.zz);
  r.zzz = e_mul(w, p.zzz);
  return r;
}

template <typename E>
inline xyzz_t<E> xyzz_add_mixed(xyzz_t<E> p, affine_t<E> q) {
  if (affine_is_infinity(q))
    return p;
  if (xyzz_is_infinity(p))
    return xyzz_from_affine(q);
  const E dx = e_sub(e_mul(q.x, p.zz), p.x);
  const E dy = e_sub(e_mul(q.y, p.zzz), p.y);
  if (e_is_zero(dx))
    return e_is_zero(dy) ? xyzz_double(p) : xyzz_infinity<E>();
  const E pp = e_square(dx);
  const E ppp = e_mul(dx, pp);
  const E q2 = e_mul(p.x, pp);
  xyzz_t<E> r;
  r.x = e_sub(e_sub(e_sub(e_square(dy), ppp), q2), q2);
  r.y = e_sub(e_mul(e_sub(q2, r.x), dy), e_mul(p.y, ppp));
  r.zz = e_mul(p.zz, pp);
  r.zzz = e_mul(p.zzz, ppp);
  return r;
}

// 2. The first accumulated point has unit ZZ and ZZZ.
template <typename E>
inline xyzz_t<E> xyzz_add_affine_pair(xyzz_t<E> p, affine_t<E> q) {
  if (affine_is_infinity(q))
    return p;
  if (xyzz_is_infinity(p))
    return xyzz_from_affine(q);
  const E dx = e_sub(q.x, p.x);
  const E dy = e_sub(q.y, p.y);
  if (e_is_zero(dx))
    return e_is_zero(dy) ? xyzz_double(p) : xyzz_infinity<E>();
  const E pp = e_square(dx);
  const E ppp = e_mul(dx, pp);
  const E q2 = e_mul(p.x, pp);
  xyzz_t<E> r;
  r.x = e_sub(e_sub(e_sub(e_square(dy), ppp), q2), q2);
  r.y = e_sub(e_mul(e_sub(q2, r.x), dy), e_mul(p.y, ppp));
  r.zz = pp;
  r.zzz = ppp;
  return r;
}

template <typename E> inline xyzz_t<E> xyzz_add(xyzz_t<E> p, xyzz_t<E> q) {
  if (xyzz_is_infinity(q))
    return p;
  if (xyzz_is_infinity(p))
    return q;
  const E u1 = e_mul(p.x, q.zz);
  const E s1 = e_mul(p.y, q.zzz);
  const E dx = e_sub(e_mul(q.x, p.zz), u1);
  const E dy = e_sub(e_mul(q.y, p.zzz), s1);
  if (e_is_zero(dx))
    return e_is_zero(dy) ? xyzz_double(p) : xyzz_infinity<E>();
  const E pp = e_square(dx);
  const E ppp = e_mul(dx, pp);
  const E q2 = e_mul(u1, pp);
  xyzz_t<E> r;
  r.x = e_sub(e_sub(e_sub(e_square(dy), ppp), q2), q2);
  r.y = e_sub(e_mul(e_sub(q2, r.x), dy), e_mul(s1, ppp));
  r.zz = e_mul(e_mul(p.zz, q.zz), pp);
  r.zzz = e_mul(e_mul(p.zzz, q.zzz), ppp);
  return r;
}

// 3. One inversion of ZZ * ZZZ recovers both affine coordinates.
template <typename E> inline affine_t<E> xyzz_to_affine(xyzz_t<E> p) {
  if (xyzz_is_infinity(p))
    return affine_t<E>{e_zero<E>(), e_zero<E>()};
  const E t = e_inv(e_mul(p.zz, p.zzz));
  return affine_t<E>{e_mul(e_mul(p.x, p.zzz), t), e_mul(e_mul(p.y, p.zz), t)};
}

template <typename E> inline jacobian_t<E> xyzz_to_jacobian(xyzz_t<E> p) {
  if (xyzz_is_infinity(p))
    return jacobian_t<E>{e_zero<E>(), e_one<E>(), e_zero<E>()};
  return jacobian_t<E>{e_mul(p.x, p.zz), e_mul(p.y, p.zzz), p.zz};
}
