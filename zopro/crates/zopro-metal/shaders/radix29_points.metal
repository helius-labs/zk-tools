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
  const uint begin = start[shard];
  const uint end = begin + count[shard];
  xyzz_t<fp29> acc = xyzz_from_affine(pack_point(signed_base<fp_t>(bases, sorted[begin])));
  uint i = begin + 1;
  if (i < end) {
    acc = xyzz_add_affine_pair(acc, pack_point(signed_base<fp_t>(bases, sorted[i])));
    ++i;
  }
  for (; i < end; ++i)
    acc = xyzz_add_mixed(acc, pack_point(signed_base<fp_t>(bases, sorted[i])));
  out[dst[shard]] = unpack_point(acc);
}
