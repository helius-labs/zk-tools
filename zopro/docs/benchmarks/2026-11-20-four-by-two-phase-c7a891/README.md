# Curve replay and stage timing for 4×2 on C7a

This benchmark uses the exact four-epoch seeded-solver and Poseidon-emission settings from the [14.726 ms 4×2 run](../2026-11-20-small-seeded-solver-c7a854/README.md). The frozen control runs that original executable. The separately compiled observer uses the same source and environment with measurement hooks added. Zopro uses 64 main and 32 frontend physical cores on c7a.24xlarge. Prepared Aeglos uses all 96. All 132 exports independently verified. The first invocation passed preflight and stopped before timing because its wrapper had already created the results directory. The timed retry let the runner create it. Both temporary instances and their storage were removed.

Each replay stores one query output during warmup. A timed hit requires an exact match of the base pointer and the complete input coefficients. Every proof still solves a witness, computes its quotient, samples fresh blinding and verifies the result. Separate probes checked replay, request mismatch and denial at zero optional workspace. These diagnostic runs use one key per process and replay measurements are excluded from the performance scorecard.

The untouched seeded SDK took 15.093 / 15.038 ms on the original and held-out transactions. The instrumented build with every observer disabled took 15.161 / 14.982 ms, a paired change of +0.93% / -0.40%. All-query replay took 7.503 / 7.576 ms, leaving 3.397 / 3.324 ms for curve work within the 10.9 ms target when measured this way.

The paired changes against the ordinary build were -0.656 / -0.445 ms for replaying A, -0.454 / -0.803 ms for replaying K and H, and -1.331 / -1.101 ms for replaying B2. Each input uses six counterbalanced blocks with three primes and thirteen timed requests. The clocked all-query arm compares with unclocked all-query replay. All other paired rows compare with the ordinary observer build. Paired deltas are medians of matched process differences.

| Input | Mode | Proof ms | Paired change ms | Paired change and wins | Peak MiB |
|---|---|---:|---:|---:|---:|
| catalog | release | 15.671 | +0.469 | +3.08% (0/6) | 205.0 |
| catalog | frozen | 15.093 | -0.140 | -0.92% (4/6) | 199.6 |
| catalog | ordinary | 15.161 | +0.000 | +0.00% (0/6) | 206.5 |
| catalog | clock | 15.159 | +0.052 | +0.35% (1/6) | 199.0 |
| catalog | a | 14.393 | -0.656 | -4.35% (6/6) | 196.8 |
| catalog | kh | 14.721 | -0.454 | -2.99% (6/6) | 189.8 |
| catalog | b2 | 13.807 | -1.331 | -8.83% (6/6) | 147.2 |
| catalog | all | 7.503 | -7.657 | -50.38% (6/6) | 139.7 |
| catalog | all-clock | 7.539 | +0.097 | +1.28% (2/6) | 144.1 |
| catalog | aeglos | 18.536 | +3.450 | +22.87% (0/6) | 1346.8 |
| heldout | release | 15.711 | +0.707 | +4.73% (0/6) | 208.0 |
| heldout | frozen | 15.038 | +0.060 | +0.40% (2/6) | 194.4 |
| heldout | ordinary | 14.982 | +0.000 | +0.00% (0/6) | 199.7 |
| heldout | clock | 15.145 | +0.208 | +1.39% (2/6) | 212.4 |
| heldout | a | 14.583 | -0.445 | -2.95% (6/6) | 193.0 |
| heldout | kh | 14.186 | -0.803 | -5.38% (6/6) | 213.1 |
| heldout | b2 | 13.942 | -1.101 | -7.31% (6/6) | 138.9 |
| heldout | all | 7.576 | -7.406 | -49.43% (6/6) | 141.1 |
| heldout | all-clock | 7.641 | +0.068 | +0.89% (1/6) | 142.0 |
| heldout | aeglos | 18.555 | +3.551 | +23.56% (0/6) | 1342.8 |

Clocked rows report median process means. Serialization includes the curve result encoding and SDK output. The remaining time includes assembly, verification and uninstrumented transitions. Each query clock includes nested worker activity. Its tail is the time it continues after the other three have finished.

| Input | Mode | Parse | Solve | Gather | FFT | Curve stage | Serialization | Other |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | clock | 0.291 | 3.070 | 1.002 | 2.067 | 7.529 | 0.025 | 1.167 |
| catalog | all-clock | 0.214 | 3.002 | 0.986 | 2.038 | 0.272 | 0.018 | 1.023 |
| heldout | clock | 0.314 | 3.052 | 0.992 | 2.049 | 7.547 | 0.025 | 1.169 |
| heldout | all-clock | 0.228 | 3.049 | 0.985 | 2.067 | 0.260 | 0.019 | 1.023 |

| Input | Mode | A | K | H | B2 | A tail | K tail | H tail | B2 tail |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| catalog | clock | 5.616 | 7.210 | 5.844 | 7.183 | 0.056 | 0.089 | 0.004 | 0.153 |
| catalog | all-clock | 0.111 | 0.194 | 0.209 | 0.206 | 0.000 | 0.005 | 0.005 | 0.007 |
| heldout | clock | 4.962 | 7.247 | 5.193 | 7.210 | 0.005 | 0.155 | 0.000 | 0.124 |
| heldout | all-clock | 0.109 | 0.191 | 0.200 | 0.204 | 0.000 | 0.003 | 0.007 | 0.004 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Observer source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 14.726 | 109 / 48 | 7.40× / 3.26× | +3.826 |
| 8×2 | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
