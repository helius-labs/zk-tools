# Borrowed witness inputs in the CPU SDK on Ryzen

The trial passes the existing witness field slice from the SDK to the CPU solver. It avoids allocating a byte buffer, encoding the witness, and decoding it into a second field buffer. The original byte API decodes through the same solver path. The GPU path still receives encoded bytes. The focused native test checks borrowing and exact decoding, including malformed byte length.

The four shapes use 24 main and eight frontend logical CPUs. Each input has five matched process blocks. Each process primes three proofs and measures thirteen. All 136 exports independently verified, including separate route checks at ordinary settings and with zero optional workspace.

Typed input did not beat both controls consistently across the two inputs of any tested shape. The code remains experimental.



| Shape | Input | Profile | Proof ms | Paired vs same-build control | Paired vs release | Peak MiB |
|---|---|---|---:|---:|---:|---:|
| 1×2 | catalog | release | 6.944 | +0.36% (2/5) | +0.00% (0/5) | 57.2 |
| 1×2 | catalog | ordinary | 6.939 | +0.00% (0/5) | -0.35% (3/5) | 58.2 |
| 1×2 | catalog | typed | 6.893 | -0.12% (3/5) | -1.24% (3/5) | 57.8 |
| 1×2 | heldout | release | 6.842 | +0.68% (2/5) | +0.00% (0/5) | 57.5 |
| 1×2 | heldout | ordinary | 6.796 | +0.00% (0/5) | -0.68% (3/5) | 57.5 |
| 1×2 | heldout | typed | 6.827 | -0.42% (3/5) | +0.28% (2/5) | 57.2 |
| 2×2 | catalog | release | 9.464 | -0.42% (4/5) | +0.00% (0/5) | 97.5 |
| 2×2 | catalog | ordinary | 9.500 | +0.00% (0/5) | +0.42% (1/5) | 95.2 |
| 2×2 | catalog | typed | 9.492 | -0.15% (3/5) | +0.39% (1/5) | 93.0 |
| 2×2 | heldout | release | 9.569 | -0.07% (3/5) | +0.00% (0/5) | 98.2 |
| 2×2 | heldout | ordinary | 9.532 | +0.00% (0/5) | +0.07% (2/5) | 98.0 |
| 2×2 | heldout | typed | 9.559 | -0.23% (3/5) | -0.94% (3/5) | 94.0 |
| 4×2 | catalog | release | 14.453 | -0.30% (4/5) | +0.00% (0/5) | 210.0 |
| 4×2 | catalog | ordinary | 14.490 | +0.00% (0/5) | +0.31% (1/5) | 217.2 |
| 4×2 | catalog | typed | 14.434 | -0.99% (3/5) | -0.31% (4/5) | 202.2 |
| 4×2 | heldout | release | 14.380 | +0.39% (2/5) | +0.00% (0/5) | 199.8 |
| 4×2 | heldout | ordinary | 14.401 | +0.00% (0/5) | -0.39% (3/5) | 199.5 |
| 4×2 | heldout | typed | 14.472 | +0.83% (0/5) | +0.92% (1/5) | 197.2 |
| 8×2 | catalog | release | 22.391 | +0.06% (2/5) | +0.00% (0/5) | 368.5 |
| 8×2 | catalog | ordinary | 22.426 | +0.00% (0/5) | -0.06% (3/5) | 365.0 |
| 8×2 | catalog | typed | 22.430 | +0.03% (1/5) | +0.59% (2/5) | 354.2 |
| 8×2 | heldout | release | 22.816 | -0.05% (3/5) | +0.00% (0/5) | 381.0 |
| 8×2 | heldout | ordinary | 22.711 | +0.00% (0/5) | +0.05% (2/5) | 371.5 |
| 8×2 | heldout | typed | 22.523 | -0.76% (4/5) | -1.73% (4/5) | 372.0 |

## Witness buffers

The two temporarily allocated input buffers in the byte route each held 32 bytes per field. The total below covers those two buffers and excludes the existing solver working storage.

| Shape | Witness fields | Eliminated buffer KiB |
|---|---:|---:|
| 1×2 | 144 | 9.0 |
| 2×2 | 236 | 14.8 |
| 4×2 | 420 | 26.2 |
| 8×2 | 788 | 49.2 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json)
