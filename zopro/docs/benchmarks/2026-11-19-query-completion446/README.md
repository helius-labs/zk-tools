# Direct completion times for the four curve-query branches

Each worker recorded a monotonic timestamp after completing A, K, H or the B branch. The caller read those timestamps after the join. The B branch includes the B1/B2 join where present. The experiment compared the integrated release, the isolated build with the clock disabled and the same build with the clock enabled. Each shape and worker count has 33 timed trace records in three independent processes.

| Shape | Workers | Wall ms release / disabled / traced | Last branch A / K / H / B | Median gap to runner-up, ms | Median last branch to join, µs |
|---|---:|---:|---:|---:|---:|
| 16x2 | 16 | 44.583 / 44.315 / 44.354 | 1 / 0 / 30 / 2 | 0.109 | 4.2 |
| 16x2 | 32 | 40.264 / 39.874 / 39.430 | 3 / 3 / 20 / 7 | 0.108 | 4.2 |
| merge-24x1 | 16 | 70.433 / 69.987 / 69.431 | 1 / 2 / 25 / 5 | 0.118 | 4.7 |
| merge-24x1 | 32 | 65.286 / 64.919 / 65.182 | 7 / 2 / 24 / 0 | 0.121 | 4.1 |

H finished last in 20 of 33 requests on 16×2 and 24 of 33 on merge at 32 workers. The median gap from the last branch to the runner-up was only 0.108 and 0.121 ms respectively. Most of the branch computation overlaps, so wall time saved by accelerating H also depends on how the freed workers affect the other branches.

All 36 exported proofs verified independently. These measurements cover one Ryzen host. A hardware-matched C7a run is required to identify which branch finishes last at 96 cores.

[Source patch](artifacts/source.patch) · [Per-request measurements](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Archived C7a samples](../2026-11-19-c7a-final-ms445/README.md)
