# Four G2 point ranges on the accepted prover source

At 32 logical Ryzen workers, four G2 point ranges increased full-proof latency on both small shapes. The production-source change therefore proceeds to the C7a comparison only at the widths where the previous experimental prover had independently repeated its gain. The default stays on two ranges.

The candidate reuses the baseline workspace forecast. Before running 11-bit windows with four point ranges, it reserves the positive increase in parent storage plus any positive per-worker scratch increase. Admission failure falls back to 12-bit windows and two ranges. The tree option merges two pairs in parallel and then merges the two results. Neither variant retains per-key data.

| Shape | Mode | Median ms | Paired vs off | Wins | Median CPU ms | Peak MiB | M5 CPU / Metal speedup |
|---|---|---:|---:|---:|---:|---:|---:|
| 1x2 | accepted | 7.424 | -1.12% | 3/5 | 163 | 116 | 4.98× / 3.23× |
| 1x2 | off | 7.479 | +0.00% | 0/5 | 162 | 116 | 4.95× / 3.21× |
| 1x2 | split | 7.667 | +3.90% | 0/5 | 173 | 122 | 4.83× / 3.13× |
| 1x2 | tree | 7.638 | +2.62% | 0/5 | 172 | 114 | 4.84× / 3.14× |
| 2x2 | accepted | 10.438 | -0.40% | 4/5 | 238 | 285 | 5.65× / 2.87× |
| 2x2 | off | 10.448 | +0.00% | 0/5 | 240 | 289 | 5.65× / 2.87× |
| 2x2 | split | 10.693 | +1.65% | 0/5 | 249 | 290 | 5.52× / 2.81× |
| 2x2 | tree | 11.371 | +8.83% | 0/5 | 249 | 284 | 5.19× / 2.64× |

Each cell uses five fresh process means with 51 requests and three primes. All 40 process exports and seven separate route proofs passed external verification. Probes saw 48 fill callbacks with either enabled mode at 32 workers, and 22 when disabled, with a zero-byte optional allowance, or with four workers. Both geometries and merge orders matched a full-width generator-sum oracle. Generic and native-without-Horner libraries compiled.

[Source patch](artifacts/source.patch) · [Route counts](artifacts/routes.json) · [Raw timings](artifacts/rows.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
