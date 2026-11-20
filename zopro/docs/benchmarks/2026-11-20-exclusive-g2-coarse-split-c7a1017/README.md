# Chunked exclusive G2 scalar decomposition on C7a

No chunked scalar-decomposition profile qualified on both requests. On 1×16, the 512-scalar chunks cut the split from 1.26–1.38 ms to 0.38–0.40 ms in diagnostics. Paired proof latency rose by about 7.1% on both inputs.

Zopro has paired leads on 39 of 41 shapes across matched shape-specific C7a runs.

The experiment splits the existing scalar buffer in place on the exclusive G2 pool. Each task applies the original scalar transformation to a contiguous chunk. The point queries and the workspace reservations for two or four window ranges follow the existing path. The experiment uses the 1016 main+G2 placement for 1×4, 1×16 and 2×2, and 48+48 for 1×2 with the independent pinned 1×2 profile on the same host.

| Shape | Input | Selected | Full proof ms | Aeglos ms | Paired vs exact SDK | Paired vs Aeglos | Peak Zopro / Aeglos MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | Exact SDK | 8.041 | 8.118 | +0.00% (0/8) | -0.24% (4/8) | 51.9 / 464.3 |
| 1×2 | heldout | Exact SDK | 7.781 | 8.264 | +0.00% (0/8) | -5.69% (8/8) | 54.2 / 464.2 |
| 1×4 | catalog | Exact SDK | 9.794 | 9.667 | +0.00% (0/8) | +1.31% (2/8) | 101.4 / 579.8 |
| 1×4 | heldout | Exact SDK | 9.764 | 9.706 | +0.00% (0/8) | +0.88% (3/8) | 99.2 / 584.8 |
| 1×16 | catalog | Exact SDK | 12.664 | 12.130 | +0.00% (0/8) | +4.72% (1/8) | 164.5 / 827.9 |
| 1×16 | heldout | Exact SDK | 12.695 | 12.334 | +0.00% (0/8) | +2.90% (0/8) | 168.2 / 824.4 |
| 2×2 | catalog | Exact SDK | 10.621 | 11.559 | +0.00% (0/8) | -8.23% (8/8) | 75.2 / 804.3 |
| 2×2 | heldout | Exact SDK | 10.655 | 11.750 | +0.00% (0/8) | -9.28% (8/8) | 76.0 / 809.5 |

To qualify, a chunk size must record a lower paired median and win at least six of eight blocks on each input against both the same-build trial control and the exact SDK. The 1×2 profile must also beat the pinned profile at the same threshold. The table presents the exact SDK for shapes without a qualifying mode.

## Phase timings

The Zopro columns come from separate ELF probes. They report the witness solver call and the completion order of the observed curve calls. The Aeglos columns are its native stage counters in the timing panel. The libraries assign setup and overlap differently. Probes increased Zopro wall time by 9.8–22.7%, so the phase durations describe the instrumented path. G1 finished last in all 30 traces on each of 1×4 and 1×16. That order and the uninstrumented G2 results support testing the witness and G1 paths next.

| Shape | Input | Zopro witness ms | Zopro pre-G2 ms | Zopro G2 ms | Zopro G2 minus last G1 finish ms | Aeglos witness ms | Aeglos FFT ms | Aeglos MSM ms | Probe wall change |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | 2.149 | 3.195 | 4.886 | -0.622 | 1.572 | 0.910 | 4.082 | +22.74% |
| 1×2 | heldout | 1.946 | 3.114 | 4.944 | +1.574 | 1.579 | 0.933 | 4.114 | +17.64% |
| 1×4 | catalog | 2.035 | 3.025 | 4.677 | -3.064 | 1.630 | 1.313 | 5.110 | +22.14% |
| 1×4 | heldout | 1.913 | 2.963 | 4.398 | -2.956 | 1.588 | 1.369 | 5.184 | +20.22% |
| 1×16 | catalog | 2.745 | 3.805 | 6.971 | -2.277 | 1.730 | 1.318 | 7.171 | +9.76% |
| 1×16 | heldout | 2.784 | 3.949 | 6.645 | -2.622 | 1.751 | 1.335 | 7.382 | +10.25% |
| 2×2 | catalog | 2.405 | 3.332 | 6.813 | -0.192 | 1.707 | 1.291 | 6.865 | +11.33% |
| 2×2 | heldout | 2.395 | 3.412 | 6.724 | -0.038 | 1.710 | 1.296 | 6.717 | +16.56% |

