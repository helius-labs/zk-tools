# Bounded point tiles for exclusive G2 on Ryzen

The tiled path activated on 2×2 and reduced 52,274 B2 terms to 27,218 or 27,267 exact scalar classes, depending on the input. Its full proof was 0.07–0.82% slower in paired median than the same-build control. The compactor declined admission on 1×2 and used the ordinary G2 path. No shard count qualified on these two keys.

The trial reuses the [canonical preparation compactor](../2026-11-20-fused-kh-class-tiles-ryzen982/README.md) and applies it directly to exclusive B2 before the existing G2 windows. Key points remain shared and the compactor allocates proof-local scratch through the existing budget. A zero allowance forces the ordinary path. Oracle runs compare the complete B2 point with the direct native MSM. The benchmark remaps the exact C7a inputs and worker profiles to 24 main and eight frontend Ryzen hardware threads.

| Shape | Request | Mode | Proof ms | Paired vs pinned | Paired vs same-build | Exclusive G2 ms | Tile preparation ms | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | Pinned C7a build | 6.897 | — | -1.10% (6/8) | — | — | 60.8 |
| 1×2 | catalog | Same-build control | 6.922 | +1.12% (2/8) | — | 4.680 | — | 58.2 |
| 1×2 | catalog | Ordinary grouping | 6.816 | -0.97% (7/8) | -1.71% (8/8) | 4.648 | — | 57.5 |
| 1×2 | catalog | Eight tile shards | 6.894 | +0.17% (4/8) | -0.88% (7/8) | 4.186 | — | 58.0 |
| 1×2 | catalog | Sixteen tile shards | 6.874 | +0.36% (3/8) | -0.35% (7/8) | 4.198 | — | 58.2 |
| 1×2 | heldout | Pinned C7a build | 6.758 | — | -0.85% (6/8) | — | — | 58.8 |
| 1×2 | heldout | Same-build control | 6.755 | +0.85% (2/8) | — | 4.582 | — | 58.5 |
| 1×2 | heldout | Ordinary grouping | 6.741 | +0.47% (3/8) | -0.48% (6/8) | 4.561 | — | 57.2 |
| 1×2 | heldout | Eight tile shards | 6.741 | +0.21% (3/8) | -0.57% (6/8) | 4.051 | — | 57.8 |
| 1×2 | heldout | Sixteen tile shards | 6.786 | +0.45% (2/8) | -0.56% (5/8) | 4.074 | — | 57.0 |
| 2×2 | catalog | Pinned C7a build | 9.439 | — | -0.61% (6/8) | — | — | 96.0 |
| 2×2 | catalog | Same-build control | 9.482 | +0.62% (2/8) | — | 5.994 | — | 94.5 |
| 2×2 | catalog | Eight tile shards | 9.512 | +1.10% (2/8) | +0.32% (3/8) | 6.010 | 1.822 | 94.2 |
| 2×2 | catalog | Sixteen tile shards | 9.520 | +1.25% (3/8) | +0.82% (3/8) | 6.099 | 1.867 | 99.5 |
| 2×2 | heldout | Pinned C7a build | 9.408 | — | -0.48% (6/8) | — | — | 96.5 |
| 2×2 | heldout | Same-build control | 9.535 | +0.48% (2/8) | — | 6.027 | — | 99.2 |
| 2×2 | heldout | Eight tile shards | 9.522 | +0.93% (2/8) | +0.47% (3/8) | 6.047 | 1.855 | 101.0 |
| 2×2 | heldout | Sixteen tile shards | 9.553 | +0.70% (2/8) | +0.07% (4/8) | 6.054 | 1.861 | 99.0 |

Eight paired blocks prime three proofs and time seventeen. Query timing comes from a separate instrumented panel. When admission declines, the ordinary query clock begins after the attempted dispatch. For 1×2 the same-build control retains the pinned grouping setting and the ordinary grouping arm isolates the effect of enabling it. The four native tests and all 204 independent proof verifications passed.

[Samples and route records](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Source](artifacts/source.patch.gz) · [Native tests](artifacts/native-tests.json)
