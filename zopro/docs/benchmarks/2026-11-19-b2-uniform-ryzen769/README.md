# Uniform scalar-class point reduction on Ryzen

The 4×2 training captures each contain about 17,280 scalar classes with exactly four points. The three 8×2 captures each contain about 17,040 classes with exactly eight points. The trial uses the exact class sizes already computed during proving. For a group of up to 256 consecutive classes with the same eligible size, it applies the same affine pair kernel at doubling strides. This avoids compacting intermediate sums or storing a pair-index list. Tiles of mixed sizes take the ordinary reduction. Identity points or equal X coordinates trigger a reload of the tile and complete treatment by the ordinary path. The trial adds no persistent point table.

Five matched process blocks on two inputs for each shape use 24 main and eight frontend logical threads on Ryzen. They discard three warmups and measure 13 requests. The released SDK, trial with the experiment disabled, and same-binary ordinary B2 layout provide controls. The native test compares uniform and ordinary results for both sizes, mixed sizes, identity, opposite points, and equal X coordinates reached after a successful layer. All 80 exported proofs independently verified.

The uniform path did not lead all three controls on the two requests for either circuit. The C7a scorecard remains the previous measurement.

| Shape | Input | Profile | Proof ms | Paired vs ordinary layout | Paired vs disabled trial | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.553 | +0.08% (2/5) | +0.42% (1/5) | +0.00% (0/5) | 208.0 |
| 4×2 | catalog | off | 14.519 | -0.18% (5/5) | +0.00% (0/5) | -0.42% (4/5) | 193.8 |
| 4×2 | catalog | b2-base | 14.541 | +0.00% (0/5) | +0.18% (0/5) | -0.08% (3/5) | 206.8 |
| 4×2 | catalog | uniform | 14.500 | -0.25% (3/5) | -0.07% (3/5) | -0.12% (3/5) | 204.0 |
| 4×2 | heldout | release | 14.495 | -1.08% (4/5) | -0.98% (5/5) | +0.00% (0/5) | 196.8 |
| 4×2 | heldout | off | 14.581 | -0.03% (3/5) | +0.00% (0/5) | +0.99% (0/5) | 199.8 |
| 4×2 | heldout | b2-base | 14.491 | +0.00% (0/5) | +0.03% (2/5) | +1.09% (1/5) | 204.8 |
| 4×2 | heldout | uniform | 14.588 | +0.45% (2/5) | +0.33% (2/5) | +0.36% (2/5) | 203.5 |
| 8×2 | catalog | release | 22.402 | +0.30% (2/5) | +0.42% (1/5) | +0.00% (0/5) | 413.2 |
| 8×2 | catalog | off | 22.260 | -0.37% (4/5) | +0.00% (0/5) | -0.41% (4/5) | 398.5 |
| 8×2 | catalog | b2-base | 22.428 | +0.00% (0/5) | +0.37% (1/5) | -0.30% (3/5) | 405.8 |
| 8×2 | catalog | uniform | 22.534 | +0.79% (2/5) | +1.64% (0/5) | +0.97% (1/5) | 408.2 |
| 8×2 | heldout | release | 22.623 | +0.09% (2/5) | -1.23% (4/5) | +0.00% (0/5) | 404.2 |
| 8×2 | heldout | off | 22.847 | +0.78% (0/5) | +0.00% (0/5) | +1.25% (1/5) | 405.5 |
| 8×2 | heldout | b2-base | 22.558 | +0.00% (0/5) | -0.77% (5/5) | -0.09% (3/5) | 412.5 |
| 8×2 | heldout | uniform | 22.758 | +0.27% (1/5) | -0.64% (4/5) | +0.25% (2/5) | 410.2 |

[Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Source](artifacts/uniform.patch) · [Training census](artifacts/training-census.json) · [Verification](artifacts/verification.json) · [C7a reference timings](../2026-11-19-grouped-b2-c7a768/README.md)
