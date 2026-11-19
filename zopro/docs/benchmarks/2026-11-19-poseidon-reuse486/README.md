# Repeated Poseidon states within one proof

The diagnostic compares the complete field state after the first ordinary round of every recognized Poseidon span on the first proving call. A repeated state with the same permutation width determines identical remaining arithmetic. The counter also records whether a matching earlier span had finished when this call looked up its state. Each process probes one public request, releases the comparison table before continuing and exports a freshly randomized verified proof. No state or hash values are written to the report. The mutex and comparisons perturb scheduling, so the completion counts describe this instrumented run. They give no full-proof speed estimate.

| Shape | Request | Recognized permutations | Repeated states | Prior match completed |
|---|---:|---:|---:|---:|
| 1x2 | Original | 83 | 1 | 1 |
| 2x2 | Original | 150 | 68 | 22 |
| 4x2 | Original | 251 | 175 | 22 |
| 8x2 | Original | 484 | 407 | 84 |
| 12x2 | Original | 733 | 644 | 75 |
| 16x2 | Original | 1020 | 922 | 67 |
| 24x2 | Original | 1468 | 1347 | 465 |
| merge-24x1 | Original | 1424 | 1333 | 411 |
| 12x2 | 137 | 733 | 379 | 24 |
| 12x2 | 138 | 733 | 381 | 30 |
| 12x2 | 139 | 733 | 380 | 29 |
| 12x2 | 140 | 733 | 383 | 20 |
| 12x2 | 141 | 733 | 379 | 22 |
| 12x2 | 142 | 733 | 381 | 35 |
| 12x2 | 143 | 733 | 379 | 44 |
| 12x2 | 144 | 733 | 378 | 20 |
| merge-24x1 | 129 | 1424 | 1237 | 413 |
| merge-24x1 | 130 | 1424 | 1177 | 418 |
| merge-24x1 | 131 | 1424 | 1175 | 386 |
| merge-24x1 | 132 | 1424 | 1225 | 408 |
| merge-24x1 | 133 | 1424 | 1177 | 400 |
| merge-24x1 | 134 | 1424 | 1159 | 367 |
| merge-24x1 | 135 | 1424 | 1212 | 387 |
| merge-24x1 | 136 | 1424 | 1185 | 392 |

The eight scattered transfers and eight populated-tree merges use regenerated valid paths. A reuse implementation would also need to emit the intermediate witness and A/B/C row values for each duplicate. All 24 exports verified independently. [Counts by width and request](artifacts/counts.json) · [Diagnostic source](artifacts/source.patch) · [Verification](artifacts/verification.json)
