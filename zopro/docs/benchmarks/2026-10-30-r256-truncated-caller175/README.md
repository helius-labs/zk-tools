# R256-only truncated reduction in MSM batch callers

30 October 2026. **Rejected at the complete-caller gate; no SDK trial or adoption.** The all-context aggregate improves 1.197% against fresh OFF, but the six 32-worker contexts regress 2.024%, with only 2/31 winning block aggregates. Both borrowed-G1 sizes at 32 workers exceed the 3% nonregression bound. All control calibrations pass.

This is a new representation-specific integration of earlier checked arithmetic. Its failed generic gate remains unchanged. Caller timing is not proof latency; accepted production and all proof results remain unchanged.

The candidate dispatches only native eight-lane R256 products to truncated full-radix reduction. R260, four-lane, scalar and portable arithmetic retain accepted routines. The negative-inverse constant and product body match the [earlier checked implementation](../2026-10-24-truncated-redc-component/README.md) exactly. That generic trial improved R256 components but regressed R260 curve expressions and remains rejected. This independent trial measures complete G1 and G2 batch callers, including surrounding inversion, field packing and point finishing. There is no new heap buffer, point-key copy, persistent table or execution plan; the inverse constant is 40 bytes per field/radix. Fixed vector temporaries can change stack/register pressure.

## Measurements

Ryzen 9950X, 16 physical cores and 32 logical CPUs. Solo uses CPU 2, physical16 uses CPUs 0–15, and logical32 uses CPUs 0–31. Thirty-one fixed rotated/reversed blocks compare the exact frozen caller, fresh OFF and ON in all 18 contexts. Each worker executes 8,192 changing batches after three checked 1,024-call primes. Common wave wall time includes the slowest worker and barrier tail. Loading, fixture construction and output checks are outside timers. Full generator scalar-sum checks, matching digests/capacities and zero timed allocations are required for every process. These synthetic valid-curve fixtures are not SDK proof distributions. No owned build, helper, hash, archive, analyzer or independent workload overlaps performance capture.

Times are microseconds per complete batch wave; paired ratios need not equal ratios of displayed marginal medians.

|Caller/context|Frozen µs|OFF µs|ON µs|Paired ON/OFF|Bounds|Wins/31|
|---|---:|---:|---:|---:|---|---:|
|g1-owned-200-solo|4.115521|4.120992|3.993636|0.968229054|[0.964808421, 0.975164894]|31|
|g1-owned-200-physical16|4.626848|4.632920|4.514020|0.973177565|[0.971171754, 0.975355425]|31|
|g1-owned-200-logical32|6.867840|6.882118|6.965471|1.012338351|[1.010780298, 1.019470492]|2|
|g1-owned-352-solo|6.737329|6.729947|6.515929|0.972342716|[0.961609452, 0.975032809]|31|
|g1-owned-352-physical16|7.641884|7.700853|7.457660|0.972784288|[0.971405987, 0.974280492]|29|
|g1-owned-352-logical32|11.421520|11.446949|11.619536|1.014990782|[1.006828468, 1.018352849]|3|
|g1-borrowed-signed-200-solo|4.331179|4.337088|4.227082|0.970313602|[0.969230928, 0.979860630]|31|
|g1-borrowed-signed-200-physical16|4.862267|4.856338|4.771276|0.978483526|[0.976632948, 0.981222701]|31|
|g1-borrowed-signed-200-logical32|7.063644|7.086778|7.312945|1.031396937|[1.023559924, 1.039144072]|2|
|g1-borrowed-signed-352-solo|7.096264|7.093085|6.910804|0.977181124|[0.967976074, 0.980398519]|31|
|g1-borrowed-signed-352-physical16|8.096276|8.092453|7.920941|0.980684421|[0.977121860, 0.984492890]|30|
|g1-borrowed-signed-352-logical32|11.836783|11.771176|12.177634|1.034225079|[1.028155456, 1.039029626]|1|
|g2-borrowed-signed-200-solo|8.465092|8.475042|8.218741|0.967117778|[0.959550207, 0.977184531]|31|
|g2-borrowed-signed-200-physical16|9.834747|9.899417|9.616359|0.971983190|[0.967442244, 0.976600470]|30|
|g2-borrowed-signed-200-logical32|15.920406|15.983739|16.273216|1.017085506|[1.014207791, 1.020115041]|1|
|g2-borrowed-signed-352-solo|14.249700|14.240455|13.867265|0.972084304|[0.965059293, 0.981027505]|31|
|g2-borrowed-signed-352-physical16|16.905699|16.880251|16.419273|0.973936975|[0.971173777, 0.976331292]|30|
|g2-borrowed-signed-352-logical32|27.550371|27.652951|27.914275|1.016229004|[1.011233896, 1.021488947]|2|

