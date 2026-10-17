// Montgomery multiplication on AArch64 with R = 2^256: CIOS over four
// 64-bit limbs, the reduction interleaved into each row, in assembly so
// that the products go through MUL and UMULH with one carry chain each.
// Inputs reduced modulo p, p < 2^254 (both BN254 moduli), so the unreduced
// result is below 2p and one conditional subtraction reduces it. Generated
// unrolled: the row shift is a renaming of the accumulator registers.
// The one place the crate uses unsafe: the assembly reads `b` through a
// pointer that lives for the call and touches nothing else.
#![allow(unsafe_code)]
use ark_ff::BigInt;

#[inline(always)]
pub fn multiply(a: BigInt<4>, b: BigInt<4>, p: BigInt<4>, inv: u64) -> BigInt<4> {
    let (r0, r1, r2, r3): (u64, u64, u64, u64);
    // SAFETY: reads the four limbs of `b` through its pointer, which lives
    // for the call; every other operand is a register.
    unsafe {
        core::arch::asm!(
            "ldr {bi}, [{bp}, #0]",
            "mul {t0}, {a0}, {bi}",
            "mul {t1}, {a1}, {bi}",
            "mul {t2}, {a2}, {bi}",
            "mul {t3}, {a3}, {bi}",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t1}, {t1}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t2}, {t2}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t3}, {t3}, {h0}",
            "adc {t4}, {h1}, xzr",
            "mul {m}, {t0}, {inv}",
            "mul {l0}, {m}, {p0}",
            "mul {l1}, {m}, {p1}",
            "adds {t0}, {t0}, {l0}",
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
            "adc {t0}, {t0}, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t2}, {t2}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t3}, {t3}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t4}, {t4}, {h0}",
            "adc {t0}, {t0}, {h1}",
            "mul {m}, {t1}, {inv}",
            "mul {l0}, {m}, {p0}",
            "mul {l1}, {m}, {p1}",
            "adds {t1}, {t1}, {l0}",
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
            "adc {t1}, {t1}, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t3}, {t3}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t4}, {t4}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t0}, {t0}, {h0}",
            "adc {t1}, {t1}, {h1}",
            "mul {m}, {t2}, {inv}",
            "mul {l0}, {m}, {p0}",
            "mul {l1}, {m}, {p1}",
            "adds {t2}, {t2}, {l0}",
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
            "adc {t2}, {t2}, xzr",
            "umulh {h0}, {a0}, {bi}",
            "umulh {h1}, {a1}, {bi}",
            "adds {t4}, {t4}, {h0}",
            "umulh {h0}, {a2}, {bi}",
            "adcs {t0}, {t0}, {h1}",
            "umulh {h1}, {a3}, {bi}",
            "adcs {t1}, {t1}, {h0}",
            "adc {t2}, {t2}, {h1}",
            "mul {m}, {t3}, {inv}",
            "mul {l0}, {m}, {p0}",
            "mul {l1}, {m}, {p1}",
            "adds {t3}, {t3}, {l0}",
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
            a0 = in(reg) a.0[0],
            a1 = in(reg) a.0[1],
            a2 = in(reg) a.0[2],
            a3 = in(reg) a.0[3],
            bp = in(reg) b.0.as_ptr(),
            p0 = in(reg) p.0[0],
            p1 = in(reg) p.0[1],
            p2 = in(reg) p.0[2],
            p3 = in(reg) p.0[3],
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
    BigInt([r0, r1, r2, r3])
}

#[cfg(test)]
mod tests {
    use super::multiply;
    use ark_ff::{BigInt, Field, Fp256, MontBackend, MontConfig, PrimeField, UniformRand};

    #[test]
    fn matches_arkworks() {
        type Original = Fp256<MontBackend<crate::OriginalFqConfig, 4>>;
        let mut rng = ark_std::test_rng();
        let p = <crate::OriginalFqConfig as MontConfig<4>>::MODULUS;
        let inv = <crate::OriginalFqConfig as MontConfig<4>>::INV;
        let mut last = p;
        last.0[0] -= 1;
        let edge = [Original::from_bigint(BigInt([0; 4])).unwrap(), Original::from_bigint(BigInt([1, 0, 0, 0])).unwrap(), Original::new_unchecked(last)];
        for _ in 0..20000 {
            let (a, b) = (Original::rand(&mut rng), Original::rand(&mut rng));
            assert_eq!(multiply(a.0, b.0, p, inv), (a * b).0);
            assert_eq!(multiply(a.0, a.0, p, inv), a.square().0);
            for e in edge {
                assert_eq!(multiply(a.0, e.0, p, inv), (a * e).0);
                assert_eq!(multiply(e.0, a.0, p, inv), (e * a).0);
            }
        }
    }
}
