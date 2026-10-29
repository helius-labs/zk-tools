# Frozen SDK stage entry/return diagnostic

29 October 2026. This records exact function boundaries in the accepted source-175 SDK without changing or rebuilding the prover. Production remains unchanged; these are instrumented observations, not a new proof-latency qualification. The original eight CPU 10× targets remain unmet.

At 32 workers, the traced median prefix through product completion is **2.182 ms**, followed by **5.324 ms** through canonical completion. G2 returns after G1 in 155/217 traced requests, with a median return gap of only **0.0107 ms**. The measured phases suggest investigating the prefix as well as MSM work, while avoiding an assumption that G2 alone controls latency. Both parallel topologies fail the declared probe-overhead check, so these are observations under instrumentation.

## Method

Linux uprobes and return probes attach at nine unique native symbols, using exact ELF program-header file offsets: SDK prove, solve_inner, canonical-wire solve, solver, product computation, G1 canonical MSM, G2 exclusive query, scalar grouping and balanced K/H feed. Targets and the accepted executable SHA are pinned. Probe names occupy one unique owned group, removed after captures on success or failure; other groups are untouched. Entry/return records carry PID/TID, CPU and nanosecond timestamps. No debug or arithmetic source is replaced.

Ryzen 9 9950X; the same 1×2 image, request and accepted environment at one worker on CPU 2, 16 workers on physical CPUs 0–15 and 32 workers on logical CPUs 0–31. Three independent Gnark preflights precede capture. Seven fixed rotated/reversed blocks compare plain and traced processes; each makes 31 requests after three primes. FIFO control enables tracing only for the request loop. All 42 captures finish before probe cleanup, ownership transfer, remaining verification, decoding and lost-record audits. No owned compiler, analysis/helper/hash/archive or second CPU workload overlaps capture. The original helpers were compiled beforehand; an offline reader repair after capture is documented below.

## Instrumentation overhead

Values are medians of seven process medians. Paired ratios need not equal ratios of displayed medians. Extrema of seven paired ratios give nominal 98.4375% individual median coverage under independent/exchangeable blocks, without a familywise guarantee. An interval outside [1/1.03,1.03] prevents treating that topology as unperturbed scheduling. It does not invalidate retained instrumented observations. No exclusions, extensions, threshold changes or capture reruns are used.

|Workers|Plain ms|Traced ms|Paired traced/plain|Bounds|Within factor 1.03|
|---|---:|---:|---:|---|---|
|1|95.780472|95.716950|0.998941945|[0.996121783, 1.000236370]|true|
|16|9.318502|10.139739|1.044096260|[0.830575045, 1.106500465]|false|
|32|8.020676|8.225231|1.029462841|[0.997750683, 1.047691649]|false|

The one-worker overhead interval qualifies. The 16-worker interval is wide and the 32-worker upper bound exceeds 1.03; neither parallel topology qualifies as unperturbed scheduling. No observations are discarded or recaptured.

## Inclusive stage intervals

These are per-span medians over the traced requests at each topology. Nested and parallel intervals overlap and must not be added. K/H feed is only part of its query. Missing symbols or residual intervals are unobserved, not zero. A function-entry timestamp is not a scheduler-ready timestamp, and a late return does not isolate a CPU, cache or scheduling cause.

|Stage|1 worker ms|16 workers ms|32 workers ms|
|---|---:|---:|---:|
|sdk|95.708748|10.180405|8.208445|
|canonical|94.877294|9.387925|7.360660|
|solver|1.791281|1.026532|1.164270|
|product|3.038487|0.535758|0.571816|
|g1|14.554105|7.367663|5.238196|
|g2|40.073343|7.551318|5.305102|
|groups|0.270659|0.589419|0.905333|
|kh_feed|0.705878|0.737187|1.024298|

Full per-request start/end offsets and thread/CPU IDs remain in analysis.json. Query returns can be compared with canonical completion, but any remaining tail includes uninstrumented K/H reduction, output work and runtime effects. It is not automatically assigned to one component.

|Workers|Prefix through product end ms|Product end to canonical end ms|Canonical end minus last G1/G2 return ms|G2 returned after G1 / 217|
|---|---:|---:|---:|---:|
|1|5.270818|89.738324|0.005681|217|
|16|1.902149|7.565997|0.011422|200|
|32|2.181675|5.323728|0.012374|155|

These columns are separate medians and need not sum to the displayed SDK median. The prefix begins at SDK entry and includes solver, public correction, canonical conversion, scalar extraction and product work. Query end is the canonical-solve return. In traced runs, solver intervals fall from 1.791 ms at one worker to 1.164 ms at 32; product intervals fall from 3.038 to 0.572 ms. This gives a reason to examine limited parallelism before the query join. It does not establish a removable serial fraction or predict a speedup, particularly with the failed parallel overhead checks. [Derived interval summary](artifacts/interval-summary.json).

## Verification and limits

All 21 recorded processes have one sampled SDK PID, zero lost records, complete entry/return stacks and exactly 31 nonoverlapping SDK intervals each: 651 request intervals. Every retained span belongs to exactly one SDK interval. Six parser tests cover nanosecond timestamps, CPU/TID fields, parallel/nested spans, unmatched entry/return rejection and proof-receipt binding. Accepted field/core/prover qualification is inherited because no source or executable changes.

The trial makes 1,440 internally verified SDK requests and exports 45 fresh 164-byte proofs, independently Gnark-verified and bound to the expected public input and image. Every export is distinct from the prior 22,144 identities; the new ledger is 22,189. Source, receipts and proof archives round-trip byte-exactly. Raw perf recordings remain local with hashes. This diagnostic does not adopt code, claim minimum RAM or qualify concurrent-throughput, memory-pressure, mobile/ARM/TEE or AWS performance.

The original offline parser stopped on a perf `# cmdline` header containing the event-group name. The separately compiled repair skips comment headers and uses the saved captures and verification receipts. Its added regression test passes; the six original parser tests remain recorded. No SDK request, successful test, capture or Gnark verification is repeated. Original pinned helpers remain unchanged. [Repair record](artifacts/analysis-repair/failure.md), [repaired reader](artifacts/analysis-repair/analysis-fixed.rs), [regression result](artifacts/analysis-repair/header-test.log), [probe cleanup](artifacts/cleanup.json).

## Accepted prover comparisons

Accepted local 1×2 remains 8.022165 ms and 134 MiB observed peak RSS. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96. Those are inherited measurements. The supplied CPU/Metal references and all nine local/AWS rows remain in the [current summary](../2026-10-29-current-cpu-summary/README.md); no new latency result replaces them. The accepted physical-16 calibration engineering exception and separate final-artifact qualification remain documented.

Research this turn rechecked [SRE](https://arxiv.org/abs/2610.27452), [Hrabar](https://uwspace.uwaterloo.ca/items/43cc12b9-cb03-42bd-a01c-4b0ed5e12403), [Noël–Plantard](https://link.springer.com/article/10.1007/s13389-026-00393-z) and [binary/ternary joint sparse forms](https://eprint.iacr.org/2008/285). Local history already contains scalar reuse, memory-band and mixed-width studies. No paper speedup or implementation is transferred to this diagnostic.

[Protocol](artifacts/design.md), [all intervals and checks](artifacts/analysis.json), [ELF targets](artifacts/targets.json), [verification inventory](artifacts/proof-inventory.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [archive review](artifacts/publication-review.json).
