# Qualification of fused FFT input packing

The clean native implementation uses 64 columns per task when ZOPRO_CPU_FFT_INPUT_COLUMNS=1 on Euler domains of at least 8,192 points. The default uses the existing inverse transform. No per-key data are added. The module uses a single packed buffer and per-task scratch. Byte-identical probe runs observed one function entry per timed enabled proof and none when disabled. The core test suite ran with the new option enabled. Seven fresh matched blocks compare the enabled build, the identical build with the option off, and the exact preceding release. Each process timed 31 complete verified requests.

| Workers | Mode | Median ms | CPU-ms | Peak MiB | M5 CPU / Metal |
|---:|---|---:|---:|---:|---:|
| 16 | release | 38.421 | 565.1 | 551.2 | 7.39× / 2.63× |
| 16 | off | 38.207 | 563.0 | 554.0 | 7.43× / 2.64× |
| 16 | 1 | 38.093 | 559.8 | 563.5 | 7.46× / 2.65× |
| 32 | release | 32.924 | 915.0 | 721.5 | 8.63× / 3.07× |
| 32 | off | 32.935 | 913.6 | 704.0 | 8.62× / 3.07× |
| 32 | 1 | 32.560 | 906.6 | 709.2 | 8.72× / 3.10× |

| Workers | Paired vs release | Wins / 7 | Paired vs disabled | Wins / 7 |
|---:|---:|---:|---:|---:|
| 16 | -0.96% | 6 | -1.26% | 5 |
| 32 | -1.04% | 5 | -0.48% | 4 |

The disabled executable differed from the preceding release by +0.25% at 16 workers and -0.01% at 32, using median paired latency.

The 32-thread candidate did not meet both prespecified comparisons.

## Shape coverage

One fresh process per row with eleven timed requests. These are coverage checks on the candidate executable. They do not establish per-shape gains.

| Shape | ms | Peak MiB | M5 CPU / Metal ms | CPU / Metal speedup |
|---|---:|---:|---:|---:|
| 1x2 | 7.512 | 60.2 | 37.0 / 24.0 | 4.93× / 3.19× |
| 2x2 | 10.113 | 102.8 | 59.0 / 30.0 | 5.83× / 2.97× |
| 4x2 | 15.423 | 257.0 | 109.0 / 48.0 | 7.07× / 3.11× |
| 8x2 | 24.003 | 414.0 | 189.0 / 74.0 | 7.87× / 3.08× |
| 12x2 | 32.579 | 560.8 | 284.0 / 101.0 | 8.72× / 3.10× |
| 16x2 | 40.304 | 698.8 | 335.0 / 124.0 | 8.31× / 3.08× |
| 24x2 | 69.352 | 1103.6 | 528.0 / 176.0 | 7.61× / 2.54× |
| merge-24x1 | 66.011 | 1026.2 | 507.0 / 175.0 | 7.68× / 2.65× |

[Candidate source](artifacts/source.patch) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json) · [Process samples](artifacts/processes.json.gz) · [Decision](artifacts/decision.json)
