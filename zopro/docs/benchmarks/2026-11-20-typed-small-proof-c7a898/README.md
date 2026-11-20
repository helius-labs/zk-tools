# Typed proof handoff for 1×2 and 2×2 on C7a

The fused curve solver can pass projective A, K/H and B2 outputs directly to rerandomization, avoiding intermediate affine normalization and byte conversion. The `final` profile changes only the return from rerandomization to verification, using affine proof objects in memory. The `projective` profile also passes the fused parts directly. The `batch` profile additionally normalizes both final G1 points with one inversion. All profiles use fresh blinding and the same pairing verification. The typed validation checks the points on their curves and uses the existing generated-output subgroup certificate. The uncertified route retains the original parsing and subgroup checks. A native test compared the old and new points under identical blinding and exercised the identity-A fallback.

The 1×2 and 2×2 release controls retain the four-range 80+16 and two-range 64+32 profiles, respectively. Prepared Aeglos uses all 96 cores. Each process primes three proofs and times thirteen. Eight counterbalanced blocks per input compare the frozen release, same-build ordinary, three typed variants, and prepared Aeglos. A typed mode needs a lower paired median and at least six wins against each Zopro control on both inputs. Peak RSS covers the entire process.

On 1×2, release measured 7.930 / 7.875 ms and prepared Aeglos measured 8.224 / 8.465 ms. The fastest typed profile on the original input was `projective` at 7.872 / 7.837 ms, with paired changes of -0.45% / -0.48% against the same-build ordinary profile. Every typed profile fell short of the paired requirement on at least one input.

On 2×2, release measured 12.101 / 12.149 ms and prepared Aeglos measured 11.901 / 11.856 ms. The fastest typed profile on the original input was `final` at 12.120 / 12.170 ms, with paired changes of -0.01% / -0.27% against the same-build ordinary profile. Every typed profile fell short of the paired requirement on at least one input.

| Shape | Input | Profile | Proof ms | Paired vs release | Paired vs ordinary | Paired vs Aeglos | Peak MiB | Blocks meeting 10× M5 CPU |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | catalog | release | 7.930 | +0.00% (0/8) | +0.29% (2/8) | -3.53% (6/8) | 40.0 | 0/8 |
| 1x2 | catalog | ordinary | 7.895 | -0.29% (6/8) | +0.00% (0/8) | -3.60% (7/8) | 41.4 | 0/8 |
| 1x2 | catalog | final | 7.899 | -0.21% (4/8) | -0.08% (4/8) | -3.74% (6/8) | 43.0 | 0/8 |
| 1x2 | catalog | projective | 7.872 | -0.51% (5/8) | -0.45% (6/8) | -4.39% (6/8) | 44.4 | 0/8 |
| 1x2 | catalog | batch | 7.936 | -0.44% (6/8) | +0.21% (2/8) | -3.21% (7/8) | 43.8 | 0/8 |
| 1x2 | catalog | aeglos | 8.224 | +3.66% (2/8) | +3.75% (1/8) | +0.00% (0/8) | 455.7 | 0/8 |
| 1x2 | heldout | release | 7.875 | +0.00% (0/8) | -0.23% (4/8) | -7.12% (7/8) | 42.2 | 0/8 |
| 1x2 | heldout | ordinary | 7.898 | +0.23% (4/8) | +0.00% (0/8) | -7.00% (7/8) | 41.4 | 0/8 |
| 1x2 | heldout | final | 7.854 | -0.42% (5/8) | -0.57% (6/8) | -6.99% (7/8) | 42.3 | 0/8 |
| 1x2 | heldout | projective | 7.837 | -0.18% (5/8) | -0.48% (6/8) | -7.17% (8/8) | 39.2 | 0/8 |
| 1x2 | heldout | batch | 7.859 | -0.52% (5/8) | -0.54% (7/8) | -7.10% (7/8) | 40.0 | 0/8 |
| 1x2 | heldout | aeglos | 8.465 | +7.67% (1/8) | +7.53% (1/8) | +0.00% (0/8) | 460.5 | 0/8 |
| 2x2 | catalog | release | 12.101 | +0.00% (0/8) | +0.36% (3/8) | +2.29% (0/8) | 73.1 | 0/8 |
| 2x2 | catalog | ordinary | 12.085 | -0.36% (5/8) | +0.00% (0/8) | +2.02% (0/8) | 72.2 | 0/8 |
| 2x2 | catalog | final | 12.120 | +0.32% (3/8) | -0.01% (4/8) | +1.57% (0/8) | 76.0 | 0/8 |
| 2x2 | catalog | projective | 12.193 | +0.49% (3/8) | +1.01% (1/8) | +2.74% (0/8) | 80.4 | 0/8 |
| 2x2 | catalog | batch | 12.161 | +0.02% (4/8) | +0.74% (3/8) | +1.89% (0/8) | 76.0 | 0/8 |
| 2x2 | catalog | aeglos | 11.901 | -2.24% (8/8) | -1.98% (8/8) | +0.00% (0/8) | 809.4 | 0/8 |
| 2x2 | heldout | release | 12.149 | +0.00% (0/8) | -0.41% (5/8) | +2.17% (0/8) | 72.9 | 0/8 |
| 2x2 | heldout | ordinary | 12.149 | +0.42% (3/8) | +0.00% (0/8) | +2.33% (1/8) | 81.3 | 0/8 |
| 2x2 | heldout | final | 12.170 | +0.24% (3/8) | -0.27% (4/8) | +2.53% (1/8) | 75.2 | 0/8 |
| 2x2 | heldout | projective | 12.297 | +0.92% (1/8) | +0.63% (4/8) | +3.09% (0/8) | 76.7 | 0/8 |
| 2x2 | heldout | batch | 12.234 | +0.44% (3/8) | -0.44% (5/8) | +2.52% (0/8) | 74.5 | 0/8 |
| 2x2 | heldout | aeglos | 11.856 | -2.12% (8/8) | -2.27% (7/8) | +0.00% (0/8) | 806.6 | 0/8 |

| Shape | Input | Projective vs final | Batch vs projective |
|---|---|---:|---:|
| 1x2 | catalog | -0.23% (4/8) | +0.83% (2/8) |
| 1x2 | heldout | -0.28% (5/8) | -0.12% (4/8) |
| 2x2 | catalog | +0.69% (2/8) | +0.10% (3/8) |
| 2x2 | heldout | +0.82% (2/8) | -0.52% (5/8) |

All 243 exports independently verified against the pinned Gnark verifier, verifying key and public input. This includes 17 Ryzen preflight proofs, 17 C7a preflight proofs and 209 C7a workflow proofs. The traces confirmed the fused and byte paths, the uncertified fallback on both shapes and the 2×2 raw-parts fallback at zero optional workspace. The temporary C7a instance and storage were removed.

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Route probes](artifacts/preflight.json.gz) · [Candidate source](artifacts/source.patch) · [Verification](artifacts/verification.json)

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
