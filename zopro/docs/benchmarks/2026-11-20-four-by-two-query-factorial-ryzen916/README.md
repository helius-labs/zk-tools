# Which curve queries limit 4×2 proof latency on Ryzen

Replaying the exact results of K and B2 together reduced median paired proof time by **3.99 / 3.95 ms** on the two inputs. Replaying B2 alone saved **2.19 / 2.18 ms**, while replaying K alone saved **0.69 / 0.63 ms**. The joint effect exceeded the sum of the individual effects by **1.13 / 1.16 ms**, with a positive excess in all eight blocks on both inputs. Every single-query replay remained above 10.9 ms in every block. Joint K and B2 replay fell below 10.9 ms in every block, with A and H still computed normally.

| Input | All active ms | K replay saving ms | B2 replay saving ms | Joint saving ms | Joint excess ms | A and H active ms | All replayed ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 13.698 | +0.690 | +2.185 | +3.988 | +1.127 | 9.737 | 4.474 |
| heldout | 13.674 | +0.627 | +2.177 | +3.950 | +1.165 | 9.699 | 4.426 |

Each process warms three proofs, then measures thirteen proofs in one of eight counterbalanced blocks per input. Every subset of the four curve queries is tested. The replay checks the base identity and every query coefficient against the first primed request. The witness solver, quotient FFT, final proof construction, fresh blinding and SDK verifier still run. The timed panel has tracing disabled. Every exported proof also passed independent Gnark verification, for 384 verified exports.

The masks use 24 main and eight frontend hardware threads on the same 16-core Ryzen. Each frontend hardware thread shares a physical core with one main thread. The original seeded executable, fresh build with replay disabled, and fresh build with all queries active provide the controls. Fresh replay-disabled versus original was −0.33% / +0.28% in paired medians. The all-active mask versus fresh replay-disabled was +1.30% / +0.23%. All four calibrations passed the 2% threshold.

In a separate three-block clocked panel, replaying B2 while A, K and H remained active reduced K's elapsed span from 8.48 / 8.15 ms to 6.69 / 6.63 ms. K then outlasted the other queries by 1.24 / 1.25 ms. Replaying both K and B2 left an A/H span of about 4.98 ms. Every traced replay matched the expected query and input. The clocked spans include time spent waiting and running nested jobs. The experiment does not isolate how much of the interaction comes from worker availability, SMT contention or memory traffic.

## All query combinations

Values are medians of process means. Saved time is the median within-block difference from the all-active mask.

| Queries computed | Catalog ms | Held-out ms | Catalog saved ms | Held-out saved ms |
|---|---:|---:|---:|---:|
| None | 4.474 | 4.426 | +9.289 | +9.196 |
| A | 6.163 | 6.261 | +7.535 | +7.432 |
| K | 8.741 | 8.735 | +4.918 | +4.984 |
| A, K | 9.476 | 9.490 | +4.211 | +4.248 |
| H | 8.879 | 8.902 | +4.835 | +4.824 |
| A, H | 9.737 | 9.699 | +3.988 | +3.950 |
| K, H | 11.354 | 11.419 | +2.311 | +2.299 |
| A, K, H | 11.486 | 11.486 | +2.185 | +2.177 |
| B2 | 9.362 | 9.611 | +4.320 | +4.075 |
| A, B2 | 10.092 | 10.163 | +3.628 | +3.526 |
| K, B2 | 11.171 | 11.185 | +2.514 | +2.490 |
| A, K, B2 | 11.763 | 11.670 | +1.993 | +2.027 |
| H, B2 | 12.114 | 12.078 | +1.609 | +1.656 |
| A, H, B2 | 13.049 | 13.048 | +0.690 | +0.627 |
| K, H, B2 | 12.803 | 12.772 | +0.892 | +0.907 |
| A, K, H, B2 | 13.698 | 13.674 | +0.000 | +0.000 |

Exact-result replay is a diagnostic. Its timing depends on the cost of checking a stored result and the work left on the two pools. The experiment establishes a joint opportunity on this Ryzen layout, with no measured production speedup or C7a retest. The [C7a replay study](../2026-11-20-four-by-two-phase-c7a891/README.md) measured a different core layout and did not include joint K and B2 replay.

The all-active catalog run is 7.96× as fast as the supplied 109 ms M5 CPU reference and 3.50× as fast as the 48 ms Metal reference. The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) contains CPU and Metal comparisons for all eight supplied M5 references. Five reach the 10× M5 CPU target. AWS SSO expired before this run, leaving the pending 2×2 confirmation unrun.

[Protocol](artifacts/protocol.json) · [Samples and clocks](artifacts/rows.json.gz) · [Aggregates](artifacts/summary.json) · [Proof verification](artifacts/verification.json)
