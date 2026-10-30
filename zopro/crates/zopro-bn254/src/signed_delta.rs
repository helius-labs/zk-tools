//! Signed coordinate differences for canonical R256 inputs in eight lanes.
//! Kept outside generated arithmetic; results are normalized and strictly below 2p.
use crate::{ifma8::Ifma8, Fp, FpConfig};
use core::{arch::x86_64::*, marker::PhantomData};
const MASK: u64 = (1 << 52) - 1;

#[inline(always)]
const fn split(x: [u64; 4]) -> [u64; 5] {
    [
        x[0] & MASK,
        ((x[0] >> 52) | (x[1] << 12)) & MASK,
        ((x[1] >> 40) | (x[2] << 24)) & MASK,
        ((x[2] >> 28) | (x[3] << 36)) & MASK,
        x[3] >> 16,
    ]
}

impl<C: FpConfig> Ifma8<C, true> {
    #[inline(always)]
    fn signed_delta_splat(x: u64) -> __m512i {
        unsafe { _mm512_set1_epi64(x as i64) }
    }
    #[inline(always)]
    fn signed_delta_reduce(mut t: [__m512i; 5], modulus: [u64; 4]) -> Self {
        unsafe {
            let p = split(modulus).map(Self::signed_delta_splat);
            let mut d = [Self::signed_delta_splat(0); 5];
            let mut borrow = Self::signed_delta_splat(0);
            let mask = Self::signed_delta_splat(MASK);
            for j in 0..5 {
                let v = _mm512_add_epi64(_mm512_sub_epi64(t[j], p[j]), borrow);
                d[j] = _mm512_and_si512(v, mask);
                borrow = _mm512_srai_epi64::<52>(v);
            }
            let keep = _mm512_cmpneq_epi64_mask(borrow, Self::signed_delta_splat(0));
            for j in 0..5 {
                t[j] = _mm512_mask_blend_epi64(keep, d[j], t[j]);
            }
            Self(t, PhantomData)
        }
    }
    /// Skip the fused result's final subtraction when its high limbs
    /// prove every lane is already below 2p. Equal high limbs remain
    /// ambiguous and take the full reduction, preserving the exact result.
    #[inline(always)]
    fn signed_delta_reduce_fused(t: [__m512i; 5]) -> Self {
        let modulus = const { crate::portable::add4(C::P, C::P).0 };
        let high = Self::signed_delta_splat(split(modulus)[4]);
        let needs = unsafe { _mm512_cmpge_epu64_mask(t[4], high) };
        if needs == 0 {
            Self(t, PhantomData)
        } else {
            core::hint::cold_path();
            Self::signed_delta_reduce(t, modulus)
        }
    }

