# Independent 16-core G2 range confirmation

The same binaries and keys from the initial experiment ran on one hardware thread per physical Ryzen core. Twelve balanced blocks measured the original and eight each measured the additional two inputs. Each process timed 21 verified SDK calls. All 124 exports, including the twelve separate CPU profile exports, verified independently.

The 9/16 split repeated the original improvement. It won 11/12 against equal and 10/12 against release, with peak RSS of 56.8 MiB versus 57.2 and 57.5 MiB. All three inputs had lower paired medians against equal. The two additional witnesses each split 4/8 against release, with paired differences of −0.06% and −0.02%. Select 9/16 for the measured experimental 16-core 1×2 configuration. Evidence for a production-wide policy still requires a clean implementation and wider qualification. [Exact configurations](artifacts/configs.json.gz) · [Tested implementation](../2026-11-19-range-share648/artifacts/source.patch)

| Input | Release / equal / 7 / 9 sixteenths ms | 9 vs 7 / equal / release | Peaks in same order, MiB | 9/16 vs M5 CPU / Metal |
|---|---:|---:|---:|---:|
| 1x2-0-w16 | 8.237 / 8.297 / 8.358 / 8.198 | -1.96% (11/12) / -0.80% (11/12) / -0.55% (10/12) | 57.5 / 57.2 / 58.0 / 56.8 | 4.51× / 2.93× |
| 1x2-29-w16 | 8.094 / 8.119 / 8.161 / 8.089 | -0.86% (7/8) / -0.23% (4/8) / -0.06% (4/8) | 56.0 / 55.8 / 55.8 / 55.8 | — |
| 1x2-32-w16 | 8.055 / 8.111 / 8.181 / 8.045 | -1.66% (8/8) / -0.78% (7/8) / -0.02% (4/8) | 55.0 / 55.8 / 55.5 / 55.5 | — |

[Protocol](protocol.md) · [Paired comparisons](artifacts/summary.json) · [Samples and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