All 18 fresh/frozen calibration intervals must fit factor 1.03; every ON/control nonregression upper bound must be ≤1.03. Both the all-18 and logical32-six equal-log aggregates must have upper bounds <1 and ≥21/31 wins against both controls. Bounds use ranks 10–22/31, with nominal 97.055% individual coverage conditional on independent/exchangeable blocks, without a familywise guarantee. There is no gain floor, exclusion, sample extension, changed threshold or subset promotion.

|Aggregate|Ratio|Bounds|Wins/31|
|---|---:|---|---:|
|all18/on/frozen|0.988508107|[0.987656631, 0.990415263]|31|
|all18/on/off|0.988032011|[0.983830592, 0.990223254]|31|
|logical32_6/on/frozen|1.022652425|[1.017135200, 1.025236513]|2|
|logical32_6/on/off|1.020240372|[1.018251205, 1.023095983]|2|

Failed checks: `["g1-borrowed-signed-200-logical32/on/frozen","g1-borrowed-signed-200-logical32/on/off","g1-borrowed-signed-352-logical32/on/frozen","g1-borrowed-signed-352-logical32/on/off","logical32_6/on/frozen/gain","logical32_6/on/off/gain"]`. All observations, positive/negative estimates and control comparisons are retained in [analysis](artifacts/analysis.json). This failed integration is closed without a full SDK trial.

The direction changes consistently with topology: every solo and physical16 ON/OFF estimate improves, while all six logical32 estimates regress with bounds above parity. The result supports judging arithmetic in its complete threaded caller. It does not isolate SMT contention, instruction dependencies, register pressure or another hardware effect as the cause. The new reduction has more static IFMA work: sampled caller disassemblies contain 872 versus 800 sites in G1 and 1,558 versus 1,450 in borrowed G2. These are static instruction sites, not dynamic counts or proof of the regression's cause. [Code audit](artifacts/codegen.json).

## Correctness, profiles and memory

Fresh OFF passes 33 field and 128 core tests; ON passes 35 field and 128 core tests, unfiltered. The two new tests cover both primes and inclusive 2p, lazy and radix boundaries in eight-step ordinary/fused/difference chains. They compare normalized lazy digits exactly with accepted CIOS and check canonical exports and ranges: 131,072 result lanes across both radices. Test-only counters require 8,192 R256 dispatches and zero R260 dispatches; the instrumentation is absent from library builds. Existing independent Arkworks field tests run in both variants. The old 108,800-case integer model and coefficient argument are inherited for the unchanged helper. No full-proof, portable or pressure qualification follows.

Frozen caller source reverses exactly to all 175 accepted files after removing diagnostic exports; its coherent libraries and executable are reused without rebuilding or rerunning tests. The fresh snapshot has 177 files: 172 originals are exact, three have reversible parent edits, and two children are new. This prototype uses a private include; template/generator integration has not been completed.

