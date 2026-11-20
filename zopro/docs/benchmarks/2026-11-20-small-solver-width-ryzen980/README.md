# Solver pool width for small canonical-wires proofs on Ryzen

The smaller solver pools increased full-proof latency on both shapes. The four-core pool was 1.07–1.17% slower on 1×2 and 2.96–3.53% slower on 2×2 in paired medians versus ordinary.

The small proof preparation path can run its solver on a narrower frontend pool. Gather and quotient FFT run on the eight-core frontend in every profile. The main proof pool has 24 logical workers. This moves the solver only, using the same pool configuration previously measured on [4×2 C7a](../2026-11-20-four-by-two-prep-placement-c7a946/README.md).

| Shape | Input | Mode | Proof ms | Paired vs ordinary | Solve ms | Gather ms | FFT ms | Peak MiB |
|---|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | Exact frozen | 6.857 | -0.57% (5/8) | — | — | — | 57.2 |
| 1×2 | catalog | Ordinary | 6.867 | — | 0.840 | 0.103 | 0.536 | 58.0 |
| 1×2 | catalog | Two-core solver | 7.226 | +5.05% (0/8) | 1.227 | 0.103 | 0.558 | 53.8 |
| 1×2 | catalog | Four-core solver | 6.928 | +1.07% (1/8) | 0.940 | 0.104 | 0.546 | 55.5 |
| 1×2 | heldout | Exact frozen | 6.719 | -0.91% (5/8) | — | — | — | 58.2 |
| 1×2 | heldout | Ordinary | 6.724 | — | 0.842 | 0.098 | 0.543 | 59.5 |
| 1×2 | heldout | Two-core solver | 7.127 | +5.95% (0/8) | 1.231 | 0.103 | 0.545 | 53.8 |
| 1×2 | heldout | Four-core solver | 6.823 | +1.17% (2/8) | 0.942 | 0.104 | 0.551 | 56.0 |
| 2×2 | catalog | Exact frozen | 9.430 | -0.36% (5/8) | — | — | — | 96.2 |
| 2×2 | catalog | Ordinary | 9.450 | — | 1.119 | 0.165 | 1.223 | 109.0 |
| 2×2 | catalog | Two-core solver | 10.514 | +11.44% (1/8) | 2.234 | 0.168 | 1.209 | 84.5 |
| 2×2 | catalog | Four-core solver | 9.732 | +2.96% (1/8) | 1.480 | 0.166 | 1.230 | 89.0 |
| 2×2 | heldout | Exact frozen | 9.411 | -0.36% (4/8) | — | — | — | 95.8 |
| 2×2 | heldout | Ordinary | 9.436 | — | 1.134 | 0.161 | 1.208 | 106.8 |
| 2×2 | heldout | Two-core solver | 10.487 | +10.45% (1/8) | 2.248 | 0.166 | 1.213 | 85.2 |
| 2×2 | heldout | Four-core solver | 9.754 | +3.53% (1/8) | 1.476 | 0.164 | 1.233 | 92.2 |

The exact frozen controls are the packed final-exponentiation SDK for 1×2 and the Euler FFT SDK for 2×2. The same-build ordinary calibrated within 0.92% of those controls. The largest paired clock shift was 1.36%. Each input and mode uses eight blocks of seventeen timed proofs, with four separate clocked blocks. The route clocks confirm the configured solver width and the full frontend width for gather and FFT. Four native tests and independent verification of all 176 proof exports passed.

[Results](artifacts/summary.json) · [Proof samples](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.json) · [Source patch](artifacts/source.patch.gz)
