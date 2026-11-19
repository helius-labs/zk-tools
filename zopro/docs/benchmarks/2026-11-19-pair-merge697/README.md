# Pairwise reduction of four G2 bucket partitions

The four-range G2 implementation fills four independent bucket arrays for each signed scalar window. The control waits for all four before merging three arrays into one accumulator. The candidate starts merging each pair as soon as its two fills finish, then merges the two results. Both routes perform three bucket merges and use the existing four partition arrays. A native test compares the result at 1, 4 and 16 threads against full-width scalar multiplication with identity bases, opposite points and signed scalars.

The Ryzen 9950X used 32 logical workers. Separate one-call dispatch checks precede five rotated blocks, each comparing the existing release with the candidate disabled and enabled on the same binary. Each measurement times thirteen single proofs after three warmups. Times are medians of process means. Negative changes favor the pairwise reduction. The route column gives G2 query length, window bits and window count. A fallback means that setting the option did not exercise the implementation on this shape. The verifier checked the expected public inputs and accepted all 140 exports.

| Shape | Request | Observed route | Release / off / on ms | Paired vs off (wins) | Paired vs release (wins) | Peak MiB release / on |
|---|---|---|---:|---:|---:|---:|
| 1x2 | catalog | G2_PAIR_MERGE 30089 11 12 | 7.273 / 7.821 / 7.757 | -0.25% (4/5) | +7.28% (0/5) | 58.5 / 59.5 |
| 1x2 | 29 | G2_PAIR_MERGE 30089 11 12 | 7.015 / 7.608 / 7.635 | -0.44% (3/5) | +8.09% (0/5) | 57.0 / 56.8 |
| 1x4 | catalog | G2_PAIR_MERGE 27814 11 12 | 8.800 / 9.354 / 9.322 | -1.28% (4/5) | +5.50% (0/5) | 86.2 / 92.0 |
| 1x4 | 57 | G2_PAIR_MERGE 27814 11 12 | 8.780 / 9.323 / 9.327 | -0.19% (3/5) | +6.65% (0/5) | 87.5 / 90.0 |
| 1x8 | catalog | G2_PAIR_MERGE 32871 11 12 | 10.287 / 11.026 / 11.143 | +1.51% (2/5) | +8.00% (0/5) | 106.0 / 104.5 |
| 1x8 | 57 | G2_PAIR_MERGE 32871 11 12 | 10.234 / 11.166 / 10.903 | -2.22% (4/5) | +6.66% (1/5) | 109.5 / 98.5 |
| 1x16 | catalog | G2_PAIR_MERGE 42397 11 12 | 13.318 / 14.165 / 14.118 | +0.47% (2/5) | +7.42% (0/5) | 130.2 / 124.4 |
| 1x16 | 57 | G2_PAIR_MERGE 42397 11 12 | 13.141 / 14.105 / 13.987 | +0.57% (2/5) | +7.44% (0/5) | 123.4 / 131.7 |
| 2x2 | catalog | G2_PAIR_MERGE 27218 11 12 | 9.713 / 10.420 / 10.334 | -0.63% (4/5) | +6.65% (0/5) | 106.0 / 105.8 |

All five shapes entered the four-range path. Against its disabled control, the paired merge won four of five 2×2 blocks and modestly improved both 1×4 requests. On 1×16 it lost three of five blocks on both requests. The enabled executable was slower than the existing release in 44 of 45 pairs. The local results do not justify enabling this path by default. The 96-core screen includes the local release and same-binary controls because these configurations previously showed width-dependent behavior.

[Implementation](artifacts/source.patch) · [Dispatch checks](artifacts/routes.json) · [Measurements](artifacts/rows.json.gz) · [Native validation](artifacts/build.json) · [Proof verification](artifacts/verification.json)
