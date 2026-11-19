# Caller-allocated folded FFT scratch in the ordinary Linux SDK

The ordinary key and its public wire offsets are identical in all five arms. The parked control attaches the same pool without dispatching the product. Each time is the median of seven complete process means. Peak RSS includes setup and all requests.

| 24×2 request | Release / fresh / off / parked / pooled ms | Pooled vs parked / off | Peak release / fresh / off / parked / pooled MiB | Pooled CPU-ms / minor faults per proof | Pooled vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 63.938 / 63.695 / 63.167 / 64.035 / 63.144 | -1.39% (5/7) / -0.23% (4/7) | 1027.5 / 1058.7 / 1035.4 / 1036.9 / 891.2 | 1798.6 / 3,186 | 8.36× / 2.79× |
| 97 | 63.455 / 63.566 / 63.895 / 64.148 / 63.677 | -0.99% (7/7) / -0.49% (5/7) | 1070.5 / 1033.5 / 1040.6 / 1038.2 / 894.5 | 1798.8 / 3,659 | — |
| 99 | 63.629 / 62.999 / 63.038 / 64.428 / 62.546 | -3.30% (6/7) / -1.32% (6/7) | 1028.6 / 1025.2 / 1032.9 / 1026.3 / 938.7 | 1782.1 / 3,855 | — |
| 101 | 63.205 / 63.500 / 63.161 / 64.053 / 63.350 | -1.78% (5/7) / +0.21% (3/7) | 1049.2 / 1011.4 / 1003.3 / 1025.4 / 931.2 | 1790.6 / 4,145 | — |

All 143 exports verified. [Protocol](protocol.md) · [Release-source patch](artifacts/source.patch) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
