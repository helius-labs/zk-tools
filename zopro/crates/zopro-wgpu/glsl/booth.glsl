// Choose each carry from the high bit of its digit. The preceding bit
// adds the carry from the lower window. Digits in [-radix/2, radix/2]
// reconstruct the canonical scalar and can be computed independently.
int booth_digit(fr scalar, uint window) {
  const uint offset = window * sort.width;
  const uint radix = 1u << sort.width;
  const uint overlapping = window == 0u
      ? (scalar.l0 & (radix - 1u)) << 1u
      : window_value(scalar, offset - 1u, sort.width + 1u);
  return int((overlapping >> 1u) + (overlapping & 1u))
      - int((overlapping >> sort.width) * radix);
}
