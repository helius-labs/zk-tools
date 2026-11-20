# Exact relations and shared scalar inputs in the 4×2 K and B2 queries

Combining distinct scalar classes using signs, small powers of two and small offsets saves at most **0.087%** of post-grouping digit arrivals in the relaxed operation model. The fixed disjoint pairing saves less than 0.046%. These relations are too sparse to justify a native curve implementation.

The same captures reveal a larger opportunity in scalar preparation. The solver image maps **95,535 of the 95,537 B2 positions directly to private K rows**. Every mapped scalar matches byte-for-byte on both requests. The remaining B2 positions are the two public wires. The exact nonzero coefficient multiset for B2 is contained in K on both requests, including those public values. That last property is observed for these inputs. A classifier using the private wire mapping would need to handle public rows explicitly.

The capture reads K and B2 before any mutation on the fourth proof of each process. Two fresh processes per input produced byte-identical snapshots. Grouped term counts, class counts and digit arrivals match the independent C7a census, including the indexed leading windows. All four exported proofs independently verified. [C7a query census](../2026-11-20-four-by-two-curve-work-c7a896/README.md)

| Input | Query | Post-group digit arrivals | Chosen pairs | Net modeled gain | Relaxed gain bound | Share of arrivals |
|---|---|---:|---:|---:|---:|---:|
| catalog | K | 650,785 | 9 | 176 | 330 | 0.051% |
| catalog | B2 | 693,601 | 9 | 195 | 344 | 0.050% |
| heldout | K | 655,478 | 14 | 292 | 560 | 0.085% |
| heldout | B2 | 698,582 | 14 | 318 | 603 | 0.086% |

The model matches a child coefficient to a parent coefficient multiplied by a signed power of two with exponent zero through three, plus an offset from −2 to +2. It credits the child's removed digits and charges the point merge, doublings and a separate correction. It assigns each of those operations equal cost. The relaxed count takes the best eligible incoming relation for every child, ignoring conflicts and required roots. This bound applies to that accounting model. It excludes search, data movement, scheduling and changes in first bucket assignments.

The Rust scanner verifies each field relation and reconstructs a weighted sum for each selected pairing. Two focused tests check the field identity and independently reconstructed digit counts. The experiment measures structural opportunity. It adds no runtime mechanism or latency result. An older shared A/B classification experiment covered a different small canonical path and regressed the timed 1×2 workload. [Prior shared-classification trial](../2026-10-29-shared-wire-sdk175/README.md)

The [recorded C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) remains five of eight at 10× M5 CPU and includes all eight Metal comparisons.

[Protocol](artifacts/protocol.json) · [Counts and wire mapping](artifacts/analysis.json) · [Rust scanner](artifacts/relations.rs) · [Capture source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
