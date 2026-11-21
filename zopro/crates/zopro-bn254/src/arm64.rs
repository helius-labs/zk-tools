// Montgomery multiplication on AArch64 with R = 2^256: CIOS over four
// 64-bit limbs, the reduction interleaved into each row, in assembly so
// that the products go through MUL and UMULH with one carry chain each.
// For inputs below 2p and p < 2^254 (both BN254 moduli), the Montgomery
// result is below p + (2p)^2 / R < 2p. One conditional subtraction reduces
// it. The unrolled row shift renames the accumulator registers.
// Where an assembly block reads an operand through a pointer, the pointer
// lives for the call and the block touches nothing else.
// A reduction row cancels its low limb. Because the low product is its
// additive inverse modulo 2^64, the carry is exactly one for a nonzero
// limb. Comparison supplies it without multiplying the low halves.

#[inline(always)]
pub fn multiply(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    multiply_inner::<true>(a, b, p, inv)
}

/// Inputs are at most 2p and the output is below 2p. Each CIOS row is
/// bounded by 3p after its carry shift, and p + (2p)^2 / R < 2p.
#[cfg(target_os = "macos")]
#[inline(always)]
pub fn multiply_loose(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    multiply_inner::<false>(a, b, p, inv)
}

/// Signed incoming y minus the bucket's y, with a bias of p or 2p.
/// For canonical operands, the result is in (0, 2p] and goes directly
/// into a loose Montgomery multiplication.
#[cfg(target_os = "macos")]
#[inline(always)]
pub fn affine_signed_difference(a: [u64; 4], b: [u64; 4], negative: bool, p: [u64; 4]) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: all operands are registers.
    unsafe {
        core::arch::asm!(
            "subs {r0}, {p0}, {b0}",
            "sbcs {r1}, {p1}, {b1}",
            "sbcs {r2}, {p2}, {b2}",
            "sbc {r3}, {p3}, {b3}",
            "cmp {negative:w}, #0",
            "csel {r0}, {r0}, {b0}, ne",
            "csel {r1}, {r1}, {b1}, ne",
            "csel {r2}, {r2}, {b2}, ne",
            "csel {r3}, {r3}, {b3}, ne",
            "adds {r0}, {r0}, {p0}",
            "adcs {r1}, {r1}, {p1}",
            "adcs {r2}, {r2}, {p2}",
            "adc {r3}, {r3}, {p3}",
            "subs {r0}, {r0}, {a0}",
            "sbcs {r1}, {r1}, {a1}",
            "sbcs {r2}, {r2}, {a2}",
            "sbc {r3}, {r3}, {a3}",
            negative = in(reg) u32::from(negative),
            p0 = in(reg) p[0], p1 = in(reg) p[1], p2 = in(reg) p[2], p3 = in(reg) p[3],
            a0 = in(reg) a[0], a1 = in(reg) a[1], a2 = in(reg) a[2], a3 = in(reg) a[3],
            b0 = in(reg) b[0], b1 = in(reg) b[1], b2 = in(reg) b[2], b3 = in(reg) b[3],
            r0 = out(reg) r0, r1 = out(reg) r1, r2 = out(reg) r2, r3 = out(reg) r3,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

#[inline(always)]
fn multiply_inner<const NORMALIZE: bool>(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: all operands are registers.
    unsafe {
        core::arch::asm!(
            "mul {t0}, {a0}, {b0}",
            "mul {t1}, {a1}, {b0}",
            "mul {t2}, {a2}, {b0}",
            "mul {t3}, {a3}, {b0}",
            "umulh {h0}, {a0}, {b0}",
            "umulh {h1}, {a1}, {b0}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {a2}, {b0}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {a3}, {b0}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {h1}, xzr",
            "mul {m}, {t0}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t0}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t1}, {t1}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t2}, {t2}, {l0}",
            "adcs {t3}, {t3}, {l1}",
            "adc {t4}, {t4}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {t4}, {h1}",
            "mul {l0}, {a0}, {b1}",
            "mul {l1}, {a1}, {b1}",
            "adds {t1}, {t1}, {l0}",
            "mul {l0}, {a2}, {b1}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {a3}, {b1}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adc {t0}, xzr, xzr",
            "umulh {h0}, {a0}, {b1}",
            "umulh {h1}, {a1}, {b1}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {a2}, {b1}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {a3}, {b1}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t0}, {t0}, {h1}",
            "mul {m}, {t1}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t1}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adc {t0}, {t0}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t0}, {t0}, {h1}",
            "mul {l0}, {a0}, {b2}",
            "mul {l1}, {a1}, {b2}",
            "adds {t2}, {t2}, {l0}",
            "mul {l0}, {a2}, {b2}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {a3}, {b2}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t0}, {t0}, {l1}",
            "adc {t1}, xzr, xzr",
            "umulh {h0}, {a0}, {b2}",
            "umulh {h1}, {a1}, {b2}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {a2}, {b2}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {a3}, {b2}",
            "adcs {t0}, {t0}, {h0}",
            "adc {t1}, {t1}, {h1}",
            "mul {m}, {t2}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t2}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t0}, {t0}, {l1}",
            "adc {t1}, {t1}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t0}, {t0}, {h0}",
            "adc {t1}, {t1}, {h1}",
            "mul {l0}, {a0}, {b3}",
            "mul {l1}, {a1}, {b3}",
            "adds {t3}, {t3}, {l0}",
            "mul {l0}, {a2}, {b3}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {a3}, {b3}",
            "adcs {t0}, {t0}, {l0}",
            "adcs {t1}, {t1}, {l1}",
            "adc {t2}, xzr, xzr",
            "umulh {h0}, {a0}, {b3}",
            "umulh {h1}, {a1}, {b3}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {a2}, {b3}",
            "adcs {t0}, {t0}, {h1}",
            "umulh {h1}, {a3}, {b3}",
            "adcs {t1}, {t1}, {h0}",
            "adc {t2}, {t2}, {h1}",
            "mul {m}, {t3}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t3}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t0}, {t0}, {l0}",
            "adcs {t1}, {t1}, {l1}",
            "adc {t2}, {t2}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t0}, {t0}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t1}, {t1}, {h0}",
            "adc {t2}, {t2}, {h1}",
            ".if {normalize}",
            "subs {l0}, {t4}, {p0}",
            "sbcs {l1}, {t0}, {p1}",
            "sbcs {h0}, {t1}, {p2}",
            "sbcs {h1}, {t2}, {p3}",
            "csel {r0}, {t4}, {l0}, lo",
            "csel {r1}, {t0}, {l1}, lo",
            "csel {r2}, {t1}, {h0}, lo",
            "csel {r3}, {t2}, {h1}, lo",
            ".else",
            "mov {r0}, {t4}",
            "mov {r1}, {t0}",
            "mov {r2}, {t1}",
            "mov {r3}, {t2}",
            ".endif",
            normalize = const NORMALIZE as u32,
            a0 = in(reg) a[0],
            a1 = in(reg) a[1],
            a2 = in(reg) a[2],
            a3 = in(reg) a[3],
            b0 = in(reg) b[0],
            b1 = in(reg) b[1],
            b2 = in(reg) b[2],
            b3 = in(reg) b[3],
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            inv = in(reg) inv,
            m = out(reg) _,
            l0 = out(reg) _,
            l1 = out(reg) _,
            h0 = out(reg) _,
            h1 = out(reg) _,
            t0 = out(reg) _,
            t1 = out(reg) _,
            t2 = out(reg) _,
            t3 = out(reg) _,
            t4 = out(reg) _,
            r0 = lateout(reg) r0,
            r1 = lateout(reg) r1,
            r2 = lateout(reg) r2,
            r3 = lateout(reg) r3,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

/// a b + c d, Montgomery-reduced once: CIOS whose every row adds both
/// products before its reduction step. With p < 2^254 a row's accumulator
/// stays below 2^320 (five limbs) and the result below 1.5 p, so one
/// conditional subtraction reduces it. arkworks' quadratic extension
/// multiplies through sums of two products: this is an Fq2 product's
/// component.
#[inline(always)]
pub fn sum2(
    a: [u64; 4],
    b: [u64; 4],
    c: [u64; 4],
    d: [u64; 4],
    p: [u64; 4],
    inv: u64,
) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: reads the four limbs of `b` and of `d` through their pointers,
    // which live for the call; every other operand is a register.
    unsafe {
        core::arch::asm!(
            "mov {t0}, xzr",
            "mov {t1}, xzr",
            "mov {t2}, xzr",
            "mov {t3}, xzr",
            "mov {t4}, xzr",
            "ldr {bi}, [{bp}, #0]",
            "mul {l0}, {a0}, {bi}",
            "mul {l1}, {a1}, {bi}",
            "adds {t0}, {t0}, {l0}",
            "mul {l0}, {a2}, {bi}",
            "adcs {t1}, {t1}, {l1}",
            "mul {l1}, {a3}, {bi}",
            "adcs {t2}, {t2}, {l0}",
            "adcs {t3}, {t3}, {l1}",
            "adc {t4}, {t4}, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {t4}, {h1}",
            "ldr {bi}, [{dp}, #0]",
            "mul {l0}, {c0}, {bi}",
            "mul {l1}, {c1}, {bi}",
            "adds {t0}, {t0}, {l0}",
            "mul {l0}, {c2}, {bi}",
            "adcs {t1}, {t1}, {l1}",
            "mul {l1}, {c3}, {bi}",
            "adcs {t2}, {t2}, {l0}",
            "adcs {t3}, {t3}, {l1}",
            "adc {t4}, {t4}, xzr",
            "umulh {h0}, {c0}, {bi}",
            "umulh {h1}, {c1}, {bi}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {c2}, {bi}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {c3}, {bi}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {t4}, {h1}",
            "mul {m}, {t0}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t0}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t1}, {t1}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t2}, {t2}, {l0}",
            "adcs {t3}, {t3}, {l1}",
            "adc {t4}, {t4}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {t4}, {h1}",
            "ldr {bi}, [{bp}, #8]",
            "mul {l0}, {a0}, {bi}",
            "mul {l1}, {a1}, {bi}",
            "adds {t1}, {t1}, {l0}",
            "mul {l0}, {a2}, {bi}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {a3}, {bi}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adc {t0}, xzr, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t0}, {t0}, {h1}",
            "ldr {bi}, [{dp}, #8]",
            "mul {l0}, {c0}, {bi}",
            "mul {l1}, {c1}, {bi}",
            "adds {t1}, {t1}, {l0}",
            "mul {l0}, {c2}, {bi}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {c3}, {bi}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adc {t0}, {t0}, xzr",
            "umulh {h0}, {c0}, {bi}",
            "umulh {h1}, {c1}, {bi}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {c2}, {bi}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {c3}, {bi}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t0}, {t0}, {h1}",
            "mul {m}, {t1}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t1}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adc {t0}, {t0}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t0}, {t0}, {h1}",
            "ldr {bi}, [{bp}, #16]",
            "mul {l0}, {a0}, {bi}",
            "mul {l1}, {a1}, {bi}",
            "adds {t2}, {t2}, {l0}",
            "mul {l0}, {a2}, {bi}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {a3}, {bi}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t0}, {t0}, {l1}",
            "adc {t1}, xzr, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t0}, {t0}, {h0}",
            "adc {t1}, {t1}, {h1}",
            "ldr {bi}, [{dp}, #16]",
            "mul {l0}, {c0}, {bi}",
            "mul {l1}, {c1}, {bi}",
            "adds {t2}, {t2}, {l0}",
            "mul {l0}, {c2}, {bi}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {c3}, {bi}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t0}, {t0}, {l1}",
            "adc {t1}, {t1}, xzr",
            "umulh {h0}, {c0}, {bi}",
            "umulh {h1}, {c1}, {bi}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {c2}, {bi}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {c3}, {bi}",
            "adcs {t0}, {t0}, {h0}",
            "adc {t1}, {t1}, {h1}",
            "mul {m}, {t2}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t2}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t0}, {t0}, {l1}",
            "adc {t1}, {t1}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t0}, {t0}, {h0}",
            "adc {t1}, {t1}, {h1}",
            "ldr {bi}, [{bp}, #24]",
            "mul {l0}, {a0}, {bi}",
            "mul {l1}, {a1}, {bi}",
            "adds {t3}, {t3}, {l0}",
            "mul {l0}, {a2}, {bi}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {a3}, {bi}",
            "adcs {t0}, {t0}, {l0}",
            "adcs {t1}, {t1}, {l1}",
            "adc {t2}, xzr, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t0}, {t0}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t1}, {t1}, {h0}",
            "adc {t2}, {t2}, {h1}",
            "ldr {bi}, [{dp}, #24]",
            "mul {l0}, {c0}, {bi}",
            "mul {l1}, {c1}, {bi}",
            "adds {t3}, {t3}, {l0}",
            "mul {l0}, {c2}, {bi}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {c3}, {bi}",
            "adcs {t0}, {t0}, {l0}",
            "adcs {t1}, {t1}, {l1}",
            "adc {t2}, {t2}, xzr",
            "umulh {h0}, {c0}, {bi}",
            "umulh {h1}, {c1}, {bi}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {c2}, {bi}",
            "adcs {t0}, {t0}, {h1}",
            "umulh {h1}, {c3}, {bi}",
            "adcs {t1}, {t1}, {h0}",
            "adc {t2}, {t2}, {h1}",
            "mul {m}, {t3}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t3}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t0}, {t0}, {l0}",
            "adcs {t1}, {t1}, {l1}",
            "adc {t2}, {t2}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t0}, {t0}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t1}, {t1}, {h0}",
            "adc {t2}, {t2}, {h1}",
            "subs {l0}, {t4}, {p0}",
            "sbcs {l1}, {t0}, {p1}",
            "sbcs {h0}, {t1}, {p2}",
            "sbcs {h1}, {t2}, {p3}",
            "csel {r0}, {t4}, {l0}, lo",
            "csel {r1}, {t0}, {l1}, lo",
            "csel {r2}, {t1}, {h0}, lo",
            "csel {r3}, {t2}, {h1}, lo",
            a0 = in(reg) a[0],
            a1 = in(reg) a[1],
            a2 = in(reg) a[2],
            a3 = in(reg) a[3],
            c0 = in(reg) c[0],
            c1 = in(reg) c[1],
            c2 = in(reg) c[2],
            c3 = in(reg) c[3],
            bp = in(reg) b.as_ptr(),
            dp = in(reg) d.as_ptr(),
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            inv = in(reg) inv,
            bi = out(reg) _,
            m = out(reg) _,
            l0 = out(reg) _,
            l1 = out(reg) _,
            h0 = out(reg) _,
            h1 = out(reg) _,
            t0 = out(reg) _,
            t1 = out(reg) _,
            t2 = out(reg) _,
            t3 = out(reg) _,
            t4 = out(reg) _,
            r0 = lateout(reg) r0,
            r1 = lateout(reg) r1,
            r2 = lateout(reg) r2,
            r3 = lateout(reg) r3,
            options(readonly, nostack),
        );
    }
    [r0, r1, r2, r3]
}

// Addition, subtraction and negation without branches: arkworks' compare
// and conditionally adjust, and the branch on random field elements is
// mispredicted half the time (a quarter of a G2 MSM's time went to Fq2
// subtractions on an M5 Pro). Inputs reduced modulo p < 2^254.

/// a + b mod p: the sum (below 2p, no carry out of 256 bits) less p unless
/// that borrows.
#[inline(always)]
pub fn add(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: registers only.
    unsafe {
        core::arch::asm!(
            "adds {s0}, {s0}, {b0}",
            "adcs {s1}, {s1}, {b1}",
            "adcs {s2}, {s2}, {b2}",
            "adc {s3}, {s3}, {b3}",
            "subs {d0}, {s0}, {p0}",
            "sbcs {d1}, {s1}, {p1}",
            "sbcs {d2}, {s2}, {p2}",
            "sbcs {d3}, {s3}, {p3}",
            "csel {r0}, {s0}, {d0}, lo",
            "csel {r1}, {s1}, {d1}, lo",
            "csel {r2}, {s2}, {d2}, lo",
            "csel {r3}, {s3}, {d3}, lo",
            s0 = inout(reg) a[0] => _,
            s1 = inout(reg) a[1] => _,
            s2 = inout(reg) a[2] => _,
            s3 = inout(reg) a[3] => _,
            b0 = in(reg) b[0],
            b1 = in(reg) b[1],
            b2 = in(reg) b[2],
            b3 = in(reg) b[3],
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            d0 = out(reg) _,
            d1 = out(reg) _,
            d2 = out(reg) _,
            d3 = out(reg) _,
            r0 = lateout(reg) r0,
            r1 = lateout(reg) r1,
            r2 = lateout(reg) r2,
            r3 = lateout(reg) r3,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

/// Conditional subtraction for an input below twice the modulus.
#[cfg(target_os = "macos")]
#[inline(always)]
pub fn reduce_once(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: all operands are registers.
    unsafe {
        core::arch::asm!(
            "subs {d0}, {a0}, {p0}",
            "sbcs {d1}, {a1}, {p1}",
            "sbcs {d2}, {a2}, {p2}",
            "sbcs {d3}, {a3}, {p3}",
            "csel {a0}, {a0}, {d0}, lo",
            "csel {a1}, {a1}, {d1}, lo",
            "csel {a2}, {a2}, {d2}, lo",
            "csel {a3}, {a3}, {d3}, lo",
            a0 = inout(reg) a[0] => r0,
            a1 = inout(reg) a[1] => r1,
            a2 = inout(reg) a[2] => r2,
            a3 = inout(reg) a[3] => r3,
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            d0 = out(reg) _,
            d1 = out(reg) _,
            d2 = out(reg) _,
            d3 = out(reg) _,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

/// a - b mod p: the difference, plus p when it borrows.
#[inline(always)]
pub fn sub(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: registers only.
    unsafe {
        core::arch::asm!(
            "subs {d0}, {d0}, {b0}",
            "sbcs {d1}, {d1}, {b1}",
            "sbcs {d2}, {d2}, {b2}",
            "sbcs {d3}, {d3}, {b3}",
            "csel {m0}, {p0}, xzr, lo",
            "csel {m1}, {p1}, xzr, lo",
            "csel {m2}, {p2}, xzr, lo",
            "csel {m3}, {p3}, xzr, lo",
            "adds {r0}, {d0}, {m0}",
            "adcs {r1}, {d1}, {m1}",
            "adcs {r2}, {d2}, {m2}",
            "adc {r3}, {d3}, {m3}",
            d0 = inout(reg) a[0] => _,
            d1 = inout(reg) a[1] => _,
            d2 = inout(reg) a[2] => _,
            d3 = inout(reg) a[3] => _,
            b0 = in(reg) b[0],
            b1 = in(reg) b[1],
            b2 = in(reg) b[2],
            b3 = in(reg) b[3],
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            m0 = out(reg) _,
            m1 = out(reg) _,
            m2 = out(reg) _,
            m3 = out(reg) _,
            r0 = lateout(reg) r0,
            r1 = lateout(reg) r1,
            r2 = lateout(reg) r2,
            r3 = lateout(reg) r3,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

/// -a mod p: p - a, or zero for zero.
#[inline(always)]
pub fn neg(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: registers only.
    unsafe {
        core::arch::asm!(
            "subs {d0}, {p0}, {a0}",
            "sbcs {d1}, {p1}, {a1}",
            "sbcs {d2}, {p2}, {a2}",
            "sbc {d3}, {p3}, {a3}",
            "orr {z}, {a0}, {a1}",
            "orr {z}, {z}, {a2}",
            "orr {z}, {z}, {a3}",
            "cmp {z}, #0",
            "csel {r0}, {d0}, xzr, ne",
            "csel {r1}, {d1}, xzr, ne",
            "csel {r2}, {d2}, xzr, ne",
            "csel {r3}, {d3}, xzr, ne",
            a0 = in(reg) a[0],
            a1 = in(reg) a[1],
            a2 = in(reg) a[2],
            a3 = in(reg) a[3],
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            d0 = out(reg) _,
            d1 = out(reg) _,
            d2 = out(reg) _,
            d3 = out(reg) _,
            z = out(reg) _,
            r0 = lateout(reg) r0,
            r1 = lateout(reg) r1,
            r2 = lateout(reg) r2,
            r3 = lateout(reg) r3,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

/// The 512-bit product a b, schoolbook with one carry chain for the low and
/// one for the high halves of each row.
#[inline(always)]
pub fn wide(a: [u64; 4], b: &[u64; 4]) -> [u64; 8] {
    let (t0, t1, t2, t3, t4, t5, t6, t7): (u64, u64, u64, u64, u64, u64, u64, u64);
    // SAFETY: all operands are registers.
    unsafe {
        core::arch::asm!(
            "mul {t0}, {a0}, {b0}",
            "mul {t1}, {a1}, {b0}",
            "mul {t2}, {a2}, {b0}",
            "mul {t3}, {a3}, {b0}",
            "umulh {h0}, {a0}, {b0}",
            "umulh {h1}, {a1}, {b0}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {a2}, {b0}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {a3}, {b0}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {h1}, xzr",
            "mul {l0}, {a0}, {b1}",
            "mul {l1}, {a1}, {b1}",
            "adds {t1}, {t1}, {l0}",
            "mul {l0}, {a2}, {b1}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {a3}, {b1}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adc {t5}, xzr, xzr",
            "umulh {h0}, {a0}, {b1}",
            "umulh {h1}, {a1}, {b1}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {a2}, {b1}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {a3}, {b1}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t5}, {t5}, {h1}",
            "mul {l0}, {a0}, {b2}",
            "mul {l1}, {a1}, {b2}",
            "adds {t2}, {t2}, {l0}",
            "mul {l0}, {a2}, {b2}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {a3}, {b2}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t5}, {t5}, {l1}",
            "adc {t6}, xzr, xzr",
            "umulh {h0}, {a0}, {b2}",
            "umulh {h1}, {a1}, {b2}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {a2}, {b2}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {a3}, {b2}",
            "adcs {t5}, {t5}, {h0}",
            "adc {t6}, {t6}, {h1}",
            "mul {l0}, {a0}, {b3}",
            "mul {l1}, {a1}, {b3}",
            "adds {t3}, {t3}, {l0}",
            "mul {l0}, {a2}, {b3}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {a3}, {b3}",
            "adcs {t5}, {t5}, {l0}",
            "adcs {t6}, {t6}, {l1}",
            "adc {t7}, xzr, xzr",
            "umulh {h0}, {a0}, {b3}",
            "umulh {h1}, {a1}, {b3}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {a2}, {b3}",
            "adcs {t5}, {t5}, {h1}",
            "umulh {h1}, {a3}, {b3}",
            "adcs {t6}, {t6}, {h0}",
            "adc {t7}, {t7}, {h1}",
            a0 = in(reg) a[0],
            a1 = in(reg) a[1],
            a2 = in(reg) a[2],
            a3 = in(reg) a[3],
            b0 = in(reg) b[0],
            b1 = in(reg) b[1],
            b2 = in(reg) b[2],
            b3 = in(reg) b[3],
            l0 = out(reg) _,
            l1 = out(reg) _,
            h0 = out(reg) _,
            h1 = out(reg) _,
            t0 = out(reg) t0,
            t1 = out(reg) t1,
            t2 = out(reg) t2,
            t3 = out(reg) t3,
            t4 = out(reg) t4,
            t5 = out(reg) t5,
            t6 = out(reg) t6,
            t7 = out(reg) t7,
            options(pure, nomem, nostack),
        );
    }
    [t0, t1, t2, t3, t4, t5, t6, t7]
}

/// Accumulate the off-diagonal products once, double them and add the squares.
#[cfg(target_os = "macos")]
#[inline(always)]
pub fn square(a: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    square_inner::<true>(a, p, inv)
}

/// The square of a value below 2p is below pR, so the Montgomery
/// reduction's result before normalization is below 2p.
#[cfg(target_os = "macos")]
#[inline(always)]
pub fn square_loose(a: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    square_inner::<false>(a, p, inv)
}

#[cfg(target_os = "macos")]
#[inline(always)]
fn square_inner<const NORMALIZE: bool>(a: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let mut product = [0u64; 8];
    for i in 0..3 {
        let mut carry = 0u64;
        for j in i + 1..4 {
            let limb = a[i] as u128 * a[j] as u128 + product[i + j] as u128 + carry as u128;
            product[i + j] = limb as u64;
            carry = (limb >> 64) as u64;
        }
        product[i + 4] = carry;
    }
    let mut carry = 0u64;
    for limb in &mut product {
        let next = *limb >> 63;
        *limb = (*limb << 1) | carry;
        carry = next;
    }
    for (i, pair) in product.chunks_exact_mut(2).enumerate() {
        let low = a[i] as u128 * a[i] as u128 + pair[0] as u128 + carry as u128;
        let high = (low >> 64) + pair[1] as u128;
        pair[0] = low as u64;
        pair[1] = high as u64;
        carry = (high >> 64) as u64;
    }
    redc_inner::<NORMALIZE>(product, p, inv)
}

/// Montgomery reduction of a 512-bit t < pR, where R = 2^256.
/// The unreduced result is below 2p.
#[inline(always)]
pub fn redc(t: [u64; 8], p: [u64; 4], inv: u64) -> [u64; 4] {
    redc_inner::<true>(t, p, inv)
}

#[inline(always)]
fn redc_inner<const NORMALIZE: bool>(t: [u64; 8], p: [u64; 4], inv: u64) -> [u64; 4] {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: registers only.
    unsafe {
        core::arch::asm!(
            "mul {m}, {t0}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t0}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t1}, {t1}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t2}, {t2}, {l0}",
            "adcs {t3}, {t3}, {l1}",
            "adcs {t4}, {t4}, xzr",
            "adcs {t5}, {t5}, xzr",
            "adcs {t6}, {t6}, xzr",
            "adcs {t7}, {t7}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t3}, {t3}, {h0}",
            "adcs {t4}, {t4}, {h1}",
            "adcs {t5}, {t5}, xzr",
            "adcs {t6}, {t6}, xzr",
            "adcs {t7}, {t7}, xzr",
            "mul {m}, {t1}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t1}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t2}, {t2}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t3}, {t3}, {l0}",
            "adcs {t4}, {t4}, {l1}",
            "adcs {t5}, {t5}, xzr",
            "adcs {t6}, {t6}, xzr",
            "adcs {t7}, {t7}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t4}, {t4}, {h0}",
            "adcs {t5}, {t5}, {h1}",
            "adcs {t6}, {t6}, xzr",
            "adcs {t7}, {t7}, xzr",
            "mul {m}, {t2}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t2}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t3}, {t3}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t4}, {t4}, {l0}",
            "adcs {t5}, {t5}, {l1}",
            "adcs {t6}, {t6}, xzr",
            "adcs {t7}, {t7}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t5}, {t5}, {h0}",
            "adcs {t6}, {t6}, {h1}",
            "adcs {t7}, {t7}, xzr",
            "mul {m}, {t3}, {inv}",
            "mul {l1}, {m}, {p1}",
            "cmp {t3}, #1",
            "mul {l0}, {m}, {p2}",
            "adcs {t4}, {t4}, {l1}",
            "mul {l1}, {m}, {p3}",
            "adcs {t5}, {t5}, {l0}",
            "adcs {t6}, {t6}, {l1}",
            "adcs {t7}, {t7}, xzr",
            "umulh {h0}, {m}, {p0}",
            "umulh {h1}, {m}, {p1}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {m}, {p2}",
            "adcs {t5}, {t5}, {h1}",
            "umulh {h1}, {m}, {p3}",
            "adcs {t6}, {t6}, {h0}",
            "adcs {t7}, {t7}, {h1}",
            ".if {normalize}",
            "subs {l0}, {t4}, {p0}",
            "sbcs {l1}, {t5}, {p1}",
            "sbcs {h0}, {t6}, {p2}",
            "sbcs {h1}, {t7}, {p3}",
            "csel {t4}, {t4}, {l0}, lo",
            "csel {t5}, {t5}, {l1}, lo",
            "csel {t6}, {t6}, {h0}, lo",
            "csel {t7}, {t7}, {h1}, lo",
            ".endif",
            normalize = const NORMALIZE as u32,
            t0 = inout(reg) t[0] => _,
            t1 = inout(reg) t[1] => _,
            t2 = inout(reg) t[2] => _,
            t3 = inout(reg) t[3] => _,
            t4 = inout(reg) t[4] => r0,
            t5 = inout(reg) t[5] => r1,
            t6 = inout(reg) t[6] => r2,
            t7 = inout(reg) t[7] => r3,
            p0 = in(reg) p[0],
            p1 = in(reg) p[1],
            p2 = in(reg) p[2],
            p3 = in(reg) p[3],
            inv = in(reg) inv,
            m = out(reg) _,
            l0 = out(reg) _,
            l1 = out(reg) _,
            h0 = out(reg) _,
            h1 = out(reg) _,
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3]
}

/// a - b over 512 bits (a >= b).
#[inline(always)]
pub fn sub_wide(a: [u64; 8], b: [u64; 8]) -> [u64; 8] {
    let (r0, r1, r2, r3, r4, r5, r6, r7): (u64, u64, u64, u64, u64, u64, u64, u64);
    // SAFETY: registers otherwise.
    unsafe {
        core::arch::asm!(
            "subs {a0}, {a0}, {b0}",
            "sbcs {a1}, {a1}, {b1}",
            "sbcs {a2}, {a2}, {b2}",
            "sbcs {a3}, {a3}, {b3}",
            "sbcs {a4}, {a4}, {b4}",
            "sbcs {a5}, {a5}, {b5}",
            "sbcs {a6}, {a6}, {b6}",
            "sbcs {a7}, {a7}, {b7}",
            a0 = inout(reg) a[0] => r0,
            a1 = inout(reg) a[1] => r1,
            a2 = inout(reg) a[2] => r2,
            a3 = inout(reg) a[3] => r3,
            a4 = inout(reg) a[4] => r4,
            a5 = inout(reg) a[5] => r5,
            a6 = inout(reg) a[6] => r6,
            a7 = inout(reg) a[7] => r7,
            b0 = in(reg) b[0],
            b1 = in(reg) b[1],
            b2 = in(reg) b[2],
            b3 = in(reg) b[3],
            b4 = in(reg) b[4],
            b5 = in(reg) b[5],
            b6 = in(reg) b[6],
            b7 = in(reg) b[7],
            options(pure, nomem, nostack),
        );
    }
    [r0, r1, r2, r3, r4, r5, r6, r7]
}

/// a - b + c over 512 bits (a + c >= b, the sum below 2^512).
#[inline(always)]
pub fn sub_add_wide(a: [u64; 8], b: [u64; 8], c: &[u64; 8]) -> [u64; 8] {
    let (r0, r1, r2, r3, r4, r5, r6, r7): (u64, u64, u64, u64, u64, u64, u64, u64);
    // SAFETY: reads the eight limbs of `c` through its pointer; registers otherwise.
    unsafe {
        core::arch::asm!(
            "subs {a0}, {a0}, {b0}",
            "sbcs {a1}, {a1}, {b1}",
            "sbcs {a2}, {a2}, {b2}",
            "sbcs {a3}, {a3}, {b3}",
            "sbcs {a4}, {a4}, {b4}",
            "sbcs {a5}, {a5}, {b5}",
            "sbcs {a6}, {a6}, {b6}",
            "sbcs {a7}, {a7}, {b7}",
            "ldp {b0}, {b1}, [{c}]",
            "ldp {b2}, {b3}, [{c}, #16]",
            "ldp {b4}, {b5}, [{c}, #32]",
            "ldp {b6}, {b7}, [{c}, #48]",
            "adds {a0}, {a0}, {b0}",
            "adcs {a1}, {a1}, {b1}",
            "adcs {a2}, {a2}, {b2}",
            "adcs {a3}, {a3}, {b3}",
            "adcs {a4}, {a4}, {b4}",
            "adcs {a5}, {a5}, {b5}",
            "adcs {a6}, {a6}, {b6}",
            "adcs {a7}, {a7}, {b7}",
            a0 = inout(reg) a[0] => r0,
            a1 = inout(reg) a[1] => r1,
            a2 = inout(reg) a[2] => r2,
            a3 = inout(reg) a[3] => r3,
            a4 = inout(reg) a[4] => r4,
            a5 = inout(reg) a[5] => r5,
            a6 = inout(reg) a[6] => r6,
            a7 = inout(reg) a[7] => r7,
            b0 = inout(reg) b[0] => _,
            b1 = inout(reg) b[1] => _,
            b2 = inout(reg) b[2] => _,
            b3 = inout(reg) b[3] => _,
            b4 = inout(reg) b[4] => _,
            b5 = inout(reg) b[5] => _,
            b6 = inout(reg) b[6] => _,
            b7 = inout(reg) b[7] => _,
            c = in(reg) c.as_ptr(),
            options(readonly, nostack),
        );
    }
    [r0, r1, r2, r3, r4, r5, r6, r7]
}


#[cfg(all(test, target_os = "macos"))]
mod loose_tests {
    use super::*;
    use crate::{FpConfig, FqConfig, portable};

    #[test]
    fn loose_products_match_normalized_operands_at_the_bounds() {
        let p = FqConfig::P;
        let inv = FqConfig::INV;
        let twice_p = portable::add4(p, p).0;
        let one = [1, 0, 0, 0];
        let cases = [
            [0; 4],
            one,
            portable::sub4(p, one).0,
            p,
            portable::add4(p, one).0,
            portable::sub4(twice_p, one).0,
        ];
        let normalize = |x| portable::reduce_once(x, p);
        for a in cases {
            assert_eq!(reduce_once(a, p), normalize(a));
            let sq = square_loose(a, p, inv);
            assert!(portable::sub4(sq, twice_p).1);
            assert_eq!(normalize(sq), square(normalize(a), p, inv));
            for b in cases {
                let product = multiply_loose(a, b, p, inv);
                assert!(portable::sub4(product, twice_p).1);
                assert_eq!(normalize(product), multiply(normalize(a), normalize(b), p, inv));
            }
        }
    }
}
