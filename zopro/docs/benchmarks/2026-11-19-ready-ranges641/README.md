# G2 bucket handoff for two independent ranges

The flat schedules use one task for each window/range pair. The second finishing task performs its window's merge and weighted reduction. Every case has eight balanced process blocks per arm and one CPU profile per arm. All 157 native core tests passed and all 186 exported proofs verified.

The ordinary release had the lowest median latency in all five cases. On the original 1×2 at 32 workers, the traced G2 query spanned 4.53–4.72 ms across the two schedules, with 0.17–0.18 ms left after the last bucket fill. Fill tasks started over a 2.69–2.70 ms interval. On 2×2 the corresponding start spread was 0.01–0.23 ms. These are within-query measurements on the local machine. They give priority to the fills and their competition with other proof work over optimizing only the final reduction.

| Input and workers | Release / nested / window-major / range-major ms | Range-major vs window-major / nested / release | Peaks in same order, MiB | Range-major vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2-0-w32 | 7.019 / 7.255 / 7.143 / 7.201 | +0.57% (2/8) / -1.04% (5/8) / +2.48% (1/8) | 60.2 / 62.5 / 60.0 / 61.5 | 5.14× / 3.33× |
| 1x2-0-w16 | 8.268 / 8.711 / 8.650 / 8.675 | -0.44% (5/8) / -0.51% (8/8) / +4.02% (0/8) | 56.8 / 56.8 / 57.0 / 56.2 | 4.26× / 2.77× |
| 1x2-29-w32 | 6.982 / 7.195 / 7.173 / 7.136 | -0.93% (6/8) / -0.76% (6/8) / +2.26% (0/8) | 58.8 / 58.8 / 58.5 / 58.0 | — |
| 1x2-32-w32 | 6.983 / 7.126 / 7.217 / 7.134 | -1.15% (7/8) / +0.06% (4/8) / +2.50% (0/8) | 57.8 / 58.5 / 58.5 / 58.2 | — |
| 2x2-0-w32 | 9.587 / 9.978 / 9.928 / 9.936 | +0.30% (3/8) / -0.51% (6/8) / +2.92% (0/8) | 112.0 / 112.2 / 109.5 / 109.2 | 5.94× / 3.02× |

[Protocol](protocol.md) · [All paired comparisons](artifacts/summary.json) · [Diagnostic events](artifacts/traces.json.gz) · [Samples and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)

## Range completion timelines

These separate dispatch probes time one G2 query from its earliest task start to its last weighted window completion. They include all three priming proofs and one measured proof per process. Finish-to-finish intervals use the same monotonic clock across workers. Records were emitted after the parallel iterator returned. The full-proof timings above were collected with recording disabled.

| Input and workers | Order | Query observations | Median span ms | Median spread of fill start times ms | Median remaining after final fill ms | Median last window reduction ms | Last window indices in execution order |
|---|---|---:|---:|---:|---:|---:|---|
| 1x2-0-w32 | window | 4 | 4.717 | 2.693 | 0.182 | 0.181 | 7, 6, 3, 9 |
| 1x2-0-w32 | range | 4 | 4.534 | 2.696 | 0.174 | 0.174 | 4, 4, 4, 9 |
| 1x2-0-w16 | window | 4 | 6.109 | 4.562 | 0.032 | 0.031 | 4, 10, 10, 10 |
| 1x2-0-w16 | range | 4 | 5.976 | 4.514 | 0.081 | 0.080 | 10, 10, 7, 7 |
| 2x2-0-w32 | window | 4 | 2.424 | 0.014 | 0.232 | 0.237 | 5, 2, 10, 1 |
| 2x2-0-w32 | range | 4 | 2.367 | 0.226 | 0.193 | 0.220 | 9, 3, 4, 10 |

[Per-range timestamps](artifacts/timelines.json.gz)

## Concurrent work inside the G2 query

Each fill interval runs from the start of a range task to its completed bucket array. The table reports the median across four diagnostic queries of the average number of overlapping intervals. The final quarter is the last 25% of each query's measured span. Other prover tasks can run concurrently, so these counts cover only the observed G2 query. The simultaneous-start estimate holds each recorded fill and reduction duration fixed and starts all range fills together. Finite workers and shared execution resources limit what a scheduler can achieve.

| Input and order | Mean active fills | Active fills in final quarter | Active reductions in final quarter | Measured span ms | Simultaneous-start estimate ms |
|---|---:|---:|---:|---:|---:|
| 1x2-0-w32-window | 9.37 | 6.01 | 1.83 | 4.72 | 4.10 |
| 1x2-0-w32-range | 8.99 | 5.71 | 1.61 | 4.53 | 3.80 |
| 1x2-0-w16-window | 4.51 | 4.66 | 1.58 | 6.11 | 2.60 |
| 1x2-0-w16-range | 4.64 | 5.33 | 1.42 | 5.98 | 2.49 |
| 2x2-0-w32-window | 16.20 | 3.08 | 3.35 | 2.42 | 2.42 |
| 2x2-0-w32-range | 16.69 | 3.97 | 3.52 | 2.37 | 2.36 |

[Per-query interval analysis](artifacts/range-occupancy.json)

## Worker distribution in the ordinary release

Three independently recorded release profiles per case use the same binary and a 397 Hz sampling frequency. The table gives the median across recordings. For each named function, a worker's share is its sampled cycles divided by all sampled cycles in that function. The effective worker count is one divided by the sum of squared shares. A larger value indicates that more threads shared the sampled work. The overlap column sums the smaller of the two function shares on each thread. These statistics describe work over the whole recording. They do not measure simultaneous execution or time spent waiting.

| Reference | G1 stream / G2 borrowed update, % of sampled cycles | G1 / G2 effective worker count | Worker-share overlap |
|---|---:|---:|---:|
| 1×2, 16 workers | 25.57 / 17.85 | 11.53 / 10.68 | 61.0% |
| 1×2, 32 workers | 20.95 / 20.69 | 16.41 / 13.51 | 45.5% |
| 2×2, 32 workers | 24.44 / 12.19 | 20.24 / 18.13 | 57.2% |

At 32 workers the 1×2 G2 update had a more concentrated distribution than on 2×2. The traces and profiles use different executables. Relating the narrower distribution to fill launch delays requires a synchronized measurement on the ordinary schedule. Each profile contained 59–97 samples in the named G2 update. Per-recording totals include the sampling period, the thread identity, the independent proof digest and the sample counts for both functions. [Release profile breakdown](artifacts/release-worker-samples.json)
