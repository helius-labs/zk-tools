# Fixed radix-3072 G2 work screen

The algebra screen passes. The fixed representation reduces modeled generic point joins by **0.5604% on 1×2** and **0.5441% on 2×2**. This screen does not select a native implementation: most of the weighted-tree saving is offset by more fill joins, while extraction and a larger top window add costs not resolved here. There is no measured speedup or slowdown. Production remains unchanged.

The representation uses integer radix 3072 = 3×1024 for both signed Phi components, eleven windows, two existing point partitions and 1536 magnitude buckets per window. Ordinary bucket capacity falls 25%, but total window capacity falls only 17.76% because the original top has 64 buckets. This is a nonbinary integer recoding, separate from the earlier width-13 and hexagonal-ring studies; their negative results remain closed. No sweep or revised gate was run.

## Counts

The unchanged zero-aware grouping policy declines 1×2 (30,089 rows) and admits 2×2 (52,274 → 28,852 rows). All original fixture points are nonidentity; grouped-point cancellation is not modeled. Counts assume generic noncancelling sums. Weighted joins follow the production 256-bucket subtrees, including copied even-index lists and structural identity skipping.

| Shape | Quantity | Accepted representation | Radix3072 | Difference |
|---|---|---:|---:|---:|
| 1x2 | Nonzero arrivals | 613891 | 614359 | +468 |
| 1x2 | Fill joins | 572819 | 581663 | +8844 |
| 1x2 | Partition merge joins | 20536 | 16348 | -4188 |
| 1x2 | Weighted-tree joins | 40265 | 32058 | -8207 |
| 1x2 | Total modeled joins | 633620 | 630069 | -3551 |
| 1x2 | Affine bucket-transform Fq products | 82044 | 65380 | -16664 |
| 2x2 | Nonzero arrivals | 633695 | 634159 | +464 |
| 2x2 | Fill joins | 592623 | 601463 | +8840 |
| 2x2 | Partition merge joins | 20536 | 16348 | -4188 |
| 2x2 | Weighted-tree joins | 40265 | 32058 | -8207 |
| 2x2 | Total modeled joins | 653424 | 649869 | -3555 |
| 2x2 | Affine bucket-transform Fq products | 82072 | 65384 | -16688 |

For 1×2, the ordinary windows save 4,948 modeled joins while the top adds 1,397, leaving 3,551 fewer overall. For 2×2, the net saving is 3,555 joins. Fewer occupied buckets mean fewer free first insertions: fill joins increase even though weighted reductions become smaller. Endomorphism-transform counts charge two Fq products per occupied high-component affine bucket; exceptional projective transforms and cancellations are excluded. These products and G2 joins are different units and are not combined into a runtime estimate.

Ten radix3072 Horner transitions replace ten doublings with ten additional projective additions. Those operations are reported separately from affine joins. Recoding divisions, scalar splitting, inversions, queue fallback, batch topology, normalization, scheduling, allocation, grouping, cache behavior and memory traffic are excluded. The model does not establish an upper bound on possible latency gains or a universal algorithmic limit.

## Correctness and memory

Both immutable fixture hashes and all 176 accepted production-source hashes match. For every modeled scalar, the existing signed Phi split reconstructs the Fr scalar; both binary and radix3072 digits reconstruct each component exactly as signed integers. A separately written Euclidean-division recoder also reconstructs each value. Boundary and deterministic coverage checks 8,323 additional signed integers. Weighted-tree checks exhaust identity masks through eight leaves and check the dense formula through 256 leaves. The math checks are in one model implementation; there is no independently reparsed curve or timing experiment.

One Rust compile, preparation, and one source-pinned xtask model command succeed on their first executions. The model command takes about 59 ms including its checks; this is workflow duration, not a recoding benchmark. Existing field/core/native/oracle/SDK/proof qualifications are inherited and were not rerun. No point/scalar fixture, key, witness, binary or library payload is published.

A literal eleven-i16-per-component codec requires 44 bytes per row versus the existing exclusive 32-byte scalar storage: 361,068 additional payload bytes on 1×2 and 346,224 on grouped 2×2. These are layout calculations, not allocated runtime peaks. A no-expansion implementation needs a separate on-demand extraction or compression design. Reduced bucket capacity does not prove lower overall RSS or minimum RAM. No native layout, pressure, concurrency, portable/mobile or large-circuit qualification is claimed.

The strictly-positive modeled-join opportunity check passes on both shapes; there is no minimum percentage gate. Choosing not to start a native campaign is an engineering decision given the modest structural saving, changed top path and unqualified codec. This closes this fixed proposal at the model stage, not all nonbinary MSM algorithms.

## Accepted proof baseline, unchanged

These accepted local 32-worker proof medians come from the [previous complete-proof campaign](../2026-11-06-window-batch-proof-capture176/README.md). Supplied M5 references were not remeasured. Ratios compare different machines. No new proof latency or AWS/Aeglos/Gnark comparison is added.

| Shape | Accepted CPU ms | M5 CPU ms | M5 Metal ms | M5 CPU / accepted | M5 Metal / accepted | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 7.915095 | 37 | 24 | 4.675× | 3.032× | 3.7 |
| 2×2 | 11.169812 | 59 | 30 | 5.282× | 2.686× | 5.9 |
| 4×2 | 16.085882 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8×2 | 24.517520 | 189 | 74 | 7.709× | 3.018× | 18.9 |
| 12×2 | 33.838626 | 284 | 101 | 8.393× | 2.985× | 28.4 |
| 16×2 | 40.083672 | 335 | 124 | 8.358× | 3.094× | 33.5 |
| 24×2 | 65.506198 | 528 | 176 | 8.060× | 2.687× | 52.8 |
| merge24×1 | 65.200982 | 507 | 175 | 7.776× | 2.684× | 50.7 |

Merge54×1 remains 161.718956 ms, without a supplied M5 reference. All eight CPU10× requirements remain active and unmet (0/8). The canonical proof ledger remains 31,387.

[Design](artifacts/design.md), [model](artifacts/model.rs), [reused split/Booth source](artifacts/split.rs), [aggregate results](artifacts/analysis.json), [input/source hashes](artifacts/pins.json), [workflow](artifacts/plan.json), [command receipt](artifacts/command.json), [exit receipt](artifacts/exit.json) and [checksums](SHA256SUMS) preserve the screen.
