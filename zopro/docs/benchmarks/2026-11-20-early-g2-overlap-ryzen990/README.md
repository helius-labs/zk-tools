# Overlapping the exclusive G2 call with the quotient on Ryzen

Moving G2 before the quotient created 1.02–1.12 ms of overlap. Warm proof latency still increased by 3.85–5.12% versus the same-build late start on 1×8 and 1×16. This version leaves a blocking receive in the main curve join. A separate variant will test moving the join after the main pool finishes G1.

The [exact C7a traces](../2026-11-20-exact-one-input-phase-c7a986/README.md) place the ordinary B2 start after the quotient. This experiment starts the same B2 arithmetic on the disjoint frontend pool after witness preparation. The key remains borrowed and the optional scratch goes through its existing account. The local placement uses eight distinct physical cores per pool and excludes SMT siblings. Both inputs of each key preserve the original public-wire corrections.

| Shape | Request | Pinned / late / early proof ms | Early paired vs late | G2 / quotient / overlap ms | Peak late / early MiB |
|---|---|---:|---:|---:|---:|
| 1×8 | catalog | 12.040 / 12.106 / 12.705 | +5.12% (0/8) | 6.348 / 1.023 / 1.018 | 102.5 / 104.8 |
| 1×8 | heldout | 12.115 / 12.205 / 12.791 | +4.89% (2/8) | 6.354 / 1.025 / 1.021 | 102.2 / 103.8 |
| 1×16 | catalog | 15.200 / 15.311 / 16.018 | +3.85% (2/8) | 7.801 / 1.126 / 1.121 | 135.5 / 119.2 |
| 1×16 | heldout | 15.327 / 15.319 / 16.061 | +4.06% (0/8) | 7.924 / 1.125 / 1.121 | 122.6 / 121.9 |

Eight timing blocks prime three proofs and time seventeen. The separate phase panel records 39 requests per key and input, with overlap on all of them. Scratch denial returned to the ordinary schedule and all 116 process exports passed independent verification.

[Samples](artifacts/rows.json.gz) · [Stage data](artifacts/summary.json) · [Exact placement](artifacts/protocol.json) · [Source](artifacts/source.patch.gz)
