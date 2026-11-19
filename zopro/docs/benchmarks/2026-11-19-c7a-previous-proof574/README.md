# Where the 96-core time goes before and after the curve query

The twelve archived C7a processes contain eleven sequential proofs apiece. The reader checks nine disjoint tracepoint intervals against the full SDK duration for every proof. It also reproduces all 132 previously reported G2 timing labels and curve durations, and checks the trace, placement-log and proof digests. These are historical instrumented processes on one C7a host. No new cloud timings are included.

| Shape | Elapsed interval | Mean 64-core ms | Mean 96-core ms | 96 minus 64 ms | Width differences per recording block, ms |
|---|---|---:|---:|---:|---:|
| 16×2 | Witness construction | 0.910 | 0.969 | +0.059 | +0.065 / +0.082 / +0.029 |
| 16×2 | Solver | 5.295 | 6.293 | +0.998 | +0.990 / +1.146 / +0.859 |
| 16×2 | Solver return to FFT entry | 1.344 | 1.769 | +0.425 | +0.425 / +0.366 / +0.483 |
| 16×2 | FFT product | 6.421 | 6.540 | +0.118 | +0.130 / +0.156 / +0.069 |
| 16×2 | Curve query | 20.915 | 20.577 | -0.338 | -1.773 / +0.838 / -0.078 |
| 16×2 | Curve return to SDK return | 1.068 | 1.196 | +0.128 | +0.178 / +0.094 / +0.111 |
| 16×2 | Other intervals | 0.314 | 0.335 | +0.022 | +0.024 / +0.013 / +0.027 |
| 16×2 | Whole proof | 36.267 | 37.678 | +1.412 | +0.039 / +2.696 / +1.500 |
| Merge-24×1 | Witness construction | 1.107 | 1.218 | +0.110 | +0.134 / +0.083 / +0.114 |
| Merge-24×1 | Solver | 10.112 | 10.818 | +0.706 | +0.606 / +1.013 / +0.498 |
| Merge-24×1 | Solver return to FFT entry | 1.165 | 1.655 | +0.490 | +0.515 / +0.490 / +0.465 |
| Merge-24×1 | FFT product | 10.254 | 9.569 | -0.685 | -0.751 / -0.594 / -0.710 |
| Merge-24×1 | Curve query | 26.591 | 27.928 | +1.338 | +2.445 / +1.750 / -0.182 |
| Merge-24×1 | Curve return to SDK return | 1.058 | 1.163 | +0.106 | +0.088 / +0.108 / +0.121 |
| Merge-24×1 | Other intervals | 0.334 | 0.346 | +0.012 | +0.004 / +0.018 / +0.014 |
| Merge-24×1 | Whole proof | 50.620 | 52.696 | +2.077 | +3.041 / +2.869 / +0.321 |

## CPU samples between solver return and FFT entry

For the exact interval in each request, the reader counts every userspace CPU-clock sample from all threads. Indexed collection matches symbols naming the solver's parallel collection of field elements from 32-bit wire indices. This category can include scheduling instructions in the same inlined function. Percentages describe sampled CPU activity across the whole pool and cannot be interpreted as percentages of elapsed time.

| Shape | Cores | Samples | Indexed field collection / Crossbeam / sleep, wait or futex / other |
|---|---:|---:|---:|
| 16 | 64 | 835 | 41.8% / 40.4% / 13.2% / 4.7% |
| 16 | 96 | 1810 | 39.0% / 45.9% / 12.4% / 2.8% |
| merge | 64 | 801 | 58.2% / 25.6% / 11.0% / 5.2% |
| merge | 96 | 1654 | 51.7% / 31.1% / 14.2% / 3.0% |

[Per-request counts](artifacts/gap-samples.json) · [Full symbol counts](artifacts/gap-symbols.json.gz)

### Instructions inside the indexed collection symbols

The address reader relocates each sampled process against its pinned SDK ELF. At least three independently named functions constrain each page-aligned load address. Every indexed-collection sample then falls within the exact named symbol range and at a valid instruction boundary. Two hot collector closures contain identical loops that load a word, test a bit, and conditionally perform an atomic compare-exchange with a backwards retry branch. The following rows count samples from the load through that branch. Source line information did not resolve the precise inlined operation.

