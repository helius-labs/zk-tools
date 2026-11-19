# One 12×2 group plan across adjacent and scattered requests

The adaptive prover uses the same three group-index files on every request, with no fixture-family selection. Its plan was derived from separate adjacent-leaf public training requests. Eight adjacent and eight scattered held-out requests plus the original use five randomized blocks. Two production controls use 32 and 30 workers. A third control uses the adaptive binary with prepared groups disabled at 30 workers. The adaptive process uses the same 30-worker CPU mask as both 30-worker controls within each block. Each warms three proofs and times eleven. Peak RSS includes setup. The separate optional-budget probes are functional checks, not minimum-RAM measurements.

| Request | Release32 / release30 / disabled30 / adaptive30 ms | Adaptive vs release32 | Adaptive vs release30 | Adaptive vs disabled | Peak MiB release32 / adaptive |
|---|---:|---:|---:|---:|---:|
| Original | 31.389 / 31.278 / 31.303 / 29.564 | -5.81% (5/5) | -5.58% (5/5) | -5.44% (5/5) | 561.9 / 522.0 |
| 57 | 31.529 / 31.591 / 31.510 / 29.880 | -5.05% (4/5) | -5.91% (4/5) | -5.21% (4/5) | 584.3 / 516.2 |
| 58 | 31.343 / 31.633 / 31.150 / 29.105 | -6.48% (5/5) | -7.78% (5/5) | -6.36% (5/5) | 548.9 / 524.7 |
| 59 | 31.467 / 31.350 / 31.403 / 29.206 | -6.54% (5/5) | -6.40% (5/5) | -6.95% (5/5) | 566.4 / 534.1 |
| 60 | 32.105 / 31.263 / 31.442 / 29.256 | -8.85% (5/5) | -6.71% (5/5) | -6.74% (5/5) | 552.5 / 522.0 |
| 61 | 31.609 / 31.364 / 31.348 / 29.463 | -6.66% (5/5) | -5.76% (5/5) | -5.87% (5/5) | 569.5 / 517.1 |
| 62 | 31.187 / 31.565 / 31.301 / 29.123 | -6.10% (5/5) | -8.23% (5/5) | -6.92% (5/5) | 548.0 / 530.7 |
| 63 | 31.460 / 31.399 / 31.338 / 29.310 | -7.43% (5/5) | -6.65% (5/5) | -6.25% (5/5) | 580.9 / 526.2 |
| 64 | 31.327 / 31.338 / 31.459 / 29.313 | -5.16% (5/5) | -6.46% (5/5) | -6.01% (5/5) | 583.6 / 521.5 |
| 137 | 47.505 / 48.578 / 47.817 / 47.658 | +0.08% (2/5) | -2.36% (3/5) | +0.90% (2/5) | 953.3 / 1019.8 |
| 138 | 47.894 / 48.320 / 47.703 / 47.732 | +0.14% (2/5) | -0.69% (3/5) | -0.31% (3/5) | 995.8 / 1006.1 |
| 139 | 47.677 / 48.533 / 47.801 / 47.266 | -0.64% (3/5) | -0.16% (3/5) | -0.72% (3/5) | 978.5 / 1010.6 |
| 140 | 47.412 / 47.211 / 47.259 / 47.207 | -0.46% (3/5) | -0.15% (3/5) | +0.55% (2/5) | 973.3 / 1060.0 |
| 141 | 47.670 / 47.031 / 47.309 / 46.669 | -1.51% (5/5) | -1.56% (4/5) | -2.00% (5/5) | 1008.3 / 988.5 |
| 142 | 47.454 / 47.476 / 47.247 / 47.251 | -0.96% (3/5) | -0.44% (4/5) | +0.84% (2/5) | 939.6 / 995.4 |
| 143 | 47.446 / 47.906 / 47.224 / 47.139 | -0.91% (4/5) | -0.57% (4/5) | -0.93% (4/5) | 945.5 / 983.9 |
| 144 | 47.943 / 47.437 / 47.520 / 47.774 | +1.22% (2/5) | -0.11% (3/5) | +0.15% (2/5) | 940.5 / 974.7 |

The original request reached the 28.4 ms target in 0/5 blocks. At 29.564 ms its speedups versus the supplied M5 CPU and Metal timings are 9.61× and 3.42×. All 350 exports verified independently. [Samples](artifacts/processes.json.gz) · [Budget probes](artifacts/budget-probes.json) · [Input identities](artifacts/inputs.json) · [Verification](artifacts/verification.json)
