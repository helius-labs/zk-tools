# Computing G2 split scalars during witness gathering

Moving the endomorphism split to the B-scalar gather increased local 1×2 latency by 6.05% at 16 Ryzen threads and 2.20% at 32. All six paired blocks favored the original schedule.

The gather writes the split scalar directly into its existing four-word output slot. The diversity check reads original canonical values from the retained wire array using validated B indices. A successful certificate lets G2 consume the split representation directly. If the certificate is denied, the code restores the original B scalars in place and uses the existing grouping path. The benchmark does not allocate a second scalar array.

| Threads | Exact 257 ms | OFF ms | ON ms | Paired change | CPU OFF / ON ms | Peak OFF / ON MiB | M5 CPU / Metal on ON |
|---:|---:|---:|---:|---:|---:|---:|---:|
| 16 | 8.623 | 8.533 | 9.049 | +6.05% | 105 / 105 | 64 / 56 | 4.09× / 2.65× |
| 32 | 7.446 | 7.411 | 7.589 | +2.20% | 166 / 163 | 62 / 62 | 4.88× / 3.16× |

The switch engaged on all measured 1×2 proofs in the ON arm. Both 2×2 settings followed the original route. Seven forced-restoration 1×2 requests exercised canonical fallback after pre-splitting, and the exported proof verified. The old split-during-FFT switch was disabled in every process. These are three local blocks per setting and width, with 51 timed requests per 1×2 process plus three primes. All 21 process exports passed Gnark. The native tests and a generic core check passed. [Paired results](artifacts/cells.json) · [Source delta from experiment 264](artifacts/changes-from-264.patch.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
