# Bounded indexed-tile reduction

**The complete-query gate fails; this version is not adopted.** The primary single-worker G2 query improves10.75% against runtime OFF, but the primary32-worker comparison is0.999658 [0.979189,1.008486], with8/15 wins. Its two-shape32-worker aggregate is0.996474 [0.986127,1.008088], also8/15 wins. Neither establishes the required threaded gain. Production and accepted full-proof timings remain unchanged.

## Change and count screen

The new G2-only reducer greedily bundles consecutive existing signed-bucket tiles while their combined point count stays≤1,024, the original maximum tile size. Each bucket tile retains its exact point order and pair tree. The original complete affine kernel shares inversion batches across those independent trees. Safe disjoint mutable slices receive one result per original tile; original partial-bin grouping, signs, Phi/component/part merges and all ordinary windows remain unchanged.

This adds no scalar grouping, joint digit classes, changed width, point planes, point table, key/solver-plan copy or runtime pool. It differs from [whole-group GLS4 coalescing](../2026-10-29-g2-gls4-coalesced-top177/README.md), [bounded joint-top marginal reduction](../2026-10-30-joint-top-bounded175/README.md), [weighted-frontier fusion](../2026-10-31-weighted-frontier176/README.md) and [full-scalar coalescing](../2026-10-30-coalesce-query176/README.md). Their failed gates remain closed. There is one fixed geometry with no parameter sweep or post-result rescue.

The new observer preserves each existing tile output and all generic additions. It reads accepted top counts from sealed prior output; the old model/tests are not replayed.

|Raw G2 top quantity|Primary accepted / bundled|Raw2×2 accepted / bundled|
|---|---:|---:|
|Generic affine pairs|55,165 /55,165|95,796 /95,796|
|Batch inversion calls|3,295 /468|3,576 /907|
|Full eight-lane groups|5,729 /6,693|10,401 /11,557|
|Scalar-tail pairs|9,333 /1,621|12,588 /3,340|
|Existing tile outputs|448 /448|448 /448|
|Bundled jobs|58|112|
|Maximum leaf points|1,024|1,013|

Primary component/part phases have14/14/15/15 jobs; raw2×2 has28 per phase. Fewer inversion calls therefore come with less parallelism. These are noncancelling structural counts, not measured field instructions or proof costs. Raw2×2 is diagnostic because its actual query admits original whole-scalar grouping. Six new boundary patterns check empty/singleton/dense/ragged tiles, exact point/task coverage, original output populations and lane/tail conservation. The count pass permits native testing only.

## Native qualification and memory

The private native snapshot adds one module and reversible parent/API hooks:174 of176 production files are exact, and reversing the hooks recovers all176 originals. Field formulas, representations, scalar decomposition, original reducer body and solver/key code remain unchanged. [Native source](artifacts/indexed_tiles.rs), [source recovery](artifacts/source-audit.json), [design](artifacts/design.md).

All127 native core tests pass. Two new tests cover known-generator bucket sums for empty, odd, lane, tile and multiple-tile boundaries, identities, repeated/opposite points and sparse bins under1/4/16/32-worker pools with512KiB stacks; charge denial/overflow and input preservation are also checked. The native option helper defaults ON, so these core tests exercise the new integration. Their parent-API comparison can also route ON and is not an independent OFF reference; the known-generator expected sums provide the independent group result. The subsequent complete-query trial checks every output against inherited Arkworks references. No separate prover/portable/huge-circuit suite is added after the query gate fails.

Optional checked admission charges new bundle-job descriptors and worst per-worker starts/counts, vector headers and margins before allocation. Existing point/tile/partial-output arrays retain original accounting. A denied charge or overflow selects the unchanged original reducer without modifying inputs. All new leaf work is nonforking. There is no synthetic global RAM cap. Observed maximum additional reservation per indexed call is20,552bytes at one worker,270,872 at16 and537,880 at32; this is neither an allocated-memory measurement nor a full-proof/process bound. The inherited caller's `retained_payload_bytes` field stores that reservation. Every ON query records four active indexed calls.

The first build lacked an explicit RUSTC path, selected the unpinned default toolchain and failed in dependency compilation. No tests had executed. The corrected workflow pins installed rustc/rustdoc1.98.1; the child also receives its required type imports, with no algorithm change. Both source/pin sets and the failed receipt remain. The corrected full core suite and coherent release libraries build once. The artifact reader later failed on a root rlib without an rmeta alias; its separate correction handles that emitted path without rebuilding native code. These failures are tooling failures, not performance exclusions.

