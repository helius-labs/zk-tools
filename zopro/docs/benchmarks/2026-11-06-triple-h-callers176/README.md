# H-only complete unweighted G1 caller comparison

The cancelled-slope native G1 reducer was measured against both frozen and fresh binary controls in all seven original caller cases with 1, 16 and 32 workers. **The predeclared component gate did not pass.** This is a complete-reduction experiment, not a proof measurement or production adoption. Accepted local 1×2 remains **7.928660 ms**; the CPU 10× target remains unmet for all eight shapes.

The blanket ternary replacement is **not selected for SDK integration**. All 21 fresh-control calibration checks pass, and all counter/profile panels qualify. The gate fails 27 gain/nonregression checks: tile257 regresses, tile511 loses at high concurrency, and several other tiles do not establish a gain. The four-main 32-worker aggregate improves, but that does not waive the individual failures. No favorable subset of this implementation is promoted.

There is a substantial retained result in the large group. At 16 workers, its paired new/OFF ratio is **0.305102 [0.299897, 0.309476]**, 15/15 wins (about **3.28× faster**); at 32 workers it is **0.485347 [0.480909, 0.488594]**, 15/15 wins (about **2.06× faster**). Against the frozen control the corresponding ratios are 0.307184 and 0.488344, both 15/15. Solo improves only about 2.6%. Tile511 at 32 workers has new/OFF 1.057721 [1.045720, 1.064149], 0/15 wins, so this is not a universal arithmetic win.

The large group's reported retained capacity falls **6,162,264 → 3,360,944 bytes per worker**, including common storage, binary fallback and fixed structs. Whole-process maximum observed RSS falls 178.25 → 122.75 MiB at 16 workers and 354.50 → 243.00 MiB at 32. These are observations of this public synthetic fixture, not minimum RAM, memory-pressure feasibility or a full-proof resource result.

Counter evidence supports a working-set hypothesis. The 32-worker large-group PMU panel has cycles/wave 3.78233 billion → 1.83422 billion (about −51.5%), instructions 352.277 million → 351.142 million (about −0.32%) and cache misses 8.911 million → 6.475 million (about −27.3%). Both routes keep roughly 31 CPUs busy. Minor faults remain small, with measured medians 0.125 → 0.4375 per wave. The frozen panel agrees closely with OFF. Profiles retain the same amount of outer Work/reduction-copy sampling while most of the reduction-kernel cycle mass falls. This is consistent with memory/cache working-set costs; no direct DRAM-bandwidth or causal cache-level decomposition was measured. Fewer instructions, page faults or idle CPUs do not explain the size of the observed improvement on their own.

The next distinct question is whether G1 can use compact **binary** tree records without the ternary arithmetic and small-case regressions. The accepted generic tree already has a two-vector prefix/norm route enabled for Fq2; Fq currently uses five-vector groups. Earlier G1 **streaming** prefix-only records were negative and remain so. A G1 binary-tree lifetime change is a hypothesis requiring source/history review, fixed scratch ownership, complete weighted/unweighted correctness and a new prospective protocol. It is not an adoption of the favorable ternary case or a size-threshold sweep.

Independent review reproduces all 89 timing checks, all 27 failures, all 1,107 workflow receipts, 1,032 threaded callers plus 14 legacy preflights, and 10,321,095 profile samples. All 12 profiles have no lost events and qualify. No fresh proof, SDK latency, AWS/Aeglos comparison or all-eight 10× achievement is added.


## Implementation and controls

The [native qualification](../2026-11-06-triple-h-native176/README.md) passed five focused native tests and independent Arkworks checks, including exceptional fallback, partial lanes, scratch reuse and 512 KiB stacks on 1/4/16/32 workers. Those successful tests were inherited here, not rerun. The accepted field and qualified native core were reused unchanged. This formula uses H rather than dH, retains two SIMD vectors per record, shares the inverse with pair remainders, rereads coordinates and defers compaction until reverse stores are complete. Weighted trees remain unchanged.

