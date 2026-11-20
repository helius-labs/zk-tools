# Running G1 A during quotient evaluation with scoped G2

The early-A route was 2.27% to 17.82% slower on paired medians across 1×2 and 2×2 on Ryzen. The ordinary scoped G2 schedule remains the baseline.

The candidate starts G1 A and the quotient together on the main Rayon pool. The auxiliary pool runs the exclusive G2 query. After A and the quotient join, the main pool handles the other G1 queries. The control and candidate share an executable, curve arithmetic and request fixtures. Optional scratch for the overlap is charged to the existing budget.

| Shape | Request | Main + G2 | Ordinary / early proof ms | Paired early vs ordinary | A/quotient overlap ms | Peak ordinary / early MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 8+8 | 8.441 / 8.966 | +6.17% (1/6) | 2.290 | 52.5 / 55.0 |
| 1×2 | catalog | 16+16 | 6.760 / 8.001 | +17.82% (0/6) | 1.586 | 58.0 / 57.2 |
| 1×2 | heldout | 8+8 | 8.148 / 8.556 | +4.99% (0/6) | 1.865 | 52.8 / 53.8 |
| 1×2 | heldout | 16+16 | 6.551 / 7.562 | +15.51% (0/6) | 2.248 | 56.5 / 57.5 |
| 2×2 | catalog | 8+8 | 12.492 / 12.760 | +2.27% (0/6) | 2.825 | 88.5 / 89.2 |
| 2×2 | catalog | 16+16 | 9.964 / 10.968 | +9.95% (0/6) | 2.442 | 92.5 / 91.5 |
| 2×2 | heldout | 8+8 | 12.423 / 12.831 | +3.18% (0/6) | 2.878 | 82.2 / 90.5 |
| 2×2 | heldout | 16+16 | 9.949 / 11.009 | +10.19% (0/6) | 2.677 | 89.0 / 92.5 |

Six blocks prime three proofs and time thirteen. Separate clocks record two timed proofs per request and placement, including forced zero-budget fallback. The 8+8 layout uses sixteen physical cores. The 16+16 layout includes sibling threads. All 120 exports passed independent verification.

[Summary](artifacts/summary.json) · [Samples and diagnostics](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz)
