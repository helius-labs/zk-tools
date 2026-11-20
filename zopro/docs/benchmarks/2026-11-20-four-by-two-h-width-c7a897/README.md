# Wider H digits for the seeded 4×2 profile on C7a

The [curve census](../2026-11-20-four-by-two-curve-work-c7a896/README.md) counted about 1.87 million H streaming additions per proof. This trial changes H alone. Unsigned width 13 and signed width 14 each use 8,192 ordinary buckets. Unsigned width 12 and signed width 13 use 4,096. Widths 12 and 13 unsigned and width 14 signed each have a one-range control. The solver, FFT, K placement, B2 and verifier retain the frozen settings.

Zopro runs with 64 main and 32 frontend physical cores. Prepared Aeglos uses all 96 on the same c7a.24xlarge. Each input has eight counterbalanced timing blocks with three primes and thirteen timed calls. Advancement requires a lower paired median and six wins against the frozen, ordinary and explicit unsigned width-12 controls on both inputs. All 212 exports independently verified, including the zero-optional-workspace fallbacks for both wider recodings. The temporary instance and storage were removed.

The frozen seeded profile took 15.163 / 15.134 ms. The fastest alternate on the original input was `signed13` at 15.116 / 15.219 ms, with paired changes of +0.61% / +0.73% against the same-build ordinary profile. No alternate consistently improved all matched controls on both inputs.

| Input | Profile | Proof ms | Paired vs ordinary | Paired vs frozen | Paired vs unsigned12 | Paired vs Aeglos | Peak MiB | Blocks at or below 10.9 ms |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | frozen | 15.163 | +0.03% (4/8) | +0.00% (0/8) | -0.32% (4/8) | -19.63% (8/8) | 211.8 | 0/8 |
| catalog | ordinary | 15.030 | +0.00% (0/8) | -0.03% (4/8) | -1.19% (5/8) | -19.00% (8/8) | 205.8 | 0/8 |
| catalog | unsigned12 | 15.147 | +1.20% (3/8) | +0.32% (4/8) | +0.00% (0/8) | -17.95% (8/8) | 201.9 | 0/8 |
| catalog | unsigned12-one | 15.798 | +5.23% (0/8) | +4.13% (0/8) | +4.06% (0/8) | -14.75% (8/8) | 204.3 | 0/8 |
| catalog | signed13 | 15.116 | +0.61% (2/8) | +0.75% (3/8) | -0.15% (5/8) | -18.09% (8/8) | 196.6 | 0/8 |
| catalog | unsigned13 | 15.444 | +2.93% (0/8) | +2.56% (1/8) | +1.31% (0/8) | -17.05% (8/8) | 213.1 | 0/8 |
| catalog | unsigned13-one | 15.826 | +4.73% (0/8) | +4.26% (0/8) | +3.99% (0/8) | -15.13% (8/8) | 210.1 | 0/8 |
| catalog | signed14 | 15.400 | +2.48% (0/8) | +1.57% (2/8) | +0.93% (0/8) | -16.77% (8/8) | 211.7 | 0/8 |
| catalog | signed14-one | 15.608 | +4.12% (0/8) | +4.62% (0/8) | +2.85% (0/8) | -15.65% (8/8) | 213.1 | 0/8 |
| catalog | aeglos | 18.552 | +23.45% (0/8) | +24.42% (0/8) | +21.88% (0/8) | +0.00% (0/8) | 1348.3 | 0/8 |
| heldout | frozen | 15.134 | +0.25% (4/8) | +0.00% (0/8) | -0.35% (4/8) | -18.33% (8/8) | 193.6 | 0/8 |
| heldout | ordinary | 15.109 | +0.00% (0/8) | -0.25% (4/8) | -0.65% (5/8) | -18.58% (8/8) | 204.2 | 0/8 |
| heldout | unsigned12 | 15.139 | +0.65% (3/8) | +0.36% (4/8) | +0.00% (0/8) | -18.34% (8/8) | 198.1 | 0/8 |
| heldout | unsigned12-one | 15.782 | +4.66% (0/8) | +4.35% (0/8) | +3.97% (0/8) | -15.01% (8/8) | 199.0 | 0/8 |
| heldout | signed13 | 15.219 | +0.73% (3/8) | +1.26% (1/8) | +0.69% (3/8) | -17.40% (8/8) | 207.9 | 0/8 |
| heldout | unsigned13 | 15.475 | +1.90% (0/8) | +2.58% (0/8) | +1.96% (0/8) | -16.20% (8/8) | 221.4 | 0/8 |
| heldout | unsigned13-one | 15.862 | +5.03% (0/8) | +4.98% (0/8) | +4.95% (0/8) | -14.64% (8/8) | 200.5 | 0/8 |
| heldout | signed14 | 15.440 | +2.45% (0/8) | +3.00% (0/8) | +1.90% (0/8) | -16.33% (8/8) | 210.9 | 0/8 |
| heldout | signed14-one | 15.591 | +3.52% (0/8) | +3.13% (0/8) | +3.12% (1/8) | -15.78% (8/8) | 210.9 | 0/8 |
| heldout | aeglos | 18.549 | +22.82% (0/8) | +22.45% (0/8) | +22.46% (0/8) | +0.00% (0/8) | 1362.3 | 0/8 |

