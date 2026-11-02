# Paired G2 coordinate packing

The candidate **fails the declared caller screen and is not adopted**. This measures complete signed borrowed G2 addition batches on synthetic valid points; it is not a proof or full MSM latency result. Production and the accepted SDK remain unchanged.

Logical32 ON/OFF equal-log ratio is **1.012284 [1.009150, 1.014990], 1/15**; ON/FROZEN is **1.011777 [1.004952, 1.014439], 2/15**. All calibration and individual nonregression checks pass, while all four required aggregate-gain checks fail. This closes the paired-constructor implementation without an SDK follow-up or layout/attribute rescue.

The actual G2 function shrinks from 51,746 to 51,617 bytes and from 8,587 to 8,573 instruction lines. Static IFMA sites stay at 1,450, selected shuffle lines rise from 70 to 118, and stack-base operand lines rise from 1,475 to 1,491. These observations describe a different load/shuffle trade; they do not isolate the cause of the measured regression or measure dynamic spills. Removing source-level arrays did not establish faster execution.

The constructor loads both adjacent Fq components from each existing 64-byte Fq2 coordinate and performs an 8x8 transpose using fixed-immediate shuffles. It removes explicit field staging arrays and reverse-pass point copies, at the cost of more transpose operations than the original pair of 8x4 transposes. Only full-vector borrowed G2 prefix/reverse coordinate loads change. Field multiplication, norms, signed differences, inversions, tails, point formulas, scratch capacities, bucket geometry, scheduling, solver and key representation stay original. No heap buffer, lookup table, copied key, retained plan, arbitrary RAM cap or C/assembly implementation is added. Native stack use and emitted code can change.

The earlier G1 direct-reference constructor is a distinct rejected experiment: it joined 32-byte loads and retained the single-field transpose. This screen does not reopen that result.

## Qualification and protocol

Both private snapshots recover all 176 accepted production files exactly. FROZEN adds only a caller adapter; the candidate snapshot contains 179 files, with 171 originals exact and five parents recoverable. Original IFMA arithmetic and generator files remain exact; the generator consistency check passes. Both OFF and ON pass 36 unfiltered native field tests and 129 core tests. FROZEN's 129 core tests pass once and are reused. The new field test checks 49,488 component-lane inputs against independent raw bit extraction, the original constructor/unpacker and scalar products on canonicalized operands. It includes lazy boundaries through 2p, unaligned/noncontiguous coordinates, repeated references and lane permutations.

The first field run passed 35 tests and failed the new test's lazy unpacking expectation. Existing unpacking normalizes values, so exact lazy-word preservation was an incorrect oracle. Only that oracle and scalar reference operand normalization were repaired; the constructor was unchanged. Original sources and failed receipts remain available. An initial wrong xtask subcommand and a Rust helper ownership compile error are also retained. Four successful frozen build steps were reused; no successful qualification or performance capture was repeated.

Three separately frozen coherent builds use native O3, fat LTO and one codegen unit. ON alone enables the paired constructor. Six contexts cover sizes 200/352 at 1, 16 and 32 pinned workers. Each fresh process runs three checked 1,024-batch primes, then 8,192 changing batches per worker. Fifteen rotated/reversed blocks follow 18 preflights. Timed allocations are zero; every final bucket matches the generator scalar-sum oracle and outputs/capacities match across arms. Parallel workers here issue independent addition batches; this is not the full proof scheduler or a concurrent-proof throughput result.

Paired ratios are candidate/control; lower is faster. Bounds use ranks 4..12 of 15, with nominal 96.484375% marginal coverage under independent exchangeable blocks and no familywise guarantee. All six calibration intervals must fit factor 1.03, all ON/control upper bounds must be <=1.03, and both all-six/logical32 aggregates require upper<1 and >=10 wins against both controls. No minimum gain floor, exclusions, extensions, unchanged reruns or subset promotion.

|Context|FROZEN / OFF / ON us per wave|ON/OFF [bounds], wins|ON/FROZEN [bounds], wins|
|---|---:|---|---|
|g2-borrowed-signed-200-solo|7.884 / 7.866 / 7.866|1.003501 [0.993867, 1.008336], 7/15|0.997948 [0.991026, 1.010939], 8/15|
|g2-borrowed-signed-200-physical16|9.116 / 9.042 / 9.033|0.997036 [0.995157, 1.005277], 8/15|0.991356 [0.980240, 1.002178], 10/15|
|g2-borrowed-signed-200-logical32|14.687 / 14.713 / 14.855|1.010224 [1.005085, 1.012780], 2/15|1.005897 [1.002255, 1.016266], 3/15|
|g2-borrowed-signed-352-solo|13.304 / 13.274 / 13.314|1.000982 [0.992392, 1.008023], 7/15|1.001796 [0.995500, 1.007722], 6/15|
|g2-borrowed-signed-352-physical16|15.389 / 15.371 / 15.392|1.001759 [0.993216, 1.005424], 7/15|0.998001 [0.991807, 1.006333], 9/15|
|g2-borrowed-signed-352-logical32|25.278 / 25.175 / 25.455|1.013353 [1.011137, 1.017750], 1/15|1.010766 [1.003133, 1.023843], 0/15|

