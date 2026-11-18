# Integrating opt-in G2 diversity checking

Commit `f7b2ddcdf9` adds the measured inline grouping check to the native prover. Set `ZOPRO_CPU_GROUP_DIVERSITY=1` to enable it alongside scalar grouping. The check uses at most 32 KiB of bitmap payload plus the fixed reservation overhead. With an inconclusive lower bound or a denied reservation, ordinary grouping continues.

All 191 implementation and harness file hashes used for the repository rebuild match experiment 304. The linked SDK digest and native text section differ. The two-host C7a figures apply to the retained 304 executable. On 16 local Ryzen cores, the three-block 1×2 rebuild comparison was noisy: the enabled repository build was 15.20% slower by median paired ratio than the retained enabled candidate. This integration check establishes correctness across all eight shapes. C7a latency of the separately linked binary requires a target-host comparison. [Matched C7a evidence](../2026-11-18-combined-c7a307/README.md)

| Shape | Archived accepted ms | Archived enabled ms | Rebuilt off ms | Rebuilt on ms | Rebuilt on M5 CPU / Metal | Peak MiB on / off |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.619 | 8.605 | 8.516 | 9.913 | 3.73× / 2.42× | 58 / 70 |
| 2x2 | 11.866 | 12.031 | 12.088 | 11.949 | 4.94× / 2.51× | 152 / 153 |
| 4x2 | 18.312 | 17.172 | 17.144 | 17.080 | 6.38× / 2.81× | 210 / 206 |
| 8x2 | 26.787 | 27.164 | 26.735 | 26.683 | 7.08× / 2.77× | 345 / 361 |
| 12x2 | 42.589 | 38.745 | 38.787 | 38.996 | 7.28× / 2.59× | 504 / 507 |
| 16x2 | 46.485 | 45.989 | 46.813 | 46.846 | 7.15× / 2.65× | 610 / 591 |
| 24x2 | 75.807 | 79.745 | 75.461 | 75.381 | 7.00× / 2.33× | 953 / 940 |
| merge-24x1 | 76.982 | 76.968 | 74.884 | 76.857 | 6.60× / 2.28× | 897 / 884 |

The native suite passed 142 tests on the repository build, and the no-IFMA prover and Zolana SDK compiled. Each timing process used three primes and 11 timed proofs. All 96 process exports, two additional low-worker or zero-allowance proofs, and 18 exports from two simultaneous clients passed external verification. The two-client tests covered enabled, disabled and denied preparation. Four-worker and generic validation used x86 hardware.

[Timings and concurrency data](artifacts/results.json.gz) · [Allocation-denial and low-worker checks](artifacts/fallback.json) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md)
