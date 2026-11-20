# Comparing two- and four-component B2 layouts on small proofs

A four-component split with 13-bit windows appears to save about 6% of modeled curve additions versus the Ryzen 12-bit baseline. Matching the window width explains the advantage. For the original 1×2 request, two components at 13 bits need **574,834** modeled additions and four components in two separately weighted pairs need **575,565**. Both use 20 range jobs. The native four-component split took **27.5–28.1 ns per scalar** on one Ryzen worker, compared with **7.5–7.6 ns** for the existing split.

The four-component trial uses the BN254 scalar eigenvalues of the cube-root map and the twist Frobenius map. It applies the signed decomposition to the existing B2 coefficient vectors. We checked the lattice determinant and all four basis identities. All 144,752 native decompositions matched the high-precision arithmetic model. A separate group oracle used 1,141 sampled coefficients with synthetic subgroup points and checked the reconstructed group sums. [gnark-crypto BN254 lattice](https://github.com/Consensys-Incorporated/gnark-crypto/blob/v0.22.0/ecc/bn254/bn254.go) · [Smith on scalar decompositions](https://eprint.iacr.org/2013/672)

## Work at matched window widths

The captured vectors enter the actual exclusive B2 route on the pinned SDK profiles. The 2×2 inputs are captured after grouping. Each request was solved in two fresh processes, which yielded byte-identical B2 inputs. Gnark independently verified all ten exported proofs. The captures omit the scalar values from the published artifacts.

Counts use the exact signed digits, ignore zero digits and identity bases, and model each bucket's first insert, range merges and the Boolean form of the layered weighted reduction. The calculation assumes nonempty buckets do not cancel. It excludes projective folding, special leading-window handling, field-transform cost and execution overhead. The four-component paired layout computes two separate weighted sums and applies the Frobenius map after weighting.

| Input | B2 terms | Two, 12-bit × 2 | Two, 11-bit × 4 | Two, 13-bit × 2 | Four, 13-bit × 2 per pair |
|---|---:|---:|---:|---:|---:|
| 1x2/catalog | 30,089 | 612,045 | 656,411 | 574,834 | 575,565 |
| 1x2/heldout | 30,089 | 613,080 | 657,497 | 575,758 | 576,437 |
| 1x2/v27 | 30,089 | 613,067 | 657,492 | 575,731 | 576,503 |
| 2x2/catalog | 27,218 | 617,705 | 662,597 | 579,926 | 580,652 |
| 2x2/heldout | 27,267 | 618,758 | 663,746 | 580,875 | 581,592 |

The original 1×2 matched-width layouts use 81,920 logical bucket slots across all jobs. The 12-bit baseline uses 45,056. The two-component 13-bit case has 71,375 modeled cube-root bucket transforms, against 70,603 for the four-component paired layout. Applying all four components to the same buckets reduces curve additions further, at the cost of 204,558 cube-root and 71,040 Frobenius bucket transforms in the 13-bit, four-range model. Peak memory depends on how many jobs hold their buffers concurrently.

The work counts support measuring the existing two-component recurrence at 13 bits before implementing the four-component MSM. The capture runs are correctness runs. The recorded [C7a scorecard](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) still shows five of eight shapes at 10× M5 CPU.

[Model and counts](artifacts/model.json) · [Model code](artifacts/model.py) · [Native split measurements](artifacts/native.json) · [Capture source](artifacts/source.patch) · [Verification](artifacts/preflight.json)
