// The quotient's transforms over Fr. A DIF stage pairs lo and hi a gap
// apart: lo + hi, (lo - hi) w; a DIT stage: lo + hi w, lo - hi w;
// w = root^(j << level) with gap = n >> (level + 1), j the pair's index
// within its half block. DIF over natural order leaves bit-reversed order,
// which DIT consumes.
//
// The stages run several to a pass through threadgroup memory: a tile of
// rows x cols elements, rows a power of two (2^levels) a stride s apart,
// cols consecutive, closed under the pass's stages, whose gaps run from
// (rows / 2) s to s. Each thread takes one pair per stage; DIF from the
// widest gap down, DIT from s up. Element (m, c) of tile g sits at
// block * rows * s + j0 + c + m * s, the tiles of a block side by side.
struct PassArgs {
  uint n;
  uint first;      // the global level of the widest gap
  uint levels;     // stages in the pass: rows = 2^levels
  uint stride_log; // s = 2^stride_log
  uint cols_log;   // cols = 2^cols_log <= s
  uint pad0;
  uint pad1;
  uint pad2;
};

inline uint pass_index(constant PassArgs &a, uint g, uint m, uint c) {
  const uint per_block = 1u << (a.stride_log - a.cols_log);
  const uint block = g >> (a.stride_log - a.cols_log);
  const uint j0 = (g & (per_block - 1)) << a.cols_log;
  return (block << (a.levels + a.stride_log)) + j0 + c + (m << a.stride_log);
}

template <bool DIT>
inline void fr_pass(device fr_t *values, device const fr_t *roots,
                    constant PassArgs &a, threadgroup fr_t *tile, uint t,
                    uint g, uint threads) {
  const uint cols_mask = (1u << a.cols_log) - 1;
  for (uint e = t; e < 2 * threads; e += threads)
    tile[e] = values[pass_index(a, g, e >> a.cols_log, e & cols_mask)];
  threadgroup_barrier(mem_flags::mem_threadgroup);
  const uint c = t & cols_mask;
  const uint q = t >> a.cols_log;
  for (uint step = 0; step < a.levels; step++) {
    // Local stage k pairs rows a gap 2^(levels - 1 - k) apart.
    const uint k = DIT ? a.levels - 1 - step : step;
    const uint h_log = a.levels - 1 - k;
    const uint jm = q & ((1u << h_log) - 1);
    const uint m0 = ((q >> h_log) << (h_log + 1)) | jm;
    const uint m1 = m0 | (1u << h_log);
    const uint level = a.first + k;
    const uint i0 = pass_index(a, g, m0, c);
    const uint gap = a.n >> (level + 1);
    const fr_t w = roots[(i0 & (gap - 1)) << level];
    const uint l0 = (m0 << a.cols_log) | c;
    const uint l1 = (m1 << a.cols_log) | c;
    const fr_t u = tile[l0];
    if (DIT) {
      const fr_t v = fr_mul(tile[l1], w);
      tile[l0] = fr_add(u, v);
      tile[l1] = fr_sub(u, v);
    } else {
      const fr_t v = tile[l1];
      tile[l0] = fr_add(u, v);
      tile[l1] = fr_mul(fr_sub(u, v), w);
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
  }
  for (uint e = t; e < 2 * threads; e += threads)
    values[pass_index(a, g, e >> a.cols_log, e & cols_mask)] = tile[e];
}

kernel void fr_dif_pass(device fr_t *values [[buffer(0)]],
                        device const fr_t *roots [[buffer(1)]],
                        constant PassArgs &a [[buffer(2)]],
                        threadgroup fr_t *tile [[threadgroup(0)]],
                        uint t [[thread_position_in_threadgroup]],
                        uint g [[threadgroup_position_in_grid]],
                        uint threads [[threads_per_threadgroup]]) {
  fr_pass<false>(values, roots, a, tile, t, g, threads);
}

kernel void fr_dit_pass(device fr_t *values [[buffer(0)]],
                        device const fr_t *roots [[buffer(1)]],
                        constant PassArgs &a [[buffer(2)]],
                        threadgroup fr_t *tile [[threadgroup(0)]],
                        uint t [[thread_position_in_threadgroup]],
                        uint g [[threadgroup_position_in_grid]],
                        uint threads [[threads_per_threadgroup]]) {
  fr_pass<true>(values, roots, a, tile, t, g, threads);
}

kernel void fr_scale(device fr_t *values [[buffer(0)]],
                     device const fr_t *weights [[buffer(1)]],
                     constant uint &n [[buffer(2)]],
                     uint i [[thread_position_in_grid]]) {
  if (i < n)
    values[i] = fr_mul(values[i], weights[i]);
}

// a, b, c each n long, consecutive: a = a b - c.
kernel void fr_pointwise(device fr_t *abc [[buffer(0)]],
                         constant uint &n [[buffer(2)]],
                         uint i [[thread_position_in_grid]]) {
  if (i < n)
    abc[i] = fr_sub(fr_mul(abc[i], abc[n + i]), abc[2 * n + i]);
}
