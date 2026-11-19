# Two-pass diversity certification at a fixed bitmap size

On four executions of the 1×2 reference, the release reached the 21,006-entry dictionary reservation 4 times and the candidate reached it 0 times. Both use a maximum 32 KiB bitmap allocation. The second scan distinguishes an additional hash bit. The candidate passed all 157 native core tests and all 92 exported proofs verified independently.

| Shape and input | Release / candidate ms | Paired change and wins | Peak release / candidate MiB | Candidate vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2-0 | 6.983 / 7.016 | +0.47% (4/8) | 61.0 / 58.8 | 5.27× / 3.42× |
| 1x2-29 | 6.958 / 6.900 | -0.54% (6/8) | 58.0 / 58.8 | — |
| 1x2-32 | 6.941 / 6.940 | -0.54% (5/8) | 60.5 / 58.0 | — |
| 2x2-0 | 9.629 / 9.594 | -0.14% (4/8) | 111.0 / 104.8 | 6.15× / 3.13× |
| 4x2-0 | 15.122 / 15.294 | +0.91% (2/8) | 262.5 / 257.8 | 7.13× / 3.14× |

[Protocol](protocol.md) · [Source](artifacts/source.patch) · [Samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
