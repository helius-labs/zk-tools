# Lossless local difference-basis screen

Both fixed adjacent query-basis layouts increase the declared point-work proxy in every one of 27 query vectors across nine fixtures. Primary pair2 ratios are 1.008629 for A and 1.003781 for B; binary8 is 1.017848 and 1.006869. No key-point rewrite, curve prototype or latency trial follows. Production is unchanged.

For a pair, (P,Q) becomes (P+Q,Q), while (a,b) becomes (a,b-a). The number of stored points stays fixed and the identity holds for arbitrary inputs. A binary block of eight applies left-point addition and right-scalar subtraction at each power-of-two level. It can expose nearby equal values as zeros without an additional table. Unlike the [global modal-origin screen](../2026-10-26-modal-scalar-census/README.md), it does not rely on a nonzero globally dominant value.

The screen counts distinct scalars as well as zeros, reproduces the accepted grouping sample/population policy under assumed successful reservation, and uses source-extracted exact unsigned digits, Booth digits and G2 phi splitting. Width/cache choices use the accepted 32-worker defaults: eligible exclusive G2 uses phi12, eligible deferred G1 uses signed12, and generic paths use their unsigned cache policy. Raw K remains diagnostic rather than accepted folded K/Euler H. This is query-level dispatch modeling, not execution of the full Key proof dispatcher.

The proxy adds modeled grouping-formation additions to nonzero digit contributions. Contributions are counted before free bucket seeding; weighted reduction, exceptional handling, grouping/translation time, preparation, reservation failure and scheduling are excluded. The proxy is neither exact total curve additions nor a latency prediction. Every tested candidate increases it even before charging transform costs.

| Shape | Query | Original nonzero classes | Pair2 classes | Binary8 classes | Pair2 proxy/original | Binary8 proxy/original |
| --- | --- | ---: | ---: | ---: | ---: | ---: |
| 1x2 | A_G1 | 17454 | 17789 | 18114 | 1.008629 | 1.017848 |
| 1x2 | B_G2 | 26142 | 27038 | 27554 | 1.003781 | 1.006869 |
| 1x2 | rawK_diagnostic | 26194 | 27136 | 27554 | 1.008729 | 1.015801 |
| 2x2 | A_G1 | 19268 | 19581 | 25764 | 1.015084 | 1.312007 |
| 2x2 | B_G2 | 28852 | 46863 | 47608 | 1.577669 | 1.601893 |
| 2x2 | rawK_diagnostic | 28910 | 46913 | 47335 | 1.575616 | 1.589308 |
| 4x2 | A_G1 | 20386 | 20730 | 33248 | 1.014557 | 1.551489 |
| 4x2 | B_G2 | 30544 | 49322 | 67938 | 1.543537 | 2.082240 |
| 4x2 | rawK_diagnostic | 30614 | 49377 | 68060 | 1.536675 | 2.055911 |
| 8x2 | A_G1 | 25236 | 25595 | 38508 | 1.011500 | 1.430046 |
| 8x2 | B_G2 | 37850 | 57400 | 93787 | 1.429483 | 2.227903 |
| 8x2 | rawK_diagnostic | 37940 | 57487 | 93844 | 1.422500 | 2.192096 |
| 12x2 | A_G1 | 30704 | 31235 | 44341 | 1.013624 | 1.350028 |
| 12x2 | B_G2 | 46062 | 66131 | 102560 | 1.350001 | 1.984516 |
| 12x2 | rawK_diagnostic | 46167 | 66234 | 102710 | 1.334383 | 1.953665 |
| 16x2 | A_G1 | 34326 | 34718 | 48450 | 1.008540 | 1.311822 |
| 16x2 | B_G2 | 51597 | 72922 | 110377 | 1.320634 | 1.882449 |
| 16x2 | rawK_diagnostic | 51727 | 73069 | 110477 | 1.305588 | 1.851727 |
| 24x2 | A_G1 | 43958 | 44387 | 59180 | 1.006884 | 1.252277 |
| 24x2 | B_G2 | 66221 | 89561 | 128549 | 1.263947 | 1.703861 |
| 24x2 | rawK_diagnostic | 66391 | 89770 | 128699 | 1.257671 | 1.686751 |
| merge-24x1 | A_G1 | 41272 | 56380 | 81765 | 1.265181 | 1.699892 |
| merge-24x1 | B_G2 | 62005 | 72660 | 93938 | 1.128379 | 1.383945 |
| merge-24x1 | rawK_diagnostic | 62259 | 84939 | 122861 | 1.257810 | 1.698665 |
| merge-54x1 | A_G1 | 75952 | 96014 | 124966 | 1.180103 | 1.440692 |
| merge-54x1 | B_G2 | 114985 | 125750 | 152663 | 1.066871 | 1.232815 |
| merge-54x1 | rawK_diagnostic | 115539 | 145825 | 188889 | 1.180743 | 1.437832 |

The main problem is lost duplicate grouping. For example, 2x2 B has 28,852 distinct nonzero scalar values; pair differences produce 46,863, and binary8 produces 47,608. Adjacent values rarely match, so extra differences usually add distinct values instead of zeros. This rejects these two fixed layouts on the measured fixtures, not arbitrary circuit-informed basis changes or other inputs.

## Validation

All 81 real variant vectors invert exactly back to original scalars and pass 162 independent weighted field functionals. Transformed point weights are independently reconstructed using subtree-interval prefix sums; the point-update algorithm is not executed by that oracle. Another 168 synthetic variant vectors cover 14 ragged lengths, zero/equal/opposite/random patterns and all three variants, with 168 inverse and 336 weighted field checks. These are scalar-algebra checks, not actual curve or proof tests. Nine raw/cached-image solver output comparisons agree, from 18 explicit scalar solves. Private scalars/witnesses/points/proofs are not exported.

An initial link failed because the copied phi helper retains pub(super), which cannot appear at crate root. Nesting the byte-identical helper text in a module and adding call wrappers repaired visibility before any census execution; arithmetic, protocol and models were unchanged. The failed source/receipt remain. All 205 original pins match with one exact initial-probe relocation, and all 207 corrected pins match. Extracted helper bodies match the pinned accepted files exactly.

No candidate point image, SDK proof, Gnark export, latency, minimum-RAM, physical-device or AWS result is claimed. Proposed same-count storage has no measured memory qualification. Accepted primary remains historically 8.594267 ms and merge54 178.891799 ms. [Accepted comparisons against every supplied M5 CPU and Metal reference](../2026-10-25-grouping-module/artifacts/perf/report.md) remain unchanged; all eight CPU 10× targets are unmet.

[Protocol](artifacts/experiment.json), [full counts and closing audit](artifacts/results.json), [driver](artifacts/probe.rs), [extracted native helpers](artifacts/native-helpers.rs), [link repair](artifacts/link-repair.json), [workflow](artifacts/plan-v2.json), [input pins](artifacts/input-pins-v2.json).
