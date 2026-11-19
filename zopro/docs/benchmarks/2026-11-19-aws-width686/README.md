# Zopro and Aeglos across C7a physical-core placements

Eight blocks on one c7a.24xlarge compared the retained SDK, current ordinary release and prepared Aeglos at 32 and 96 physical cores with SMT disabled. Every process warmed three proofs and timed eleven sequential proofs. The main table shows medians of process means, paired changes within each block, warm CPU-ms per proof and the maximum lifetime peak RSS including setup. Every timed Aeglos request reported a prepared-key cache hit. All 256 process exports passed both remote and independent local verification with the original key and expected public inputs.

At 96 cores the earlier Ryzen-tuned options measured 9.403 ms on 1×2 against 8.302 ms for Aeglos. On 1×16 the ordinary release took 19.857 ms against 12.577 ms. The 5×16 packed FFT took 26.859 ms against 27.420 ms, while enabling the additional grouped G2 option took 28.793 ms. On merge54 the tuned release reached 103.392 ms against 172.570 ms for Aeglos. The same 5×16 grouped option led at 32 cores. These observations apply to the tested keys and witnesses.

Prior uses the historical catalog executable. Release uses the current ordinary executable. Tuned on 1×2 combines an eight-worker frontend and two G2 ranges with the earlier Ryzen profile. Tuned on 5×16 enables packed FFT and owned additionally enables grouped G2. The main pool always uses the stated physical width. See the workflow for the full merge54 settings. Each row with a supplied Apple reference includes both speedups. Among these four shapes, only 1×2 has a supplied M5 reference.

| Shape | Physical cores | Configuration | Proof ms | Paired vs Aeglos | Paired vs release | CPU-ms | Peak MiB | M5 CPU / Metal |
|---|---:|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 32 | prior | 10.903 | -9.65% (8/8) | -0.01% (4/8) | 228.5 | 70.7 | 3.39× / 2.20× |
| 1x2 | 32 | release | 10.977 | -8.47% (8/8) | +0.00% (0/8) | 232.4 | 63.2 | 3.37× / 2.19× |
| 1x2 | 32 | tuned | 10.103 | -15.61% (8/8) | -8.26% (8/8) | 199.5 | 52.7 | 3.66× / 2.38× |
| 1x2 | 32 | aeglos | 12.027 | +0.00% (0/8) | +9.25% (0/8) | 327.0 | 411.5 | 3.08× / 2.00× |
| 1x2 | 96 | prior | 12.578 | +50.95% (0/8) | +1.97% (1/8) | 690.4 | 68.5 | 2.94× / 1.91× |
| 1x2 | 96 | release | 12.354 | +48.04% (0/8) | +0.00% (0/8) | 628.0 | 46.8 | 3.00× / 1.94× |
| 1x2 | 96 | tuned | 9.403 | +14.84% (1/8) | -23.68% (8/8) | 395.0 | 43.0 | 3.93× / 2.55× |
| 1x2 | 96 | aeglos | 8.302 | +0.00% (0/8) | -32.45% (8/8) | 521.7 | 454.8 | 4.46× / 2.89× |
| 1x16 | 32 | prior | 17.911 | -13.59% (8/8) | +2.87% (1/8) | 404.7 | 276.3 | — |
| 1x16 | 32 | release | 17.376 | -16.20% (8/8) | +0.00% (0/8) | 400.3 | 244.8 | — |
| 1x16 | 32 | aeglos | 20.729 | +0.00% (0/8) | +19.34% (0/8) | 567.3 | 763.5 | — |
| 1x16 | 96 | prior | 20.623 | +65.95% (0/8) | +4.75% (0/8) | 1211.0 | 264.3 | — |
| 1x16 | 96 | release | 19.857 | +58.94% (0/8) | +0.00% (0/8) | 1109.2 | 235.7 | — |
| 1x16 | 96 | aeglos | 12.577 | +0.00% (0/8) | -37.08% (8/8) | 865.6 | 824.1 | — |
| 5x16 | 32 | prior | 27.922 | -41.50% (8/8) | +0.84% (1/8) | 739.1 | 416.1 | — |
| 5x16 | 32 | release | 27.703 | -42.36% (8/8) | +0.00% (0/8) | 730.0 | 410.8 | — |
| 5x16 | 32 | tuned | 26.906 | -43.68% (8/8) | -3.20% (7/8) | 711.3 | 422.0 | — |
| 5x16 | 32 | owned | 26.236 | -45.40% (8/8) | -4.50% (8/8) | 690.9 | 427.1 | — |
| 5x16 | 32 | aeglos | 47.892 | +0.00% (0/8) | +73.49% (0/8) | 1463.5 | 1913.5 | — |
| 5x16 | 96 | prior | 29.804 | +8.49% (0/8) | +10.37% (0/8) | 2097.4 | 416.7 | — |
| 5x16 | 96 | release | 27.116 | -2.27% (7/8) | +0.00% (0/8) | 1826.5 | 394.0 | — |
| 5x16 | 96 | tuned | 26.859 | -1.94% (8/8) | -1.95% (5/8) | 1746.3 | 395.8 | — |
| 5x16 | 96 | owned | 28.793 | +5.01% (0/8) | +5.95% (0/8) | 1813.8 | 432.3 | — |
| 5x16 | 96 | aeglos | 27.420 | +0.00% (0/8) | +2.32% (1/8) | 2248.1 | 2051.6 | — |
| merge-54x1 | 32 | prior | 143.212 | -58.80% (8/8) | +4.43% (1/8) | 3639.2 | 2078.7 | — |
| merge-54x1 | 32 | release | 137.389 | -60.61% (8/8) | +0.00% (0/8) | 3443.2 | 1966.4 | — |
| merge-54x1 | 32 | tuned | 135.149 | -61.02% (8/8) | -0.89% (5/8) | 3362.1 | 1957.2 | — |
| merge-54x1 | 32 | aeglos | 348.319 | +0.00% (0/8) | +153.86% (0/8) | 10673.7 | 12563.2 | — |
| merge-54x1 | 96 | prior | 123.793 | -28.22% (8/8) | +0.39% (4/8) | 6467.5 | 1933.2 | — |
| merge-54x1 | 96 | release | 122.857 | -28.93% (8/8) | +0.00% (0/8) | 6302.2 | 1929.8 | — |
| merge-54x1 | 96 | tuned | 103.392 | -40.24% (8/8) | -16.21% (8/8) | 5778.4 | 1949.2 | — |
| merge-54x1 | 96 | aeglos | 172.570 | +0.00% (0/8) | +40.70% (0/8) | 14899.5 | 13070.0 | — |

