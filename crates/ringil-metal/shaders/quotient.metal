// The quotient's transforms over Fr, radix-2 stages in global memory. A DIF
// stage pairs lo and hi a gap apart: lo + hi, (lo - hi) w; a DIT stage:
// lo + hi w, lo - hi w; w = root^(j << level) with gap = n >> (level + 1),
// j the pair's index within its half block. DIF over natural order leaves
// bit-reversed order, which DIT consumes.
struct StageArgs {
  uint n;
  uint level;
  uint gap_log;
  uint pad;
};

inline void pair_indices(constant StageArgs &a, uint p, thread uint &i0,
                         thread uint &i1, thread uint &twiddle) {
  const uint gap = 1u << a.gap_log;
  const uint j = p & (gap - 1);
  const uint block = p >> a.gap_log;
  i0 = (block << (a.gap_log + 1)) | j;
  i1 = i0 | gap;
  twiddle = j << a.level;
}

kernel void fr_dif_stage(device fr_t *values [[buffer(0)]],
                         device const fr_t *roots [[buffer(1)]],
                         constant StageArgs &a [[buffer(2)]],
                         uint p [[thread_position_in_grid]]) {
  if (p >= a.n / 2)
    return;
  uint i0, i1, t;
  pair_indices(a, p, i0, i1, t);
  const fr_t u = values[i0];
  const fr_t v = values[i1];
  values[i0] = fr_add(u, v);
  values[i1] = fr_mul(fr_sub(u, v), roots[t]);
}

kernel void fr_dit_stage(device fr_t *values [[buffer(0)]],
                         device const fr_t *roots [[buffer(1)]],
                         constant StageArgs &a [[buffer(2)]],
                         uint p [[thread_position_in_grid]]) {
  if (p >= a.n / 2)
    return;
  uint i0, i1, t;
  pair_indices(a, p, i0, i1, t);
  const fr_t u = values[i0];
  const fr_t v = fr_mul(values[i1], roots[t]);
  values[i0] = fr_add(u, v);
  values[i1] = fr_sub(u, v);
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
