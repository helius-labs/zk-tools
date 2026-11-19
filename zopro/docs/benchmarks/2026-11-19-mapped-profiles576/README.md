# Warm CPU profiles for the ordinary and preconverted merge keys

Three randomized blocks per input compare the same executable with ordinary and mapped public-origin key images on 32 Ryzen workers. Every process primes three proofs and uses acknowledged FIFOs to enable perf counters or cycle sampling only for the next 31 complete requests. Counter coverage must reach 95% for the measured hardware events. All exported proofs use the original verifier and independently known public inputs.

The configured allocator settings were passed through to the measured process. An initial pilot that omitted those settings was excluded before restarting the profile series.

Cycle samples are grouped by their demangled instruction-pointer symbol. Rayon iterator wrappers can contain inlined arithmetic and are reported separately from Rayon Core and Crossbeam. Shared field kernels and unknown addresses remain in the other category. The table uses the same 18 counter and 18 sampling processes for both latency and CPU analysis. The raw symbols are available for reclassification. [Symbol classifier](artifacts/classify_symbols.py)

| Request | Key | Counter run ms | Million cycles / instructions per proof | Million cache misses | Minor / major faults | G1 / G2 / FFT / origin / Rayon iterators / runtime / other samples |
|---|---|---:|---:|---:|---:|---:|
| Original | ordinary | 62.859 | 8301.7 / 7892.8 | 47.02 | 2797 / 0 | 28.6% / 7.2% / 28.8% / 0.0% / 13.8% / 6.8% / 14.9% |
| Original | mapped | 62.805 | 8075.6 / 6548.4 | 39.94 | 3499 / 0 | 20.9% / 7.6% / 30.4% / 0.3% / 16.9% / 8.2% / 15.7% |
| 109 | ordinary | 68.241 | 8640.1 / 8229.4 | 45.55 | 7142 / 0 | 27.8% / 7.8% / 27.6% / 0.0% / 15.1% / 7.0% / 14.6% |
| 109 | mapped | 64.459 | 7944.5 / 6320.1 | 35.32 | 3351 / 0 | 18.0% / 8.5% / 30.3% / 0.3% / 18.9% / 8.9% / 15.0% |
| 122 | ordinary | 67.784 | 8725.3 / 8292.4 | 46.41 | 3502 / 0 | 28.1% / 8.0% / 27.3% / 0.0% / 15.4% / 6.8% / 14.5% |
| 122 | mapped | 68.577 | 8524.9 / 7282.7 | 41.41 | 4492 / 0 | 22.6% / 8.1% / 28.4% / 0.2% / 18.0% / 8.0% / 14.7% |

All 36 proof exports verified. [Counters and complete symbol counts](artifacts/processes.json.gz) · [Leading symbols](artifacts/symbols.json) · [Verification](artifacts/verification.json)

## Changes within paired profiling blocks

The table shows the median of three within-block mapped-versus-ordinary percentage changes. Retired instructions and cache misses fell on all three inputs. The latency response varied with the input and the observer. Counters are aggregate process measurements from the warm proof window.

| Request | Counter-run wall | Sampling-run wall | Aggregate cycles | Instructions | Cache misses |
|---|---:|---:|---:|---:|---:|
| Original | -0.02% | -3.24% | -2.72% | -17.08% | -14.98% |
| 109 | -5.83% | -4.01% | -8.40% | -23.20% | -23.61% |
| 122 | +1.64% | +0.47% | -2.30% | -12.18% | -10.76% |
