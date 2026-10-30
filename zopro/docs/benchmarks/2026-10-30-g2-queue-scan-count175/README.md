# Bounded scan of the G2 collision queue

30 October 2026. An ordinary bucket-scheduling count model using two real B-query scalar populations. The necessary opportunity check passes, selecting a separate valid-curve caller experiment. **No latency or proving-speed gain is measured, and production is unchanged.**

## Mechanism

The current borrowed G2 queue drain stops at its first busy destination. A bounded descending scan instead swap-removes free entries and leaves blocked entries for later. A moved tail entry has already been inspected and remains busy because busy bits only become set during the drain. The scan starts after execute with an empty pending batch; at most199 queued entries enter an existing200-entry batch. It adds no queue, buffer, point table, point copy or sort and preserves width12, two point partitions and batch200. A native implementation is not yet installed.

This differs from the [earlier affine queue drain](../2026-10-29-queue-drain175/README.md), which changes flush_queue and retains the original early-stopping take_from_queue. The [routed-round trial](../2026-10-24-g2-routed-rounds/README.md) instead creates index lists and changes task geometry; it remains rejected. No earlier failed gate is rewritten.

## Counts and tradeoffs

The frozen accepted solver supplies one fresh scalar solution for each eligible input. The model reconstructs original grouping and stable classes, uses source-extracted Phi/Booth recoding, and counts all20 ordinary window-part jobs per shape. The special indexed top window is excluded. Every bucket sum matches a separate direct accumulation under two deterministic full-width field-token streams; all scalar and digit reconstructions match. Token checks preserve algebraic scheduling semantics and do not test real curve exceptions.

|Shape|B rows|Terms after grouping|Policy|Projective arrivals|Projective accumulator adds|Finish merges|Affine updates|Batch inversions|Drain inspections|
|---|---:|---:|---|---:|---:|---:|---:|---:|---:|
|1x2|30089|30089|lifo|1178|749|429|516476|2605|31310|
|1x2|30089|30089|scan|617|503|114|517037|2610|40089|
|2x2|52274|28852|lifo|376|16|360|535224|2688|30612|
|2x2|52274|28852|scan|284|13|271|535316|2689|30927|

For1×2,561 projective arrivals and315 final merges are avoided, while561 affine updates, five batch inversions and8,779 busy checks are added. For2×2,92 projective arrivals and89 final merges are avoided, with92 affine updates, one inversion and315 checks added. The fraction of all primary arrivals moved from projective to affine is only about0.10%; this is not evidence of a2× breakthrough. Costs per operation differ and the model supplies no measured conversion to time. All queue/pending maxima remain within199/200; the candidate max queue is44 versus77 in the2×2 control. Every per-job count and full/tail batch histogram is retained.

The necessary criterion requires fewer primary modeled projective arrivals with exact bucket sums and unchanged capacity limits; it imposes no minimum positive-gain floor. It passes. More inversions and metadata work could outweigh the saved projective arithmetic. The next qualification must include actual curve identities, doubling, cancellation, scratch capacities, the complete caller and CPU profiles before deciding whether to advance. No positive subset can adopt production from this model.

## Fidelity, tests and scope

The ordinary model tracks whether a bucket has received a first value and assumes no cancellation or same-x exception. Formal field tokens do not change occupancy when their sum is zero. Actual key points, exceptional branches, field multiplication/inversion cost, normalization, weighted/top reductions, allocation overhead, proof scheduling and concurrency are excluded. Counterfactual point intermediates could change exception counts. The two shapes' outputs are modeled schedules, not runtime traces of a changed prover.

Primary grouping declines;2×2 grouping admits28,852 classes from52,274 original rows. These decisions and dimensions match the inherited count records. Original B dimensions on the other seven supplied fixtures exceed65,535 and fail the source guard before a specialized Phi route can be chosen. Those cases are explicitly ineligible, not measured improvements. Existing resource admission and every actual execution path still require qualification for an implementation.

Two new model tests pass on their first executions. One covers220 synthetic geometries across empty/short/large inputs, repeated/sign-mixed destinations and batch sizes8/16/32/200, checking every bucket, operation conservation and queue/batch bounds for both schedules. The other isolates a blocked tail hiding a free entry. Existing field/core/prover suites are inherited without repetition because no prover source or arithmetic changes. The two fresh solver calls provide new count inputs; no proof or independent Gnark verification occurs. No witness/scalar/point arrays are written to disk.

The initial test compilation rejected include! in an impl body before a test executed. A separate generated impl wraps the exact original LIFO method outside the model impl; arithmetic and scheduling rules are unchanged. Initial source, failed receipt and pin/plan revisions are retained. The corrected four-step workflow completes compile-tests, model tests, compile-probe and count-real-inputs. No successful test/count/build repeats. All frozen-library/input pins and175 production source hashes are checked.

## Accepted proof comparisons

The following local proof timings and supplied historical M5 references are inherited, not count-model performance. The all-eight verified CPU10× goal remains active and unmet0/8.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted local primary remains8.022165ms/134MiB;54→1 remains162.250464ms/1831.87MiB without a supplied M5 reference. Latest AWS primary remains Zopro11.915591ms at32 workers versus Aeglos Sorted7.959184ms at96; no new AWS measurement. The physical16 accepted calibration exception remains recorded. No mobile/ARM/TEE, huge-circuit, memory-pressure/swap, minimum-RAM or concurrent-throughput claim. Ledger23,971 unchanged. [Current comparison](../2026-10-29-current-cpu-summary/README.md).

[All modeled counts](artifacts/analysis.json), [prospective design](artifacts/design.md), [model](artifacts/model.rs), [original LIFO method](artifacts/lifo-method.rs), [scalar extraction and oracles](artifacts/probe.rs), [reused recoding](artifacts/split.rs), [command receipts](artifacts/receipts.json.gz), [archive reconstruction](artifacts/archive-review.json), [source/input review](artifacts/review.json), [corrected workflow](artifacts/plan-fixed.json).