The frozen binary route and new route use the same threaded executable linked to that unchanged native core, with different CLI selections. This frozen control contains the byte-exact accepted binary reducer; it is not the accepted proof SDK. Fresh OFF is a separately built core differing only by replacing the Work loop's SELECTED condition with false. All other 177 snapshot files, struct layout, fixtures and binary arithmetic are exact. A new threaded driver was needed for the declared contention contexts. The original qualified solo caller was retained and used in 14 diagnostic preflights.

All new core/final-consumer commands explicitly use O3/native/oneCGU; final consumers use fatLTO. The three missing builds completed once. There was no field rebuild, lookup table, key/image copy, additional production pool, C/ASM change or layout/attribute/threshold sweep. All 176 production source files remain exact.

## Protocol and complete results

The full 1,107-command Rust/xtask protocol was frozen before capture: 63 threaded preflights, 14 original solo preflights and one check; 945 latency processes across 21 contexts, three arms and 15 paired blocks; 12 PMU panels, 12 profiles and 60 offline steps. Every output passed the complete projective fixture oracle. Preflights also matched the original caller's coordinate hashes, capacities and allocation counts. The threaded driver primes 24 complete reductions per worker, then measures only reductions between barriers; output checks/hashing occur after release.

Tile257/384/511/2048 use fresh buffers; group35973/groups31x256/tails reuse buffers. Workers duplicate complete public fixtures with private scratch. Solo is pinned to CPU2, 16 workers to the 16 physical cores and 32 workers to all SMT threads. These are contention measurements, not proof-level parallel scaling or concurrent-proof throughput.

Ratios below are paired new/control medians with conservative median intervals (sorted ranks 4–12 of 15) and wins. Below one is faster. Fresh OFF/frozen calibration requires the entire interval in [0.97,1.03] for every context. New/control nonregression requires upper ≤1.01 for every context; both-control gain requires upper <1 and at least 11 wins for every main context, plus the four-main 32-worker geometric aggregate. No minimum gain, extra blocks, favorable subset or extension was used. All 89 timing checks and profile/counter eligibility are preserved.

