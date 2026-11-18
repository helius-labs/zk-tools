# Yielding a K/H window between its two input arrays

The exact experimental parent is labeled ref. In the new SDK, off uses the original parallel iterator. Whole submits one task per window. Split submits separate K, H and weighting tasks for each window, passing ownership of its existing bucket state. This lets the executing worker return to its pending join after each task. The build uses the existing worker pool and twenty window states. The feature is opt-in and limited to pools with at least twenty workers.

On 1×2 with 32 local threads, splitting the task reduced paired latency by 3.08% against the disabled build. It won four of seven pairs and its 7.646 ms median remained above the exact parent’s 7.526 ms. The local result falls short of the prespecified five paired wins. The 64-core C7a experiment tests the environment where the long G2 tail was observed.

| Shape | Cores / threads | Mode | Median ms | Paired vs off | Wins | CPU ms | Peak MiB | Requests <9 / ≥11 ms | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 16 | off | 8.783 | — | — | 101 | 64 | 42 / 0 | 4.21× / 2.73× |
| 1x2 | 16 | split | 8.891 | +1.58% | 1/3 | 101 | 64 | 41 / 0 | 4.16× / 2.70× |
| 1x2 | 32 | off | 7.740 | — | — | 150 | 118 | 241 / 90 | 4.78× / 3.10× |
| 1x2 | 32 | ref | 7.526 | -2.99% | 4/7 | 149 | 107 | 285 / 48 | 4.92× / 3.19× |
| 1x2 | 32 | split | 7.646 | -3.08% | 4/7 | 150 | 94 | 291 / 49 | 4.84× / 3.14× |
| 1x2 | 32 | whole | 7.670 | +1.22% | 2/7 | 150 | 100 | 279 / 63 | 4.82× / 3.13× |
| 2x2 | 32 | off | 10.463 | — | — | 200 | 251 | 0 / 23 | 5.64× / 2.87× |
| 2x2 | 32 | split | 10.360 | -1.61% | 2/3 | 200 | 252 | 0 / 4 | 5.69× / 2.90× |

On the local 32-thread Ryzen, seven processes per mode timed 51 warm 1×2 requests each. Each secondary cell used three processes timing 21 requests. Frontend width was eight. A separate trace resolved exactly twenty K, twenty H and twenty weighting jobs on a split 1×2 proof. Whole ran twenty jobs. At 16 threads on 1×2 and at 32 threads on 2×2 the new task callbacks did not run. All 146 native core tests, the generic compile check and all 45 exported proofs passed. [Patch](artifacts/source.patch) · [Route counts](artifacts/routes.json)

[Paired blocks](artifacts/pairs.json) · [All requests](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
