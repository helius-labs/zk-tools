# Scalar repeat distances across eight verified proof shapes

This build applies only the grouping change to the accepted production source. The original classifier is selected on ABIs where the dictionary-entry size or alignment would change. The B2 timing instrumentation is absent. Each of five fresh randomized blocks compares the release with the disabled and enabled settings of one experimental executable on eight original requests and five additional inputs. Each process warms three proofs and times 31 requests on the two smallest shapes or eleven elsewhere. Peak RSS includes setup. The public offset files and worker settings match across each comparison.

| Shape | Request | Release / ordinary / candidate ms | Paired vs ordinary | Paired vs release | Peak MiB ordinary / candidate | Candidate vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | Original | 7.101 / 7.056 / 7.095 | +0.81% (1/5) | -0.11% (3/5) | 62.8 / 64.2 | 5.22× / 3.38× |
| 2x2 | Original | 9.698 / 9.622 / 9.658 | +0.89% (1/5) | +0.02% (2/5) | 117.8 / 119.2 | 6.11× / 3.11× |
| 4x2 | Original | 15.513 / 15.637 / 15.432 | -0.59% (4/5) | -0.51% (3/5) | 255.8 / 280.2 | 7.06× / 3.11× |
| 8x2 | Original | 23.815 / 24.287 / 24.222 | -0.31% (3/5) | +1.68% (0/5) | 440.0 / 438.0 | 7.80× / 3.06× |
| 12x2 | Original | 32.232 / 31.171 / 31.217 | +0.48% (1/5) | -2.13% (3/5) | 552.8 / 556.5 | 9.10× / 3.24× |
| 16x2 | Original | 39.368 / 40.426 / 39.036 | -2.95% (5/5) | -0.39% (4/5) | 650.3 / 665.2 | 8.58× / 3.18× |
| 24x2 | Original | 63.897 / 64.622 / 64.123 | +0.12% (2/5) | -0.21% (3/5) | 1020.2 / 1030.9 | 8.23× / 2.74× |
| merge-24x1 | Original | 63.505 / 63.443 / 63.599 | -0.57% (4/5) | +0.15% (2/5) | 1020.0 / 1050.6 | 7.97× / 2.75× |
| 2x2 | 40 | 9.613 / 9.562 / 9.619 | +0.18% (2/5) | -0.08% (3/5) | 116.0 / 114.5 | — |
| 12x2 | 142 | 47.909 / 46.726 / 47.893 | +2.50% (1/5) | -0.82% (3/5) | 898.9 / 915.1 | — |
| 16x2 | 72 | 40.348 / 40.076 / 39.676 | -1.02% (3/5) | -1.67% (5/5) | 650.3 / 670.0 | — |
| 24x2 | 103 | 64.084 / 63.821 / 63.190 | -0.97% (3/5) | -1.46% (3/5) | 1040.3 / 1029.6 | — |
| merge-24x1 | 124 | 70.647 / 69.384 / 68.960 | -0.29% (3/5) | -2.39% (5/5) | 1132.5 / 1112.0 | — |

All 195 exports verified against the original public inputs. [Samples](artifacts/samples.json.gz) · [Grouping source](artifacts/source.patch) · [Build hashes](artifacts/build.json) · [Verification](artifacts/verification.json)