| Case | Workers | Main | Frozen ns/wave | OFF ns/wave | New ns/wave | New/OFF [interval], wins | New/frozen [interval], wins | OFF/frozen calibration |
|---|---:|:---:|---:|---:|---:|---|---|---|
|tile257|1|no|12674.13|12599.41|12841.37|1.020414 [1.011400, 1.027313], 0/15|1.014256 [1.004998, 1.022659], 2/15|0.995778 [0.985934, 0.998063], 12/15|
|tile257|16|no|14270.83|14163.58|14429.33|1.018616 [1.015950, 1.019638], 0/15|1.010216 [1.008290, 1.014115], 1/15|0.992695 [0.989863, 0.994944], 14/15|
|tile257|32|no|19897.91|19923.42|20609.93|1.031779 [1.015420, 1.048397], 2/15|1.028781 [1.020329, 1.049997], 1/15|0.998278 [0.991569, 1.006043], 8/15|
|tile384|1|yes|16197.60|16134.52|15694.98|0.974179 [0.966078, 0.979134], 15/15|0.965865 [0.963650, 0.974298], 15/15|0.992518 [0.990014, 1.000849], 11/15|
|tile384|16|yes|18241.91|18165.47|17608.95|0.970873 [0.957405, 0.976869], 15/15|0.967914 [0.957182, 0.971886], 15/15|0.995299 [0.985898, 1.006807], 9/15|
|tile384|32|yes|25543.15|25443.94|25805.13|1.008288 [0.978926, 1.024250], 6/15|1.007427 [0.988582, 1.019714], 5/15|0.998160 [0.990450, 1.003861], 10/15|
|tile511|1|yes|19335.67|19313.29|19346.25|1.001246 [0.997042, 1.005825], 7/15|0.995427 [0.989897, 1.002042], 8/15|0.994691 [0.988568, 1.001422], 10/15|
|tile511|16|yes|21733.72|21605.96|21728.52|1.006959 [1.001987, 1.013397], 2/15|0.999997 [0.992491, 1.004447], 8/15|0.994111 [0.985360, 0.998531], 12/15|
|tile511|32|yes|30276.43|30528.69|32153.10|1.057721 [1.045720, 1.064149], 0/15|1.056157 [1.039674, 1.071940], 1/15|1.003240 [0.995062, 1.015202], 7/15|
|tile2048|1|yes|56506.08|56386.60|56566.12|1.003431 [0.997128, 1.008610], 7/15|0.998399 [0.994592, 1.006173], 8/15|0.995849 [0.994539, 1.003183], 10/15|
|tile2048|16|yes|64009.17|63769.26|63323.89|0.998606 [0.984809, 1.001997], 10/15|0.989946 [0.982034, 1.003621], 11/15|0.997284 [0.987973, 1.004441], 10/15|
|tile2048|32|yes|97809.36|97161.74|97532.39|1.003913 [0.979024, 1.015161], 7/15|1.000035 [0.989559, 1.013546], 7/15|1.001743 [0.991126, 1.016335], 6/15|
|group35973|1|yes|902258.88|903783.00|880071.44|0.973820 [0.972155, 0.977521], 15/15|0.976745 [0.974314, 0.977944], 15/15|1.001689 [0.999100, 1.003318], 5/15|
|group35973|16|yes|6471105.25|6500397.94|1990812.62|0.305102 [0.299897, 0.309476], 15/15|0.307184 [0.302549, 0.310983], 15/15|1.003819 [0.996545, 1.016420], 5/15|
|group35973|32|yes|23259560.62|23436452.31|11390268.19|0.485347 [0.480909, 0.488594], 15/15|0.488344 [0.486894, 0.491363], 15/15|1.008044 [1.003186, 1.010526], 2/15|
|groups31x256|1|no|193020.41|194234.41|188290.70|0.971145 [0.966610, 0.975683], 15/15|0.975527 [0.972958, 0.980324], 15/15|1.006742 [1.001672, 1.010359], 2/15|
|groups31x256|16|no|221768.50|222427.39|213949.38|0.961107 [0.956623, 0.967646], 15/15|0.963473 [0.955343, 0.968648], 14/15|0.999783 [0.997606, 1.008578], 8/15|
|groups31x256|32|no|394392.80|401213.77|352962.34|0.884155 [0.866421, 0.894884], 15/15|0.894377 [0.882437, 0.915763], 15/15|1.013419 [1.004748, 1.028358], 3/15|
|tails|1|no|11783.08|11733.66|10385.48|0.883265 [0.875113, 0.888708], 15/15|0.877213 [0.863181, 0.889033], 15/15|0.994535 [0.984569, 1.001112], 10/15|
|tails|16|no|12828.12|12775.21|11300.96|0.883858 [0.880901, 0.885897], 15/15|0.880297 [0.876058, 0.885281], 15/15|0.995171 [0.991456, 1.001719], 11/15|
|tails|32|no|18872.06|18818.70|16420.89|0.875820 [0.863185, 0.878259], 15/15|0.876952 [0.867724, 0.881593], 15/15|0.997558 [0.988778, 1.013744], 8/15|

Four-main geometric aggregate at 32 workers:

- New/OFF: 0.843865 [0.839575, 0.857532], 15/15.
- New/frozen: 0.849287 [0.840992, 0.854506], 15/15.

Failed checks (all retained):

- `tile257-w1/ON/OFF/nonregression`
- `tile257-w1/ON/frozen/nonregression`
- `tile257-w16/ON/OFF/nonregression`
- `tile257-w16/ON/frozen/nonregression`
- `tile257-w32/ON/OFF/nonregression`
- `tile257-w32/ON/frozen/nonregression`
- `tile384-w32/ON/OFF/nonregression`
- `tile384-w32/ON/OFF/gain`
- `tile384-w32/ON/frozen/nonregression`
- `tile384-w32/ON/frozen/gain`
- `tile511-w1/ON/OFF/gain`
- `tile511-w1/ON/frozen/gain`
- `tile511-w16/ON/OFF/nonregression`
- `tile511-w16/ON/OFF/gain`
- `tile511-w16/ON/frozen/gain`
- `tile511-w32/ON/OFF/nonregression`
- `tile511-w32/ON/OFF/gain`
- `tile511-w32/ON/frozen/nonregression`
- `tile511-w32/ON/frozen/gain`
- `tile2048-w1/ON/OFF/gain`
- `tile2048-w1/ON/frozen/gain`
- `tile2048-w16/ON/OFF/gain`
- `tile2048-w16/ON/frozen/gain`
- `tile2048-w32/ON/OFF/nonregression`
- `tile2048-w32/ON/OFF/gain`
- `tile2048-w32/ON/frozen/nonregression`
- `tile2048-w32/ON/frozen/gain`

