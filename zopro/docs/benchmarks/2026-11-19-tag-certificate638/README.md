# One-pass fingerprints in verified small-input proofs

Across four traced calls on the original 1×2 request the release reached the 21,006-entry dictionary reservation 4 times, the two-pass certificate 0 times and the one-pass fingerprints 0 times. The maximum temporary certificate allocation is 32 KiB in all three builds. All 157 native core tests passed on the fingerprint build, and all 108 exported proofs verified independently.

| Shape and input | Release / two-pass / tags ms | Tags vs release | Tags vs two-pass | Peak release / two-pass / tags MiB | Tags vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 1x2-0 | 7.013 / 6.997 / 7.122 | +1.05% (1/6) | +1.98% (0/6) | 61.8 / 59.0 / 59.2 | 5.19× / 3.37× |
| 1x2-29 | 6.981 / 6.939 / 7.064 | +0.61% (2/6) | +2.02% (1/6) | 59.5 / 59.8 / 59.0 | — |
| 1x2-32 | 6.993 / 6.958 / 7.036 | +0.54% (2/6) | +1.12% (1/6) | 60.8 / 59.8 / 58.8 | — |
| 2x2-0 | 9.595 / 9.574 / 9.660 | +0.77% (2/6) | +0.98% (2/6) | 111.5 / 111.8 / 109.2 | 6.11× / 3.11× |
| 4x2-0 | 15.314 / 15.191 / 15.344 | -0.27% (3/6) | +1.19% (1/6) | 278.2 / 275.5 / 270.8 | 7.10× / 3.13× |

[Protocol](protocol.md) · [Source](artifacts/source.patch) · [Samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