all6 ON/off: **1.003977 [1.002309, 1.006789], 1/15**.

all6 ON/frozen: **1.003845 [0.996498, 1.006252], 4/15**.

logical32_2 ON/off: **1.012284 [1.009150, 1.014990], 1/15**.

logical32_2 ON/frozen: **1.011777 [1.004952, 1.014439], 2/15**.

Failed checks: `["all6/on/frozen/gain","all6/on/off/gain","logical32_2/on/frozen/gain","logical32_2/on/off/gain"]`. Full calibration rows, raw ratios and CPU counters are in [all results](artifacts/analysis.json).

## CPU, memory and profiles

|Context|OFF / ON thread CPU us per batch|OFF / ON max-RSS counter MiB|OFF / ON minor faults per call|
|---|---:|---:|---:|
|g2-borrowed-signed-200-solo|7.865 / 7.865|2.25 / 2.25|0.00024 / 0.00024|
|g2-borrowed-signed-200-physical16|8.866 / 8.834|2.50 / 2.50|0.00002 / 0.00002|
|g2-borrowed-signed-200-logical32|14.512 / 14.542|3.50 / 3.50|0.00001 / 0.00001|
|g2-borrowed-signed-352-solo|13.273 / 13.313|2.50 / 2.50|0.00024 / 0.00024|
|g2-borrowed-signed-352-physical16|15.090 / 15.089|6.25 / 6.25|0.00002 / 0.00002|
|g2-borrowed-signed-352-logical32|24.833 / 25.114|10.75 / 10.75|0.00001 / 0.00001|

RSS counters are diagnostic and may undercount; they do not establish precise lifetime peaks, minimum RAM or pressure/swap behavior. Minor faults do not measure their service time or imply disk paging. Major faults are not exported.

Four fixed16M user-cycle profiles were collected after all latency measurements. Each passed the >=1,000-sample/<=5%-unknown attribution screen with zero LOST and one sampled process. Full symbol-period totals remain in the results. Absolute sampling coverage, kernel cost, local CPI and critical paths were not established. Static instruction/stack-base/shuffle counts describe emitted code, not dynamic spill or cache traffic; see [code summary](artifacts/codegen-summary.json) and [assembly](artifacts/assembly.json.gz).

## Accepted CPU and Metal comparisons, unchanged

These inherited local 32-worker SDK measurements and supplied M5 references were not remeasured by this component experiment. Accepted SDK SHA-256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All eight CPU 10x targets remain unmet.

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37 / 4.667x|24 / 3.027x|3.7|
|2x2|11.151932|59 / 5.291x|30 / 2.690x|5.9|
|4x2|16.086386|109 / 6.776x|48 / 2.984x|10.9|
|8x2|24.179453|189 / 7.817x|74 / 3.060x|18.9|
|12x2|33.884455|284 / 8.381x|101 / 2.981x|28.4|
|16x2|40.077381|335 / 8.359x|124 / 3.094x|33.5|
|24x2|65.850777|528 / 8.018x|176 / 2.673x|52.8|
|merge-24x1|65.060937|507 / 7.793x|175 / 2.690x|50.7|

Accepted merge54 remains 161.883899 ms without a supplied M5 reference. Historical source175 AWS results remain Zopro 11.915591 ms at 32 workers and Aeglos 7.959184 ms at 96 workers on c7a.24xlarge, each campaign's best tested configuration. No new AWS, Aeglos, SDK/Gnark proof, phone/ARM/TEE, 250/4999, concurrent-throughput or forced-pressure qualification is added. The canonical distinct-proof ledger remains 29,255.

## Artifacts

[Corrected design](artifacts/design-corrected.md), [source recovery](artifacts/source-audit.json), [all statistics and profiles](artifacts/analysis.json), [independent review](artifacts/review.json), [qualification plan](artifacts/build-plan-rest.json), [capture plan](artifacts/run-plan.json), [pins](artifacts/pins.json), [source snapshot](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [raw receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), and [checksums](SHA256SUMS). Source archives contain exactly 356 regular members with checked extraction hashes; both JSON gzip archives roundtrip exactly. Private inputs, executable/library binaries and raw perf captures remain local.
