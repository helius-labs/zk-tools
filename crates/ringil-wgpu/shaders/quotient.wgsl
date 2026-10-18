// The quotient's transforms over Fr, radix-2 stages in global memory, as
// ringil-metal's quotient.metal.
struct Stage { n: u32, level: u32, gap_log: u32, pad: u32, }

@group(0) @binding(0) var<storage, read_write> values: array<Fr>;
@group(0) @binding(1) var<storage, read> roots: array<Fr>;
@group(0) @binding(2) var<uniform> stage: Stage;
@group(1) @binding(0) var<storage, read_write> abc: array<Fr>;

@compute @workgroup_size(64)
fn fr_dif_stage(@builtin(global_invocation_id) gid: vec3<u32>) {
    let p = gid.x;
    if (p >= stage.n / 2u) { return; }
    let gap = 1u << stage.gap_log;
    let j = p & (gap - 1u);
    let i0 = ((p >> stage.gap_log) << (stage.gap_log + 1u)) | j;
    let i1 = i0 | gap;
    let u = values[i0];
    let v = values[i1];
    values[i0] = fr_add(u, v);
    values[i1] = fr_mul(fr_sub(u, v), roots[j << stage.level]);
}

@compute @workgroup_size(64)
fn fr_dit_stage(@builtin(global_invocation_id) gid: vec3<u32>) {
    let p = gid.x;
    if (p >= stage.n / 2u) { return; }
    let gap = 1u << stage.gap_log;
    let j = p & (gap - 1u);
    let i0 = ((p >> stage.gap_log) << (stage.gap_log + 1u)) | j;
    let i1 = i0 | gap;
    let u = values[i0];
    let v = fr_mul(values[i1], roots[j << stage.level]);
    values[i0] = fr_add(u, v);
    values[i1] = fr_sub(u, v);
}

// values[i] *= roots[i] (the weights bound as roots).
@compute @workgroup_size(64)
fn fr_scale(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < stage.n) { values[i] = fr_mul(values[i], roots[i]); }
}

// a, b, c each n long, consecutive in abc: a = a b - c.
@compute @workgroup_size(64)
fn fr_pointwise(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i < stage.n) {
        abc[i] = fr_sub(fr_mul(abc[i], abc[stage.n + i]), abc[2u * stage.n + i]);
    }
}
