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

template <bool DIT, bool DERIVE_C = false, bool PREFIX = false, bool LOOSE = false>
inline void fr_transform_tile(device const fr_t *values, device const fr_t *roots,
                    constant PassArgs &a, threadgroup fr_t *tile, uint t,
                    uint g, uint threads, device const fr_t *prefix = nullptr,
                    uint active = 0) {
  const uint cols_mask = (1u << a.cols_log) - 1;
  for (uint e = t; e < 2 * threads; e += threads) {
    const uint i = pass_index(a, g, e >> a.cols_log, e & cols_mask);
    tile[e] = PREFIX ? (i < active ? prefix[i] : fr_zero())
                     : (DERIVE_C ? fr_mul(values[i], values[a.n + i]) : values[i]);
  }
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
    const bool unit = LOOSE && (i0 & (gap - 1)) == 0;
    const uint l0 = (m0 << a.cols_log) | c;
    const uint l1 = (m1 << a.cols_log) | c;
    const fr_t u = tile[l0];
    fr_t v = tile[l1];
    if (DIT && !unit) {
      const fr_t w = roots[(i0 & (gap - 1)) << level];
      v = LOOSE ? fr_mul_loose(v, w) : fr_mul(v, w);
    }
    tile[l0] = LOOSE ? fr_loose_add(u, v) : fr_add(u, v);
    fr_t d = LOOSE ? fr_loose_sub(u, v) : fr_sub(u, v);
    if (!DIT && !unit) {
      const fr_t w = roots[(i0 & (gap - 1)) << level];
      d = LOOSE ? fr_mul_loose(d, w) : fr_mul(d, w);
    }
    tile[l1] = d;
    threadgroup_barrier(mem_flags::mem_threadgroup);
  }
}

