# Predicting a scalar class from the last observed repeat distance

The ordinary proof-local grouping loop hashes every nonzero scalar and records its first-occurrence class. The alternative learns the distance to the previous dictionary encounter for that scalar. Before the next lookup, it tries the learned distance and compares all four limbs with the earlier scalar. A match reuses its assigned class, while a mismatch performs the ordinary lookup. One policy has a single distance for the stream. The other has eight distances selected by row index modulo eight. Both start empty for every request. A successful prediction leaves the dictionary untouched. The last dictionary row and the class each fit in a u32.

The Rust scanner compares the complete sequence of output classes to a separate ordinary implementation on each captured vector, including the sentinel for zero. A periodic synthetic stream with interruptions tests that the shortcut can activate. Counts describe the nonzero-row hashing loop before considering admission, offsets in a different configuration, comparison cost, cache traffic or subsequent curve work. The scalar captures and verified proof identities are preserved below.

| Shape | Request IDs spanned | Distance slots | Fraction of nonzero lookups avoided, median (range) | Median exact matches per attempted comparison |
|---|---:|---:|---:|---:|
| 1x2 | 1–8 | 1 | 4.62% (4.51–4.71%) | 4.62% |
| 1x2 | 1–8 | 8 | 4.39% (4.31–4.53%) | 4.39% |
| 2x2 | 33–40 | 1 | 39.60% (39.53–39.67%) | 39.61% |
| 2x2 | 33–40 | 8 | 39.54% (39.43–39.59%) | 39.57% |
| 12x2 | 0–144 | 1 | 64.00% (47.23–80.47%) | 64.01% |
| 12x2 | 0–144 | 8 | 64.11% (47.31–80.61%) | 64.12% |
| 16x2 | 65–72 | 1 | 82.59% (82.57–82.62%) | 82.60% |
| 16x2 | 65–72 | 8 | 82.78% (82.75–82.80%) | 82.79% |
| merge-24x1 | 105–112 | 1 | 85.13% (81.78–89.69%) | 85.13% |
| merge-24x1 | 105–112 | 8 | 85.26% (81.94–89.85%) | 85.27% |
| merge-24x1 | 121–128 | 1 | 81.55% (77.39–86.18%) | 81.56% |
| merge-24x1 | 121–128 | 8 | 81.68% (77.53–86.28%) | 81.70% |

All 48 archived process exports verified independently again. [Exact per-request counts](artifacts/counts.json) · [Rust scanner](artifacts/scan.rs)
