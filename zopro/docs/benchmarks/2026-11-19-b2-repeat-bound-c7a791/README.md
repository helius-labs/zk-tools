# Fixed-request diagnostic for B2 preparation

This benchmark freezes the first request's B2 scalar groups. A second arm also freezes the affine sums for those groups. Each subsequent proof still solves the request, requires equality of the full B2 scalar vector, runs the original grouped MSM arithmetic, and assembles a fresh proof. The normal path handles a different witness or denied reservation. Reuse is confined to one loaded immutable key in a benchmark process. This diagnostic estimates how much latency repeated B2 preparation adds while other proof phases are active. The equality comparison and retained buffers also change the execution cost.

The same trial executable runs ordinary preparation and both replay arms. The released SDK provides a second control. Both binaries use Poseidon origins and serial B2 conversion on 8×2. 4×2 uses ordinary grouping. Ryzen uses 24 main and eight frontend logical CPUs. C7a uses 64 main and 32 frontend physical cores. Processes run three priming and thirteen measured requests. Routing checks use both original and held-out transactions, swap the priming request to exercise a cache miss, and exercise a zero optional-memory allowance.

All 224 exported proofs independently verified. The temporary C7a instance, volume and bucket were removed.

- **ryzen.** 8×2 saved +0.306 / -0.207 ms by reusing scalar classes and +1.715 / +1.303 ms by reusing classes and point sums, on the original / held-out inputs.
- **c7a.** 8×2 saved +0.280 / +0.213 ms by reusing scalar classes and +0.404 / +0.350 ms by reusing classes and point sums, on the original / held-out inputs.

The fixed-request replay reached 20.019 ms, still 1.119 ms above the 8×2 target. Reusing the groups and their point sums under this schedule leaves a measurable gap. The replay compares the full scalar vector, so its latency includes that comparison. Production prover code uses the ordinary preparation path. The experimental values in this report are excluded from the M5 scorecard.

