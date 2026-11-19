# Physical core and SMT scaling on two Ryzen cache domains

Both placements with one worker per core span the same two L3 caches. The eight-core run alternates between disjoint sets of four physical cores per domain. The sixteen-core run fills both domains. The final run enables both SMT siblings on each of those same sixteen cores. Each shape and executable uses all six orders of the three widths. Each process warms three requests and times nine requests sequentially. All 90 exported proofs verified independently with the original public inputs and keys. The prepared Aeglos engine reported a cache hit on every timed request.

Release is the ordinary Zopro executable pinned in the staged C7a pilot. Owned uses the integrated candidate with its grouped G2 and packed FFT options on 5×16. The table reports median process means, paired changes versus the preceding width, counts of lower latency, warm process CPU-ms per proof and peak lifetime RSS. Aeglos reports elapsed time inside its MSM. These two additional catalog shapes have no supplied M5 references. The local SMT change and the C7a physical-core increase are different interventions.

| Shape | Engine | Workers | Proof ms | Change vs preceding width | CPU-ms/proof | Peak MiB | Aeglos MSM ms |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x16 | aeglos | 8 | 37.279 | — | 280.0 | 772.3 | 31.860 |
| 1x16 | aeglos | 16 | 23.054 | -37.6% (6/6) | 355.1 | 775.2 | 19.186 |
| 1x16 | aeglos | 32 | 22.234 | -3.5% (6/6) | 662.3 | 772.8 | 18.600 |
| 1x16 | release | 8 | 23.104 | — | 169.5 | 136.8 | — |
| 1x16 | release | 16 | 14.798 | -36.0% (6/6) | 191.9 | 192.8 | — |
| 1x16 | release | 32 | 13.041 | -12.4% (6/6) | 310.6 | 237.8 | — |
| 5x16 | aeglos | 8 | 106.061 | — | 798.1 | 1910.0 | 87.863 |
| 5x16 | aeglos | 16 | 68.768 | -35.0% (6/6) | 1075.4 | 1886.0 | 56.701 |
| 5x16 | aeglos | 32 | 69.924 | +1.6% (1/6) | 2164.0 | 1887.2 | 59.069 |
| 5x16 | owned | 8 | 40.329 | — | 302.8 | 270.8 | — |
| 5x16 | owned | 16 | 25.869 | -35.9% (6/6) | 363.1 | 327.8 | — |
| 5x16 | owned | 32 | 23.375 | -10.3% (6/6) | 624.5 | 404.8 | — |
| 5x16 | release | 8 | 41.814 | — | 312.4 | 272.5 | — |
| 5x16 | release | 16 | 26.193 | -37.4% (6/6) | 379.0 | 344.5 | — |
| 5x16 | release | 32 | 24.333 | -7.6% (6/6) | 654.7 | 416.0 | — |

Doubling physical cores from eight to sixteen reduced Zopro latency in every pair on each shape, by 36.0% on 1×16 and 37.4% on 5×16. Prepared Aeglos gained 37.6% and 35.0%. Going from sixteen to thirty-two logical workers on those same cores gave Zopro an additional 12.4% and 7.6%. Aeglos gained 3.5% on 1×16 and took 1.6% longer on 5×16. This machine reproduces neither of the two historical C7a scaling reversals at these physical widths.

The outstanding AWS comparison measures 32 physical cores in four versus twelve L3 domains before using all 96 cores. That comparison keeps core count fixed while changing placement, then keeps the twelve domains available while adding cores. The primary endpoints are single-proof latency and CPU-ms per proof, with Aeglos MSM time and peak RSS available for interpretation. [Staged C7a experiment](../2026-11-19-c7a-placement674/README.md) · [Earlier C7a catalog](../2026-11-18-full-catalog-c7a244/README.md)

[Topology](artifacts/topology.json) · [Executable and input hashes](artifacts/pins.json) · [Rust xtask workflow](artifacts/workflow.json.gz) · [Process measurements](artifacts/rows.json.gz) · [Proof verification](artifacts/verification.json)
