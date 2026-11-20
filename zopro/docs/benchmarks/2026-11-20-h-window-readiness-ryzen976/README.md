# H window readiness after K on Ryzen 4×2

All 42 H range fills had started by the time K finished in every trace. Some ordinary weighting work remained: at least one weighting stage had yet to start in 11 of 52 catalog traces and 14 of 52 held-out traces. H completed a median 0.115 and 0.284 ms after K.

This trial clocks the existing unsigned H windows and range fills on the K/B2 tile layout. The H and K clocks use the same origin. Part fills, ordinary merges and weighting have separate timestamps. A short leading window is recorded as a whole task. The runner uses 24 main, eight frontend and four solver workers. [K/B2 layout](../2026-11-20-shard-point-tiles-c7a971/README.md)

| Input | Proof clocks | H completed after K | Proofs with an H range fill still unstarted | Median unstarted H range fills | Median in-flight H range fills | Median H weighting stages unstarted | Median H completion after K ms |
|---|---:|---:|---:|---:|---:|---:|---:|
| Catalog | 52 | 39 | 0 | 0 | 0 | 0 | +0.115 |
| Held-out | 52 | 42 | 0 | 0 | 0 | 0 | +0.284 |

| Input | Mode | Proof ms | Peak MiB | Paired change vs previous mode |
|---|---|---:|---:|---:|
| Catalog | Exact 974 control | 15.618 | 225.0 | — |
| Catalog | Observer off | 15.017 | 245.2 | -3.68% (3/4) |
| Catalog | Root clocks | 15.026 | 221.8 | +0.96% (1/4) |
| Catalog | Root and H window clocks | 15.025 | 215.0 | -0.06% (2/4) |
| Held-out | Exact 974 control | 15.177 | 246.5 | — |
| Held-out | Observer off | 14.741 | 235.0 | -2.43% (4/4) |
| Held-out | Root clocks | 14.956 | 248.2 | +2.04% (1/4) |
| Held-out | Root and H window clocks | 15.071 | 242.0 | +0.26% (2/4) |

Each timed proof contains 98,674 H bases, 22 windows at width 12 and 2 ranges per ordinary window. The observed counts are 42 range-fill jobs and 1 short leading windows per proof. A fill counts as unstarted when its first timestamp is at or after K completion. Adding the detailed H clocks changed paired proof medians by 0.06% and 0.26% in magnitude relative to root clocks. The uninstrumented observer build was 3.68% and 2.43% faster than its exact parent in this four-block sample. The binary-level difference limits transfer of the scheduling measurements to other builds or hosts.

Four paired blocks per input and mode prime three proofs and time thirteen. All 32 exports independently verified. Two existing native tests of the unsigned paths passed.

[Summary](artifacts/summary.json) · [Window timestamps and samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Experimental source](artifacts/source.patch.gz)
