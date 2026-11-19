# G2 decomposition and correction windows on the experimental mapped 12×2 prover

The current experimental 12×2 prover evaluates the main B2 sum and up to two sparse corrections. On the catalog request the two levels emitted 1,873 and 1,899 correction terms, while the top-level query contained 43,189 terms. The main mode decomposes owned scalar classes concurrently with summing their points when the existing grouping succeeds. The eight- and ten-bit modes run a one-part endomorphism kernel on lists of 256 through 4,095 correction terms. Both mode combines main and eight. Frozen is the previous mapped binary, and off disables the two changes on the new binary. No additional key tables are allocated.

The original request ran at two placements of 30 hardware threads spanning all 16 physical Ryzen cores. Two other requests assess sensitivity to the witness. Each process warmed three requests and timed eleven. The table reports medians of process means and paired changes. CPU profiles used separate processes. M5 ratios are shown only for the supplied original request. Every exported proof verified against the original key and public inputs.

| Input | Mode | Warm ms | Paired vs off | Paired vs frozen | Peak MiB | M5 CPU / Metal (10× CPU blocks) |
|---|---|---:|---:|---:|---:|---:|
| original0 | frozen | 28.580 | -0.28% (6/8) | +0.00% (0/8) | 554.8 | 9.94× / 3.53× (1/8) |
| original0 | off | 28.538 | +0.00% (0/8) | +0.28% (2/8) | 536.0 | 9.95× / 3.54× (3/8) |
| original0 | both | 28.607 | +0.37% (3/8) | +0.10% (4/8) | 548.8 | 9.93× / 3.53× (0/8) |
| original0 | main | 28.510 | -0.18% (5/8) | -0.10% (5/8) | 541.7 | 9.96× / 3.54× (4/8) |
| original0 | ten | 28.994 | +1.06% (1/8) | +1.27% (1/8) | 539.0 | 9.80× / 3.48× (0/8) |
| original0 | eight | 28.923 | +1.14% (3/8) | +0.58% (1/8) | 549.4 | 9.82× / 3.49× (0/8) |
| original4 | frozen | 28.503 | -0.64% (4/6) | +0.00% (0/6) | 542.2 | 9.96× / 3.54× (3/6) |
| original4 | off | 28.662 | +0.00% (0/6) | +0.64% (2/6) | 538.5 | 9.91× / 3.52× (0/6) |
| original4 | both | 28.634 | -0.09% (3/6) | +0.50% (3/6) | 539.9 | 9.92× / 3.53× (1/6) |
| original4 | main | 28.502 | -0.37% (5/6) | -0.54% (3/6) | 546.6 | 9.96× / 3.54× (2/6) |
| original4 | ten | 28.751 | +0.52% (1/6) | +1.49% (2/6) | 545.0 | 9.88× / 3.51× (0/6) |
| original4 | eight | 28.471 | -0.95% (5/6) | -0.14% (3/6) | 548.2 | 9.98× / 3.55× (3/6) |
| held58 | frozen | 28.852 | +1.77% (0/4) | +0.00% (0/4) | 542.8 | — |
| held58 | off | 28.377 | +0.00% (0/4) | -1.73% (4/4) | 532.8 | — |
| held58 | both | 28.653 | +0.87% (0/4) | -0.68% (3/4) | 518.1 | — |
| held58 | main | 29.136 | +2.81% (1/4) | +1.61% (1/4) | 569.0 | — |
| held58 | ten | 28.802 | +1.49% (0/4) | -0.27% (2/4) | 554.8 | — |
| held58 | eight | 28.710 | +0.87% (0/4) | -0.40% (2/4) | 534.5 | — |
| scattered139 | frozen | 45.119 | -0.83% (2/4) | +0.00% (0/4) | 1004.0 | — |
| scattered139 | off | 45.428 | +0.00% (0/4) | +0.84% (2/4) | 1012.9 | — |
| scattered139 | both | 45.117 | -0.24% (2/4) | -0.19% (2/4) | 992.0 | — |
| scattered139 | main | 44.810 | -1.28% (3/4) | -0.11% (2/4) | 960.2 | — |
| scattered139 | ten | 45.002 | -0.71% (3/4) | +0.34% (1/4) | 982.9 | — |
| scattered139 | eight | 45.250 | +0.06% (2/4) | +0.13% (2/4) | 966.3 | — |

The two original placements gave different answers for the eight-bit correction kernel. It won 3/8 and 5/6 pairs against disabled. The ten-bit kernel lost 12/14 pairs against disabled. Neither placement's candidate median met 28.4 ms. Diagnostics saw both small correction calls on the original, while the optional concurrent main-group decomposition recorded no entry. Its timing differences on that witness cannot be attributed to the proposed decomposition. All 159 native core tests passed, and the study independently verified 161 fresh exports.

[Patch relative to mapped representatives](artifacts/source.patch) · [Dispatch counts](artifacts/diagnostics.json) · [Actual configurations](artifacts/configs.json.gz) · [Raw profiles and timings](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
