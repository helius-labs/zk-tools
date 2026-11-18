# Clean signed H options on Ryzen

The candidate selects two memory-admitted signed scalar layouts through `ZOPRO_CPU_DENSE_H_WINDOWS=17s` or `18s`. Its unsigned-19 case uses the existing arithmetic. The signed modes compute digits from the input scalar without a key table. Two restricted-memory 16×2 calls independently verified at zero optional preparation allowance. Each timing is the median of five fresh process means.

| Shape | Signed layout | Previous ms | Unsigned ms | Prototype ms | Clean ms | Paired vs previous | Faster blocks | Peak MiB |
|---|---|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | 17s | 45.505 | 45.388 | 46.684 | 46.118 | +2.17% | 0/5 | 622 |
| 16x2 | 18s | 45.505 | 45.388 | 45.056 | 45.397 | +0.79% | 2/5 | 612 |
| 24x2 | 17s | 73.429 | 73.954 | 75.037 | 75.618 | +2.98% | 2/5 | 982 |
| 24x2 | 18s | 73.429 | 73.954 | 74.182 | 72.779 | -0.72% | 3/5 | 984 |
| merge-24x1 | 17s | 71.251 | 71.327 | 71.877 | 71.991 | +0.75% | 1/5 | 894 |
| merge-24x1 | 18s | 71.251 | 71.327 | 71.401 | 70.930 | -0.68% | 4/5 | 888 |

[Protocol](artifacts/protocol.md) · [Source](artifacts/source.patch.gz) · [Process timings](artifacts/local.json.gz)

[Exact C7a scorecard and paired controls](aws.md)
