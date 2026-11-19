# Euler stage timing inside verified proofs

The preceding eight-shape screen selected one radix policy per shape by the paired median against its disabled control. Four new randomized blocks compare that policy with ordinary radix conversion using both the plain prover and a timing build. All requests use 32 Ryzen workers. Each 12×2 request uses the same nested B2 plan, and the 24×2 requests use their fixed adaptive plan. The varied transactions were excluded from the preceding screen. The SDK warms three proofs before timing and the trace records only durations.

The local-center timings split first inverse columns, the interior transform and weighting, final forward columns, and product combination. The ordinary route splits the inverse including packing, weighting, forward transform, and product combination. Total Euler time spans those four stages. It excludes the subsequent curve queries. The final column reports the tracing build's paired change against the uninstrumented candidate.

| Shape | Request | Policy | Plain off / candidate ms | Plain paired change | Total Euler off / candidate ms | Product combine off / candidate ms | Trace effect |
|---|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | product | 7.005 / 6.972 | -0.47% (3/4) | 0.560 / 0.548 | 0.038 / 0.028 | +0.96% |
| 12x2 | Original | derivative | 29.257 / 29.395 | +0.47% (2/4) | 8.966 / 8.834 | 0.825 / 0.799 | +1.10% |
| 24x2 | Original | derivative | 59.683 / 59.578 | +0.13% (2/4) | 20.279 / 20.353 | 2.255 / 2.287 | -1.97% |
| 1x2 | 26 | product | 6.964 / 6.938 | -0.39% (3/4) | 0.553 / 0.546 | 0.037 / 0.028 | +0.46% |
| 1x2 | 31 | product | 6.937 / 6.964 | +0.06% (2/4) | 0.555 / 0.551 | 0.037 / 0.028 | +0.32% |
| 12x2 | 58 | derivative | 29.322 / 29.256 | -0.22% (2/4) | 8.945 / 8.783 | 0.814 / 0.808 | +1.33% |
| 12x2 | 142 | derivative | 45.664 / 45.630 | +0.25% (2/4) | 8.543 / 8.710 | 0.822 / 0.790 | +0.27% |
| 24x2 | 103 | derivative | 57.309 / 57.645 | +0.73% (1/4) | 20.149 / 20.613 | 2.242 / 2.311 | +0.43% |

Original-input speedups against M5 CPU / Metal: 1x2 5.31× / 3.44×, 12x2 9.66× / 3.44×, 24x2 8.86× / 2.95×. All 128 exports verified independently. [Samples and stages](artifacts/processes.json.gz) · [Timing change](artifacts/trace.patch) · [Verification](artifacts/verification.json)
