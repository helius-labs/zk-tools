# G2 trees with current prefix-record additions

The prefix-tree component does not pass the declared gate; stream calibration fails. Production remains unchanged. This is a rawG2 component ablation, not full-proof latency or M5 speed evidence.

Primary32 prefix/stream paired geometric ratio is1.038746 (3.87% slower,one of seven wins); prefix/frozen is1.050265 (5.03% slower,zero wins). The revised storage/kernel uses less requested peak than the generic tree, but primary32 still needs24.961MiB above prepared inputs versus8.653MiB for the fresh stream. No SDK experiment or adoption follows from this result.

Five per-block frozen-stream calibration failures limit fine timing conclusions. All are retained. In primary16/block5 the frozen median is5.724185ms and fresh stream3.732940ms; the favorable pooled prefix/frozen ratio at16workers has only one of seven winning blocks. It is not selected as a speedup. No cause is assigned from that difference or from context-switch counts alone, and no capture is repeated.

The previous [generic grouped tree](../2026-10-26-g2-level-tree/README.md) coupled topology with a larger generic addition kernel. This variant keeps256-bucket groups and level pairing, using raw signed G2Affine storage and the current BatchAdd prefix-record kernel at each level. Identities,doubling and cancellation are handled before disjoint ordinary pairs enter the batch; pair results compact in place,including odd tails. Width12,parts2,phi,narrow top handling and weighting stay unchanged. Existing field/prefix kernels and SDK dispatch are untouched. No new pool or key table. This isolates the combined storage/kernel change, not each instruction independently.

Typed optional admission includes point/pair capacities,growth bounds of prefix records/tails,allocator margins and stack metadata across all potentially live parents,above an original raw-query floor. Explicit configuration/admission errors prevent hidden fallback. It is conservative and not a universal process-RAM guarantee.

## Complete component caller

Actual1x2/2x2B2 points and fixture scalars. Seven rotated/reversed fresh-process blocks,seven timed requests after two primes,plus one stream reference per process. CPU0/CPUs0–15/CPUs0–31 for1/16/32workers. Frozen is the exact prior component binary; other modes share the new binary. Timers include scalar clone,decomposition,admission,routing,allocations,all reductions/merges,result normalization/equality and temporary drops. They exclude load,witness solving,reference setup,scalar grouping,other queries,blinding,SDK verification and serialization. Both binaries use allocator-accounting atomics.

Displayed times are medians of process medians. Paired ratios use within-block geometric means and may differ from displayed-median ratios. Requested peaks are caller-layout bytes above prepared inputs,not minimum RAM; RSS includes load/reference/primes.

|Case|Frozen ms|Fresh stream ms|Generic tree ms|Prefix tree ms|Prefix/stream|Prefix/frozen|Prefix/generic|
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2-w1|40.400939|40.984620|49.620806|44.672255|1.089463|1.106097|0.899742|
|1x2-w16|3.660984|3.638822|4.061228|3.719645|1.028480|0.977396|0.904753|
|1x2-w32|2.996611|2.997844|3.258225|3.130414|1.038746|1.050265|0.953696|
|2x2-w1|64.793079|65.878736|76.497491|68.613585|1.043583|1.060868|0.900821|
|2x2-w16|5.990477|6.035703|6.221924|5.782765|1.027732|1.026633|0.910811|
|2x2-w32|4.880935|4.865425|5.312238|4.920450|1.015546|1.011582|0.923072|

## Memory observations

|Case|Stream/prefix requested peak MiB|Stream/prefix whole-process RSS MiB|Prefix typed forecast MiB|
|---|---:|---:|---:|
|1x2-w1|1.556/2.199|60.75/60.75|110.65|
|1x2-w16|7.615/16.741|64.00/72.32|130.11|
|1x2-w32|8.653/24.961|64.31/83.00|150.86|
|2x2-w1|2.252/3.283|94.53/94.52|176.59|
|2x2-w16|8.547/22.830|100.67/105.17|196.04|
|2x2-w32|9.769/35.777|99.39/121.46|216.79|

## Gates and checks

Fresh/frozen stream calibration: factor1.05perblock and1.03pairedmedian allsixcases. Prefix must improve primary16/32againstbothstreamcontrols with>=4/7wins each;2x2ratios<=1.02againstbothat16/32. Workers1andprefix/generic are descriptive. No minimum gain floor or changed gate.

Calibration failures:

- 1x2/w16/b1 stream/frozen1.062054081634883
- 1x2/w16/b2 stream/frozen1.0578960345148176
- 1x2/w16/b5 stream/frozen0.6521347580485258
- 2x2/w16/b0 stream/frozen1.0582392511318803
- 2x2/w16/b2 stream/frozen0.8992489203565348

Speed/nonregression failures:

- 1x2/w16 prefix_stream ratio1.0284800191033112 wins2/7
- 1x2/w16 prefix_frozen ratio0.9773962204222469 wins1/7
- 1x2/w32 prefix_stream ratio1.0387457673821539 wins1/7
- 1x2/w32 prefix_frozen ratio1.0502648678082611 wins0/7
- 2x2/w16 prefix_stream ratio1.0277317145173184
- 2x2/w16 prefix_frozen ratio1.0266334666767671

Full117nativecore tests,raw2/cubic6andgeneratedIFMA checks pass. New tests cover signed complete-curve groups,empty/odd/skewed buckets,typed overflow,worker oracles and30089random points. Field math is unchanged;qualification inherited. Every timed output equals its stream reference,point order agrees with the compressed key,and requested peak stays within its typed envelope plus scalar clone. No SDK/Gnark proofs;prior5958unchanged.

Three primary32profiles follow ALL168latencies,199Hzwith64timedcallsafter2primes,gated after preparation. perf --no-buildid-cache avoids the prior metadata-hardlink interruption. Profiles are descriptive user-cycle samples,not latency evidence.

|Mode|Samples|Lost|SDK PIDs|
|---|---:|---:|---:|
|stream|1795|0|1|
|tree|1616|0|1|
|prefix|1329|0|1|

The prefix profile samples the current G2 prefix kernel prominently (40.94%),while generic `reduce_layer` remains11.44% for unchanged top/weighted work. This confirms the intended kernel is exercised,without making an exclusive-stage or causal timing claim. Inlined Rayon helper symbols may include arithmetic,point-copy and setup work. Using the current kernel closes part of the observed generic-tree deficit in this run; it does not make the tree faster than the existing stream.

These local results do not establish behavior on a96-physical-core host. Any further investigation of the parallel-depth hypothesis should be a separately frozen high-core component study,not another local group-width variant or a promotion of this failed candidate. It must preserve the negative local result and cannot substitute a raw-query gain for full verified proof latency.

Total1881rawqueries includes171reference setups. This component does not establish a whole-proof/M5 speedup or qualify allninecircuits. [Latest all-nine SDK table](../2026-10-26-prefix-pool/README.md) retains supplied CPU/Metal references. No AWS,Aeglos,phone,TEE,pressure,giant or throughput qualification. No adoption. The original all-eightCPU10× goal remains active and unmet.

All helper binaries compiled before latency;no successful capture repeats or exclusions. [Design](artifacts/design.md),[analysis](artifacts/analysis.json),[source audit](artifacts/source-audit.json),[jobs](artifacts/jobs.json),[artifact index](artifacts/artifact-index.json) retain the evidence. Raw profiles,binaries and large inputs remain local with hashes.
