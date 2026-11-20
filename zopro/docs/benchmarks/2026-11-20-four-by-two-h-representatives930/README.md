# Equivalent scalar representations in the 4×2 quotient MSM

The best recoding in the tested radix-4096 family can remove at most **1.76%** of modeled bucket-accumulation additions on either archived request. The simpler centered representation removes **1.59%** and cuts the number of leading-window arrivals from about **65,000 to 33,000**, using the existing point list and bucket count.

Every candidate represents the original scalar modulo the BN254 group order. The centered mode uses the smaller of the canonical magnitude and its negative representative. The unsigned modes allow every positive or negative lift whose magnitude fits 254 or 256 bits, and choose the fewest nonzero digits. The signed mode also chooses where to carry in the 21 full windows. Each digit's absolute value remains at most 4,095, and the short leading digit stays within 0–15 before applying the representative's sign. No extra point multiples are needed. The LFG construction is related background for changing bucket digit sets and uses precomputed point multiples. [Luo, Practical Optimal Bucket Sets](https://eprint.iacr.asia/2026/2750)

The model counts each nonzero digit as one arrival and gives the first arrival in an occupied absolute-digit bucket a free assignment. It counts accumulation after merging the two original point ranges. For the optimistic limit, it gives away every possible free seed in every window and ignores recoding, sign handling, input grouping, weighted reduction and scheduling. This bound applies to the stated radix and digit limits. It does not estimate proof latency.

| Input | Mode | Digit arrivals | Leading arrivals | Occupied buckets | Modeled bucket adds | Cut |
|---|---|---:|---:|---:|---:|---:|
| catalog | ordinary | 2,105,816 | 65,087 | 85,998 | 2,019,818 | 0.00% |
| catalog | centered | 2,073,616 | 32,836 | 85,996 | 1,987,620 | 1.59% |
| catalog | unsigned254 | 2,073,437 | 33,014 | 85,998 | 1,987,439 | 1.60% |
| catalog | unsigned256 | 2,072,149 | 33,660 | 86,010 | 1,986,139 | 1.67% |
| catalog | signed256 | 2,070,384 | 34,366 | 85,991 | 1,984,393 | 1.75% |
| heldout | ordinary | 2,107,232 | 65,037 | 85,998 | 2,021,234 | 0.00% |
| heldout | centered | 2,075,081 | 32,869 | 85,996 | 1,989,085 | 1.59% |
| heldout | unsigned254 | 2,074,885 | 33,022 | 85,998 | 1,988,887 | 1.60% |
| heldout | unsigned256 | 2,073,571 | 33,636 | 86,010 | 1,987,561 | 1.67% |
| heldout | signed256 | 2,071,818 | 34,375 | 85,993 | 1,985,825 | 1.75% |

Every candidate reconstructs its chosen integer and the original field scalar. Two Rust tests check arithmetic boundaries and exhaustive three-column selections. A separately written forward recurrence agrees on 508 sampled scalar minima. All four archived proof exports were reverified with Gnark against the original key and public inputs. Two byte-identical captures per request give the same results. The ordinary counts reconcile with the earlier independent C7a work census. [H capture](../2026-11-20-four-by-two-h-reuse-ryzen915/README.md) · [C7a work census](../2026-11-20-four-by-two-curve-work-c7a896/README.md)

The inexpensive centered case is of interest because of the shorter leading window. The broader recoding search adds little to that case before its own costs. This report adds no proof timing. The verified C7a score remains **5/8 at 10× M5 CPU**. [CPU and Metal scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings)

[Model](artifacts/screen.rs) · [Counts and proof bindings](artifacts/analysis.json) · [Independent checks](artifacts/independent.json)
