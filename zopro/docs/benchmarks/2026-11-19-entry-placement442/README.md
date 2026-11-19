# Explicit frontend placement in the public SDK benchmark

Seven randomized blocks measured the original 2×2 request on Ryzen. Both frontends used eight physical cores in one L3, alternating L3 groups by block. The twelve-worker setting also used four SMT siblings. The main Rayon pool had 32 pinned workers. Wall time is the median of seven process means, each over 51 verified SDK calls. Peak RSS is the highest process reading for that configuration.

| Driver | 8 workers ms | 12 workers ms | Median paired change | Faster blocks | CPU-ms, 8 / 12 | Peak MiB, 8 / 12 |
|---|---:|---:|---:|---:|---:|---:|
| example | 9.924 | 9.842 | -0.30% | 4/7 | 195.2 / 199.9 | 117.0 / 127.2 |
| reference | 10.089 | 9.907 | -1.70% | 7/7 | 201.3 / 203.3 | 119.2 / 128.0 |

The public example reached 5.99× the supplied M5 CPU speed and 3.05× the M5 Metal speed. The benchmark reported the requested CPU assignments for both worker pools. A separate run exercised the default placement and a startup check rejected a CPU outside the process affinity mask. All 29 exported proofs passed independent Gnark verification.

[Process samples](artifacts/processes.json.gz) · [Build identities](artifacts/build.json) · [Verification](artifacts/verification.json) · [Protocol](artifacts/protocol.md) · [Eight varied inputs](../2026-11-19-varied-smt440/README.md)
