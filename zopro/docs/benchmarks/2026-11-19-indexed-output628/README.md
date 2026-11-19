# Joint indexed grouping and tiled FFT paths on two transfer sizes

For experimental 24×2 on this Ryzen host, the indexed dictionary with tiled output on the smaller pool won **20/20 pairs versus ordinary** and **16/20 versus the existing tiled, pooled path**. On the original input its median was 56.639 ms versus 57.477 ms for the existing path. Peak RSS fell from 960.1 to 925.4 MiB. The original remained 3.839 ms above the 10× M5 CPU cutoff. The selected experimental latency profile uses all three changes. On input 99 the paired latency change against the existing path was +0.13%. On input 97 peak RSS rose by 19.9 MiB.

On 12×2 the combined path reduced maximum RSS on all four inputs. The original took 28.785 ms versus 28.523 ms for tiled output alone. On the scattered request the combined path improved latency in every pair, from 45.203 to 43.424 ms, and reduced peak RSS from 970.5 to 714.1 MiB. This panel did not produce a new 10× reference result.

The enhanced path uses tiled output on 12×2 and adds a caller-allocated smaller product pool on 24×2. Each reported latency is the median process mean, with within-block changes for both enhancements versus the frozen enhanced executable. RSS covers the full process.

| Shape | Request | Ordinary / indexed / enhanced / both ms | Both vs enhanced | Peaks in same order, MiB | Both vs M5 CPU / Metal (10× CPU blocks) |
|---|---|---:|---:|---:|---:|
| 12x2 | Original | 28.555 / 28.759 / 28.523 / 28.785 | +1.30% (3/8) | 579.4 / 551.2 / 575.3 / 547.3 | 9.87× / 3.51× (0/8) |
| 12x2 | 58 | 29.015 / 28.612 / 28.732 / 28.811 | +0.29% (1/4) | 589.0 / 543.6 / 573.3 / 529.7 | — |
| 12x2 | 61 | 28.679 / 28.696 / 28.655 / 28.480 | -0.37% (2/4) | 583.3 / 551.8 / 561.3 / 538.0 | — |
| 12x2 | 139 | 45.453 / 43.501 / 45.203 / 43.424 | -4.10% (4/4) | 1031.6 / 706.7 / 970.5 / 714.1 | — |
| 24x2 | Original | 59.319 / 58.236 / 57.477 / 56.639 | -1.82% (7/8) | 1133.4 / 1070.9 / 960.1 / 925.4 | 9.32× / 3.11× (0/8) |
| 24x2 | 97 | 57.798 / 57.361 / 56.326 / 55.083 | -2.43% (3/4) | 1119.5 / 1045.5 / 891.9 / 911.8 | — |
| 24x2 | 99 | 58.595 / 57.464 / 56.403 / 56.239 | +0.13% (2/4) | 1102.8 / 1049.2 / 929.4 / 897.3 | — |
| 24x2 | 101 | 58.774 / 56.579 / 56.421 / 55.246 | -1.76% (4/4) | 1119.0 / 1039.9 / 918.1 / 855.4 | — |

All 192 exports independently verified. [Protocol](protocol.md) · [Samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
