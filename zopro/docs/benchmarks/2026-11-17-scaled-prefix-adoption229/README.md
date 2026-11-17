# Scaled-prefix CPU diagnostics and adoption

The measured scaled-prefix/uncarried-X composition is **adopted locally** in
commit `b0a41f30de4715019b06447f2991db63d48517c1`. Its exact186-file source snapshot is applied without a
relink. The [all-shape qualification](../2026-11-17-scaled-prefix-all228/README.md)
passes70/70 checks; current CPU diagnostics and both raw readers also pass.
No push has been made. The all-eight CPU10× goal remains unmet,0/8.

Primary1×2 at16 workers improves1.18% paired versus accepted194, with all three
backings positive; at32 the pooled improvement is0.56%, with the per-backing
consistency limitation retained. The32-worker process-mean median is7.634234ms
versus7.662845ms for the matched accepted control. These are small gains, not a
new absolute record or a twofold improvement. Full CPU/Metal and all-shape/RSS
tables remain in228; merge54 is reported separately without an invented M5ref.

## Actual warmed-proof CPU work

Three matched blocks at each worker count;251timed SDK calls after3primes per
plain/stat/cycles panel and arm. Ratios below are medians of three paired
ON/accepted counter readings, not confidence intervals or wall-time fractions.
Perf is FIFO-gated to the warmed SDK interval.

|Workers|Retired instructions|User cycles|Generic cache misses|Branch misses|
|---|---:|---:|---:|---:|
|1x2-w16|0.978944|0.976309|1.001462|0.989887|
|1x2-w32|0.979278|0.986547|1.001051|0.996800|

Retired instructions fall about2.1% at both counts, consistent with removing
intermediate normalization work. Generic cache misses do not improve in the
paired median. These counters do not establish a causal cache/DRAM/scheduler
explanation or prove that an isolated kernel percentage is removable proof time.

All48 PMU readings report100% scheduled. All12 user-cycle profiles qualify with
period16000000,≥1000samples each,≤5%unknown and zeroLOST. SDK/Rayon PID/TID/CPU
assignment and executable MMAP checks pass. The separate reader reconstructs
raw samples/means, ownership/counts/unknown shares, PMU numbers and observer
ratios; detailed MMAP/event parsers are shared inherited components, not an
external audit. Profile period coverage remains unvalidated as an absolute
cycle-accounting measure.

|Context / arm|Samples across3 profiles|Unknown share|
|---|---:|---:|
|1x2-w16 /REF|25310|1.628%|
|1x2-w16 /ON|24728|1.423%|
|1x2-w32 /REF|36014|1.816%|
|1x2-w32 /ON|35429|1.869%|

G1 streaming and borrowed G2 updates remain the leading point-work symbols.
The summary retains ranked symbols/shares for every context/arm. Flat shares
include inlined work and are not exclusive critical-path measurements.

## Observer quality and verification

All panels run as root with identical benchmark environment and fixed worker
affinity/default stacks. Plain panels provide a matching UID/control. Eight
paired median stat/plain or cycles/plain checks pass the prospective1.05 factor.

|Context / arm / panel|Median observer/plain|
|---|---:|
|1x2-w16 /REF /stat|1.006460|
|1x2-w16 /REF /cycles|1.005270|
|1x2-w16 /ON /stat|1.002760|
|1x2-w16 /ON /cycles|0.999076|
|1x2-w32 /REF /stat|1.002336|
|1x2-w32 /REF /cycles|1.004724|
|1x2-w32 /ON /stat|1.002524|
|1x2-w32 /ON /cycles|1.003582|

All36 fresh diagnostic exports pass independent Gnark verification,9144 SDK
calls include primes and9036 timed records are retained. All36 captures precede
decoders, verifiers and analysis; no owned CPU cojobs overlap capture. Both
readers agree on132 command receipts/264 stream hashes and the full inventory
union. No successful capture, verifier or analysis is repeated. The three-stage
227/228/229 program totals1005 fresh Gnark exports and56544 SDK calls.

Native185 and generic122 tests pass or are inherited by exact source/configuration
identity, including independent field/point oracles, tails/signs, both inverse
branches, worker stacks/capacities and integration proofs. The initial227 missing
test-fixture compile failure stays disclosed in227; no arithmetic changed in
that repair. Source allocation/scratch geometry and admission/fallback are
unchanged; no table, key copy, new heap buffer or pool is added.

The declared228 RSS rule is10%+8MiB. An additional retrospective comparison shows
all its measured peaks also fit5%+1MiB versus both controls; this is descriptive
and does not change any old campaign's gate or decision. The227 short-screen
merge54 increase remains recorded. LifetimeRSS is not minimum RAM or pressure
qualification. There is no new C7a, ARM/mobile, TEE, huge-circuit, swap or
concurrent-throughput result. Prior183/186 failures remain unchanged.

Canonical inventory now61104, SHA `40c7d386963090fd69829225e51b35a7256ee7301348e4025957ca372bfa3e6c`. Raw keys, requests, proofs,
addresses/thread streams, rawperf, full inventories and binaries remain private.

Elapsed preparation including the wait for228 was1761.10s (allowance1800s);
capture span90.39s (allowance600s), active SDK commands90.36s, and protocol to
final independent review1865.69s. Both allowances were met. These descriptive
spans use command/exit/review file timestamps; active durations use xtask receipts.
Active preparation was not separately instrumented.

[Protocol](artifacts/protocol.md), [redacted diagnostics](artifacts/summary.json),
[independent review](artifacts/independent-main.json),
[source review](artifacts/adoption-source-review.json),
[production manifest](artifacts/production-source-manifest.json),
[sources/plans](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
The broader optimization goal remains active; the next experiment must use this
accepted binary/source as its baseline and target substantially more useful work.
