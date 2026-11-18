# Independent effects of frontend width and G2 scalar grouping on C7a

At 96 physical cores on one c7a.24xlarge, accepted Zopro took **12.37 ms**, the combined prototype **10.07 ms**, and Aeglos **7.90 ms** on warm, verified 1×2 proofs. The prototype took 18.4% less wall time and 34.4% less CPU than accepted Zopro in the median paired block. Aeglos finished first in all seven blocks. The prototype used less aggregate CPU time and peaked at 70.7 MiB RSS, versus 479.1 MiB for Aeglos.

## Independent changes

The frontend limit uses 32 workers during the canonical solve, preparation and FFT, then releases all workers for the curve queries. The second change runs a bounded scalar-diversity check alongside the FFT. A successful check establishes that G2 can skip a grouping map that it would later discard. Both switches use the same experimental executable. [Implementation and nine-shape test](../2026-11-18-early-diversity257/README.md)

| Engine or experimental switches | Cores | Verified wall ms | CPU ms/proof | Peak process RSS MiB |
|---|---:|---:|---:|---:|
| Accepted Zopro | 96 | 12.37 | 690 | 117.3 |
| Prior width-only prototype, experiment 255 | 96 | 11.52 | 484 | 117.4 |
| Both switches off | 96 | 12.26 | 624 | 113.0 |
| Frontend limit only | 96 | 11.50 | 484 | 120.2 |
| Diversity check only | 96 | 10.68 | 595 | 64.8 |
| Both enabled | 96 | 10.07 | 454 | 70.7 |
| Both enabled, 32 cores available | 32 | 10.66 | 231 | 70.7 |
| Aeglos | 96 | 7.90 | 521 | 479.1 |

Each latency and CPU value is the median of seven process means. Peak RSS is the largest process high-water mark observed in that arm, including setup. Each block used independent executable backings, rotated or reversed run order and 151 timed requests per arm after three primes. The two control comparisons passed the factor-of-1.03 wall calibration. CPU differed between accepted Zopro and the experimental executable with both switches off. Same-binary comparisons below isolate the switches.

| Change at 96 cores | Paired wall reduction | Paired CPU reduction | Blocks with lower latency |
|---|---:|---:|---:|
| Enable diversity check, full frontend width | 13.3% | 4.8% | 7/7 |
| Enable diversity check, limited frontend width | 13.7% | 6.6% | 7/7 |
| Limit frontend width, original grouping | 6.7% | 22.7% | 7/7 |
| Limit frontend width, diversity check enabled | 5.5% | 23.5% | 7/7 |

The diversity check works independently of changing worker participation. The median multiplicative interaction was 1.0002 across seven blocks. With both features enabled, 96 cores reduced paired mean latency by 5.7% versus 32 cores and consumed 1.98× the CPU per request. Relative to the supplied 37 ms M5 CPU and 24 ms Metal times, the 96-core prototype achieved 3.67× and 2.38×. This run covers one shape. [Per-block factors](artifacts/analysis.json) · [Frozen protocol](artifacts/protocol.md)

## Two latency groups on 96 cores

Individual request times reveal a cost that the means conceal. With both switches enabled, **759 of 1,057 proofs took under 10 ms, four took 10 to under 11 ms, and 294 took at least 11 ms**. The slow group appeared in all seven blocks. All timed requests certified and the timed loops recorded zero major page faults. The earlier experiment on a different C7a instance independently showed 324, three and 126 proofs in the same intervals. These thresholds describe the recordings and were chosen after inspection.

| Pooled warm 1×2 latency | Median ms | p95 ms | p99 ms | Within 9 ms |
|---|---:|---:|---:|---:|
| Accepted Zopro, 96 cores | 12.21 | 13.44 | 13.69 | 0/1,057 |
| Both switches, 96 cores | 9.34 | 12.67 | 12.95 | 67/1,057 |
| Both switches, 32 cores | 10.62 | 11.34 | 11.56 | 0/1,057 |
| Aeglos, 96 cores | 7.83 | 8.35 | 8.81 | 1,052/1,057 |

The 96-core candidate has lower median latency and higher p95 than the 32-core candidate. Aeglos's MSM averaged 3.80 ms in the median process. The existing accepted-build trace found all 22 G2 fills started within 0.033 ms on average. The combined candidate has yet to be traced per request. Timing FFT completion, scalar-check completion, G2 fill and query return on the same proof would distinguish late work before the queries from slow bucket processing or a late join. A comparison of the two latency groups should retain worker IDs, check observer overhead and verify the split survives instrumentation. [Latency histogram](artifacts/distribution.json) · [Earlier-instance counts](artifacts/replication.json) · [Accepted-build G2 trace](../2026-11-18-g2-interior256/README.md)

All 64 exported proofs passed Gnark on the AWS host and again after retrieval, covering 8,464 timed requests and 8,656 calls including primes. Only one proof per process was exported for independent verification. The cumulative ledger reached 65,133 distinct hashes. The instance terminated, the tagged EBS query returned no volumes and the temporary bucket was removed. Production adoption and concurrent-server qualification remain open. [Verification](artifacts/verification.json) · [Per-process rows](artifacts/host-summary.json) · [Executable digests](artifacts/binaries.sha256)
