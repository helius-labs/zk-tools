# Query return versus covered serial service

**G2 contains the last covered serial-service event in all nine saved main graphs.** K/H returns last in five graphs and G2 in four; window callback returns have the same split. The query-return labels therefore do not identify which query performed the last covered service. This is an offline diagnostic, with no optimization or new proof timing.

## Attribution

The analysis reuses the fixed nine accepted175 service-trace graphs, covering rounds 0/15/30 in three processes. Logical parent IDs assign each event to its query/window; physical worker IDs identify where it ran. For each query, the table examines the interval from its last covered serial event to query return, on the worker owning that query root. A foreign service span belongs to another logical query on that same physical worker.

|Query|Median interval ms|Median foreign service-span overlap ms|Median per-graph foreign span fraction|Median fully contained foreign CPU ms|
|---|---:|---:|---:|---:|
|a|2.283676|0.872686|44.72%|0.807801|
|kh|2.530101|1.385807|53.01%|1.273776|
|g2|0.073128|0.000000|0.00%|0.000000|

Medians are calculated independently and need not add or divide exactly. Elapsed service spans include any descheduling inside the recorded leaf. Exact thread-CPU deltas are summed only for leaves entirely inside an interval; partially clipped leaves retain unknown CPU rather than a prorated estimate. Uncovered elapsed spans are neither idle time nor removable work.

K/H's median last-window-callback-to-query-return interval is only 0.059402 ms, while its last-covered-service-to-query-return interval is 2.530101 ms. Much of the separation therefore lies within callback lifetimes. The per-window analysis includes service from sibling windows of the same query as well as other queries.

|Query|Window callbacks, all graphs|Median largest per-graph service→callback-return gap ms|Median other-window span fraction in that largest gap|
|---|---:|---:|---:|
|a|198|2.535652|89.37%|
|kh|180|2.619230|83.06%|
|g2|99|0.085231|62.94%|

The last covered event is a G2 `weighted_affine` serial leaf in all nine graphs: six 256-bucket leaves and three 64-bucket leaves. This identifies the final covered event, not the exclusive bottleneck, all work on the critical path, or a recoverable proof-time fraction. Preparation, normalization, Horner assembly, coordinator work and other unrecorded service remain outside the covered set. A late query or callback can include service logically owned elsewhere.

## G2 preparation remains unresolved

The median G2 query-entry-to-input-marker interval is 2.051028 ms. Foreign service spans on its owner cover a median 0.297852 ms, with a median per-graph fraction of 14.52%. That leaves most of this interval unattributed by the existing leaf hooks. It cannot be called two milliseconds of scalar-conversion CPU or two milliseconds of removable waiting. The next useful measurement is preparation/decomposition service at the actual Rayon fold boundaries, while preserving task geometry and qualifying observer overhead.

Starting B first was already rejected in the [older query-order study](../2026-10-22-universal-primary-b-first/README.md); completing G1 queries before G2 also regressed in the [whole-query-wave study](../2026-10-28-whole-query-waves174/README.md). This analysis supplies no reason to repeat those policies unchanged. The initial bucket-range audit likewise found that the [23 October experiment](../2026-10-23-universal-g2-bucket-bands/conclusions.json) already borrowed full inputs into disjoint magnitude bands without point copies or an index plan: two/four bands regressed 4.59%/6.43% against fresh OFF. That ownership mechanism is closed.

## Fidelity and scope

All 11,943 original 80-byte records reconstruct exactly, totaling 955,440 bytes. Raw hashes match the published service-trace summaries. Original GRAPH thread lengths, zero-drop counts, per-worker slot IDs, logical parent containment, acyclic ancestry and non-overlapping nonforking serial leaves pass. Recomputed query/window boundaries and covered serial CPU totals match the earlier summaries. All 200 final source/data/helper pins close, including all 175 production sources.

Three initial decoder/interval tests pass. After inspecting the initial result, an explicitly named exploratory expansion adds the last-own-service and per-window analyses, plus one new sibling-window ownership test. The three initial tests are inherited; the new test runs separately. Original analysis, source, binaries' identities, logs and pins remain preserved, with checked relocations for files replaced by the expanded analyzer. No capture, proof or native prover suite is rerun. [Design and expansion](artifacts/design.md), [all per-graph intervals](artifacts/analysis.json), [decoded raw records](artifacts/decoded-events.json), [review](artifacts/review.json).

The earlier service-trace overhead qualification remains as originally reported. It does not make these observations unbiased or turn span coverage into a causal scheduler model. No ideal schedule, immutable latency bound, AWS/Aeglos, phone/ARM/TEE, giant-circuit or memory-pressure result follows. Zero new SDK requests, exports or verifier calls occur; the independent proof ledger stays 17,811. Production and accepted timings remain unchanged.

Publication review preserves three trailing-blank-line findings in the original test logs and their retained initial copy. Authored Rust source has no whitespace findings. A prose count of final 256/64-bucket leaves was corrected to six/three against the decoded records before publication; event data and results are unchanged.

## Inherited CPU and Metal comparison

These are accepted local measurements against supplied historical M5 references. Primary peak RSS remains 134 MiB. The accepted full-performance-gate failure and physical16 calibration engineering exception remain.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet (0/8).
