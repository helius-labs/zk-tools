# C7a comparison if the bitmap candidate is accepted

This is the measured outcome **assuming we accept the candidate**. It is a proposed-adoption report; production has not been changed. The candidate passes proof verification, but the full performance qualification fails calibration and several regression checks. Those limitations remain part of the result.

The best tested 1→2 configuration is **12.096 ms at 64 workers**, with **69.3 MiB peak process RSS**. Aeglos's best tested result is **8.007 ms at 96 workers**, with **464.9 MiB RSS**. Aeglos remains 1.51× faster on this small proof; the candidate uses 6.71× less measured RSS. Against the accepted Zopro build's best tested 12.369 ms at 32 workers, selecting the candidate's best tested configuration improves latency by 2.21%.

The candidate wins on **six of nine measured shapes** against Aeglos when each uses its best tested worker count. Aeglos wins on 1→2, 2→2 and 4→2. Zopro's advantage on larger shapes and lower memory use mostly existed before this candidate; they should not be attributed to the bitmap change.

## Candidate versus Aeglos

Dedicated c7a.24xlarge, 96 physical cores, 30 GiB container. Each result is the median of three fresh-process medians. Every proof includes fresh witness solving, randomness, proving, SDK verification and serialization. Loading and preparation are excluded from latency. Workers are chosen from the tested counts: 32/64/96 for 1→2, and 64/96 for other shapes. This is a descriptive selection, not an implemented automatic worker policy.

| Shape | Candidate ms | Workers | Aeglos ms | Workers | Candidate RSS MiB | Aeglos RSS MiB |
|---|---:|---:|---:|---:|---:|---:|
|1x2|12.096|64|8.007|96|69.3|464.9|
|2x2|17.809|64|11.556|96|221.5|808.5|
|4x2|21.104|64|18.461|96|270.6|1349.1|
|8x2|27.456|64|31.115|96|476.2|2458.0|
|12x2|36.085|64|48.776|96|669.2|3710.5|
|16x2|40.090|64|56.886|96|793.3|4622.7|
|24x2|57.359|64|87.358|96|1262.3|6705.9|
|merge-24x1|59.220|64|84.884|96|1147.8|6569.8|
|merge-54x1|126.333|64|173.729|96|1916.1|12991.8|

At these configurations, candidate RSS is **3.7–6.8× lower** than Aeglos. RSS is the maximum whole process, including load/setup/primes; it is not minimum required RAM, an allocation bound or a phone measurement.

## What accepting this candidate changes

The candidate avoids some full scalar-grouping hash maps that would be discarded. It first counts occupied bits in a temporary 16–64 KiB bitmap. Occupancy is an exact lower bound on distinct nonzero scalars; collisions only fall back to the existing code. The grouping cutoff and proving math stay the same. The bitmap fits the existing grouping memory forecast, is released before full map allocation, and is not a retained key table.

The change has a larger effect on 1→2 with many C7a workers than on the local 32-thread Ryzen. Same-worker results:

| Workers | Accepted Zopro ms | Candidate ms | Paired gain versus accepted | Paired gain versus candidate OFF |
|---|---:|---:|---:|---:|
|32|12.369|12.377|0.31%|-0.00%|
|64|12.631|12.096|4.18%|5.59%|
|96|14.145|12.845|9.67%|13.63%|

These paired gains are geometric means of block ratios; they can differ from ratios of the displayed medians. At 32 workers there is no meaningful ON/OFF improvement. The 64-worker candidate uses more summed CPU per proof than accepted Zopro at 32 workers, despite its slightly lower latency:

| Configuration | CPU ms per proof, summed over threads |
|---|---:|
|Accepted Zopro, 32 workers|274.3|
|Candidate, 64 workers|476.3|
|Aeglos, 96 workers|509.5|

CPU includes coordination and spinning. Concurrent-proof throughput was not measured, so the fastest single-proof setting should not automatically become the server-wide default.

Using the candidate requires the `zopro_grouping_lower_bound` build flag and `ZOPRO_CPU_GROUPING_LOWER_BOUND=1`; the complete tested flags and environment are pinned in the benchmark artifacts. The 64-worker setting above is supported only as the best tested single-proof 1→2 result.

## Supplied M5 comparison

M5 values are historical user-supplied references, not fresh Apple measurements. These ratios use candidate best-tested latency. No M5 reference was supplied for 54→1.

| Shape | Candidate ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
|1x2|12.096|37|3.06×|24|1.98×|
|2x2|17.809|59|3.31×|30|1.68×|
|4x2|21.104|109|5.16×|48|2.27×|
|8x2|27.456|189|6.88×|74|2.70×|
|12x2|36.085|284|7.87×|101|2.80×|
|16x2|40.090|335|8.36×|124|3.09×|
|24x2|57.359|528|9.21×|176|3.07×|
|merge-24x1|59.220|507|8.56×|175|2.96×|
|merge-54x1|126.333|—|—|—|—|

**The all-eight CPU 10× goal is unmet: 0/8 rows reach it.** For 1→2, the target is 3.7 ms; the candidate's 12.096 ms still requires about a further 3.27× latency reduction. This candidate is an incremental optimization.

## Acceptance limits

All proof checks pass: **5,640 SDK calls, 272 distinct independently Gnark-verified exports across nine keys**, and all 272 exports verify again locally after transport. The fresh target build passes 113 core tests and the raw/cubic proof tests.

The full predeclared performance gate fails. Primary 64/96 speed checks pass, but some OFF/accepted calibration checks fail and several other shapes are slower against the accepted build. Across the original eight shapes, paired ON/accepted aggregate latency changes by:

- 64 workers: +0.93% (positive is slower).
- 96 workers: -0.35% (positive is slower).

Accepting the results therefore does not support describing this as a general prover speedup. The evidence supports a small-proof benefit on C7a with many workers, while broader adoption needs those calibration/regression concerns resolved. There are no new phone, TEE, concurrent-throughput, memory-pressure or huge-circuit measurements.

The whole workflow also records some CPU quota throttling; per-case attribution is unavailable. It is not established as the cause of any regression. Exact counters, all block ratios and every failed check are in the [full benchmark report](README.md) and its [analysis](artifacts/analysis.json).

The owned AWS instance and temporary storage/image/transfer resources are cleaned up. **This report assumes acceptance for comparison; the candidate remains unadopted and production source is unchanged.**
