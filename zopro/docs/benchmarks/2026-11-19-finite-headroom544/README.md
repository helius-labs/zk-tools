# Reading cgroup usage only for finite memory limits

For a cgroup v2 ancestor with unlimited memory.max and memory.high, current usage cannot impose a finite headroom bound. The candidate reads the limits first and skips memory.current in that case. Finite limits and usage are reread at each admission, and the ancestor walk still checks all levels. A test exercises newly finite high and max limits, changing usage, headroom saturation and a cgroup v1 hierarchy.

Separate traced processes include loading, three warmups and one timed proof. The counts include attempted opens at the mount root. Each pair gives the parent and candidate counts. Timing below comes from separate untraced processes.

| Shape | memory.max opens | memory.high opens | memory.current opens |
|---|---:|---:|---:|
| 1x2 | 190 / 190 | 152 / 152 | 190 / 0 |
| 12x2 | 490 / 490 | 392 / 392 | 490 / 0 |
| 24x2 | 400 / 400 | 320 / 320 | 400 / 0 |

Five randomized blocks used 32 Ryzen workers and the same public request in each pair. The 12×2 and 24×2 cohorts used the same prepared groups and direct nested scalar view. All other shapes used their original profiles.

| Shape | Request | Parent / candidate ms | Paired change | Peak MiB parent / candidate | Candidate vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|
| 1x2 | Original | 7.223 / 7.162 | -0.49% (3/5) | 64.0 / 65.8 | 5.17× / 3.35× |
| 2x2 | Original | 9.647 / 9.588 | +0.15% (2/5) | 113.8 / 115.2 | 6.15× / 3.13× |
| 4x2 | Original | 15.506 / 15.794 | +2.07% (1/5) | 259.8 / 270.5 | 6.90× / 3.04× |
| 8x2 | Original | 24.190 / 23.911 | -0.90% (3/5) | 434.5 / 432.5 | 7.90× / 3.09× |
| 12x2 | Original | 29.706 / 29.323 | -1.83% (4/5) | 542.0 / 547.3 | 9.69× / 3.44× |
| 16x2 | Original | 39.451 / 39.535 | +0.51% (2/5) | 657.3 / 653.2 | 8.47× / 3.14× |
| 24x2 | Original | 60.378 / 59.728 | -0.21% (3/5) | 1105.9 / 1120.7 | 8.84× / 2.95× |
| merge-24x1 | Original | 63.756 / 63.393 | -0.32% (3/5) | 1027.2 / 1004.0 | 8.00× / 2.76× |
| 12x2 | 142 | 45.964 / 45.169 | -1.45% (3/5) | 970.2 / 987.1 | — |
| 24x2 | 103 | 57.755 / 60.009 | +1.19% (2/5) | 1120.1 / 1154.9 | — |

All 106 exports verified independently. [Timings](artifacts/timings.json.gz) · [Syscall counts](artifacts/traces.json.gz) · [Source and resource test](artifacts/source.patch) · [Verification](artifacts/verification.json)
