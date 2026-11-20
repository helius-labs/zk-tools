# Shared Poseidon equality checks in 4×2 proving

The shared checks fell short of the full-proof requirement. The shared path took **13.387 / 13.391 ms** on the original and held-out requests. The measurements use Ryzen with 24 main and eight frontend threads.

The [earlier census](../2026-11-20-quartet-origin-census-ryzen947/README.md) identified 14,394 quartets whose coefficient equality follows from the same 64 complete Poseidon state comparisons. This trial checks those states before public wire offsets are applied and shares the verdict across K and B2. It binds both prepared point lists to the exact solver projection. A quartet touched by offsets or lacking a passing state check uses the original direct verification. The extra plan uses one byte per quartet and at most 128 short state tuples, plus the existing optional solver origin metadata.

Each input ran eight counterbalanced blocks. Every process primed three proofs and timed seventeen. The comparison uses the frozen 4×2 SDK, current ordinary execution, the exact earlier compacted build, and the new build with direct checks. All 90 exported proofs passed independent Gnark verification.

| Input | Frozen ms | Ordinary ms | Prior compacted ms | Direct checks ms | Shared checks ms | Shared vs direct | Shared vs prior | Shared / direct peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | 13.626 | 13.732 | 13.397 | 13.288 | 13.387 | +0.55% (1/8) | -0.18% (5/8) | 275.8 / 220.2 |
| heldout | 13.578 | 13.699 | 13.410 | 13.339 | 13.391 | +0.60% (3/8) | -0.17% (4/8) | 279.8 / 218.2 |

The same-build direct arm had lower paired medians than the frozen, ordinary, and prior compacted controls on both inputs. It won eight of eight against frozen on each input and six of eight against the prior compacted executable. This is a separate local result for the direct path used to calibrate the shared-check comparison.

| Input | Quartets using a shared check per query | With offsets removed | Direct vs prior calibration |
|---|---:|---:|---:|
| catalog | 14,394 | 14,394 | -0.96% (6/8) |
| heldout | 14,394 | 14,394 | -0.57% (6/8) |

The traced runs independently compared every shared verdict with the four direct coefficients. Zero optional workspace returned to ordinary processing. A malformed K plan left B2 on its direct check. The native tests also exercise unequal state fields and reject alternate row identities. The evidence covers local full proofs.

[Results](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Samples](artifacts/rows.json.gz) · [Experimental source](artifacts/source.patch.gz)
