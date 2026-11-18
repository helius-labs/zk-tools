# A thread CPU clock around the G2 scalar split

The narrower observer missed the declared 5% local latency tolerance across both worker counts. It produced one record for each of 306 timed 1×2 proofs in the observed arms. It measures K/H feed work on the same thread only while that thread owns the split.

| Threads | Exact 257 ms | OFF ms | ON ms | Paired ON/OFF | Paired parent/OFF | Proofs with a K/H feed in the split |
|---:|---:|---:|---:|---:|---:|---:|
| 16 | 9.211 | 8.634 | 8.578 | 1.002 | 1.044 | 10/153 |
| 32 | 9.504 | 7.566 | 10.131 | 1.079 | 1.012 | 7/153 |

Wall time and Linux thread CPU time bracket the scalar decomposition. A thread-local marker enables a second pair of timestamps only for a K/H feed called on the coordinating thread in this interval. The benchmark consumes one fixed-size record after the SDK timer stops. Counts exclude feeds on all other workers. The difference between wall and thread CPU time estimates time off CPU, subject to clock resolution. The observer covers the scalar decomposition only. It leaves the bucket-reduction and final join intervals unmeasured. [Measurements by thread count](artifacts/activity.json) · [Request records](artifacts/processes.json.gz)

Three independent process blocks per arm and count each included three primes and 51 timed proofs. All 18 exports passed Gnark, and the native tests and generic compile check passed. The prototype uses one shared result slot in an isolated benchmark process with one proof in flight. A fresh C7a experiment with plain controls is required before attributing the AWS tail. [Source changes](artifacts/prototype.patch.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
