# K/H continuations on a C7a

The exact experimental parent is labeled ref. In the new SDK, off uses the original parallel iterator. Whole submits one task per window. Split submits separate K, H and weighting tasks for each window, passing ownership of its existing bucket state. This lets the executing worker return to its pending join after each task. The build uses the existing worker pool and twenty window states. The feature is opt-in and limited to pools with at least twenty workers.

The split candidate met the prespecified 64-core screen. It finished in **9.390 ms**, against 9.575 ms for the disabled build and 9.499 ms for the exact parent. The median within-block reduction against disabled was **2.37%**, with five wins in seven pairs. The number of requests taking at least 11 ms fell from 239 to 150 of 707, with similar CPU time and peak RSS. Retained Aeglos reached 8.031 ms at 96 cores on this host. The paired gains were smaller against the exact parent (0.44%, four wins) and whole-window control (0.15%, four wins). This makes an independent same-host comparison especially important.

At 96 cores the split measured 9.950 ms against 10.064 ms disabled. The one-job-per-window control measured 9.911 ms, leaving the benefit of splitting specific to the execution width. The 2×2 fixture follows a different route.

| Shape | Cores / threads | Mode | Median ms | Paired vs off | Wins | CPU ms | Peak MiB | Requests <9 / ≥11 ms | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | off | 9.575 | — | — | 318 | 74 | 451 / 239 | 3.86× / 2.51× |
| 1x2 | 64 | ref | 9.499 | +0.05% | 3/7 | 313 | 73 | 456 / 225 | 3.90× / 2.53× |
| 1x2 | 64 | split | 9.390 | -2.37% | 5/7 | 316 | 72 | 456 / 150 | 3.94× / 2.56× |
| 1x2 | 64 | whole | 9.466 | -0.43% | 4/7 | 315 | 81 | 466 / 219 | 3.91× / 2.54× |
| 1x2 | 96 | aeglos | 8.031 | — | — | 508 | 479 | 703 / 0 | 4.61× / 2.99× |
| 1x2 | 96 | off | 10.064 | — | — | 471 | 67 | 32 / 179 | 3.68× / 2.38× |
| 1x2 | 96 | ref | 10.176 | +0.49% | 2/7 | 459 | 68 | 39 / 202 | 3.64× / 2.36× |
| 1x2 | 96 | split | 9.950 | -1.78% | 5/7 | 474 | 70 | 16 / 141 | 3.72× / 2.41× |
| 1x2 | 96 | whole | 9.911 | -0.91% | 6/7 | 468 | 73 | 43 / 150 | 3.73× / 2.42× |
| 2x2 | 48 | off | 13.943 | — | — | 379 | 312 | 0 / 707 | 4.23× / 2.15× |
| 2x2 | 48 | ref | 13.974 | +0.45% | 2/7 | 378 | 303 | 0 / 707 | 4.22× / 2.15× |
| 2x2 | 48 | split | 13.978 | +0.25% | 2/7 | 378 | 318 | 0 / 707 | 4.22× / 2.15× |
| 2x2 | 96 | aeglos | 11.538 | — | — | 831 | 827 | 0 / 682 | 5.11× / 2.60× |

Seven rotated processes per cell, 101 warm sequential requests per process. The same host ran Zopro with 32 frontend workers and retained Aeglos at 96 cores. Every one of 91 fresh exports verified independently on AWS and after retrieval. The instance terminated and its temporary storage was removed. [Local implementation and route checks](../2026-11-18-kh-continuation295/README.md)

[Paired blocks](artifacts/pairs.json) · [All requests](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
