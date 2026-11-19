# Small-shape performance with both FFT options disabled

The comparison uses the two exact SDK binaries from scorecard 428. Each shape has twelve paired blocks at 32 Ryzen hardware threads. Each process timed 51 verified SDK requests after three warmups. Six blocks run the new binary first and six run it second. The two options stayed disabled for all proofs.

| Shape | Previous / current ms | Paired change | Slower pairs / 12 | Current first / second paired change | Peak MiB previous / current | Regression criterion met |
|---|---:|---:|---:|---:|---:|---|
| 1x2 | 7.289 / 7.259 | -0.15% | 6 | -0.41% / +0.18% | 66.0 / 66.2 | no |
| 2x2 | 10.092 / 10.140 | +0.44% | 7 | +0.81% / -0.13% | 120.0 / 119.0 | no |
| 4x2 | 15.244 / 15.447 | +1.13% | 10 | +1.57% / +0.47% | 326.5 / 335.8 | yes |

[Protocol](artifacts/protocol.md) · [Block order](artifacts/order.json) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [All eight shapes](../2026-11-19-qualified-score428/README.md)
