# Exact proving phases for the three single-input C7a gaps

The exclusive G2 call was the last observed curve call to finish in 437 of 450 traced proofs. It took 8.06–9.74 ms across shape and request medians, with 0.85–0.93 ms left before SDK return. The plain Zopro runs trailed Aeglos by 2.16–3.52 ms. The measurements point to the exclusive G2 call and its start time as the next places to test changes.

The full-catalog Zopro profiles for 1×4, 1×8, and 1×16 ran with byte-identical traced copies and prepared Aeglos on one c7a.24xlarge. The 1×4 profile uses 96 main workers and an overlapping 16-core frontend. The 1×8 and 1×16 profiles use 48 main and 48 frontend cores with disjoint affinity. Both engines use the same raw key and request in each pair. Each process primes three proofs and times fifteen. Five paired blocks reverse arm order in consecutive blocks. The independent Gnark verifier checked all 90 exports and the temporary AWS resources were removed.

| Shape | Request | Plain Zopro / Aeglos ms | Paired vs Aeglos | Trace vs plain | Peak Zopro / Aeglos MiB |
|---|---|---:|---:|---:|---:|
| 1×4 | catalog | 12.031 / 9.703 | +24.02% (0/5) | +8.23% | 115.6 / 579.7 |
| 1×4 | heldout | 11.794 / 9.636 | +23.02% (0/5) | +11.25% | 115.8 / 583.4 |
| 1×8 | catalog | 13.225 / 10.436 | +24.46% (0/5) | +4.16% | 124.0 / 689.2 |
| 1×8 | heldout | 13.288 / 10.502 | +25.45% (0/5) | +2.45% | 133.0 / 687.3 |
| 1×16 | catalog | 15.652 / 12.130 | +29.04% (0/5) | +2.13% | 151.1 / 826.4 |
| 1×16 | heldout | 15.479 / 12.287 | +25.98% (0/5) | +3.18% | 160.7 / 822.3 |

Probe spans are medians of per-process means across 450 warm requests. G1 and G2 columns use unions of the observed evaluator intervals. The G2 start and time after G2 are measured from SDK entry and return, respectively. A dash marks a G2 evaluator without a covered interval. The probes can miss inlined or otherwise uninstrumented work. Observed phases may overlap, and tracing can change their timing.

| Shape | Request | Solve ms | Quotient ms | Observed G1 ms | Observed G2 ms | G2 start ms | Time after G2 ms | G1/G2 overlap ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 1×4 | catalog | 1.730 | 1.562 | 5.601 | 8.059 | 3.933 | 0.929 | 5.601 |
| 1×4 | heldout | 1.722 | 1.597 | 5.726 | 8.192 | 3.961 | 0.867 | 5.719 |
| 1×8 | catalog | 2.154 | 1.372 | 6.340 | 8.351 | 4.640 | 0.852 | 6.145 |
| 1×8 | heldout | 2.219 | 1.308 | 6.272 | 8.146 | 4.653 | 0.855 | 6.092 |
| 1×16 | catalog | 2.819 | 1.339 | 7.076 | 9.735 | 5.408 | 0.868 | 6.881 |
| 1×16 | heldout | 2.859 | 1.356 | 7.141 | 9.600 | 5.478 | 0.867 | 6.947 |

[Full-catalog profiles](../2026-11-20-all-shapes-c7a979/README.md) · [Phase and timing summary](artifacts/summary.json) · [Proof samples and spans](artifacts/rows.json.gz) · [Probe sites](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json)
