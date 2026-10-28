# Small-factor scalar recoding work screen

The exact BN254 screen does not support applying factor-of-two recoding universally. At width 12 it reduces nonzero digit terms by 0.74% on uniform full-width scalars, but increases them 6.27× on the small-scalar set. Simple signed centering gives a similar full-width reduction while preserving this small-scalar set. No native prover implementation or proving-time gain is claimed.

[Oleksandr Hrabar's Waterloo thesis](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403), chapter 4, describes making each scalar divisible by two using either the scalar or the subgroup-order complement, tracking a point sign, and doubling the final MSM. It also studies separate factor groups for 3/2 and 5/3/2. We read the selected chapter and related sections; this is an independent BN254 screen, not a reproduction of its BLS12-381 benchmark. The locally downloaded PDF hash is recorded; the PDF is not redistributed.

The native overlapping Booth digit function is copied verbatim and checked against the accepted source. For all widths 4..15, `ceil(255/width) == ceil(254/width)`: shortening these scalar magnitudes by one bit removes no complete window. This does not rule out smaller savings in the remaining windows.

Three exact tests passed: modular recoding against independent arbitrary-precision integers and Arkworks Fr; native Booth digits against direct bit extraction and integer reconstruction at every width; and separate factor-group MSMs against an independent Arkworks G1 MSM, including identity bases. The scalar fixture contains 5,116 values: small integers, near-order values, powers of two and predecessors, and 4,096 deterministic full-width samples. The independent group test uses 139 selected values. No SDK request or new proof was needed.

The fixed census covers 32,768 uniform rejection-sampled canonical scalars and all integers 0..16,384 exclusive. Its 120 rows cover five strategies and twelve widths. These are synthetic work-count inputs, not measured proof-witness distributions.

| Input set | Strategy | Nonzero terms | Terms / original | Ideal insertion additions | Group windows | Sum of maximum bucket indices |
|---|---|---:|---:|---:|---:|---:|
| uniform_full_width | canonical | 715288 | 1.000000 | 672277 | 22 | 43011 |
| uniform_full_width | centered | 709874 | 0.992431 | 666864 | 22 | 43010 |
| uniform_full_width | factor2 | 709971 | 0.992567 | 666961 | 22 | 43010 |
| uniform_full_width | priority3_2 | 706257 | 0.987374 | 620445 | 44 | 86016 |
| uniform_full_width | priority5_3_2 | 701352 | 0.980517 | 574114 | 66 | 129019 |
| small_0_through_16383 | canonical | 30716 | 1.000000 | 28664 | 22 | 2052 |
| small_0_through_16383 | centered | 30716 | 1.000000 | 28664 | 22 | 2052 |
| small_0_through_16383 | factor2 | 192507 | 6.267320 | 190437 | 22 | 19676 |
| small_0_through_16383 | priority3_2 | 191120 | 6.222164 | 186981 | 44 | 40917 |
| small_0_through_16383 | priority5_3_2 | 191278 | 6.227308 | 185478 | 66 | 63371 |

Ideal insertion additions count terms minus distinct occupied absolute-digit destinations. They omit weighted bucket reduction. Splitting into factor groups can reduce that number by introducing more free first assignments while adding bucket tables, weighted reductions and final correction work. The separate group/window and maximum-index counts make that tradeoff visible; none is a native runtime estimate. Actual signed bucket cancellations, point exceptions, affine batching, top-window trees, grouping, scheduling and transform costs are not modeled.

At width 15, centering reduces full-width terms by only five but increases ideal insertion additions from 318,837 to 324,313 because occupancy changes. Consequently even the centered variant has no universal work-improvement claim. Factor recoding's large expansion of small odd scalars also prevents a universal adoption decision based on random full-width counts.

The next useful step is an actual-scalar census plus a history audit for signed centering, followed by native integration only if the real workload supports it. Integration must charge any sign metadata and preserve the signed top-window, deferred-Y and fused paths. Existing 127-bit G2 Frobenius components cannot independently use a whole-Fr subgroup-order recoding. This screen establishes no explanation or resolution of the C7a/Aeglos latency gap.

All 172 accepted production files and the accepted SDK hash remain exact. No lookup table, retained key plan, production allocation or source change was introduced. No CPU latency, peak RSS, minimum-RAM, portable/mobile/TEE, pressure, concurrency or AWS measurement was run. The independent proof ledger remains 12,580. The global CPU 10× target remains active and unmet for all eight shapes.

The following accepted local observations are inherited from the [FFT adoption report](../2026-10-27-clean-fft-adoption/README.md), compared with the supplied M5 references. They are not new measurements from this screen. That adoption retains its documented engineering exception: the full performance and qualified-scope gates remain false.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU ratio | Metal ratio |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37 | 24 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59 | 30 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109 | 48 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189 | 74 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284 | 101 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335 | 124 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528 | 176 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507 | 175 | 7.790× | 2.689× |

The accepted primary peak RSS observation is 134.50 MiB. Merge 54→1 remains 161.341835 ms with the historical 1,910.1875 MiB peak RSS observation; no supplied M5 reference exists for that shape. These are not minimum RAM requirements.

Artifacts include the declared design, exact model and native digit source, build/test logs, full census, source audit and summary. The reproduction script uses the pinned cached Rust dependencies; their hashes and compiler identity are recorded. No successful test or model run was repeated.
