# Deferring the G2 join until after G1 on Ryzen

Moving the G2 join out of the main curve work reduced paired proof latency by 4.38–5.17% against the early schedule with a blocking receive. Relative to the same-build late start, the changes ranged from −0.36% to +0.32%, and neither shape qualified. With the scoped schedule, the final G1 output came 3.36–4.55 ms after the G2 finish in the clock panel.

The previous [early-start trial](../2026-11-20-early-g2-overlap-ryzen990/README.md) received the G2 point inside a main Rayon worker. The scoped variant completes all G1 work on the main pool before joining the frontend task and appending its G2 point. The original G2 arithmetic, public-wire correction and optional scratch fallback remain in use. The Ryzen run uses eight distinct physical cores per pool. The [C7a profile](../2026-11-20-exact-one-input-phase-c7a986/README.md) uses 48 cores per pool and usually finishes G2 last, so the same scoped candidate is being measured separately there.

| Shape | Request | Pinned / late / blocking / scoped ms | Scoped paired vs late | Scoped paired vs blocking | Scoped G2 / G1 finish ms | Peak late / scoped MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×8 | catalog | 12.067 / 12.162 / 12.687 / 12.064 | -0.36% (5/8) | -4.73% (8/8) | 6.295 / 9.647 | 102.5 / 101.2 |
| 1×8 | heldout | 12.066 / 12.067 / 12.705 / 12.137 | +0.32% (2/8) | -4.38% (8/8) | 6.314 / 9.656 | 100.2 / 102.8 |
| 1×16 | catalog | 15.201 / 15.235 / 16.016 / 15.205 | -0.08% (5/8) | -5.17% (8/8) | 7.717 / 12.261 | 120.2 / 118.2 |
| 1×16 | heldout | 15.211 / 15.260 / 16.093 / 15.300 | -0.05% (4/8) | -4.52% (6/8) | 7.637 / 12.141 | 123.9 / 127.6 |

Eight paired timing blocks prime three proofs and time seventeen. The separate clock panel observes 1.01–1.14 ms of overlap between G2 and the quotient. The G1 and G2 finish columns share a time origin at handoff after witness preparation. Scratch-denial probes selected the original path. All 160 process exports passed independent verification.

[Samples](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Protocol](artifacts/protocol.json) · [Source](artifacts/source.patch.gz)
