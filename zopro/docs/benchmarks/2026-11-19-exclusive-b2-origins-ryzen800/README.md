# Poseidon origins for exclusive 2×2 G2 grouping

This trial uses recognized Poseidon spans to classify B2 coefficients in the exclusive G2 path. It compares every candidate class against the actual canonical scalar. Unlabeled terms follow the ordinary hash path. The index plan and scratch use the existing optional memory account. The experiment keeps the native G2 arithmetic and released medium K/H option in every timing arm.

Six paired blocks on the Ryzen host compare the released SDK, the same-build ordinary classifier, and the origin path on both 2×2 inputs. The process uses 24 main and eight frontend logical CPUs. Each process primes three calls and measures twenty-one complete proofs. All 42 exports independently verified. Separate captures use zero optional-memory allowance.

- 2×2 changed by -0.34% and -0.35% versus the same-build control.

| Shape | Input | B2 terms | Origin-assisted classes |
|---|---|---:|---:|
| 2×2 | catalog | 52274 | 27218 |
| 2×2 | heldout | 52274 | 27267 |

A dash means the origin classifier was unavailable or declined admission.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 2×2 | catalog | release | 9.479 | +0.14% (3/6) | +0.00% (0/6) | 94.5 |
| 2×2 | catalog | ordinary | 9.462 | +0.00% (0/6) | -0.13% (3/6) | 98.8 |
| 2×2 | catalog | origins | 9.440 | -0.34% (6/6) | -0.45% (4/6) | 99.0 |
| 2×2 | heldout | release | 9.460 | -0.16% (4/6) | +0.00% (0/6) | 99.0 |
| 2×2 | heldout | ordinary | 9.474 | +0.00% (0/6) | +0.16% (2/6) | 96.8 |
| 2×2 | heldout | origins | 9.431 | -0.35% (4/6) | -0.30% (4/6) | 99.5 |

[Samples](artifacts/rows.json.gz) · [Routes](artifacts/routes.json) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
