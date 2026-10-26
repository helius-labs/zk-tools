// SPDX-License-Identifier: Apache-2.0
// Counts each bucket's entries (glsl/msm_count.comp): one invocation per
// scalar, its digit in every window. Bucket b of window w is
// w * half + |digit| - 1.
@group(0) @binding(0) var<storage, read> scalars: array<Fr>;
@group(0) @binding(1) var<storage, read_write> counts: array<atomic<u32>>;
@group(0) @binding(2) var<uniform> sort: Sort;

@compute @workgroup_size(64)
fn msm_count(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = sort.offset + id.x;
    if (i >= sort.n) {
        return;
    }
    let s = sort_canon(scalars[i]);
    let radix = 1u << sort.width;
    var carry = 0u;
    for (var w = 0u; w < sort.windows; w++) {
        let value = window_value(s, w * sort.width, sort.width) + carry;
        carry = (value + radix / 2u) >> sort.width;
        let digit = i32(value) - i32(carry * radix);
        if (digit != 0) {
            atomicAdd(&counts[w * sort.half_range + u32(abs(digit)) - 1u], 1u);
        }
    }
}