## Full timing panel

| Shape | Input | Profile | Proof ms | Paired vs trial control | Paired vs exact SDK | Paired vs Aeglos | Split diagnostic ms | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | Exact SDK | 8.041 | +1.15% (0/8) | +0.00% (0/8) | -0.24% (4/8) | — | 51.9 |
| 1×2 | catalog | Trial control | 7.901 | +0.00% (0/8) | -1.14% (8/8) | -2.27% (6/8) | 0.609 (4 ranges) | 51.3 |
| 1×2 | catalog | 512 scalars | 7.892 | -0.97% (5/8) | -2.00% (5/8) | -1.08% (5/8) | 0.278 (4 ranges) | 46.6 |
| 1×2 | catalog | 2,048 scalars | 8.233 | +5.15% (3/8) | +1.82% (3/8) | +0.23% (3/8) | 0.326 (4 ranges) | 49.0 |
| 1×2 | catalog | 8,192 scalars | 8.184 | +2.37% (3/8) | +1.73% (3/8) | +0.68% (2/8) | 0.219 (4 ranges) | 46.9 |
| 1×2 | catalog | Prepared Aeglos | 8.118 | +2.35% (2/8) | +0.25% (4/8) | +0.00% (0/8) | — | 464.3 |
| 1×2 | catalog | Pinned 1×2 | 7.825 | -1.15% (7/8) | -2.45% (8/8) | -3.56% (7/8) | — | 43.0 |
| 1×2 | heldout | Exact SDK | 7.781 | +1.08% (0/8) | +0.00% (0/8) | -5.69% (8/8) | — | 54.2 |
| 1×2 | heldout | Trial control | 7.681 | +0.00% (0/8) | -1.07% (8/8) | -7.21% (8/8) | 0.635 (4 ranges) | 51.3 |
| 1×2 | heldout | 512 scalars | 7.630 | -0.54% (6/8) | -1.95% (7/8) | -7.37% (8/8) | 0.277 (4 ranges) | 46.7 |
| 1×2 | heldout | 2,048 scalars | 7.648 | -0.15% (5/8) | -1.61% (8/8) | -7.35% (8/8) | 0.348 (4 ranges) | 48.3 |
| 1×2 | heldout | 8,192 scalars | 7.651 | -0.26% (5/8) | -1.90% (8/8) | -7.21% (8/8) | 0.210 (4 ranges) | 51.2 |
| 1×2 | heldout | Prepared Aeglos | 8.264 | +7.77% (0/8) | +6.03% (0/8) | +0.00% (0/8) | — | 464.2 |
| 1×2 | heldout | Pinned 1×2 | 7.745 | +0.84% (1/8) | -0.49% (6/8) | -6.31% (8/8) | — | 41.4 |
| 1×4 | catalog | Exact SDK | 9.794 | +1.14% (2/8) | +0.00% (0/8) | +1.31% (2/8) | — | 101.4 |
| 1×4 | catalog | Trial control | 9.683 | +0.00% (0/8) | -1.13% (6/8) | +0.65% (3/8) | 0.064 (2 ranges) | 91.6 |
| 1×4 | catalog | 512 scalars | 9.667 | +0.06% (4/8) | -1.00% (6/8) | -0.49% (5/8) | 0.060 (2 ranges) | 96.1 |
| 1×4 | catalog | 2,048 scalars | 9.814 | +1.36% (1/8) | +0.41% (3/8) | +2.20% (2/8) | 0.168 (2 ranges) | 101.5 |
| 1×4 | catalog | 8,192 scalars | 10.019 | +3.39% (1/8) | +2.84% (1/8) | +3.53% (0/8) | 0.407 (2 ranges) | 100.7 |
| 1×4 | catalog | Prepared Aeglos | 9.667 | -0.64% (5/8) | -1.30% (6/8) | +0.00% (0/8) | — | 579.8 |
| 1×4 | heldout | Exact SDK | 9.764 | +1.69% (2/8) | +0.00% (0/8) | +0.88% (3/8) | — | 99.2 |
| 1×4 | heldout | Trial control | 9.575 | +0.00% (0/8) | -1.66% (6/8) | -0.51% (5/8) | 0.063 (2 ranges) | 95.5 |
| 1×4 | heldout | 512 scalars | 9.769 | +1.30% (2/8) | -0.24% (5/8) | +0.54% (3/8) | 0.059 (2 ranges) | 99.1 |
| 1×4 | heldout | 2,048 scalars | 9.746 | +1.90% (1/8) | -0.31% (5/8) | +1.59% (2/8) | 0.176 (2 ranges) | 94.7 |
| 1×4 | heldout | 8,192 scalars | 9.915 | +2.98% (0/8) | +1.39% (0/8) | +2.37% (2/8) | 0.214 (2 ranges) | 96.9 |
| 1×4 | heldout | Prepared Aeglos | 9.706 | +0.51% (3/8) | -0.87% (5/8) | +0.00% (0/8) | — | 584.8 |
| 1×16 | catalog | Exact SDK | 12.664 | -0.16% (5/8) | +0.00% (0/8) | +4.72% (1/8) | — | 164.5 |
| 1×16 | catalog | Trial control | 12.724 | +0.00% (0/8) | +0.16% (3/8) | +4.26% (1/8) | 1.259 (4 ranges) | 164.4 |
| 1×16 | catalog | 512 scalars | 13.576 | +7.10% (0/8) | +7.12% (0/8) | +12.04% (0/8) | 0.403 (4 ranges) | 169.8 |
| 1×16 | catalog | 2,048 scalars | 13.620 | +6.76% (0/8) | +7.14% (0/8) | +11.97% (1/8) | 0.488 (4 ranges) | 172.7 |
| 1×16 | catalog | 8,192 scalars | 13.462 | +5.27% (0/8) | +5.82% (0/8) | +8.81% (1/8) | 0.628 (4 ranges) | 161.5 |
| 1×16 | catalog | Prepared Aeglos | 12.130 | -4.09% (7/8) | -4.51% (7/8) | +0.00% (0/8) | — | 827.9 |
| 1×16 | heldout | Exact SDK | 12.695 | -0.14% (5/8) | +0.00% (0/8) | +2.90% (0/8) | — | 168.2 |
| 1×16 | heldout | Trial control | 12.682 | +0.00% (0/8) | +0.14% (3/8) | +3.24% (0/8) | 1.384 (4 ranges) | 163.7 |
| 1×16 | heldout | 512 scalars | 13.621 | +7.09% (0/8) | +7.27% (0/8) | +10.34% (0/8) | 0.384 (4 ranges) | 165.2 |
| 1×16 | heldout | 2,048 scalars | 13.741 | +7.92% (0/8) | +8.38% (0/8) | +11.73% (0/8) | 0.486 (4 ranges) | 168.2 |
| 1×16 | heldout | 8,192 scalars | 13.395 | +5.55% (0/8) | +5.81% (0/8) | +8.65% (0/8) | 0.291 (4 ranges) | 168.2 |
| 1×16 | heldout | Prepared Aeglos | 12.334 | -3.14% (8/8) | -2.82% (8/8) | +0.00% (0/8) | — | 824.4 |
| 2×2 | catalog | Exact SDK | 10.621 | +0.28% (3/8) | +0.00% (0/8) | -8.23% (8/8) | — | 75.2 |
| 2×2 | catalog | Trial control | 10.610 | +0.00% (0/8) | -0.28% (5/8) | -8.53% (8/8) | 0.383 (2 ranges) | 82.7 |
| 2×2 | catalog | 512 scalars | 10.625 | +0.40% (1/8) | +0.13% (2/8) | -8.12% (8/8) | 0.389 (2 ranges) | 75.3 |
| 2×2 | catalog | 2,048 scalars | 10.700 | +0.72% (1/8) | +0.45% (2/8) | -7.36% (8/8) | 0.467 (2 ranges) | 85.0 |
| 2×2 | catalog | 8,192 scalars | 10.655 | +0.55% (3/8) | -0.08% (5/8) | -8.93% (8/8) | 0.375 (2 ranges) | 79.7 |
| 2×2 | catalog | Prepared Aeglos | 11.559 | +9.32% (0/8) | +8.97% (0/8) | +0.00% (0/8) | — | 804.3 |
| 2×2 | heldout | Exact SDK | 10.655 | -0.02% (4/8) | +0.00% (0/8) | -9.28% (8/8) | — | 76.0 |
| 2×2 | heldout | Trial control | 10.674 | +0.00% (0/8) | +0.02% (4/8) | -8.63% (8/8) | 0.380 (2 ranges) | 80.5 |
| 2×2 | heldout | 512 scalars | 10.629 | -0.27% (5/8) | +0.01% (4/8) | -9.46% (8/8) | 0.382 (2 ranges) | 76.8 |
| 2×2 | heldout | 2,048 scalars | 10.742 | +0.57% (2/8) | +1.25% (2/8) | -8.24% (8/8) | 0.492 (2 ranges) | 80.6 |
| 2×2 | heldout | 8,192 scalars | 10.624 | -0.61% (6/8) | -0.65% (5/8) | -9.61% (8/8) | 0.259 (2 ranges) | 76.1 |
| 2×2 | heldout | Prepared Aeglos | 11.750 | +9.45% (0/8) | +10.23% (0/8) | +0.00% (0/8) | — | 809.5 |