template <bool DIT, bool DERIVE_C = false, bool PREFIX = false, bool LOOSE = false>
inline void fr_pass(device fr_t *values, device const fr_t *roots,
                    constant PassArgs &a, threadgroup fr_t *tile, uint t,
                    uint g, uint threads, device const fr_t *prefix = nullptr,
                    uint active = 0) {
  fr_transform_tile<DIT, DERIVE_C, PREFIX, LOOSE>(values, roots, a, tile, t, g, threads, prefix, active);
  const uint cols_mask = (1u << a.cols_log) - 1;
  device fr_t *output = values + (DERIVE_C ? 2 * a.n : 0);
  for (uint e = t; e < 2 * threads; e += threads) {
    const fr_t value = tile[e];
    output[pass_index(a, g, e >> a.cols_log, e & cols_mask)] =
        LOOSE ? field_reduce_once<fr_t>(value.v, FR_Q) : value;
  }
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

// Consume the solver's compact rows and produce the first inverse pass
// directly in the otherwise uninitialized transform buffer.
kernel void fr_dif_from_active(device fr_t *values [[buffer(0)]],
                               device const fr_t *roots [[buffer(1)]],
                               constant PassArgs &a [[buffer(2)]],
                               device const fr_t *prefix [[buffer(3)]],
                               constant uint &active [[buffer(4)]],
                               threadgroup fr_t *tile [[threadgroup(0)]],
                               uint t [[thread_position_in_threadgroup]],
                               uint g [[threadgroup_position_in_grid]],
                               uint threads [[threads_per_threadgroup]]) {
  fr_pass<false, false, true>(values, roots, a, tile, t, g, threads, prefix, active);
}

kernel void fr_dif_from_active_loose(device fr_t *values [[buffer(0)]],
                                     device const fr_t *roots [[buffer(1)]],
                                     constant PassArgs &args [[buffer(2)]],
                                     device const fr_t *prefix [[buffer(3)]],
                                     constant uint &active [[buffer(4)]],
                                     threadgroup fr_t *tile [[threadgroup(0)]],
                                     uint t [[thread_position_in_threadgroup]],
                                     uint g [[threadgroup_position_in_grid]],
                                     uint threads [[threads_per_threadgroup]]) {
  fr_pass<false, false, true, true>(values, roots, args, tile, t, g, threads, prefix, active);
}

#define EULER_LOOSE_PASS(NAME, DIT) \
  kernel void NAME(device fr_t *values [[buffer(0)]], \
                   device const fr_t *roots [[buffer(1)]], \
                   constant PassArgs &args [[buffer(2)]], \
                   threadgroup fr_t *tile [[threadgroup(0)]], \
                   uint t [[thread_position_in_threadgroup]], \
                   uint g [[threadgroup_position_in_grid]], \
                   uint threads [[threads_per_threadgroup]]) { \
    fr_pass<DIT, false, false, true>(values, roots, args, tile, t, g, threads); \
  }
EULER_LOOSE_PASS(fr_dif_euler_loose, false)
EULER_LOOSE_PASS(fr_dit_euler_loose, true)
#undef EULER_LOOSE_PASS

// When the host solver has checked every row, C(i) = A(i) * B(i).
// Construct C while loading its first inverse-transform tile.
kernel void fr_dif_derive_c(device fr_t *values [[buffer(0)]],
                           device const fr_t *roots [[buffer(1)]],
                           constant PassArgs &a [[buffer(2)]],
                           threadgroup fr_t *tile [[threadgroup(0)]],
                           uint t [[thread_position_in_threadgroup]],
                           uint g [[threadgroup_position_in_grid]],
                           uint threads [[threads_per_threadgroup]]) {
  fr_pass<false, true>(values, roots, a, tile, t, g, threads);
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

// values[i] *= its weight, chunks[i >> low_bits] * lows[i & (2^low_bits - 1)].
kernel void fr_scale(device fr_t *values [[buffer(0)]],
                     device const fr_t *lows [[buffer(1)]],
                     constant uint &n [[buffer(2)]],
                     device const fr_t *chunks [[buffer(3)]],
                     constant uint &low_bits [[buffer(4)]],
                     uint i [[thread_position_in_grid]]) {
  if (i < n)
    values[i] = fr_mul(fr_mul(values[i], lows[i & ((1u << low_bits) - 1)]), chunks[i >> low_bits]);
}

// A and B are in coset evaluation form, C is in coefficient form.
kernel void fr_product(device fr_t *abc [[buffer(0)]],
                       constant uint &n [[buffer(2)]],
                       uint i [[thread_position_in_grid]]) {
  if (i < n)
    abc[i] = fr_mul(abc[i], abc[n + i]);
}

// Weight bit-reversed inverse-FFT coefficients by their natural degree.
// The two tables add to that degree in the Montgomery field.
kernel void fr_euler_weight(device fr_t *values [[buffer(0)]],
                            device const fr_t *lows [[buffer(1)]],
                            constant uint &n [[buffer(2)]],
                            device const fr_t *chunks [[buffer(3)]],
                            constant uint &low_bits [[buffer(4)]],
                            uint i [[thread_position_in_grid]]) {
  if (i < n)
    values[i] = fr_mul(values[i], fr_add(lows[i & ((1u << low_bits) - 1)], chunks[i >> low_bits]));
}

kernel void fr_euler_product(device fr_t *values [[buffer(0)]],
                             device const fr_t *rows [[buffer(1)]],
                             constant uint2 &size [[buffer(2)]],
                             uint i [[thread_position_in_grid]]) {
  const uint domain = size.x, active = size.y;
  if (i < active) {
    const fr_t a = rows[i], b = rows[active + i];
    values[i] = fr_add(fr_mul(values[i], b), fr_mul(a, values[domain + i]));
  }
}

// The inverse transforms are unnormalized. The reverse weights combine
// n^-1, the quotient denominator, and the inverse coset powers. C was
// transformed on the original domain and takes only the constant weight.
kernel void fr_finish_quotient(device fr_t *abc [[buffer(0)]],
                              device const fr_t *lows [[buffer(1)]],
                              constant uint &n [[buffer(2)]],
                              device const fr_t *chunks [[buffer(3)]],
                              constant uint &low_bits [[buffer(4)]],
                              uint i [[thread_position_in_grid]]) {
  if (i < n) {
    const fr_t product = fr_mul(fr_mul(abc[i], lows[i & ((1u << low_bits) - 1)]),
                                chunks[i >> low_bits]);
    abc[i] = fr_sub(product, fr_mul(abc[2 * n + i], chunks[0]));
  }
}

// Store signed digits window-major within each block of consecutive scalars.
inline void write_quotient_digits(fr_t words, device short *digits,
                                  constant uint4 &args, uint i) {
  const uint n = args.x, width = args.y, windows = args.z, chunk = args.w;
  const uint base = (i / chunk) * chunk;
  const uint len = min(chunk, n - base);
  const uint radix = 1u << width;
  uint carry = 0;
  for (uint w = 0; w < windows; w++) {
    const uint offset = w * width;
    const uint word = offset / 32, shift = offset % 32;
    uint digit = words.v[word] >> shift;
    if (shift + width > 32 && word < 7)
      digit |= words.v[word + 1] << (32 - shift);
    digit = (digit & (radix - 1)) + carry;
    carry = (digit + radix / 2) >> width;
    digits[base * windows + w * len + i - base] = short(int(digit) - int(carry * radix));
  }
}

kernel void fr_quotient_digits(device const fr_t *coefficients [[buffer(0)]],
                              device short *digits [[buffer(1)]],
                              constant uint4 &args [[buffer(2)]],
                              uint i [[thread_position_in_grid]]) {
  if (i < args.x)
    write_quotient_digits(fr_to_canon(coefficients[i]), digits, args, i);
}

// Derivatives are canonical and the original rows are Montgomery.
// Each Montgomery multiplication yields the canonical digit input.
kernel void fr_euler_product_digits(device fr_t *values [[buffer(0)]],
                                    device short *digits [[buffer(1)]],
                                    constant uint4 &args [[buffer(2)]],
                                    device const fr_t *rows [[buffer(3)]],
                                    constant uint &domain [[buffer(4)]],
                                    uint i [[thread_position_in_grid]]) {
  if (i < args.x) {
    const fr_t a = rows[i], b = rows[args.x + i];
    const fr_t result = fr_add(fr_mul(values[i], b), fr_mul(a, values[domain + i]));
    values[i] = result;
    write_quotient_digits(result, digits, args, i);
  }
}

// Booth digits overlap the preceding window's sign bit and telescope to
// the original canonical scalar. Fixed geometry gives each extraction a
// constant word index and uses the top zero bit at bit 254.
inline void write_quotient_digits_15(fr_t result, device short *digits,
                                     uint n, uint chunk, uint i) {
  const uint base = i & ~(chunk - 1u);
  const uint length = min(chunk, n - base);
  device short *output = digits + base * 17 + i - base;
  uint previous_sign = 0;
#pragma clang loop unroll(full)
  for (uint w = 0; w < 17; ++w) {
    const uint offset = w * 15u;
    const uint word = offset >> 5, shift = offset & 31u;
    uint digit = result.v[word] >> shift;
    if (shift + 15u > 32u)
      digit |= result.v[word + 1] << (32u - shift);
    digit &= 32767u;
    const uint sign = digit >> 14;
    output[w * length] = short(int(digit) + int(previous_sign) - int(sign << 15));
    previous_sign = sign;
  }
}

kernel void fr_dit_euler_final(device fr_t *values [[buffer(0)]],
                               device const fr_t *roots [[buffer(1)]],
                               constant PassArgs &a [[buffer(2)]],
                               device const fr_t *rows [[buffer(3)]],
                               device short *digits [[buffer(4)]],
                               constant uint4 &output [[buffer(5)]],
                               threadgroup fr_t *tile [[threadgroup(0)]],
                               uint t [[thread_position_in_threadgroup]],
                               uint g [[threadgroup_position_in_grid]],
                               uint threads [[threads_per_threadgroup]]) {
  const uint n = output.x, chunk = output.y;
  const bool combine = output.z != 0;
  fr_transform_tile<true, false, false, true>(values + (combine ? a.n : 0),
                                                roots, a, tile, t, g, threads);
  const uint cols_mask = (1u << a.cols_log) - 1;
  for (uint e = t; e < 2 * threads; e += threads) {
    const uint i = pass_index(a, g, e >> a.cols_log, e & cols_mask);
    if (i < n) {
      const fr_t derivative = tile[e];
      if (combine) {
        // A and B's derivatives are below 2q. Multiplication by a
        // canonical row normalizes the result and keeps its ordinary encoding.
        const fr_t first = fr_mul(values[i], rows[n + i]);
        const fr_t second = fr_mul(derivative, rows[i]);
        const fr_t result = fr_add(first, second);
        values[i] = result;
        write_quotient_digits_15(result, digits, n, chunk, i);
      } else {
        values[i] = derivative;
      }
    }
  }
}

// Match the recoder's block layout while reading a contiguous run for
// each window. Retain the digits for the scatter, or for the host fallback.
kernel void fr_digit_counts(device const short *digits [[buffer(0)]],
                            device atomic_uint *counts [[buffer(1)]],
                            constant uint4 &args [[buffer(2)]],
                            uint2 tid [[thread_position_in_grid]]) {
  const uint i = tid.x, w = tid.y;
  const uint n = args.x, buckets = args.y, windows = args.z, chunk = args.w;
  if (i >= n || w >= windows)
    return;
  const uint base = (i / chunk) * chunk;
  const uint len = min(chunk, n - base);
  const uint b = uint(abs(int(digits[base * windows + w * len + i - base])));
  if (b != 0) {
    const uint index = b <= buckets ? w * buckets + b - 1 : windows * buckets;
    atomic_fetch_add_explicit(&counts[index], 1u, memory_order_relaxed);
  }
}

kernel void fr_digit_scatter(device const short *digits [[buffer(0)]],
                             device const uint *starts [[buffer(1)]],
                             device const uint *counts [[buffer(2)]],
                             device atomic_uint *offsets [[buffer(3)]],
                             device uint *sorted [[buffer(4)]],
                             constant uint4 &args [[buffer(5)]],
                             uint2 tid [[thread_position_in_grid]]) {
  const uint n = args.x, buckets = args.y, windows = args.z, chunk = args.w;
  const uint i = tid.x, w = tid.y;
  if (i >= n || w >= windows)
    return;
  const uint base = (i / chunk) * chunk;
  const uint len = min(chunk, n - base);
  const int d = int(digits[base * windows + w * len + i - base]);
  const uint b = uint(abs(d));
  if (b != 0 && b <= buckets) {
    const uint index = w * buckets + b - 1;
    const uint offset = atomic_fetch_add_explicit(&offsets[index], 1u, memory_order_relaxed);
    if (offset < counts[index])
      sorted[starts[index] + offset] = i | (d < 0 ? 0x80000000u : 0u);
  }
}
