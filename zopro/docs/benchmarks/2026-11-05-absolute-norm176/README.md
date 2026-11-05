# Absolute differences in the G2 prefix norm

The complete-caller screen **fails; the candidate is rejected**. Production and the accepted SDK remain unchanged. These are signed borrowed G2 addition-batch measurements, with no new proving-time claim.

Logical32 ON/OFF is **1.005747 [1.002050, 1.012271], 1/15**; ON/FROZEN is **1.005557 [1.001942, 1.010002], 1/15**. All calibration and individual nonregression checks pass; the four required aggregate gains fail. This implementation is closed without an SDK follow-up.

Actual G2 code grows from 51,746 to 52,047 bytes and from 8,587 to 8,654 instruction lines. Its static IFMA sites stay at 1,450 and stack-base operand lines rise from 1,475 to 1,486. These describe generated code, without isolating the cause of the regression or measuring dynamic spills.

The forward prefix pass only needs the norm of the coordinate difference. Each component's sign disappears on squaring, so the candidate uses the absolute integer difference of canonical coordinates. Each absolute difference is below p. For T=a²+b²<2p² and R=2^256, the standard Montgomery bound REDC(T)<p+T/R<2p holds because 2p<R for both BN254 primes. The original fused CIOS product/cancellation/radix/normalization body can therefore omit its final conditional 2p correction.

Leading digits decide the absolute difference's sign when unequal. Any leading-digit ties trigger an exact comparison of lower digits for tied lanes. Masked negation of signed digit differences precedes one signed carry pass. This introduces comparisons and a branch, whose cost is measured. There is no approximate-correctness assumption. It keeps the original 150-IFMA norm arithmetic rather than the earlier rejected symmetric-square variants. The general high-borrow, canonical-Y, loose-delta and product-sub trials remain closed at their original outcomes.

Only the full-vector forward norm in borrowed G2 changes. Reverse evaluation recomputes the original signed denominator from original coordinates. All point formulas, signs, scalar tails, prefix records, scratch capacities, input packing, queues, bucket geometry, solver and key representation retain their original code. No heap buffer, lookup table, cached plan, copied key, pool policy, RAM cap or C/assembly implementation is added. Compiler frames and emitted code can change.

## Qualification and protocol

The private snapshot contains 179 files: 171 of 176 originals exact, five reversible parents, and three children. All 176 original bytes recover. Original generated IFMA files and their generator remain exact. OFF and ON each pass 37 native field and 129 core tests, unfiltered. The new field tests check 32,768 paired lane cases across both primes, all 256 top-digit tie masks, boundaries through p−1, lower-word carries, zero components and random canonical values. They compare normalized raw digits to independent four-word subtraction, compare norm digits to the original fused routine and compare canonical values to Arkworks. Core tests cover signed points, tails, repeated updates and small-stack pools at 1/4/16/32 workers.

The initial compile failed on one remaining private associated helper reference in the independently located module. It was replaced with the byte-equivalent free broadcast helper before any test execution. The original source and failed command are retained; the arithmetic algorithm did not change. No successful qualification or performance capture was repeated.

FROZEN reuses the immutable accepted-source snapshot, 129-test core qualification, coherent frozen libraries and caller executable from the paired-Fq2 campaign. All 176 original source bytes, exact caller adapter, flags, transitive library/proc-macro pins and executable hash are checked before reuse. Its archived source is a byte-exact copy at publication; original build paths remain in its identity. Fresh OFF and ON use the new snapshot and separate build directories, differing by one compile-time cfg. Final links use native O3, fat LTO and one codegen unit.

Six actual caller contexts use 200/352 pairs at solo CPU 2, physical 0–15 and logical 0–31. Three checked 1,024-batch primes precede 8,192 timed changing batches per worker. Eighteen preflights precede 15 fixed rotated/reversed blocks,270 latency processes, then four profiles. Timed allocations are zero, outputs match generator scalar-sum references and outputs/capacities agree across all arms. These independent parallel batch callers do not reproduce the full proof scheduler or concurrent-proof throughput.

Ratios are ON/control; lower is faster. Rank 4..12 of 15 bounds have nominal 96.484375% marginal coverage under independent exchangeable blocks, without familywise guarantees. All six control-calibration intervals must fit factor 1.03, all ON/control upper bounds must be <=1.03, and both all-six/logical32 aggregates require upper <1 with >=10 wins against both controls. No gain floor, exclusions, extensions, unchanged retries, subset promotion or layout/attribute rescue.

