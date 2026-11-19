# Grouping admission on the ordinary SDK path

The 1×2 reference constructed one dictionary for 21,006 scalars. A previous inspection of the same-sized A query found 17,454 distinct nonzero scalars against a retention cutoff of 16,804. That historical dictionary was discarded after construction. All larger reference shapes here constructed three dictionaries per proof. The earlier 16-core test of early exact rejection on the canonical path recorded a small, mixed latency effect. This trace identifies the work reached on the present SDK. It does not measure the latency saved by skipping it. [A-query census](../2026-11-18-a-group-census313/README.md) · [Prior 16-core comparison](../2026-11-18-canonical-group314/README.md)

The two release binaries had identical per-proof trace counts and admission decisions on each of nine original inputs. The input lengths are scalar counts before full grouping and can include zero values. A successful admission reaches the dictionary construction loop. Sampling can also stop a call before the trace is emitted.

| Shape | Zero-aware sample traces per proof | Admitted builds per proof | Input length × calls |
|---|---:|---:|---|
| 1x2 | 2 | 1 | 21,006 × 1 |
| 2x2 | 4 | 3 | 36,637 × 1, 52,274 × 1, 53,079 × 1 |
| 4x2 | 4 | 3 | 67,154 × 1, 95,537 × 1, 97,075 × 1 |
| 8x2 | 4 | 3 | 129,064 × 1, 183,347 × 1, 186,411 × 1 |
| 12x2 | 3 | 3 | 190,734 × 1, 270,757 × 1, 275,427 × 1 |
| 16x2 | 3 | 3 | 252,548 × 1, 358,343 × 1, 364,699 × 1 |
| 24x2 | 3 | 3 | 376,992 × 1, 534,619 × 1, 544,587 × 1 |
| merge-24x1 | 3 | 3 | 359,220 × 1, 507,895 × 1, 516,297 × 1 |
| merge-54x1 | 3 | 3 | 804,765 × 1, 1,137,190 × 1, 1,156,887 × 1 |

All 18 exports independently verified. [Protocol](protocol.md) · [Sampled diversity and admission counts](artifacts/traces.json) · [Verification](artifacts/verification.json)
