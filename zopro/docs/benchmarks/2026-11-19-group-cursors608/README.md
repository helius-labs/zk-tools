# Reusing the final class offsets as insertion cursors

The allocation volume column sums the temporary payload eliminated over one proof. Peak RSS is measured independently over each process. Latencies are medians of seven process means.

| Transfer | Bytes eliminated per proof | Prior / control / candidate ms | Paired candidate vs control | Peak control / candidate MiB | Candidate vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2 | 0 | 7.106 / 7.032 / 7.105 | +0.49% (1/7) | 64.5 / 66.8 | 5.21× / 3.38× |
| 2x2 | 581,448 | 9.506 / 9.499 / 9.442 | +0.69% (3/7) | 126.0 / 117.0 | 6.25× / 3.18× |
| 4x2 | 652,280 | 15.324 / 15.294 / 15.435 | -0.90% (4/7) | 323.2 / 323.0 | 7.06× / 3.11× |
| 5x16 | 1,105,864 | 24.343 / 24.391 / 24.403 | +0.09% (3/7) | 440.5 / 451.8 | — |
| 6x2 | 754,592 | 21.336 / 21.199 / 21.422 | +0.64% (2/7) | 391.8 / 386.0 | — |

All 110 exports verified independently. [Protocol](protocol.md) · [Source](artifacts/source.patch) · [Samples and class counts](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