## Memory, CPU use and faults

Capacity totals below include the common points/counts/output, retained binary fallback scratch, ternary indices and both prefix/denominator buffers, plus fixed structs. Each worker reported stable capacities across blocks; reused cases had zero measured allocation calls. Values are for one complete reduction worker. The RSS columns are the maximum observed whole-process counter across the latency blocks and include setup/fixtures/workers; they are not minimum RAM.

| Case | Workers | OFF capacity B/worker | New capacity B/worker | OFF/new allocations per worker-round | OFF/new RSS MiB | OFF/new busy CPUs | OFF/new minor faults/wave |
|---|---:|---:|---:|---|---|---|---|
|tile257|1|45136|26968|11.000 / 10.000|2.500 / 2.250|1.000 / 1.000|0.002 / 0.003|
|tile257|16|45136|26968|11.000 / 10.000|2.250 / 2.250|15.548 / 15.501|0.002 / 0.002|
|tile257|32|45136|26968|11.000 / 10.000|2.500 / 2.250|30.708 / 30.212|0.011 / 0.004|
|tile384|1|67088|38640|12.000 / 10.000|2.500 / 2.500|1.000 / 1.000|0.003 / 0.003|
|tile384|16|67088|38640|12.000 / 10.000|2.250 / 2.250|15.499 / 15.577|0.002 / 0.002|
|tile384|32|67088|38640|12.000 / 10.000|2.500 / 2.500|31.011 / 29.844|0.012 / 0.006|
|tile511|1|86416|51936|12.000 / 10.000|2.500 / 2.500|1.000 / 1.000|0.003 / 0.003|
|tile511|16|86416|51936|12.000 / 10.000|2.250 / 2.250|15.550 / 15.539|0.002 / 0.002|
|tile511|32|86416|51936|12.000 / 10.000|2.750 / 2.500|30.947 / 29.620|0.025 / 0.009|
|tile2048|1|346128|194336|14.000 / 9.000|3.000 / 2.750|1.000 / 1.000|0.008 / 0.012|
|tile2048|16|346128|194336|14.000 / 9.000|10.250 / 6.750|15.364 / 15.556|0.008 / 0.008|
|tile2048|32|346128|194336|14.000 / 9.000|18.750 / 11.250|30.083 / 30.432|0.008 / 0.008|
|group35973|1|6162264|3360944|0.000 / 0.000|13.500 / 10.000|1.000 / 1.000|0.188 / 0.188|
|group35973|16|6162264|3360944|0.000 / 0.000|178.250 / 122.750|15.588 / 15.222|0.125 / 0.125|
|group35973|32|6162264|3360944|0.000 / 0.000|354.500 / 243.000|31.078 / 30.675|0.125 / 0.438|
|groups31x256|1|1327864|828152|0.000 / 0.000|4.750 / 4.000|1.000 / 1.001|0.047 / 0.047|
|groups31x256|16|1327864|828152|0.000 / 0.000|38.500 / 27.000|15.517 / 15.447|0.031 / 0.031|
|groups31x256|32|1327864|828152|0.000 / 0.000|74.750 / 51.500|29.986 / 31.084|0.031 / 0.031|
|tails|1|46952|27768|0.000 / 0.000|2.500 / 2.250|1.000 / 1.000|0.002 / 0.002|
|tails|16|46952|27768|0.000 / 0.000|2.500 / 2.250|15.488 / 15.465|0.002 / 0.002|
|tails|32|46952|27768|0.000 / 0.000|2.250 / 2.250|29.926 / 30.022|0.002 / 0.002|

