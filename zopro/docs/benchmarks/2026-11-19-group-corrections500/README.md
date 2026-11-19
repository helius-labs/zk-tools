# Selecting scalar representatives and combining duplicate corrections

The prepared group normally uses its first member’s scalar. The vote variant chooses the middle member when its scalar matches the last member. Every group member, including the first, contributes its exact difference from the selected representative. An independent option sends residuals through the existing duplicate-aware curve-query functions. The prepared base sum is unchanged.

The original 12×2 and adjacent request 59 use the extended plan; scattered request 141 uses the mixed plan. Each uses 28 Ryzen workers. The larger shapes use their previously screened plans and 32 workers. Three randomized blocks include both the previous grouped binary and the recompiled control. Each timed process primes three times and measures eleven full proofs.

| Shape | Request | Frozen / off / vote / residual grouping / both ms | Both vs off | Both vs frozen |
|---|---:|---:|---:|---:|
| 12x2 | Original | 31.996 / 32.060 / 31.862 / 29.340 / 29.508 | -8.74% (3/3) | -7.61% (3/3) |
| 12x2 | 59 | 32.378 / 32.440 / 32.514 / 29.718 / 29.452 | -9.21% (3/3) | -8.17% (3/3) |
| 12x2 | 141 | 47.438 / 50.260 / 50.646 / 49.753 / 48.886 | -2.19% (3/3) | +2.48% (0/3) |
| 16x2 | Original | 39.538 / 40.349 / 37.829 / 38.982 / 38.788 | -3.87% (3/3) | -1.83% (3/3) |
| 16x2 | 73 | 36.475 / 37.650 / 37.337 / 37.376 / 37.137 | -1.03% (3/3) | +1.82% (0/3) |
| 24x2 | Original | 65.081 / 65.661 / 60.897 / 61.326 / 59.510 | -9.37% (2/3) | -6.85% (2/3) |
| 24x2 | 97 | 58.225 / 58.483 / 59.179 / 59.033 / 59.666 | +0.58% (1/3) | +2.36% (0/3) |

All 119 exported proofs passed independent Gnark verification. [Measurements](artifacts/processes.json.gz) · [Correction counts](artifacts/probes.json) · [Experimental change](artifacts/incremental.patch) · [Verification](artifacts/verification.json)
