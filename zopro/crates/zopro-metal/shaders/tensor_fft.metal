#include <MetalPerformancePrimitives/MetalPerformancePrimitives.h>
using namespace mpp::tensor_ops;

// Each row of weights contains one reduced multiple of the fixed operand.
// An exact byte dot product reconstructs the unreduced field result.
inline fr_t normalize_linear(threadgroup const int *coefficients) {
  uint words[8];
  uint carry = 0;
#pragma clang loop unroll(full)
  for (uint word = 0; word < 8; ++word) {
    uint limb = 0;
#pragma clang loop unroll(full)
    for (uint byte = 0; byte < 4; ++byte) {
      const uint v = uint(coefficients[4 * word + byte]) + carry;
      limb |= (v & 255u) << (8 * byte);
      carry = v >> 8;
    }
    words[word] = limb;
  }
  const ulong high = (ulong(carry) << 32) | words[7];
  // floor(2^49 / (floor(Fr modulus / 2^224) + 1)). The dot
  // product is less than 8160 moduli, so this underestimates by at most one.
  const uint q = uint((high * 693390UL) >> 49);
  ulong mul_carry = 0;
  uint borrow = 0;
#pragma clang loop unroll(full)
  for (uint word = 0; word < 8; ++word) {
    const ulong p = ulong(q) * FR_Q[word] + mul_carry;
    const uint v = words[word];
    const uint difference = v - uint(p);
    words[word] = difference - borrow;
    borrow = uint(v < uint(p) || difference < borrow);
    mul_carry = p >> 32;
  }
  return field_reduce_once<fr_t>(words, FR_Q);
}

constant uint ROWS = 32;
constant uint LEAF = 16;
constant uint TILE = ROWS * LEAF;
constant uint GROUPS = 4;

inline uint transpose_fft_index(uint pos) {
  return (pos % LEAF) * ROWS + pos / LEAF;
}