Busy CPUs are process CPU time divided by elapsed wall time, using the reported medians. Minor faults count page mappings that did not require disk reads; this experiment does not establish swap behavior, device limits or a memory-pressure cause. Optional ternary and binary fallback capacities may coexist. Future runtime routing needs existing resource admission; no universal RAM cap is inferred.

## Counters and profiles

All latency finished before PMU/profiling. Counter panels cover cycles, instructions, cache misses and branch misses with declared ≥90% coverage. Fixed-period user-cycle profiles use 16,000,000 cycles per sample, measured-region FIFO control, raw ownership/hash checks, saved metadata/normal decodes, event attributes, worker TID→CPU attribution and lost-event checks. A profile qualifies only with at least 1,000 samples, unknown ≤5% and no lost events. Profiles explain results and cannot rescue a failed latency gate.

| Profile | Samples | Unknown | Qualified |
|---|---:|---:|:---:|
|cycles-group35973-w1-OFF|4987|0.0000%|true|
|cycles-group35973-w1-ON|4850|0.0825%|true|
|cycles-group35973-w1-frozen|4965|0.0806%|true|
|cycles-group35973-w32-OFF|3850245|0.1952%|true|
|cycles-group35973-w32-ON|1902175|0.1195%|true|
|cycles-group35973-w32-frozen|3849629|0.1951%|true|
|cycles-tile384-w1-OFF|5653|0.7430%|true|
|cycles-tile384-w1-ON|5507|0.9624%|true|
|cycles-tile384-w1-frozen|5667|0.5294%|true|
|cycles-tile384-w32-OFF|232313|1.3736%|true|
|cycles-tile384-w32-ON|222284|1.6960%|true|
|cycles-tile384-w32-frozen|232820|1.4256%|true|

## Scope and continuation

Only a fully eligible complete-caller result selects a separate verified SDK trial covering all eight shapes, merge54, physical16, portable execution, concurrency, resources and Gnark verification. Component gains do not establish proof-level gains. Previous [old dH caller](../2026-10-28-unweighted-triple-caller174/README.md) and [mixed SDK](../2026-10-28-mixed-sdk/README.md) negative results remain unchanged. No new SDK/Gnark proof, current AWS/Aeglos comparison, phone/ARM/TEE, huge-circuit, pressure/swap, minimum-RAM or concurrent-proof throughput result is added.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. They are not new measurements.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS132.75 MiB is inherited. Source176/SDK6d83 and the 30,846-proof ledger are unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.




## Artifact storage

The 552 MiB of raw recordings and several GiB of decoded streams remain intact at `/home/ubuntu/src/zopro-perf-work/triple-h-callers176/perf/` and `runs/`. They are indexed by exact byte size and SHA256, and were independently reviewed before publication. These large files are not embedded in Git. The receipt archive retains every command/exit and all timing/preflight/counter output; its only omitted workflow files are the 24 large normal/metadata decode stdout streams, all listed in [the decoded-stream index](artifacts/profile-stream-index.json). No capture or decode was repeated for packaging.


[Analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [decision](artifacts/decision.md), [design](artifacts/design.md), [declaration](artifacts/declaration.json), [identities](artifacts/identities.json), [driver](artifacts/driver.rs), [perf control](artifacts/perf_support.rs), [capture plan](artifacts/plan.json), [pins](artifacts/pins.json), [profile symbols](artifacts/profile-symbols.json), [next stage](artifacts/next.md), [one OFF source delta](artifacts/OFF-source-delta.json.gz), [exact helpers/receipts](artifacts/helpers-and-receipts.json.gz), [raw profile index](artifacts/raw-profile-index.json), [publication](artifacts/publication.json), [checksums](SHA256SUMS). ON source is bound by the [prior native source manifest/deltas](../2026-11-06-triple-h-native176/README.md); full snapshots/binaries stay local. No private input, key/image/witness/proof bytes, external paper text or executable/library blobs are included.
