# Sharing equal-scalar group decisions between curve queries

The loader matches each A or folded K group to a B2 group only if every member refers to the identical witness wire in identical order. Each proof computes the B2 representative position and exact exception count once. Eligible A and K groups use the same decision; each query still applies every residual to its own curve points. Unmatched groups use their own scalar comparisons. The map, its temporary lookup and per-proof choices require optional memory reservations.

Five randomized blocks compare the earlier adaptive executable, the revised executable with independent choices, and the revised executable sharing choices. All three use one fixed group plan per shape, including the broad 12×2 plan on the scattered request. Each process primes three calls and times eleven full proofs. Separate probes compare all three queries’ residual counts and the set of fallback decisions between the revised modes. An unsuccessful shared B2 precheck can generate a second identical skip event. Peak RSS covers the whole process.

| Shape | Request | Prior / independent / shared ms | Shared vs independent | Shared vs prior | Matching A · K groups | Peak MiB independent / shared |
|---|---|---:|---:|---:|---:|---:|
| 12x2 | Original | 29.218 / 29.501 / 29.088 | -1.70% (3/5) | -0.88% (4/5) | 12289/12481 · 18470/18471 | 522.8 / 529.3 |
| 12x2 | 59 | 29.259 / 29.196 / 29.392 | +0.67% (2/5) | -0.02% (3/5) | 12289/12481 · 18470/18471 | 528.9 / 525.4 |
| 12x2 | 141 | 47.033 / 46.757 / 46.820 | +0.20% (2/5) | -0.52% (3/5) | 12289/12481 · 18470/18471 | 973.6 / 988.0 |
| 16x2 | Original | 37.209 / 37.532 / 39.716 | +5.95% (0/5) | +7.10% (0/5) | 12151/12151 · 18228/18228 | 761.8 / 734.8 |
| 16x2 | 74 | 36.611 / 37.191 / 40.229 | +7.56% (0/5) | +8.10% (0/5) | 12151/12151 · 18228/18228 | 762.3 / 757.6 |
| 24x2 | Original | 59.496 / 59.325 / 64.478 | +9.86% (0/5) | +8.50% (0/5) | 12970/12970 · 19457/19457 | 1102.1 / 1092.0 |
| 24x2 | 99 | 58.457 / 58.979 / 63.652 | +7.93% (0/5) | +8.40% (0/5) | 12970/12970 · 19457/19457 | 1117.4 / 1093.2 |

All 119 exported proofs verified independently. [Samples](artifacts/processes.json.gz) · [Probes](artifacts/probes.json) · [Source](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