## Complete-query latency

Ryzen9950X,16 physical/32 logical CPUs, shared host. Seven contexts cover primary and2×2 B at1/16/32 workers and primary A32 as an unchanged negative control. Workers pin to CPU2 solo or0..workers−1 with512KiB stacks. Parsing and worker construction are outside timing; reset copies, grouping, admission, all fills/reductions/merges, normalization, allocation/drop and dispatch/join are inside.

One coherent optimized query executable supplies runtime OFF/ON through an explicit0/1 option. It is the same binary in both arms, SHA256 `52e9014ab509563e61cfc5dc3bc67deb376866678c65c2e8330adc8db57bf039`. The exact optimized accepted-source API executable `0ba4cd4e…` supplies FROZEN. Drivers/perf support are byte-identical, with O/native/fatLTO/CGU1 verified for both linked artifacts. This is a runtime-controlled experiment, not two fresh compiler builds. [Identities](artifacts/identities.json).

All21 preflights validate oracle outputs, unchanged points, route activation and affinity before315 latency callers. Fifteen fixed rotated/reversed blocks each measure17 queries after three checked primes, followed by one checked post. Every timed output is checked after recording.

|Context|FROZEN /OFF /ON ms|ON/OFF paired ratio [bounds]|Wins/15|
|---|---:|---:|---:|
|Primary B,1 worker|37.335101 /37.176105 /33.189639|0.892545 [0.891853,0.894823]|15|
|Primary B,16|3.626988 /3.469254 /3.419992|0.993566 [0.974197,1.001695]|11|
|Primary B,32|3.262190 /3.050274 /3.051567|0.999658 [0.979189,1.008486]|8|
|2×2 B,1|39.940357 /39.473916 /35.690875|0.904330 [0.902537,0.905081]|15|
|2×2 B,16|4.213999 /3.936936 /3.927830|0.996427 [0.990624,1.007203]|9|
|2×2 B,32|3.805779 /3.494342 /3.505763|1.001805 [0.981631,1.010400]|6|
|Primary A,32|1.186137 /1.169285 /1.180337|1.003093 [0.980182,1.022525]|7|

Marginal time medians need not divide into paired ratio medians. The primary single-worker gain is10.75%;2×2 single-worker gains9.57%. These positives are retained, but there is no demonstrated16/32-worker B gain against runtime OFF and no full-proof gain measurement.

Primary32 and the equal-log aggregate of both B32 contexts each require upper ratio<1 and≥10/15 wins against BOTH controls. All contexts require ON/control upper≤1.02; all OFF/FROZEN calibration intervals must fit factor1.03. Maximum ON process RSS must be≤1.05 times each control in every context. Sorted ranks4–12/15 provide nominal96.484375% marginal coverage under independent exchangeable blocks, without a familywise guarantee. Diagnostics must qualify separately. No gain floor, outlier exclusion, sample extension, changed margin, favorable subset or unchanged reroll. [Prospective protocol](artifacts/protocol.md).

There are10 failed checks: five OFF/FROZEN calibrations, primary A32 ON/OFF nonregression, primary B32 and the two-shape aggregate gains against OFF, and two RSS checks against OFF. The full list and paired observations remain in [independent review](artifacts/review.json). All threaded B fresh-OFF controls are4.94–8.53% faster by paired median than FROZEN, outside calibration. The discrepancy is unresolved and does not establish an accepted-SDK improvement or its cause. The ON/FROZEN aggregate0.920298 [0.917811,0.928814],15/15 wins, cannot be attributed to the reducer when ON/OFF is0.996474 [0.986127,1.008088],8/15 wins.

## Process memory and CPU diagnostics

|Context|Maximum FROZEN /OFF /ON RSS MiB|
|---|---:|
|Primary B,1|15.50 /15.50 /15.50|
|Primary B,16|27.25 /27.75 /27.25|
|Primary B,32|32.00 /32.25 /31.00|
|2×2 B,1|27.25 /27.25 /27.25|
|2×2 B,16|43.00 /41.75 /44.00|
|2×2 B,32|61.00 /54.00 /59.25|
|Primary A,32|24.50 /24.00 /24.25|

The2×2 threaded ON peaks exceed the declared5% limit against OFF. RSS includes fixture load, primes and allocator retention; it is not live auxiliary payload, full-proof memory, minimum RAM or pressure/concurrency qualification. The original maximum point-leaf bound does not guarantee equal process RSS.

