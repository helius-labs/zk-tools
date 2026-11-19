# Poseidon origins for grouped B2 in the CPU SDK

The candidate exposes the [solver-derived origin classifier](../2026-11-19-b2-origin-clock-c7a776/README.md) as a per-key Linux x86-64 option. The solver prepares an index plan from recognized Poseidon spans under the existing workspace policy. Every reuse checks the full scalar. Mismatches fall through to ordinary hashing. The key retains indices and releases the plan when the option is disabled. Transient memory denial permits a later retry.

One c7a.24xlarge compares the released SDK, the new SDK with the option enabled and disabled, and prepared Aeglos across four 4×2 and four 8×2 inputs. The 8×2 arms also include the archived origin trial. Zopro uses 64 main and 32 frontend cores with the same late K and fused FFT settings. 8×2 enables the released serial B2 option in every Zopro arm. Prepared Aeglos uses all 96 cores.

The native core suite and generic example build passed. All 196 AWS and six Ryzen proofs independently verified, including zero optional-memory fallback. Each timed process ran three priming and thirteen measured proofs. The instance and temporary storage were removed.

The SDK option is available on Linux x86-64 and is disabled by default. It beat the released and same-build controls on both primary 8×2 inputs.

| Shape | Input | Profile | Proof ms | Paired vs same-build disabled | Paired vs release | Paired vs archive | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.387 | +17.88% (0/4) | +18.24% (0/4) | — | +0.00% (0/4) | 1364.0 | 5.93× / 2.61× |
| 4×2 | catalog | off | 15.662 | +0.00% (0/4) | +0.32% (2/4) | — | -15.16% (4/4) | 200.5 | 6.96× / 3.06× |
| 4×2 | catalog | on | 15.533 | +0.00% (2/4) | -0.81% (3/4) | — | -16.11% (4/4) | 209.5 | 7.02× / 3.09× |
| 4×2 | catalog | release | 15.629 | -0.31% (2/4) | +0.00% (0/4) | — | -15.43% (4/4) | 205.7 | 6.97× / 3.07× |
| 4×2 | heldout | aeglos | 18.641 | +18.65% (0/4) | +19.83% (0/4) | — | +0.00% (0/4) | 1343.5 | — |
| 4×2 | heldout | off | 15.713 | +0.00% (0/4) | +0.80% (0/4) | — | -15.70% (4/4) | 200.4 | — |
| 4×2 | heldout | on | 15.547 | -1.28% (4/4) | -0.47% (4/4) | — | -17.01% (4/4) | 200.4 | — |
| 4×2 | heldout | release | 15.634 | -0.79% (4/4) | +0.00% (0/4) | — | -16.53% (4/4) | 211.0 | — |
| 4×2 | v153 | aeglos | 18.727 | +19.69% (0/4) | +19.51% (0/4) | — | +0.00% (0/4) | 1343.0 | — |
| 4×2 | v153 | off | 15.645 | +0.00% (0/4) | -0.26% (3/4) | — | -16.45% (4/4) | 212.5 | — |
| 4×2 | v153 | on | 15.605 | -0.42% (3/4) | -0.79% (3/4) | — | -17.16% (4/4) | 208.1 | — |
| 4×2 | v153 | release | 15.640 | +0.26% (1/4) | +0.00% (0/4) | — | -16.32% (4/4) | 204.1 | — |
| 4×2 | v156 | aeglos | 18.728 | +19.43% (0/4) | +19.50% (0/4) | — | +0.00% (0/4) | 1346.9 | — |
| 4×2 | v156 | off | 15.632 | +0.00% (0/4) | -0.16% (2/4) | — | -16.26% (4/4) | 201.2 | — |
| 4×2 | v156 | on | 15.583 | -0.44% (2/4) | -0.61% (4/4) | — | -16.72% (4/4) | 223.8 | — |
| 4×2 | v156 | release | 15.676 | +0.16% (2/4) | +0.00% (0/4) | — | -16.32% (4/4) | 213.2 | — |
| 8×2 | catalog | aeglos | 31.440 | +51.51% (0/6) | +51.25% (0/6) | +52.71% (0/6) | +0.00% (0/6) | 2463.4 | 6.01× / 2.35× |
| 8×2 | catalog | archive | 20.604 | -0.59% (5/6) | -0.39% (4/6) | +0.00% (0/6) | -34.52% (6/6) | 394.4 | 9.17× / 3.59× |
| 8×2 | catalog | off | 20.732 | +0.00% (0/6) | -0.03% (3/6) | +0.59% (1/6) | -34.00% (6/6) | 410.4 | 9.12× / 3.57× |
| 8×2 | catalog | on | 20.556 | -0.77% (4/6) | -0.75% (4/6) | -0.23% (4/6) | -34.66% (6/6) | 414.7 | 9.19× / 3.60× |
| 8×2 | catalog | release | 20.738 | +0.03% (3/6) | +0.00% (0/6) | +0.39% (2/6) | -33.88% (6/6) | 422.6 | 9.11× / 3.57× |
| 8×2 | heldout | aeglos | 31.877 | +50.86% (0/6) | +51.65% (0/6) | +52.78% (0/6) | +0.00% (0/6) | 2482.6 | — |
| 8×2 | heldout | archive | 20.864 | -1.89% (6/6) | -1.21% (5/6) | +0.00% (0/6) | -34.55% (6/6) | 408.4 | — |
| 8×2 | heldout | off | 21.052 | +0.00% (0/6) | +0.45% (1/6) | +1.93% (0/6) | -33.71% (6/6) | 409.1 | — |
| 8×2 | heldout | on | 20.771 | -2.03% (5/6) | -1.05% (4/6) | -0.02% (3/6) | -34.61% (6/6) | 418.9 | — |
| 8×2 | heldout | release | 20.944 | -0.45% (5/6) | +0.00% (0/6) | +1.23% (1/6) | -34.05% (6/6) | 405.2 | — |
| 8×2 | v153 | aeglos | 31.721 | +51.21% (0/6) | +49.18% (0/6) | +51.48% (0/6) | +0.00% (0/6) | 2466.3 | — |
| 8×2 | v153 | archive | 20.882 | -0.76% (5/6) | -1.07% (5/6) | +0.00% (0/6) | -33.98% (6/6) | 415.2 | — |
| 8×2 | v153 | off | 21.011 | +0.00% (0/6) | -0.09% (3/6) | +0.77% (1/6) | -33.87% (6/6) | 416.6 | — |
| 8×2 | v153 | on | 20.811 | -0.84% (4/6) | -1.27% (4/6) | -0.46% (3/6) | -34.61% (6/6) | 410.7 | — |
| 8×2 | v153 | release | 21.177 | +0.09% (3/6) | +0.00% (0/6) | +1.08% (1/6) | -32.97% (6/6) | 418.1 | — |
| 8×2 | v156 | aeglos | 31.533 | +48.44% (0/6) | +48.98% (0/6) | +51.39% (0/6) | +0.00% (0/6) | 2495.8 | — |
| 8×2 | v156 | archive | 20.650 | -1.86% (5/6) | -1.59% (6/6) | +0.00% (0/6) | -33.94% (6/6) | 403.1 | — |
| 8×2 | v156 | off | 20.983 | +0.00% (0/6) | +0.25% (3/6) | +1.89% (1/6) | -32.63% (6/6) | 416.5 | — |
| 8×2 | v156 | on | 20.963 | -0.59% (5/6) | -0.32% (5/6) | +1.18% (0/6) | -33.16% (6/6) | 413.6 | — |
| 8×2 | v156 | release | 21.065 | -0.25% (3/6) | +0.00% (0/6) | +1.61% (0/6) | -32.88% (6/6) | 408.3 | — |

[Proof samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU on measured C7a profiles.

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
