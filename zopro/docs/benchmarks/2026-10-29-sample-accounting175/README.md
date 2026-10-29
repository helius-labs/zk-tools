# Sampling-period accounting against simultaneous event counts

29 October 2026. Diagnostic of the exact accepted source-175 SDK. No source changes, new proof-latency qualification or adoption. The preceding separate-run instruction/cycle attribution had lower period totals than nonsampled counts; its passing attribution checks did not establish absolute coverage. This trial compares sample periods with explicit counters from the same request loop.

The gap persists with simultaneous counts. At 32 workers, the median sample-period/count ratio is **0.735 for cycles and 0.744 for instructions** with frequency sampling, versus **0.954 and 0.973** with fixed periods. All 24 counter panels have 100% running coverage and all attribution checks pass, but **21 of 24 accounting checks fail**. Only the three fixed-period 16-worker instruction panels meet the original 2% tolerance. The underlying mechanism remains unresolved; neither mode establishes unbiased quantitative attribution.

Use explicit counters for total executed work. The similar broad arithmetic/epoch categories across modes remain useful inspection leads, but this experiment does not supply a per-symbol correction, local CPI or a removable proof-latency fraction. The fixed-period result is retained with its failed checks; no interval or period is changed to make it pass.

## Method

Ryzen 9 9950X, 16 physical cores/32 SMT threads. Three fixed rotated/reversed blocks compare 16 workers on CPUs 0–15 and 32 on 0–31, cycles:u and instructions:u, and frequency 199 Hz versus fixed period 16,000,000. Twenty-four processes each perform 151 fresh verified SDK requests after three primes. Exact accepted executable, image, request and environment are retained. Loading, primes and export are outside FIFO measurement.

perf record launches perf stat, which launches the SDK. A blocking relay outside this process tree forwards each unchanged SDK control command to record then stat, and returns the SDK acknowledgment only after both tools acknowledge. User-only counters exclude blocked FIFO waits, but enabling/disabling is sequential, so the windows are not cycle-exact. The perf-stat monitor can also appear in the recording; its samples are retained separately. SDK periods include only the process mapped to the exact accepted SDK path.

All capture and analysis helpers and the initial publisher were compiled before corrected capture. The publication-only resumer and current-summary generator run afterward, as disclosed below. All captures precede remaining independent verification and offline analysis. Only declared perf monitors and the control relay accompany the benchmark; no compiler, analysis/hash/archive helper or independent CPU workload overlaps capture. Prior exact-pinned independently verified preflights are reused without recounting.

Capability probes found no exit READ record for a no-sample /bin/true run with -s; :S is rejected in inherited per-thread mode by this perf. The alternate relay design is declared prospectively. These capability failures are not SDK captures or proof measurements.

## Checks and observed accounting

Counters require finite positive counts/runtime and at least 99.9% running coverage. Attribution requires one mapped SDK process, correct event/mode/period metadata, valid SDK CPU affinity, zero lost or throttle records, at least 1,000 SDK samples, at most 5% unknown-symbol period and at most 0.1% monitor/foreign period. Accounting additionally requires sample-period/count in [0.98,1.02]. This empirical tolerance is not a proof that individual symbol shares are unbiased. Every panel remains included; no rerolls, exclusions, threshold changes or extensions.

All attribution panels qualified: **true**. All accounting panels qualified: **false**.

|Panel|SDK samples|Count/request|Period/count|Count coverage %|Attribution|Accounting|
|---|---:|---:|---:|---:|---|---|
|b0-w16-cycles-freq|3175|580269412.7615894|0.8814433388707402|100.0|true|false|
|b0-w16-cycles-fixed|5419|587711032.4172186|0.9770085021801372|100.0|true|false|
|b0-w16-instructions-freq|3682|1380858867.02649|0.8922525137884022|100.0|true|false|
|b0-w16-instructions-fixed|12865|1377144623.9006622|0.9898588603467912|100.0|true|true|
|b0-w32-cycles-freq|6338|831605422.5099338|0.6920790913497326|100.0|true|false|
|b0-w32-cycles-fixed|7591|839176833.6754967|0.95849210629182|100.0|true|false|
|b0-w32-instructions-freq|7466|1421302343.5298014|0.744434772040306|100.0|true|false|
|b0-w32-instructions-fixed|13027|1419005034.0860927|0.972755090858388|100.0|true|false|
|b1-w16-cycles-freq|3397|590070911.6158941|0.8848026933539264|100.0|true|false|
|b1-w32-instructions-fixed|12968|1420814783.258278|0.9671160037346002|100.0|true|false|
|b1-w32-instructions-freq|7376|1420153606.5960264|0.7300847998555248|100.0|true|false|
|b1-w32-cycles-fixed|7440|831350938.1589404|0.9482690578382542|100.0|true|false|
|b1-w32-cycles-freq|6210|841701621.7549669|0.7644386774513774|100.0|true|false|
|b1-w16-instructions-fixed|12907|1380175233.7019868|0.9909097813648726|100.0|true|true|
|b1-w16-instructions-freq|3651|1380688024.741722|0.878267961986544|100.0|true|false|
|b1-w16-cycles-fixed|5444|589048645.7549669|0.9792870016361316|100.0|true|false|
|b2-w16-instructions-freq|3641|1380429851.1655629|0.8483101374301694|100.0|true|false|
|b2-w16-instructions-fixed|12872|1377917733.6291392|0.9898417710388638|100.0|true|true|
|b2-w32-cycles-freq|6073|836292602.2052981|0.7348856592347299|100.0|true|false|
|b2-w32-cycles-fixed|7497|833055789.9006623|0.9535785184987324|100.0|true|false|
|b2-w32-instructions-freq|7267|1419784174.5960264|0.7501611817874069|100.0|true|false|
|b2-w32-instructions-fixed|13045|1418306843.3907285|0.9745787112784476|100.0|true|false|
|b2-w16-cycles-freq|3209|583082002.0198675|0.8774819845334761|100.0|true|false|
|b2-w16-cycles-fixed|5419|586100599.7417219|0.9796930352054272|100.0|true|false|