[Hardware](artifacts/hardware.txt) · [Pinned pilot](../2026-11-19-aws-wide663/README.md) · [Main measurements](artifacts/main-rows.json.gz) · [Verification](artifacts/main-verification.json)

## Cache placement at fixed width

Within each six-block arm, compact places 32 cores in four L3 domains, spread places 32 cores in twelve, and full uses all 96 cores in the twelve domains. CPU selection rotates by block. These are plain proof timings from the placement experiment. MSM is the engine-reported Aeglos stage.

| Shape | Engine | Compact / spread / full ms | Spread vs compact (wins) | Full vs spread (wins) | CPU-ms compact / spread / full | Peak MiB in same order | Aeglos MSM ms in same order |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x16 | aeglos | 19.966 / 21.104 / 12.369 | +3.77% (1/6) | -41.34% (6/6) | 569.935 / 569.070 / 879.948 | 765.8 / 761.7 / 821.5 | 15.177 / 15.988 / 7.398 |
| 1x16 | release | 17.345 / 18.136 / 19.722 | +4.22% (0/6) | +8.54% (0/6) | 401.753 / 421.187 / 1111.486 | 248.5 / 244.0 / 226.8 | — |
| 5x16 | aeglos | 47.619 / 49.652 / 27.834 | +3.17% (0/6) | -44.21% (6/6) | 1454.944 / 1467.556 / 2212.082 | 1906.2 / 1905.1 / 2052.6 | 36.948 / 38.097 / 19.529 |
| 5x16 | owned | 26.528 / 27.963 / 28.582 | +5.59% (0/6) | +2.68% (1/6) | 694.143 / 734.427 / 1796.531 | 413.2 / 407.6 / 410.6 | — |
| 5x16 | release | 27.572 / 28.818 / 27.303 | +4.87% (0/6) | -4.81% (6/6) | 730.448 / 770.633 / 1833.698 | 393.0 / 418.4 / 389.7 | — |

## Warm hardware counters

Each hardware-counter process has its own plain control at the same width and placement. The grouped user-mode counters cover fifteen warm requests. Instrumented versus plain is a paired latency comparison. Instructions per cycle is computed for each process before taking the median. These aggregate counters cannot individually identify cache stalls or worker synchronization.

| Shape | Engine | Placement | Plain ms | Instrumented vs plain | Instructions M/proof | Cycles M/proof | Instructions/cycle |
|---|---|---|---:|---:|---:|---:|---:|
| 1x16 | aeglos | compact | 19.863 | +1.63% | 3259.8 | 2052.5 | 1.588 |
| 1x16 | aeglos | full | 12.455 | +1.79% | 3593.1 | 2754.4 | 1.305 |
| 1x16 | aeglos | spread | 21.130 | +1.33% | 3259.1 | 2045.0 | 1.594 |
| 1x16 | release | compact | 17.335 | +7.66% | 2115.8 | 1343.2 | 1.574 |
| 1x16 | release | full | 19.569 | +9.85% | 2708.9 | 3657.6 | 0.740 |
| 1x16 | release | spread | 18.169 | +5.69% | 2106.1 | 1420.2 | 1.483 |
| 5x16 | aeglos | compact | 47.659 | +0.98% | 8409.3 | 5223.9 | 1.610 |
| 5x16 | aeglos | full | 27.478 | +1.37% | 8705.4 | 6354.7 | 1.370 |
| 5x16 | aeglos | spread | 49.972 | +2.84% | 8412.6 | 5245.1 | 1.604 |
| 5x16 | release | compact | 27.762 | +6.24% | 3617.0 | 2459.3 | 1.471 |
| 5x16 | release | full | 27.360 | +5.94% | 4388.3 | 5680.4 | 0.773 |
| 5x16 | release | spread | 28.797 | +6.08% | 3606.9 | 2626.4 | 1.374 |

At 1×16, moving from 32 compact cores to 96 increased Zopro’s retired instructions per proof by 28% and aggregate user cycles by 172%. Instructions per cycle fell from 1.574 to 0.740. Aeglos used 10% more instructions and 34% more cycles while its plain proof latency fell from 19.863 to 12.455 ms. The fixed-width placement test also slowed both engines when spreading 32 workers across twelve L3 domains. Excess CPU work and poorer execution efficiency at full width are measurable on this shape. The counters leave the division between synchronization, memory access and useful arithmetic open.

[Placement protocol](../2026-11-19-c7a-placement674/README.md) · [Counter protocol](../2026-11-19-c7a-warm-counters682/README.md) · [All 490 measurements](artifacts/rows.json.gz) · [Independent proof verification](artifacts/verification.json)
