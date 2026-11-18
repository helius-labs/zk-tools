# Merging and weighting H buckets on C7a

All 256-bucket experimental tiles produce the same weighted sum. Staged and fused have identical arithmetic. Each cell uses five randomized blocks, or seven for 16×2 at 64.

| Shape | Cores | Release ms | Disabled ms | Staged ms | Fused ms | Paired fused vs staged | Faster blocks | Paired fused vs disabled | Fused peak MiB | Active processes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 64 | 33.851 | 33.866 | 34.142 | 33.799 | -1.95% | 4/7 | -0.49% | 801 | 7/7 |
| 16x2 | 96 | 37.234 | 35.849 | 36.086 | 36.207 | +1.13% | 2/5 | +0.49% | 781 | 5/5 |
| 24x2 | 64 | 46.140 | 46.195 | 46.294 | 48.383 | +4.42% | 1/5 | +4.74% | 1172 | 5/5 |
| 24x2 | 96 | 50.707 | 50.444 | 48.880 | 49.660 | +0.70% | 1/5 | -0.87% | 1172 | 5/5 |
| merge-24x1 | 64 | 49.050 | 49.581 | 48.305 | 49.022 | +1.48% | 2/5 | -1.07% | 961 | 5/5 |
| merge-24x1 | 96 | 50.045 | 50.102 | 50.567 | 51.427 | +1.89% | 1/5 | +3.41% | 987 | 5/5 |

[Local source and qualification](../2026-11-18-merge-weight362/README.md) · [Protocol](artifacts/tiles-protocol.md) · [Timings](artifacts/supplement.json.gz)
