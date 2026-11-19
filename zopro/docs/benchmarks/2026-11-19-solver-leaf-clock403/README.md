# Solver schedules weighted by measured leaf durations on Ryzen

Each timed request supplies its own elapsed duration for every serial leaf, measured around its existing arithmetic on the worker executing it. The model uses those durations and the dependencies from the same executable and key. The group column uses the exact adjacent pairs selected by unit cost in experiment 402. It retains inner joins. The ready column uses all true leaf dependencies. The modeled spans assume unlimited workers and omit scheduler overhead. Leaf elapsed time can itself include preemption and contention.

| Shape | Workers | Release ms | Off ms | Clock ms | Paired clock / off | Paired off / release | Within 3% | Measured execution ms | Current span ms | Group span ms | Ready span ms |
|---|---:|---:|---:|---:|---:|---:|---|---:|---:|---:|---:|
| 1x2 | 32 | 7.154 | 7.291 | 7.447 | +2.48% | +1.92% | yes | 0.696 | 0.622 | 0.622 | 0.483 |
| 16x2 | 16 | 44.662 | 45.010 | 44.831 | +0.24% | +0.78% | yes | 3.302 | 2.147 | 1.862 | 1.252 |
| 16x2 | 32 | 40.242 | 40.081 | 41.068 | +2.69% | -0.52% | yes | 3.538 | 3.073 | 2.696 | 2.131 |

All three comparisons met the prespecified 3% calibration limit. On 1×2 at 32 workers the current and grouped spans coincide. Releasing individual leaves reduces the modeled span by 0.139 ms. The measured execution phase took 0.696 ms, compared with the 3.454 ms gap between the exact release and the 3.7 ms target in this run. Most of that gap must be addressed elsewhere in the request. On 16×2 at 32 workers the modeled group saving is 0.377 ms. The separate complete-proof test in experiment 402 did not establish a repeatable gain from those groups.

[Observer](artifacts/observer.rs) · [Protocol](artifacts/protocol.md) · [Per-request models](artifacts/models.json) · [Leaf durations and original graphs](artifacts/samples.json.gz) · [Verified exports](artifacts/verification.json) · [Measured group execution](../2026-11-19-paired-epoch-groups402/README.md)
