# Packed FFT and frontend placement on six additional wide-output requests

Three fresh requests per shape used distinct blinding seeds and independently generated public inputs. The two catalog keys, executable and main 32-worker CPU placement match the earlier comparison. Every process warmed three proofs and timed eleven. Each 5×16 request used six alternating blocks. Each 1×16 request used eight balanced blocks. The 1×16 controls compare the ordinary pool, eight physical cores on one CCD, eight physical cores distributed across both CCDs, and all sixteen physical cores. Peaks are process RSS including setup. Each arm also ran a separate fifteen-request CPU profile. All 150 exported proofs verified independently.

| Witness | Configuration | Warm ms | Release / arm | Paired latency change (wins) | CPU-ms/proof | Peak MiB |
|---|---|---:|---:|---:|---:|---:|
| 5x16-1701 | release | 24.292 | 1.000× | +0.00% (0/6) | 662.7 | 429.5 |
| 5x16-1701 | packed | 23.571 | 1.031× | -2.17% (5/6) | 640.6 | 416.8 |
| 5x16-1702 | release | 23.984 | 1.000× | +0.00% (0/6) | 655.1 | 408.2 |
| 5x16-1702 | packed | 23.757 | 1.010× | -0.89% (4/6) | 650.4 | 413.0 |
| 5x16-1703 | release | 24.248 | 1.000× | +0.00% (0/6) | 663.7 | 415.5 |
| 5x16-1703 | packed | 23.680 | 1.024× | -2.00% (5/6) | 644.7 | 401.5 |
| 1x16-1701 | release | 12.993 | 1.000× | +0.00% (0/8) | 310.2 | 251.0 |
| 1x16-1701 | front8 | 13.082 | 0.993× | +0.79% (3/8) | 249.3 | 124.2 |
| 1x16-1701 | front8spread | 13.578 | 0.957× | +4.36% (0/8) | 253.5 | 125.5 |
| 1x16-1701 | front16 | 13.020 | 0.998× | +0.29% (2/8) | 269.1 | 127.0 |
| 1x16-1702 | release | 12.916 | 1.000× | +0.00% (0/8) | 310.1 | 244.2 |
| 1x16-1702 | front8 | 13.088 | 0.987× | +1.80% (1/8) | 251.4 | 129.8 |
| 1x16-1702 | front8spread | 13.571 | 0.952× | +4.91% (0/8) | 254.2 | 127.8 |
| 1x16-1702 | front16 | 13.059 | 0.989× | +1.33% (1/8) | 268.3 | 129.8 |
| 1x16-1703 | release | 13.083 | 1.000× | +0.00% (0/8) | 312.1 | 244.5 |
| 1x16-1703 | front8 | 13.185 | 0.992× | +1.06% (2/8) | 251.2 | 126.5 |
| 1x16-1703 | front8spread | 13.528 | 0.967× | +3.39% (0/8) | 253.0 | 125.8 |
| 1x16-1703 | front16 | 12.989 | 1.007× | -0.59% (6/8) | 269.1 | 132.8 |

For 5×16, select packed FFT on the main pool for the measured 32-thread Ryzen configuration. Set `ZOPRO_CPU_FFT_INPUT_COLUMNS=1` and `ZOPRO_CPU_FFT_LOCAL_EULER=1` together. It won 14/18 additional pairs and all eight earlier pairs and used less CPU time on every witness. One additional request increased observed peak RSS by 4.8 MiB. All four witnesses come from the same fixture family. Other machines and general workload distributions need separate measurement.

For memory-sensitive 1×16 runs on this host, eight frontend workers sharing one CCD used 124–130 MiB on the three additional witnesses, versus 244–251 MiB for the ordinary configuration. The paired latency cost was 0.79–1.80%. Splitting the eight workers across CCDs lost all 24 pairs against ordinary and took longer on every witness than keeping them on one CCD.

[Original two-shape experiment](../2026-11-19-wide-front656/README.md) · [Inputs](artifacts/public-fixtures.json.gz) · [Configurations](artifacts/configs.json.gz) · [Raw samples](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
