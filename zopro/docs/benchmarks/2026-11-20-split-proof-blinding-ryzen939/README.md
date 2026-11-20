# Endomorphism splitting in CPU proof blinding

The G1 and G2 split trial took **6.847 / 6.743 / 6.765 ms** for the original, held-out and v27 1×2 requests and **9.456 / 9.453 ms** for the two 2×2 requests. Against same-build ordinary, its paired changes were **-0.74% (6/8) / +0.06% (4/8) / -0.07% (4/8)** and **-0.44% (6/8) / -0.43% (5/8)**, respectively. Both modes fell short of the paired criteria on each shape, so the SDK retains ordinary assembly. The C7a confirmations await AWS SSO. The verified [C7a score](../2026-11-20-typed-small-proof-c7a898/README.md#eight-supplied-m5-reference-timings) remains five of eight shapes at 10× M5 CPU.

The [C7a phase clocks](../2026-11-19-small-phase-clock-c7a814/README.md) measured roughly 1 ms outside the solver, quotient, MSM queries and serialization. That includes assembly and verification. This trial changes assembly for certified SDK output. The G2 variable-base multiply uses the existing signed scalar decomposition and two odd-multiple tables. The existing fixed-delta table evaluates the separate delta term. The joint mode applies the corresponding G1 endomorphism to both variable-base terms. The G1 x-coordinate multiplier is the square of the G2 multiplier for the existing scalar eigenvalue. Both routes keep the same fresh (u,v) and Groth16 output equations.

The trial reuses the existing G2 subgroup certificate, checks the G1 base is on the curve and keeps ordinary G2 arithmetic when its fixed table is absent. The additional odd multiples live in proof-local buffers. The five Groth16 unit tests passed, including direct comparisons with unsplit scalar multiplication for both curves. The route probes confirmed both switches, explicit uncertified mode and absent fixed-delta tables. Each of the 185 proof exports verified with Gnark.

Ryzen uses 24 main and eight frontend hardware threads. Eight counterbalanced blocks per request prime three proofs and time seventeen. The frozen 851 executable and same-build ordinary mode use the original settings. A candidate needs ordinary to calibrate within 2%, plus lower paired medians and six wins on each fixture against both controls. Choosing the joint mode over G2 alone requires the same paired comparison.

| Shape and input | Mode | Proof ms | Paired vs frozen | Paired vs ordinary | Paired vs G2 | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2/catalog | release | 6.889 | +0.00% (0/8) | -0.19% (5/8) | +0.20% (3/8) | 133.7 | 60.0 |
| 1x2/catalog | ordinary | 6.918 | +0.19% (3/8) | +0.00% (0/8) | -0.17% (5/8) | 133.5 | 59.0 |
| 1x2/catalog | g2 | 6.894 | -0.20% (5/8) | +0.17% (3/8) | +0.00% (0/8) | 133.9 | 58.5 |
| 1x2/catalog | both | 6.847 | -0.60% (6/8) | -0.74% (6/8) | -0.50% (6/8) | 133.4 | 57.5 |
| 1x2/heldout | release | 6.784 | +0.00% (0/8) | +0.64% (1/8) | +0.77% (2/8) | 129.2 | 58.0 |
| 1x2/heldout | ordinary | 6.741 | -0.64% (7/8) | +0.00% (0/8) | +0.17% (4/8) | 128.7 | 58.5 |
| 1x2/heldout | g2 | 6.731 | -0.76% (6/8) | -0.17% (4/8) | +0.00% (0/8) | 128.7 | 57.2 |
| 1x2/heldout | both | 6.743 | -0.38% (5/8) | +0.06% (4/8) | +0.29% (3/8) | 128.9 | 58.0 |
| 1x2/v27 | release | 6.796 | +0.00% (0/8) | -0.33% (5/8) | +0.33% (2/8) | 128.8 | 58.5 |
| 1x2/v27 | ordinary | 6.780 | +0.33% (3/8) | +0.00% (0/8) | +0.72% (3/8) | 129.1 | 57.8 |
| 1x2/v27 | g2 | 6.746 | -0.33% (6/8) | -0.71% (5/8) | +0.00% (0/8) | 129.1 | 57.8 |
| 1x2/v27 | both | 6.765 | +0.13% (3/8) | -0.07% (4/8) | +0.27% (1/8) | 128.5 | 57.8 |
| 2x2/catalog | release | 9.490 | +0.00% (0/8) | +0.02% (4/8) | +0.19% (2/8) | 181.0 | 99.5 |
| 2x2/catalog | ordinary | 9.466 | -0.02% (4/8) | +0.00% (0/8) | +0.30% (3/8) | 181.3 | 100.0 |
| 2x2/catalog | g2 | 9.456 | -0.19% (6/8) | -0.30% (5/8) | +0.00% (0/8) | 180.8 | 94.0 |
| 2x2/catalog | both | 9.456 | -0.56% (6/8) | -0.44% (6/8) | -0.13% (5/8) | 181.5 | 98.5 |
| 2x2/heldout | release | 9.522 | +0.00% (0/8) | +0.09% (3/8) | +0.51% (1/8) | 181.9 | 98.5 |
| 2x2/heldout | ordinary | 9.503 | -0.09% (5/8) | +0.00% (0/8) | +0.30% (2/8) | 181.3 | 94.5 |
| 2x2/heldout | g2 | 9.480 | -0.51% (7/8) | -0.30% (6/8) | +0.00% (0/8) | 181.7 | 94.5 |
| 2x2/heldout | both | 9.453 | -0.64% (6/8) | -0.43% (5/8) | -0.04% (5/8) | 181.5 | 105.8 |

[Reproducible source patch](artifacts/source.patch) · [Protocol](artifacts/protocol.json) · [Build and timing data](artifacts/built.json) · [Proof verification](artifacts/verification.json)
