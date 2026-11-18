# Dense H window widths and a 96-core G2 profile on C7a

At 13 bits, 24×2 improved in 8/8 pairs and reached its 52.8 ms target in 5/8. At 14 bits, merge24 improved in 7/8 pairs but remained above its 50.7 ms target. Neither width produced a consistent gain on 16×2.

The three large shapes used 48 physical C7a workers in six L3 groups. Each process proved the same circuit sequentially after three primes. The prototype selected 13- or 14-bit temporary buckets only for the large Montgomery H query. Existing workspace accounting includes the selected width.

| Shape | Mode | Median ms | Paired vs disabled | Wins / 8 | Paired vs accepted | Paired CPU vs disabled | Peak MiB | M5 CPU / Metal | Blocks at 10× CPU target |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 16x2 | accepted | 37.198 | +0.03% | 4 | +0.00% | +0.17% | 773 | 9.01× / 3.33× | 0/8 |
| 16x2 | off | 37.081 | +0.00% | — | -0.02% | +0.00% | 799 | 9.03× / 3.34× | 0/8 |
| 16x2 | 13 | 36.781 | -0.29% | 4 | -0.96% | -0.66% | 793 | 9.11× / 3.37× | 0/8 |
| 16x2 | 14 | 36.839 | +0.28% | 3 | -0.53% | -1.73% | 782 | 9.09× / 3.37× | 0/8 |
| merge-24x1 | accepted | 56.195 | -0.26% | 4 | +0.00% | +0.04% | 1115 | 9.02× / 3.11× | 0/8 |
| merge-24x1 | off | 56.347 | +0.00% | — | +0.26% | +0.00% | 1158 | 9.00× / 3.11× | 0/8 |
| merge-24x1 | 13 | 55.257 | -2.47% | 6 | -1.81% | -2.65% | 1126 | 9.18× / 3.17× | 0/8 |
| merge-24x1 | 14 | 54.634 | -2.83% | 7 | -2.17% | -2.89% | 1122 | 9.28× / 3.20× | 0/8 |
| 24x2 | accepted | 54.304 | -0.38% | 5 | +0.00% | -0.89% | 1293 | 9.72× / 3.24× | 0/8 |
| 24x2 | off | 54.353 | +0.00% | — | +0.38% | +0.00% | 1214 | 9.71× / 3.24× | 0/8 |
| 24x2 | 13 | 52.587 | -2.35% | 8 | -2.17% | -2.40% | 1255 | 10.04× / 3.35× | 5/8 |
| 24x2 | 14 | 52.875 | -2.05% | 7 | -2.10% | -2.42% | 1230 | 9.99× / 3.33× | 4/8 |

Median times use eight process means, each of 21 timed requests. Ratios compare arms on the same block and affinity. RSS is the maximum whole-process peak across the eight blocks and preflight. The primary endpoint was merge24. The latency screen requires a 2% paired median improvement, seven of eight wins and a lower median than exact accepted. [Local source and oracle](../2026-11-18-h-window285/README.md) · [Pairs](artifacts/cells.json) · [Process samples](artifacts/processes.json.gz) · [Protocol](artifacts/protocol.md)

All 108 large-shape exports and six 1×2 profiling exports passed independent Gnark verification both on AWS and after retrieval. The instance terminated. No matching EBS volume remained and the temporary S3 bucket was removed. [Receipt](artifacts/verification.json)

## Sampled cycles inside the 1×2 G2 bucket calls

The profiler increased median paired latency by 7.20%, and all three blocks missed the fast/slow distribution gate. The profile describes the instrumented workload. Causal attribution for the original fast proofs remains open.

The exact experimental prover from experiment 257 ran with 96 physical C7a cores and 32 frontend workers. Plain median latency was 10.211 ms, peak RSS was 70.1 MiB, and speedup was 3.62× the M5 CPU and 2.35× M5 Metal references.

| Requests | Count | Samples inside | Bucket batch arithmetic | Feed method | Field inversion function | Epoch bookkeeping inside | Other samples during fills | Epoch bookkeeping among those other samples |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| all | 453 | 49,905 | 67.1% | 14.0% | 3.8% | 1.2% | 126,431 | 39.1% |
| fast | 208 | 22,778 | 67.5% | 13.4% | 3.8% | 1.2% | 66,194 | 30.4% |
| slow | 168 | 18,689 | 66.3% | 15.7% | 3.7% | 1.2% | 35,219 | 60.9% |

The profiler recorded one sample per 2,000,003 user-mode CPU cycles and entry and return events for the SDK and the G2 bucket callback on each worker. An inside sample belongs to a thread with at least one callback open. Other samples during fills come from a thread with no open callback while another thread has one. The categories in the table identify the function containing the sampled instruction. Inlined callees count toward the containing symbol. The bookkeeping column includes symbols containing crossbeam_epoch. Samples measure on-CPU work and exclude descheduled time.

The median paired trace/plain latency ratio was 1.0720 (individual blocks 1.0646, 1.0720, 1.1148). The observer missed its calibration criteria, so the exact sampled workload limits interpretation. Fast requests finished in under 10 ms and slow requests took at least 11 ms. The proportions of fast and slow requests stayed within 10 percentage points of plain in 0 of three blocks. Each traced request included all 22 G2 callback pairs. The decoder checked worker affinity, proof ordinals and zero reported lost or throttled records. Every exported proof passed independent Gnark verification.

[Function samples, timing and proof receipts](artifacts/fill-analysis.json.gz) · [Instruction sample counts](artifacts/fill-pcs.json.gz) · [Per-request symbol counts](artifacts/fill-profiles.json.gz) · [Profile protocol](artifacts/fill-protocol.md)
