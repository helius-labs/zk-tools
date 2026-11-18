# A thread CPU clock around the G2 scalar split

The narrower observer met the declared 5% local latency tolerance at 16 threads and missed it at 32 threads in the first panel. It produced one record for each of 306 timed 1×2 proofs in the observed arms. It measures K/H feed work on the same thread only while that thread owns the split.

| Threads | Exact 257 ms | OFF ms | ON ms | Paired ON/OFF | Paired parent/OFF | Proofs with a K/H feed in the split |
|---:|---:|---:|---:|---:|---:|---:|
| 16 | 9.211 | 8.634 | 8.578 | 1.002 | 1.044 | 10/153 |
| 32 | 9.504 | 7.566 | 10.131 | 1.079 | 1.012 | 7/153 |

Wall time and Linux thread CPU time bracket the scalar decomposition. A thread-local marker enables a second pair of timestamps only for a K/H feed called on the coordinating thread in this interval. The benchmark consumes one fixed-size record after the SDK timer stops. Counts exclude feeds on all other workers. The difference between wall and thread CPU time estimates time off CPU, subject to clock resolution. The observer covers the scalar decomposition only. It leaves the bucket-reduction and final join intervals unmeasured. [Measurements by thread count](artifacts/activity.json) · [Request records](artifacts/processes.json.gz)

Three independent process blocks per arm and count each included three primes and 51 timed proofs. All 18 exports passed Gnark, and the native tests and generic compile check passed. The prototype uses one shared result slot in an isolated benchmark process with one proof in flight. A fresh C7a experiment with plain controls is required before attributing the AWS tail. [Source changes](artifacts/prototype.patch.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)

## Idle-host repeat

A timestamp audit found that host-side verification of experiment 274 overlapped the first local panel. A complete repeat used the same binaries, order, three independent process blocks, affinity and inputs after that work ended. The paired ON/OFF differences fell to 0.49% at 16 threads and 0.65% at 32. Both OFF controls were within 3% of the exact parent. The original panel is retained above.

| Threads | Exact 257 ms | OFF ms | ON ms | Paired ON/OFF | Feed present |
|---:|---:|---:|---:|---:|---:|
| 16 | 8.545 | 8.657 | 8.661 | 1.005 | 12/153 |
| 32 | 7.429 | 7.427 | 7.475 | 1.006 | 7/153 |

All 18 additional exports passed Gnark. The cumulative ledger reached 65,743 unique proofs. [Repeat results](artifacts/quiet-cells.json) · [Repeat process records](artifacts/quiet-processes.json.gz)