| Host | Shape | Input | Mode | Proof ms | Paired Δ vs ordinary ms | Paired vs ordinary | Paired vs classes | Peak MiB |
|---|---|---|---|---:|---:|---:|---:|---:|
| ryzen | 4×2 | catalog | release | 14.442 | +0.101 | +0.70% (1/4) | -0.12% (2/4) | 211.8 |
| ryzen | 4×2 | catalog | ordinary | 14.436 | +0.000 | +0.00% (0/4) | +0.20% (2/4) | 202.8 |
| ryzen | 4×2 | catalog | classes | 14.407 | -0.029 | -0.20% (2/4) | +0.00% (0/4) | 180.5 |
| ryzen | 4×2 | catalog | affine | 13.886 | -0.588 | -4.07% (3/4) | -3.88% (3/4) | 181.2 |
| ryzen | 4×2 | heldout | release | 14.547 | +0.117 | +0.81% (1/4) | +0.55% (2/4) | 198.8 |
| ryzen | 4×2 | heldout | ordinary | 14.438 | +0.000 | +0.00% (0/4) | +0.08% (2/4) | 201.5 |
| ryzen | 4×2 | heldout | classes | 14.460 | -0.011 | -0.06% (2/4) | +0.00% (0/4) | 183.8 |
| ryzen | 4×2 | heldout | affine | 13.907 | -0.521 | -3.60% (4/4) | -3.38% (4/4) | 170.2 |
| ryzen | 8×2 | catalog | release | 22.896 | +0.160 | +0.72% (2/4) | +1.88% (1/4) | 413.2 |
| ryzen | 8×2 | catalog | ordinary | 22.544 | +0.000 | +0.00% (0/4) | +1.38% (0/4) | 409.2 |
| ryzen | 8×2 | catalog | classes | 22.319 | -0.306 | -1.36% (4/4) | +0.00% (0/4) | 373.2 |
| ryzen | 8×2 | catalog | affine | 20.968 | -1.715 | -7.61% (4/4) | -6.34% (4/4) | 351.5 |
| ryzen | 8×2 | heldout | release | 22.888 | +0.329 | +1.46% (1/4) | +0.74% (1/4) | 409.8 |
| ryzen | 8×2 | heldout | ordinary | 22.545 | +0.000 | +0.00% (0/4) | -0.91% (4/4) | 406.0 |
| ryzen | 8×2 | heldout | classes | 22.719 | +0.207 | +0.92% (0/4) | +0.00% (0/4) | 382.5 |
| ryzen | 8×2 | heldout | affine | 21.200 | -1.303 | -5.79% (4/4) | -6.65% (4/4) | 356.2 |
| c7a | 4×2 | catalog | release | 15.272 | -0.161 | -1.04% (3/4) | +2.36% (0/4) | 210.2 |
| c7a | 4×2 | catalog | ordinary | 15.432 | +0.000 | +0.00% (0/4) | +3.24% (0/4) | 204.3 |
| c7a | 4×2 | catalog | classes | 14.933 | -0.484 | -3.13% (4/4) | +0.00% (0/4) | 194.6 |
| c7a | 4×2 | catalog | affine | 15.124 | -0.249 | -1.63% (4/4) | +1.17% (0/4) | 168.1 |
| c7a | 4×2 | heldout | release | 15.489 | +0.041 | +0.27% (2/4) | +3.15% (0/4) | 208.7 |
| c7a | 4×2 | heldout | ordinary | 15.480 | +0.000 | +0.00% (0/4) | +3.42% (0/4) | 210.1 |
| c7a | 4×2 | heldout | classes | 14.927 | -0.511 | -3.31% (4/4) | +0.00% (0/4) | 208.7 |
| c7a | 4×2 | heldout | affine | 15.208 | -0.271 | -1.75% (3/4) | +1.99% (0/4) | 165.1 |
| c7a | 8×2 | catalog | release | 20.397 | -0.041 | -0.20% (5/6) | +1.38% (1/6) | 421.4 |
| c7a | 8×2 | catalog | ordinary | 20.445 | +0.000 | +0.00% (0/6) | +1.39% (1/6) | 411.4 |
| c7a | 8×2 | catalog | classes | 20.083 | -0.280 | -1.37% (5/6) | +0.00% (0/6) | 385.6 |
| c7a | 8×2 | catalog | affine | 20.019 | -0.404 | -1.98% (6/6) | -0.39% (4/6) | 376.3 |
| c7a | 8×2 | heldout | release | 20.594 | -0.024 | -0.11% (3/6) | +0.84% (2/6) | 410.7 |
| c7a | 8×2 | heldout | ordinary | 20.654 | +0.000 | +0.00% (0/6) | +1.04% (1/6) | 406.2 |
| c7a | 8×2 | heldout | classes | 20.435 | -0.213 | -1.03% (5/6) | +0.00% (0/6) | 382.3 |
| c7a | 8×2 | heldout | affine | 20.242 | -0.350 | -1.69% (5/6) | -0.60% (4/6) | 382.2 |
| c7a | 8×2 | v153 | release | 20.787 | +0.139 | +0.68% (1/6) | +1.95% (0/6) | 419.7 |
| c7a | 8×2 | v153 | ordinary | 20.627 | +0.000 | +0.00% (0/6) | +1.82% (1/6) | 421.9 |
| c7a | 8×2 | v153 | classes | 20.284 | -0.369 | -1.78% (5/6) | +0.00% (0/6) | 391.2 |
| c7a | 8×2 | v153 | affine | 20.202 | -0.458 | -2.22% (5/6) | -0.50% (4/6) | 366.2 |
| c7a | 8×2 | v156 | release | 20.562 | -0.053 | -0.25% (4/6) | +1.90% (0/6) | 420.6 |
| c7a | 8×2 | v156 | ordinary | 20.692 | +0.000 | +0.00% (0/6) | +2.28% (0/6) | 414.3 |
| c7a | 8×2 | v156 | classes | 20.301 | -0.460 | -2.23% (6/6) | +0.00% (0/6) | 379.3 |
| c7a | 8×2 | v156 | affine | 20.214 | -0.477 | -2.31% (5/6) | -0.27% (5/6) | 376.8 |

[Source](artifacts/source.patch) · [Routing checks](artifacts/routes.json) · [Ryzen samples](artifacts/ryzen-rows.json.gz) · [C7a samples](artifacts/c7a-rows.json.gz) · [Verification](artifacts/verification.json)

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
