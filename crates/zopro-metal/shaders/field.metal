#include <metal_stdlib>
using namespace metal;

// 1. Eight little-endian words match the four-limb host field layout.
#define L64(x) uint((ulong)(x) & 0xfffffffful), uint((ulong)(x) >> 32)

struct fr_t {
  uint v[8];
};

struct fp_t {
  uint v[8];
};

constant uint FR_Q[8] = {L64(0x43e1f593f0000001ul), L64(0x2833e84879b97091ul),
                         L64(0xb85045b68181585dul), L64(0x30644e72e131a029ul)};
constant uint FR_ONE[8] = {L64(0xac96341c4ffffffbul), L64(0x36fc76959f60cd29ul),
                           L64(0x666ea36f7879462eul), L64(0x0e0a77c19a07df2ful)};
constant uint FR_R2[8] = {L64(0x1bb8e645ae216da7ul), L64(0x53fe3ab1e35c59e3ul),
                          L64(0x8c49833d53bb8085ul), L64(0x0216d0b17f4e44a5ul)};
constant uint FR_QM2[8] = {L64(0x43e1f593effffffful), L64(0x2833e84879b97091ul),
                           L64(0xb85045b68181585dul), L64(0x30644e72e131a029ul)};
constant uint FR_INV = 0xefffffffu;

constant uint FP_Q[8] = {L64(0x3c208c16d87cfd47ul), L64(0x97816a916871ca8dul),
                         L64(0xb85045b68181585dul), L64(0x30644e72e131a029ul)};
constant uint FP_ONE[8] = {L64(0xd35d438dc58f0d9dul), L64(0x0a78eb28f5c70b3dul),
                           L64(0x666ea36f7879462cul), L64(0x0e0a77c19a07df2ful)};
constant uint FP_R2[8] = {L64(0xf32cfc5b538afa89ul), L64(0xb5e71911d44501fbul),
                          L64(0x47ab1eff0a417ff6ul), L64(0x06d89f71cab8351ful)};
constant uint FP_QM2[8] = {L64(0x3c208c16d87cfd45ul), L64(0x97816a916871ca8dul),
                           L64(0xb85045b68181585dul), L64(0x30644e72e131a029ul)};
constant uint FP_INV = 0xe4866389u;

template <typename F> inline F field_zero() {
  F r;
  for (int i = 0; i < 8; i++)
    r.v[i] = 0;
  return r;
}

template <typename F> inline F field_load(constant uint *words) {
  F r;
  for (int i = 0; i < 8; i++)
    r.v[i] = words[i];
  return r;
}

template <typename F> inline bool field_is_zero(thread const F &a) {
  uint x = 0;
  for (int i = 0; i < 8; i++)
    x |= a.v[i];
  return x == 0;
}

template <typename F> inline bool field_eq(thread const F &a, thread const F &b) {
  uint x = 0;
  for (int i = 0; i < 8; i++)
    x |= a.v[i] ^ b.v[i];
  return x == 0;
}

template <typename F>
inline F field_reduce_once(thread const uint *t, constant uint *q) {
  F d;
  uint borrow = 0;
  for (int j = 0; j < 8; j++) {
    ulong x = (ulong)t[j] - q[j] - borrow;
    d.v[j] = (uint)x;
    borrow = (uint)(x >> 63);
  }
  F r;
  for (int j = 0; j < 8; j++)
    r.v[j] = borrow ? t[j] : d.v[j];
  return r;
}

template <typename F>
inline F field_add(thread const F &a, thread const F &b, constant uint *q) {
  uint s[8];
  ulong c = 0;
  for (int j = 0; j < 8; j++) {
    c = (ulong)a.v[j] + b.v[j] + c;
    s[j] = (uint)c;
    c >>= 32;
  }
  return field_reduce_once<F>(s, q);
}

template <typename F>
inline F field_sub(thread const F &a, thread const F &b, constant uint *q) {
  F d;
  uint borrow = 0;
  for (int j = 0; j < 8; j++) {
    ulong x = (ulong)a.v[j] - b.v[j] - borrow;
    d.v[j] = (uint)x;
    borrow = (uint)(x >> 63);
  }
  const uint mask = 0u - borrow;
  ulong c = 0;
  for (int j = 0; j < 8; j++) {
    c = (ulong)d.v[j] + (q[j] & mask) + c;
    d.v[j] = (uint)c;
    c >>= 32;
  }
  return d;
}

