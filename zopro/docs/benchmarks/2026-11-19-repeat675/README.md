# Independent repeats of the two observed local improvements

Eight fresh counterbalanced blocks tested each input. The worker-ordering candidate uses real L3 domains on 6×2 and the held-out 5×16 witness. The G2 candidate writes the sparse correction list into disjoint output ranges on 8×2 and 16×2. Frozen is the respective original executable. Off and candidate share an executable. Each process primes three calls and times thirteen single proofs. Separate CPU profiles use seventeen. All 108 exports passed independent verification. The 5×16 held-out repeat also checks the earlier block that overlapped a short compiler invocation.

| Shape or witness | Candidate | Frozen / off / candidate ms | Paired vs off (wins) | Paired vs frozen (wins) | Off / candidate CPU-ms | Off / candidate peak MiB | Candidate vs M5 CPU / Metal |
|---|---|---:|---:|---:|---:|---:|---:|
| 5x16-1702 | local | 22.849 / 23.032 / 22.822 | -0.98% (5/8) | +0.29% (4/8) | 619.4 / 616.7 | 443.2 / 449.5 | — |
| 6x2 | local | 21.363 / 20.767 / 20.729 | -0.19% (5/8) | -2.40% (7/8) | 569.2 / 567.7 | 373.8 / 371.5 | — |
| 8x2 | g2 | 22.934 / 23.274 / 22.941 | -1.62% (5/8) | -0.02% (4/8) | 617.4 / 616.8 | 522.0 / 525.8 | 8.24× / 3.23× |
| 16x2 | g2 | 37.077 / 37.519 / 37.139 | -0.74% (5/8) | +0.24% (3/8) | 1052.5 / 1052.0 | 791.4 / 752.4 | 9.02× / 3.34× |

Against the original executable the G2 collector split 4/8 on 8×2 and lost 5/8 on 16×2, despite beating the new binary's disabled mode in 5/8 blocks on each. The earlier 16×2 gain against frozen did not repeat. The cache-local queue order also split 4/8 against frozen on the held-out 5×16 request. On 6×2 it beat frozen in 7/8 pairs, yet its same-binary advantage was only 0.19%. Recompilation accounts for much of the difference to frozen on that input. Keep both experimental features outside release defaults. Neither reached a twofold improvement on a measured input.

[Worker ordering](../2026-11-19-local-victims673/README.md) · [G2 correction collection](../2026-11-19-parallel-corrections669/README.md) · [Configurations](artifacts/configs.json.gz) · [Proof timings and profiles](artifacts/rows.json.gz) · [Verification](artifacts/verification.json)
