# M5 Pro and AWS C7a proof benchmarks

All 41 circuit shapes from the AWS catalog have warm proof latency and peak process RSS results for M5 Pro Zopro CPU, M5 Pro Zopro Metal, and AWS Zopro, Aeglos, and Gnark.

- M5 Metal took **12.0–368.9 ms** per proof and was **1.88×** faster than M5 CPU at the median shape ratio. The CPU range was **18.6–748.1 ms**.
- Against the 96-core AWS run, M5 Metal took **1.64×** as long as Zopro at the median shape ratio and beat Aeglos on **5/41** shapes. AWS Gnark took **5.26×** as long as M5 Metal at the median shape ratio.
- M5 Metal peak process RSS was **93.6–1779.9 MiB**. Its macOS peak physical footprint was **170.5–1624.0 MiB**. The footprint series is in the memory table and CSV.

[Comparison CSV](comparison.csv) · [Raw M5 blocks](artifacts/m5-blocks.jsonl) · [AWS three-prover report](https://github.com/helius-labs/zk-tools/blob/d942372f85491f42596d157ed4c3424277d99e4c/zopro/docs/benchmarks/2026-11-20-c7a-three-provers/README.md#catalog-hot)

## Warm proof latency in milliseconds

| Shape | M5 CPU | M5 Metal | AWS Zopro | AWS Aeglos | AWS Gnark |
|---|---:|---:|---:|---:|---:|
| 1×2 | 18.55 | 12.00 | 7.84 | 8.29 | 49.77 |
| 1×4 | 28.49 | 15.13 | 8.67 | 10.19 | 60.72 |
| 1×8 | 32.20 | 17.32 | 10.57 | 10.88 | 72.99 |
| 1×16 | 35.99 | 21.85 | 11.89 | 12.74 | 86.12 |
| 2×2 | 26.02 | 15.60 | 10.33 | 12.16 | 83.06 |
| 2×4 | 27.16 | 18.36 | 11.37 | 12.38 | 88.09 |
| 2×8 | 46.84 | 29.16 | 13.28 | 15.36 | 109.00 |
| 2×16 | 50.32 | 31.13 | 17.10 | 17.32 | 121.20 |
| 3×2 | 45.03 | 27.01 | 14.28 | 16.30 | 120.51 |
| 3×4 | 45.69 | 28.87 | 14.80 | 17.20 | 124.15 |
| 3×8 | 47.45 | 29.80 | 15.57 | 17.82 | 134.04 |
| 4×2 | 46.79 | 28.35 | 14.13 | 19.07 | 144.40 |
| 4×4 | 47.20 | 30.75 | 15.94 | 18.94 | 148.87 |
| 4×8 | 50.26 | 29.89 | 16.80 | 19.90 | 155.08 |
| 4×16 | 55.19 | 33.06 | 18.78 | 21.83 | 169.60 |
| 5×2 | 48.76 | 29.96 | 20.96 | 20.87 | 167.53 |
| 5×4 | 50.57 | 32.19 | 20.35 | 21.05 | 169.30 |
| 5×8 | 71.33 | 38.65 | 25.64 | 27.30 | 200.82 |
| 5×16 | 76.14 | 40.38 | 26.53 | 27.92 | 213.86 |
| 6×2 | 68.76 | 33.14 | 25.09 | 28.06 | 216.30 |
| 6×4 | 70.03 | 37.83 | 25.53 | 27.90 | 217.02 |
| 6×8 | 74.05 | 38.89 | 26.41 | 29.02 | 221.18 |
| 8×2 | 74.33 | 37.22 | 18.68 | 31.11 | 253.16 |
| 8×4 | 78.06 | 36.66 | 27.51 | 32.44 | 254.26 |
| 8×8 | 74.04 | 36.87 | 28.42 | 33.12 | 262.46 |
| 8×16 | 78.85 | 43.13 | 30.60 | 34.92 | 274.77 |
| 12×2 | 117.24 | 50.41 | 29.51 | 49.58 | 382.67 |
| 12×4 | 115.48 | 51.69 | 36.58 | 49.63 | 393.29 |
| 12×8 | 119.25 | 51.37 | 37.76 | 52.66 | 393.43 |
| 16×2 | 120.95 | 53.68 | 33.37 | 58.05 | 459.56 |
| 16×4 | 120.83 | 53.39 | 42.64 | 58.19 | 462.48 |
| 16×8 | 127.15 | 56.43 | 43.25 | 59.41 | 463.55 |
| 24×2 | 272.81 | 136.13 | 47.47 | 87.44 | 660.08 |
| 24×4 | 267.53 | 96.65 | 58.51 | 88.20 | 665.56 |
| 32×2 | 344.61 | 179.51 | 69.05 | 103.47 | 770.84 |
| 40×2 | 371.33 | 106.93 | 80.38 | 118.62 | 870.95 |
| 48×2 | 662.30 | 345.77 | 104.94 | 167.50 | 1147.31 |
| 49×2 | 748.11 | 352.85 | 108.29 | 169.99 | 1100.71 |
| merge-8×1 | 71.27 | 33.41 | 29.19 | 30.62 | 243.74 |
| merge-24×1 | 250.86 | 92.50 | 46.19 | 86.15 | 683.54 |
| merge-54×1 | 669.09 | 368.90 | 105.16 | 174.62 | 1203.70 |

## Peak process RSS in MiB

The last two columns give macOS physical footprint. All columns include initialization, warmup, and the timed proofs.

| Shape | M5 CPU | M5 Metal | AWS Zopro | AWS Aeglos | AWS Gnark | M5 CPU footprint | M5 Metal footprint |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 70.2 | 95.1 | 40.8 | 452.9 | 264.9 | 49.1 | 170.5 |
| 1×4 | 67.0 | 93.6 | 43.0 | 571.0 | 393.4 | 42.3 | 179.3 |
| 1×8 | 75.6 | 107.8 | 62.6 | 674.0 | 413.4 | 47.2 | 190.1 |
| 1×16 | 98.2 | 130.5 | 75.4 | 805.4 | 421.2 | 62.9 | 206.9 |
| 2×2 | 99.5 | 124.9 | 60.9 | 803.8 | 433.1 | 66.8 | 206.9 |
| 2×4 | 99.0 | 116.2 | 138.1 | 807.3 | 436.6 | 64.8 | 193.4 |
| 2×8 | 133.7 | 185.0 | 140.4 | 1003.1 | 531.8 | 91.8 | 244.8 |
| 2×16 | 148.4 | 221.2 | 174.8 | 1190.0 | 537.8 | 99.5 | 274.7 |
| 3×2 | 132.3 | 196.3 | 147.2 | 1154.2 | 503.8 | 86.3 | 252.8 |
| 3×4 | 132.2 | 200.2 | 153.0 | 1192.7 | 573.4 | 84.8 | 255.9 |
| 3×8 | 143.1 | 212.8 | 163.1 | 1260.7 | 546.2 | 91.9 | 264.6 |
| 4×2 | 145.7 | 221.2 | 180.2 | 1329.5 | 535.8 | 90.2 | 271.1 |
| 4×4 | 154.4 | 227.1 | 169.1 | 1346.4 | 570.8 | 97.5 | 275.5 |
| 4×8 | 163.8 | 239.0 | 194.4 | 1445.6 | 627.9 | 103.1 | 283.5 |
| 4×16 | 179.0 | 264.8 | 209.4 | 1568.8 | 578.1 | 111.4 | 303.2 |
| 5×2 | 169.4 | 243.8 | 209.4 | 1526.9 | 585.5 | 104.4 | 286.4 |
| 5×4 | 169.8 | 249.7 | 223.7 | 1551.2 | 532.3 | 103.2 | 290.7 |
| 5×8 | 199.2 | 274.1 | 288.2 | 1909.8 | 677.0 | 121.0 | 341.5 |
| 5×16 | 219.1 | 293.5 | 321.9 | 2012.2 | 765.6 | 133.8 | 354.1 |
| 6×2 | 199.5 | 254.4 | 280.1 | 1972.4 | 729.1 | 117.0 | 310.4 |
| 6×4 | 201.2 | 283.6 | 304.0 | 1992.8 | 754.1 | 117.2 | 346.6 |
| 6×8 | 217.5 | 293.6 | 316.8 | 2145.7 | 658.4 | 129.7 | 354.9 |
| 8×2 | 230.6 | 298.2 | 304.2 | 2418.9 | 903.5 | 129.0 | 334.9 |
| 8×4 | 238.7 | 304.2 | 372.6 | 2458.1 | 921.0 | 135.7 | 338.1 |
| 8×8 | 252.8 | 319.3 | 373.1 | 2521.4 | 952.9 | 145.9 | 349.4 |
| 8×16 | 259.7 | 361.6 | 431.0 | 2678.3 | 957.4 | 145.9 | 398.6 |
| 12×2 | 341.0 | 451.5 | 454.0 | 3594.3 | 1296.6 | 185.4 | 509.6 |
| 12×4 | 346.3 | 463.9 | 517.7 | 3621.9 | 1322.0 | 189.1 | 526.3 |
| 12×8 | 352.2 | 466.9 | 537.3 | 3694.5 | 1231.9 | 191.3 | 519.7 |
| 16×2 | 401.1 | 547.0 | 573.4 | 4540.3 | 1466.6 | 207.5 | 566.3 |
| 16×4 | 395.1 | 548.2 | 641.2 | 4576.8 | 1390.8 | 199.9 | 565.9 |
| 16×8 | 400.9 | 552.3 | 673.4 | 4664.6 | 1480.5 | 201.9 | 566.1 |
| 24×2 | 610.8 | 946.0 | 882.0 | 6664.4 | 2518.1 | 308.7 | 937.2 |
| 24×4 | 607.7 | 947.1 | 1029.6 | 6643.7 | 2220.9 | 303.7 | 936.6 |
| 32×2 | 709.1 | 1106.0 | 1231.3 | 8284.0 | 2646.2 | 330.5 | 1009.5 |
| 40×2 | 817.7 | 1177.5 | 1429.0 | 9650.5 | 3305.2 | 363.9 | 1022.0 |
| 48×2 | 1139.9 | 1696.5 | 1693.6 | 12228.6 | 4481.4 | 546.6 | 1602.8 |
| 49×2 | 1153.4 | 1729.5 | 1552.5 | 12410.4 | 4615.1 | 550.7 | 1603.0 |
| merge-8×1 | 222.1 | 286.8 | 324.5 | 2339.6 | 700.2 | 125.1 | 327.7 |
| merge-24×1 | 570.7 | 818.6 | 929.5 | 6517.5 | 2300.6 | 279.9 | 825.8 |
| merge-54×1 | 1179.9 | 1779.9 | 1725.7 | 12878.1 | 4907.1 | 551.5 | 1624.0 |

## Measurement details

- **M5 Pro** used 18 CPU cores and 48 GiB of unified memory on macOS 27.2.1. Zopro ran at commit `0904d31`, with default worker selection. Each backend ran in three fresh processes per shape, alternating CPU and Metal order. Each process primed three proofs and timed five. The table takes the median of the three process means.
- **AWS** used the catalog request from the three-prover run on a 96-core c7a.24xlarge. Each engine used its recorded build and placement. Its hot timing likewise uses the median of three process means, each covering five warm responses after three primes. The CSV also carries the AWS cold measurements and the separately tuned Zopro and Aeglos results from the original scorecard. [AWS three-prover data](https://github.com/helius-labs/zk-tools/blob/d942372f85491f42596d157ed4c3424277d99e4c/zopro/docs/benchmarks/2026-11-20-c7a-three-provers/artifacts/single-summary.csv) · [Original tuned AWS scorecard](https://github.com/helius-labs/zk-tools/blob/d942372f85491f42596d157ed4c3424277d99e4c/zopro/docs/benchmarks/2026-11-20-c7a-41-shape-latency-memory/README.md)
- **Workloads** used the same 41 circuit shapes and matching Gnark key hashes. The M5 runs used deterministic fixtures exported from the historical Zolana test suite, while AWS used its saved catalog requests. M5 warm time starts with decoding the exported witness and includes proof generation and verification. AWS warm time includes request-level witness construction, proof generation, and verification. [Zopro revision](https://github.com/helius-labs/zk-tools/commit/0904d3119aea84eac2fe9b9d790e0e9d66649b10) · [Zolana fixture revision](https://github.com/helius-labs/zolana/commit/2f1f02bc7ffc05c9a42a7d62e4c2ee4c06c0ec99) · [AWS input and key pins](https://github.com/helius-labs/zk-tools/blob/d942372f85491f42596d157ed4c3424277d99e4c/zopro/docs/benchmarks/2026-11-20-c7a-three-provers/artifacts/pins.json)
- **Memory** uses the maximum across processes for each configuration. The M5 runner records BSD `time -l` peak RSS and peak memory footprint. These are distinct macOS counters, and RSS can account for mapped pages differently across macOS and Linux. M5 images were prepared before starting the timed process. The CSV labels its first response from an existing mapped image separately from the AWS cold response.
- **Verification** ran on every M5 proof. The runner checked its public inputs against the fixture and checked that the requested CPU or Metal backend stayed active. The raw blocks record each key hash, archive hash, witness hash, and proof time. [AWS verification ledger](https://github.com/helius-labs/zk-tools/blob/d942372f85491f42596d157ed4c3424277d99e4c/zopro/docs/benchmarks/2026-11-20-c7a-three-provers/artifacts/verification.json)

## M5 Pro follow-up on two large shapes

The follow-up measures scalar grouping that reserves scratch for each phase and splits oversized point groups into bounded tiles. The same M5 Pro ran both builds with the default memory policy. Each value uses three fresh processes per configuration and the same key and witness as the catalog above. Warm time is the median of the process means. Memory is the maximum across processes. [Previous build](https://github.com/helius-labs/zk-tools/commit/06292c48928eb1d19ad409e26cbd923bdaf8bf37) · [Raw follow-up blocks](artifacts/m5-scalar-grouping.jsonl)

| Shape and backend | Previous warm ms | Updated warm ms | Previous RSS MiB | Updated RSS MiB | Previous footprint MiB | Updated footprint MiB |
|---|---:|---:|---:|---:|---:|---:|
| 24×2 CPU | 269.0 | 241.6 | 618.8 | 595.3 | 319.3 | 293.3 |
| 24×2 Metal | 131.4 | 89.0 | 918.6 | 810.7 | 909.7 | 806.1 |
| 40×2 CPU | 445.9 | 254.4 | 821.5 | 838.8 | 367.8 | 385.0 |
| 40×2 Metal | 208.1 | 99.2 | 1276.5 | 1143.2 | 1117.0 | 987.6 |

The three 40×2 Metal reductions requested about 21 MiB, 25 MiB, and 40 MiB. Admission depends on the host's free memory and the other queries' live reservations. The raw blocks include the pre-run free-page count and the run with an explicit 145 MiB preparation ceiling.

## M5 Pro follow-up on the 1×2 CPU proof

The paired CPU schedule queues reduction after all point ranges for a window finish. The M5 Pro used `zopro-spr` with a mapped raw4 image and the repository's 1×2 request fixture. Each of 32 blocks ran three builds in randomized order. A fresh process primed two proofs and timed six. Warm time is the median of the process medians. Memory is the maximum across processes. [Request fixture](../../../tools/spr/fixtures/transfer-1x2.json) · [Raw blocks](artifacts/m5-1x2-cpu-scheduling.jsonl)

| Build | Warm proof ms | Peak RSS MiB | Peak physical footprint MiB |
|---|---:|---:|---:|
| Original PR `87b35e3` | 30.82 | 43.56 | 22.89 |
| Bounded grouping `70941be` | 17.07 | 73.33 | 52.13 |
| Queued window reduction | 16.64 | 74.41 | 53.20 |

## M5 Pro follow-up on scalar gathering

The 1×2 CPU schedule gathers A scalars while the G1 pool prepares K and computes the quotient. The G2 pool gathers B2 scalars when it starts. The measurement uses the same `zopro-spr` runner and repository request fixtures as the previous follow-up, with 36 randomized three-build blocks for 1×2 CPU and 10 for each other configuration. The speedup is the median of the original-to-updated latency ratios within blocks. [Original PR build](https://github.com/helius-labs/zk-tools/commit/87b35e3f8cce89bc60ae83344c9166ab59058e3b) · [Queued window reduction](https://github.com/helius-labs/zk-tools/commit/842b05a3d7a31b302abe37edae30e5362454b02e) · [Raw blocks](artifacts/m5-apple-cpu-scalar-handoff.jsonl)

| Shape and backend | Original warm ms | Queued warm ms | Updated warm ms | Speedup | Queued peak RSS MiB | Updated peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 CPU | 30.79 | 16.68 | 16.50 | 1.866× | 76.42 | 75.28 |
| 1×2 Metal | 25.99 | 11.57 | 11.57 | 2.244× | 97.66 | 97.42 |
| 2×2 CPU | 51.80 | 24.32 | 24.39 | 2.132× | 98.47 | 97.86 |
| 2×2 Metal | 34.04 | 15.39 | 15.37 | 2.210× | 129.16 | 127.84 |
| 12×2 CPU | 238.90 | 111.60 | 112.87 | 2.129× | 345.20 | 349.45 |
| 12×2 Metal | 108.62 | 50.02 | 49.73 | 2.129× | 444.91 | 448.69 |

The 1×2 CPU proof gained 1.0% over queued window reduction in paired runs. Its peak physical footprint went from 55.28 to 54.11 MiB. When K grouping is unavailable, the overlapping schedule uses a temporary copy of the private scalars for the ungrouped query.

The first 12×2 CPU sample was 1.1% slower with the updated build. A 24-block repeat measured 113.83 ms for queued reduction and 112.07 ms for the update, with a median paired speedup of 1.004×. Both series are in the raw blocks.

## M5 Pro follow-up on G1 affine batches

The G1 affine batch kernel stores each denominator beside its scaled numerator and starts the next slope before writing the current point. These measurements use 36 randomized three-build blocks for 1×2 CPU and 10 for each other configuration, with the same `zopro-spr` runner, proof count, and repository fixtures as the scalar-gathering follow-up. Speedup reports the median original-to-updated ratio within blocks. [Previous build](https://github.com/helius-labs/zk-tools/commit/66ac1a41558ff20814a59315dfee9b8e84a2ed8d) · [Raw blocks](artifacts/m5-pipelined-g1-batches.jsonl)

| Shape and backend | Original warm ms | Previous warm ms | Updated warm ms | Speedup | Previous peak RSS MiB | Updated peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 CPU | 30.77 | 16.42 | 16.32 | 1.883× | 74.81 | 76.00 |
| 1×2 Metal | 26.05 | 11.56 | 11.55 | 2.250× | 98.72 | 97.48 |
| 2×2 CPU | 52.02 | 24.53 | 23.95 | 2.175× | 98.95 | 97.73 |
| 2×2 Metal | 34.04 | 15.35 | 15.36 | 2.212× | 126.94 | 129.25 |
| 12×2 CPU | 240.15 | 112.76 | 110.95 | 2.151× | 348.09 | 347.66 |
| 12×2 Metal | 102.51 | 49.29 | 49.24 | 2.086× | 444.42 | 447.48 |

The median paired gain over the previous build was 0.6% for 1×2 CPU and 2.0% for 2×2 CPU. Peak physical footprint for 1×2 CPU went from 53.64 to 54.84 MiB.

## M5 Pro follow-up on G2 affine batches

The seven-worker G2 query moves the extension-field numerator into the forward pass and starts the next slope before writing the current point. It uses this kernel for batches of 64 or more. These measurements use the same randomized three-build comparison as the G1 follow-up. [Previous build](https://github.com/helius-labs/zk-tools/commit/900704a2d36e72cbfb0f9f8ceab75ebb599515d7) · [Raw blocks](artifacts/m5-pipelined-g2-batches.jsonl)

| Shape and backend | Original warm ms | Previous warm ms | Updated warm ms | Speedup | Previous peak RSS MiB | Updated peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| 1×2 CPU | 30.86 | 16.35 | 16.21 | 1.900× | 76.02 | 74.19 |
| 1×2 Metal | 26.00 | 11.58 | 11.54 | 2.253× | 98.17 | 97.52 |
| 2×2 CPU | 52.28 | 24.15 | 24.13 | 2.163× | 98.89 | 99.22 |
| 2×2 Metal | 34.00 | 15.37 | 15.38 | 2.212× | 130.44 | 130.27 |
| 12×2 CPU | 238.39 | 112.45 | 112.93 | 2.138× | 348.83 | 347.39 |
| 12×2 Metal | 102.38 | 49.08 | 49.13 | 2.091× | 450.09 | 449.09 |

The median paired gain over the previous build was 1.2% for 1×2 CPU. Peak physical footprint for this configuration went from 54.84 to 52.94 MiB.