// 2. Moduli below 2^254 keep the interleaved reduction within eight words.
template <typename F>
inline F field_mul(thread const F &a, thread const F &b, constant uint *q,
                   uint inv) {
  uint t[8];
  for (int j = 0; j < 8; j++)
    t[j] = 0;
  for (int i = 0; i < 8; i++) {
    const uint bi = b.v[i];
    ulong A = (ulong)a.v[0] * bi + t[0];
    t[0] = (uint)A;
    A >>= 32;
    const uint m = t[0] * inv;
    ulong C = ((ulong)m * q[0] + t[0]) >> 32;
    for (int j = 1; j < 8; j++) {
      A = (ulong)a.v[j] * bi + t[j] + A;
      t[j] = (uint)A;
      A >>= 32;
      C = (ulong)m * q[j] + t[j] + C;
      t[j - 1] = (uint)C;
      C >>= 32;
    }
    t[7] = (uint)(C + A);
  }
  return field_reduce_once<F>(t, q);
}

// The same with the outer loop rolled, for G1's point arithmetic: fewer
// live registers let an A12Z's GPU run 512 threads a group rather than 448,
// and its bucket accumulation 12 % faster. G2's kernels and the quotient's
// keep the unrolled loop: rolled, the Metal compiler miscomputed the G2
// MSM on an M5 Pro.
template <typename F>
inline F field_mul_rolled(thread const F &a, thread const F &b, constant uint *q,
                   uint inv) {
  uint t[8];
  for (int j = 0; j < 8; j++)
    t[j] = 0;
#pragma clang loop unroll(disable)
  for (int i = 0; i < 8; i++) {
    const uint bi = b.v[i];
    ulong A = (ulong)a.v[0] * bi + t[0];
    t[0] = (uint)A;
    A >>= 32;
    const uint m = t[0] * inv;
    ulong C = ((ulong)m * q[0] + t[0]) >> 32;
    for (int j = 1; j < 8; j++) {
      A = (ulong)a.v[j] * bi + t[j] + A;
      t[j] = (uint)A;
      A >>= 32;
      C = (ulong)m * q[j] + t[j] + C;
      t[j - 1] = (uint)C;
      C >>= 32;
    }
    t[7] = (uint)(C + A);
  }
  return field_reduce_once<F>(t, q);
}


