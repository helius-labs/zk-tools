# Full-query B2 replay on C7a

The [Ryzen diagnostic](../2026-11-19-b2-query-bound-ryzen793/README.md) reuses the B2 curve result for an identical witness on an immutable key. Every measured call solves the witness, checks the full B2 coefficients and computes the other queries with a fresh proof. The 4×2 and 8×2 comparison adds a mode that retains scalar classes and affine point sums and still computes the final MSM. Zero optional memory and a different witness fall back to the ordinary calculation.

On one c7a.24xlarge, 1×2 uses 80 main and 16 frontend physical cores, 2×2 uses 72 and 24, and 4×2 and 8×2 use 64 and 32. Five matched blocks per input compare the released SDK and same-build selections, with three primes and thirteen timed proofs per process. Routing checks confirm reuse, exact rejection of a different witness, and fallback under a zero optional-memory allowance on both inputs per shape. All 164 exported proofs independently verified. The temporary AWS resources were removed.

| Shape | Ordinary original / held-out ms | Full B2 replay original / held-out ms | Paired change original / held-out ms | 10× M5 CPU bound ms |
|---|---:|---:|---:|---:|
| 1×2 | 7.855 / 7.838 | 7.377 / 7.279 | -0.479 / -0.574 | 3.7 |
| 2×2 | 13.178 / 13.169 | 10.038 / 10.006 | -3.233 / -3.127 | 5.9 |
| 4×2 | 15.487 / 15.569 | 14.206 / 14.454 | -1.277 / -1.099 | 10.9 |
| 8×2 | 20.415 / 20.558 | 18.409 / 18.711 | -2.192 / -1.876 | 18.9 |

The fixed-request replay brought the following original requests within their 10× latency bounds: 8×2. These results estimate how other work proceeds when the ordinary B2 call is removed. The timed scalar comparison and changed scheduling affect the estimate. The replay is confined to the benchmark and excluded from the performance scorecard.

| Shape | Input | Mode | Proof ms | Paired Δ vs ordinary ms | Paired vs ordinary | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | release | 7.865 | +0.013 | +0.17% (1/5) | +0.00% (0/5) | 42.3 |
| 1×2 | catalog | ordinary | 7.855 | +0.000 | +0.00% (0/5) | -0.17% (4/5) | 39.9 |
| 1×2 | catalog | result | 7.377 | -0.479 | -6.10% (5/5) | -6.21% (5/5) | 42.3 |
| 1×2 | heldout | release | 7.859 | +0.018 | +0.22% (1/5) | +0.00% (0/5) | 40.0 |
| 1×2 | heldout | ordinary | 7.838 | +0.000 | +0.00% (0/5) | -0.22% (4/5) | 39.2 |
| 1×2 | heldout | result | 7.279 | -0.574 | -7.29% (5/5) | -7.92% (5/5) | 39.9 |
| 2×2 | catalog | release | 13.202 | -0.014 | -0.11% (3/5) | +0.00% (0/5) | 86.4 |
| 2×2 | catalog | ordinary | 13.178 | +0.000 | +0.00% (0/5) | +0.11% (2/5) | 93.9 |
| 2×2 | catalog | result | 10.038 | -3.233 | -24.53% (5/5) | -24.45% (5/5) | 78.2 |
| 2×2 | heldout | release | 13.332 | +0.340 | +2.58% (2/5) | +0.00% (0/5) | 85.7 |
| 2×2 | heldout | ordinary | 13.169 | +0.000 | +0.00% (0/5) | -2.52% (3/5) | 84.3 |
| 2×2 | heldout | result | 10.006 | -3.127 | -23.74% (5/5) | -25.54% (5/5) | 79.0 |
| 4×2 | catalog | release | 15.459 | -0.085 | -0.55% (3/5) | +0.00% (0/5) | 207.2 |
| 4×2 | catalog | ordinary | 15.487 | +0.000 | +0.00% (0/5) | +0.55% (2/5) | 206.4 |
| 4×2 | catalog | result | 14.206 | -1.277 | -8.21% (5/5) | -7.68% (5/5) | 153.9 |
| 4×2 | catalog | affine | 15.128 | -0.286 | -1.84% (4/5) | -2.48% (5/5) | 167.6 |
| 4×2 | heldout | release | 15.568 | -0.000 | -0.00% (3/5) | +0.00% (0/5) | 212.4 |
| 4×2 | heldout | ordinary | 15.569 | +0.000 | +0.00% (0/5) | +0.00% (2/5) | 199.0 |
| 4×2 | heldout | result | 14.454 | -1.099 | -7.06% (5/5) | -7.06% (5/5) | 156.9 |
| 4×2 | heldout | affine | 14.964 | -0.554 | -3.53% (5/5) | -2.64% (5/5) | 166.7 |
| 8×2 | catalog | release | 20.465 | +0.072 | +0.35% (2/5) | +0.00% (0/5) | 411.4 |
| 8×2 | catalog | ordinary | 20.415 | +0.000 | +0.00% (0/5) | -0.35% (3/5) | 413.9 |
| 8×2 | catalog | result | 18.409 | -2.192 | -10.64% (5/5) | -9.85% (5/5) | 349.0 |
| 8×2 | catalog | affine | 19.913 | -0.688 | -3.34% (5/5) | -3.07% (5/5) | 368.8 |
| 8×2 | heldout | release | 20.666 | +0.159 | +0.77% (2/5) | +0.00% (0/5) | 415.3 |
| 8×2 | heldout | ordinary | 20.558 | +0.000 | +0.00% (0/5) | -0.77% (3/5) | 419.7 |
| 8×2 | heldout | result | 18.711 | -1.876 | -9.15% (5/5) | -8.84% (5/5) | 350.1 |
| 8×2 | heldout | affine | 20.129 | -0.521 | -2.52% (4/5) | -2.20% (5/5) | 367.1 |

[Samples](artifacts/c7a-rows.json.gz) · [Workflow](artifacts/c7a-workflow.json.gz) · [Source](artifacts/source.patch) · [Routing checks](artifacts/routes.json) · [Proof verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU with verified non-replay profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
