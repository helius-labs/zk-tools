# A top-limb quotient for the four-input inverse butterfly

For the sum S of four R260 field values below 2p, let T = 2²⁰⁸ and t be the sum of their top radix-52 digits without carry. Then tT ≤ S < (t+4)T. Using u = floor(p/T) and k = floor(2⁵²/(u+1)), q = floor(tk/2⁵²) is a lower estimate of S/p. Before truncation, its error is less than 4/u + t/(u(u+1)) + t/2⁵² < 1 for t < 8(u+1). Consequently S−qp lies in [0,2p). The candidate subtracts qp during the carry pass. The frozen and ordinary controls perform the two conditional modular reductions from the earlier four-input butterfly. The difference-times-root result uses the same arithmetic in all three modes. The candidate uses the existing allocations and root tables.

The independent integer model checks the bound, the exact signed carries, all eight attainable estimates and input boundaries. The native field and transform oracles run before building the benchmark. Five randomized blocks compare the frozen grouped build, a new ordinary build and the new four-input path on all eight original requests and two additional public requests. Each process uses 32 Ryzen workers, three warmups and eleven timed proofs, or 31 on the two smallest shapes. The 12×2 and 24×2 profiles use fixed prepared groups across all arms.

| Shape | Request | Frozen / ordinary / candidate ms | Paired vs ordinary | Paired vs frozen | Peak MiB ordinary / candidate | Candidate vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.136 / 7.101 / 7.078 | -0.29% (3/5) | -1.04% (3/5) | 64.2 / 63.5 | 5.23× / 3.39× |
| 2x2 | Original | 9.711 / 9.592 / 9.617 | -0.43% (3/5) | -1.71% (3/5) | 120.2 / 114.2 | 6.13× / 3.12× |
| 4x2 | Original | 15.601 / 15.601 / 15.713 | +1.08% (1/5) | +1.65% (1/5) | 264.8 / 261.2 | 6.94× / 3.05× |
| 8x2 | Original | 24.753 / 24.248 / 25.144 | +3.70% (1/5) | +2.14% (2/5) | 432.0 / 427.8 | 7.52× / 2.94× |
| 12x2 | Original | 29.360 / 29.434 / 29.495 | +0.29% (2/5) | -0.53% (3/5) | 536.2 / 544.4 | 9.63× / 3.42× |
| 16x2 | Original | 40.598 / 39.561 / 40.207 | -0.88% (3/5) | +1.52% (1/5) | 682.1 / 663.1 | 8.33× / 3.08× |
| 24x2 | Original | 59.499 / 59.657 / 59.974 | +0.69% (2/5) | -0.34% (3/5) | 1103.1 / 1121.7 | 8.80× / 2.93× |
| merge-24x1 | Original | 63.996 / 63.626 / 63.495 | +0.01% (2/5) | -0.57% (3/5) | 1041.0 / 1044.2 | 7.98× / 2.76× |
| 12x2 | 142 | 45.276 / 45.468 / 45.782 | +0.74% (2/5) | +1.12% (1/5) | 957.9 / 1030.5 | — |
| 24x2 | 103 | 58.306 / 58.279 / 58.271 | -1.36% (3/5) | +0.32% (2/5) | 1115.1 / 1113.1 | — |

All 150 process exports verified. [Samples](artifacts/timings.json.gz) · [Source and oracle](artifacts/source.patch) · [Verification](artifacts/verification.json)
