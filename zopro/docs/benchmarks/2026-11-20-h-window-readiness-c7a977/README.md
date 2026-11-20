# H window readiness after K on C7a 4×2

All 42 H range fills were complete when K finished in all 416 detailed traces. With A/K/B2 tiles enabled, weighting had yet to start for a median of one window on catalog and zero on held-out. H finished last in 31 and 39 of 104 traces, while B2 finished last in 65 and 63. This points toward B2 as the next stage to time.

The existing unsigned H path exposes 42 ordinary range fills, 21 weighting stages and one short leading window. The probe records them on the same proof clock as K and B2. It uses the [Ryzen observer](../2026-11-20-h-window-readiness-ryzen976/README.md) and checks the [qualified 975 A/K/B2 layout](../2026-11-20-a-class-tiles-c7a975/README.md) alongside the K/B2 layout at 64 main, 32 frontend and 16 solver physical cores.

| Input | Layout | H traces | Any fill unstarted at K end | Median unstarted fills | Any merge unstarted | Any weighting unstarted | Median unstarted weighting stages | Median H completion after K ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| Catalog | K/B2 tiles | 104 | 0 | 0 | 26 | 60 | 1 | +0.918 |
| Catalog | A/K/B2 tiles | 104 | 0 | 0 | 17 | 54 | 1 | +0.963 |
| Held-out | K/B2 tiles | 104 | 0 | 0 | 36 | 66 | 1 | +1.464 |
| Held-out | A/K/B2 tiles | 104 | 0 | 0 | 22 | 46 | 0 | +0.670 |

| Input | Layout | A last | H last | K last | B2 last | Median H completion after B2 ms | 90th percentile H-only tail ms |
|---|---|---:|---:|---:|---:|---:|---:|
| Catalog | K/B2 tiles | 32 | 28 | 0 | 44 | -0.020 | 0.117 |
| Catalog | A/K/B2 tiles | 8 | 31 | 0 | 65 | -0.061 | 0.155 |
| Held-out | K/B2 tiles | 32 | 36 | 0 | 36 | +0.040 | 0.131 |
| Held-out | A/K/B2 tiles | 2 | 39 | 0 | 63 | -0.074 | 0.219 |

The H-only tail is the time from the last of A, K and B2 completing until H completes, with zero when H finishes earlier. The added H clocks increased paired proof medians by 3.60–4.47% over root clocks. Root clocks themselves added 1.52–2.83% over the uninstrumented observer. The observer with clocks off was within 1.11% of the exact 974 binary. Use the clocked results to describe task readiness and treat the measured millisecond differences as diagnostic.

| Input | Layout | Exact 974 ms | Observer off ms | Plain vs 974 | Root clocks vs plain | H windows vs roots |
|---|---|---:|---:|---:|---:|---:|
| Catalog | K/B2 tiles | 14.434 | 14.557 | +0.23% | +1.52% | +4.47% |
| Catalog | A/K/B2 tiles | 14.317 | 14.108 | -1.11% | +2.81% | +4.13% |
| Held-out | K/B2 tiles | 14.422 | 14.281 | -1.00% | +2.83% | +4.09% |
| Held-out | A/K/B2 tiles | 14.168 | 14.245 | +0.16% | +2.42% | +3.60% |

Each input, layout and mode used eight paired blocks of thirteen timed proofs after three primes. In every window trace the route used 98,674 H bases, 22 unsigned windows at width 12 and two ranges per ordinary window. All 128 proof exports independently verified. The temporary C7a instance and storage were removed.

[Results](artifacts/summary.json) · [Samples and timelines](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Cleanup](artifacts/cleanup.json) · [Eight-shape scorecard](../2026-11-20-a-class-tiles-c7a975/README.md#eight-supplied-m5-examples)