| Shape | Cores | All gap samples | Indexed collection samples | Samples in two atomic loops | Of collection | Of all gap samples | At the two bit-test instructions |
|---|---:|---:|---:|---:|---:|---:|---:|
| 16 | 64 | 835 | 349 | 245 | 70.2% | 29.3% | 245 |
| 16 | 96 | 1810 | 706 | 568 | 80.5% | 31.4% | 568 |
| merge | 64 | 801 | 466 | 308 | 66.1% | 38.5% | 308 |
| merge | 96 | 1654 | 855 | 666 | 77.9% | 40.3% | 666 |

A sample at the bit test cannot distinguish a cache miss on the preceding load, a failed compare-exchange or sampling skid. The two loops are distinct from the loops that copy the field values into output memory. This supports testing fewer task publications while keeping the same scalar data and output buffers.

[Address reader](artifacts/check_collector.py) · [Relocated sample addresses](artifacts/collector-pcs.json.gz) · [Instructions and ELF digest](artifacts/atomic-loops.json)

## Which atomic operation appears in the two gather closures?

Both closures store a Rayon stack job into a worker deque and then test bit 32 of the same counter layout. The conditional path can set that bit with compare-exchange. Both closures subsequently test the low sixteen bits for sleepers, inspect the next sixteen bits for inactive workers and can call the named function `Sleep::wake_any_threads(1)`. These instructions match the job-availability notification in Rayon Core 1.13.0. Its worker push calls `new_internal_jobs`, which checks the pool's job event counter after publishing each job. This attribution narrows the bounded-task experiment to a specific scheduler operation. The archived samples cannot quantify actual wakeups, unsuccessful retries or time recoverable by changing task count. [Instructions and package identity](artifacts/job-notification.json)

## Does the previous proof's late G2 work predict the next solver duration?

Within a recording, requests 2 through 10 serve as the nine solver observations. Each observation is classified three ways using either the prior, current or next request's distribution of sampled G2 prefix work. “Late” means most prefix samples occurred after H returned. Every comparison uses the same nine target requests. The table shows how much longer the median solver lasted in the late group for each individual process. A future label cannot cause the earlier solver. Each group can contain very few requests, so the process contrasts are descriptive.

| Shape | Cores | G2 label used for solver | Late/early observations per block | Late minus early solver ms per block |
|---|---:|---|---|---|
| 16×2 | 64 | Prior proof | 3/6, 2/7, 3/6 | -0.040 / -0.043 / +0.221 |
| 16×2 | 64 | Same proof | 2/7, 2/7, 3/6 | -0.014 / +0.015 / -0.276 |
| 16×2 | 64 | Next proof | 1/8, 2/7, 3/6 | -0.183 / -0.049 / +0.298 |
| 16×2 | 96 | Prior proof | 2/7, 5/4, 6/3 | +0.158 / +0.029 / +0.197 |
| 16×2 | 96 | Same proof | 1/8, 4/5, 6/3 | +0.196 / -0.012 / +0.187 |
| 16×2 | 96 | Next proof | 1/8, 3/6, 5/4 | -0.293 / -0.111 / -0.228 |
| Merge-24×1 | 64 | Prior proof | 3/6, 6/3, 2/7 | +0.085 / +0.234 / +0.166 |
| Merge-24×1 | 64 | Same proof | 2/7, 5/4, 2/7 | +0.207 / +0.112 / -0.046 |
| Merge-24×1 | 64 | Next proof | 2/7, 4/5, 2/7 | -0.388 / +0.020 / +0.115 |
| Merge-24×1 | 96 | Prior proof | 7/2, 6/3, 2/7 | -0.143 / +0.318 / -0.471 |
| Merge-24×1 | 96 | Same proof | 6/3, 6/3, 2/7 | -0.143 / -0.344 / +0.162 |
| Merge-24×1 | 96 | Next proof | 6/3, 5/4, 2/7 | -0.202 / +0.344 / -0.366 |

[Request-level intervals](artifacts/requests.json) · [Independent trace reader](artifacts/read.rs) · [Source identities](../2026-11-18-c7a-worker-attribution381/artifacts/inputs.json) · [G2 sample-placement analysis](../2026-11-19-c7a-tail-placement558/README.md)
