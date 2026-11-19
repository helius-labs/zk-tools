# Mapped replacement B2 for 4×2 on C7a

The [locally qualified replacement image](../2026-11-19-packed-b2-four782/README.md) retains public-wire offsets and the original 95,537 B2 points. It has a 44,072-point sparse prefix and a 764,312-byte index trailer. The packed arms use the exact same image, direct scalar equality for residual detection and exact differences for corrections. One arm uses the exclusive G2 prefix kernel. The other uses the ordinary grouped prefix kernel. Separate checks exercise the dense route at zero optional-memory allowance.

One c7a.24xlarge runs six matched blocks on each of four inputs. Zopro uses 64 main and 32 frontend cores with the frontend solver, fused FFT and late frontend K. Prepared Aeglos uses all 96 cores. Ordinary-image controls include released Zopro, the same executable, and its serial B2 option. Each process discarded three priming requests and timed thirteen proofs. All 156 exports verified independently against the ordinary key. The instance and temporary storage were removed.

The earlier 4×2 reference remains the fastest qualifying measurement.

| Input | Profile | Proof ms | Paired vs release | Paired vs ordinary same build | Paired vs ordinary serial | Paired vs packed Phi | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | aeglos | 18.371 | +18.45% (0/6) | +19.03% (0/6) | +19.67% (0/6) | +12.47% (0/6) | +0.00% (0/6) | 1347.0 | 5.93× / 2.61× |
| catalog | ordinary | 15.434 | -0.23% (3/6) | +0.00% (0/6) | +0.90% (1/6) | -5.43% (6/6) | -15.99% (6/6) | 199.7 | 7.06× / 3.11× |
| catalog | ordinary-serial | 15.308 | -0.69% (6/6) | -0.89% (5/6) | +0.00% (0/6) | -5.81% (6/6) | -16.44% (6/6) | 204.2 | 7.12× / 3.14× |
| catalog | packed-eq | 16.264 | +5.09% (0/6) | +5.75% (0/6) | +6.17% (0/6) | +0.00% (0/6) | -11.09% (6/6) | 225.1 | 6.70× / 2.95× |
| catalog | packed-eq-raw | 15.542 | +0.83% (2/6) | +0.70% (1/6) | +1.55% (0/6) | -4.57% (6/6) | -14.64% (6/6) | 220.7 | 7.01× / 3.09× |
| catalog | release | 15.447 | +0.00% (0/6) | +0.24% (3/6) | +0.70% (0/6) | -4.84% (6/6) | -15.58% (6/6) | 207.9 | 7.06× / 3.11× |
| heldout | aeglos | 18.378 | +17.56% (0/6) | +18.82% (0/6) | +19.55% (0/6) | +11.13% (0/6) | +0.00% (0/6) | 1348.8 | — |
| heldout | ordinary | 15.446 | -0.35% (4/6) | +0.00% (0/6) | +1.10% (1/6) | -5.98% (6/6) | -15.83% (6/6) | 198.3 | — |
| heldout | ordinary-serial | 15.308 | -1.58% (6/6) | -1.09% (5/6) | +0.00% (0/6) | -7.28% (6/6) | -16.35% (6/6) | 214.0 | — |
| heldout | packed-eq | 16.494 | +5.98% (0/6) | +6.36% (0/6) | +7.85% (0/6) | +0.00% (0/6) | -10.02% (6/6) | 222.1 | — |
| heldout | packed-eq-raw | 15.653 | +0.95% (1/6) | +0.96% (1/6) | +2.45% (1/6) | -4.90% (6/6) | -14.07% (6/6) | 215.4 | — |
| heldout | release | 15.506 | +0.00% (0/6) | +0.35% (2/6) | +1.60% (0/6) | -5.65% (6/6) | -14.93% (6/6) | 195.3 | — |
| v153 | aeglos | 18.433 | +19.22% (0/6) | +19.15% (0/6) | +19.56% (0/6) | +13.02% (0/6) | +0.00% (0/6) | 1346.7 | — |
| v153 | ordinary | 15.458 | -0.27% (4/6) | +0.00% (0/6) | +0.82% (2/6) | -5.66% (6/6) | -16.08% (6/6) | 208.7 | — |
| v153 | ordinary-serial | 15.390 | -0.30% (4/6) | -0.81% (4/6) | +0.00% (0/6) | -5.48% (6/6) | -16.35% (6/6) | 213.9 | — |
| v153 | packed-eq | 16.309 | +5.84% (0/6) | +6.00% (0/6) | +5.79% (0/6) | +0.00% (0/6) | -11.52% (6/6) | 219.1 | — |
| v153 | packed-eq-raw | 15.662 | +1.49% (1/6) | +1.77% (1/6) | +2.62% (1/6) | -3.66% (6/6) | -15.04% (6/6) | 216.2 | — |
| v153 | release | 15.407 | +0.00% (0/6) | +0.27% (2/6) | +0.30% (2/6) | -5.52% (6/6) | -16.12% (6/6) | 201.9 | — |
| v156 | aeglos | 18.530 | +19.75% (0/6) | +18.51% (0/6) | +20.52% (0/6) | +13.41% (0/6) | +0.00% (0/6) | 1345.1 | — |
| v156 | ordinary | 15.631 | +0.85% (1/6) | +0.00% (0/6) | +1.71% (0/6) | -3.98% (5/6) | -15.62% (6/6) | 208.7 | — |
| v156 | ordinary-serial | 15.312 | -0.85% (5/6) | -1.68% (6/6) | +0.00% (0/6) | -6.02% (6/6) | -17.03% (6/6) | 214.6 | — |
| v156 | packed-eq | 16.303 | +5.04% (0/6) | +4.14% (1/6) | +6.41% (0/6) | +0.00% (0/6) | -11.83% (6/6) | 222.2 | — |
| v156 | packed-eq-raw | 15.576 | +0.70% (3/6) | -0.35% (3/6) | +2.26% (0/6) | -4.50% (6/6) | -14.87% (6/6) | 221.4 | — |
| v156 | release | 15.486 | +0.00% (0/6) | -0.84% (5/6) | +0.85% (1/6) | -4.79% (6/6) | -16.49% (6/6) | 201.9 | — |

[Samples and route checks](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Packed source](artifacts/source.patch) · [Kernel switch](artifacts/kernel.patch) · [Image layout](artifacts/image-layout.json) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU. [Preceding scorecard](../2026-11-19-packed-b2-kernel-c7a781/README.md)

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
