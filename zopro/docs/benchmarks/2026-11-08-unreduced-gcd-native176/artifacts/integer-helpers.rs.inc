#[inline(always)]
fn bit_len(x: &[u64; 4]) -> usize {
    for i in (0..4).rev() {
        if x[i] != 0 {
            return i * 64 + 64 - x[i].leading_zeros() as usize;
        }
    }
    0
}

#[inline(always)]
fn approximate(x: &[u64; 4], bits: usize) -> u64 {
    if bits <= 64 {
        return x[0];
    }
    let shift = bits - 33;
    let (word, offset) = (shift / 64, shift % 64);
    let mut high = x[word] >> offset;
    if offset != 0 && word < 3 {
        high |= x[word + 1] << (64 - offset);
    }
    (high << BITS) | (x[0] & MASK)
}

/// The signed 320-bit linear combination. Coefficients have row L1 norm at most 2^62,
/// so each 128-bit intermediate and the signed high word have ample room.
#[inline(always)]
fn linear(a: &[u64; 4], f: i64, b: &[u64; 4], g: i64) -> [u64; 5] {
    let mut out = [0; 5];
    let mut carry = 0i128;
    for j in 0..4 {
        let t = (a[j] as i128) * (f as i128) + (b[j] as i128) * (g as i128) + carry;
        out[j] = t as u64;
        carry = t >> 64;
    }
    out[4] = carry as u64;
    out
}

#[inline(always)]
fn integer_update(a: &[u64; 4], b: &[u64; 4], f: &mut i64, g: &mut i64) -> [u64; 4] {
    let mut t = linear(a, *f, b, *g);
    if (t[4] as i64) < 0 {
        *f = -*f;
        *g = -*g;
        let mut carry = 1u128;
        for limb in &mut t {
            carry += (!*limb) as u128;
            *limb = carry as u64;
            carry >>= 64;
        }
    }
    debug_assert_eq!(t[0] & MASK, 0, "GCD matrix must cancel its low bits");
    debug_assert_eq!(t[4] >> BITS, 0);
    core::array::from_fn(|j| (t[j] >> BITS) | (t[j + 1] << (64 - BITS)))
}