    /// Canonical operands below p. Apply incoming sign and subtract the anchor
    /// together. The transient negative-zero 2p is corrected before returning.
    #[inline(always)]
    pub fn signed_difference_canonical(&self, ay: &Self, negative: u8) -> Self {
        #[cfg(test)]
        SIGNED_DELTA_HITS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        unsafe {
            let p = split(C::P);
            let twice = split(crate::portable::add4(C::P, C::P).0);
            let flip = _mm512_maskz_mov_epi64(negative, Self::signed_delta_splat(MASK));
            let mut carry = _mm512_maskz_mov_epi64(negative, Self::signed_delta_splat(1));
            let mut t = [Self::signed_delta_splat(0); 5];
            let mask = Self::signed_delta_splat(MASK);
            for j in 0..5 {
                let offset = _mm512_mask_blend_epi64(
                    negative,
                    Self::signed_delta_splat(p[j]),
                    Self::signed_delta_splat(twice[j]),
                );
                let signed = _mm512_xor_si512(self.0[j], flip);
                let v = _mm512_add_epi64(
                    _mm512_sub_epi64(_mm512_add_epi64(offset, signed), ay.0[j]),
                    carry,
                );
                t[j] = _mm512_and_si512(v, mask);
                carry = _mm512_srai_epi64::<52>(v);
            }
            // Positive: p+by-ay in (0,2p). Negative: 2p-by-ay in (0,2p].
            // Preserve the strict <2p invariant, including both input coordinates zero.
            Self::signed_delta_reduce_fused(t)
        }
    }
}
#[cfg(test)]
static SIGNED_DELTA_HITS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(test)]
mod signed_delta_tests {
    use super::*;
    fn run<C: FpConfig, A: ark_ff::PrimeField>() {
        use ark_ff::Field;
        let inv_r = A::from(2u64).pow([256]).inverse().unwrap();
        let convert = |v: Fp<C>| {
            A::from_le_bytes_mod_order(
                &v.0.into_iter()
                    .flat_map(u64::to_le_bytes)
                    .collect::<Vec<_>>(),
            ) * inv_r
        };
        let mut values = vec![
            Fp::<C>::ZERO,
            Fp::<C>::ONE,
            -Fp::<C>::ONE,
            Fp::<C>::from_montgomery([1, 0, 0, 0]),
            Fp::<C>::from_montgomery(crate::portable::sub4(C::P, [1, 0, 0, 0]).0),
        ];
        for bit in 0..254 {
            let mut w = [0; 4];
            w[bit / 64] = 1u64 << (bit % 64);
            if crate::portable::less(w, C::P) {
                values.push(Fp::from_montgomery(w));
                if bit != 0 {
                    values.push(Fp::from_montgomery(
                        crate::portable::sub4(w, [1, 0, 0, 0]).0,
                    ));
                }
            }
        }
        let mut seed = 0x3bb7225620a40519u64;
        let mut count = 0;
        for round in 0..values.len() + 4096 {
            let mut random = || {
                let mut bytes = [0; 64];
                for b in bytes.chunks_mut(8) {
                    seed ^= seed << 13;
                    seed ^= seed >> 7;
                    seed ^= seed << 17;
                    b.copy_from_slice(&seed.to_le_bytes());
                }
                Fp::<C>::from_random_bytes(&bytes)
            };
            let by: [Fp<C>; 8] = core::array::from_fn(|i| {
                if round < values.len() {
                    values[(round + i) % values.len()]
                } else {
                    random()
                }
            });
            let ay: [Fp<C>; 8] = core::array::from_fn(|i| {
                if round < values.len() {
                    values[(round * 37 + i * 71) % values.len()]
                } else {
                    random()
                }
            });
            let signs = round as u8;
            let dy =
                Ifma8::<C, true>::new(&by).signed_difference_canonical(&Ifma8::new(&ay), signs);
            let out = dy.get();
            let words: [[u64; 8]; 5] = unsafe { dy.0.map(|v| core::mem::transmute(v)) };
            let two = split(crate::portable::add4(C::P, C::P).0);
            for i in 0..8 {
                let w = core::array::from_fn::<u64, 5, _>(|j| words[j][i]);
                assert!(w.iter().all(|v| *v <= MASK));
                assert!(w.iter().rev().cmp(two.iter().rev()).is_lt());
                let y = convert(by[i]);
                assert_eq!(
                    convert(out[i]),
                    (if signs >> i & 1 != 0 { -y } else { y }) - convert(ay[i])
                );
                count += 1;
            }
            let scale = Ifma8::<C, true>::new(&by);
            let subsequent = dy.sum2(&scale, &dy, &scale).get();
            for i in 0..8 {
                assert_eq!(
                    convert(subsequent[i]),
                    convert(out[i]) * convert(by[i]) * A::from(2u64)
                );
            }
        }
        for signs in 0..=255 {
            let zero = Ifma8::<C, true>::zero();
            let got = zero.signed_difference_canonical(&zero, signs);
            let raw: [[u64; 8]; 5] = unsafe { got.0.map(|x| core::mem::transmute(x)) };
            for i in 0..8 {
                let v = core::array::from_fn::<u64, 5, _>(|j| raw[j][i]);
                assert_eq!(
                    v,
                    if signs >> i & 1 != 0 {
                        [0; 5]
                    } else {
                        split(C::P)
                    }
                );
            }
            count += 8;
        }
        assert!(SIGNED_DELTA_HITS.load(std::sync::atomic::Ordering::Relaxed) > 0);
        println!(
            "{count} signed difference Arkworks lane comparisons plus following fused products"
        );
    }
    #[test]
    fn signed_delta_fq_ark() {
        run::<crate::FqConfig, ark_ref::Fq>();
    }
    #[test]
    fn signed_delta_fr_ark() {
        run::<crate::FrConfig, ark_ref::Fr>();
    }
}
