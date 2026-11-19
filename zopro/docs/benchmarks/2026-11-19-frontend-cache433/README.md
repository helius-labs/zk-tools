# Frontend worker placement with a fixed 32-thread main pool

The main prover used all 32 logical CPUs of a Ryzen 9950X in every process. Eight frontend workers shared those logical CPUs, each on a distinct physical core. Compact masks used all eight cores of the first or second L3. Spread masks used four from each. The driver checked every frontend worker’s pin and verified the main pool’s affinity before and after timing. Eight shuffled blocks per request and placement each timed 51 proofs after three warmups. The release used its original driver and first-L3 placement. The other four settings shared one executable linked against the same prover libraries.

| Shape | Placement | Median ms | CPU-ms | Peak MiB |
|---|---|---:|---:|---:|
| 1x2 | release | 7.288 | 151.7 | 83.0 |
| 1x2 | compact0 | 7.259 | 151.3 | 65.8 |
| 1x2 | compact1 | 7.190 | 150.1 | 88.8 |
| 1x2 | spread0 | 7.380 | 151.5 | 95.5 |
| 1x2 | spread1 | 7.315 | 151.9 | 80.2 |
| 2x2 | release | 10.164 | 202.5 | 115.2 |
| 2x2 | compact0 | 10.165 | 203.6 | 118.8 |
| 2x2 | compact1 | 10.161 | 203.0 | 122.5 |
| 2x2 | spread0 | 10.505 | 205.6 | 115.0 |
| 2x2 | spread1 | 10.472 | 205.0 | 118.5 |

| Shape | Spread vs compact, paired | Spread wins / 8 | Paired change each half (wins / 8) | Compact0 vs release |
|---|---:|---:|---|---:|
| 1x2 | +1.58% | 1 | +1.27% (0), +1.77% (2) | -0.31% |
| 2x2 | +3.52% | 0 | +3.98% (0), +3.07% (0) | -0.16% |

Keeping the eight frontend workers on a single L3 gave lower combined latency on seven of eight 1×2 blocks and all eight 2×2 blocks. The existing placement already uses a single L3 on this host.

Each paired change compares process means from the same block. The combined comparison averages the two complementary masks before taking the ratio. The measurements apply to this Ryzen topology.

[CPU masks](artifacts/topology.json) · [Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md) · [Verification](artifacts/verification.json)
