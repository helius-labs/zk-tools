// SPDX-License-Identifier: Apache-2.0
// The GPU sort's digits (glsl/digits.glsl in WGSL), as the host's recoding:
// window w of `width` bits plus the previous window's carry, minus 2^width
// when that reaches half the radix (carrying one on); the extra top window
// absorbs the last carry. Scalars arrive in Montgomery form.
struct Sort { n: u32, width: u32, windows: u32, half_range: u32, }

fn sort_canon(a: Fr) -> Fr {
    return fr_mul(a, Fr(array<u32, 8>(1u, 0u, 0u, 0u, 0u, 0u, 0u, 0u)));
}

fn window_value(s: Fr, offset: u32, width: u32) -> u32 {
    let word = offset >> 5u;
    let shift = offset & 31u;
    var value = 0u;
    if (word < 8u) {
        value = s.limbs[word] >> shift;
    }
    if (shift + width > 32u && word + 1u < 8u) {
        value |= s.limbs[word + 1u] << (32u - shift);
    }
    return value & ((1u << width) - 1u);
}
