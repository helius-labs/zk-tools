# Confirming a lower-memory FFT placement on additional witnesses

On 24×2, the 30-worker caller-allocated product reduced peak process RSS on all four inputs and improved paired median latency against the main pool by 0.07% to 3.52%. It won 15 of the 20 block pairs. On the reference it took 58.840 ms with a 968.7 MiB peak. A main-pool proof took 59.549 ms with a 1,149.0 MiB peak. Input 99 ran faster even though its minor faults rose. At 12×2 the selected width also reduced peak RSS on every input. Its latency improved only on the scattered witness. The 24×2 configuration is the selected experimental Ryzen server profile for the release-source comparison. [Release integration trial](../2026-11-19-release-pool621/protocol.md)

The selected worker count is fixed per shape before these five confirmation blocks. The three non-reference requests in each shape were excluded from width selection. The paired percentages compare the selected configuration with ordinary main and earlier auxiliary dispatch.

| Shape | Request | Product workers | Main / previous dispatch / selected ms | Selected vs main / previous | Peak main / selected MiB | Minor faults/proof main / selected | Selected vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| 12x2 | Original | 24 | 28.887 / 28.717 / 28.909 | +1.35% (2/5) / +1.60% (1/5) | 581.8 / 457.1 | 3,252 / 927 | 9.82× / 3.49× |
| 12x2 | 58 | 24 | 29.074 / 28.612 / 29.231 | +0.54% (1/5) / +1.83% (1/5) | 586.2 / 465.9 | 3,222 / 1,137 | — |
| 12x2 | 61 | 24 | 28.832 / 28.781 / 29.368 | +1.86% (2/5) / +2.86% (1/5) | 586.1 / 448.5 | 3,623 / 1,181 | — |
| 12x2 | 139 | 24 | 45.766 / 44.485 / 44.579 | -2.57% (4/5) / +0.21% (2/5) | 1065.6 / 595.9 | 8,634 / 1,386 | — |
| 24x2 | Original | 30 | 59.549 / 61.547 / 58.840 | -0.91% (3/5) / -4.93% (4/5) | 1149.0 / 968.7 | 5,972 / 3,479 | 8.97× / 2.99× |
| 24x2 | 97 | 30 | 58.875 / 60.574 / 58.041 | -2.10% (4/5) / -4.68% (5/5) | 1148.6 / 934.4 | 5,711 / 2,444 | — |
| 24x2 | 99 | 30 | 60.065 / 60.829 / 58.220 | -3.52% (4/5) / -4.95% (5/5) | 1154.7 / 930.5 | 5,811 / 6,620 | — |
| 24x2 | 101 | 30 | 58.611 / 61.957 / 58.503 | -0.07% (3/5) / -4.39% (5/5) | 1139.2 / 941.9 | 10,350 / 4,060 | — |

All 174 exported proofs verified. [Protocol](protocol.md) · [Width selection](artifacts/selection.json) · [Samples](artifacts/samples.json.gz) · [Verification](artifacts/verification.json)
