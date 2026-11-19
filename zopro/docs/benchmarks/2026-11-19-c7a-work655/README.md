# Why the historical 96-core gap needs both less CPU work and better parallelism

On the 20 shapes where the published 96-core comparison favored Aeglos, Zopro used more CPU-ms per proof on 17. It occupied fewer cores on all 20. On 5×8, 5×16 and merge 8×1, Zopro used less aggregate CPU time and still finished later. The largest relative deficit was 1×16. These two patterns make 1×16 and 5×16 useful additional cases alongside 1×2 and the large merge in a fresh same-host comparison.

Holding CPU work fixed and keeping all 96 cores busy would still fall short of a twofold improvement on every one of the 20 shapes. The measured work would have to fall by at least 12.8–35.2% even under that ideal scheduling assumption. Reaching only Aeglos parity would also fall short of 2× on every historical loss. The actual target must be recomputed from the **current release's** independently verified C7a latency.

| Historical loss, 96 cores | Zopro / Aeglos warm ms | Zopro / Aeglos CPU-ms ratio | Mean busy cores, Zopro / Aeglos | Half historical Zopro ms | Minimum CPU-work cut for 2× at 96 fully busy cores |
|---|---:|---:|---:|---:|---:|
| 1x16 | 21.02 / 12.42 | 1.39× | 58.6 / 71.7 | 10.51 | 18.1% |
| 1x2 | 12.69 / 8.24 | 1.33× | 55.1 / 63.6 | 6.35 | 12.8% |
| 1x4 | 16.15 / 10.01 | 1.40× | 57.5 / 67.9 | 8.07 | 16.5% |
| 1x8 | 17.63 / 10.82 | 1.43× | 59.8 / 68.3 | 8.81 | 19.7% |
| 2x16 | 24.33 / 17.13 | 1.25× | 65.2 / 74.2 | 12.17 | 26.3% |
| 2x2 | 17.17 / 12.01 | 1.21× | 57.7 / 69.2 | 8.59 | 16.9% |
| 2x4 | 17.68 / 12.45 | 1.18× | 57.7 / 69.5 | 8.84 | 16.8% |
| 2x8 | 20.32 / 15.29 | 1.10× | 61.7 / 76.1 | 10.16 | 22.1% |
| 3x2 | 19.83 / 16.15 | 1.11× | 67.3 / 75.0 | 9.91 | 28.7% |
| 3x4 | 20.89 / 16.55 | 1.09× | 65.7 / 75.7 | 10.44 | 26.9% |
| 3x8 | 21.85 / 17.68 | 1.13× | 67.4 / 74.1 | 10.92 | 28.8% |
| 4x16 | 26.44 / 21.66 | 1.11× | 67.9 / 74.4 | 13.22 | 29.3% |
| 4x2 | 21.64 / 18.40 | 1.05× | 69.2 / 77.0 | 10.82 | 30.7% |
| 4x4 | 22.92 / 19.02 | 1.10× | 67.4 / 74.2 | 11.46 | 28.8% |
| 4x8 | 24.07 / 19.69 | 1.12× | 68.0 / 75.3 | 12.04 | 29.4% |
| 5x16 | 29.80 / 27.91 | 0.91× | 70.3 / 82.9 | 14.90 | 31.7% |
| 5x2 | 23.89 / 21.14 | 1.09× | 71.7 / 72.9 | 11.95 | 33.0% |
| 5x4 | 24.69 / 21.38 | 1.08× | 70.2 / 74.4 | 12.34 | 31.6% |
| 5x8 | 27.69 / 26.56 | 0.93× | 71.6 / 80.4 | 13.84 | 33.0% |
| merge-8x1 | 30.92 / 30.25 | 0.92× | 74.1 / 83.0 | 15.46 | 35.2% |

Each row summarizes nine paired processes, each timing seven warm requests. Wall time is the measured loop duration divided by seven. CPU work is process user plus system time per request. Mean busy cores are CPU work divided by elapsed time. For each block the wall-time ratio is exactly the CPU-work ratio times the inverse busy-core ratio. Each table column is then summarized by its own median. For wall time T and work W, finishing by T/2 on 96 cores permits at most 48T CPU-ms; the minimum fractional reduction in measured work is max(0, 1 − 48T/W). Measured work includes scheduler overhead and on-CPU stalls. The arithmetic alone cannot distinguish useful instructions, memory stalls, spinning or lower core frequency.

In an archived matched 1×2 run the 32-to-96 increase added 1.82 ms to Zopro solving, preparation and FFT while post-FFT query execution saved 0.39 ms. Aeglos's MSM saved 4.52 ms. On a later build, the 96-core 1×2 gap narrowed to 8.060 versus 7.943 ms. The earlier catalog is a guide for instrumentation and cannot substitute for a new release baseline. [Matched phase clocks](../2026-11-18-c7a-phase251/README.md) · [Later focused comparison](../2026-11-18-overlap-c7a369/README.md)

The next same-host screen should measure ordinary warm latency with instrumented runs in separate processes, plus cumulative user/system time, minor and major faults, per-stage elapsed time and per-thread curve and scheduler samples. Start with the two distinct loss patterns and a large-shape control. A faster current median against the prior release and an improvement against Aeglos answer different questions. A claimed 2× speedup requires fresh paired full proofs against the current baseline, held-out requests and reported peak RSS. [Catalog and classification](../2026-11-18-full-catalog-c7a244/README.md) · [Fault and scheduler counters](../2026-11-19-catalog-counters615/README.md) · [All 41 paired calculations](artifacts/paired-work.json.gz)