// A Montgomery square in straight-line SOS: 36 products for the square
// (cross products once, doubled), 64 for the reduction; every index is
// a constant, so the array stays in registers. Generated by
// tools/gen_metal_sqr.py.
template <typename F>
inline F field_square(thread const F &x, constant uint *q, uint inv) {
  const uint a0 = x.v[0], a1 = x.v[1], a2 = x.v[2], a3 = x.v[3], a4 = x.v[4], a5 = x.v[5], a6 = x.v[6], a7 = x.v[7];
  uint t0 = 0, t1 = 0, t2 = 0, t3 = 0, t4 = 0, t5 = 0, t6 = 0, t7 = 0, t8 = 0, t9 = 0, t10 = 0, t11 = 0, t12 = 0, t13 = 0, t14 = 0, t15 = 0, t16 = 0;
  ulong uv;
  uint c;
  c = 0;
  uv = (ulong)a0 * a1 + t1 + c; t1 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a0 * a2 + t2 + c; t2 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a0 * a3 + t3 + c; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a0 * a4 + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a0 * a5 + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a0 * a6 + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a0 * a7 + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  t8 = c;
  c = 0;
  uv = (ulong)a1 * a2 + t3 + c; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a1 * a3 + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a1 * a4 + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a1 * a5 + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a1 * a6 + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a1 * a7 + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  t9 = c;
  c = 0;
  uv = (ulong)a2 * a3 + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a2 * a4 + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a2 * a5 + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a2 * a6 + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a2 * a7 + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  t10 = c;
  c = 0;
  uv = (ulong)a3 * a4 + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a3 * a5 + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a3 * a6 + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a3 * a7 + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  t11 = c;
  c = 0;
  uv = (ulong)a4 * a5 + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a4 * a6 + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a4 * a7 + t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  t12 = c;
  c = 0;
  uv = (ulong)a5 * a6 + t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a5 * a7 + t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  t13 = c;
  c = 0;
  uv = (ulong)a6 * a7 + t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  t14 = c;
  c = 0;
  t15 = c;
  t16 = t15 >> 31;
  t15 = (t15 << 1) | (t14 >> 31);
  t14 = (t14 << 1) | (t13 >> 31);
  t13 = (t13 << 1) | (t12 >> 31);
  t12 = (t12 << 1) | (t11 >> 31);
  t11 = (t11 << 1) | (t10 >> 31);
  t10 = (t10 << 1) | (t9 >> 31);
  t9 = (t9 << 1) | (t8 >> 31);
  t8 = (t8 << 1) | (t7 >> 31);
  t7 = (t7 << 1) | (t6 >> 31);
  t6 = (t6 << 1) | (t5 >> 31);
  t5 = (t5 << 1) | (t4 >> 31);
  t4 = (t4 << 1) | (t3 >> 31);
  t3 = (t3 << 1) | (t2 >> 31);
  t2 = (t2 << 1) | (t1 >> 31);
  t1 = t1 << 1;
  c = 0;
  uv = (ulong)a0 * a0 + t0 + c; t0 = (uint)uv;
  uv = (uv >> 32) + t1; t1 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a1 * a1 + t2 + c; t2 = (uint)uv;
  uv = (uv >> 32) + t3; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a2 * a2 + t4 + c; t4 = (uint)uv;
  uv = (uv >> 32) + t5; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a3 * a3 + t6 + c; t6 = (uint)uv;
  uv = (uv >> 32) + t7; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a4 * a4 + t8 + c; t8 = (uint)uv;
  uv = (uv >> 32) + t9; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a5 * a5 + t10 + c; t10 = (uint)uv;
  uv = (uv >> 32) + t11; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a6 * a6 + t12 + c; t12 = (uint)uv;
  uv = (uv >> 32) + t13; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)a7 * a7 + t14 + c; t14 = (uint)uv;
  uv = (uv >> 32) + t15; t15 = (uint)uv; c = (uint)(uv >> 32);
  t16 += c;
  uint m;
  m = t0 * inv; c = 0;
  uv = (ulong)m * q[0] + t0 + c; t0 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t1 + c; t1 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t2 + c; t2 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t3 + c; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t1 * inv; c = 0;
  uv = (ulong)m * q[0] + t1 + c; t1 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t2 + c; t2 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t3 + c; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t2 * inv; c = 0;
  uv = (ulong)m * q[0] + t2 + c; t2 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t3 + c; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t3 * inv; c = 0;
  uv = (ulong)m * q[0] + t3 + c; t3 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t4 * inv; c = 0;
  uv = (ulong)m * q[0] + t4 + c; t4 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t5 * inv; c = 0;
  uv = (ulong)m * q[0] + t5 + c; t5 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t6 * inv; c = 0;
  uv = (ulong)m * q[0] + t6 + c; t6 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  m = t7 * inv; c = 0;
  uv = (ulong)m * q[0] + t7 + c; t7 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[1] + t8 + c; t8 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[2] + t9 + c; t9 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[3] + t10 + c; t10 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[4] + t11 + c; t11 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[5] + t12 + c; t12 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[6] + t13 + c; t13 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)m * q[7] + t14 + c; t14 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t15 + c; t15 = (uint)uv; c = (uint)(uv >> 32);
  uv = (ulong)t16 + c; t16 = (uint)uv; c = (uint)(uv >> 32);
  const uint r[8] = {t8, t9, t10, t11, t12, t13, t14, t15};
  return field_reduce_once<F>(r, q);
}

template <typename F>
inline F field_pow_words(F base, constant uint *exponent, F one, constant uint *q,
                         uint inv) {
  F result = one;
  for (int word = 7; word >= 0; word--) {
    const uint bits = exponent[word];
    for (int bit = 31; bit >= 0; bit--) {
      result = field_mul(result, result, q, inv);
      if ((bits >> bit) & 1u)
        result = field_mul(result, base, q, inv);
    }
  }
  return result;
}

