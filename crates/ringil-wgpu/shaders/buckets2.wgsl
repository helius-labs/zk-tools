// buckets.wgsl over G2.
// The bucket accumulation and window reduction of a signed-window Pippenger
// MSM over runs the host sorted (as ringil-metal's, without SIMD shuffles).
// Bucket b of window w is b - w * half, its weight that plus one.
const SIGN: u32 = 0x80000000u;

struct Shards { count: u32, pad0: u32, pad1: u32, pad2: u32, }
struct Reduce { half: u32, block: u32, block_log: u32, blocks: u32, }

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

// A window sum is the sum of m * B_m. Stage one: each thread takes a block
// of `block` buckets and writes its plain sum S and its weighted sum V
// (weights 1..block within the block) to pairs[2 * index], pairs[2 *
// index + 1]. Stage two: one thread per window combines its blocks: W =
// sum of V_b + block * T, with T = sum over b of b * S_b, a running sum.
@group(0) @binding(0) var<storage, read> buckets: array<Xyzz2>;
@group(0) @binding(1) var<storage, read_write> pairs: array<Xyzz2>;
@group(0) @binding(2) var<storage, read_write> sums: array<Xyzz2>;
@group(0) @binding(3) var<uniform> reduce: Reduce;

@compute @workgroup_size(64)
fn reduce_blocks_g2(@builtin(global_invocation_id) gid: vec3<u32>) {
    let index = gid.x;
    let window = index / reduce.blocks;
    let first = window * reduce.half + (index % reduce.blocks) * reduce.block;
    if (first >= (window + 1u) * reduce.half) { return; }
    var s = xyzz2_infinity();
    var v = xyzz2_infinity();
    for (var j = reduce.block; j > 0u; j--) {
        s = xyzz2_add(s, buckets[first + j - 1u]);
        v = xyzz2_add(v, s);
    }
    pairs[2u * index] = s;
    pairs[2u * index + 1u] = v;
}

@compute @workgroup_size(1)
fn reduce_windows_g2(@builtin(global_invocation_id) gid: vec3<u32>) {
    let window = gid.x;
    var total_v = xyzz2_infinity();
    var t = xyzz2_infinity();
    var acc = xyzz2_infinity();
    for (var b = reduce.blocks; b > 0u; b--) {
        let at = 2u * (window * reduce.blocks + b - 1u);
        total_v = xyzz2_add(total_v, pairs[at + 1u]);
        if (b > 1u) {
            acc = xyzz2_add(acc, pairs[at]);
            t = xyzz2_add(t, acc);
        }
    }
    sums[window] = xyzz2_add(total_v, xyzz2_shift(t, reduce.block_log));
}
