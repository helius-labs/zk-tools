# Linear identities from pairs of nonlinear constraints

For each input image the reader expands the exact solver linear forms A, B and C, including stored slots and the assigned wire on its recorded side. For nonlinear rows it removes the constants from A and B and normalizes each nonzero linear factor by its first coefficient. Unique factorization makes an unordered pair of normalized factors an exact test for proportional homogeneous products. A SHA-256 digest indexes potential matches. The reader compares both complete normalized factors before subtracting the scaled rows.

This can expose a linear relation even if both constraints have two nonlinear operands, as happens when their products differ only by scaling, factor order and constant shifts. A positive control checks the predicted linear equation for four field assignments and rejects four perturbed outputs. The B2 column excludes wires emitted by the Bits hint.

| Shape | Nonlinear rows | Proportional rows beyond first member | Residual equations up to scale | Upper bound on additional non-hint B2 pivots |
|---|---:|---:|---:|---:|
| 1x2 | 30,839 | 86 | 86 | 86 |
| 2x2 | 53,753 | 162 | 162 | 162 |
| 4x2 | 98,477 | 314 | 314 | 314 |
| 8x2 | 189,269 | 618 | 618 | 618 |
| 12x2 | 279,741 | 922 | 922 | 922 |
| 16x2 | 370,469 | 1,226 | 1,226 | 1,226 |
| 24x2 | 553,269 | 1,834 | 1,834 | 1,834 |
| merge-24x1 | 525,246 | 1,860 | 1,860 | 1,860 |

Each image digest was verified before the scan. These are algebraic counts from unmodified public keys. No latency or memory improvement is claimed. [Counts and image identities](artifacts/counts.json) · [Reader](artifacts/scan.py)
