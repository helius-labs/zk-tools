# Local Euler weighting on three larger Ryzen circuits

The accepted binary ran with 32 pinned logical workers on the Ryzen 9950X. Both modes pack the first inverse columns from the inputs. Enabling local weighting performs the inner inverse transform, Euler weighting and inner forward transform in the same expanded leaf. Each original request ran in seven randomized matched blocks, with three primes and eleven timed verified SDK requests per process. Separate debugger probes observed four entries into the local function per enabled process and zero with the option disabled.

| Shape | Ordinary / local ms | Paired change (wins) | CPU-ms ordinary / local | Peak MiB ordinary / local | Local speedup vs M5 CPU / Metal |
|---|---:|---:|---:|---:|---:|
| 16x2 | 40.579 / 39.620 | -1.90% (4/7) | 1172.9 / 1141.1 | 763.0 / 734.2 | 8.46× / 3.13× |
| 24x2 | 67.338 / 65.824 | -2.81% (7/7) | 1966.9 / 1903.5 | 1112.2 / 1129.2 | 8.02× / 2.67× |
| merge-24x1 | 65.062 / 63.511 | -2.69% (5/7) | 1859.5 / 1817.7 | 1059.9 / 1023.2 | 7.98× / 2.76× |

A favorable paired median and at least five wins nominate a shape for independent qualification on additional witnesses. This is an original-request screen. The all-eight [accepted scorecard](../2026-11-19-integrated-wire467/README.md) uses the existing profiles. All 48 exports passed independent Gnark verification.

[Samples](artifacts/processes.json.gz) · [Debugger probes](artifacts/probe.json) · [Qualification candidates](artifacts/decision.json) · [Verification](artifacts/verification.json)
