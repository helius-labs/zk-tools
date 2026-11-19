# B2 origin classification and endomorphism components on C7a

This host checks two changes to ordinary B2. On 2×2, the [origin classifier](../2026-11-19-exclusive-b2-origins-ryzen800/README.md) uses recognized Poseidon spans and checks the complete scalar before sharing a group. All current Zopro 2×2 arms use medium fused K/H with 64 main and 32 frontend cores. The archived G2 band reference uses 72 and 24 cores. On 4×2 and 8×2, the [component trial](../2026-11-19-b2-component-sums-ryzen799/README.md) performs separate short scalar MSMs and applies the endomorphism once to one result. 4×2 and 8×2 use 64 main and 32 frontend cores, with released Poseidon origins and serial conversion on 8×2. The existing integrated Phi path and ordinary backend provide controls. Prepared Aeglos uses all 96 cores throughout.

The two primary inputs use four matched blocks on 4×2 and six on 2×2 and 8×2. Two additional 8×2 inputs use four blocks. Each measured process warms three requests and times thirteen. Every Aeglos sample hit its preparation cache. The independent verifier checked all 282 exports against the original keys. Routing checks exercised both experiments and the zero optional-memory fallback. The instance and temporary storage were removed.

- **2×2.** `origins` led the experimental arms by the mean paired change on both primary inputs. It took 12.064 / 12.115 ms, with paired changes of +0.34% / -0.48% versus ordinary.
- **4×2.** `pc-12x2` led the experimental arms by the mean paired change on both primary inputs. It took 15.313 / 15.422 ms, with paired changes of -0.59% / -0.02% versus ordinary.
- **8×2.** `pc-12x2` led the experimental arms by the mean paired change on both primary inputs. It took 20.132 / 20.425 ms, with paired changes of -0.81% / +0.12% versus ordinary.

