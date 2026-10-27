# Across-hash Poseidon with complete writes

The eight-lane component has **2.453× / 2.452× aggregate single-thread throughput** versus its two scalar controls. It wins all seven blocks in every one of the 16 cases, including original wire, slot and constraint-row writes. The formal advancement gate is **inconclusive** because three identical-control comparisons fail calibration. The positive kernel is retained; no prover integration or adoption follows from this experiment.

The private Rust kernel packs eight independent width-three hash heads into existing `Ifma8<FrConfig,true>` arithmetic. It retains scalar prefixes and the accepted first-round prelude, then writes every original wire, slot and A/B/C row at each S-box. It preserves task order within dependent chains. When chains exhaust or widths diverge, remaining work uses the old scalar implementation. Untagged/unsupported inputs retain scalar execution. Existing constants and field source are unchanged; no new arithmetic primitive, lookup table, execution plan or production dispatcher is added.

The component fixture has one Plan containing independent task ranges, three input-copy constraints before each hash, and either one hash per task or ragged chains of one/two/three hashes. Its symbolic forms come from the existing independent Python fixture. Normal/reversed product operands and present/omitted C are covered. Component-only fixtures allocate their plan and buffers before measurement; the prover architecture acquires no duplicate plan.

## Fixed comparison

Ryzen 9950X, one OS thread pinned to CPU 2. Seven rotated/reversed fresh-process blocks compare control A, control B and batch in one native/fat-LTO executable. Both controls call the same preserved scalar function; they are identical-code calibration controls, not independently rebuilt accepted-core component APIs. Timers include tag decoding, dispatch, prefixes, full arithmetic and writes. Construction, interpreter reference and checking are outside capture. Three warm calls precede each loop. A black-box mutable probe prevents removal of repeated writes.

Case keys: `g` is groups of eight tasks (1/16), `c` materializes C (0/1), `r` reverses multiplication operands (0/1), and `h` selects uniform one-hash tasks (0) or ragged chains (1). Groups 1 run 1,000 iterations; groups 16 run 64. Values below normalize complete-loop time by executed hashes. They are component throughput, not proof or individual-task latency.

|Case|A µs/hash|B µs/hash|Batch µs/hash|Batch/A paired|Batch/B paired|Wins A/B|
|---|---:|---:|---:|---:|---:|---|
|g1-c0-r0-h0|8.381611|8.383256|2.172774|0.259407|0.259258|7/7;7/7|
|g1-c0-r0-h1|8.409752|8.401937|5.063762|0.599783|0.600095|7/7;7/7|
|g1-c0-r1-h0|8.312213|8.305245|2.158467|0.259828|0.259987|7/7;7/7|
|g1-c0-r1-h1|8.315835|8.341955|5.028687|0.604023|0.590698|7/7;7/7|
|g1-c1-r0-h0|8.404092|8.409297|2.238667|0.266538|0.266447|7/7;7/7|
|g1-c1-r0-h1|8.406613|8.415199|5.108345|0.606884|0.607368|7/7;7/7|
|g1-c1-r1-h0|8.376260|8.367943|2.232554|0.266235|0.267003|7/7;7/7|
|g1-c1-r1-h1|8.389440|8.382558|5.089609|0.604814|0.618843|7/7;7/7|
|g16-c0-r0-h0|8.475028|8.433196|2.291763|0.270922|0.271466|7/7;7/7|
|g16-c0-r0-h1|8.483296|8.480944|5.355023|0.629176|0.630912|7/7;7/7|
|g16-c0-r1-h0|8.409335|8.389808|2.285294|0.270000|0.272583|7/7;7/7|
|g16-c0-r1-h1|8.402979|8.405313|5.314301|0.635226|0.635743|7/7;7/7|
|g16-c1-r0-h0|8.529408|8.536095|2.386016|0.278523|0.278932|7/7;7/7|
|g16-c1-r0-h1|8.527951|8.550083|5.430988|0.631305|0.633546|7/7;7/7|
|g16-c1-r1-h0|8.470537|8.467964|2.372119|0.280135|0.274254|7/7;7/7|
|g16-c1-r1-h1|8.470508|8.454582|5.395295|0.635777|0.638829|7/7;7/7|

