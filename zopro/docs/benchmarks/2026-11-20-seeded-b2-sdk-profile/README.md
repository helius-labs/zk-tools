# Seeded tasks and B2 vector conversion in the CPU SDK

The 8×2 server profile uses seeded solver task claims and eight-lane conversion of distinct, grouped B2 scalars. Enable seeded claims with `ZOPRO_SOLVE_TASK_CLAIMS=seed` and vector conversion with `ZOPRO_CPU_GROUPED_B2_VECTOR=1`. Vector conversion requires serial grouped B2 preparation and a native AVX-512 IFMA build. The measured profile also enables encoded Poseidon emission and uses 72 main and 24 frontend C7a cores. The full configuration and worker placement are recorded in the confirmation workflow.

The prototype and the SDK confirmation independently verified 136 and 144 proof exports. The original input cleared 18.90 ms in four of six prototype blocks and five of eight SDK blocks. The SDK won 26 of 28 pairs against the released profile. Its held-out comparison with the same-build control split four wins and four losses, with a paired median 0.25% lower. Including the separate prototype run, the combined profile had lower paired medians and a majority of wins against the respective ordinary controls on all four inputs. This evidence supports offering it as a server opt-in. [Prototype](../2026-11-20-seed-vector-b2-c7a850/README.md) · [SDK confirmation and full configuration](../2026-11-20-seed-vector-b2-sdk-c7a852/README.md)

| Input | Prototype ms | SDK ms | Aeglos ms | SDK wins vs release | SDK wins vs ordinary | Both runs vs ordinary | SDK / Aeglos peak MiB |
|---|---:|---:|---:|---:|---:|---:|---:|
| catalog | 18.808 | 18.849 | 30.917 | 8/8 | 5/8 | 11/14 | 358.2 / 2488.5 |
| heldout | 19.083 | 19.046 | 30.628 | 8/8 | 4/8 | 9/14 | 361.2 / 2502.2 |
| v153 | 19.171 | 19.093 | 30.649 | 5/6 | 4/6 | 7/10 | 375.5 / 2465.9 |
| v156 | 19.059 | 19.090 | 30.643 | 5/6 | 5/6 | 9/10 | 357.4 / 2489.8 |

## Eight supplied M5 reference timings

**5/8** original requests reach 10× M5 CPU using the recorded per-shape profiles.

| Shape | Profile | C7a ms | M5 CPU / Metal ms | CPU / Metal speedup | Gap to 10× CPU ms |
|---|---|---:|---:|---:|---:|
| 1×2 | 805 released SDK 80+16 | 7.858 | 37 / 24 | 4.71× / 3.05× | +4.158 |
| 2×2 | 805 released SDK medium K/H 64+32 | 12.029 | 59 / 30 | 4.90× / 2.49× | +6.129 |
| 4×2 | 828 frontend solver full | 15.103 | 109 / 48 | 7.22× / 3.18× | +4.203 |
| 8×2 | 852 SDK seeded tasks and B2 vector conversion | 18.849 | 189 / 74 | 10.03× / 3.93× | -0.051 |
| 12×2 | 750 indexed fixed-on | 27.790 | 284 / 101 | 10.22× / 3.63× | -0.610 |
| 16×2 | 747 recorded SDK 64+32 | 30.884 | 335 / 124 | 10.85× / 4.01× | -2.616 |
| 24×2 | 707 catalog | 47.046 | 528 / 176 | 11.22× / 3.74× | -5.754 |
| merge-24×1 | 747 opt-in fusion 64+32 | 45.724 | 507 / 175 | 11.09× / 3.83× | -4.976 |