inline fr_t fr_zero() { return field_zero<fr_t>(); }
inline fr_t fr_one() { return field_load<fr_t>(FR_ONE); }
inline bool fr_is_zero(thread const fr_t &a) { return field_is_zero(a); }
inline bool fr_eq(thread const fr_t &a, thread const fr_t &b) {
  return field_eq(a, b);
}
inline fr_t fr_add(fr_t a, fr_t b) { return field_add(a, b, FR_Q); }
inline fr_t fr_sub(fr_t a, fr_t b) { return field_sub(a, b, FR_Q); }
inline fr_t fr_neg(fr_t a) { return field_sub(fr_zero(), a, FR_Q); }
inline fr_t fr_mul(fr_t a, fr_t b) { return field_mul(a, b, FR_Q, FR_INV); }
inline fr_t fr_from_canon(fr_t a) {
  return field_mul(a, field_load<fr_t>(FR_R2), FR_Q, FR_INV);
}
inline fr_t fr_to_canon(fr_t a) {
  fr_t one = fr_zero();
  one.v[0] = 1;
  return field_mul(a, one, FR_Q, FR_INV);
}
// 3. Zero has no inverse and maps to zero.
inline fr_t fr_inv(fr_t a) {
  return field_pow_words(a, FR_QM2, fr_one(), FR_Q, FR_INV);
}
inline fr_t fr_inv_canon(fr_t a) {
  return fr_to_canon(fr_inv(fr_from_canon(a)));
}
inline fr_t fr_pow(fr_t base, ulong exponent) {
  fr_t result = fr_one();
  while (exponent) {
    if (exponent & 1ul)
      result = fr_mul(result, base);
    exponent >>= 1;
    if (exponent)
      base = fr_mul(base, base);
  }
  return result;
}

inline fp_t fp_zero() { return field_zero<fp_t>(); }
inline fp_t fp_one() { return field_load<fp_t>(FP_ONE); }
inline bool fp_is_zero(thread const fp_t &a) { return field_is_zero(a); }
inline fp_t fp_add(fp_t a, fp_t b) { return field_add(a, b, FP_Q); }
inline fp_t fp_sub(fp_t a, fp_t b) { return field_sub(a, b, FP_Q); }
inline fp_t fp_neg(fp_t a) { return field_sub(fp_zero(), a, FP_Q); }
inline fp_t fp_mul(fp_t a, fp_t b) { return field_mul(a, b, FP_Q, FP_INV); }
// G1's point arithmetic (e_mul, e_square on fp_t).
inline fp_t fp_mul_g1(fp_t a, fp_t b) { return field_mul_rolled(a, b, FP_Q, FP_INV); }
inline fp_t fp_square_g1(fp_t a) { return field_square(a, FP_Q, FP_INV); }
inline fp_t fp_double(fp_t a) { return fp_add(a, a); }
inline fp_t fp_triple(fp_t a) { return fp_add(fp_add(a, a), a); }
inline fp_t fp_inv(fp_t a) {
  return field_pow_words(a, FP_QM2, fp_one(), FP_Q, FP_INV);
}

// 4. The BN254 extension requires the square of i to equal minus one.
struct fp2_t {
  fp_t a0;
  fp_t a1;
};

inline fp2_t fp2_zero() { return fp2_t{fp_zero(), fp_zero()}; }
inline fp2_t fp2_one() { return fp2_t{fp_one(), fp_zero()}; }
inline bool fp2_is_zero(thread const fp2_t &a) {
  return fp_is_zero(a.a0) && fp_is_zero(a.a1);
}
inline fp2_t fp2_add(fp2_t a, fp2_t b) {
  return fp2_t{fp_add(a.a0, b.a0), fp_add(a.a1, b.a1)};
}
inline fp2_t fp2_sub(fp2_t a, fp2_t b) {
  return fp2_t{fp_sub(a.a0, b.a0), fp_sub(a.a1, b.a1)};
}
inline fp2_t fp2_neg(fp2_t a) { return fp2_t{fp_neg(a.a0), fp_neg(a.a1)}; }
inline fp2_t fp2_double(fp2_t a) { return fp2_add(a, a); }
inline fp2_t fp2_triple(fp2_t a) { return fp2_add(fp2_add(a, a), a); }
inline fp2_t fp2_mul(fp2_t a, fp2_t b) {
  const fp_t p0 = fp_mul(a.a0, b.a0);
  const fp_t p1 = fp_mul(a.a1, b.a1);
  const fp_t p2 = fp_mul(fp_add(a.a0, a.a1), fp_add(b.a0, b.a1));
  return fp2_t{fp_sub(p0, p1), fp_sub(fp_sub(p2, p0), p1)};
}
// 5. The inverse divides the conjugate by the norm a0^2 + a1^2.
inline fp2_t fp2_inv(fp2_t a) {
  const fp_t t = fp_inv(fp_add(fp_mul(a.a0, a.a0), fp_mul(a.a1, a.a1)));
  return fp2_t{fp_mul(a.a0, t), fp_neg(fp_mul(a.a1, t))};
}
inline fp2_t fp2_square(fp2_t a) {
  const fp_t real = fp_mul(fp_add(a.a0, a.a1), fp_sub(a.a0, a.a1));
  const fp_t product = fp_mul(a.a0, a.a1);
  return fp2_t{real, fp_add(product, product)};
}
