# Vector decoding of grouped B2 scalars on Ryzen

The candidate uses eight-lane IFMA to convert distinct B2 scalars from Montgomery form in the existing serial preparation pass. It preserves the exact scalar classes and point reduction. The last partial vector uses the scalar backend. The differential native test checks representative field values, every vector tail and long slices against ordinary canonical conversion.

Zopro uses 24 main and eight frontend logical CPUs with the 8×2 four-epoch solver profile. The original and first held-out inputs have five matched blocks. The two additional inputs have four. Each process primes three proofs and measures thirteen. All 70 exports independently verified, including probes for the vector route and zero optional workspace.

Vector decoding did not beat both controls consistently across the four inputs. The code remains experimental.


| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 8×2 | catalog | release | 22.510 | +0.45% (2/5) | +0.00% (0/5) | 374.2 |
| 8×2 | catalog | ordinary | 22.558 | +0.00% (0/5) | -0.45% (3/5) | 377.5 |
| 8×2 | catalog | vector | 22.556 | +0.20% (2/5) | -0.25% (3/5) | 367.5 |
| 8×2 | heldout | release | 22.681 | +0.24% (1/5) | +0.00% (0/5) | 384.2 |
| 8×2 | heldout | ordinary | 22.620 | +0.00% (0/5) | -0.24% (4/5) | 383.2 |
| 8×2 | heldout | vector | 22.747 | +0.95% (1/5) | -0.09% (3/5) | 373.8 |
| 8×2 | v153 | release | 22.701 | +0.72% (1/4) | +0.00% (0/4) | 372.8 |
| 8×2 | v153 | ordinary | 22.567 | +0.00% (0/4) | -0.71% (3/4) | 369.5 |
| 8×2 | v153 | vector | 22.524 | +0.05% (2/4) | -0.81% (4/4) | 376.2 |
| 8×2 | v156 | release | 22.641 | +0.64% (1/4) | +0.00% (0/4) | 368.8 |
| 8×2 | v156 | ordinary | 22.499 | +0.00% (0/4) | -0.63% (3/4) | 372.2 |
| 8×2 | v156 | vector | 22.843 | +0.75% (0/4) | +0.55% (1/4) | 385.0 |

Three separately clocked blocks show the time spent decoding the proof-local classes.

| Input | Mode | Proof ms | Paired clock effect | Classes | Decode µs |
|---|---|---:|---:|---:|---:|
| catalog | ordinary-clock | 22.246 | -0.81% | 38,075 | 615.5 |
| catalog | vector-clock | 22.522 | +0.72% | 38,075 | 112.6 |
| heldout | ordinary-clock | 22.450 | -0.37% | 39,891 | 633.0 |
| heldout | vector-clock | 22.804 | +0.25% | 39,891 | 127.6 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
