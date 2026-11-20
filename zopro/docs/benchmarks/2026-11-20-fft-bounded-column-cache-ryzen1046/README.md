# Direct allocation for the Euler FFT column cache on Ryzen

The trial fills one fixed-size twiddle buffer per eligible FFT level. It reserves the requested storage plus a small allowance per allocation. The original builder allocates temporary vectors per column and reserves 256 bytes per domain element. At 12×2 that request is 128 MiB against a 64 MiB process-wide ceiling, which prevents the original cache from being built. The direct builder keeps the same ceiling, domain limit, and FFT arithmetic.

Paired full-proof latency changed by -2.74% to -0.85% against the same-build control. Peak RSS changed by -13.0 to +72.4 MiB. The candidate meets the local latency criterion on 12×2, 16×2. C7a confirmation is pending.

| Shape | Input | Frozen / control / direct ms | Direct vs control | Direct vs frozen | Control / direct peak MiB |
|---|---|---:|---:|---:|---:|
| 8×2 | catalog | 24.621 / 24.817 / 24.573 | -1.24% (5/6) | -0.62% (5/6) | 390.2 / 377.2 |
| 8×2 | heldout | 24.831 / 24.916 / 24.709 | -0.97% (5/6) | -0.36% (4/6) | 398.5 / 389.8 |
| 12×2 | catalog | 32.314 / 32.216 / 31.250 | -2.57% (6/6) | -3.56% (6/6) | 493.8 / 542.1 |
| 12×2 | heldout | 32.250 / 32.209 / 31.281 | -2.74% (5/6) | -2.76% (5/6) | 521.8 / 546.7 |
| 16×2 | catalog | 39.765 / 39.587 / 38.835 | -1.90% (6/6) | -2.32% (6/6) | 621.1 / 648.1 |
| 16×2 | heldout | 39.440 / 39.308 / 38.657 | -1.77% (6/6) | -1.83% (6/6) | 626.3 / 661.8 |
| 24×2 | catalog | 63.828 / 63.102 / 62.454 | -0.85% (6/6) | -2.28% (5/6) | 934.3 / 949.3 |
| 24×2 | heldout | 63.698 / 63.083 / 62.568 | -1.46% (4/6) | -1.98% (5/6) | 893.8 / 966.2 |

Six order-balanced blocks measure thirteen completed proofs per process after three warmups on the Ryzen 9950X with 24 main and eight frontend logical workers. The frozen binary, requests, and images come from the three-prover C7a benchmark. Peak memory is the maximum whole-process RSS across the six blocks. A shape qualifies on both requests with at least five paired wins and a lower paired median against each control, plus same-build control calibration within 2% of the frozen binary.

## Setup probes

| Shape | Input | Control / direct root payload MiB | Control / direct setup ms |
|---|---|---:|---:|
| 8×2 | catalog | 9.99 / 19.98 | 3.459 / 1.310 |
| 8×2 | heldout | 9.99 / 19.98 | 3.595 / 1.317 |
| 12×2 | catalog | 0.00 / 39.96 | 0.000 / 2.930 |
| 12×2 | heldout | 0.00 / 39.96 | 0.000 / 2.982 |
| 16×2 | catalog | 0.00 / 39.96 | 0.000 / 2.828 |
| 16×2 | heldout | 0.00 / 39.96 | 0.000 / 2.878 |
| 24×2 | catalog | 0.00 / 39.96 | 0.000 / 3.011 |
| 24×2 | heldout | 0.00 / 39.96 | 0.000 / 2.963 |

Root payload sums the initialized buffers across successful cache allocations in a separate process per case. Setup time sums the successful builder calls. At 24×2 only one of three cache attempts fit under the shared ceiling. The zero-budget probe declined all attempts for every shape and both requests, then completed a valid proof.

The native test checked allocated capacities and every stored root against the source layers on three domains in both directions. Gnark independently reverified all 168 proof exports across timing, setup, and zero-budget runs.

[Summary](artifacts/summary.json) · [Block samples and setup probes](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Trial patch](artifacts/source.patch.gz) · [C7a baseline](../2026-11-20-c7a-three-provers/README.md)
