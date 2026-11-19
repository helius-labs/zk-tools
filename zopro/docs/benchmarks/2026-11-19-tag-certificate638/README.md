# One-pass fingerprints in verified small-input proofs

The single-pass fingerprint check eliminated the unwanted dictionary construction on 1×2, but it was slower than the release on all three 1×2 witnesses by paired median. The two-pass implementation split the 14 paired reference blocks evenly against the release across the two studies. Its reference peak RSS was lower in both. Keep the existing release latency path. [Two-pass block comparisons](artifacts/two-pass-comparisons.json)

Across four traced calls on the original 1×2 request the release reached the 21,006-entry dictionary reservation 4 times, the two-pass certificate 0 times and the one-pass fingerprints 0 times. The maximum temporary certificate allocation is 32 KiB in all three builds. All 157 native core tests passed on the fingerprint build, and all 108 exported proofs verified independently.

| Shape and input | Release / two-pass / tags ms | Tags vs release | Tags vs two-pass | Peak release / two-pass / tags MiB | Tags vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2-0 | 7.013 / 6.997 / 7.122 | +1.05% (1/6) | +1.98% (0/6) | 61.8 / 59.0 / 59.2 | 5.19× / 3.37× |
| 1x2-29 | 6.981 / 6.939 / 7.064 | +0.61% (2/6) | +2.02% (1/6) | 59.5 / 59.8 / 59.0 | — |
| 1x2-32 | 6.993 / 6.958 / 7.036 | +0.54% (2/6) | +1.12% (1/6) | 60.8 / 59.8 / 58.8 | — |
| 2x2-0 | 9.595 / 9.574 / 9.660 | +0.77% (2/6) | +0.98% (2/6) | 111.5 / 111.8 / 109.2 | 6.11× / 3.11× |
| 4x2-0 | 15.314 / 15.191 / 15.344 | -0.27% (3/6) | +1.19% (1/6) | 278.2 / 275.5 / 270.8 | 7.10× / 3.13× |

[Protocol](protocol.md) · [Source](artifacts/source.patch) · [Samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

## Remaining curve work on 1×2

Across the two release profiles of the original request, the G1 streaming bucket update and G2 borrowed-point update accounted for 22.87% and 19.56% of sampled CPU cycles. The named scalar classification routine accounted for 0.27%. Its callers and inlined code can account for additional work. On inputs 29 and 32 the two curve functions together accounted for 46.46% and 43.40%. Percentages sum hardware sampling periods across workers in both profiles before division. In the earlier repeated-input diagnostic, substituting already-computed B2 or K+H sums saved 2.905 or 2.261 ms respectively on the original reference. Those historical timings estimate the work available in the curve queries and include the diagnostic build’s control overhead. [Profile period totals](artifacts/release-profile-periods.json) · [Earlier per-query diagnostic](../2026-11-19-query-ablation580/README.md)
