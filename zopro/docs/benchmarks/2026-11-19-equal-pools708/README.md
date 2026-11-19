# Equal and larger G2 pools within sixteen physical cores

Experiment 705 favored ten main workers and six G2 workers on each of its seven Ryzen shapes. This follow-up gives G2 eight or ten of the sixteen cores. The benchmark permits an auxiliary pool as large as the main pool only when both masks are fixed, disjoint and inside the process allocation. These layouts leave the original smaller-pool condition for solver and FFT intact. They therefore run the frontend on the main workers and dedicate the auxiliary cores to G2 during curve work. Every optional G2 dispatch still needs the original live memory reservation.

The previous 10+6 executable and a fresh 10+6 control run alongside 8+8 and 6+10. Each new layout compares idle and enabled G2 placement with identical affinities and frontend execution. Four paired blocks used eleven timed full proofs after three primes. The additional 1×16 input differs from the catalog transaction. Separate one-proof diagnostics verify both pool masks, actual G2 worker counts and a zero-budget fallback. All 129 process exports verified.

The 8+8 split beat the earlier 10+6 profile on 1×4, 1×8 and both 1×16 transactions in every pair. The paired gain on 1×16 was 2.43% on the catalog request and 1.82% on the second. On 2×8 the 8+8 split lost all four. Assigning ten cores to G2 and six to the main pool lost every pair against the previous 10+6 profile. The 96-core comparison includes the even split on the three qualifying shapes when still unresolved.

| Shape | Request | Main + auxiliary | Previous 10+6 / fresh 10+6 / idle / active ms | Active vs idle | Active vs previous 10+6 | Peak MiB previous / active |
|---|---|---|---:|---:|---:|---:|
| 1x16 | catalog | 8+8 | 15.475 / 15.648 / 21.714 / 15.068 | -30.65% (4/4) | -2.43% (4/4) | 99.6 / 118.6 |
| 1x16 | catalog | 6+10 | 15.475 / 15.648 / 27.613 / 19.235 | -30.48% (4/4) | +24.96% (0/4) | 99.6 / 107.4 |
| 1x16 | heldout | 8+8 | 15.418 / 15.581 / 22.091 / 15.120 | -31.60% (4/4) | -1.82% (4/4) | 106.5 / 118.8 |
| 1x16 | heldout | 6+10 | 15.418 / 15.581 / 27.652 / 19.315 | -30.28% (4/4) | +24.98% (0/4) | 106.5 / 110.5 |
| 1x4 | catalog | 8+8 | 10.541 / 10.702 / 14.686 / 10.218 | -30.60% (4/4) | -2.87% (4/4) | 70.0 / 84.5 |
| 1x4 | catalog | 6+10 | 10.541 / 10.702 / 18.821 / 13.144 | -30.24% (4/4) | +25.05% (0/4) | 70.0 / 75.0 |
| 1x8 | catalog | 8+8 | 12.437 / 12.525 / 17.342 / 12.009 | -30.68% (4/4) | -3.31% (4/4) | 78.8 / 94.8 |
| 1x8 | catalog | 6+10 | 12.437 / 12.525 / 22.131 / 15.197 | -31.46% (4/4) | +22.17% (0/4) | 78.8 / 91.2 |
| 2x8 | catalog | 8+8 | 16.266 / 16.387 / 22.173 / 16.552 | -25.05% (4/4) | +2.05% (0/4) | 114.8 / 127.8 |
| 2x8 | catalog | 6+10 | 16.266 / 16.387 / 28.448 / 21.241 | -25.54% (4/4) | +31.08% (0/4) | 114.8 / 122.2 |

[Implementation](artifacts/source.patch) · [Dispatch and reserved bytes](artifacts/routes.json) · [Measurements](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