The separate observer evaluates H while replaying A, K and B2 after exact coefficient comparisons. Each count reproduced in two processes per input. Counted pairs sum the ordinary additions reported by the streaming, merge and shared tree counters. Projective exceptional work and field-operation costs are outside that total. The reservation is the admission forecast for H scratch, and the timing table reports whole-process peak RSS. The observer executable supplies no scored timing.

| Input | Profile | Windows | Streaming | Merge | Tree | Counted pairs | Change vs ordinary | Reserved MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | ordinary | 22 | 1,868,495 | 85,993 | 233,738 | 2,188,226 | +0.00% | — |
| catalog | unsigned12 | 22 | 1,868,495 | 85,993 | 233,738 | 2,188,226 | +0.00% | 99.7 |
| catalog | unsigned12-one | 22 | 1,954,593 | 0 | 233,738 | 2,188,331 | +0.00% | 94.2 |
| catalog | signed13 | 20 | 1,690,697 | 77,821 | 249,389 | 2,017,907 | -7.78% | 98.7 |
| catalog | unsigned13 | 20 | 1,535,985 | 154,823 | 401,507 | 2,092,315 | -4.38% | 185.6 |
| catalog | unsigned13-one | 20 | 1,690,962 | 0 | 401,507 | 2,092,469 | -4.38% | 175.5 |
| catalog | signed14 | 19 | 1,455,192 | 146,705 | 370,362 | 1,972,259 | -9.87% | 184.5 |
| catalog | signed14-one | 19 | 1,602,019 | 0 | 370,362 | 1,972,381 | -9.86% | 175.0 |
| heldout | ordinary | 22 | 1,869,905 | 85,992 | 233,688 | 2,189,585 | +0.00% | — |
| heldout | unsigned12 | 22 | 1,869,905 | 85,992 | 233,688 | 2,189,585 | +0.00% | 99.7 |
| heldout | unsigned12-one | 22 | 1,955,992 | 0 | 233,688 | 2,189,680 | +0.00% | 94.2 |
| heldout | signed13 | 20 | 1,691,990 | 77,823 | 249,493 | 2,019,306 | -7.78% | 98.7 |
| heldout | unsigned13 | 20 | 1,537,555 | 154,773 | 401,560 | 2,093,888 | -4.37% | 185.6 |
| heldout | unsigned13-one | 20 | 1,692,247 | 0 | 401,560 | 2,093,807 | -4.37% | 175.5 |
| heldout | signed14 | 19 | 1,456,597 | 146,680 | 370,283 | 1,973,560 | -9.87% | 184.5 |
| heldout | signed14-one | 19 | 1,603,254 | 0 | 370,283 | 1,973,537 | -9.87% | 175.0 |

[Samples and counts](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [H source](artifacts/source.patch) · [Observer source](artifacts/meter.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU on the recorded profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× |
| 4×2 | 854 seed4-emit | 14.726 | 109 / 48 | 7.40× / 3.26× |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× |
