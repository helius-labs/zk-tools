# Stage clocks for small CPU proofs on Ryzen

The clocks isolate witness parsing, solver execution, canonical gather and public correction, quotient FFT, pool-transition time, curve queries, and result serialization. The other column is SDK proving time outside the measured core sequence and includes proof assembly and verification. Each stage is timed in the same request. The all-curve replay mode reuses the original G1 and G2 results only after comparing full canonical coefficients. [Prior fixed-request diagnostic](../2026-11-19-g1-query-bound-c7a803/README.md).

Both shapes use 24 main and eight frontend logical CPUs. The 2×2 arms use medium fused K/H. Five matched blocks compare release, the same-binary ordinary proof, the clocked proof, replay, and clocked replay. Each process primes three calls and measures thirteen. All 108 exports independently verified. The paired clock change compares ordinary clocking against ordinary execution and clocked replay against unclocked replay. Durations are medians of process means in milliseconds and can differ slightly from the total when added.

| Shape | Input | Mode | Full proof | Clock effect | Parse and assign | Solve | Gather and correction | Quotient FFT | Pool transition | Curves | Other proving | Serialization |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | clock | 6.967 | +0.07% | 0.084 | 0.839 | 0.105 | 0.531 | 0.010 | 4.771 | 0.579 | 0.010 |
| 1×2 | catalog | all-clock | 2.118 | +2.57% | 0.053 | 0.769 | 0.112 | 0.476 | 0.009 | 0.144 | 0.541 | 0.008 |
| 1×2 | heldout | clock | 6.874 | +0.53% | 0.098 | 0.845 | 0.100 | 0.534 | 0.010 | 4.712 | 0.602 | 0.012 |
| 1×2 | heldout | all-clock | 2.086 | +0.21% | 0.053 | 0.760 | 0.126 | 0.463 | 0.009 | 0.144 | 0.514 | 0.007 |
| 2×2 | catalog | clock | 9.485 | +0.70% | 0.135 | 1.141 | 0.159 | 1.258 | 0.010 | 6.137 | 0.617 | 0.012 |
| 2×2 | catalog | all-clock | 3.044 | -0.56% | 0.087 | 0.956 | 0.194 | 0.978 | 0.009 | 0.271 | 0.545 | 0.007 |
| 2×2 | heldout | clock | 9.479 | +0.39% | 0.140 | 1.145 | 0.159 | 1.254 | 0.010 | 6.115 | 0.632 | 0.012 |
| 2×2 | heldout | all-clock | 3.048 | +0.43% | 0.081 | 0.960 | 0.203 | 0.988 | 0.009 | 0.271 | 0.528 | 0.007 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Current C7a scorecard](../2026-11-19-vector-decode-c7a812/README.md)
