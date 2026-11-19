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