All latency captures finish before six FIFO-gated four-event counter panels and six separate fixed16Mcycles:u profiles, each200 queries after three excluded primes. No owned build, compiler/helper, hash, archive, analyzer, verifier or second workload overlaps any capture, including preflight. Decoding follows all capture processes. Every event runs100%; all six profiles qualify, totaling21,169 samples with zero lost records and at most1.043% unknown. PID, worker TID/CPU, period and executable MMAP bindings pass. ON profiles contain the new module; OFF profiles do not.

|Panel|Instructions/query M|Cycles/query M|Generic cache misses/query M|
|---|---:|---:|---:|
|Primary16 OFF|549.625|222.676|0.735|
|Primary16 ON|499.803|198.861|0.740|
|Primary32 OFF|567.720|318.550|1.043|
|Primary32 ON|519.338|288.868|1.078|
|2×2/32 OFF|596.285|346.637|1.302|
|2×2/32 ON|549.384|318.405|1.335|

Primary32 retired instructions decrease8.52% and user cycles9.32% in these separate diagnostic panels. Inverse-symbol samples decrease277→159 there. These observations support reduced CPU work; they do not isolate the critical path or identify a cache level. The batching changes the task graph and retains fewer independent jobs, consistent with the limited threaded latency result, but causal/exclusive stage timing is not established. Symbol shares and the unvalidated absolute-period coverage are not removable wall-time fractions. No diagnostic time enters the latency gate.

## Validation and decision

An independent reader verifies228 model pins,183 native pins,277 capture pins,384 successful command receipts and the preserved initial build failure. It recovers all176 production originals, verifies the import-only child correction, reconstructs all315 process medians and paired ratios from raw caller receipts, all memory/gain/calibration checks and the six profile/counter qualifications. All348 caller processes check9,168 complete-query invocations against inherited independent Arkworks result hashes. The reader checks stored decodes and does not rerun the model, capture, old oracle or perf decoding.

The first reader used a global import substitution in its source-recovery assertion, inadvertently changing the nested test import too. Its source/failure remain. The separate corrected reader substitutes only the first occurrence and passes; no source, model output, test, measurement or original analysis is changed or repeated.

**Close this fixed version without an SDK trial or adoption.** The single-worker benefit and lower CPU service remain useful evidence. The server latency/resource gate remains false. No post-result worker selector, smaller leaf, favorable subset or revised threshold is used to promote it. The next research target must remove work on the measured ordinary-window critical path while preserving enough independent jobs; top-window inversion reduction alone did not do so here.

## Accepted CPU and Metal comparisons, inherited

Production176, accepted SDK6d83 and canonical35,171 verified proofs remain unchanged. This trial produces zero new proofs or complete SDK requests. Earlier accepted local primary observations are7.915/7.929/7.996ms, with latest8.003ms from the same binary. Inter-run variation remains unexplained; no adopted regression is established.

|Shape|Accepted local ms|M5 CPU ms /speedup|M5 Metal ms /speedup|CPU10× target ms|
|---|---:|---:|---:|---:|
|1×2|8.002770|37 /4.623×|24 /2.999×|3.7|
|2×2|11.241463|59 /5.248×|30 /2.669×|5.9|
|4×2|15.933816|109 /6.841×|48 /3.012×|10.9|
|8×2|24.234311|189 /7.799×|74 /3.054×|18.9|
|12×2|33.855565|284 /8.389×|101 /2.983×|28.4|
|16×2|40.071563|335 /8.360×|124 /3.094×|33.5|
|24×2|65.917462|528 /8.010×|176 /2.670×|52.8|
|merge-24×1|65.305023|507 /7.764×|175 /2.680×|50.7|

Source: [preceding accepted comparison](../2026-11-08-tau9-ring-census176/README.md). Seven nonprimary rows remain from the earlier full campaign. Merge54 remains162.240109ms without a supplied M5 reference. The verified ALL8 CPU10× goal remains active and unmet, **0/8**. No new AWS/Aeglos, huge-circuit, mobile/ARM/TEE, pressure/swap, minimum-RAM or concurrent-throughput result is added.

[Count output](artifacts/model-analysis.json), [capture plan](artifacts/capture-plan.json), [source/receipt/decoded-evidence archive](artifacts/evidence.json.gz), [archive index](artifacts/archive-index.json) and [checksums](SHA256SUMS) preserve the positive and negative observations and tooling failures. All UTF-8 member hashes and the gzip roundtrip are checked. Private point/scalar/key/witness/proof arrays, binaries/libraries, raw perf data, unchanged full production source and full papers are excluded. Private root: `/home/ubuntu/src/zopro-perf-work/indexed-tile-bundles176`.
