// The signed digits of a scalar, as the host's recoding: window w of
// `width` bits plus the previous window's carry, minus 2^width when that
// reaches half the radix (carrying one on); the extra top window absorbs
// the last carry. Scalars arrive in Montgomery form.
layout(std140, set = 0, binding = SORT_UNIFORM) uniform Sort { uint n; uint width; uint windows; uint half_range; uint offset; uint pad0; uint pad1; uint pad2; } sort;

uint window_value(fr s, uint offset, uint width) {
  uint limbs[8] = uint[8](s.l0, s.l1, s.l2, s.l3, s.l4, s.l5, s.l6, s.l7);
  const uint word = offset >> 5u, shift = offset & 31u;
  uint value = word < 8u ? limbs[word] >> shift : 0u;
  if (shift + width > 32u && word + 1u < 8u)
    value |= limbs[word + 1u] << (32u - shift);
  return value & ((1u << width) - 1u);
}
