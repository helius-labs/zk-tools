# B2 origin grouping on C7a

The experiment uses recognized Poseidon spans to replace individual scalar hashes with origin labels. Every reuse compares the scalar's full representation. Unlabeled terms and mismatches go through the ordinary exact dictionary. The grouping plan holds indices only. The [Ryzen trial](../2026-11-19-b2-poseidon-label-ryzen775/README.md) tested the same classifier. This run adds clocks around classification, including preparation of the grouped scalars.

All Zopro processes use 64 main and 32 frontend cores on one c7a.24xlarge. Prepared Aeglos uses all 96 cores. The ordinary and origins arms use parallel scalar conversion on 4×2 and the released serial option on 8×2. The other and origins-other arms switch to the alternate conversion policy, including its matching reservation. The released control uses the same settings as ordinary.

- **4×2.** Origin-assisted proof 15.500 / 15.540 ms, ordinary 15.356 / 15.590 ms, released Zopro 15.505 / 15.559 ms, prepared Aeglos 18.350 / 18.368 ms across original and second inputs.
- **8×2.** Origin-assisted proof 20.341 / 20.527 ms, ordinary 20.561 / 20.708 ms, released Zopro 20.655 / 20.803 ms, prepared Aeglos 31.080 / 31.114 ms across original and second inputs.

New qualifying measured profiles are recorded in the scorecard below.

## Classification clocks

There are three matched proof blocks per arm with and without logging. Each traced process discarded three warmups and measured nine proofs. Classification occurs while other curve work is active, so savings in this interval need not transfer in full to proof latency.

| Shape | Input | Ordinary / origin classification ms | Whole proof ordinary plain / traced ms | Whole proof origin plain / traced ms | Paired overhead ordinary / origins |
|---|---|---:|---:|---:|---:|
| 4×2 | catalog | 3.454 / 2.974 | 15.364 / 15.513 | 15.553 / 15.566 | -0.33% (2/3) / +0.53% (1/3) |
| 4×2 | heldout | 3.619 / 3.171 | 15.436 / 15.582 | 15.363 / 15.491 | +0.95% (1/3) / +0.77% (0/3) |
| 8×2 | catalog | 4.814 / 3.912 | 20.436 / 20.812 | 20.416 / 20.506 | +1.90% (1/3) / +0.64% (1/3) |
| 8×2 | heldout | 4.883 / 4.066 | 20.942 / 20.753 | 20.629 / 20.554 | +0.16% (1/3) / +0.47% (1/3) |

## Full-proof timing

Primary inputs use six matched blocks and two additional 8×2 inputs use three. Timed processes discarded three priming requests and measured thirteen. Peak RSS is the highest whole-process observation among blocks. All 222 C7a and six Ryzen exports passed independent verification. Separate checks confirm origin reuse and fallback under a zero optional-memory allowance. The temporary AWS resources were removed.

