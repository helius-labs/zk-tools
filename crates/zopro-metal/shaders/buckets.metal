// The bucket accumulation and window reduction of a signed-window Pippenger
// MSM, over runs the host sorted: the base indices of a bucket, the top bit
// negating the point. Bucket b of window w is b - w * half_buckets, its
// weight that plus one.
constant uint bucket_sign = 0x80000000u;

template <typename E>
inline affine_t<E> signed_base(device const affine_t<E> *bases, uint encoded) {
  affine_t<E> point = bases[encoded & ~bucket_sign];
  if (encoded & bucket_sign)
    point.y = e_neg(point.y);
  return point;
}

// Runs split into shards of bounded length, which threads sum in parallel
// level by level until every bucket holds one point: the first level reads
// the sorted base indices, later levels the previous level's sums. Shard s
// covers source[start[s]..start[s] + count[s]) and writes out[dst[s]].
template <typename E>
inline void shards_first(device const affine_t<E> *bases, device const uint *sorted,
                         device const uint *start, device const uint *count,
                         device const uint *dst, device xyzz_t<E> *out, uint shards,
                         uint s) {
  if (s >= shards)
    return;
  const uint begin = start[s];
  const uint end = begin + count[s];
  xyzz_t<E> acc = xyzz_from_affine(signed_base(bases, sorted[begin]));
  uint i = begin + 1;
  if (i < end) {
    acc = xyzz_add_affine_pair(acc, signed_base(bases, sorted[i]));
    i++;
  }
  for (; i < end; i++)
    acc = xyzz_add_mixed(acc, signed_base(bases, sorted[i]));
  out[dst[s]] = acc;
}

