// buckets.wgsl over G2.
// The bucket accumulation and window reduction of a signed-window Pippenger
// MSM over runs the host sorted (as zopro-metal's, without SIMD shuffles).
// Bucket b of window w is b - w * half, its weight that plus one.
const SIGN: u32 = 0x80000000u;

struct Shards { count: u32, pad0: u32, pad1: u32, pad2: u32, }
struct Reduce { half: u32, leaf: u32, per_window: u32, pad: u32, }

@group(0) @binding(0) var<storage, read> bases: array<Affine2>;
@group(0) @binding(1) var<storage, read> sorted: array<u32>;
@group(0) @binding(2) var<storage, read> start: array<u32>;
@group(0) @binding(3) var<storage, read> count: array<u32>;
@group(0) @binding(4) var<storage, read> dst: array<u32>;
@group(0) @binding(5) var<storage, read_write> out: array<Xyzz2>;
@group(0) @binding(6) var<uniform> shards: Shards;
@group(1) @binding(0) var<storage, read> tree: array<Xyzz2>;

fn signed_base(encoded: u32) -> Affine2 {
    var p = bases[encoded & ~SIGN];
    if ((encoded & SIGN) != 0u) { p.y = fp2_neg(p.y); }
    return p;
}

// Runs split into shards: the first level over base points, later levels
// over the previous level's sums; shard s covers [start[s], start[s] +
// count[s]) and writes out[dst[s]].
@compute @workgroup_size(64)
fn shards_first_g2(@builtin(global_invocation_id) gid: vec3<u32>) {
    let s = gid.x;
    if (s >= shards.count) { return; }
    let begin = start[s];
    let end = begin + count[s];
    var acc = xyzz2_from_affine(signed_base(sorted[begin]));
    var i = begin + 1u;
    if (i < end) { acc = xyzz2_add_affine_pair(acc, signed_base(sorted[i])); i++; }
    for (; i < end; i++) { acc = xyzz2_add_mixed(acc, signed_base(sorted[i])); }
    out[dst[s]] = acc;
}

@compute @workgroup_size(64)
fn shards_tree_g2(@builtin(global_invocation_id) gid: vec3<u32>) {
    let s = gid.x;
    if (s >= shards.count) { return; }
    let begin = start[s];
    let end = begin + count[s];
    var acc = tree[begin];
    for (var i = begin + 1u; i < end; i++) { acc = xyzz2_add(acc, tree[i]); }
    out[dst[s]] = acc;
}

// A window sum is the sum of m * B_m. Each thread takes a block of `leaf`
// buckets after `first`: a running sum gives V, the sum of (m - first) *
// B_m, and S, the sum of B_m; its share is V + first * S, by double-and-add
// over first's bits. A workgroup adds its threads' shares in memory; the
// host adds the workgroups' partial sums. (As zopro-metal's, without SIMD
// shuffles.)
@group(0) @binding(0) var<storage, read> buckets: array<Xyzz2>;
@group(0) @binding(1) var<storage, read_write> partials: array<Xyzz2>;
@group(0) @binding(3) var<uniform> reduce: Reduce;
var<workgroup> shares: array<Xyzz2, 64>;

fn xyzz2_mul_small(p: Xyzz2, k: u32) -> Xyzz2 {
    var r = xyzz2_infinity();
    if (k == 0u) { return r; }
    for (var bit = i32(31u - countLeadingZeros(k)); bit >= 0; bit--) {
        r = xyzz2_double(r);
        if (((k >> u32(bit)) & 1u) == 1u) { r = xyzz2_add(r, p); }
    }
    return r;
}

@compute @workgroup_size(64)
fn reduce_blocks_g2(
    @builtin(global_invocation_id) gid: vec3<u32>,
    @builtin(local_invocation_index) local: u32,
    @builtin(workgroup_id) group: vec3<u32>,
) {
    let window = gid.x / reduce.per_window;
    let first = (gid.x % reduce.per_window) * reduce.leaf;
    let base = window * reduce.half;
    var s = xyzz2_infinity();
    var v = xyzz2_infinity();
    for (var j = reduce.leaf; j > 0u; j--) {
        if (first + j <= reduce.half) { s = xyzz2_add(s, buckets[base + first + j - 1u]); }
        v = xyzz2_add(v, s);
    }
    shares[local] = xyzz2_add(v, xyzz2_mul_small(s, first));
    workgroupBarrier();
    for (var width = 32u; width > 0u; width >>= 1u) {
        if (local < width) { shares[local] = xyzz2_add(shares[local], shares[local + width]); }
        workgroupBarrier();
    }
    if (local == 0u) { partials[group.x] = shares[0]; }
}
