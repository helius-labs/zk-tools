// The current multiplication (field.wgsl's fp_mul) in the same chain.
@group(0) @binding(0) var<storage, read_write> data16: array<Fp>;
@group(0) @binding(1) var<uniform> rounds16: vec4<u32>;

@compute @workgroup_size(64)
fn chain16(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= rounds16.y) { return; }
    var x = data16[2u * i];
    let y = data16[2u * i + 1u];
    for (var k = 0u; k < rounds16.x; k++) { x = fp_mul(x, y); }
    data16[2u * i] = x;
}