template <typename E>
inline void shards_tree(device const xyzz_t<E> *tree, device const uint *start,
                        device const uint *count, device const uint *dst,
                        device xyzz_t<E> *out, uint shards, uint s) {
  if (s >= shards)
    return;
  const uint begin = start[s];
  const uint end = begin + count[s];
  xyzz_t<E> acc = tree[begin];
  for (uint i = begin + 1; i < end; i++)
    acc = xyzz_add(acc, tree[i]);
  out[dst[s]] = acc;
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

// A window sum is the sum of m * B_m over its buckets. Each thread takes a
// block of `leaf` buckets after `first`: a running sum gives V, the sum of
// (m - first) * B_m, and S, the sum of B_m; its share is V + first * S, by
// double-and-add over first's bits. The shares then only add: across the
// SIMD group by shuffles, across the threadgroup through memory; the host
// adds the threadgroups' partial sums. The longest chain is leaf + 2 * 14
// point operations plus a few additions, where merging blocks with shifts
// had one scaling per tree level.
template <typename E> inline xyzz_t<E> xyzz_mul_small(xyzz_t<E> p, uint k) {
  xyzz_t<E> r = xyzz_infinity<E>();
  if (k == 0)
    return r;
  for (int bit = 31 - int(clz(k)); bit >= 0; bit--) {
    r = xyzz_double(r);
    if ((k >> uint(bit)) & 1u)
      r = xyzz_add(r, p);
  }
  return r;
}

// A thread's share of its window: V + first * S over its block of buckets.
template <typename E>
inline xyzz_t<E> reduce_share(device const xyzz_t<E> *buckets, uint half_buckets, uint leaf,
                              uint per_window, uint gid) {
  const uint window = gid / per_window;
  const uint first = (gid % per_window) * leaf;
  device const xyzz_t<E> *items = buckets + (ulong)window * half_buckets;
  xyzz_t<E> s = xyzz_infinity<E>();
  xyzz_t<E> v = xyzz_infinity<E>();
  for (uint j = leaf; j > 0; j--) {
    if (first + j <= half_buckets)
      s = xyzz_add(s, items[first + j - 1]);
    v = xyzz_add(v, s);
  }
  return xyzz_add(v, xyzz_mul_small(s, first));
}

template <typename E>
inline void reduce(device const xyzz_t<E> *buckets, device xyzz_t<E> *partials,
                   uint half_buckets, uint leaf, uint per_window, uint gid, uint group,
                   uint index, uint lane, uint simd, uint simds,
                   threadgroup xyzz_t<E> *shared_t) {
  xyzz_t<E> share = reduce_share(buckets, half_buckets, leaf, per_window, gid);
  for (ushort delta = 16; delta > 0; delta >>= 1)
    share = xyzz_add(share, xyzz_shuffle_down(share, delta));
  if (lane == 0)
    shared_t[simd] = share;
  threadgroup_barrier(mem_flags::mem_threadgroup);
  if (index == 0) {
    xyzz_t<E> total = shared_t[0];
    for (uint i = 1; i < simds; i++)
      total = xyzz_add(total, shared_t[i]);
    partials[group] = total;
  }
}

// The same without SIMD-group functions, which an A12's GPU lacks: the
// shares add as a tree through threadgroup memory, at most REDUCE_TG
// threads a group (a power of two).
#define REDUCE_TG 64
template <typename E>
inline void reduce_tg(device const xyzz_t<E> *buckets, device xyzz_t<E> *partials,
                      uint half_buckets, uint leaf, uint per_window, uint gid, uint group,
                      uint index, uint threads, threadgroup xyzz_t<E> *shared_t) {
  shared_t[index] = reduce_share(buckets, half_buckets, leaf, per_window, gid);
  threadgroup_barrier(mem_flags::mem_threadgroup);
  for (uint stride = threads / 2; stride > 0; stride >>= 1) {
    if (index < stride)
      shared_t[index] = xyzz_add(shared_t[index], shared_t[index + stride]);
    threadgroup_barrier(mem_flags::mem_threadgroup);
  }
  if (index == 0)
    partials[group] = shared_t[0];
}

struct ReduceArgs {
  uint half_buckets;
  uint leaf;
  uint per_window;
  uint pad;
};

#define BUCKET_KERNELS(E, suffix)                                               \
  kernel void shards_first_##suffix(                                            \
      device const affine_t<E> *bases [[buffer(0)]],                            \
      device const uint *sorted [[buffer(1)]],                                  \
      device const uint *start [[buffer(2)]],                                   \
      device const uint *count [[buffer(3)]],                                   \
      device const uint *dst [[buffer(4)]],                                     \
      device xyzz_t<E> *out [[buffer(5)]],                                      \
      constant uint &shards [[buffer(6)]],                                      \
      uint s [[thread_position_in_grid]]) {                                     \
    shards_first<E>(bases, sorted, start, count, dst, out, shards, s);          \
  }                                                                             \
  kernel void shards_tree_##suffix(                                             \
      device const xyzz_t<E> *tree [[buffer(0)]],                               \
      device const uint *start [[buffer(2)]],                                   \
      device const uint *count [[buffer(3)]],                                   \
      device const uint *dst [[buffer(4)]],                                     \
      device xyzz_t<E> *out [[buffer(5)]],                                      \
      constant uint &shards [[buffer(6)]],                                      \
      uint s [[thread_position_in_grid]]) {                                     \
    shards_tree<E>(tree, start, count, dst, out, shards, s);                    \
  }                                                                             \
  kernel void reduce_##suffix(                                                  \
      device const xyzz_t<E> *buckets [[buffer(0)]],                            \
      device xyzz_t<E> *partials [[buffer(1)]],                                 \
      constant ReduceArgs &a [[buffer(2)]],                                     \
      uint gid [[thread_position_in_grid]],                                     \
      uint group [[threadgroup_position_in_grid]],                              \
      uint index [[thread_position_in_threadgroup]],                            \
      uint lane [[thread_index_in_simdgroup]],                                  \
      uint simd [[simdgroup_index_in_threadgroup]],                             \
      uint simds [[simdgroups_per_threadgroup]]) {                              \
    threadgroup xyzz_t<E> shared_t[32];                                         \
    reduce<E>(buckets, partials, a.half_buckets, a.leaf, a.per_window, gid,     \
              group, index, lane, simd, simds, shared_t);                       \
  }                                                                             \
  kernel void reduce_tg_##suffix(                                               \
      device const xyzz_t<E> *buckets [[buffer(0)]],                            \
      device xyzz_t<E> *partials [[buffer(1)]],                                 \
      constant ReduceArgs &a [[buffer(2)]],                                     \
      uint gid [[thread_position_in_grid]],                                     \
      uint group [[threadgroup_position_in_grid]],                              \
      uint index [[thread_position_in_threadgroup]],                            \
      uint threads [[threads_per_threadgroup]]) {                               \
    threadgroup xyzz_t<E> shared_t[REDUCE_TG];                                  \
    reduce_tg<E>(buckets, partials, a.half_buckets, a.leaf, a.per_window, gid,  \
                 group, index, threads, shared_t);                              \
  }

BUCKET_KERNELS(fp_t, g1)
