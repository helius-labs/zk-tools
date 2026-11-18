# Why moving witness work earlier slowed 24×2

On a Ryzen 9950X with 16 pinned workers, starting the witness curve queries alongside the FFT increased its mean elapsed time from 23.69 to 49.06 ms. The dependent H query finished 8.64 ms faster on average, too little to recover the extra FFT time. Moving only scalar grouping into the FFT also increased total proof time. The original schedule kept H and the witness queries running together in every traced request.

| Schedule | Plain proof ms | FFT ms | H query ms | After H ms | H started after all three witnesses | Trace/plain |
|---|---:|---:|---:|---:|---:|---:|
| Original schedule | 75.28 | 23.69 | 38.65 | 3.71 | 0/51 | 1.012 |
| Full witness overlap | 90.96 | 49.06 | 30.01 | 0.83 | 50/51 | 0.993 |
| Grouping overlap | 85.52 | 30.35 | 39.16 | 6.57 | 0/51 | 1.007 |

Plain proof latency is the median of three uninstrumented process means. Stage times are means across 51 traced proofs per schedule. “After H” runs from H return to SDK return. Each trace captured all three witness groupings, the G1 and G2 calls, the FFT and the enclosing proof. In 50 of 51 full-overlap traces all three witness queries finished before H started. Trace/plain ratios are paired medians. Every arm met the declared 5% observer tolerance.

H had 554,226 points in every trace. The protocol predicted a power-of-two length. The Euler product uses the active constraint rows of the 1,048,576-point FFT domain. Both input slice lengths agreed at every raw call, the two grouped G1 calls had 43,958 and 66,488 points, and H always started after the FFT. [Fixture census](../2026-10-24-derivative-basis-study/README.md)

Three fresh processes per schedule and observer produced 306 timed requests after 54 primes. All 18 exported proofs verified against the pinned witness and key. These intervals locate the local latency change. They do not distinguish cache contention, worker scheduling, or memory bandwidth, and the AWS kernels were not traced in this experiment. The original schedule measured 7.01× the supplied M5 CPU speed and 2.34× Metal on this local panel. [Process and interval data](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Trace reader](artifacts/analyze.rs)