|Context|FROZEN / OFF / ON us per wave|ON/OFF [bounds], wins|ON/FROZEN [bounds], wins|
|---|---:|---|---|
|g2-borrowed-signed-200-solo|7.876 / 7.875 / 7.906|1.006945 [0.996544, 1.014563], 5/15|1.002876 [0.995258, 1.013305], 6/15|
|g2-borrowed-signed-200-physical16|9.049 / 9.142 / 9.145|1.003516 [0.988757, 1.006581], 5/15|1.004769 [1.002491, 1.010650], 3/15|
|g2-borrowed-signed-200-logical32|14.680 / 14.641 / 14.737|1.003355 [0.997429, 1.009349], 4/15|1.005670 [0.999316, 1.008598], 4/15|
|g2-borrowed-signed-352-solo|13.294 / 13.283 / 13.389|1.005679 [0.998022, 1.014107], 4/15|1.007576 [0.999413, 1.014340], 4/15|
|g2-borrowed-signed-352-physical16|15.521 / 15.615 / 15.413|1.003390 [0.944736, 1.010213], 6/15|1.000819 [0.985415, 1.004348], 6/15|
|g2-borrowed-signed-352-logical32|25.149 / 25.187 / 25.323|1.006707 [1.004916, 1.015694], 0/15|1.005809 [1.004581, 1.011212], 2/15|

all6 ON/off: **1.002822 [0.997096, 1.006883], 4/15**.

all6 ON/frozen: **1.005354 [1.002306, 1.009159], 2/15**.

logical32_2 ON/off: **1.005747 [1.002050, 1.012271], 1/15**.

logical32_2 ON/frozen: **1.005557 [1.001942, 1.010002], 1/15**.

Failed checks: `["all6/on/frozen/gain","all6/on/off/gain","logical32_2/on/frozen/gain","logical32_2/on/off/gain"]`. Calibration rows and all raw ratios remain in [the results](artifacts/analysis.json).

## CPU, memory and profiles

|Context|OFF / ON thread CPU us per batch|OFF / ON max-RSS counter MiB|OFF / ON minor faults per call|
|---|---:|---:|---:|
|g2-borrowed-signed-200-solo|7.874 / 7.905|2.25 / 2.25|0.00024 / 0.00024|
|g2-borrowed-signed-200-physical16|8.856 / 8.862|2.50 / 2.50|0.00002 / 0.00002|
|g2-borrowed-signed-200-logical32|14.491 / 14.474|3.50 / 3.50|0.00001 / 0.00001|
|g2-borrowed-signed-352-solo|13.283 / 13.388|2.50 / 2.50|0.00024 / 0.00024|
|g2-borrowed-signed-352-physical16|15.075 / 15.122|6.50 / 6.25|0.00002 / 0.00002|
|g2-borrowed-signed-352-logical32|24.869 / 24.964|10.75 / 10.75|0.00001 / 0.00001|

RSS counters are diagnostic and can undercount; they are not precise lifetime peaks, minimum RAM or pressure/swap bounds. Minor faults do not measure their service time or imply disk paging. Major faults are not exported.

Four fixed 16M user-cycle profiles follow all latency. Each passes the >=1,000-sample/<=5%-unknown attribution screen, with zero LOST and one sampled process. Full symbol-period totals remain in the results. Absolute period coverage, kernel cost, local CPI and critical paths are unvalidated. See [actual static code](artifacts/codegen-summary.json) and [assembly](artifacts/assembly.json.gz); static sites/stack operands are not dynamic spill counts or causal profiles.

## Accepted CPU and Metal comparisons, unchanged

The following inherited local 32-worker SDK timings and supplied M5 references are not fresh component timings. Accepted SDK SHA256 remains `6d83f1d6fda6f16576cb351b81789e8d26a6177cc0f26f169a3dc98448c0e661`. All eight CPU 10x targets remain unmet.

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

Accepted merge54 remains 161.883899 ms without a supplied M5 reference. Historical source 175 AWS best-tested configurations remain Zopro 11.915591 ms at 32 workers and Aeglos 7.959184 ms at 96 workers on c7a.24xlarge. This adds no AWS/Aeglos, full-SDK/Gnark proof, portable, phone/ARM/TEE, 250/4999, concurrent-throughput or forced-pressure qualification. The canonical distinct-proof ledger remains 29,255 and the all-eight 10x objective stays active.

## Artifacts

[Design](artifacts/design.md), [source recovery](artifacts/source-audit.json), [results and profiles](artifacts/analysis.json), [independent review](artifacts/review.json), [qualification plan](artifacts/build-plan.json), [capture plan](artifacts/run-plan.json), [pins](artifacts/pins.json), [source archive](artifacts/source.tar.gz), [source member hashes](artifacts/source-members.json), [raw receipts](artifacts/receipts.json.gz), [publisher](artifacts/publish.rs), and [checksums](SHA256SUMS). Source archive has 356 regular members with checked extraction hashes; both JSON gzip archives roundtrip exactly. Private inputs, executable/library binaries and raw perf captures remain local.
