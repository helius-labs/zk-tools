# Independent C7a confirmation of wider H buckets and lighter G2 sampling

On the second host, 24×2 measured 53.073 ms against 52.8 ms required, with three of eight blocks meeting target. The first host measured 52.587 ms and five of eight. The 13-bit setting beat its disabled control in 14 of 16 pairs across the two hosts. Its relative benefit was 2.35% and 1.89% respectively. The 14-bit merge24 gain fell from 2.83% on the first host to 0.79% here. Neither met the prespecified independent confirmation criteria. [First C7a host](../2026-11-18-h-profile287/README.md)

The two prespecified shapes used 48 physical C7a workers in six L3 groups. Each process proved the same circuit sequentially after three primes. The prototype selected 13- or 14-bit temporary buckets only for the large Montgomery H query. Existing workspace accounting includes the selected width.

| Shape | Mode | Median ms | Paired vs disabled | Wins / 8 | Paired vs accepted | Paired CPU vs disabled | Peak MiB | M5 CPU / Metal | Blocks at 10× CPU target |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 24x2 | accepted | 54.675 | -0.59% | 5 | +0.00% | -0.47% | 1283 | 9.66× / 3.22× | 1/8 |
| 24x2 | off | 53.499 | +0.00% | — | +0.59% | +0.00% | 1271 | 9.87× / 3.29× | 4/8 |
| 24x2 | on | 53.073 | -1.89% | 6 | -3.08% | -2.34% | 1327 | 9.95× / 3.32× | 3/8 |
| merge-24x1 | accepted | 55.742 | -0.83% | 5 | +0.00% | +0.28% | 1161 | 9.10× / 3.14× | 0/8 |
| merge-24x1 | off | 55.467 | +0.00% | — | +0.84% | +0.00% | 1182 | 9.14× / 3.16× | 0/8 |
| merge-24x1 | on | 55.646 | -0.79% | 5 | -1.79% | -0.33% | 1206 | 9.11× / 3.14× | 0/8 |

Median times use eight process means, each of 31 timed requests. Ratios compare arms on the same block and affinity. RSS is the maximum whole-process peak across the eight blocks and preflight. The primary absolute endpoint was 24×2 at 13 bits. Merge24 used 14 bits. Confirmation of 24×2 required a median at or below 52.8 ms and at least five qualifying blocks. The latency screen requires a 2% paired median improvement, seven of eight wins and a lower median than exact accepted. [Local source and oracle](../2026-11-18-h-window285/README.md) · [Pairs](artifacts/cells.json) · [Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md)

All 54 large-shape exports and six 1×2 profiling exports passed independent Gnark verification both on AWS and after retrieval. The instance terminated. No matching EBS volume remained and the temporary S3 bucket was removed. [Receipt](artifacts/verification.json)

## Sampled cycles inside the 1×2 G2 bucket calls

The lower sampling frequency met the 5% wall-time tolerance, with a 3.65% median increase. Only one of three paired blocks preserved both fast and slow proportions within ten percentage points. The agreement in arithmetic and bookkeeping shares between the two profiled hosts is descriptive. The data cannot assign the gap to Aeglos to either source.

The exact experimental prover from experiment 257 ran with 96 physical C7a cores and 32 frontend workers. Plain median latency was 10.106 ms, peak RSS was 64.0 MiB, and speedup was 3.66× the M5 CPU and 2.37× M5 Metal references.

| Requests | Count | Samples inside | Bucket batch arithmetic | Feed method | Field inversion function | Epoch bookkeeping inside | Other samples during fills | Epoch bookkeeping among those other samples |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| all | 453 | 6,261 | 67.2% | 14.5% | 3.5% | 0.9% | 16,836 | 41.6% |
| fast | 292 | 4,060 | 67.4% | 14.4% | 3.5% | 0.7% | 12,190 | 33.5% |
| slow | 147 | 2,028 | 66.8% | 14.7% | 3.6% | 1.2% | 4,125 | 66.3% |

The profiler recorded one sample per 16,000,019 user-mode CPU cycles and entry and return events for the SDK and the G2 bucket callback on each worker. An inside sample belongs to a thread with at least one callback open. Other samples during fills come from a thread with no open callback while another thread has one. The categories in the table identify the function containing the sampled instruction. Inlined callees count toward the containing symbol. The bookkeeping column includes symbols containing crossbeam_epoch. Samples measure on-CPU work and exclude descheduled time.

The median paired trace/plain latency ratio was 1.0365 (individual blocks 0.9952, 1.0365, 1.0822). The observer missed its calibration criteria, so the exact sampled workload limits interpretation. Fast requests finished in under 10 ms and slow requests took at least 11 ms. The proportions of fast and slow requests stayed within 10 percentage points of plain in 1 of three blocks. Each traced request included all 22 G2 callback pairs. The decoder checked worker affinity, proof ordinals and zero reported lost or throttled records. Every exported proof passed independent Gnark verification.

[Function samples, timing and proof receipts](artifacts/fill-analysis.json.gz) · [Instruction sample counts](artifacts/fill-pcs.json.gz) · [Per-request symbol counts](artifacts/fill-profiles.json.gz) · [Profile protocol](artifacts/fill-protocol.md)
