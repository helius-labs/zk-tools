# Overlapping verification work on C7a

Both timing and peak RSS include complete SDK proofs. The two small-shape configurations use the release settings selected before this trial. Every exported result was verified by Gnark on C7a and on the local host.

| Shape | Cores | Release ms | Disabled ms | Late ms | Early ms | Paired early vs release | Faster blocks | Early CPU-ms | Early peak MiB | M5 CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 64 | 8.141 | 8.248 | 8.190 | 8.149 | +0.17% | 3/7 | 277.8 | 53 | 4.54× / 2.95× |
| 1x2 | 96 | 8.268 | 8.250 | 8.221 | 8.176 | -0.97% | 6/7 | 386.4 | 51 | 4.53× / 2.94× |
| 2x2 | 48 | 13.085 | 13.036 | 13.018 | 12.963 | -1.08% | 3/5 | 345.4 | 97 | 4.55× / 2.31× |
| 16x2 | 64 | 34.590 | 33.729 | 33.632 | 37.554 | +9.22% | 0/5 | 1699.8 | 756 | 8.92× / 3.30× |

On 1×2 at 96 physical cores Aeglos measured 8.188 ms and 472 MiB peak. Zopro early measured 8.176 ms and 51 MiB. Early was +0.38% versus Aeglos in paired blocks, with 3/7 lower process means.

[Protocol](artifacts/aws-protocol.md) · [Process samples](artifacts/aws.json.gz)