None of the experimental profiles gained consistently against the applicable controls. The experiment leaves the released code intact.

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Paired vs release | Paired vs archive | Paired vs Aeglos | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 2×2 | catalog | aeglos | 11.673 | -2.85% (6/6) | -2.71% (6/6) | -4.38% (6/6) | +0.00% (0/6) | 803.5 |
| 2×2 | catalog | archive | 12.263 | +1.76% (0/6) | +1.20% (1/6) | +0.00% (0/6) | +4.59% (0/6) | 117.8 |
| 2×2 | catalog | ordinary | 12.013 | +0.00% (0/6) | -0.80% (4/6) | -1.73% (6/6) | +2.94% (0/6) | 74.4 |
| 2×2 | catalog | origins | 12.064 | +0.34% (2/6) | -0.47% (3/6) | -1.72% (6/6) | +3.54% (0/6) | 71.3 |
| 2×2 | catalog | release | 12.137 | +0.82% (2/6) | +0.00% (0/6) | -1.18% (5/6) | +2.78% (0/6) | 74.5 |
| 2×2 | heldout | aeglos | 11.656 | -4.27% (6/6) | -2.84% (6/6) | -4.44% (6/6) | +0.00% (0/6) | 800.7 |
| 2×2 | heldout | archive | 12.226 | +0.34% (1/6) | +1.62% (0/6) | +0.00% (0/6) | +4.65% (0/6) | 120.2 |
| 2×2 | heldout | ordinary | 12.155 | +0.00% (0/6) | +1.14% (2/6) | -0.34% (5/6) | +4.46% (0/6) | 78.2 |
| 2×2 | heldout | origins | 12.115 | -0.48% (4/6) | +0.90% (2/6) | -0.90% (4/6) | +3.27% (0/6) | 77.3 |
| 2×2 | heldout | release | 11.997 | -1.12% (4/6) | +0.00% (0/6) | -1.60% (6/6) | +2.92% (0/6) | 78.3 |
| 4×2 | catalog | aeglos | 18.109 | +18.00% (0/4) | +17.89% (0/4) | — | +0.00% (0/4) | 1345.7 |
| 4×2 | catalog | integrated-phi | 16.392 | +6.42% (0/4) | +6.93% (0/4) | — | -9.48% (4/4) | 204.3 |
| 4×2 | catalog | ordinary | 15.382 | +0.00% (0/4) | +0.17% (2/4) | — | -15.25% (4/4) | 199.8 |
| 4×2 | catalog | pc-11x2 | 15.453 | +0.61% (1/4) | +0.68% (0/4) | — | -14.73% (4/4) | 203.4 |
| 4×2 | catalog | pc-12x1 | 16.627 | +7.94% (0/4) | +8.78% (0/4) | — | -8.53% (4/4) | 196.8 |
| 4×2 | catalog | pc-12x2 | 15.313 | -0.59% (3/4) | +0.26% (2/4) | — | -15.56% (4/4) | 208.8 |
| 4×2 | catalog | pc-12x3 | 15.575 | +1.78% (0/4) | +1.73% (0/4) | — | -14.07% (4/4) | 221.5 |
| 4×2 | catalog | release | 15.335 | -0.17% (2/4) | +0.00% (0/4) | — | -15.18% (4/4) | 202.0 |
| 4×2 | heldout | aeglos | 18.054 | +17.32% (0/4) | +16.86% (0/4) | — | +0.00% (0/4) | 1366.4 |
| 4×2 | heldout | integrated-phi | 16.379 | +6.31% (0/4) | +6.24% (0/4) | — | -9.28% (4/4) | 203.5 |
| 4×2 | heldout | ordinary | 15.391 | +0.00% (0/4) | -0.38% (3/4) | — | -14.76% (4/4) | 198.1 |
| 4×2 | heldout | pc-11x2 | 15.670 | +1.59% (0/4) | +1.00% (0/4) | — | -13.41% (4/4) | 210.1 |
| 4×2 | heldout | pc-12x1 | 16.677 | +8.28% (0/4) | +8.26% (0/4) | — | -7.52% (4/4) | 209.4 |
| 4×2 | heldout | pc-12x2 | 15.422 | -0.02% (2/4) | -0.08% (2/4) | — | -14.26% (4/4) | 199.7 |
| 4×2 | heldout | pc-12x3 | 15.722 | +1.93% (0/4) | +1.92% (0/4) | — | -12.77% (4/4) | 195.2 |
| 4×2 | heldout | release | 15.393 | +0.38% (1/4) | +0.00% (0/4) | — | -14.43% (4/4) | 213.9 |
| 8×2 | catalog | aeglos | 30.677 | +51.67% (0/6) | +51.05% (0/6) | — | +0.00% (0/6) | 2498.3 |
| 8×2 | catalog | integrated-phi | 21.603 | +6.46% (0/6) | +6.62% (0/6) | — | -30.15% (6/6) | 423.5 |
| 8×2 | catalog | ordinary | 20.314 | +0.00% (0/6) | -0.35% (4/6) | — | -34.06% (6/6) | 410.2 |
| 8×2 | catalog | pc-11x2 | 20.196 | +0.09% (3/6) | -0.44% (5/6) | — | -34.21% (6/6) | 407.1 |
| 8×2 | catalog | pc-12x1 | 22.014 | +8.67% (0/6) | +8.14% (0/6) | — | -28.65% (6/6) | 409.3 |
| 8×2 | catalog | pc-12x2 | 20.132 | -0.81% (4/6) | -0.91% (5/6) | — | -34.17% (6/6) | 409.3 |
| 8×2 | catalog | pc-12x3 | 20.669 | +1.99% (0/6) | +1.54% (0/6) | — | -32.73% (6/6) | 407.0 |
| 8×2 | catalog | release | 20.328 | +0.35% (2/6) | +0.00% (0/6) | — | -33.79% (6/6) | 400.2 |
| 8×2 | heldout | aeglos | 30.708 | +50.53% (0/6) | +49.69% (0/6) | — | +0.00% (0/6) | 2461.7 |
| 8×2 | heldout | integrated-phi | 21.823 | +6.96% (0/6) | +6.12% (0/6) | — | -28.86% (6/6) | 413.8 |
| 8×2 | heldout | ordinary | 20.386 | +0.00% (0/6) | -0.39% (4/6) | — | -33.57% (6/6) | 408.7 |
| 8×2 | heldout | pc-11x2 | 20.684 | +1.59% (1/6) | +1.76% (2/6) | — | -32.38% (6/6) | 417.5 |
| 8×2 | heldout | pc-12x1 | 22.247 | +9.27% (0/6) | +8.52% (0/6) | — | -27.44% (6/6) | 425.7 |
| 8×2 | heldout | pc-12x2 | 20.425 | +0.12% (3/6) | -0.33% (4/6) | — | -33.42% (6/6) | 415.2 |
| 8×2 | heldout | pc-12x3 | 20.865 | +2.28% (0/6) | +2.19% (0/6) | — | -32.04% (6/6) | 419.0 |
| 8×2 | heldout | release | 20.511 | +0.40% (2/6) | +0.00% (0/6) | — | -33.19% (6/6) | 419.8 |
| 8×2 | v153 | aeglos | 30.558 | +48.19% (0/4) | +49.46% (0/4) | — | +0.00% (0/4) | 2495.0 |
| 8×2 | v153 | integrated-phi | 21.829 | +6.38% (0/4) | +6.89% (0/4) | — | -28.83% (4/4) | 408.5 |
| 8×2 | v153 | ordinary | 20.486 | +0.00% (0/4) | -0.05% (2/4) | — | -32.52% (4/4) | 406.3 |
| 8×2 | v153 | pc-12x1 | 22.309 | +8.16% (0/4) | +9.11% (0/4) | — | -26.70% (4/4) | 402.6 |
| 8×2 | v153 | pc-12x2 | 20.465 | -0.63% (3/4) | +0.07% (2/4) | — | -32.90% (4/4) | 418.2 |
| 8×2 | v153 | release | 20.477 | +0.05% (2/4) | +0.00% (0/4) | — | -33.09% (4/4) | 415.2 |
| 8×2 | v156 | aeglos | 30.362 | +47.50% (0/4) | +48.42% (0/4) | — | +0.00% (0/4) | 2472.0 |
| 8×2 | v156 | integrated-phi | 21.667 | +5.41% (0/4) | +4.51% (0/4) | — | -28.64% (4/4) | 397.8 |
| 8×2 | v156 | ordinary | 20.458 | +0.00% (0/4) | -1.07% (3/4) | — | -32.20% (4/4) | 408.3 |
| 8×2 | v156 | pc-12x1 | 22.456 | +9.30% (0/4) | +8.12% (0/4) | — | -26.06% (4/4) | 421.9 |
| 8×2 | v156 | pc-12x2 | 20.437 | -0.39% (3/4) | -1.25% (3/4) | — | -32.51% (4/4) | 418.2 |
| 8×2 | v156 | release | 20.608 | +1.09% (1/4) | +0.00% (0/4) | — | -32.62% (4/4) | 414.3 |

[Samples](artifacts/c7a-rows.json.gz) · [Workflow](artifacts/c7a-workflow.json.gz) · [G2 components](artifacts/source.patch) · [2×2 classification](artifacts/exclusive-source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU.

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
