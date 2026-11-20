# SDK integration preflight for scoped exclusive G2

The integrated early profile stayed within 1.15% of the archived early profile in paired medians across both requests of four circuits. Probes confirmed quotient overlap on all sixteen measured early calls. In the late and zero-budget controls, all measured G2 calls began after the quotient.

The candidate introduces a per-key Rust API for assigning a disjoint worker pool to an exclusive G2 query. The existing scratch policy governs each dispatch. The early path starts G2 once the canonical inputs are ready and joins it after G1. The threshold and point arithmetic follow the [archived C7a implementation](../2026-11-20-scoped-g2-shape-transfer-c7a998/README.md). The Ryzen panel uses 12+4 physical cores for 1×2 and 2×2 and 8+8 for 1×8 and 1×16.

| Shape | Request | Archive late / early ms | SDK late / early ms | Paired SDK vs archive early | Observed overlap ms | Peak SDK early MiB |
|---|---|---:|---:|---:|---:|---:|
| 1×2 | catalog | 11.457 / 10.504 | 15.202 / 10.389 | -1.15% (4/4) | 0.546 | 49.5 |
| 1×2 | heldout | 11.376 / 10.531 | 14.852 / 10.436 | -0.69% (3/4) | 0.653 | 49.0 |
| 2×2 | catalog | 14.423 / 12.628 | 19.573 / 12.721 | +1.07% (1/4) | 0.972 | 94.0 |
| 2×2 | heldout | 14.341 / 12.676 | 19.469 / 12.637 | -0.17% (4/4) | 2.371 | 92.8 |
| 1×8 | catalog | 12.092 / 12.109 | 12.043 / 12.038 | -0.33% (3/4) | 0.994 | 100.5 |
| 1×8 | heldout | 12.150 / 12.162 | 13.589 / 12.185 | +0.77% (1/4) | 1.020 | 107.2 |
| 1×16 | catalog | 15.425 / 15.238 | 15.227 / 15.188 | -0.44% (3/4) | 1.207 | 119.8 |
| 1×16 | heldout | 15.351 / 15.332 | 15.238 / 15.246 | -0.58% (3/4) | 1.132 | 120.4 |

Four paired timing blocks prime three proofs and time thirteen. The separate trace panel uses byte-identical executables and captures two proofs per case. The integrated late controls were substantially slower than the archived late controls for the 12+4 layouts. Server measurements will compare each early arm directly to the archived candidate and Aeglos. All 152 process exports passed independent verification and the generic x86-64 prover and Zolana crates passed their build check.

[Summary](artifacts/summary.json) · [Samples and phases](artifacts/rows.json.gz) · [Candidate patch](artifacts/source.patch.gz) · [Validation](artifacts/validation.json)
