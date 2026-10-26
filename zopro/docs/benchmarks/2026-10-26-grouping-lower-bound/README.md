# Exact cardinality lower bound before scalar grouping

The bitmap candidate does not pass the declared local adoption gate. Accepted production is unchanged. It replaces some rejected full grouping passes with an exact occupied-bit lower bound; it changes no proving math or grouping cutoff. This is a local Ryzen9950X comparison, with no new AWS or Aeglos execution.

## Verified SDK latency

Three rotated/reversed fresh-process blocks,31timed requests after three primes,32Rayon workers pinned to CPUs0–31. OFF and ON use the same executable and differ only in the new feature's explicit runtime flag. Witness solving, fresh randomness, proving, SDK verification and serialization are included. Loading/preparation is excluded. Each cell is the median of the three process medians.

| Circuit | Accepted ms | OFF ms | ON ms | ON/OFF paired geometric mean | ON maximum RSS MiB | Supplied M5 CPU / Metal ms | Candidate CPU / Metal speedup |
|---|---:|---:|---:|---:|---:|---:|---:|
|1x2|9.076810|8.640367|8.594861|0.995826|75.00|37 / 24|4.30× / 2.79×|
|2x2|11.638353|11.693535|11.777965|1.005682|270.25|59 / 30|5.01× / 2.55×|
|4x2|16.317358|16.409943|16.378142|0.986837|310.00|109 / 48|6.66× / 2.93×|
|8x2|24.329279|24.998121|25.008352|1.014096|511.00|189 / 74|7.56× / 2.96×|
|12x2|34.973165|34.781885|34.497167|0.999453|716.00|284 / 101|8.23× / 2.93×|
|16x2|40.529709|40.970010|40.791761|1.000497|825.00|335 / 124|8.21× / 3.04×|
|24x2|68.193555|69.053378|68.955805|0.998256|1285.02|528 / 176|7.66× / 2.55×|
|merge-24x1|67.461142|68.021279|68.544458|1.001544|1191.94|507 / 175|7.40× / 2.55×|
|merge-54x1|175.645760|178.766112|176.242775|0.993712|1913.49|not supplied|—|

Primary paired ON/OFF=0.995826, winning 2 of3blocks. Original-eight aggregate ON/OFF=1.000247; ON/accepted=0.998679. RSS is the maximum whole SDK process, including loading/setup/primes; it is not incremental scratch or minimum RAM. M5 references are historical supplied values. No shape is established at10× CPU speedup by this experiment.

## Gate outcome

The [predeclared gate](artifacts/design.md) requires primary ON/OFF≤0.98 with at least two block wins, no other shape slower than1.02, original-eight aggregate ON/OFF and ON/accepted≤0.995, and OFF/accepted calibration within1.03 paired and1.05 per block. All results are retained. Calibration failures and regressions:

- 1x2-w32 b0 OFF/accepted=0.9037281467244795
- 1x2-w32 b2 OFF/accepted=0.9523702710533765
- 1x2-w32 paired OFF/accepted=0.9523702710533765
- regressions: none.

## Algorithm and memory

The [preceding stage diagnostic](../2026-10-26-stage-costs/README.md) measured a1.35ms local primary G2 grouping policy, including a1.10ms serial map build/free which ultimately declines. That evidence motivates this separate test; it does not guarantee equal SDK latency savings when arithmetic competes for the same workers.

For8192≤n<32768 and the existing80percent grouping cutoff, hash nonzero scalars into next_power_of_two(16*n) bits. The number of occupied bits cannot exceed distinct nonzero scalars. If it exceeds floor(n*80/100), the unchanged exact grouping policy would necessarily reject. Otherwise run the original exact policy. Collisions only cause fallback. This is neither a probabilistic rejection decision nor a changed cutoff. It does not repeat the earlier95percent grouping-acceptance experiment.

The bitmap is temporary,16–64KiB, allocated after the existing checked grouping reservation and freed before full map/values/counts/classes allocation. Its vector allocation plus the existing64byte allowance fits within the already forecast classes Vec<u32> phase for every eligible size. Larger queries and other cutoffs allocate nothing extra. There is no retained key lookup table and no universal memory-cap change.

## Qualification and CPU profiles

All113 unfiltered native core tests pass, including four new tests covering exact cutoffs/zeros, adversarial collisions, independent cardinality oracles and every eligible size's memory bound. The generated-IFMA check, two raw tests and six cubic tests pass. Field source is unchanged; existing field qualification is inherited. All-nine OFF/ON correctness preflights complete72SDK18Gnark. The81timing processes complete2754SDK81Gnark, followed by three primary199Hz cycle profiles completing162SDK3Gnark. Total **2988SDK calls,102distinct independently Gnark-verified exports and204proof/public members**. Fixture public hashes,164byte proof lengths, source/binary pins and distinct randomized proof identities are checked against5238 prior exports. No successful capture is repeated.

Profiles follow all latencies and never decide adoption. Sample shares are aggregate user cycles, not wall-time savings. The generic grouping classifier matches scalar_groups/hashbrown. Compiler inlining can attribute grouping to a caller category, and a separately emitted new lower_bound symbol may fall into other. These categories are not isolated grouping-stage costs and should not be compared as such. Raw decoded symbols are retained, so no missing symbol is treated as zero work.

| Variant | Profile samples | Lost records | Grouping category % | Epoch + scheduler % |
|---|---:|---:|---:|---:|
|accepted|2985|0|0.226|14.075|
|off|3196|0|0.008|11.986|
|on|3139|0|0.001|9.280|

All170 accepted source files and the accepted SDK are unchanged. The candidate is preserved privately and is not adopted. No phone, TEE, memory-pressure, giant-circuit, concurrent throughput or C7a qualification is added. The original all-eight10× CPU objective remains unmet.

[Full analysis](artifacts/analysis.json), [frozen measurement configuration](artifacts/perf-plan.json), [exact source change](artifacts/edits.json), [new module](artifacts/candidate/crates/zopro-core/src/msm_affine/grouping/lower_bound.rs), [native validation](artifacts/native/result.json), [raw timing/profile rows](artifacts/performance/rows.json), [proof inventory](artifacts/proof-inventory.json) and [artifact index](artifacts/artifact-index.json) retain the evidence. Raw perf.data and binaries remain local with hashes; decoded profiles and proof exports are archived.
