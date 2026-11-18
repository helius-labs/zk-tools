# Sampled cycles inside G2 bucket calls on Ryzen

The exact experimental prover from experiment 257 ran with 32 pinned Ryzen hardware threads across 16 physical cores and eight frontend workers. Plain median latency was 7.537 ms, peak RSS was 66.2 MiB, and speedup was 4.91× the M5 CPU and 3.18× M5 Metal references.

| Requests | Count | Samples inside | Bucket batch arithmetic | Feed method | Field inversion function | Epoch bookkeeping inside | Other samples during fills | Epoch bookkeeping among those other samples |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| all | 93 | 10,140 | 66.8% | 10.8% | 5.1% | 0.3% | 13,886 | 8.5% |
| fast | 93 | 10,140 | 66.8% | 10.8% | 5.1% | 0.3% | 13,886 | 8.5% |

The profiler recorded one sample per 2,000,003 user-mode CPU cycles and entry and return events for the SDK and the G2 bucket callback on each worker. An inside sample belongs to a thread with at least one callback open. Other samples during fills come from a thread with no open callback while another thread has one. The categories in the table identify the function containing the sampled instruction. Inlined callees count toward the containing symbol. The bookkeeping column includes symbols containing crossbeam_epoch. Samples measure on-CPU work and exclude descheduled time.

The median paired trace/plain latency ratio was 1.0008 (individual blocks 1.0008, 1.0029, 0.9990). The latency calibration passed. Fast requests finished in under 10 ms and slow requests took at least 11 ms. All 93 main traced local requests were fast. Each traced request included all 22 G2 callback pairs. The decoder checked worker affinity, proof ordinals and zero reported lost or throttled records. Every exported proof passed independent Gnark verification.

[Function samples, timing and proof receipts](artifacts/fill-analysis.json.gz) · [Instruction sample counts](artifacts/fill-pcs.json.gz) · [Per-request symbol counts](artifacts/fill-profiles.json.gz) · [Profile protocol](artifacts/fill-protocol.md)
