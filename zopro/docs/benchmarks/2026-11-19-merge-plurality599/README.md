# Single-key merge proofs with a public plurality reference

Each SDK process makes fifteen timed, verified proofs after three warmups. Seven randomized blocks compare three images on 32 pinned Ryzen workers. Both preconverted images use the flat row format. Two inputs (125 and 128) were absent from the earlier diagnostic. The independent verifier used the ordinary key and expected public inputs for every export.

| Request | Ordinary / modal / plurality ms | Plurality vs modal | Plurality vs ordinary | Max modal / plurality MiB | Plurality vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| Original | 63.675 / 61.275 / 61.107 | -0.53% (5/7) | -3.62% (7/7) | 1088.2 / 1073.3 | 8.30× / 2.86× |
| 121 | 66.777 / 65.616 / 65.521 | +0.12% (3/7) | -2.29% (6/7) | 1215.6 / 1168.8 | — |
| 124 | 69.003 / 68.085 / 67.976 | -0.21% (5/7) | -1.87% (4/7) | 1178.6 / 1179.8 | — |
| 125 | 69.261 / 69.484 / 69.633 | +0.25% (3/7) | +0.46% (2/7) | 1195.0 / 1180.5 | — |
| 128 | 70.091 / 71.074 / 71.594 | +0.55% (3/7) | +1.63% (0/7) | 1222.9 / 1275.2 | — |

The single plurality dictionary adds 2,606,554 bytes on disk and retains 3,799,486 bytes of decoded pair and row data. The converter reported 363043 ms and 1131.0 MiB whole-process peak RSS. It added zero private slots. All 106 exports passed. [Schedule](protocol.md) · [Samples](artifacts/samples.json.gz) · [Key and conversion records](artifacts/keys.json) · [Verification](artifacts/verification.json)

The offline conversion overlapped a concurrent local compilation, an image hash audit and short route probes. Its elapsed setup time includes that interference. These activities all finished before the first timed comparison.
