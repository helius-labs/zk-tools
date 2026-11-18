# Solver task claims on physical cores and SMT siblings

Nine randomized blocks of 61 verified 1×2 requests on a Ryzen 9950X. The two 16-worker settings have the same frontend size and worker count. Process and worker CPU masks were verified before and after measurement. Each process exported a fresh proof for independent Gnark verification.

| Worker placement | Release ms | Disabled ms | Claims ms | Paired vs release | Wins / 9 | Paired vs disabled | Wins / 9 | CPU-ms disabled / claims | Peak MiB release / claims | M5 CPU / Metal gain |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| 8 physical | 14.000 | 13.985 | 13.670 | -2.41% | 9 | -2.28% | 9 | 91.8 / 92.3 | 48 / 50 | 2.71× / 1.76× |
| 16 physical | 8.786 | 8.751 | 8.532 | -3.03% | 9 | -3.12% | 9 | 100.8 / 100.9 | 60 / 60 | 4.34× / 2.81× |
| 16 on 8 SMT cores | 11.016 | 11.085 | 10.784 | -2.68% | 9 | -2.65% | 9 | 131.1 / 131.6 | 60 / 59 | 3.43× / 2.23× |
| 32 on 16 SMT cores | 7.656 | 7.656 | 7.369 | -3.60% | 9 | -3.94% | 9 | 151.4 / 151.6 | 66 / 68 | 5.02× / 3.26× |

The shared task cursor improved 1×2 wall time in all 36 matched blocks against each control, including both placements that used one worker per physical core. On the 32-worker layout, CPU time per proof stayed close to 151 ms even as wall time fell. The local result therefore supports a Ryzen latency improvement across these placements. The separate C7a test found no repeatable improvement at 96 physical cores.

[Raw process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md)
