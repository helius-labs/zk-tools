# Latency budgets for the remaining small proofs on C7a

With the observed non-curve time in the ordinary C7a runs, reaching 10× M5 CPU through curve work alone would require removing **99% of curve latency on 1×2, 88% on 2×2, and 56–57% on 4×2**. Even using the lower non-curve time from replay runs, the required cuts are about **95%, 85%, and 52%**. The budget for 1×2 and 2×2 therefore depends on reducing solver or quotient time alongside curve work.

The analysis pairs the existing ordinary and all-curve replay clocks by block and input. Subtract the full curve interval from each end-to-end measurement, including the comparison and lookup interval in the replay run. The target minus the measured non-curve time is the time left for curve work. The results below are medians of the block-level calculations. They show how much curve work would have to shrink if the rest of the proof took the measured time. A change to overlap or resource contention could change both intervals. [Small-proof clocks](../2026-11-19-small-phase-clock-c7a814/README.md) · [4×2 clocks](../2026-11-20-four-by-two-phase-c7a891/README.md)

| Shape | Input | Target ms | Active curve ms | Ordinary non-curve ms | Replay non-curve ms | Curve cut at ordinary budget | Curve cut at replay budget | Gap if curve time halves, ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | catalog | 3.7 | 4.202 | 3.662 | 3.496 | 99.1% | 95.1% | +2.060 |
| 1x2 | heldout | 3.7 | 4.117 | 3.675 | 3.487 | 99.4% | 94.9% | +2.021 |
| 2x2 | catalog | 5.9 | 7.010 | 5.054 | 4.830 | 87.9% | 85.3% | +2.659 |
| 2x2 | heldout | 5.9 | 7.187 | 5.008 | 4.832 | 87.6% | 85.0% | +2.703 |
| 4x2 | catalog | 10.9 | 7.529 | 7.630 | 7.264 | 56.9% | 51.9% | +0.516 |
| 4x2 | heldout | 10.9 | 7.547 | 7.612 | 7.377 | 56.2% | 52.6% | +0.466 |

On 4×2, replaying K and B2 together saved about 4 ms on Ryzen, with A and H still evaluated normally. The prior C7a run measured B2 and the K/H pair individually. A matched K/B2 run on C7a will measure whether the joint effect carries over at 96 physical cores. [Ryzen factorial](../2026-11-20-four-by-two-query-factorial-ryzen916/README.md)

## C7a confirmations ready

- **1×2 packed final exponentiation.** Twelve blocks on each of the original, held-out and v27 inputs compare the frozen SDK, the same build with packed final exponentiation off and on, and prepared Aeglos. Zopro uses 80 main and 16 frontend cores. Aeglos uses 96. The package uses the exact Ryzen candidate with the ordinary Miller path. [Local result](../2026-11-20-pairing-factorial-ryzen924/README.md) · [Protocol](artifacts/packed-final925/protocol.json)
- **4×2 joint K/B2.** Twelve blocks on each of two inputs compare frozen and same-build controls, all queries active, K replay, B2 replay, joint K/B2 replay, and all queries replay. The package uses the frozen seeded profile with 64 main and 32 frontend cores. Fourteen separate probes verify the expected exact replay hits before timing. [Protocol](artifacts/joint-k-b2-926/protocol.json)

The archive checks matched all pinned file hashes. Gnark independently verified 38 Ryzen preflight exports. The first 1×2 preflight used 96 Aeglos workers with 32-thread affinity. The corrected run used 32 workers within that affinity and confirmed prepared cache hits. AWS SSO had expired, so both C7a confirmations await execution.

The [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) contains the CPU and Metal comparisons for all eight supplied shapes. Five reach 10× M5 CPU.

[Block-level calculations](artifacts/analysis.json) · [1×2 preflight](artifacts/packed-final925/preflight.json) · [4×2 preflight](artifacts/joint-k-b2-926/preflight.json)
