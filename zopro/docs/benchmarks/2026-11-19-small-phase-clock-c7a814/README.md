# Stage clocks for small CPU proofs on C7a

The clocks isolate witness parsing, solver execution, canonical gather and public correction, quotient FFT, pool-transition time, curve queries, and result serialization. The other column is SDK proving time outside the measured core sequence and includes proof assembly and verification. Each stage is timed in the same request. The all-curve replay mode reuses the original G1 and G2 results only after comparing full canonical coefficients. [Prior fixed-request diagnostic](../2026-11-19-g1-query-bound-c7a803/README.md).

1×2 uses 80 main and 16 frontend physical cores. 2×2 uses 64 and 32. The temporary C7a instance and its storage were removed. The 2×2 arms use medium fused K/H. Five matched blocks compare release, the same-binary ordinary proof, the clocked proof, replay, and clocked replay. Each process primes three calls and measures thirteen. All 108 exports independently verified. The paired clock change compares ordinary clocking against ordinary execution and clocked replay against unclocked replay. Durations are medians of process means in milliseconds and can differ slightly from the total when added.

| Shape | Input | Mode | Full proof | Clock effect | Parse and assign | Solve | Gather and correction | Quotient FFT | Pool transition | Curves | Other proving | Serialization |
|---|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | catalog | clock | 7.858 | +1.35% | 0.118 | 1.460 | 0.207 | 0.777 | 0.038 | 4.202 | 1.026 | 0.016 |
| 1×2 | catalog | all-clock | 4.043 | +0.64% | 0.093 | 1.433 | 0.198 | 0.743 | 0.038 | 0.566 | 0.959 | 0.014 |
| 1×2 | heldout | clock | 7.774 | -0.39% | 0.127 | 1.482 | 0.206 | 0.776 | 0.037 | 4.117 | 1.025 | 0.017 |
| 1×2 | heldout | all-clock | 4.044 | +1.04% | 0.097 | 1.427 | 0.196 | 0.739 | 0.038 | 0.566 | 0.963 | 0.013 |
| 2×2 | catalog | clock | 12.064 | -0.08% | 0.177 | 2.111 | 0.291 | 1.386 | 0.044 | 7.010 | 0.982 | 0.015 |
| 2×2 | catalog | all-clock | 5.575 | +0.65% | 0.149 | 2.063 | 0.300 | 1.325 | 0.035 | 0.742 | 0.951 | 0.013 |
| 2×2 | heldout | clock | 12.183 | +0.58% | 0.176 | 2.099 | 0.305 | 1.392 | 0.043 | 7.187 | 0.981 | 0.015 |
| 2×2 | heldout | all-clock | 5.580 | -0.01% | 0.143 | 2.043 | 0.300 | 1.331 | 0.033 | 0.745 | 0.954 | 0.013 |

[Source](artifacts/source.patch) · [Samples](artifacts/rows.json.gz) · [Workflow](artifacts/workflow.json.gz) · [Verification](artifacts/verification.json) · [Current C7a scorecard](../2026-11-19-vector-decode-c7a812/README.md)
