# Shard-local K scalar classification output on Ryzen 4×2

Shard-local output saved at most 12 µs in the parallel hash pass and added 58–66 µs for the scatter back into input order. The full K hash interval increased by 0.03–0.08 ms. Neither local-output width qualified on both inputs.

The [original shard classifier](../2026-11-20-stable-k-hash-ryzen964/README.md) writes representative indices into an atomic array in the original input order. The new layout gives each shard a disjoint, contiguous output range in partition order. After the workers join, the caller scatters the representatives into the existing class array, then assigns classes in first-occurrence order. The exact hash, scalar partition, class order and curve calls use the same logic. Both layouts use the same optional scratch bound, about 3.53 MiB for 97,075 K inputs.

The phase changes below compare local output with the width-matched atomic arm. They are paired medians in milliseconds over four blocks of thirteen measured proofs.

| Input | Shards | Partition | Setup | Parallel hashing | Scatter | Remap | Full hash interval |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 8 | −0.003 | +0.002 | −0.003 | +0.060 | +0.019 | +0.067 |
| catalog | 32 | −0.005 | +0.001 | −0.011 | +0.066 | +0.011 | +0.060 |
| heldout | 8 | −0.007 | ≈0 | −0.011 | +0.058 | −0.001 | +0.028 |
| heldout | 32 | +0.006 | ≈0 | ≈0 | +0.066 | +0.011 | +0.085 |

| Input | Frozen ms | Ordinary ms | Atomic 8 ms | Local 8 ms | Atomic 32 ms | Local 32 ms |
|---|---:|---:|---:|---:|---:|---:|
| catalog | 14.490 | 14.493 | 14.525 | 14.586 | 14.535 | 14.551 |
| heldout | 14.541 | 14.467 | 14.471 | 14.540 | 14.616 | 14.525 |

The unclocked proof runs use eight paired blocks of seventeen timed proofs with 24 main, eight frontend and four solver workers. Ordinary calibrated within 0.60% of frozen and clocked arms stayed within 1.10% of their unclocked comparisons. The native oracle and SDK route probes checked exact class metadata and ordinary fallback under zero experimental scratch. All 148 exported proofs independently verified. [C7a scorecard](../2026-11-20-curve-query-timing-c7a961/README.md#eight-supplied-m5-examples)

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
