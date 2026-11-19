# Scalar distributions across six public 1×2 transactions

Six public transactions used the same key, public coefficient file and prover configuration on sixteen physical Ryzen cores. Each made four complete requests. Each row gives the counts observed at the canonical MSM entry or the exclusive G2 entry in every request. Small counts nonzero canonical integers below 2⁶⁴. The last column gives the number of classes returned by the existing optional grouping policy, or zero when it declined. The diagnostic hash map exists only in this test binary. No scalar values were exported. All six proof exports passed independent verification.

| Transaction | Query | Scalars | Zeros | Small | Distinct nonzero | Singletons | Largest class | Grouped classes |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | G2Exclusive | 30,089 | 2,768 | 385 | 25,816 | 25,259 | 382 | 0 |
| catalog | G1Config | 21,006 | 2,551 | 489 | 17,234 | 16,862 | 484 | 0 |
| heldout | G2Exclusive | 30,089 | 2,585 | 504 | 26,650 | 26,368 | 491 | 0 |
| heldout | G1Config | 21,006 | 2,371 | 609 | 17,800 | 17,610 | 593 | 0 |
| v25 | G2Exclusive | 30,089 | 2,709 | 408 | 26,637 | 26,355 | 395 | 0 |
| v25 | G1Config | 21,006 | 2,493 | 512 | 17,791 | 17,602 | 497 | 0 |
| v26 | G2Exclusive | 30,089 | 2,650 | 447 | 26,645 | 26,363 | 433 | 0 |
| v26 | G1Config | 21,006 | 2,436 | 555 | 17,795 | 17,605 | 535 | 0 |
| v27 | G2Exclusive | 30,089 | 2,599 | 493 | 26,650 | 26,368 | 480 | 0 |
| v27 | G1Config | 21,006 | 2,385 | 598 | 17,800 | 17,610 | 582 | 0 |
| v28 | G2Exclusive | 30,089 | 2,694 | 414 | 26,634 | 26,347 | 401 | 0 |
| v28 | G1Config | 21,006 | 2,483 | 516 | 17,789 | 17,598 | 501 | 0 |

[Counts](artifacts/counts.json) · [Instrumentation](artifacts/source.patch) · [AWS request timings](../2026-11-19-varied-phases716/README.md) · [Verification](artifacts/verification.json)
