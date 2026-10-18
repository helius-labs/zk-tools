// Experiment: BN254 Fq Montgomery multiplication over twenty 13-bit limbs
// (R = 2^260), with lazy carries: each limb accumulates up to forty 26-bit
// products per row before any carry, so the rows have no serial carry chain.
struct F13 { l: array<u32, 20>, }
const MASK13: u32 = 0x1fffu;
// -q^-1 mod 2^13
const NP13: u32 = NP13_VALUE;
const Q13: array<u32, 20> = array<u32, 20>(Q13_VALUES);

fn f13_mul(a: F13, b: F13) -> F13 {
    let a0 = a.l[0];
    let a1 = a.l[1];
    let a2 = a.l[2];
    let a3 = a.l[3];
    let a4 = a.l[4];
    let a5 = a.l[5];
    let a6 = a.l[6];
    let a7 = a.l[7];
    let a8 = a.l[8];
    let a9 = a.l[9];
    let a10 = a.l[10];
    let a11 = a.l[11];
    let a12 = a.l[12];
    let a13 = a.l[13];
    let a14 = a.l[14];
    let a15 = a.l[15];
    let a16 = a.l[16];
    let a17 = a.l[17];
    let a18 = a.l[18];
    let a19 = a.l[19];
    var t0 = 0u;
    var t1 = 0u;
    var t2 = 0u;
    var t3 = 0u;
    var t4 = 0u;
    var t5 = 0u;
    var t6 = 0u;
    var t7 = 0u;
    var t8 = 0u;
    var t9 = 0u;
    var t10 = 0u;
    var t11 = 0u;
    var t12 = 0u;
    var t13 = 0u;
    var t14 = 0u;
    var t15 = 0u;
    var t16 = 0u;
    var t17 = 0u;
    var t18 = 0u;
    var t19 = 0u;
    for (var i = 0u; i < 20u; i++) {
        let bi = b.l[i];
        t0 += a0 * bi;
        t1 += a1 * bi;
        t2 += a2 * bi;
        t3 += a3 * bi;
        t4 += a4 * bi;
        t5 += a5 * bi;
        t6 += a6 * bi;
        t7 += a7 * bi;
        t8 += a8 * bi;
        t9 += a9 * bi;
        t10 += a10 * bi;
        t11 += a11 * bi;
        t12 += a12 * bi;
        t13 += a13 * bi;
        t14 += a14 * bi;
        t15 += a15 * bi;
        t16 += a16 * bi;
        t17 += a17 * bi;
        t18 += a18 * bi;
        t19 += a19 * bi;
        let m = (t0 * NP13) & MASK13;
        t0 += m * Q13[0];
        t1 += m * Q13[1];
        t2 += m * Q13[2];
        t3 += m * Q13[3];
        t4 += m * Q13[4];
        t5 += m * Q13[5];
        t6 += m * Q13[6];
        t7 += m * Q13[7];
        t8 += m * Q13[8];
        t9 += m * Q13[9];
        t10 += m * Q13[10];
        t11 += m * Q13[11];
        t12 += m * Q13[12];
        t13 += m * Q13[13];
        t14 += m * Q13[14];
        t15 += m * Q13[15];
        t16 += m * Q13[16];
        t17 += m * Q13[17];
        t18 += m * Q13[18];
        t19 += m * Q13[19];
        let carry = t0 >> 13u;
        t0 = t1;
        t1 = t2;
        t2 = t3;
        t3 = t4;
        t4 = t5;
        t5 = t6;
        t6 = t7;
        t7 = t8;
        t8 = t9;
        t9 = t10;
        t10 = t11;
        t11 = t12;
        t12 = t13;
        t13 = t14;
        t14 = t15;
        t15 = t16;
        t16 = t17;
        t17 = t18;
        t18 = t19;
        t19 = 0u;
        t0 += carry;
    }
    var r: F13;
    var c = 0u;
    { let v = t0 + c; r.l[0] = v & MASK13; c = v >> 13u; }
    { let v = t1 + c; r.l[1] = v & MASK13; c = v >> 13u; }
    { let v = t2 + c; r.l[2] = v & MASK13; c = v >> 13u; }
    { let v = t3 + c; r.l[3] = v & MASK13; c = v >> 13u; }
    { let v = t4 + c; r.l[4] = v & MASK13; c = v >> 13u; }
    { let v = t5 + c; r.l[5] = v & MASK13; c = v >> 13u; }
    { let v = t6 + c; r.l[6] = v & MASK13; c = v >> 13u; }
    { let v = t7 + c; r.l[7] = v & MASK13; c = v >> 13u; }
    { let v = t8 + c; r.l[8] = v & MASK13; c = v >> 13u; }
    { let v = t9 + c; r.l[9] = v & MASK13; c = v >> 13u; }
    { let v = t10 + c; r.l[10] = v & MASK13; c = v >> 13u; }
    { let v = t11 + c; r.l[11] = v & MASK13; c = v >> 13u; }
    { let v = t12 + c; r.l[12] = v & MASK13; c = v >> 13u; }
    { let v = t13 + c; r.l[13] = v & MASK13; c = v >> 13u; }
    { let v = t14 + c; r.l[14] = v & MASK13; c = v >> 13u; }
    { let v = t15 + c; r.l[15] = v & MASK13; c = v >> 13u; }
    { let v = t16 + c; r.l[16] = v & MASK13; c = v >> 13u; }
    { let v = t17 + c; r.l[17] = v & MASK13; c = v >> 13u; }
    { let v = t18 + c; r.l[18] = v & MASK13; c = v >> 13u; }
    { let v = t19 + c; r.l[19] = v & MASK13; c = v >> 13u; }
    var ge = true;
    for (var j = 20u; j > 0u;) {
        j--;
        if (r.l[j] != Q13[j]) { ge = r.l[j] > Q13[j]; break; }
    }
    if (ge) {
        var borrow = 0u;
        for (var j = 0u; j < 20u; j++) {
            let v = r.l[j] - Q13[j] - borrow;
            borrow = select(0u, 1u, r.l[j] < Q13[j] + borrow);
            r.l[j] = v & MASK13;
        }
    }
    return r;
}

@group(0) @binding(0) var<storage, read_write> data13: array<F13>;
@group(0) @binding(1) var<uniform> rounds13: vec4<u32>;

@compute @workgroup_size(64)
fn chain13(@builtin(global_invocation_id) gid: vec3<u32>) {
    let i = gid.x;
    if (i >= rounds13.y) { return; }
    var x = data13[2u * i];
    let y = data13[2u * i + 1u];
    for (var k = 0u; k < rounds13.x; k++) { x = f13_mul(x, y); }
    data13[2u * i] = x;
}
