# Structural equality of solver outputs before MSM

The census gives exact expression IDs to validated solver operations. It folds constants and neutral elements, recognizes commutative operations and sorted sum terms, and tracks supported hints and conditional division. Input wires start as separate symbols. Where two query operands share an ID, the solver expression is identical under these rules. The model checks every proposed class and constant against the actual field values on each proof.

This diagnostic measures opportunities to share work before seeing a witness. It also counts scalar equalities on each actual witness. The dynamic column excludes zeros. The structural column counts repeated IDs, including repetitions of any statically known zero. Across the queries examined, the largest structural reduction was 0.28% of entries. Differences from the observed equality counts show how much grouping cannot be inferred by this syntactic model.

The diagnostic does not combine points, omit curve queries or change proof generation. All 8 exported proofs independently verified. A focused test covers constant division, commutativity and exact cancellation. The diagnostic uses a bounded temporary expression table during each checked call.

| Shape | Input | Query | Terms | Structural repeats | Observed nonzero repeats | Known / observed zero |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | A | 21,006 | 23 | 1,819 | 0 / 1,733 |
| 1×2 | catalog | B2 | 30,089 | 83 | 2,347 | 0 / 1,600 |
| 1×2 | catalog | K | 30,529 | 83 | 2,465 | 0 / 1,865 |
| 1×2 | heldout | A | 21,006 | 23 | 1,320 | 0 / 1,604 |
| 1×2 | heldout | B2 | 30,089 | 83 | 1,553 | 0 / 1,476 |
| 1×2 | heldout | K | 30,529 | 83 | 1,678 | 0 / 1,733 |
| 2×2 | catalog | A | 36,637 | 30 | 14,302 | 0 / 3,067 |
| 2×2 | catalog | B2 | 52,274 | 117 | 20,659 | 0 / 2,763 |
| 2×2 | catalog | K | 53,079 | 117 | 20,901 | 0 / 3,259 |
| 2×2 | heldout | A | 36,637 | 30 | 14,215 | 0 / 2,892 |
| 2×2 | heldout | B2 | 52,274 | 117 | 20,453 | 0 / 2,597 |
| 2×2 | heldout | K | 53,079 | 117 | 20,700 | 0 / 3,083 |
| 4×2 | catalog | A | 67,154 | 44 | 41,001 | 0 / 5,767 |
| 4×2 | catalog | B2 | 95,537 | 181 | 59,873 | 0 / 5,120 |
| 4×2 | catalog | K | 97,075 | 181 | 60,354 | 0 / 6,090 |
| 4×2 | heldout | A | 67,154 | 44 | 40,151 | 0 / 5,517 |
| 4×2 | heldout | B2 | 95,537 | 181 | 58,485 | 0 / 4,893 |
| 4×2 | heldout | K | 97,075 | 181 | 58,980 | 0 / 5,846 |
| 8×2 | catalog | A | 129,064 | 72 | 92,630 | 0 / 11,198 |
| 8×2 | catalog | B2 | 183,347 | 309 | 135,616 | 0 / 9,881 |
| 8×2 | catalog | K | 186,411 | 309 | 136,605 | 0 / 11,833 |
| 8×2 | heldout | A | 129,064 | 72 | 91,818 | 0 / 10,767 |
| 8×2 | heldout | B2 | 183,347 | 309 | 134,184 | 0 / 9,497 |
| 8×2 | heldout | K | 186,411 | 309 | 135,199 | 0 / 11,420 |

[Counts and proof bindings](artifacts/rows.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json)
