// SPDX-License-Identifier: Apache-2.0
// Writes each entry, point index with the digit's sign in the top bit, at
// its bucket's cursor (glsl/msm_scatter.comp). Order within a bucket is the
// atomics': a bucket's sum does not depend on it.
@group(0) @binding(0) var<storage, read> scalars: array<Fr>;
@group(0) @binding(1) var<storage, read_write> cursors: array<atomic<u32>>;
@group(0) @binding(2) var<storage, read_write> sorted: array<u32>;
@group(0) @binding(3) var<uniform> sort: Sort;

@compute @workgroup_size(64)
fn msm_scatter(@builtin(global_invocation_id) id: vec3<u32>) {
    let i = id.x;
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
            let at = atomicAdd(&cursors[w * sort.half_range + u32(abs(digit)) - 1u], 1u);
            sorted[at] = i | select(0u, 0x80000000u, digit < 0);
        }
    }
}
