# Recognized Poseidon origins in the B2 classifier

The experiment uses the schedule's recognized Poseidon spans to label B2 terms. The key retains the B2 positions of each span's first-round state and the origin of each covered B2 term. These are integer indices derived from solver recognition. The key retains no witness values or curve points. The probe in the [coverage study](../2026-11-19-b2-poseidon-census774/README.md) measured each origin against the solved values.

During proving, the classifier hashes each span state once. For every labeled term, its state class and output coordinate select the scalar class. An exact four-limb comparison verifies every reuse. Unlabeled terms and mismatches go through the ordinary exact dictionary. Different class labels can remain separate even when their scalars happen to match. The optional workspace is charged and denied attempts fall back to the ordinary classifier.

Five matched process blocks on the primary inputs and four on the additional 8×2 inputs used 24 main and eight frontend logical threads on Ryzen. Three warmups preceded 13 timed requests. The released 8×2 control uses the confirmed serial SDK option. The same-binary comparisons hold the scalar conversion policy fixed. Separate route checks verify activation and the zero optional-memory fallback. All 152 exports independently verified.

The following profiles led consistently enough for a C7a comparison. 4×2 origins-parallel.

| Shape | Input | Profile | Proof ms | Paired vs same conversion without origins | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.477 | +0.30% (2/5) | +0.00% (0/5) | 206.0 |
| 4×2 | catalog | parallel | 14.497 | +0.00% (0/5) | -0.30% (3/5) | 200.2 |
| 4×2 | catalog | serial | 14.374 | +0.00% (0/5) | -0.49% (3/5) | 202.8 |
| 4×2 | catalog | origins-parallel | 14.463 | -0.34% (3/5) | -0.10% (3/5) | 206.2 |
| 4×2 | catalog | origins-serial | 14.458 | +0.06% (2/5) | -0.53% (3/5) | 205.0 |
| 4×2 | heldout | release | 14.524 | +0.13% (2/5) | +0.00% (0/5) | 211.8 |
| 4×2 | heldout | parallel | 14.516 | +0.00% (0/5) | -0.13% (3/5) | 203.5 |
| 4×2 | heldout | serial | 14.420 | +0.00% (0/5) | -0.43% (3/5) | 209.8 |
| 4×2 | heldout | origins-parallel | 14.465 | -0.35% (3/5) | -0.22% (3/5) | 217.2 |
| 4×2 | heldout | origins-serial | 14.429 | +0.70% (2/5) | -0.68% (3/5) | 208.8 |
| 8×2 | catalog | release | 22.390 | +0.50% (2/5) | +0.00% (0/5) | 406.8 |
| 8×2 | catalog | parallel | 22.473 | +0.00% (0/5) | -0.50% (3/5) | 391.2 |
| 8×2 | catalog | serial | 22.296 | +0.00% (0/5) | -1.36% (3/5) | 409.0 |
| 8×2 | catalog | origins-parallel | 22.578 | +0.40% (2/5) | +0.64% (2/5) | 401.5 |
| 8×2 | catalog | origins-serial | 22.417 | +0.30% (2/5) | -0.19% (3/5) | 406.2 |
| 8×2 | heldout | release | 22.487 | -2.11% (4/5) | +0.00% (0/5) | 408.0 |
| 8×2 | heldout | parallel | 22.921 | +0.00% (0/5) | +2.15% (1/5) | 398.5 |
| 8×2 | heldout | serial | 22.370 | +0.00% (0/5) | -0.93% (4/5) | 421.0 |
| 8×2 | heldout | origins-parallel | 22.749 | +0.01% (2/5) | +1.17% (0/5) | 430.2 |
| 8×2 | heldout | origins-serial | 22.682 | +1.10% (0/5) | +0.17% (1/5) | 397.2 |
| 8×2 | v153 | release | 22.448 | -0.36% (4/4) | +0.00% (0/4) | 409.0 |
| 8×2 | v153 | parallel | 22.635 | +0.00% (0/4) | +0.36% (0/4) | 395.2 |
| 8×2 | v153 | serial | 22.422 | +0.00% (0/4) | -0.44% (3/4) | 404.0 |
| 8×2 | v153 | origins-parallel | 22.574 | -0.20% (2/4) | +0.44% (1/4) | 398.5 |
| 8×2 | v153 | origins-serial | 22.483 | +0.27% (1/4) | +0.05% (2/4) | 410.5 |
| 8×2 | v156 | release | 22.502 | +0.22% (1/4) | +0.00% (0/4) | 398.8 |
| 8×2 | v156 | parallel | 22.379 | +0.00% (0/4) | -0.22% (3/4) | 398.0 |
| 8×2 | v156 | serial | 22.549 | +0.00% (0/4) | +0.63% (1/4) | 414.5 |
| 8×2 | v156 | origins-parallel | 22.782 | +0.63% (2/4) | +1.20% (1/4) | 399.8 |
| 8×2 | v156 | origins-serial | 22.596 | +0.55% (1/4) | +0.74% (1/4) | 407.8 |

## Route checks

| Shape | Input | Origin classes | Reused labels | Mismatches sent to ordinary hashing | Total classes |
|---|---|---:|---:|---:|---:|
| 4×2 | catalog | 16,708 | 39,345 | 0 | 29,738 |
| 4×2 | heldout | 15,766 | 38,457 | 0 | 29,961 |
| 8×2 | catalog | 20,561 | 95,130 | 0 | 38,315 |
| 8×2 | heldout | 21,011 | 94,680 | 0 | 39,891 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Verification](artifacts/verification.json) · [C7a scorecard](../2026-11-19-b2-serial-sdk-c7a773/README.md)
