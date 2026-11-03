// Four-limb Montgomery arithmetic with R = 2^256 in plain Rust: the
// backend where there is no assembly, and, being `const`, the one that
// turns the constants into Montgomery form at compile time. Inputs are
// reduced modulo p < 2^254 (both BN254 moduli).

/// x - p when x >= p, else x; for x < 2p.
#[inline(always)]
pub const fn reduce_once(x: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (d, borrow) = sub4(x, p);
    select(borrow, x, d)
}

/// `a` when `pick`, else `b`, without a branch.
#[inline(always)]
pub const fn select(pick: bool, a: [u64; 4], b: [u64; 4]) -> [u64; 4] {
    let mask = (pick as u64).wrapping_neg();
    [
        (a[0] & mask) | (b[0] & !mask),
        (a[1] & mask) | (b[1] & !mask),
        (a[2] & mask) | (b[2] & !mask),
        (a[3] & mask) | (b[3] & !mask),
    ]
}

/// a + b over 256 bits and the carry out.
#[inline(always)]
pub const fn add4(a: [u64; 4], b: [u64; 4]) -> ([u64; 4], bool) {
    let mut r = [0u64; 4];
    let mut carry = false;
    let mut i = 0;
    while i < 4 {
        let (s, c1) = a[i].overflowing_add(b[i]);
        let (s, c2) = s.overflowing_add(carry as u64);
        r[i] = s;
        carry = c1 | c2;
        i += 1;
    }
    (r, carry)
}

/// a - b over 256 bits and the borrow out.
#[inline(always)]
pub const fn sub4(a: [u64; 4], b: [u64; 4]) -> ([u64; 4], bool) {
    let mut r = [0u64; 4];
    let mut borrow = false;
    let mut i = 0;
    while i < 4 {
        let (s, b1) = a[i].overflowing_sub(b[i]);
        let (s, b2) = s.overflowing_sub(borrow as u64);
        r[i] = s;
        borrow = b1 | b2;
        i += 1;
    }
    (r, borrow)
}

#[inline(always)]
pub const fn add(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    reduce_once(add4(a, b).0, p)
}

#[inline(always)]
#[allow(dead_code)]
pub const fn sub(a: [u64; 4], b: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let (d, borrow) = sub4(a, b);
    add4(d, select(borrow, p, [0; 4])).0
}

#[inline(always)]
pub const fn neg(a: [u64; 4], p: [u64; 4]) -> [u64; 4] {
    let zero = (a[0] | a[1] | a[2] | a[3]) == 0;
    select(zero, [0; 4], sub4(p, a).0)
}

/// a b 2^-256 mod p: CIOS.
#[inline(always)]
pub const fn multiply(a: [u64; 4], b: [u64; 4], p: [u64; 4], inv: u64) -> [u64; 4] {
    let mut t = [0u64; 6];
    let mut i = 0;
    while i < 4 {
        let mut carry = 0u64;
        let mut j = 0;
        while j < 4 {
            let v = (a[j] as u128) * (b[i] as u128) + t[j] as u128 + carry as u128;
            t[j] = v as u64;
            carry = (v >> 64) as u64;
            j += 1;
        }
        let v = t[4] as u128 + carry as u128;
        t[4] = v as u64;
        t[5] = (v >> 64) as u64;
        let m = t[0].wrapping_mul(inv);
        let v = (m as u128) * (p[0] as u128) + t[0] as u128;
        let mut carry = (v >> 64) as u64;
        let mut j = 1;
        while j < 4 {
            let v = (m as u128) * (p[j] as u128) + t[j] as u128 + carry as u128;
            t[j - 1] = v as u64;
            carry = (v >> 64) as u64;
            j += 1;
        }
        let v = t[4] as u128 + carry as u128;
        t[3] = v as u64;
        t[4] = t[5] + (v >> 64) as u64;
        i += 1;
    }
    reduce_once([t[0], t[1], t[2], t[3]], p)
}

/// a b + c d, as two products.
#[inline(always)]
#[allow(dead_code)]
pub const fn sum2(
    a: [u64; 4],
    b: [u64; 4],
    c: [u64; 4],
    d: [u64; 4],
    p: [u64; 4],
    inv: u64,
) -> [u64; 4] {
    add(multiply(a, b, p, inv), multiply(c, d, p, inv), p)
}

/// -p^-1 mod 2^64, by Newton's iteration.
pub const fn inv64(p0: u64) -> u64 {
    let mut x = 1u64;
    let mut i = 0;
    while i < 7 {
        x = x.wrapping_mul(2u64.wrapping_sub(p0.wrapping_mul(x)));
        i += 1;
    }
    x.wrapping_neg()
}

/// 2^k mod p.
pub const fn power_of_two(k: u32, p: [u64; 4]) -> [u64; 4] {
    let mut x = [1u64, 0, 0, 0];
    let mut i = 0;
    while i < k {
        x = add(x, x, p);
        i += 1;
    }
    x
}

/// A decimal literal below p, as limbs.
pub const fn decimal(s: &str) -> [u64; 4] {
    let bytes = s.as_bytes();
    let mut x = [0u64; 4];
    let mut i = 0;
    while i < bytes.len() {
        let digit = bytes[i];
        assert!(digit.is_ascii_digit(), "a decimal literal");
        // x = 10 x + digit
        let mut carry = (digit - b'0') as u128;
        let mut j = 0;
        while j < 4 {
            let v = (x[j] as u128) * 10 + carry;
            x[j] = v as u64;
            carry = v >> 64;
            j += 1;
        }
        assert!(carry == 0, "a literal below 2^256");
        i += 1;
    }
    x
}

/// Whether a < b as integers.
#[inline(always)]
pub const fn less(a: [u64; 4], b: [u64; 4]) -> bool {
    sub4(a, b).1
}
