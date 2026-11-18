# Request preparation and core proving time on all eight supplied shapes

The same experimental 257 SDK used in the C7a experiments ran on 16 local Ryzen cores. Each row covers three plain and three traced processes, with three primes and 11 timed proofs per process. The latency comparison below uses uninstrumented process means. M5 times are the supplied references.

The core prover accounted for **94.2% of SDK time on 1×2** and **98.3% on merge24**. Constructing the ordered witness from a request took 0.078–0.808 ms across the eight shapes. On 24×2 and merge24, the FFT inside the core prover alone took about 24 ms. These timings put the largest opportunity for the overall 10× goal inside the prover.

| Shape | Zopro ms | 10× target ms | Above target ms | M5 CPU speedup | M5 Metal speedup | Peak MiB | Paired trace overhead | Calibration |
|---|---:|---:|---:|---:|---:|---:|---:|---|
| 1x2 | 8.628 | 3.7 | +4.93 | 4.29× | 2.78× | 52 | +0.95% | pass |
| 2x2 | 11.964 | 5.9 | +6.06 | 4.93× | 2.51× | 156 | -1.63% | pass |
| 4x2 | 16.832 | 10.9 | +5.93 | 6.48× | 2.85× | 211 | +0.87% | pass |
| 8x2 | 26.669 | 18.9 | +7.77 | 7.09× | 2.77× | 356 | -0.15% | pass |
| 12x2 | 38.942 | 28.4 | +10.54 | 7.29× | 2.59× | 518 | +1.84% | pass |
| 16x2 | 45.726 | 33.5 | +12.23 | 7.33× | 2.71× | 592 | +1.30% | pass |
| 24x2 | 74.769 | 52.8 | +21.97 | 7.06× | 2.35× | 966 | +2.35% | pass |
| merge-24x1 | 73.835 | 50.7 | +23.14 | 6.87× | 2.37× | 864 | +1.29% | pass |

## Measured SDK intervals

| Shape | SDK witness call, ms | Before core, ms | Core prover, ms | Core share | After core, ms | Solver within core, ms | FFT within core, ms | G2 raw symbol, ms |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 0.078 | 0.084 | 8.357 | 94.2% | 0.410 | 1.005 | 0.487 | 5.935 |
| 2x2 | 0.142 | 0.151 | 11.198 | 94.9% | 0.454 | 1.568 | 1.143 | 4.021 |
| 4x2 | 0.206 | 0.219 | 16.212 | 95.7% | 0.518 | 1.982 | 2.328 | — |
| 8x2 | 0.332 | 0.348 | 25.763 | 96.7% | 0.545 | 3.007 | 4.582 | — |
| 12x2 | 0.489 | 0.512 | 37.845 | 97.3% | 0.509 | 3.585 | 9.778 | — |
| 16x2 | 0.578 | 0.601 | 45.104 | 97.6% | 0.516 | 4.754 | 10.251 | — |
| 24x2 | 0.808 | 0.838 | 74.785 | 98.2% | 0.507 | 7.111 | 23.669 | — |
| merge-24x1 | 0.684 | 0.710 | 73.552 | 98.3% | 0.549 | 8.099 | 23.919 | — |

Before core includes request parsing, constructing the ordered witness, and encoding. Core prover covers the call to solve_rerandomized. After core includes decoding, verifying and serializing the resulting proof. The solver and FFT columns are intervals measured inside the core call. The G2 column identifies the raw_exclusive implementation when present; a dash indicates another route. Columns use medians of three process-level means. Individual nested stages cannot be added to the inclusive core time. The calibration limit was ±5% for each shape.

All 48 process exports passed independent Gnark verification. Every one of 264 traced requests produced the expected witness, core, solver and FFT call boundaries. [Block and request data](artifacts/blocks.json.gz) · [Scores](artifacts/scores.json) · [Protocol](artifacts/protocol.md) · [Probe offsets](artifacts/targets.tsv) · [Verification](artifacts/verification.json)

## Peak RSS measurement

The original Python trace driver hashed large fixtures in memory before spawning the SDK. Several small-shape processes inherited a spurious 377 MiB high-water mark across exec. The table uses a separate, clean shell launcher with GNU time and a fresh SDK child for each of 24 additional runs. The SDK and GNU time high-water marks agreed to within 4 MiB in each process. The original latency runs and raw high-water values remain in the block file. All 24 additional proof exports passed Gnark verification. [Clean RSS runs](artifacts/rss.json)
