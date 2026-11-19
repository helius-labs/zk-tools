# Early folded curve queries on C7a

The [Ryzen trial](../2026-11-19-folded-query-overlap-ryzen819/README.md) starts B2 alone or G1 A and B2 together while the frontend runs the folded Euler quotient. K and H begin after the quotient returns. Route admission reserves space for overlapping work. Both schedules compute fresh outputs using the existing equations and curve routines.

Zopro uses 64 main and 32 frontend physical cores on one c7a.24xlarge. 4×2 enables the frontend solver. 8×2 uses recognized Poseidon origins and serial scalar conversion. Prepared Aeglos uses all 96 cores. There are four paired blocks per 4×2 input, six per primary 8×2 input, and four per additional 8×2 input. Each process primes three calls and measures thirteen. All 156 exported proofs independently verified. Route probes covered both schedules and the zero optional-memory fallback. The temporary instance and storage were removed.

- 4×2 `early-b2` took 15.239 / 15.214 ms across original and held-out, paired -0.85% / -0.96% from the same-build ordinary arm.
- 8×2 `early-b2` took 21.973 / 22.409 ms across original and held-out, paired +7.50% / +9.16% from the same-build ordinary arm.

Neither schedule improved both controls consistently across the tested inputs. The source is experimental.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 15.327 | -0.44% (3/4) | +0.00% (0/4) | -15.74% (4/4) | 205.7 |
| 4×2 | catalog | ordinary | 15.382 | +0.00% (0/4) | +0.44% (1/4) | -15.13% (4/4) | 198.9 |
| 4×2 | catalog | early-b2 | 15.239 | -0.85% (3/4) | -0.24% (3/4) | -16.11% (4/4) | 205.8 |
| 4×2 | catalog | early-ab2 | 15.978 | +3.93% (0/4) | +3.65% (0/4) | -11.79% (4/4) | 213.2 |
| 4×2 | catalog | aeglos | 18.181 | +17.82% (0/4) | +18.68% (0/4) | +0.00% (0/4) | 1363.1 |
| 4×2 | heldout | release | 15.267 | -0.40% (2/4) | +0.00% (0/4) | -16.77% (4/4) | 204.9 |
| 4×2 | heldout | ordinary | 15.321 | +0.00% (0/4) | +0.41% (2/4) | -16.14% (4/4) | 194.4 |
| 4×2 | heldout | early-b2 | 15.214 | -0.96% (4/4) | -0.30% (2/4) | -16.74% (4/4) | 205.0 |
| 4×2 | heldout | early-ab2 | 15.929 | +3.63% (0/4) | +3.69% (0/4) | -13.35% (4/4) | 214.6 |
| 4×2 | heldout | aeglos | 18.344 | +19.26% (0/4) | +20.16% (0/4) | +0.00% (0/4) | 1349.1 |
| 8×2 | catalog | release | 20.272 | -0.35% (4/6) | +0.00% (0/6) | -33.18% (6/6) | 404.6 |
| 8×2 | catalog | ordinary | 20.393 | +0.00% (0/6) | +0.35% (2/6) | -32.92% (6/6) | 416.9 |
| 8×2 | catalog | early-b2 | 21.973 | +7.50% (0/6) | +7.52% (0/6) | -28.08% (6/6) | 458.6 |
| 8×2 | catalog | early-ab2 | 22.100 | +9.09% (0/6) | +9.08% (0/6) | -27.07% (6/6) | 429.6 |
| 8×2 | catalog | aeglos | 30.390 | +49.08% (0/6) | +49.65% (0/6) | +0.00% (0/6) | 2456.1 |
| 8×2 | heldout | release | 20.441 | -0.08% (3/6) | +0.00% (0/6) | -32.44% (6/6) | 415.1 |
| 8×2 | heldout | ordinary | 20.505 | +0.00% (0/6) | +0.08% (3/6) | -32.69% (6/6) | 421.7 |
| 8×2 | heldout | early-b2 | 22.409 | +9.16% (0/6) | +9.12% (0/6) | -26.25% (6/6) | 458.8 |
| 8×2 | heldout | early-ab2 | 22.073 | +7.68% (0/6) | +8.48% (0/6) | -27.44% (6/6) | 428.1 |
| 8×2 | heldout | aeglos | 30.245 | +48.58% (0/6) | +48.01% (0/6) | +0.00% (0/6) | 2488.4 |
| 8×2 | v153 | release | 20.516 | +0.10% (2/4) | +0.00% (0/4) | -32.35% (4/4) | 405.4 |
| 8×2 | v153 | ordinary | 20.542 | +0.00% (0/4) | -0.10% (2/4) | -32.60% (4/4) | 410.3 |
| 8×2 | v153 | early-b2 | 22.254 | +9.29% (0/4) | +8.49% (0/4) | -26.44% (4/4) | 436.4 |
| 8×2 | v153 | early-ab2 | 22.424 | +8.63% (0/4) | +9.09% (0/4) | -26.65% (4/4) | 412.8 |
| 8×2 | v153 | aeglos | 30.326 | +48.38% (0/4) | +47.84% (0/4) | +0.00% (0/4) | 2491.0 |
| 8×2 | v156 | release | 20.450 | -0.29% (3/4) | +0.00% (0/4) | -32.46% (4/4) | 400.2 |
| 8×2 | v156 | ordinary | 20.509 | +0.00% (0/4) | +0.29% (1/4) | -31.97% (4/4) | 401.0 |
| 8×2 | v156 | early-b2 | 21.826 | +6.09% (0/4) | +6.78% (0/4) | -27.92% (4/4) | 441.6 |
| 8×2 | v156 | early-ab2 | 22.323 | +8.91% (0/4) | +8.39% (0/4) | -27.42% (4/4) | 431.6 |
| 8×2 | v156 | aeglos | 30.280 | +46.99% (0/4) | +48.07% (0/4) | +0.00% (0/4) | 2487.1 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## CPU use on the original inputs

CPU-ms covers user and system time per timed proof. Average busy cores divide the same CPU time by the elapsed loop time. Both include spinning and can describe total activity without identifying which workers execute useful curve or FFT operations.

| Shape | Mode | CPU-ms per proof | Average busy cores |
|---|---|---:|---:|
| 4×2 | release | 697.2 | 45.6 |
| 4×2 | ordinary | 695.9 | 45.3 |
| 4×2 | early-b2 | 671.5 | 44.1 |
| 4×2 | early-ab2 | 720.4 | 45.3 |
| 4×2 | aeglos | 1354.5 | 74.4 |
| 8×2 | release | 1081.7 | 53.4 |
| 8×2 | ordinary | 1086.3 | 53.3 |
| 8×2 | early-b2 | 1074.5 | 49.0 |
| 8×2 | early-ab2 | 1113.9 | 50.2 |
| 8×2 | aeglos | 2504.1 | 82.6 |

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
