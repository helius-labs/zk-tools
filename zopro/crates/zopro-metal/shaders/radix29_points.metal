inline fp29 e_add(fp29 a, fp29 b) {
  fp29 r;
  uint carry = 0;
#pragma clang loop unroll(full)
  for (uint i = 0; i < 9; i++) {
    const uint value = a.v[i] + b.v[i] + carry;
    r.v[i] = value & FP29_MASK;
    carry = value >> 29;
  }
  return fp29_reduce(r);
}

inline fp29 e_sub(fp29 a, fp29 b) {
  fp29 r;
  int borrow = 0;
#pragma clang loop unroll(full)
  for (uint i = 0; i < 9; i++) {
    const int value = int(a.v[i]) - int(b.v[i]) - borrow;
    r.v[i] = uint(value) & FP29_MASK;
    borrow = int(value < 0);
  }
  const uint mask = 0u - uint(borrow);
  uint carry = 0;
#pragma clang loop unroll(full)
  for (uint i = 0; i < 9; i++) {
    const uint value = r.v[i] + (FP29_Q[i] & mask) + carry;
    r.v[i] = value & FP29_MASK;
    carry = value >> 29;
  }
  return r;
}


inline fp29 e_mul(fp29 a, fp29 b) { return fp29_mul(a, b); }
inline fp29 e_square(fp29 a) { return fp29_mul<true>(a, a); }
inline fp29 e_diff_products(fp29 a, fp29 b, fp29 c, fp29 d) {
  return e_sub(e_mul(a, b), e_mul(c, d));
}
inline bool e_is_zero(thread const fp29 &a) {
  uint nonzero = 0;
#pragma clang loop unroll(full)
  for (uint i = 0; i < 9; ++i)
    nonzero |= a.v[i];
  return nonzero == 0;
}
template <> inline fp29 e_zero<fp29>() { return fp29{}; }
template <> inline fp29 e_one<fp29>() { return fp29_pack(fp_one()); }

inline affine_t<fp29> pack_point(affine_t<fp_t> input) {
  return {fp29_pack(input.x), fp29_pack(input.y)};
}
inline xyzz_t<fp_t> unpack_point(xyzz_t<fp29> result) {
  return {fp29_unpack(result.x), fp29_unpack(result.y),
          fp29_unpack(result.zz), fp29_unpack(result.zzz)};
}

// Pick a lower bound on the number of moduli in a - b - 2*c + 3*q.
// The low limbs contribute a carry in (-3, 4), and each subtracted
// modulus can contribute up to one more. The result lies in [0, 2*q).
inline fp29 fp29_sub_two(fp29 a, fp29 b, fp29 c) {
  const int top = int(a.v[8]) - int(b.v[8]) - 2 * int(c.v[8]);
  const int q = int(FP29_Q[8]);
  const int moduli = int(top < 6) + int(top < 6-q) + int(top < 6-2*q);
  fp29 result;
  int carry = 0;
#pragma clang loop unroll(full)
  for (uint i = 0; i < 9; ++i) {
    const int digit = int(a.v[i]) - int(b.v[i]) - 2 * int(c.v[i])
        + moduli * int(FP29_Q[i]) + carry;
    result.v[i] = uint(digit) & FP29_MASK;
    carry = digit >> 29;
  }
  return result.v[8] < FP29_Q[8] ? result : fp29_reduce(result);
}

inline xyzz_t<fp29> radix29_add_mixed(xyzz_t<fp29> p, affine_t<fp29> q) {
  if (affine_is_infinity(q))
    return p;
  if (xyzz_is_infinity(p))
    return xyzz_from_affine(q);
  const fp29 dx = e_sub(e_mul(q.x, p.zz), p.x);
  const fp29 dy = e_sub(e_mul(q.y, p.zzz), p.y);
  if (e_is_zero(dx))
    return e_is_zero(dy) ? xyzz_double(p) : xyzz_infinity<fp29>();
  const fp29 pp = e_square(dx);
  const fp29 ppp = e_mul(dx, pp);
  const fp29 q2 = e_mul(p.x, pp);
  const fp29 x = fp29_sub_two(e_square(dy), ppp, q2);
  const fp29 delta = e_sub(q2, x);
  const fp29 y = e_sub(e_mul(delta, dy), e_mul(p.y, ppp));
  return {x, y, e_mul(p.zz, pp), e_mul(p.zzz, ppp)};
}

