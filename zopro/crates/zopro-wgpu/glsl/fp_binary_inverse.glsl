const fp MODULUS = fp(0xd87cfd47u, 0x3c208c16u, 0x6871ca8du, 0x97816a91u, 0x8181585du, 0xb85045b6u, 0xe131a029u, 0x30644e72u);
const fp R_SQUARED = fp(0x538afa89u, 0xf32cfc5bu, 0xd44501fbu, 0xb5e71911u, 0x0a417ff6u, 0x47ab1effu, 0xcab8351fu, 0x06d89f71u);

bool integer_one(fp a) {
  return a.l0 == 1u && (a.l1 | a.l2 | a.l3 | a.l4 | a.l5 | a.l6 | a.l7) == 0u;
}

fp right_shift(fp x) {
  return fp((x.l0 >> 1u) | (x.l1 << 31u), (x.l1 >> 1u) | (x.l2 << 31u), (x.l2 >> 1u) | (x.l3 << 31u), (x.l3 >> 1u) | (x.l4 << 31u), (x.l4 >> 1u) | (x.l5 << 31u), (x.l5 >> 1u) | (x.l6 << 31u), (x.l6 >> 1u) | (x.l7 << 31u), x.l7 >> 1u);
}

fp modular_half(fp a) {
  uint c0, c1, carry = 0u;
  const uint mask = 0u - (a.l0 & 1u);
  fp c;
  c.l0 = uaddCarry(a.l0, Q0 & mask, c0); c.l0 = uaddCarry(c.l0, carry, c1); carry = c0 | c1;
  c.l1 = uaddCarry(a.l1, Q1 & mask, c0); c.l1 = uaddCarry(c.l1, carry, c1); carry = c0 | c1;
  c.l2 = uaddCarry(a.l2, Q2 & mask, c0); c.l2 = uaddCarry(c.l2, carry, c1); carry = c0 | c1;
  c.l3 = uaddCarry(a.l3, Q3 & mask, c0); c.l3 = uaddCarry(c.l3, carry, c1); carry = c0 | c1;
  c.l4 = uaddCarry(a.l4, Q4 & mask, c0); c.l4 = uaddCarry(c.l4, carry, c1); carry = c0 | c1;
  c.l5 = uaddCarry(a.l5, Q5 & mask, c0); c.l5 = uaddCarry(c.l5, carry, c1); carry = c0 | c1;
  c.l6 = uaddCarry(a.l6, Q6 & mask, c0); c.l6 = uaddCarry(c.l6, carry, c1); carry = c0 | c1;
  c.l7 = uaddCarry(a.l7, Q7 & mask, c0); c.l7 = uaddCarry(c.l7, carry, c1); carry = c0 | c1;
  return right_shift(c);
}

fp raw_subtract(fp a, fp b, out uint borrow) {
  uint b0, b1;
  borrow = 0u;
  fp d;
  d.l0 = usubBorrow(a.l0, b.l0, b0); d.l0 = usubBorrow(d.l0, borrow, b1); borrow = b0 | b1;
  d.l1 = usubBorrow(a.l1, b.l1, b0); d.l1 = usubBorrow(d.l1, borrow, b1); borrow = b0 | b1;
  d.l2 = usubBorrow(a.l2, b.l2, b0); d.l2 = usubBorrow(d.l2, borrow, b1); borrow = b0 | b1;
  d.l3 = usubBorrow(a.l3, b.l3, b0); d.l3 = usubBorrow(d.l3, borrow, b1); borrow = b0 | b1;
  d.l4 = usubBorrow(a.l4, b.l4, b0); d.l4 = usubBorrow(d.l4, borrow, b1); borrow = b0 | b1;
  d.l5 = usubBorrow(a.l5, b.l5, b0); d.l5 = usubBorrow(d.l5, borrow, b1); borrow = b0 | b1;
  d.l6 = usubBorrow(a.l6, b.l6, b0); d.l6 = usubBorrow(d.l6, borrow, b1); borrow = b0 | b1;
  d.l7 = usubBorrow(a.l7, b.l7, b0); d.l7 = usubBorrow(d.l7, borrow, b1); borrow = b0 | b1;
  return d;
}

fp field_inverse(fp x) {
  if (fp_is_zero(x))
    return x;
  fp u = x;
  fp v = MODULUS;
  // u*x^(-1)*R^2 == a, v*x^(-1)*R^2 == b, both modulo p.
  // The input is x = value*R, so the winning coefficient is value^(-1)*R.
  fp a = R_SQUARED;
  fp b = fp_zero();
  [[dont_unroll]] while (!integer_one(u) && !integer_one(v)) {
    if ((u.l0 & 1u) == 0u) {
      u = right_shift(u);
      a = modular_half(a);
    } else if ((v.l0 & 1u) == 0u) {
      v = right_shift(v);
      b = modular_half(b);
    } else {
      uint borrow;
      const fp difference = raw_subtract(u, v, borrow);
      if (borrow == 0u) {
        u = right_shift(difference);
        a = modular_half(fp_sub(a, b));
      } else {
        v = right_shift(raw_subtract(v, u, borrow));
        b = modular_half(fp_sub(b, a));
      }
    }
  }
  return integer_one(u) ? a : b;
}
