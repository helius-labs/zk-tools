# Verified 1×2 latency across input coins and Merkle trees

Both provers used the same 32 hardware threads on a 16-core Ryzen 9950X and processed the same requests. Aeglos used 32 native and Go workers with warm cache hits. Zopro used 32 main workers plus eight frontend workers on that CPU mask. The disabled arm turns off the G2 scalar-diversity shortcut in the same Zopro executable. Each cell is the median of three process means, with three warmups and 31 timed verified requests per process. Witness 0 is the original fixture. The next eight vary the input coin, public commitments, state leaf and populated state and nullifier trees.

| Witness | Zopro ms | Disabled ms | Aeglos ms | Zopro vs disabled, paired | Zopro vs Aeglos, paired | Peak MiB Zopro / Aeglos |
|---:|---:|---:|---:|---:|---:|---:|
| 0 | 7.239 | 7.386 | 12.879 | -1.99% | -43.80% | 62.8 / 429.2 |
| 1 | 7.120 | 7.219 | 12.916 | -1.91% | -45.36% | 62.8 / 422.0 |
| 2 | 7.092 | 7.216 | 12.826 | -1.72% | -44.69% | 62.8 / 425.9 |
| 3 | 7.086 | 7.256 | 12.828 | -1.73% | -44.51% | 62.5 / 422.9 |
| 4 | 7.155 | 7.261 | 12.797 | -1.45% | -44.46% | 62.0 / 424.5 |
| 5 | 7.093 | 7.233 | 12.826 | -2.07% | -44.70% | 62.2 / 423.2 |
| 6 | 7.081 | 7.302 | 12.825 | -3.07% | -44.79% | 65.0 / 432.4 |
| 7 | 7.047 | 7.247 | 12.912 | -3.43% | -45.43% | 63.0 / 431.1 |
| 8 | 7.125 | 7.269 | 12.823 | -1.91% | -44.43% | 65.0 / 426.6 |

| Eight new witnesses | Median ms across witnesses | Median CPU-ms | Maximum peak MiB |
|---|---:|---:|---:|
| Zopro | 7.092 | 146.9 | 65.0 |
| no-diversity | 7.251 | 144.2 | 71.2 |
| Aeglos | 12.826 | 374.8 | 432.4 |

Against the same build with the diversity shortcut disabled, Zopro had lower paired latency on 8/8 new witnesses. The median of the eight paired changes was -1.91%.

Against Aeglos, Zopro had lower paired latency on 8/8 new witnesses. The median of the eight paired changes was -44.69%.

The supplied M5 references are for the original case. These new inputs have no separately measured Apple reference.

[Protocol](artifacts/protocol.md) · [Process samples](artifacts/processes.json.gz) · [Verification](artifacts/verification.json) · [Input generation and G2 measurements](../2026-11-19-g2-tree-variation418/README.md)