## Three-block summaries

Descriptive medians across three separate processes. Instrumented timings are retained in analysis.json but do not replace accepted performance.

|Workers/event/mode|Median count/request|Median period/count|All accounting panels qualified|
|---|---:|---:|---|
|w16/cycles/fixed|587711032.4172186|0.9792870016361316|false|
|w16/cycles/freq|583082002.0198675|0.8814433388707402|false|
|w16/instructions/fixed|1377917733.6291392|0.9898588603467912|true|
|w16/instructions/freq|1380688024.741722|0.878267961986544|false|
|w32/cycles/fixed|833055789.9006623|0.9535785184987324|false|
|w32/cycles/freq|836292602.2052981|0.7348856592347299|false|
|w32/instructions/fixed|1419005034.0860927|0.972755090858388|false|
|w32/instructions/freq|1420153606.5960264|0.744434772040306|false|

A matching aggregate total does not prove correct per-IP attribution: skid, symbol inlining, execution placement, incomplete final sampling intervals and shared-host variation remain. Separate cycle and instruction captures are not per-symbol CPI. The local SMT experiment does not establish a cause of the AWS physical-core scaling gap or predict a removable wall-time fraction. Original attribution and PMU captures remain unchanged.

## Verification and inherited performance

All 168 capture/offline steps complete. The trial contributes 3,696 internally verified SDK requests and 24 fresh 164-byte exports, independently verified by Gnark and bound to exact image/public hashes. All new identities are distinct from the prior 22,210; the new ledger is **22,234**. Eight original parser/relay tests and one padding regression pass before corrected capture, across two invocations without repeating successful tests. Source and binary are unchanged, so existing field/core/prover qualification is inherited rather than rerun. Source, receipts, proofs and complete symbol archives round-trip byte-exactly; raw perf remains local with recorded hashes.

Accepted local primary remains **8.022165 ms / 134 MiB observed lifetime peak RSS**. Latest AWS c7a.24xlarge remains Zopro 11.915591 ms at 32 workers versus Aeglos Sorted 7.959184 ms at 96. These are inherited measurements. The accepted physical-16 calibration exception remains documented. This diagnostic qualifies no AWS, phone/ARM/TEE, giant circuit, memory-pressure/swap, minimum-RAM or concurrent-throughput result.

M5 values are supplied historical references, not new matched-machine measurements. Accepted local timings below remain unchanged.

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

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The original all-eight CPU10× goal is active and unmet, **0/8**.

[Protocol](artifacts/design.md), [all results](artifacts/analysis.json), [complete symbol counts](artifacts/profile-symbols.json.gz), [verification inventory](artifacts/proof-inventory.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [proofs](artifacts/proofs.json.gz), [archive review](artifacts/publication-review.json), [preceding attribution](../2026-10-29-instruction-profile175/README.md), [consolidated accepted benchmarks](../2026-10-29-cpu-benchmark-report/README.md).

## Control-reader repair

The original first capture failed because perf writes NUL padding after an acknowledgment newline. The original reader rejected the padding on the next command; no completed SDK receipt or proof export was produced. The failed recording, counter file, workflow receipts and original pinned helpers are preserved. A separate reader accepts leading NUL padding, with one new consecutive-ack regression test passing after the original eight tests. No successful test or capture was repeated. All 24 corrected captures and 144 offline steps use separately named files and workflows with unchanged measurement criteria. All corrected helpers were compiled before those captures. The failed attempt is not counted among the 3,696 completed SDK requests or 24 exports.

[Repair source and failure records](artifacts/repair.json.gz).

Offline publication initially stopped while hashing the original failed recording, which was still root-owned. After changing ownership of that task-owned file, a separate publication-only resumer validates and reuses every completed archive. Its source compiles after captures solely to finish publication. No SDK, verifier, capture or successful test is rerun; original pinned helpers remain unchanged.
