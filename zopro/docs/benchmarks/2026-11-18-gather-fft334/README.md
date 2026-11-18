# Concurrent canonical scalar gathering and product FFT

At 32 logical Ryzen workers the 1×2 control took 7.718 ms. Running gather and FFT together on the smaller pool took 7.793 ms. Moving one branch to the main pool gave 7.752 or 7.844 ms. None of the three modes met the prespecified primary threshold. On 2×2 the shared frontend pool won five pairs at 32 workers, with a 0.96% median paired gain. The sequential schedule remains in the integrated prover.

Each fresh process used eight caller-owned frontend workers on the main affinity mask and measured 21 requests after three primes. The 16-worker setting uses physical cores, while the 32-worker setting includes SMT. The overlap requires a temporary charge for the two query outputs that can coexist with FFT scratch.

| Shape | Main workers | Mode | Median ms | Paired vs off | Faster blocks | CPU ms | Peak MiB |
|---|---:|---|---:|---:|---:|---:|---:|
| 1x2 | 16 | retained | 8.791 | -0.65% | 3/5 | 101.0 | 58 |
| 1x2 | 16 | off | 8.840 | +0.00% | 0/5 | 101.2 | 64 |
| 1x2 | 16 | together | 8.909 | +0.32% | 2/5 | 100.7 | 63 |
| 1x2 | 16 | main-gather | 8.880 | +0.09% | 1/5 | 101.4 | 65 |
| 1x2 | 16 | main-fft | 8.801 | -1.04% | 3/5 | 104.4 | 60 |
| 1x2 | 32 | retained | 7.786 | +0.01% | 2/5 | 152.2 | 66 |
| 1x2 | 32 | off | 7.718 | +0.00% | 0/5 | 152.0 | 70 |
| 1x2 | 32 | together | 7.793 | +0.07% | 2/5 | 152.2 | 66 |
| 1x2 | 32 | main-gather | 7.752 | +0.97% | 2/5 | 154.7 | 74 |
| 1x2 | 32 | main-fft | 7.844 | +0.86% | 2/5 | 166.5 | 67 |
| 2x2 | 16 | retained | 12.154 | +0.78% | 0/5 | 137.9 | 108 |
| 2x2 | 16 | off | 11.996 | +0.00% | 0/5 | 137.0 | 107 |
| 2x2 | 16 | together | 12.252 | +2.07% | 1/5 | 137.8 | 124 |
| 2x2 | 16 | main-gather | 12.235 | +1.62% | 0/5 | 139.2 | 136 |
| 2x2 | 16 | main-fft | 12.037 | +0.57% | 2/5 | 146.0 | 175 |
| 2x2 | 32 | retained | 10.963 | +0.85% | 1/5 | 203.9 | 118 |
| 2x2 | 32 | off | 10.973 | +0.00% | 0/5 | 205.7 | 138 |
| 2x2 | 32 | together | 10.842 | -0.96% | 5/5 | 201.4 | 123 |
| 2x2 | 32 | main-gather | 10.951 | -0.20% | 3/5 | 209.6 | 144 |
| 2x2 | 32 | main-fft | 11.001 | -0.63% | 3/5 | 235.3 | 231 |

Separate thread-ID probes verified where each branch ran, including the sequential fallback at zero optional memory. The routing artifact counts gather entries observed during the FFT in each probe run. All 110 process exports verified.

[Measurement protocol](artifacts/protocol.md) · [Thread routing](artifacts/routing.json) · [Process data](artifacts/processes.json.gz) · [Verification](artifacts/verification.json)