Four fixed-period `cycles:u` profiles follow all latency: borrowed G1/200 and G2/200, logical32, OFF/ON, with 262,144 batches per worker. FIFO acknowledgments exclude primes and output checks. Captures precede ownership changes and decoding. All four exceed the required 1,000 samples, stay below 5% unknown period, contain one sampled PID and have zero lost records. Every sample has period 16,000,000 and a TID from the recorded worker/caller set. Time and CPU fields are recorded. Symbol shares and static counts do not establish intrinsic instruction latency, exclusive costs or removable proof wall time. Full symbol periods remain in analysis; raw profiles stay local with hashes.

|Profile|Samples|Unknown period|Sampled worker TIDs|Lost|
|---|---:|---:|---:|---:|
|G1/200 OFF|15,574|0.058%|32|0|
|G1/200 ON|15,991|0.044%|32|0|
|G2/200 OFF|34,080|0.029%|32|0|
|G2/200 ON|34,206|0.058%|32|0|

The candidate keeps heap capacities; observed lifetime RSS/capacities in raw rows are not minimum RAM, compiler-stack bounds or memory-pressure guarantees. Frame/register effects remain part of measured caller cost. No per-key lookup expansion is introduced.

The native workflow initially stopped after detecting a temporary edit to its pinned design.md while a separate profile requirement was being drafted. The original protocol bytes were restored, additions moved to capture-design.md before capture, and the unchanged native plan resumed with successful field/core/library steps reused. Both attempts and the failure are retained. No successful suite, link or timing is rerun to replace an outcome.

All 1,748 capture/offline workflow steps complete. The 1,732 capture processes execute 344,665,664 kernel calls including primes, preflights and profiles, with matching output/capacity signatures and zero timed allocations. These are batch calls, not proof requests. A separate [offline review](artifacts/offline-review.json) reproduces all 54 latency cells, 54 paired intervals, four aggregates and all six failed checks from original outputs. It checks all 777 pins, all 177 archived source members and 8,808 receipt members byte for byte, plus profile periods, worker TIDs, loss and symbol accounting. It reruns no test, benchmark, proof or verifier. Interpretation and this review were added after capture; original pinned helpers remain unchanged.

The exact [inherited helper text](artifacts/inherited-helper.txt.gz) is compressed after publication; decompression matches the original bytes, including its trailing blank line. This changes storage only. The original publisher still emits the plain-text copy.

## Accepted CPU and Metal comparison

The inherited accepted table is unchanged. M5 numbers are supplied historical references, not new matched-hardware measurements. Accepted local 1×2 RSS remains 134 MiB; its physical16 calibration engineering exception remains explicit in the [complete snapshot](../2026-10-30-current-bench-snapshot175/README.md).

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37|4.612×|24|2.992×|3.7|
|2x2|11.335969|59|5.205×|30|2.646×|5.9|
|4x2|15.945544|109|6.836×|48|3.010×|10.9|
|8x2|24.090485|189|7.845×|74|3.072×|18.9|
|12x2|33.844855|284|8.391×|101|2.984×|28.4|
|16x2|39.931557|335|8.389×|124|3.105×|33.5|
|24x2|66.233309|528|7.972×|176|2.657×|52.8|
|merge-24x1|65.273224|507|7.767×|175|2.681×|50.7|
|merge-54x1|162.250464|—|—|—|—|—|

All eight CPU 10× targets remain unmet; no M5 reference exists for 54→1. This caller trial adds zero SDK requests or Gnark exports; the distinct-proof ledger remains 25,231. Latest AWS primary remains 11.915591 ms at 32 workers for Zopro versus 7.959184 ms at 96 workers for Aeglos Sorted. No new AWS, Aeglos, mobile/ARM/TEE, huge-circuit, swap, pressure or concurrent-throughput result is claimed.

[Protocol](artifacts/design.md), [capture declaration](artifacts/capture-design.md), [history audit](artifacts/history-audit.md), [source audit](artifacts/source-audit.json), [all observations/profiles](artifacts/analysis.json), [source archive](artifacts/source.json.gz), [workflow receipts](artifacts/receipts.json.gz), [pins](artifacts/pins.json), [review](artifacts/publication-review.json), [checksums](SHA256SUMS).
