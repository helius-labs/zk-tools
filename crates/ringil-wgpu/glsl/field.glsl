// SPDX-License-Identifier: Apache-2.0
// BN254 Fp, Fp2, G1, G2 and Fr arithmetic for Vulkan GLSL (32-bit limbs,
// umulExtended, uaddCarry), from Aeglos's generated field_gen.glsl
// (helius-labs/aeglos, android branch, host/vulkan/gen_field.py), with the
// multiplication's eight rounds in a loop: Adreno compiled the straight-line
// form for 15-30 s per kernel (minutes for G2) and ran it slower.
// Copyright (c) 2026 Helius Blockchain Technologies, Inc.
#extension GL_EXT_control_flow_attributes : require
struct fp { uint l0, l1, l2, l3, l4, l5, l6, l7; };
const uint Q0 = 0xd87cfd47u;
const uint Q1 = 0x3c208c16u;
const uint Q2 = 0x6871ca8du;
const uint Q3 = 0x97816a91u;
const uint Q4 = 0x8181585du;
const uint Q5 = 0xb85045b6u;
const uint Q6 = 0xe131a029u;
const uint Q7 = 0x30644e72u;
const uint FP_INV = 0xe4866389u;
fp fp_zero() { return fp(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u); }
fp fp_one() { return fp(0xc58f0d9du, 0xd35d438du, 0xf5c70b3du, 0x0a78eb28u, 0x7879462cu, 0x666ea36fu, 0x9a07df2fu, 0x0e0a77c1u); }
bool fp_is_zero(fp a) { return (a.l0 | a.l1 | a.l2 | a.l3 | a.l4 | a.l5 | a.l6 | a.l7) == 0u; }
fp fp_reduce_once(uint t0, uint t1, uint t2, uint t3, uint t4, uint t5, uint t6, uint t7) {
  uint b = 0u, b1, b2;
  uint d0 = usubBorrow(t0, Q0, b1); d0 = usubBorrow(d0, b, b2); b = b1 | b2;
  uint d1 = usubBorrow(t1, Q1, b1); d1 = usubBorrow(d1, b, b2); b = b1 | b2;
  uint d2 = usubBorrow(t2, Q2, b1); d2 = usubBorrow(d2, b, b2); b = b1 | b2;
  uint d3 = usubBorrow(t3, Q3, b1); d3 = usubBorrow(d3, b, b2); b = b1 | b2;
  uint d4 = usubBorrow(t4, Q4, b1); d4 = usubBorrow(d4, b, b2); b = b1 | b2;
  uint d5 = usubBorrow(t5, Q5, b1); d5 = usubBorrow(d5, b, b2); b = b1 | b2;
  uint d6 = usubBorrow(t6, Q6, b1); d6 = usubBorrow(d6, b, b2); b = b1 | b2;
  uint d7 = usubBorrow(t7, Q7, b1); d7 = usubBorrow(d7, b, b2); b = b1 | b2;
  return fp(b != 0u ? t0 : d0, b != 0u ? t1 : d1, b != 0u ? t2 : d2, b != 0u ? t3 : d3, b != 0u ? t4 : d4, b != 0u ? t5 : d5, b != 0u ? t6 : d6, b != 0u ? t7 : d7);
}
fp fp_add(fp a, fp b) {
  uint c = 0u, c1, c2;
  uint s0 = uaddCarry(a.l0, b.l0, c1); s0 = uaddCarry(s0, c, c2); c = c1 + c2;
  uint s1 = uaddCarry(a.l1, b.l1, c1); s1 = uaddCarry(s1, c, c2); c = c1 + c2;
  uint s2 = uaddCarry(a.l2, b.l2, c1); s2 = uaddCarry(s2, c, c2); c = c1 + c2;
  uint s3 = uaddCarry(a.l3, b.l3, c1); s3 = uaddCarry(s3, c, c2); c = c1 + c2;
  uint s4 = uaddCarry(a.l4, b.l4, c1); s4 = uaddCarry(s4, c, c2); c = c1 + c2;
  uint s5 = uaddCarry(a.l5, b.l5, c1); s5 = uaddCarry(s5, c, c2); c = c1 + c2;
  uint s6 = uaddCarry(a.l6, b.l6, c1); s6 = uaddCarry(s6, c, c2); c = c1 + c2;
  uint s7 = uaddCarry(a.l7, b.l7, c1); s7 = uaddCarry(s7, c, c2); c = c1 + c2;
  return fp_reduce_once(s0, s1, s2, s3, s4, s5, s6, s7);
}
fp fp_sub(fp a, fp b) {
  uint b0 = 0u, b1, b2;
  uint d0 = usubBorrow(a.l0, b.l0, b1); d0 = usubBorrow(d0, b0, b2); b0 = b1 | b2;
  uint d1 = usubBorrow(a.l1, b.l1, b1); d1 = usubBorrow(d1, b0, b2); b0 = b1 | b2;
  uint d2 = usubBorrow(a.l2, b.l2, b1); d2 = usubBorrow(d2, b0, b2); b0 = b1 | b2;
  uint d3 = usubBorrow(a.l3, b.l3, b1); d3 = usubBorrow(d3, b0, b2); b0 = b1 | b2;
  uint d4 = usubBorrow(a.l4, b.l4, b1); d4 = usubBorrow(d4, b0, b2); b0 = b1 | b2;
  uint d5 = usubBorrow(a.l5, b.l5, b1); d5 = usubBorrow(d5, b0, b2); b0 = b1 | b2;
  uint d6 = usubBorrow(a.l6, b.l6, b1); d6 = usubBorrow(d6, b0, b2); b0 = b1 | b2;
  uint d7 = usubBorrow(a.l7, b.l7, b1); d7 = usubBorrow(d7, b0, b2); b0 = b1 | b2;
  const uint mask = 0u - b0;
  uint c = 0u, c1, c2;
  d0 = uaddCarry(d0, Q0 & mask, c1); d0 = uaddCarry(d0, c, c2); c = c1 + c2;
  d1 = uaddCarry(d1, Q1 & mask, c1); d1 = uaddCarry(d1, c, c2); c = c1 + c2;
  d2 = uaddCarry(d2, Q2 & mask, c1); d2 = uaddCarry(d2, c, c2); c = c1 + c2;
  d3 = uaddCarry(d3, Q3 & mask, c1); d3 = uaddCarry(d3, c, c2); c = c1 + c2;
  d4 = uaddCarry(d4, Q4 & mask, c1); d4 = uaddCarry(d4, c, c2); c = c1 + c2;
  d5 = uaddCarry(d5, Q5 & mask, c1); d5 = uaddCarry(d5, c, c2); c = c1 + c2;
  d6 = uaddCarry(d6, Q6 & mask, c1); d6 = uaddCarry(d6, c, c2); c = c1 + c2;
  d7 = uaddCarry(d7, Q7 & mask, c1); d7 = uaddCarry(d7, c, c2); c = c1 + c2;
  return fp(d0, d1, d2, d3, d4, d5, d6, d7);
}
fp fp_mul(fp a, fp b) {
  // The unrolled rounds of field_gen.glsl as a loop over b's limbs: an
  // eighth of the code, for shader compilers that take minutes over the
  // straight-line form.
  uint t0 = 0u; uint t1 = 0u; uint t2 = 0u; uint t3 = 0u; uint t4 = 0u; uint t5 = 0u; uint t6 = 0u; uint t7 = 0u;
  uint A, C, m, hi, lo, c1, c2, bj;
  for (uint j = 0u; j < 8u; j++) {
    switch (j) {
      case 0u: bj = b.l0; break;
      case 1u: bj = b.l1; break;
      case 2u: bj = b.l2; break;
      case 3u: bj = b.l3; break;
      case 4u: bj = b.l4; break;
      case 5u: bj = b.l5; break;
      case 6u: bj = b.l6; break;
      default: bj = b.l7; break;
    }
    A = 0u; umulExtended(a.l0, bj, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
    m = t0 * FP_INV; C = 0u; umulExtended(m, Q0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2;
    umulExtended(a.l1, bj, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
    umulExtended(m, Q1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
    umulExtended(a.l2, bj, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
    umulExtended(m, Q2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
    umulExtended(a.l3, bj, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
    umulExtended(m, Q3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
    umulExtended(a.l4, bj, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
    umulExtended(m, Q4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
    umulExtended(a.l5, bj, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
    umulExtended(m, Q5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
    umulExtended(a.l6, bj, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
    umulExtended(m, Q6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
    umulExtended(a.l7, bj, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
    umulExtended(m, Q7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
    t7 = C + A;
  }
  return fp_reduce_once(t0, t1, t2, t3, t4, t5, t6, t7);
}

fp fp_square(fp a) { return fp_mul(a, a); }
fp fp_neg(fp a) { return fp_sub(fp_zero(), a); }

struct affine { fp x; fp y; };
struct xyzz { fp x; fp y; fp zz; fp zzz; };

xyzz xyzz_infinity() { return xyzz(fp_zero(), fp_zero(), fp_zero(), fp_zero()); }

xyzz xyzz_from_affine(affine p) {
  if (fp_is_zero(p.x) && fp_is_zero(p.y))
    return xyzz_infinity();
  return xyzz(p.x, p.y, fp_one(), fp_one());
}

xyzz xyzz_double(xyzz p) {
  if (fp_is_zero(p.zz) || fp_is_zero(p.y))
    return xyzz_infinity();
  const fp u = fp_add(p.y, p.y);
  const fp v = fp_square(u);
  const fp w = fp_mul(u, v);
  const fp s = fp_mul(p.x, v);
  const fp xx = fp_square(p.x);
  const fp m = fp_add(fp_add(xx, xx), xx);
  xyzz r;
  r.x = fp_sub(fp_sub(fp_square(m), s), s);
  r.y = fp_sub(fp_mul(fp_sub(s, r.x), m), fp_mul(w, p.y));
  r.zz = fp_mul(v, p.zz);
  r.zzz = fp_mul(w, p.zzz);
  return r;
}

xyzz xyzz_add_mixed(xyzz p, affine q) {
  if (fp_is_zero(q.x) && fp_is_zero(q.y))
    return p;
  if (fp_is_zero(p.zz))
    return xyzz_from_affine(q);
  const fp dx = fp_sub(fp_mul(q.x, p.zz), p.x);
  const fp dy = fp_sub(fp_mul(q.y, p.zzz), p.y);
  if (fp_is_zero(dx)) {
#ifdef NO_DOUBLE
    return xyzz_infinity();
#else
    return fp_is_zero(dy) ? xyzz_double(p) : xyzz_infinity();
#endif
  }
  const fp pp = fp_square(dx);
  const fp ppp = fp_mul(dx, pp);
  const fp q2 = fp_mul(p.x, pp);
  xyzz r;
  r.x = fp_sub(fp_sub(fp_sub(fp_square(dy), ppp), q2), q2);
  r.y = fp_sub(fp_mul(fp_sub(q2, r.x), dy), fp_mul(p.y, ppp));
  r.zz = fp_mul(p.zz, pp);
  r.zzz = fp_mul(p.zzz, ppp);
  return r;
}

xyzz xyzz_add(xyzz p, xyzz q) {
  if (fp_is_zero(q.zz))
    return p;
  if (fp_is_zero(p.zz))
    return q;
  const fp u1 = fp_mul(p.x, q.zz);
  const fp s1 = fp_mul(p.y, q.zzz);
  const fp dx = fp_sub(fp_mul(q.x, p.zz), u1);
  const fp dy = fp_sub(fp_mul(q.y, p.zzz), s1);
  if (fp_is_zero(dx))
    return fp_is_zero(dy) ? xyzz_double(p) : xyzz_infinity();
  const fp pp = fp_square(dx);
  const fp ppp = fp_mul(dx, pp);
  const fp q2 = fp_mul(u1, pp);
  xyzz r;
  r.x = fp_sub(fp_sub(fp_sub(fp_square(dy), ppp), q2), q2);
  r.y = fp_sub(fp_mul(fp_sub(q2, r.x), dy), fp_mul(s1, ppp));
  r.zz = fp_mul(fp_mul(p.zz, q.zz), pp);
  r.zzz = fp_mul(fp_mul(p.zzz, q.zzz), ppp);
  return r;
}

struct fp2 { fp a0; fp a1; };
fp2 e2_zero() { return fp2(fp_zero(), fp_zero()); }
fp2 e2_one() { return fp2(fp_one(), fp_zero()); }
bool e2_is_zero(fp2 a) { return fp_is_zero(a.a0) && fp_is_zero(a.a1); }
fp2 e2_add(fp2 a, fp2 b) { return fp2(fp_add(a.a0, b.a0), fp_add(a.a1, b.a1)); }
fp2 e2_sub(fp2 a, fp2 b) { return fp2(fp_sub(a.a0, b.a0), fp_sub(a.a1, b.a1)); }
// (a0 + a1 u)(b0 + b1 u) with u^2 = -1, in three products.
#ifdef COMPACT_E2
// One multiplication site keeps the kernel small enough for the driver
// compiler; the products run as a loop the driver does not unroll.
fp2 e2_mul(fp2 a, fp2 b) {
  fp p0 = fp_zero(), p1 = fp_zero(), p2 = fp_zero();
  const fp sa = fp_add(a.a0, a.a1), sb = fp_add(b.a0, b.a1);
  [[dont_unroll]] for (int k = 0; k < 3; k++) {
    const fp x = k == 0 ? a.a0 : k == 1 ? a.a1 : sa;
    const fp y = k == 0 ? b.a0 : k == 1 ? b.a1 : sb;
    const fp m = fp_mul(x, y);
    if (k == 0)
      p0 = m;
    else if (k == 1)
      p1 = m;
    else
      p2 = m;
  }
  return fp2(fp_sub(p0, p1), fp_sub(fp_sub(p2, p0), p1));
}
#else
fp2 e2_mul(fp2 a, fp2 b) {
  const fp p0 = fp_mul(a.a0, b.a0);
  const fp p1 = fp_mul(a.a1, b.a1);
  const fp p2 = fp_mul(fp_add(a.a0, a.a1), fp_add(b.a0, b.a1));
  return fp2(fp_sub(p0, p1), fp_sub(fp_sub(p2, p0), p1));
}
#endif

fp2 e2_square(fp2 a) { return e2_mul(a, a); }
fp2 e2_neg(fp2 a) { return e2_sub(e2_zero(), a); }

struct affine2 { fp2 x; fp2 y; };
struct xyzz2 { fp2 x; fp2 y; fp2 zz; fp2 zzz; };

xyzz2 xyzz2_infinity() { return xyzz2(e2_zero(), e2_zero(), e2_zero(), e2_zero()); }

xyzz2 xyzz2_from_affine(affine2 p) {
  if (e2_is_zero(p.x) && e2_is_zero(p.y))
    return xyzz2_infinity();
  return xyzz2(p.x, p.y, e2_one(), e2_one());
}

xyzz2 xyzz2_double(xyzz2 p) {
  if (e2_is_zero(p.zz) || e2_is_zero(p.y))
    return xyzz2_infinity();
  const fp2 u = e2_add(p.y, p.y);
  const fp2 v = e2_square(u);
  const fp2 w = e2_mul(u, v);
  const fp2 s = e2_mul(p.x, v);
  const fp2 xx = e2_square(p.x);
  const fp2 m = e2_add(e2_add(xx, xx), xx);
  xyzz2 r;
  r.x = e2_sub(e2_sub(e2_square(m), s), s);
  r.y = e2_sub(e2_mul(e2_sub(s, r.x), m), e2_mul(w, p.y));
  r.zz = e2_mul(v, p.zz);
  r.zzz = e2_mul(w, p.zzz);
  return r;
}

xyzz2 xyzz2_add_mixed(xyzz2 p, affine2 q) {
  if (e2_is_zero(q.x) && e2_is_zero(q.y))
    return p;
  if (e2_is_zero(p.zz))
    return xyzz2_from_affine(q);
  const fp2 dx = e2_sub(e2_mul(q.x, p.zz), p.x);
  const fp2 dy = e2_sub(e2_mul(q.y, p.zzz), p.y);
  if (e2_is_zero(dx)) {
#ifdef NO_DOUBLE
    return xyzz2_infinity();
#else
    return e2_is_zero(dy) ? xyzz2_double(p) : xyzz2_infinity();
#endif
  }
  const fp2 pp = e2_square(dx);
  const fp2 ppp = e2_mul(dx, pp);
  const fp2 q2 = e2_mul(p.x, pp);
  xyzz2 r;
  r.x = e2_sub(e2_sub(e2_sub(e2_square(dy), ppp), q2), q2);
  r.y = e2_sub(e2_mul(e2_sub(q2, r.x), dy), e2_mul(p.y, ppp));
  r.zz = e2_mul(p.zz, pp);
  r.zzz = e2_mul(p.zzz, ppp);
  return r;
}

xyzz2 xyzz2_add(xyzz2 p, xyzz2 q) {
  if (e2_is_zero(q.zz))
    return p;
  if (e2_is_zero(p.zz))
    return q;
  const fp2 u1 = e2_mul(p.x, q.zz);
  const fp2 s1 = e2_mul(p.y, q.zzz);
  const fp2 dx = e2_sub(e2_mul(q.x, p.zz), u1);
  const fp2 dy = e2_sub(e2_mul(q.y, p.zzz), s1);
  if (e2_is_zero(dx))
    return e2_is_zero(dy) ? xyzz2_double(p) : xyzz2_infinity();
  const fp2 pp = e2_square(dx);
  const fp2 ppp = e2_mul(dx, pp);
  const fp2 q2 = e2_mul(u1, pp);
  xyzz2 r;
  r.x = e2_sub(e2_sub(e2_sub(e2_square(dy), ppp), q2), q2);
  r.y = e2_sub(e2_mul(e2_sub(q2, r.x), dy), e2_mul(s1, ppp));
  r.zz = e2_mul(e2_mul(p.zz, q.zz), pp);
  r.zzz = e2_mul(e2_mul(p.zzz, q.zzz), ppp);
  return r;
}
struct fr { uint l0, l1, l2, l3, l4, l5, l6, l7; };
const uint RQ0 = 0xf0000001u;
const uint RQ1 = 0x43e1f593u;
const uint RQ2 = 0x79b97091u;
const uint RQ3 = 0x2833e848u;
const uint RQ4 = 0x8181585du;
const uint RQ5 = 0xb85045b6u;
const uint RQ6 = 0xe131a029u;
const uint RQ7 = 0x30644e72u;
const uint FR_INV = 0xefffffffu;
fr fr_zero() { return fr(0u, 0u, 0u, 0u, 0u, 0u, 0u, 0u); }
fr fr_one() { return fr(0x4ffffffbu, 0xac96341cu, 0x9f60cd29u, 0x36fc7695u, 0x7879462eu, 0x666ea36fu, 0x9a07df2fu, 0x0e0a77c1u); }
bool fr_is_zero(fr a) { return (a.l0 | a.l1 | a.l2 | a.l3 | a.l4 | a.l5 | a.l6 | a.l7) == 0u; }
fr fr_reduce_once(uint t0, uint t1, uint t2, uint t3, uint t4, uint t5, uint t6, uint t7) {
  uint b = 0u, b1, b2;
  uint d0 = usubBorrow(t0, RQ0, b1); d0 = usubBorrow(d0, b, b2); b = b1 | b2;
  uint d1 = usubBorrow(t1, RQ1, b1); d1 = usubBorrow(d1, b, b2); b = b1 | b2;
  uint d2 = usubBorrow(t2, RQ2, b1); d2 = usubBorrow(d2, b, b2); b = b1 | b2;
  uint d3 = usubBorrow(t3, RQ3, b1); d3 = usubBorrow(d3, b, b2); b = b1 | b2;
  uint d4 = usubBorrow(t4, RQ4, b1); d4 = usubBorrow(d4, b, b2); b = b1 | b2;
  uint d5 = usubBorrow(t5, RQ5, b1); d5 = usubBorrow(d5, b, b2); b = b1 | b2;
  uint d6 = usubBorrow(t6, RQ6, b1); d6 = usubBorrow(d6, b, b2); b = b1 | b2;
  uint d7 = usubBorrow(t7, RQ7, b1); d7 = usubBorrow(d7, b, b2); b = b1 | b2;
  return fr(b != 0u ? t0 : d0, b != 0u ? t1 : d1, b != 0u ? t2 : d2, b != 0u ? t3 : d3, b != 0u ? t4 : d4, b != 0u ? t5 : d5, b != 0u ? t6 : d6, b != 0u ? t7 : d7);
}
fr fr_add(fr a, fr b) {
  uint c = 0u, c1, c2;
  uint s0 = uaddCarry(a.l0, b.l0, c1); s0 = uaddCarry(s0, c, c2); c = c1 + c2;
  uint s1 = uaddCarry(a.l1, b.l1, c1); s1 = uaddCarry(s1, c, c2); c = c1 + c2;
  uint s2 = uaddCarry(a.l2, b.l2, c1); s2 = uaddCarry(s2, c, c2); c = c1 + c2;
  uint s3 = uaddCarry(a.l3, b.l3, c1); s3 = uaddCarry(s3, c, c2); c = c1 + c2;
  uint s4 = uaddCarry(a.l4, b.l4, c1); s4 = uaddCarry(s4, c, c2); c = c1 + c2;
  uint s5 = uaddCarry(a.l5, b.l5, c1); s5 = uaddCarry(s5, c, c2); c = c1 + c2;
  uint s6 = uaddCarry(a.l6, b.l6, c1); s6 = uaddCarry(s6, c, c2); c = c1 + c2;
  uint s7 = uaddCarry(a.l7, b.l7, c1); s7 = uaddCarry(s7, c, c2); c = c1 + c2;
  return fr_reduce_once(s0, s1, s2, s3, s4, s5, s6, s7);
}
fr fr_sub(fr a, fr b) {
  uint b0 = 0u, b1, b2;
  uint d0 = usubBorrow(a.l0, b.l0, b1); d0 = usubBorrow(d0, b0, b2); b0 = b1 | b2;
  uint d1 = usubBorrow(a.l1, b.l1, b1); d1 = usubBorrow(d1, b0, b2); b0 = b1 | b2;
  uint d2 = usubBorrow(a.l2, b.l2, b1); d2 = usubBorrow(d2, b0, b2); b0 = b1 | b2;
  uint d3 = usubBorrow(a.l3, b.l3, b1); d3 = usubBorrow(d3, b0, b2); b0 = b1 | b2;
  uint d4 = usubBorrow(a.l4, b.l4, b1); d4 = usubBorrow(d4, b0, b2); b0 = b1 | b2;
  uint d5 = usubBorrow(a.l5, b.l5, b1); d5 = usubBorrow(d5, b0, b2); b0 = b1 | b2;
  uint d6 = usubBorrow(a.l6, b.l6, b1); d6 = usubBorrow(d6, b0, b2); b0 = b1 | b2;
  uint d7 = usubBorrow(a.l7, b.l7, b1); d7 = usubBorrow(d7, b0, b2); b0 = b1 | b2;
  const uint mask = 0u - b0;
  uint c = 0u, c1, c2;
  d0 = uaddCarry(d0, RQ0 & mask, c1); d0 = uaddCarry(d0, c, c2); c = c1 + c2;
  d1 = uaddCarry(d1, RQ1 & mask, c1); d1 = uaddCarry(d1, c, c2); c = c1 + c2;
  d2 = uaddCarry(d2, RQ2 & mask, c1); d2 = uaddCarry(d2, c, c2); c = c1 + c2;
  d3 = uaddCarry(d3, RQ3 & mask, c1); d3 = uaddCarry(d3, c, c2); c = c1 + c2;
  d4 = uaddCarry(d4, RQ4 & mask, c1); d4 = uaddCarry(d4, c, c2); c = c1 + c2;
  d5 = uaddCarry(d5, RQ5 & mask, c1); d5 = uaddCarry(d5, c, c2); c = c1 + c2;
  d6 = uaddCarry(d6, RQ6 & mask, c1); d6 = uaddCarry(d6, c, c2); c = c1 + c2;
  d7 = uaddCarry(d7, RQ7 & mask, c1); d7 = uaddCarry(d7, c, c2); c = c1 + c2;
  return fr(d0, d1, d2, d3, d4, d5, d6, d7);
}
fr fr_mul(fr a, fr b) {
  uint t0 = 0u; uint t1 = 0u; uint t2 = 0u; uint t3 = 0u; uint t4 = 0u; uint t5 = 0u; uint t6 = 0u; uint t7 = 0u;
  uint A, C, m, hi, lo, c1, c2;
  A = 0u; umulExtended(a.l0, b.l0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l0, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l0, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l0, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l0, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l0, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l0, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l0, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l1, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l1, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l1, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l1, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l1, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l1, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l1, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l2, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l2, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l2, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l2, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l2, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l2, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l2, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l3, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l3, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l3, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l3, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l3, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l3, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l3, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l4, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l4, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l4, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l4, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l4, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l4, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l4, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l5, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l5, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l5, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l5, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l5, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l5, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l5, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l6, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l6, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l6, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l6, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l6, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l6, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l6, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  A = 0u; umulExtended(a.l0, b.l7, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t0 = lo;
  m = t0 * FR_INV; C = 0u; umulExtended(m, RQ0, hi, lo); lo = uaddCarry(lo, t0, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; lo = lo;
  umulExtended(a.l1, b.l7, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t1 = lo;
  umulExtended(m, RQ1, hi, lo); lo = uaddCarry(lo, t1, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t0 = lo;
  umulExtended(a.l2, b.l7, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t2 = lo;
  umulExtended(m, RQ2, hi, lo); lo = uaddCarry(lo, t2, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t1 = lo;
  umulExtended(a.l3, b.l7, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t3 = lo;
  umulExtended(m, RQ3, hi, lo); lo = uaddCarry(lo, t3, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t2 = lo;
  umulExtended(a.l4, b.l7, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t4 = lo;
  umulExtended(m, RQ4, hi, lo); lo = uaddCarry(lo, t4, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t3 = lo;
  umulExtended(a.l5, b.l7, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t5 = lo;
  umulExtended(m, RQ5, hi, lo); lo = uaddCarry(lo, t5, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t4 = lo;
  umulExtended(a.l6, b.l7, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t6 = lo;
  umulExtended(m, RQ6, hi, lo); lo = uaddCarry(lo, t6, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t5 = lo;
  umulExtended(a.l7, b.l7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, A, c2); A = hi + c1 + c2; t7 = lo;
  umulExtended(m, RQ7, hi, lo); lo = uaddCarry(lo, t7, c1); lo = uaddCarry(lo, C, c2); C = hi + c1 + c2; t6 = lo;
  t7 = C + A;
  return fr_reduce_once(t0, t1, t2, t3, t4, t5, t6, t7);
}
fr fr_from_canon(fr a) { return fr_mul(a, fr(0xae216da7u, 0x1bb8e645u, 0xe35c59e3u, 0x53fe3ab1u, 0x53bb8085u, 0x8c49833du, 0x7f4e44a5u, 0x0216d0b1u)); }
fr fr_to_canon(fr a) { return fr_mul(a, fr(1u, 0u, 0u, 0u, 0u, 0u, 0u, 0u)); }
