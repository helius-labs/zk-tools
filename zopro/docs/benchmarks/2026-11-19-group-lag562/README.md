# Using an exact repeat-distance check during scalar grouping

The candidate learns one recent distance between equal nonzero scalars in the ordinary grouping loop. An exact four-limb comparison can reuse the class at that earlier row. Each miss follows the hash-table path. The extra row index fits in the existing table-entry padding on this native target. Compile-time checks enforce both size and alignment. The reference classes, counts and scalar order matched a separate implementation in the native test on zero, repeated, high-diversity and interrupted patterns. Earlier tests also checked each class on 48 captured requests.

Five randomized blocks used the original requests, two worker counts and three warmups plus eleven timed requests per process. Both experimental settings share one binary. The B2 direct-call option from the readiness experiment stays disabled in all arms. Two separate trace arms record the same query and B2 preparation intervals. Peak RSS includes loading and preparation. All 100 process exports verified independently.

| Shape | Ryzen workers | Release / ordinary / candidate ms | Candidate paired vs ordinary | Candidate paired vs release | Candidate vs M5 CPU / Metal | Ordinary / candidate peak MiB |
|---|---:|---:|---:|---:|---:|---:|
| 16x2 | 16 | 43.759 / 44.402 / 43.817 | -1.32% (5/5) | +0.04% | 7.65× / 2.83× | 576.2 / 572.8 |
| 16x2 | 32 | 39.879 / 39.474 / 39.930 | +0.04% (2/5) | +0.19% | 8.39× / 3.11× | 647.5 / 664.3 |
| merge-24x1 | 16 | 68.037 / 67.851 / 68.633 | +1.28% (0/5) | +0.80% | 7.39× / 2.55× | 871.9 / 884.2 |
| merge-24x1 | 32 | 63.771 / 63.042 / 64.073 | +1.16% (1/5) | +0.75% | 7.91× / 2.73× | 1051.5 / 1040.5 |

| Shape | Ryzen workers | Schedule | Paired trace overhead | Median ms to B entry / preparing scalars / ready to B2 entry / inside B2 | B2 entry after H return |
|---|---:|---|---:|---:|---:|
| 16x2 | 16 | control | -2.32% | 0.0010 / 5.4713 / 0.0007 / 20.8820 | 0/55 |
| 16x2 | 16 | lag | -0.21% | 0.0012 / 6.4924 / 0.0009 / 19.9921 | 0/55 |
| 16x2 | 32 | control | +0.48% | 0.0012 / 10.3490 / 0.0013 / 11.1862 | 0/55 |
| 16x2 | 32 | lag | +0.78% | 0.0012 / 10.1922 / 0.0013 / 11.9332 | 0/55 |
| merge-24x1 | 16 | control | +0.57% | 0.0039 / 8.9063 / 0.0009 / 27.3067 | 0/55 |
| merge-24x1 | 16 | lag | -0.48% | 0.0039 / 9.4358 / 0.0009 / 26.7574 | 0/55 |
| merge-24x1 | 32 | control | +0.48% | 0.0032 / 17.6244 / 0.0011 / 12.9903 | 0/55 |
| merge-24x1 | 32 | lag | -1.04% | 0.0023 / 15.9755 / 0.0014 / 14.5757 | 0/55 |

The timestamps bracket the B2 call, which can prepare additional data internally before reaching any sampled arithmetic symbol. The archived C7a observation counts samples in one arithmetic function. Both measurements may depend on the order in which workers acquire tasks. The bundled runner uses 64 and 96 physical cores by default and checks the topology. The results above use the local Ryzen mode.

[Samples](artifacts/samples.json.gz) · [Source change](artifacts/source.patch) · [Host runner](artifacts/run.py) · [Binary and input hashes](artifacts/config.json) · [Proof verification](artifacts/verification.json)
