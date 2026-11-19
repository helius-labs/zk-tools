# Exact linear dependencies among B-query key columns

A B-side constraint matrix has one column per witness wire used by the B query. If a column is a linear combination of other columns, the corresponding B2 key point has the same linear decomposition. The prover can add the appropriate multiples of the eliminated wire scalar to the surviving wire scalars and run a smaller query. This operation requires no witness-specific key data.

The reader expands the original constraint operands including assigned terms and stored partial sums. Singleton rows force their column coefficient to zero in any null relation, and removing them leaves a smaller matrix for exact sparse elimination over BN254 Fr. Back-substitution constructs each relation. The scanner checks the reconstructed relations against every original row. A separate Arkworks implementation validates each relation using the actual affine B2 key bytes and validates curve and subgroup membership. The digit model applies all folds to captured scalars using the original signed width-12 two-component recoding. Positive values indicate fewer nonzero bucket updates. Online regrouping and field multiplications can change the practical cost.

| Shape | B2 columns | Singleton pivots | Total rank | Dependent columns | Surviving terms per relation and count | Range of modeled digit savings |
|---|---:|---:|---:|---:|---|---:|
| 1x2 | 30,089 | 20,657 | 29,717 | 372 | 1: 372 | +0.799% to +0.856% |
| 2x2 | 52,274 | 35,837 | 51,587 | 687 | 1: 687 | +0.849% to +0.889% |

All 17 archived proof exports passed independent verification again. No new proof timing was measured. [Exact counts and input hashes](artifacts/results.json) · [Curve check](artifacts/point_check.rs)

To reproduce the rank and group checks from this report directory, supply the original key image and its SHA-256 from the results artifact.

```sh
python3 artifacts/scan.py "$IMAGE" "$IMAGE_SHA256" /tmp/b-column-points.bin
cargo run --release --locked --manifest-path artifacts/Cargo.toml -- /tmp/b-column-points.bin
```
