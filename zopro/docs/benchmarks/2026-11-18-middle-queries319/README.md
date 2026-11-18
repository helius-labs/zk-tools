# Last curve queries and frontend worker placement on Ryzen

Full SDK traces on 16 physical Ryzen cores recorded G2 as the last of the four measured curve queries on 28 of 33 requests for 2×2, 21 for 4×2, 20 for 8×2 and 30 for 12×2. Other queries often finished close behind. The intervals below measure the opportunity from advancing the last return while holding the other three fixed.

| Shape | G2 last | Ungrouped G1 last | Grouped G1 last | Median gap to next return (ms) | Paired trace overhead |
|---|---:|---:|---:|---:|---:|
| 2x2 | 28/33 | 1/33 | 4/33 | 0.354 | +0.80% |
| 4x2 | 21/33 | 9/33 | 3/33 | 0.052 | +0.60% |
| 8x2 | 20/33 | 10/33 | 3/33 | 0.113 | -0.02% |
| 12x2 | 30/33 | 0/33 | 3/33 | 0.257 | -1.39% |

The grouped G1 return site is in `msm_grouped<G1>` in the retained executable. Each trace contains three G1 raw calls and either one raw G2 or one exclusive G2 call per SDK request. The four returns do not include final proof assembly. Three plain/trace blocks per shape each timed eleven requests.

## Which pool actually runs preparation

An additional thread-ID trace used the exact frontend-pool candidate, with and without eight caller-owned workers inside the 16-core affinity mask. The SDK reports all main-pool worker TIDs and the complete task count before and after proving. The trace identifies the entry thread for each function. A caller entry can dispatch parallel work to the regular Rayon pool.

| Shape | Canonical frontend with pool | Solver entry | FFT entry |
|---|---|---|---|
| 1x2 | yes | frontend | frontend |
| 2x2 | yes | frontend | frontend |
| 4x2 | no | caller | caller |
| 8x2 | no | caller | caller |
| 12x2 | no | caller | caller |
| 16x2 | no | caller | caller |
| 24x2 | no | caller | caller |
| merge-24x1 | no | caller | caller |

Each pool-placement cell exported a verified proof after three observed requests. The alternative frontend pool was used for 1×2 and 2×2. The six larger shapes executed the general solver and product FFT with their caller as the entry thread even with an attached pool. Experiment 321 tests attaching the smaller pool to those general phases.

## Where time goes on the same proofs

Intervals are disjoint and use the timed SDK call on the exact 16-core integrated binary. Each column is the median of 33 request intervals. The first interval includes SDK witness work, solving and scalar gathering. The curve span starts at the first observed raw query and ends at the last observed return. Concurrent query durations are counted once.

| Shape | SDK entry to product FFT ms | Product FFT ms | FFT return to first curve ms | Concurrent curve span ms | Last curve to SDK return ms |
|---|---:|---:|---:|---:|---:|
| 2x2 | 1.955 | 1.150 | 0.005 | 8.288 | 0.654 |
| 4x2 | 2.649 | 2.368 | 0.127 | 11.233 | 0.726 |
| 8x2 | 4.069 | 4.532 | 0.210 | 17.338 | 0.728 |
| 12x2 | 5.047 | 9.707 | 0.023 | 23.027 | 0.726 |

These intervals describe local traces. The AWS frontend and curve split can differ as worker count and the last query change.