Eight timing blocks prime three proofs and time seventeen per process. Three separate diagnostic blocks time five proofs per process. The Gnark oracle verified all 520 C7a exports and eight Ryzen preflights. Three native tests passed, including a split comparison with incomplete chunks. The temporary C7a instance, storage and bucket were removed.

## All eight supplied M5 references

| Shape | C7a catalog ms | M5 CPU / Metal ms | CPU / Metal speedup | C7a panel |
|---|---:|---:|---:|---|
| 1×2 | 7.825 | 37 / 24 | 4.73× / 3.07× | This panel |
| 2×2 | 10.621 | 59 / 30 | 5.56× / 2.82× | This panel |
| 4×2 | 13.909 | 109 / 48 | 7.84× / 3.45× | [Grouped SDK widths](../2026-11-20-grouped-sdk-widths-c7a1000/README.md) |
| 8×2 | 18.646 | 189 / 74 | 10.14× / 3.97× | [Balanced LLC placement](../2026-11-20-grouped-solver-epochs-c7a1011/README.md#82-against-the-189-ms-m5-cpu-target) |
| 12×2 | 27.636 | 284 / 101 | 10.28× / 3.65× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 16×2 | 31.036 | 335 / 124 | 10.79× / 4.00× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| 24×2 | 46.118 | 528 / 176 | 11.45× / 3.82× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |
| merge-24×1 | 46.141 | 507 / 175 | 10.99× / 3.79× | [Full catalog](../2026-11-20-all-shapes-c7a979/README.md) |

Five catalog shapes meet 10× against M5 CPU. All eight Metal comparisons are below 10×.

[Summary](artifacts/summary.json) · [Samples and phase traces](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz) · [Cleanup](artifacts/cleanup.json)
