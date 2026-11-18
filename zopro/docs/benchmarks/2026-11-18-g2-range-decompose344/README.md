# Independent G2 scalar decomposition by point range

The experimental path splits each range of scalar words in place. It can start work on the first ready range while other ranges are being decomposed. Moving bucket-vector headers restores the existing order of the final reduction. The four-range configuration reserves an extra 4 KiB for headers. Measurements used five blocks on 32 local Ryzen threads and eight frontend workers.

| Shape | Mode | Time ms | Paired vs ordinary | Faster blocks | Peak MiB |
|---|---|---:|---:|---:|---:|
| 1x2 | retained | 7.844 | +0.84% | 2/5 | 68 |
| 1x2 | off | 7.856 | +0.00% | 0/5 | 65 |
| 1x2 | serial | 7.968 | +1.58% | 1/5 | 65 |
| 1x2 | range | 7.938 | +1.82% | 1/5 | 66 |
| 2x2 | retained | 11.012 | -0.02% | 3/5 | 112 |
| 2x2 | off | 10.950 | +0.00% | 0/5 | 110 |
| 2x2 | serial | 11.027 | +0.62% | 0/5 | 113 |
| 2x2 | range | 11.095 | +0.67% | 0/5 | 110 |

All 45 exports verified. The enabled path ran once and invoked 48 G2 range fills. Zero optional memory selected the ordinary two-range path with 22 fills. The trace also counted 40 G1 input-feed calls per eligible proof.

[Protocol](artifacts/protocol.md) · [Samples](artifacts/processes.json.gz) · [Routing](artifacts/routing.json) · [Verification](artifacts/verification.json)