inline void radix29_first_shard(device const affine_t<fp_t> *bases, device const uint *sorted,
                                device const uint *start, device const uint *count,
                                device const uint *dst, device xyzz_t<fp_t> *out, uint shard) {
  const uint begin = start[shard];
  const uint end = begin + count[shard];
  xyzz_t<fp29> acc = xyzz_from_affine(pack_point(signed_base<fp_t>(bases, sorted[begin])));
  uint i = begin + 1;
  if (i < end) {
    acc = xyzz_add_affine_pair(acc, pack_point(signed_base<fp_t>(bases, sorted[i])));
    ++i;
  }
  for (; i < end; ++i)
    acc = radix29_add_mixed(acc, pack_point(signed_base<fp_t>(bases, sorted[i])));
  out[dst[shard]] = unpack_point(acc);
}

kernel void shards_first_g1_radix29(
    device const affine_t<fp_t> *bases [[buffer(0)]],
    device const uint *sorted [[buffer(1)]],
    device const uint *start [[buffer(2)]],
    device const uint *count [[buffer(3)]],
    device const uint *dst [[buffer(4)]],
    device xyzz_t<fp_t> *out [[buffer(5)]],
    constant uint &shards [[buffer(6)]],
    uint shard [[thread_position_in_grid]]) {
  if (shard >= shards)
    return;
  radix29_first_shard(bases, sorted, start, count, dst, out, shard);
}

kernel void clear_shard_bands(device uint *counts [[buffer(0)]],
                              uint band [[thread_position_in_grid]]) {
    counts[band] = 0;
}

kernel void count_shard_bands(device const uint *sorted [[buffer(0)]],
                              device const uint *starts [[buffer(1)]],
                              device atomic_uint *counts [[buffer(2)]],
                              device uint *tags [[buffer(3)]],
                              constant uint &shift [[buffer(4)]],
                              uint shard [[thread_position_in_grid]]) {
    // Group shards by the address of their first base. The sign bit is
    // independent of where the base is stored.
    const uint band = (sorted[starts[shard]] & 0x7fffffffu) >> shift;
    const uint slot = atomic_fetch_add_explicit(&counts[band], 1, memory_order_relaxed);
    tags[shard] = (band << 21) | slot;
}

kernel void prefix_shard_bands(device uint *counts [[buffer(0)]],
                               constant uint &bands [[buffer(1)]],
                               uint dummy [[thread_position_in_grid]]) {
    uint first = 0;
    for (uint band = 0; band < bands; ++band) {
        const uint count = counts[band];
        counts[band] = first;
        first += count;
    }
}

kernel void scatter_shard_bands(device const uint *offsets [[buffer(0)]],
                                device const uint *tags [[buffer(1)]],
                                device uint *order [[buffer(2)]],
                                uint shard [[thread_position_in_grid]]) {
    const uint tag = tags[shard];
    order[offsets[tag >> 21] + (tag & 0x1fffffu)] = shard;
}

kernel void shards_first_g1_radix29_ordered(
    device const affine_t<fp_t> *bases [[buffer(0)]],
    device const uint *sorted [[buffer(1)]],
    device const uint *start [[buffer(2)]],
    device const uint *count [[buffer(3)]],
    device const uint *dst [[buffer(4)]],
    device xyzz_t<fp_t> *out [[buffer(5)]],
    constant uint &shards [[buffer(6)]],
    device const uint *order [[buffer(7)]],
    uint thread_id [[thread_position_in_grid]]) {
  if (thread_id >= shards)
    return;
  radix29_first_shard(bases, sorted, start, count, dst, out, order[thread_id]);
}
