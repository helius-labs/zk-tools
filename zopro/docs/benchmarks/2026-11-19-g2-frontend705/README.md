# Reusing the frontend pool for the exclusive G2 query

The solver and FFT can already use a caller-owned Rayon pool. This trial reuses that pool during the subsequent elliptic-curve phase. A and K/H run on the main pool while the entire exclusive G2 query, including grouping, runs on the frontend pool. No workers or key tables are added. Both pools keep their existing CPU affinity. The key computes an auxiliary workspace estimate when its caller attaches the pool. Each proof checks current headroom before entering it. If admission fails, it follows the existing main-pool path.

The Ryzen run assigns one worker per physical core, with the same sixteen-core process mask for every configuration. Release and shared use sixteen main workers and an overlapping four-core frontend. The other arms partition the physical cores 12+4 or 10+6. Idle uses the auxiliary pool only for its original frontend tasks, active also places G2 there. A separate marker inside G2 verifies the pool width. The zero-budget 1×16 proof exercises the fallback. Four blocks time eleven warm proofs following three primes. All 207 process exports verified against independently calculated public inputs.

With ten main workers and six G2 workers, Zopro beat both the original 16-core profile and the ten-core idle control in all 32 matched comparisons across seven shapes and a second 1×16 request. At 1×16 the two requests gained 5.59% and 5.27% against the original profile, with 110.1 and 97.4 MiB peak RSS. Moving two cores from G2 to the main pool reversed the outcome on every input. All 157 native core tests passed, as did a release build check of the generic x86-64 prover. The relative pool sizes are being tested separately on the 96-core C7a.

| Shape | Request | Main + auxiliary | Release / shared / idle / active ms | Active vs idle | Active vs release | Peak MiB release / active |
|---|---|---|---:|---:|---:|---:|
| 1x16 | catalog | 10+6 | 16.418 / 16.169 / 20.046 / 15.517 | -22.30% (4/4) | -5.59% (4/4) | 107.4 / 110.1 |
| 1x16 | catalog | 12+4 | 16.418 / 16.169 / 18.248 / 20.381 | +11.42% (0/4) | +24.20% (0/4) | 107.4 / 93.2 |
| 1x16 | heldout | 10+6 | 16.414 / 16.244 / 19.963 / 15.508 | -22.61% (4/4) | -5.27% (4/4) | 112.8 / 97.4 |
| 1x16 | heldout | 12+4 | 16.414 / 16.244 / 18.384 / 20.449 | +11.15% (0/4) | +24.60% (0/4) | 112.8 / 98.7 |
| 1x2 | catalog | 10+6 | 8.892 / 8.706 / 11.163 / 8.451 | -24.15% (4/4) | -4.60% (4/4) | 53.2 / 51.0 |
| 1x2 | catalog | 12+4 | 8.892 / 8.706 / 10.238 / 11.437 | +11.41% (0/4) | +29.16% (0/4) | 53.2 / 49.0 |
| 1x4 | catalog | 10+6 | 11.396 / 11.324 / 13.717 / 10.615 | -22.34% (4/4) | -6.54% (4/4) | 82.0 / 68.2 |
| 1x4 | catalog | 12+4 | 11.396 / 11.324 / 12.668 / 14.036 | +10.54% (0/4) | +22.70% (0/4) | 82.0 / 69.2 |
| 1x8 | catalog | 10+6 | 13.139 / 13.034 / 15.810 / 12.484 | -21.04% (4/4) | -5.04% (4/4) | 91.0 / 82.8 |
| 1x8 | catalog | 12+4 | 13.139 / 13.034 / 14.760 / 16.453 | +11.47% (0/4) | +25.55% (0/4) | 91.0 / 79.5 |
| 2x2 | catalog | 10+6 | 12.857 / 12.764 / 15.443 / 11.739 | -23.47% (4/4) | -8.47% (4/4) | 94.2 / 83.2 |
| 2x2 | catalog | 12+4 | 12.857 / 12.764 / 14.316 / 15.263 | +6.51% (0/4) | +18.17% (0/4) | 94.2 / 82.0 |
| 2x4 | catalog | 10+6 | 13.728 / 13.425 / 16.416 / 12.613 | -22.95% (4/4) | -8.42% (4/4) | 94.8 / 88.8 |
| 2x4 | catalog | 12+4 | 13.728 / 13.425 / 15.231 / 16.278 | +6.98% (0/4) | +18.44% (0/4) | 94.8 / 92.5 |
| 2x8 | catalog | 10+6 | 18.086 / 17.911 / 20.721 / 16.062 | -22.15% (4/4) | -11.20% (4/4) | 125.8 / 117.2 |
| 2x8 | catalog | 12+4 | 18.086 / 17.911 / 19.763 / 20.791 | +5.17% (0/4) | +14.97% (0/4) | 125.8 / 111.2 |

[Implementation](artifacts/source.patch) · [Dispatch and reserved bytes](artifacts/routes.json) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
