# Combined H bucket pass across eight merge witnesses

Eight valid merge-24×1 requests were generated with two real coins and 22 dummy slots each. Changing both coin amounts and blindings changed the UTXO tree roots and public inputs. Each request used the same merge key. Four randomized blocks compared the original release, the identical candidate with the option disabled and the candidate with the combined pass enabled. Each process timed eleven full SDK proofs after three warmups on 32 Ryzen hardware threads.

| Witness | Release / disabled / enabled ms | Paired vs release | Paired vs disabled | Peak MiB disabled / enabled |
|---|---:|---:|---:|---:|
| 1 | 61.283 / 62.126 / 61.824 | +0.31% | -1.05% | 979.8 / 1005.5 |
| 2 | 62.205 / 61.936 / 61.413 | -1.02% | -0.66% | 1019.9 / 979.0 |
| 3 | 61.383 / 62.532 / 61.727 | +0.13% | -1.38% | 1005.8 / 976.4 |
| 4 | 61.402 / 62.757 / 61.263 | -0.20% | -2.70% | 1009.5 / 1013.6 |
| 5 | 61.668 / 61.646 / 62.573 | +1.84% | +1.43% | 1013.7 / 1006.8 |
| 6 | 61.701 / 61.652 / 61.772 | -0.02% | +0.06% | 1001.0 / 998.8 |
| 7 | 61.789 / 61.736 / 62.290 | +1.26% | +1.57% | 1004.5 / 1035.5 |
| 8 | 62.386 / 62.190 / 61.675 | -0.86% | -1.23% | 994.2 / 996.8 |

The median paired change across witnesses was +0.06% against release (4/8 faster) and -0.85% against disabled (5/8 faster). All 96 exports passed independent Gnark verification.

The earlier gain on the original request did not generalize against the release. The existing two-pass implementation stays in the prover.

[Process samples](artifacts/processes.json.gz) · [Inputs](artifacts/inputs.json) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md) · [Original merge request](../2026-11-19-h-chunk-merge447/README.md)
