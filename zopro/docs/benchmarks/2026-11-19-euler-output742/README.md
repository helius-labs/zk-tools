# Writing the Euler product during the final column pass

The trial folds the pointwise Euler product into the last forward transform. It reads the original field rows and overwrites A only after the inverse and inner forward transforms have finished. It avoids storing and reloading the complete derivative field buffer after the outer transform. Recursive column bands borrow disjoint output slices, and the trial keeps bounded per-worker scratch. The paired arm applies the existing eight-lane product across two adjacent columns. The quad arm consumes each column independently.

On Ryzen, the process used 24 main and eight frontend hardware threads on disjoint CPU IDs, including SMT siblings. Five paired blocks per request timed thirteen proofs after three warm-ups. The prior released build uses its accepted per-circuit settings. The same-build off arm enables local Euler transforms for all three circuits, so the 16×2 accepted row and off row use different FFT configurations. Native arithmetic tests compared both emitters against materialized derivatives on three domains, varied root caches and unequal input lengths. All 132 proof exports verified. An initial local process set stopped on a source-metadata change from creating a separate benchmark bundle. The reported processes used private copies of the pinned binaries and key images.

| Shape | Input | Mode | Median ms | Paired vs same-build off | Paired vs released profile | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 12x2 | catalog | accepted | 32.370 | -0.16% (3/5) | +0.00% (0/5) | 525.3 |
| 12x2 | catalog | off | 32.188 | +0.00% (0/5) | +0.16% (2/5) | 508.5 |
| 12x2 | catalog | quad | 32.076 | -0.27% (4/5) | -0.50% (5/5) | 504.4 |
| 12x2 | catalog | pair | 31.983 | -1.04% (4/5) | -0.71% (5/5) | 490.6 |
| 12x2 | heldout | accepted | 32.218 | +0.16% (1/5) | +0.00% (0/5) | 515.5 |
| 12x2 | heldout | off | 32.376 | +0.00% (0/5) | -0.16% (4/5) | 489.8 |
| 12x2 | heldout | quad | 32.073 | -0.52% (4/5) | -0.68% (5/5) | 507.3 |
| 12x2 | heldout | pair | 32.081 | -0.50% (4/5) | -0.55% (4/5) | 511.4 |
| 16x2 | catalog | accepted | 39.933 | +0.93% (0/5) | +0.00% (0/5) | 601.9 |
| 16x2 | catalog | off | 39.668 | +0.00% (0/5) | -0.93% (5/5) | 607.7 |
| 16x2 | catalog | quad | 39.277 | -1.08% (3/5) | -1.40% (4/5) | 607.2 |
| 16x2 | catalog | pair | 38.842 | -2.12% (5/5) | -2.49% (5/5) | 604.1 |
| 16x2 | heldout | accepted | 40.143 | +1.49% (0/5) | +0.00% (0/5) | 595.1 |
| 16x2 | heldout | off | 39.555 | +0.00% (0/5) | -1.46% (5/5) | 606.4 |
| 16x2 | heldout | quad | 39.295 | -0.80% (3/5) | -1.78% (5/5) | 620.6 |
| 16x2 | heldout | pair | 39.247 | -0.58% (3/5) | -1.37% (3/5) | 600.5 |
| merge-24x1 | catalog | accepted | 63.082 | -0.17% (3/5) | +0.00% (0/5) | 962.8 |
| merge-24x1 | catalog | off | 63.192 | +0.00% (0/5) | +0.17% (2/5) | 980.8 |
| merge-24x1 | catalog | quad | 62.278 | -0.81% (3/5) | -0.62% (3/5) | 957.8 |
| merge-24x1 | catalog | pair | 62.133 | -0.87% (5/5) | -0.70% (4/5) | 989.5 |
| merge-24x1 | heldout | accepted | 60.686 | -0.26% (4/5) | +0.00% (0/5) | 986.5 |
| merge-24x1 | heldout | off | 60.846 | +0.00% (0/5) | +0.26% (1/5) | 944.6 |
| merge-24x1 | heldout | quad | 60.069 | -1.35% (4/5) | -0.99% (4/5) | 962.9 |
| merge-24x1 | heldout | pair | 59.930 | -1.33% (5/5) | -0.97% (5/5) | 931.8 |

Twelve separate proof exports checked that the alternative final pass ran. Counts include warm-ups.

| Shape | Input | Quad / paired entries |
|---|---|---:|
| 12x2 | catalog | 4 / 4 |
| 12x2 | heldout | 4 / 4 |
| 16x2 | catalog | 4 / 4 |
| 16x2 | heldout | 4 / 4 |
| merge-24x1 | catalog | 4 / 4 |
| merge-24x1 | heldout | 4 / 4 |

[Measurements](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/source.patch) · [Proof verification](artifacts/verification.json)
