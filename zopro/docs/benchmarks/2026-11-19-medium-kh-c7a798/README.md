# Medium fused K/H on 2×2 with 96 C7a cores

The existing CPU prover can combine the private K and quotient H MSMs when both queries fit its medium-size admission policy. On Ryzen the [previous panel](../2026-11-19-fused-kh-grouped-k-ryzen797/README.md) measured gains using the existing fused path. This run uses the released SDK and tests that option at 64+32, 72+24 and 80+16 main and frontend physical cores. G1 A uses the attached pool when the fused path admits it. The archived banded executable uses 72+24 cores. Prepared Aeglos uses all 96.

Six matched blocks measure each arm on the original and held-out requests. Each process primes three times and measures thirteen proofs. Routing checks confirmed fusion on both inputs and the fallback with zero optional preparation memory. All 102 exported proofs independently verified. Temporary AWS resources were removed.

The medium-72 arm took 12.054 / 12.237 ms, with paired changes of -10.13% / -7.53% from the same-width ordinary arm. The medium-64 profile updates the C7a scorecard.

| Input | Configuration | Proof ms | Paired vs same-width ordinary | Paired vs archived band | Paired vs Aeglos | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| catalog | aeglos | 11.688 | -13.21% (6/6) | -5.60% (5/6) | +0.00% (0/6) | 804.5 |
| catalog | archive | 12.262 | -9.04% (6/6) | +0.00% (0/6) | +5.94% (1/6) | 114.9 |
| catalog | ordinary-64 | 13.293 | +0.00% (0/6) | +7.41% (0/6) | +14.11% (0/6) | 85.7 |
| catalog | medium-64 | 12.127 | -7.20% (6/6) | -1.93% (4/6) | +3.30% (1/6) | 76.7 |
| catalog | ordinary-72 | 13.340 | +0.00% (0/6) | +9.94% (0/6) | +15.26% (0/6) | 85.6 |
| catalog | medium-72 | 12.054 | -10.13% (6/6) | -1.70% (4/6) | +3.15% (1/6) | 77.4 |
| catalog | ordinary-80 | 13.289 | +0.00% (0/6) | +9.35% (0/6) | +14.49% (0/6) | 84.2 |
| catalog | medium-80 | 12.102 | -8.69% (6/6) | -1.21% (5/6) | +3.23% (1/6) | 74.5 |
| heldout | aeglos | 11.625 | -12.47% (6/6) | -5.79% (5/6) | +0.00% (0/6) | 807.8 |
| heldout | archive | 12.255 | -6.54% (6/6) | +0.00% (0/6) | +6.16% (1/6) | 119.4 |
| heldout | ordinary-64 | 13.122 | +0.00% (0/6) | +6.47% (0/6) | +12.10% (0/6) | 91.7 |
| heldout | medium-64 | 12.138 | -7.75% (6/6) | -1.14% (6/6) | +4.41% (1/6) | 71.6 |
| heldout | ordinary-72 | 13.217 | +0.00% (0/6) | +7.00% (0/6) | +14.25% (0/6) | 86.4 |
| heldout | medium-72 | 12.237 | -7.53% (6/6) | +0.03% (3/6) | +5.17% (1/6) | 79.0 |
| heldout | ordinary-80 | 13.445 | +0.00% (0/6) | +9.60% (0/6) | +15.43% (0/6) | 88.7 |
| heldout | medium-80 | 12.413 | -6.58% (6/6) | -0.28% (3/6) | +5.21% (0/6) | 74.6 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Routing checks](artifacts/routes.json) · [Proof verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on verified C7a profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 798 SDK medium K/H 64 main | 12.127 | 59 / 30 | 4.87× / 2.47× | +6.227 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 781 recognized B2 origins replay | 20.098 | 189 / 74 | 9.40× / 3.68× | +1.198 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