template <bool DIT>
inline void upper_fft_levels(threadgroup fr_t *tile, device const fr_t *roots, uint t) {
  for (uint stage = 0; stage < 5; ++stage) {
    const uint gap = DIT ? LEAF << stage : (TILE / 2) >> stage;
    const uint j = t & (gap - 1);
    const uint pos = (t / gap) * 2 * gap + j;
    const uint lo = transpose_fft_index(pos);
    const uint hi = transpose_fft_index(pos + gap);
    const fr_t u = tile[lo];
    const fr_t v = tile[hi];
    if (DIT) {
      const fr_t product = j == 0 ? v : fr_mul(v, roots[j * (TILE / (2 * gap))]);
      tile[lo] = fr_add(u, product);
      tile[hi] = fr_sub(u, product);
    } else {
      const fr_t delta = fr_sub(u, v);
      tile[lo] = fr_add(u, v);
      tile[hi] = j == 0 ? delta : fr_mul(delta, roots[j * (TILE / (2 * gap))]);
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
  }
}

template <bool DIT>
inline void tensor_fft_levels(threadgroup fr_t *tile, threadgroup int *coefficients,
                              device uchar *weights, uint lane, uint simd) {
  using A = tensor<threadgroup uchar, extents<int, 32, ROWS>, tensor_inline>;
  using B = tensor<device uchar, extents<int, 32, 32>, tensor_inline>;
  using C = tensor<threadgroup int, extents<int, 32, ROWS>, tensor_inline>;
  constexpr auto descriptor = matmul2d_descriptor(ROWS, 32, 32);
  matmul2d<descriptor, execution_simdgroups<1>> operation;
  for (uint stage = 0; stage < 4; ++stage) {
    const uint gap = DIT ? 1u << stage : (LEAF / 2) >> stage;
    if (simd < GROUPS) {
      for (uint pair = simd; pair < LEAF / 2; pair += GROUPS) {
        const uint j = pair & (gap - 1);
        const uint lo = ((pair / gap) * 2 * gap + j) * ROWS;
        const uint hi = lo + gap * ROWS;
        const fr_t x = tile[lo + lane];
        const fr_t y = tile[hi + lane];
        if (!DIT) {
          tile[lo + lane] = fr_add(x, y);
          tile[hi + lane] = fr_sub(x, y);
        }
        fr_t product = y;
        if (j != 0) {
          simdgroup_barrier(mem_flags::mem_threadgroup);
          threadgroup int *columns = coefficients + simd * ROWS * 32;
          A a(reinterpret_cast<threadgroup uchar *>(tile + hi), extents<int, 32, ROWS>());
          B b(weights + j * ((LEAF / 2) / gap) * 1024, extents<int, 32, 32>());
          C c(columns, extents<int, 32, ROWS>());
          auto result = operation.template get_destination_cooperative_tensor<A, B, int>();
          operation.run(a, b, result);
          result.store(c);
          simdgroup_barrier(mem_flags::mem_threadgroup);
          product = normalize_linear(columns + lane * 32);
          simdgroup_barrier(mem_flags::mem_threadgroup);
        }
        if (DIT) {
          tile[lo + lane] = fr_add(x, product);
          tile[hi + lane] = fr_sub(x, product);
        } else if (j != 0) {
          tile[hi + lane] = product;
        }
      }
    }
    threadgroup_barrier(mem_flags::mem_threadgroup);
  }
}

#define HYBRID_FFT(NAME, DIT)                                                        \
kernel void NAME(device fr_t *values [[buffer(0)]],                                  \
                 device const fr_t *roots [[buffer(1)]],                            \
                 device uchar *weights [[buffer(2)]],                              \
                 uint t [[thread_index_in_threadgroup]],                           \
                 uint lane [[thread_index_in_simdgroup]],                           \
                 uint simd [[simdgroup_index_in_threadgroup]],                      \
                 uint group [[threadgroup_position_in_grid]]) {                    \
  threadgroup fr_t tile[TILE];                                                       \
  threadgroup int coefficients[GROUPS * ROWS * 32];                                  \
  const uint start = group * TILE;                                                   \
  tile[transpose_fft_index(t)] = values[start + t];                                  \
  tile[transpose_fft_index(t + TILE / 2)] = values[start + t + TILE / 2];             \
  threadgroup_barrier(mem_flags::mem_threadgroup);                                   \
  if (!DIT) upper_fft_levels<false>(tile, roots, t);                                  \
  tensor_fft_levels<DIT>(tile, coefficients, weights, lane, simd);                    \
  if (DIT) upper_fft_levels<true>(tile, roots, t);                                    \
  values[start + t] = tile[transpose_fft_index(t)];                                  \
  values[start + t + TILE / 2] = tile[transpose_fft_index(t + TILE / 2)];              \
}

HYBRID_FFT(fr_dif_tensor9, false)
HYBRID_FFT(fr_dit_tensor9, true)

inline fr_t fr_weight_tensor_euler(fr_t value, uint index, constant uint4 &options,
                                   device const fr_t *lows, device const fr_t *highs) {
  if (options.z != 0) {
    const uint low = index & ((1u << options.y) - 1);
    return fr_mul(value, fr_add(lows[low], highs[index >> options.y]));
  }
  return fr_mul_euler_degree(value, __builtin_bitreverse32(index) >> options.x);
}

// The outer inverse passes already put both butterflies in the same
// 512-point tile. Keep the Euler-weighted coefficients in threadgroup
// memory between the final inverse pass and the first forward pass.
kernel void fr_euler_tensor9(device fr_t *values [[buffer(0)]],
                             device const fr_t *inverse_roots [[buffer(1)]],
                             device uchar *inverse_weights [[buffer(2)]],
                             device const fr_t *forward_roots [[buffer(3)]],
                             device uchar *forward_weights [[buffer(4)]],
                             constant uint4 &options [[buffer(5)]],
                             device const fr_t *lows [[buffer(6)]],
                             device const fr_t *highs [[buffer(7)]],
                             uint t [[thread_index_in_threadgroup]],
                             uint lane [[thread_index_in_simdgroup]],
                             uint simd [[simdgroup_index_in_threadgroup]],
                             uint group [[threadgroup_position_in_grid]]) {
  threadgroup fr_t tile[TILE];
  threadgroup int coefficients[GROUPS * ROWS * 32];
  const uint start = group * TILE;
  const uint hi = t + TILE / 2;
  const uint lo_index = transpose_fft_index(t), hi_index = transpose_fft_index(hi);
  tile[lo_index] = values[start + t];
  tile[hi_index] = values[start + hi];
  threadgroup_barrier(mem_flags::mem_threadgroup);
  upper_fft_levels<false>(tile, inverse_roots, t);
  tensor_fft_levels<false>(tile, coefficients, inverse_weights, lane, simd);
  tile[lo_index] = fr_weight_tensor_euler(tile[lo_index], start + t, options, lows, highs);
  tile[hi_index] = fr_weight_tensor_euler(tile[hi_index], start + hi, options, lows, highs);
  threadgroup_barrier(mem_flags::mem_threadgroup);
  tensor_fft_levels<true>(tile, coefficients, forward_weights, lane, simd);
  upper_fft_levels<true>(tile, forward_roots, t);
  values[start + t] = tile[lo_index];
  values[start + hi] = tile[hi_index];
}
