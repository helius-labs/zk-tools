# Merging and weighting H buckets in one task

The experimental path gives each ready 256-bucket tile to a worker for both point merging and weighted reduction. The staged control uses the same arithmetic with a join between phases. The current release uses paired weighted trees. The oracle test caught missing identity padding on a partial final tile in the initial prototype. The corrected build passed the full native suite before SDK timing. The table shows medians of five process means and changes paired by process block. Each process times 15 full SDK proofs.

| Shape | Release ms | Disabled ms | Staged ms | Fused ms | Paired fused vs staged | Faster blocks | Paired fused vs release | Fused peak MiB | Active fused processes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 12x2 | 38.999 | 39.079 | 39.070 | 39.162 | +0.02% | 2/5 | +0.32% | 517 | 5/5 |
| 16x2 | 46.459 | 45.658 | 46.161 | 45.632 | -0.34% | 4/5 | -0.98% | 633 | 5/5 |
| 24x2 | 73.251 | 73.306 | 75.013 | 74.359 | -0.32% | 3/5 | +1.37% | 985 | 5/5 |
| merge-24x1 | 71.595 | 71.559 | 71.847 | 72.031 | -0.65% | 3/5 | +0.16% | 919 | 5/5 |

[Protocol](artifacts/protocol.md) · [Source patch](artifacts/source.patch.gz) · [Samples](artifacts/timings.json.gz) · [Verification](artifacts/verification.json)
