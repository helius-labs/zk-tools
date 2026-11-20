# Borrow full rows in the mixed-radix Euler FFT

- The first mixed-radix trial removed field conversions. The FFT still rebuilt full, contiguous rows with per-element bounds checks before IFMA loads. Test loading each half directly from its source slice and using padding only for short tails.
- The native test checks both cached root directions, full and partial input slices, and unchanged materialized outputs. Compare retained mixed, current ordinary and current mixed executables in paired Ryzen blocks on two 2×2, 4×2 and 8×2 requests. The same 2×2 raw layout isolates its behavior from the compact solver profile.
- Separately measure the complete fused quotient phase with and without the direct imports on the same instrumented executable. Ordinary/direct have unclocked companions. Compare the independent full-proof results before choosing adoption.
- Keep the immutable source arrays and existing scratch. Four unaligned SIMD loads access initialized rows of four fields each. No allocations, point copies or format changes.
- Switch to C7a or production qualification only with a repeatable effect above run-to-run variance. This screen allows three minutes for build/test and two minutes for measurement.
