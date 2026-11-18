# Signed mixed-width H windows on Ryzen

Signed-18 covers the 254-bit scalar and one zero sign bit with three 15-bit and fifteen 14-bit Booth digits. In each of two point ranges it uses 172,032 bucket slots across the windows, compared with 212,992 for unsigned-19. Signed-17 uses 278,528. Each digit is extracted directly from the scalar. Both experimental paths reserve optional workspace before allocating buckets.

| Shape | Signed windows | Release ms | Disabled ms | Signed ms | Paired change vs disabled | Faster blocks | Peak MiB | Active processes |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 12x2 | 17 | 40.267 | 39.256 | 38.997 | -0.73% | 4/5 | 533 | 5/5 |
| 12x2 | 18 | 40.267 | 39.256 | 39.778 | +2.22% | 2/5 | 530 | 5/5 |
| 16x2 | 17 | 45.943 | 46.027 | 46.038 | +0.02% | 2/5 | 630 | 5/5 |
| 16x2 | 18 | 45.943 | 46.027 | 45.136 | -1.94% | 5/5 | 596 | 5/5 |
| 24x2 | 17 | 74.494 | 73.989 | 73.157 | -1.12% | 4/5 | 974 | 5/5 |
| 24x2 | 18 | 74.494 | 73.989 | 73.650 | -0.46% | 4/5 | 994 | 5/5 |
| merge-24x1 | 17 | 72.110 | 71.858 | 72.304 | +0.36% | 2/5 | 883 | 5/5 |
| merge-24x1 | 18 | 72.110 | 71.858 | 71.814 | -0.32% | 3/5 | 893 | 5/5 |

Both 16×2 zero-allowance processes returned verified proofs without entering the signed H kernel. [Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz) · [Measurements](artifacts/local.json.gz)

[Matched C7a results](aws.md)

[Fresh paired confirmation](confirmation.md)
