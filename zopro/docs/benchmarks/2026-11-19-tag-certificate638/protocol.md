# Full proofs with a bounded one-pass fingerprint certificate

Implement the single-pass fingerprint layout measured in study 637 using portable Rust integer operations. It uses the same scratch byte count as the release bitmap. Each bank stores 16 byte fingerprints in a 128-bit word. A full bank ignores new fingerprints. When the grouping cutoff exceeds or equals the slot capacity, use the existing bitmap. A byte-based reference test checks the optimized count at saturation, repeated inputs, zeros and the exact rejection boundary.

Compare the release, two-pass bitmap and single-pass tags on original 1×2, witnesses 29 and 32, and original 2×2 and 4×2. Use six fresh execution-order-balanced blocks per input with 21 warm timed SDK calls and one separate profile per variant. Each process primes three proofs. A separate 1×2 admission trace must report whether the reference building the 21,006-entry table is avoided. Verify every exported proof. Report latency and memory for all three implementations including the matched control and both M5 ratios where supplied.

[Candidate source](artifacts/source.patch) · [Captured cardinalities](../2026-11-19-tag-census637/protocol.md) · [Two-pass comparison](../2026-11-19-split-certificate636/protocol.md)
