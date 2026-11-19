# How far can choosing a worker count take the historical C7a prover?

The first three blocks of the same-instance C7a catalog select either 32 or 96 physical cores for each Zopro shape by median warm loop time. The selected width is fixed for the remaining six blocks. Each evaluation compares the selected process with Zopro and prepared Aeglos at 96 cores in that same block. Ratios are medians of paired ratios. The 20 shapes below were identified as historical 96-core losses using the original nine blocks. The width holdout tests a choice on repeated executions of the same key and witness. The loss classification itself used all blocks. All 41 shapes and 1,476 timed process records are included in the supporting data.

The width rule chose 32 cores on 20/20 historical losses. It gave lower median latency than 96-core Aeglos on 2/20. Its largest median improvement over Zopro at 96 was 1.21×. It reached half the 96-core Zopro latency in 0/120 held-out shape/block pairs. These are retrospective timings on the archived executable.

The last columns test a distinct counterfactual. Let W32 and W96 be measured process CPU-ms per proof at the two widths and T96 the 96-core latency. If a new implementation could execute the same aggregate CPU work as the 32-core run, the mean occupied cores required to halve T96 would be 2 W32 / T96. The observed occupancy is W96 / T96. User and system CPU time include stalls, worker searches and synchronization. The projection assumes the smaller-width work can be preserved while parallelizing and gives no estimate of serial dependencies or of the cost of doing so. Every sample covers seven warm proofs.

| Historical 96-core loss | Selected cores | Held-out speedup vs Zopro 96 | Selected time / Aeglos 96 | CPU-work ratio 96/32 | Observed / required busy cores at 32-core work |
|---|---:|---:|---:|---:|---:|
| 1x2 | 32 | 1.170× | 1.288× | 3.05× | 55.1 / 36.3 |
| 1x4 | 32 | 1.210× | 1.328× | 3.29× | 57.5 / 34.9 |
| 1x8 | 32 | 1.207× | 1.362× | 3.16× | 59.8 / 37.9 |
| 1x16 | 32 | 1.161× | 1.451× | 3.05× | 58.6 / 38.7 |
| 2x2 | 32 | 1.173× | 1.231× | 3.11× | 57.7 / 37.2 |
| 2x4 | 32 | 1.151× | 1.233× | 3.05× | 57.7 / 37.9 |
| 2x8 | 32 | 1.154× | 1.159× | 2.95× | 61.7 / 41.8 |
| 2x16 | 32 | 1.108× | 1.277× | 2.98× | 65.2 / 43.8 |
| 3x2 | 32 | 1.091× | 1.115× | 3.00× | 67.3 / 45.0 |
| 3x4 | 32 | 1.118× | 1.132× | 2.98× | 65.7 / 44.1 |
| 3x8 | 32 | 1.086× | 1.148× | 2.94× | 67.4 / 46.2 |
| 4x2 | 32 | 1.107× | 1.072× | 3.02× | 69.2 / 46.0 |
| 4x4 | 32 | 1.107× | 1.081× | 2.94× | 67.4 / 45.7 |
| 4x8 | 32 | 1.101× | 1.120× | 2.95× | 68.0 / 46.7 |
| 4x16 | 32 | 1.070× | 1.135× | 2.83× | 67.9 / 48.2 |
| 5x2 | 32 | 1.099× | 1.041× | 3.04× | 71.7 / 47.0 |
| 5x4 | 32 | 1.106× | 1.030× | 3.01× | 70.2 / 46.8 |
| 5x8 | 32 | 1.101× | 0.945× | 2.98× | 71.6 / 48.0 |
| 5x16 | 32 | 1.069× | 1.011× | 2.84× | 70.3 / 49.7 |
| merge-8x1 | 32 | 1.118× | 0.919× | 3.12× | 74.1 / 47.6 |

Preserving 32-core work would put the required average occupancy between 34.9 and 49.7 cores across these twenty shapes by median. On 20/20, even the historically observed average occupancy would suffice in that counterfactual. The engineering question is how much of the extra CPU work is removable while preserving concurrency. A same-host experiment can distinguish that effect by recording both time and CPU-ms for each full proof and pairing uninstrumented latency with separate phase and worker profiles. The source records cover a single machine and predate the current candidate.

[Original host and verification](../2026-11-18-full-catalog-c7a244/README.md) · [Prior constant-work bound](../2026-11-19-c7a-work655/README.md) · [All block measurements and projections](artifacts/pairs.json.gz) · [Input hashes and all shape summaries](artifacts/summary.json)
