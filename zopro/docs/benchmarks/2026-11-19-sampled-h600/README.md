# Sampled H routing on 1×2 and 2×2

Five process means per setting, each covering 31 warmed verified proofs at 32 pinned workers. The final column reports both supplied M5 ratios on the reference requests.

| Shape | Input | Ordinary / ordinary sample / blocked / blocked sample / prior blocked raw ms | Blocked sample vs blocked / ordinary | Max blocked / sampled MiB | M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.078 / 7.096 / 7.042 / 7.065 / 7.049 | +0.19% (2/5) / +0.26% (2/5) | 65.5 / 64.5 | 5.24× / 3.40× |
| 1x2 | 27 | 7.005 / 6.964 / 6.921 / 6.908 / 6.927 | +0.40% (2/5) / -0.83% (5/5) | 65.0 / 62.8 | — |
| 1x2 | 32 | 6.976 / 6.993 / 6.909 / 6.940 / 6.939 | +0.91% (1/5) / -0.25% (4/5) | 64.8 / 62.0 | — |
| 2x2 | Original | 9.665 / 9.635 / 10.585 / 9.460 / 9.455 | -10.47% (4/5) / -2.03% (4/5) | 126.5 / 122.0 | 6.24× / 3.17× |
| 2x2 | 39 | 9.666 / 9.591 / 10.622 / 9.500 / 9.451 | -11.01% (5/5) / -2.57% (5/5) | 129.2 / 111.2 | — |
| 2x2 | 44 | 9.641 / 9.583 / 10.542 / 9.466 / 9.392 | -10.48% (5/5) / -1.66% (5/5) | 122.8 / 117.5 | — |
| 2x2 | 48 | 9.680 / 9.638 / 10.501 / 9.503 / 9.488 | -9.20% (4/5) / -1.61% (3/5) | 130.0 / 122.2 | — |

All 175 exports verified with the original image. [Protocol](protocol.md) · [Samples](artifacts/samples.json.gz) · [Source and tests](artifacts/source.patch) · [Identities](artifacts/pins.json) · [Verification](artifacts/verification.json)
