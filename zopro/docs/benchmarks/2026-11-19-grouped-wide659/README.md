# Decompose owned G2 coefficients while reducing their point groups

The ordinary 5×16 key has 145,951 B2 points, but the catalog witness groups to 51,707 nonzero coefficients. The four-query prover can give B2 ownership of these coefficients because B1 is omitted. The candidate converts them in place with the BN254 endomorphism while other workers sum the grouped points. Group index vectors drop before window evaluation and the reservation covers the complete operation. No key table is constructed. The earlier post-group implementation cloned the coefficients and used a different window kernel. This trial uses the current borrowed-operand kernel and the existing subgroup-valid key requirement.

Release is the ordinary reference executable. Packed is the same executable with the previously selected FFT configuration on 5×16. Off and owned share a new executable, the same FFT configuration and differ only in G2 selection. The smaller 1×16 key keeps its existing route. Each 5×16 input uses eight balanced blocks per arm, 1×16 uses six, and each arm has a separate CPU sampling process. The table reports process-mean medians, median paired latency changes and paired wins. Every process exports an independent proof. All 165 exports verified against original keys.

| Input | Setting | Warm ms | Paired vs release | Paired vs off | Paired vs prior best | CPU-ms/proof | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 5x16 | release | 24.408 | +0.00% (0/8) | +3.72% (0/8) | +4.32% (1/8) | 656.3 | 411.2 |
| 5x16 | packed | 23.425 | -4.14% (7/8) | -1.51% (5/8) | +0.00% (0/8) | 631.1 | 424.5 |
| 5x16 | owned | 22.806 | -6.31% (7/8) | -1.34% (7/8) | -2.62% (6/8) | 611.0 | 431.5 |
| 5x16 | off | 23.530 | -3.58% (8/8) | +0.00% (0/8) | +1.53% (3/8) | 632.7 | 422.0 |
| 5x16-1701 | release | 24.112 | +0.00% (0/8) | +2.81% (1/8) | +1.53% (2/8) | 655.5 | 417.0 |
| 5x16-1701 | packed | 23.975 | -1.51% (6/8) | +1.21% (2/8) | +0.00% (0/8) | 651.6 | 422.2 |
| 5x16-1701 | owned | 23.080 | -3.14% (8/8) | -0.30% (5/8) | -1.34% (8/8) | 623.7 | 433.8 |
| 5x16-1701 | off | 23.464 | -2.73% (7/8) | +0.00% (0/8) | -1.20% (6/8) | 642.1 | 419.0 |
| 5x16-1702 | release | 24.166 | +0.00% (0/8) | +2.29% (1/8) | +1.18% (1/8) | 660.5 | 415.2 |
| 5x16-1702 | packed | 23.785 | -1.17% (7/8) | +0.68% (2/8) | +0.00% (0/8) | 646.8 | 416.2 |
| 5x16-1702 | owned | 23.091 | -4.22% (8/8) | -2.26% (7/8) | -3.11% (7/8) | 621.9 | 431.8 |
| 5x16-1702 | off | 23.659 | -2.24% (7/8) | +0.00% (0/8) | -0.68% (6/8) | 639.8 | 422.8 |
| 5x16-1703 | release | 23.908 | +0.00% (0/8) | +1.65% (3/8) | +0.85% (3/8) | 653.0 | 409.2 |
| 5x16-1703 | packed | 23.707 | -0.84% (5/8) | +0.81% (3/8) | +0.00% (0/8) | 645.5 | 416.0 |
| 5x16-1703 | owned | 23.124 | -3.53% (7/8) | -1.92% (6/8) | -1.96% (7/8) | 617.8 | 447.5 |
| 5x16-1703 | off | 23.609 | -1.62% (5/8) | +0.00% (0/8) | -0.80% (5/8) | 639.4 | 438.0 |
| 1x16 | release | 12.957 | +0.00% (0/6) | +0.12% (3/6) | +0.00% (0/6) | 309.5 | 239.8 |
| 1x16 | off | 12.952 | -0.12% (3/6) | +0.00% (0/6) | -0.12% (3/6) | 308.5 | 243.8 |
| 1x16 | owned | 12.924 | -0.28% (4/6) | -0.24% (3/6) | -0.28% (4/6) | 309.3 | 242.2 |

The combined FFT and G2 configuration won 30/32 pairs against the ordinary release and 28/32 against the earlier packed FFT. The G2 change alone won 25/32 against its same-binary disabled control. On all four witnesses it reduced aggregate CPU time and increased observed peak RSS versus the earlier packed FFT. The original used 611.0 CPU-ms and 431.5 MiB versus 631.1 CPU-ms and 424.5 MiB with the packed FFT. The implementation passed 156 native core tests. The new option warrants a separately built release check before adoption. [Kernel markers in the independent profiles](artifacts/symbols.json)

[Implementation](artifacts/source.patch) · [Configuration](artifacts/configs.json.gz) · [Process timings and CPU profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json) · [Earlier post-group test](../2026-10-31-postgroup-phi176/README.md)
