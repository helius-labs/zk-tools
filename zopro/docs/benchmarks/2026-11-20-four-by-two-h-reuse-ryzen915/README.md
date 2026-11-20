# Reusing point sums across 4×2 quotient MSM windows

Sharing point sums across the existing two H point ranges removes **3.38%** of modeled bucket-accumulation additions on each of two requests, after charging every new point sum. Including all points in one hypothetical range raises the saving to **4.45–4.47%**. Neither schedule meets the predeclared 10% work threshold for a native prototype.

The observer captured the exact coefficients entering the unsigned width-12 H query. Each request produced byte-identical captures in two independent processes. Each capture has 98,674 nonidentity bases. The full-column arrivals, first bucket assignments and short-column arrivals agree exactly with the [C7a query census](../2026-11-20-four-by-two-curve-work-c7a896/README.md) on both requests.

For each pair of digit columns in fixed order, the scanner groups currently available rows by their two equal nonzero digits and pairs consecutive rows within each group. It replaces the matching terms with one reusable point sum in every column the two rows agree on. The two original rows retain their other digits. A later generation can pair residual rows and previously formed sums. The scanner reverses that graph after every generation and recovers every original scalar digit exactly. It also checks that every bucket retains its original occupancy.

| Input | Point ranges | Columns used to select pairs | New point sums | Net additions removed | Share removed |
|---|---:|---|---:|---:|---:|
| Catalog | 2 | Full width | 26,500 | 31,613 | 1.57% |
| Held-out | 2 | Full width | 26,535 | 31,662 | 1.57% |
| Catalog | 2 | Full and leading | 64,035 | 68,222 | 3.38% |
| Held-out | 2 | Full and leading | 64,038 | 68,316 | 3.38% |
| Catalog | 1 | Full and leading | 83,515 | 89,935 | 4.45% |
| Held-out | 1 | Full and leading | 83,920 | 90,389 | 4.47% |

The denominator is 2,019,818 / 2,021,234 generic additions to form all occupied buckets, counting range merges through the final bucket occupancy. Weighted bucket reduction is outside this count. For one generation of disjoint pairs within two ranges, the optimistic allowance is 62,064 / 62,029 additions. Any pair matching in \(k\) full columns saves at most \(k-1\) additions there after formation and accounts for \(k(k-1)/2\) column-pair tickets. A matching leading column can save one more addition for each eligible pair. The allowance covers disjoint pairs and the first generation only.

The model assigns equal cost to group additions. It excludes finding matches, routing residual terms, point lifetimes and scheduling. Keeping all the new affine points for the modeled two-range schedule would account for about 3.91 MiB of point payload per proof before routing data. The experiment adds no latency measurement or runtime reuse path.

Two focused scanner tests passed, including an exhaustive check of all 4,096 small support matrices against every pairing. The first observer attempt captured the shared unsigned query entry without identifying H. Its width-11 output was rejected by the scanner. The corrected observer explicitly scopes capture to the H query. All eight exported proofs independently verified, and four scoped captures contribute to the reuse counts.

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) provides CPU and Metal comparisons for all eight supplied M5 references. Five meet the 10× M5 CPU goal.

[Protocol](artifacts/protocol.json) · [Counts](artifacts/analysis.json) · [Rust scanner](artifacts/h_reuse.rs) · [Capture source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