| Shape | Input | Profile | Proof ms | Matched control | Paired vs matched control | Paired vs release | Paired vs Aeglos | Peak MiB | M5 CPU / Metal |
|---|---|---|---:|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | aeglos | 18.350 | ordinary | +19.55% (0/6) | +17.68% (0/6) | +0.00% (0/6) | 1349.0 | 5.94× / 2.62× |
| 4×2 | catalog | ordinary | 15.356 | ordinary | +0.00% (0/6) | -1.35% (5/6) | -16.34% (6/6) | 204.2 | 7.10× / 3.13× |
| 4×2 | catalog | origins | 15.500 | ordinary | +1.05% (0/6) | -0.52% (4/6) | -15.65% (6/6) | 209.4 | 7.03× / 3.10× |
| 4×2 | catalog | origins-other | 15.291 | other | +0.13% (3/6) | -1.40% (6/6) | -16.19% (6/6) | 210.2 | 7.13× / 3.14× |
| 4×2 | catalog | other | 15.293 | other | +0.00% (0/6) | -1.53% (5/6) | -16.04% (6/6) | 205.8 | 7.13× / 3.14× |
| 4×2 | catalog | release | 15.505 | ordinary | +1.37% (1/6) | +0.00% (0/6) | -15.02% (6/6) | 202.0 | 7.03× / 3.10× |
| 4×2 | heldout | aeglos | 18.368 | ordinary | +17.64% (0/6) | +18.25% (0/6) | +0.00% (0/6) | 1345.4 | — |
| 4×2 | heldout | ordinary | 15.590 | ordinary | +0.00% (0/6) | -0.17% (3/6) | -14.99% (6/6) | 195.9 | — |
| 4×2 | heldout | origins | 15.540 | ordinary | -0.32% (3/6) | -0.12% (3/6) | -15.79% (6/6) | 208.8 | — |
| 4×2 | heldout | origins-other | 15.284 | other | +0.38% (2/6) | -2.09% (5/6) | -16.85% (6/6) | 199.0 | — |
| 4×2 | heldout | other | 15.322 | other | +0.00% (0/6) | -1.68% (5/6) | -16.62% (6/6) | 213.1 | — |
| 4×2 | heldout | release | 15.559 | ordinary | +0.17% (3/6) | +0.00% (0/6) | -15.43% (6/6) | 204.1 | — |
| 8×2 | catalog | aeglos | 31.080 | ordinary | +51.26% (0/6) | +50.27% (0/6) | +0.00% (0/6) | 2487.5 | 6.08× / 2.38× |
| 8×2 | catalog | ordinary | 20.561 | ordinary | +0.00% (0/6) | -0.19% (3/6) | -33.89% (6/6) | 404.8 | 9.19× / 3.60× |
| 8×2 | catalog | origins | 20.341 | ordinary | -1.47% (6/6) | -1.45% (6/6) | -34.82% (6/6) | 416.1 | 9.29× / 3.64× |
| 8×2 | catalog | origins-other | 20.377 | other | -1.72% (6/6) | -1.10% (6/6) | -34.48% (6/6) | 400.3 | 9.28× / 3.63× |
| 8×2 | catalog | other | 20.768 | other | +0.00% (0/6) | +0.74% (1/6) | -33.09% (6/6) | 417.5 | 9.10× / 3.56× |
| 8×2 | catalog | release | 20.655 | ordinary | +0.19% (3/6) | +0.00% (0/6) | -33.45% (6/6) | 418.8 | 9.15× / 3.58× |
| 8×2 | heldout | aeglos | 31.114 | ordinary | +50.92% (0/6) | +49.70% (0/6) | +0.00% (0/6) | 2499.5 | — |
| 8×2 | heldout | ordinary | 20.708 | ordinary | +0.00% (0/6) | -0.76% (4/6) | -33.74% (6/6) | 421.8 | — |
| 8×2 | heldout | origins | 20.527 | ordinary | -1.02% (4/6) | -1.11% (6/6) | -34.06% (6/6) | 406.8 | — |
| 8×2 | heldout | origins-other | 20.673 | other | -1.13% (5/6) | -0.68% (5/6) | -33.74% (6/6) | 415.4 | — |
| 8×2 | heldout | other | 20.927 | other | +0.00% (0/6) | +0.60% (1/6) | -32.74% (6/6) | 412.2 | — |
| 8×2 | heldout | release | 20.803 | ordinary | +0.77% (2/6) | +0.00% (0/6) | -33.20% (6/6) | 411.4 | — |
| 8×2 | v153 | ordinary | 20.674 | ordinary | +0.00% (0/3) | +1.19% (1/3) | — | 402.5 | — |
| 8×2 | v153 | origins | 20.509 | ordinary | +0.03% (1/3) | -0.46% (2/3) | — | 403.2 | — |
| 8×2 | v153 | release | 20.603 | ordinary | -1.18% (2/3) | +0.00% (0/3) | — | 397.2 | — |
| 8×2 | v156 | ordinary | 20.750 | ordinary | +0.00% (0/3) | +0.14% (1/3) | — | 409.9 | — |
| 8×2 | v156 | origins | 20.870 | ordinary | +0.58% (1/3) | +0.72% (1/3) | — | 418.4 | — |
| 8×2 | v156 | release | 20.722 | ordinary | -0.14% (2/3) | +0.00% (0/3) | — | 407.1 | — |

[Samples](artifacts/rows.json.gz) · [Summary](artifacts/summary.json) · [Workflow](artifacts/workflow.json.gz) · [Classifier](artifacts/source.patch) · [Clocks](artifacts/clock.patch) · [Verification](artifacts/verification.json)

## Eight supplied M5 reference timings

**4/8** original requests reach 10× M5 CPU. [Preceding scorecard](../2026-11-19-b2-serial-sdk-c7a773/README.md)

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 737 dedicated G1 80+16 | 7.953 | 37 / 24 | 4.65× / 3.02× | +4.253 |
| 2×2 | 750 archived-band 72+24 | 12.358 | 59 / 30 | 4.77× / 2.43× | +6.458 |
| 4×2 | 757 solve-front-on | 15.257 | 109 / 48 | 7.14× / 3.15× | +4.357 |
| 8×2 | 776 recognized B2 origins | 20.341 | 189 / 74 | 9.29× / 3.64× | +1.441 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
