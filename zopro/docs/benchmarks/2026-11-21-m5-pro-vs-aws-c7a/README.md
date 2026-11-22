# M5 Pro and AWS C7a proof benchmarks

All 41 circuit shapes from the AWS catalog have warm proof latency and peak process RSS results for M5 Pro Zopro CPU, M5 Pro Zopro Metal, and AWS Zopro, Aeglos, and Gnark.

- M5 Metal took **12.0–368.9 ms** per proof and was **1.88×** faster than M5 CPU at the median shape ratio. The CPU range was **18.6–748.1 ms**.
- Against the 96-core AWS run, M5 Metal took **1.64×** as long as Zopro at the median shape ratio and beat Aeglos on **5/41** shapes. AWS Gnark took **5.26×** as long as M5 Metal at the median shape ratio.
- M5 Metal peak process RSS was **93.6–1779.9 MiB**. Its macOS peak physical footprint was **170.5–1624.0 MiB**. The footprint series is in the memory table and CSV.

[Comparison CSV](comparison.csv) · [Raw M5 blocks](artifacts/m5-blocks.jsonl) · [Apple silicon speedup audit](#apple-silicon-speedup-across-all-41-shapes) · [CPU and Metal follow-up](#apple-silicon-cpu-and-metal-schedules) · [AWS three-prover report](https://github.com/helius-labs/zk-tools/blob/d942372f85491f42596d157ed4c3424277d99e4c/zopro/docs/benchmarks/2026-11-20-c7a-three-provers/README.md#catalog-hot)

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

## M5 Pro follow-up on loose G2 field arithmetic

The seven-worker G2 affine batch kernel carries bounded Montgomery values through the prefix products and coordinate arithmetic. It reduces the input to inversion, the slope before multiplying by a loose coordinate difference, and the final point. The benchmark used 46 randomized three-build blocks for 1×2 CPU and 10 for each other configuration. Each process ran eight verified proofs, and its warm latency is the median of proofs three through eight. The RSS columns report the median process high-water mark. [Previous build](https://github.com/helius-labs/zk-tools/commit/275d42f542077e0f453b9c6169600dc6a81d9937) · [Raw blocks](artifacts/m5-loose-g2-batches.jsonl)

| Shape and backend | Original warm ms | Previous warm ms | Updated warm ms | Speedup | Previous peak RSS MiB | Updated peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 CPU | 30.862 | 16.177 | 15.949 | 1.931× | 72.484 | 72.055 |
| 1×2 Metal | 26.007 | 11.547 | 11.533 | 2.254× | 95.250 | 96.953 |
| 2×2 CPU | 52.283 | 24.110 | 24.127 | 2.173× | 96.438 | 97.539 |
| 2×2 Metal | 34.043 | 15.367 | 15.359 | 2.218× | 125.281 | 125.938 |
| 12×2 CPU | 240.012 | 113.138 | 113.532 | 2.122× | 342.422 | 343.438 |
| 12×2 Metal | 102.537 | 49.091 | 49.232 | 2.086× | 442.453 | 443.664 |

Speedup is the median of the original-to-updated ratios within blocks. The 1×2 CPU proof gained 1.6% over the previous build in paired runs and was faster in 43 of 46 blocks. Median peak physical footprint fell from 51.25 to 50.83 MiB.

## M5 Pro follow-up on affine bucket checks

The borrowed G1 and G2 window kernels check one word of the x coordinate before doing complete point comparisons. A nonzero word rules out the identity, and different words rule out coincident x coordinates. The benchmark used 46 randomized three-build blocks for 1×2 CPU and 10 for each other case. Each process verified eight proofs, and its warm latency is the median of proofs three through eight. [Previous build](https://github.com/helius-labs/zk-tools/commit/424ec2156602b4d400b0d97e23390a1beb3195f6) · [Raw blocks](artifacts/m5-borrowed-bucket-checks.jsonl)

| Shape and backend | Original warm ms | Previous warm ms | Updated warm ms | Speedup | Previous peak RSS MiB | Updated peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 CPU | 30.806 | 15.930 | 15.866 | 1.934× | 71.797 | 71.477 |
| 1×2 Metal | 26.058 | 11.552 | 11.541 | 2.250× | 95.812 | 96.273 |
| 2×2 CPU | 52.008 | 24.232 | 23.909 | 2.166× | 98.320 | 97.125 |
| 2×2 Metal | 34.012 | 15.339 | 15.363 | 2.212× | 126.414 | 126.305 |
| 12×2 CPU | 240.908 | 113.914 | 112.094 | 2.159× | 340.078 | 342.352 |
| 12×2 Metal | 101.775 | 49.148 | 48.898 | 2.084× | 443.312 | 444.672 |

Speedup is the median of the original-to-updated ratios within blocks. Peak RSS is the median process high-water mark. The paired CPU gains over the previous build were 0.4% for 1×2, 1.7% for 2×2, and 2.7% for 12×2. The 40-block repeat for 1×2 measured 15.958 to 15.877 ms with a 0.9% paired gain, and the 24-block repeat for 2×2 measured 24.089 to 24.008 ms with a 0.5% paired gain.

## M5 Pro follow-up on the small CPU proof schedule

The 11-worker G1 pool computes the quotient and gathers A while the seven-worker B2 pool also prepares K. The A query uses widths 11 and 12 for its final two windows. The fused K/H query moves the 15-bit window to the beginning. These changes apply to the small paired CPU schedule. [Previous build](https://github.com/helius-labs/zk-tools/commit/a4ea03da44ce187a0e190c734542eaf146d3b7e8) · [Raw blocks](artifacts/m5-paired-g1-layout.jsonl)

The benchmark used 48 randomized three-build blocks for 1×2 CPU, 40 for 2×2 CPU, and 10 for each other case. Each process verified eight proofs. Warm latency is the median of proofs three through eight.

| Shape and backend | Original warm ms | Previous warm ms | Updated warm ms | Speedup | Previous peak RSS MiB | Updated peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 CPU | 30.736 | 15.870 | 15.720 | 1.955× | 71.695 | 70.898 |
| 1×2 Metal | 25.976 | 11.576 | 11.578 | 2.245× | 96.648 | 94.992 |
| 2×2 CPU | 53.380 | 24.341 | 24.338 | 2.195× | 96.812 | 96.367 |
| 2×2 Metal | 34.051 | 15.398 | 15.336 | 2.224× | 125.898 | 126.312 |
| 12×2 CPU | 244.518 | 115.779 | 115.353 | 2.151× | 337.484 | 335.930 |
| 12×2 Metal | 103.073 | 49.195 | 49.155 | 2.092× | 443.242 | 442.766 |

Speedup is the median of the original-to-updated ratios within blocks. Peak RSS is the median process high-water mark. The paired 1×2 CPU gain over the previous build was 1.0%. The paired change on 2×2 CPU was +0.2%. A separate 48-block 1×2 CPU repeat measured 15.964 to 15.735 ms, a 1.4% paired gain.

## Apple silicon speedup across all 41 shapes

The median shape speedup was **2.06× on CPU** and **2.02× on Metal**. CPU reached 2× on **24/41** shapes and Metal reached 2× on **22/41** shapes. **17 shapes** reached 2× on both backends.

Both builds ran on the same M5 Pro. The runner used the pinned keys and exported witnesses from the AWS comparison above. [Original PR build](https://github.com/helius-labs/zk-tools/commit/87b35e3f8cce89bc60ae83344c9166ab59058e3b) · [Updated build](https://github.com/helius-labs/zk-tools/commit/f294171359db54af54b8483957f0b3df60ed2fd5) · [Comparison CSV](m5-original-current-catalog.csv) · [Raw blocks](artifacts/m5-original-current-catalog-blocks.jsonl)

Each shape ran in three paired blocks. Within a block, the four build and backend combinations ran in random order in fresh processes. Each process verified eight proofs, checked its public witness, and confirmed the backend after every proof. Warm latency is the median of the block means for proofs four through eight. Speedup is the median of the paired latency ratios. Peak RSS is the median process high-water mark. The warm timing includes witness decoding, proving, and verification.

| Shape | CPU original ms | CPU updated ms | CPU speedup | CPU peak RSS MiB | Metal original ms | Metal updated ms | Metal speedup | Metal peak RSS MiB |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1×2 | 30.53 | 15.67 | 1.95× | 41.8 → 71.1 | 26.06 | 11.51 | 2.26× | 85.8 → 93.3 |
| 1×4 | 41.35 | 26.00 | 1.58× | 51.2 → 66.6 | 29.54 | 14.63 | 2.02× | 104.0 → 92.2 |
| 1×8 | 47.11 | 31.10 | 1.54× | 59.7 → 75.2 | 31.23 | 17.07 | 1.83× | 115.2 → 109.0 |
| 1×16 | 56.91 | 33.54 | 1.68× | 69.9 → 97.0 | 35.85 | 20.77 | 1.72× | 133.6 → 130.0 |
| 2×2 | 52.69 | 23.69 | 2.19× | 66.0 → 95.8 | 33.90 | 15.31 | 2.22× | 127.1 → 126.4 |
| 2×4 | 54.32 | 24.88 | 2.18× | 69.0 → 99.0 | 34.75 | 17.76 | 1.96× | 131.6 → 114.4 |
| 2×8 | 70.87 | 43.87 | 1.63× | 96.5 → 131.4 | 41.81 | 28.10 | 1.48× | 166.1 → 184.2 |
| 2×16 | 80.84 | 48.25 | 1.68× | 116.3 → 150.9 | 48.42 | 29.04 | 1.70× | 195.4 → 219.6 |
| 3×2 | 72.94 | 42.47 | 1.74× | 110.3 → 135.4 | 45.53 | 27.18 | 1.65× | 184.1 → 195.5 |
| 3×4 | 80.26 | 43.82 | 1.78× | 114.5 → 138.6 | 45.46 | 28.12 | 1.63× | 190.5 → 200.8 |
| 3×8 | 82.76 | 45.04 | 1.86× | 122.7 → 148.0 | 47.95 | 27.87 | 1.71× | 202.8 → 213.3 |
| 4×2 | 88.03 | 44.99 | 1.96× | 126.2 → 151.7 | 53.29 | 28.15 | 1.92× | 212.8 → 220.9 |
| 4×4 | 92.80 | 44.79 | 2.05× | 129.1 → 156.8 | 54.90 | 27.51 | 1.92× | 216.6 → 227.6 |
| 4×8 | 98.38 | 47.24 | 2.05× | 136.8 → 166.7 | 57.69 | 28.77 | 2.00× | 228.8 → 238.1 |
| 4×16 | 108.43 | 52.77 | 2.06× | 151.7 → 184.5 | 54.11 | 32.15 | 1.69× | 267.5 → 263.0 |
| 5×2 | 103.73 | 45.59 | 2.24× | 138.8 → 170.7 | 61.58 | 30.07 | 2.11× | 241.0 → 243.6 |
| 5×4 | 107.31 | 46.07 | 2.33× | 141.7 → 169.3 | 63.59 | 31.23 | 2.02× | 245.7 → 248.5 |
| 5×8 | 133.58 | 66.34 | 2.01× | 183.7 → 203.8 | 64.25 | 36.76 | 1.74× | 321.0 → 272.9 |
| 5×16 | 142.41 | 71.11 | 2.00× | 187.1 → 218.4 | 69.05 | 38.96 | 1.76× | 338.3 → 291.9 |
| 6×2 | 135.84 | 65.05 | 2.09× | 187.9 → 207.4 | 65.62 | 31.26 | 2.10× | 335.2 → 255.0 |
| 6×4 | 138.24 | 67.25 | 2.06× | 193.0 → 210.3 | 68.24 | 36.33 | 1.88× | 340.7 → 281.3 |
| 6×8 | 141.30 | 67.78 | 2.11× | 189.2 → 217.0 | 68.07 | 37.56 | 1.82× | 348.9 → 292.5 |
| 8×2 | 155.47 | 67.35 | 2.30× | 207.0 → 233.2 | 70.60 | 33.46 | 2.11× | 383.5 → 298.7 |
| 8×4 | 157.39 | 67.79 | 2.30× | 211.7 → 239.4 | 73.74 | 33.88 | 2.18× | 387.4 → 303.3 |
| 8×8 | 159.52 | 70.74 | 2.25× | 216.3 → 249.4 | 73.13 | 35.84 | 2.04× | 396.8 → 312.2 |
| 8×16 | 170.03 | 75.67 | 2.25× | 225.4 → 259.7 | 78.21 | 41.22 | 1.89× | 415.5 → 362.6 |
| 12×2 | 249.83 | 111.84 | 2.14× | 300.8 → 339.8 | 105.23 | 48.69 | 2.17× | 583.5 → 440.7 |
| 12×4 | 254.96 | 120.11 | 2.15× | 302.9 → 333.6 | 106.70 | 50.53 | 2.10× | 587.7 → 444.7 |
| 12×8 | 256.37 | 120.07 | 2.13× | 312.5 → 335.8 | 107.47 | 51.02 | 2.11× | 599.2 → 457.0 |
| 16×2 | 269.61 | 114.36 | 2.33× | 354.8 → 396.5 | 118.57 | 52.46 | 2.25× | 679.2 → 521.2 |
| 16×4 | 275.98 | 121.36 | 2.36× | 359.7 → 398.1 | 122.21 | 52.21 | 2.29× | 685.2 → 529.2 |
| 16×8 | 305.05 | 127.92 | 2.38× | 364.1 → 401.1 | 130.80 | 55.25 | 2.34× | 694.4 → 535.3 |
| 24×2 | 471.17 | 256.49 | 1.84× | 575.0 → 591.2 | 182.69 | 88.73 | 2.06× | 1086.3 → 808.1 |
| 24×4 | 446.71 | 240.68 | 1.85× | 605.0 → 598.8 | 178.63 | 88.57 | 2.02× | 1090.5 → 814.6 |
| 32×2 | 519.11 | 247.11 | 2.10× | 710.6 → 718.1 | 218.17 | 93.25 | 2.36× | 1276.1 → 985.2 |
| 40×2 | 583.87 | 247.24 | 2.37× | 793.4 → 827.4 | 261.49 | 99.48 | 2.62× | 1463.5 → 1144.6 |
| 48×2 | 949.73 | 555.98 | 1.71× | 1120.7 → 1147.8 | 428.28 | 265.27 | 1.61× | 2082.2 → 1540.2 |
| 49×2 | 973.31 | 563.83 | 1.74× | 1134.6 → 1160.8 | 430.31 | 267.83 | 1.62× | 2105.1 → 1563.5 |
| merge-8×1 | 146.94 | 65.01 | 2.30× | 202.1 → 224.1 | 70.55 | 32.39 | 2.18× | 369.9 → 284.5 |
| merge-24×1 | 426.68 | 237.15 | 1.79× | 559.4 → 571.9 | 180.52 | 87.39 | 2.07× | 1055.3 → 780.1 |
| merge-54×1 | 993.51 | 550.81 | 1.80× | 1187.1 → 1181.5 | 452.31 | 266.70 | 1.70× | 2174.5 → 1595.8 |

## Apple silicon CPU and Metal schedules

The CPU update overlaps the 65,536-point quotient with witness preparation and G2 work. Metal uses ten-bit G1 windows, device-side quotient recoding, and concurrent witness sorts for the 131,072-point domain. The shortest B2 queries in that Metal domain share the existing separate CPU preparation pool.

Six paired blocks per row compare the original and audited builds with the new schedules on the same M5 Pro. Each block used fresh processes, randomized build order, and the same pinned key and witness. Each process verified eight proofs and checked the public witness and backend. Warm time uses proofs four through eight. The table uses medians of the warm block means, paired speedups, and process memory high-water marks.

[New build](https://github.com/helius-labs/zk-tools/commit/9a5ac0ecbfefe1dffad2b20e0329d359ed070067) · [Comparison CSV](m5-apple-proof-schedules.csv) · [Raw blocks](artifacts/m5-apple-proof-schedules.jsonl)

| Shape | Backend | Audited ms | New ms | vs. audited | vs. original | Peak RSS MiB | Peak physical footprint MiB |
|---|---|---:|---:|---:|---:|---:|---:|
| 1×2 | CPU | 15.67 | 15.65 | 0.999× | 1.912× | 70.1 → 71.3 | 48.9 → 50.2 |
| 1×4 | CPU | 27.08 | 23.71 | 1.135× | 1.765× | 66.2 → 78.1 | 41.5 → 53.3 |
| 1×8 | CPU | 29.37 | 25.75 | 1.142× | 1.808× | 75.4 → 88.1 | 47.0 → 59.7 |
| 1×16 | CPU | 33.13 | 27.08 | 1.224× | 2.028× | 97.3 → 108.6 | 61.9 → 73.2 |
| 2×2 | CPU | 23.90 | 23.93 | 1.009× | 2.145× | 96.6 → 99.3 | 63.8 → 66.5 |
| 2×4 | CPU | 24.80 | 24.05 | 1.030× | 2.261× | 99.2 → 102.7 | 64.9 → 68.2 |
| 2×8 | CPU | 43.42 | 43.84 | 0.991× | 1.598× | 129.3 → 129.6 | 87.4 → 87.7 |
| 1×4 | Metal | 14.72 | 14.66 | 1.005× | 1.990× | 93.3 → 92.5 | 179.8 → 178.6 |
| 2×8 | Metal | 28.41 | 23.26 | 1.213× | 1.824× | 184.5 → 151.6 | 244.3 → 233.1 |
| 2×16 | Metal | 33.26 | 27.20 | 1.227× | 1.905× | 218.8 → 176.0 | 272.3 → 248.6 |
| 3×2 | Metal | 28.44 | 23.98 | 1.186× | 1.891× | 195.5 → 152.5 | 252.0 → 229.3 |
| 3×4 | Metal | 29.22 | 24.38 | 1.203× | 1.946× | 199.5 → 157.3 | 255.3 → 232.8 |
| 3×8 | Metal | 29.55 | 25.34 | 1.164× | 1.988× | 212.5 → 171.0 | 264.3 → 242.5 |
| 4×2 | Metal | 29.37 | 24.82 | 1.208× | 2.217× | 220.7 → 175.6 | 270.7 → 249.9 |
| 4×4 | Metal | 28.89 | 25.02 | 1.156× | 2.142× | 226.5 → 181.1 | 274.9 → 253.6 |
| 4×8 | Metal | 29.76 | 26.49 | 1.120× | 2.244× | 237.9 → 192.5 | 282.4 → 261.3 |
| 4×16 | Metal | 32.43 | 29.42 | 1.102× | 1.889× | 263.1 → 217.4 | 301.5 → 279.1 |
| 5×2 | Metal | 30.59 | 26.68 | 1.183× | 2.476× | 243.9 → 197.9 | 286.3 → 264.5 |
| 5×4 | Metal | 32.27 | 27.14 | 1.219× | 2.483× | 247.4 → 202.1 | 288.3 → 267.3 |
| 5×8 | Metal | 36.95 | 38.29 | 0.968× | 1.715× | 272.5 → 272.9 | 339.9 → 340.3 |

All eleven affected Metal shapes improved and used lower peak RSS. The CPU changes increased peak RSS by up to 12.7 MiB in the affected class.

Sixteen additional paired blocks on each neighboring Metal shape measured 1.016× on 5×8 and 1.000× on 6×8 versus the audited build. The 5×8 slowdown in the initial six blocks did not repeat. [Raw control blocks](artifacts/m5-apple-proof-schedule-controls.jsonl)
