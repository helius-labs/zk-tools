# Sharded B2 scalar grouping on Ryzen

The trial partitions nonzero B2 scalar indices by their Montgomery words. Each shard builds exact classes in parallel while comparing all four limbs. Scalar one receives a direct class. The shard metadata is assembled into the original grouped MSM operand format. The trial uses only proof-local buffers, charges additional scratch before allocation and falls back to the existing grouping when admission declines.

Ryzen uses 24 main and eight frontend logical CPUs. 4×2 uses the frontend solver. The 8×2 controls use recognized Poseidon origins and serial conversion, and the sharded variants preserve the conversion setting. The two primary inputs use five matched blocks. Two additional 8×2 inputs use four. Each process primes three requests and measures thirteen. The exact classification oracle covers zero, one, mixed repeated scalars, and full-word equality for each of the three shard widths. All 148 exported proofs independently verified. Route probes exercised the zero optional-workspace fallback.

None of the sharded modes beat both controls consistently across the tested inputs. The code remains experimental.

| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 4×2 | catalog | release | 14.502 | -0.88% (4/5) | +0.00% (0/5) | 206.0 |
| 4×2 | catalog | ordinary | 14.614 | +0.00% (0/5) | +0.89% (1/5) | 200.5 |
| 4×2 | catalog | shards-4 | 14.811 | +1.57% (1/5) | +2.00% (0/5) | 211.0 |
| 4×2 | catalog | shards-8 | 14.936 | +2.20% (0/5) | +2.87% (0/5) | 206.8 |
| 4×2 | catalog | shards-16 | 14.896 | +1.97% (0/5) | +3.09% (0/5) | 215.8 |
| 4×2 | heldout | release | 14.556 | -0.16% (3/5) | +0.00% (0/5) | 200.5 |
| 4×2 | heldout | ordinary | 14.502 | +0.00% (0/5) | +0.16% (2/5) | 214.8 |
| 4×2 | heldout | shards-4 | 14.815 | +2.15% (1/5) | +1.38% (0/5) | 214.0 |
| 4×2 | heldout | shards-8 | 14.912 | +1.99% (0/5) | +2.15% (0/5) | 218.5 |
| 4×2 | heldout | shards-16 | 14.911 | +1.90% (1/5) | +2.06% (1/5) | 218.0 |
| 8×2 | catalog | release | 22.539 | -1.00% (4/5) | +0.00% (0/5) | 408.0 |
| 8×2 | catalog | ordinary | 22.744 | +0.00% (0/5) | +1.01% (1/5) | 403.5 |
| 8×2 | catalog | shards-4 | 22.741 | -0.90% (4/5) | +1.00% (2/5) | 391.5 |
| 8×2 | catalog | shards-8 | 23.265 | +2.26% (0/5) | +2.83% (0/5) | 396.0 |
| 8×2 | catalog | shards-16 | 23.547 | +4.07% (0/5) | +4.47% (0/5) | 390.0 |
| 8×2 | heldout | release | 22.697 | +0.08% (2/5) | +0.00% (0/5) | 412.8 |
| 8×2 | heldout | ordinary | 22.666 | +0.00% (0/5) | -0.08% (3/5) | 410.0 |
| 8×2 | heldout | shards-4 | 22.787 | +0.94% (0/5) | +0.86% (1/5) | 402.2 |
| 8×2 | heldout | shards-8 | 23.404 | +2.88% (0/5) | +3.10% (0/5) | 410.5 |
| 8×2 | heldout | shards-16 | 23.794 | +4.90% (0/5) | +4.77% (0/5) | 388.0 |
| 8×2 | v153 | release | 22.706 | -0.75% (3/4) | +0.00% (0/4) | 404.2 |
| 8×2 | v153 | ordinary | 22.876 | +0.00% (0/4) | +0.75% (1/4) | 399.2 |
| 8×2 | v153 | shards-4 | 22.637 | -0.80% (2/4) | -0.48% (3/4) | 408.0 |
| 8×2 | v153 | shards-8 | 23.426 | +2.09% (1/4) | +2.99% (0/4) | 388.2 |
| 8×2 | v153 | shards-16 | 23.681 | +3.89% (1/4) | +4.20% (0/4) | 396.8 |
| 8×2 | v156 | release | 22.975 | -0.28% (4/4) | +0.00% (0/4) | 407.5 |
| 8×2 | v156 | ordinary | 23.079 | +0.00% (0/4) | +0.28% (0/4) | 394.2 |
| 8×2 | v156 | shards-4 | 23.133 | +0.15% (1/4) | +0.90% (1/4) | 402.5 |
| 8×2 | v156 | shards-8 | 23.495 | +1.71% (0/4) | +1.97% (0/4) | 413.8 |
| 8×2 | v156 | shards-16 | 23.703 | +2.80% (0/4) | +3.08% (0/4) | 387.8 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
