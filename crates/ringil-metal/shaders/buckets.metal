// The bucket accumulation and window reduction of a signed-window Pippenger
// MSM, over runs the host sorted: sorted[starts[b]..starts[b] + counts[b])
// are the base indices of bucket b, the top bit negating the point. Bucket b
// of window w is b - w * half_buckets, its weight that plus one.
constant uint bucket_sign = 0x80000000u;

template <typename E>
inline affine_t<E> signed_base(device const affine_t<E> *bases, uint encoded) {
  affine_t<E> point = bases[encoded & ~bucket_sign];
  if (encoded & bucket_sign)
    point.y = e_neg(point.y);
  return point;
}

// One thread per bucket, the first pair with unit ZZ and ZZZ.
template <typename E>
inline void accumulate(device const affine_t<E> *bases, device const uint *sorted,
                       device const uint *starts, device const uint *counts,
                       device xyzz_t<E> *buckets, uint count, uint b) {
  if (b >= count)
    return;
  const uint begin = starts[b];
  const uint end = begin + counts[b];
  xyzz_t<E> acc = xyzz_infinity<E>();
  if (begin < end) {
    acc = xyzz_from_affine(signed_base(bases, sorted[begin]));
    uint i = begin + 1;
    if (i < end) {
      acc = xyzz_add_affine_pair(acc, signed_base(bases, sorted[i]));
      i++;
    }
    for (; i < end; i++)
      acc = xyzz_add_mixed(acc, signed_base(bases, sorted[i]));
  }
  buckets[b] = acc;
}

inline fp_t e_shuffle_down(fp_t a, ushort delta) {
  fp_t r;
  for (int i = 0; i < 8; i++)
    r.v[i] = simd_shuffle_down(a.v[i], delta);
  return r;
}
inline fp2_t e_shuffle_down(fp2_t a, ushort delta) {
  return fp2_t{e_shuffle_down(a.a0, delta), e_shuffle_down(a.a1, delta)};
}
template <typename E>
inline xyzz_t<E> xyzz_shuffle_down(xyzz_t<E> p, ushort delta) {
  return xyzz_t<E>{e_shuffle_down(p.x, delta), e_shuffle_down(p.y, delta),
                   e_shuffle_down(p.zz, delta), e_shuffle_down(p.zzz, delta)};
}
template <typename E> inline xyzz_t<E> xyzz_shift(xyzz_t<E> p, uint bits) {
  for (uint i = 0; i < bits; i++)
    p = xyzz_double(p);
  return p;
}

// A window sum is the sum of m * B_m. Each lane reduces a block of `leaf`
// buckets with a running sum; adjacent blocks merge as
// V = V_left + V_right + len_left * S_right, len_left a power of two: five
// levels of SIMD shuffles, then a threadgroup tree. (aeglos's msm_reduce_tree.)
template <typename E>
inline void reduce(device const xyzz_t<E> *buckets, device xyzz_t<E> *sums,
                   uint half_buckets, uint leaf, uint leaf_log, uint window, uint index,
                   uint lane, uint simd, uint simds,
                   threadgroup xyzz_t<E> *shared_s, threadgroup xyzz_t<E> *shared_v) {
  device const xyzz_t<E> *items = buckets + (ulong)window * half_buckets;
  xyzz_t<E> s = xyzz_infinity<E>();
  xyzz_t<E> v = xyzz_infinity<E>();
  const uint first = index * leaf;
  for (uint j = leaf; j > 0; j--) {
    if (first + j <= half_buckets)
      s = xyzz_add(s, items[first + j - 1]);
    v = xyzz_add(v, s);
  }
  for (uint level = 0; level < 5; level++) {
    const ushort delta = ushort(1u << level);
    const xyzz_t<E> s_right = xyzz_shuffle_down(s, delta);
    const xyzz_t<E> v_right = xyzz_shuffle_down(v, delta);
    if ((lane & ((2u << level) - 1)) == 0) {
      v = xyzz_add(xyzz_add(v, v_right), xyzz_shift(s_right, leaf_log + level));
      s = xyzz_add(s, s_right);
    }
  }
  if (lane == 0) {
    shared_s[simd] = s;
    shared_v[simd] = v;
  }
  threadgroup_barrier(mem_flags::mem_threadgroup);
  for (uint level = 0; (1u << level) < simds; level++) {
    const uint left = index * (2u << level);
    if (left < simds) {
      const uint right = left + (1u << level);
      const xyzz_t<E> s_right = shared_s[right];
      shared_v[left] = xyzz_add(xyzz_add(shared_v[left], shared_v[right]),
                                xyzz_shift(s_right, leaf_log + 5 + level));
      shared_s[left] = xyzz_add(shared_s[left], s_right);
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
  }
  if (index == 0)
    sums[window] = shared_v[0];
}

struct ReduceArgs {
  uint half_buckets;
  uint leaf;
  uint leaf_log;
  uint pad;
};

#define BUCKET_KERNELS(E, suffix)                                               \
  kernel void accumulate_##suffix(                                              \
      device const affine_t<E> *bases [[buffer(0)]],                            \
      device const uint *sorted [[buffer(1)]],                                  \
      device const uint *starts [[buffer(2)]],                                  \
      device const uint *counts [[buffer(3)]],                                  \
      device xyzz_t<E> *buckets [[buffer(4)]],                                  \
      constant uint &count [[buffer(5)]],                                       \
      uint b [[thread_position_in_grid]]) {                                     \
    accumulate<E>(bases, sorted, starts, counts, buckets, count, b);            \
  }                                                                             \
  kernel void reduce_##suffix(                                                  \
      device const xyzz_t<E> *buckets [[buffer(0)]],                            \
      device xyzz_t<E> *sums [[buffer(1)]],                                     \
      constant ReduceArgs &a [[buffer(2)]],                                     \
      uint window [[threadgroup_position_in_grid]],                             \
      uint index [[thread_position_in_threadgroup]],                            \
      uint lane [[thread_index_in_simdgroup]],                                  \
      uint simd [[simdgroup_index_in_threadgroup]],                             \
      uint simds [[simdgroups_per_threadgroup]]) {                              \
    threadgroup xyzz_t<E> shared_s[8];                                          \
    threadgroup xyzz_t<E> shared_v[8];                                          \
    reduce<E>(buckets, sums, a.half_buckets, a.leaf, a.leaf_log, window, index, lane,   \
              simd, simds, shared_s, shared_v);                                 \
  }

BUCKET_KERNELS(fp_t, g1)
BUCKET_KERNELS(fp2_t, g2)
