# Rechecking the Euler SDK comparison on Ryzen

The [fused input and output experiment](../2026-11-20-euler-fused-ends-ryzen912/README.md) measured 13.43% and 13.28% higher 1×2 latency for the fresh bridge control on its two inputs, and 8.74% and 7.96% higher 2×2 latency. The fresh and frozen bridge controls request the same FFT implementation. This follow-up rechecks the exact two executables, catalog requests, core masks, proving keys and environment settings. The compiler and build flags match. The 4×2 shape provides a comparison where the large initial gap was absent.

Three new paired blocks per shape compare the two executables at 19 and 151 timed requests in unprivileged direct SDK processes. Three further paired blocks use the Linux performance counters under sudo with 151 timed requests. Every process primes three proofs. Latency is the median of process means. The paired changes use the median within-block ratio. The 19-call comparison on 1×2 and 2×2 reproduces only 1.67% and 0.53% of relative latency increase, respectively, and the 151-call comparison shows 0.93% and 0.82%. The large original control discrepancy did not reproduce. The available evidence leaves its cause unresolved.

| Shape | Capture | Timed proofs per process | Frozen bridge ms | Fresh bridge ms | Paired change and wins |
|---|---|---:|---:|---:|---:|
| 1x2 | Counters | 151 | 6.846 | 6.958 | +1.14% (0/3) |
| 1x2 | Plain | 19 | 6.817 | 6.931 | +1.67% (1/3) |
| 1x2 | Plain | 151 | 6.871 | 6.906 | +0.93% (0/3) |
| 2x2 | Counters | 151 | 9.339 | 9.354 | +0.26% (0/3) |
| 2x2 | Plain | 19 | 9.334 | 9.368 | +0.53% (1/3) |
| 2x2 | Plain | 151 | 9.311 | 9.387 | +0.82% (0/3) |
| 4x2 | Counters | 151 | 13.682 | 13.701 | -0.19% (2/3) |
| 4x2 | Plain | 19 | 13.639 | 13.898 | +1.43% (0/3) |
| 4x2 | Plain | 151 | 13.632 | 13.684 | -0.23% (2/3) |

The counters span the entire process, including setup and priming. User cycle counts include all workers. Retired instructions differ by less than 0.05% on every shape. These totals do not localize a stage or establish a cause.

| Shape | User cycles | User instructions | Context switches |
|---|---:|---:|---:|
| 1x2 | +2.504% | +0.045% | +5.018% |
| 2x2 | +0.921% | -0.015% | -2.215% |
| 4x2 | +0.458% | -0.023% | -0.600% |

All 54 timed exports independently verified. A separate frozen 1×2 sampling run also exported a verified proof. Its symbol reader omitted the address field and produced empty symbol rows. The diagnosis retains that proof receipt and ends the incomplete profiling arm because the large gap did not reproduce in the paired counter runs. The original candidate latency panel remains as measured. This follow-up compares the bridge controls only and does not qualify either endpoint variant for adoption.

[Measurements and counter logs](artifacts/runs.json.gz) · [Paired summary](artifacts/summary.json) · [Verification](artifacts/verification.json)