Ratios pair within blocks; displayed medians are independent. Aggregate batch/control ratios are 0.407682366 and 0.407891015, with 7/7 winning block aggregates. Every speed/nonregression condition passes. The declared factor-1.05 per-block calibration fails these B/A comparisons:

- g1-c0-r1-h1 b2 B/A=1.1663251876839489
- g1-c1-r1-h1 b6 B/A=0.8561393865837574
- g16-c1-r1-h0 b1 B/A=1.1688739394729004

The failures remain included. No threshold changes, successful repeats or exclusions occur. They do not turn the measured large component gain into zero, but the declared advancement rule does not pass. This experiment does not integrate or adopt the kernel.

## Profiles and remaining work

After all 336 latency processes, three warm-FIFO-gated 999 Hz `cycles:u` profiles run the groups-16, omitted-C, normal-operand, ragged case for 2,000 iterations each. Samples are weighted by recorded period. Every capture has one PID/TID and zero lost records. No owned build, helper, hash or archive overlaps latency/profile captures.

|Mode|Samples|Lost records|
|---|---:|---:|
|control-a|4320|0|
|control-b|4303|0|
|batch|2726|0|

In the batch ragged profile, old scalar `emit_poseidon` and `run_poseidon` account for 47.772% and 23.232% of sampled cycle period. Source review confirms that divergent chain tails execute those routines. Those exclusive symbol shares identify scalar fallback work; they are not wall-time fractions or removable latency. Native kernel symbols and full sampled-symbol tables are retained. Raw root-owned recordings remain local with hashes and unchanged permissions.

A uniform hot group measures about 17.4 µs for eight hashes versus 8.4 µs for one scalar hash. High single-thread throughput therefore does not guarantee parallel latency improvement. A new parallel component should test grouping independent tasks by chain length using existing task descriptors, while preserving dependency order and ordinary-task parallelism. That changes the execution mechanism and must have a separately frozen protocol; it does not rescue this trial's failed calibration or justify an unchanged rerun.

The local component executable is `/tmp/zopro-poseidon-across8/native/probe-sdk`, SHA256 `70f19a422b444977ae9896b3dbc5a4d06d1e238149ddeb1e90cfb245d8c857f9`. Its arguments are `GROUPS C REVERSE RAGGED MODE ITERATIONS`; modes are `control-a`, `control-b`, and `batch`.

## Validation and scope

All 112 native core tests, two raw tests, six cubic tests, generator and portable/no-IFMA consumer checks pass. Three added tests map combined fixture buffers back to the original independent fixture, compare full scalar/batch outputs with the original interpreter across 272 fixture cases, and test untagged/unsupported-width fallbacks. C omission, reversed operands, prefixes, zero/one/minus-one/random inputs and ragged dependencies are included. All 339 measured processes match the original interpreter before/after capture and make zero timed allocation calls. No new field-suite run is claimed: original field code and its qualification are inherited.

Four original files only add five diagnostic cfg lines; removing them recovers every original byte. The remaining 166 originals, all 170 production files and exact accepted SDK are unchanged. There are no new SDK/Gnark exports; the proof ledger stays **6,946**. This component's heap-free runtime does not establish whole-prover peak or minimum RAM. Its fixture memory/cache layout is not a real-circuit performance result. An atomic source patch initially used a wrong declaration anchor and was corrected before building; no partial patch, compile/test/capture failure or rescue occurred.

The [latest full matrix](../2026-10-26-pairing-x-chain-sdk/README.md) retains all supplied M5 CPU/Metal comparisons; the [borrowed-G1 report](../2026-10-26-borrowed-g1/README.md) retains its experimental primary result and failed 16-worker check. This component supplies no new proof latency. The original all-eight CPU 10× goal remains active and unmet. No new AWS/Aeglos, phone, TEE, giant-circuit, swap, minimum-RAM or throughput qualification is claimed.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [native tests](artifacts/native/core/tests.json), [measurement protocol](artifacts/measurement-plan.json), [rows](artifacts/rows.json), [raw local hashes](artifacts/raw-local-only-sha256.json), [artifact hashes](artifact-index.json).
