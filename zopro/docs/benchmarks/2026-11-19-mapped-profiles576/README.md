# Warm CPU profiles for the ordinary and preconverted merge keys

Three randomized blocks per input compare the same executable with ordinary and mapped public-origin key images on 32 Ryzen workers. Every process primes three proofs and uses acknowledged FIFOs to enable perf counters or cycle sampling only for the next 31 complete requests. Counter coverage must reach 95% for the measured hardware events. All exported proofs use the original verifier and independently known public inputs.

The configured allocator settings were passed through to the measured process. An initial pilot that omitted those settings was excluded before restarting the profile series.

The named G1, G2, FFT, origin and runtime categories cover recognizable instruction-pointer symbols only. Each sample counts once and the remaining symbols are retained in the artifacts. Inlined code may appear under its caller or a Rayon symbol. Process wall times are measured with the indicated observer enabled.

| Request | Key | Counter run ms | Million cycles / instructions per proof | Million cache misses | Minor / major faults | G1 / G2 / FFT / origin / runtime samples |
|---|---|---:|---:|---:|---:|---:|
| Original | ordinary | 62.859 | 8301.7 / 7892.8 | 47.02 | 2797 / 0 | 8.0% / 4.4% / 0.0% / 0.0% / 3.8% |
| Original | mapped | 62.805 | 8075.6 / 6548.4 | 39.94 | 3499 / 0 | 5.4% / 4.6% / 0.0% / 0.0% / 4.4% |
| 109 | ordinary | 68.241 | 8640.1 / 8229.4 | 45.55 | 7142 / 0 | 7.5% / 4.9% / 0.0% / 0.0% / 4.1% |
| 109 | mapped | 64.459 | 7944.5 / 6320.1 | 35.32 | 3351 / 0 | 4.9% / 5.3% / 0.0% / 0.0% / 5.1% |
| 122 | ordinary | 67.784 | 8725.3 / 8292.4 | 46.41 | 3502 / 0 | 7.4% / 5.0% / 0.0% / 0.0% / 4.0% |
| 122 | mapped | 68.577 | 8524.9 / 7282.7 | 41.41 | 4492 / 0 | 5.8% / 5.1% / 0.0% / 0.0% / 4.5% |

All 36 proof exports verified. [Counters and complete symbol counts](artifacts/processes.json.gz) · [Leading symbols](artifacts/symbols.json) · [Verification](artifacts/verification.json)
