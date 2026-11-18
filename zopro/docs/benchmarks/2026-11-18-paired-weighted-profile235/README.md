# Paired weighted trees: current CPU diagnostics

Current exact233 CPU diagnostics pass all profile-quality and observer checks.
The [all-shape qualification](../2026-11-18-paired-weighted-all234/README.md) passes
70/70 checks: primary latency improves0.89% at16workers and0.44% at32 versus
accepted227, with all three backing medians positive against both controls.
This small qualified gain remains eligible. Production is still accepted227
while normal native build selection and final source/resource review are
completed. No new AWS measurement or full-goal completion is claimed.

## Warmed full-proof CPU work

Three matched blocks at each worker count, REFaccepted227 and exactON233,
251 timed calls after three primes in each plain/stat/cycles panel. The same
three executable backings are reused. Perf FIFO gating brackets the warmed
SDK interval. All panels run as root with identical benchmark environment,
default stacks and fixed worker/main affinity. No relink or source change.

|Workers|Instructions ON/REF|User cycles ON/REF|Cache misses ON/REF|Branch misses ON/REF|
|---|---:|---:|---:|---:|
|1x2-w16|0.979798347|0.976361616|0.965798420|0.947689013|
|1x2-w32|0.980722229|0.978891715|0.996815901|0.954457350|

These are medians of three matched ratios, not confidence intervals,
exclusive wall-time fractions or causal cache/DRAM/scheduler evidence.
The mechanism shares inversions between two independent original256 weighted
trees without changing their joins or output order. Full-proof instruction
counts test the work-removal hypothesis beyond an isolated kernel benchmark.

All48 PMU event readings meet>=99.9% scheduled. All12 cycle profiles meet
period16000000,>=1000samples,<=5%unknown,zeroLOST and SDK/RayonPID/TID/CPU/MMAP
ownership checks. Absolute period coverage remains unvalidated. Flat symbol
shares include inlined work and are not exclusive critical-path costs.

|Context / arm|Samples across three profiles|Unknown share|
|---|---:|---:|
|1x2-w16 / REF|24617|1.499%|
|1x2-w16 / ON|24067|1.587%|
|1x2-w32 / REF|35234|1.853%|
|1x2-w32 / ON|34489|1.977%|

All eight median stat/plain and cycles/plain checks pass factor1.05.

|Context / arm / panel|Observer/plain median|
|---|---:|
|1x2-w16 / REF / stat|1.011227700|
|1x2-w16 / REF / cycles|1.008722498|
|1x2-w16 / ON / stat|1.007308505|
|1x2-w16 / ON / cycles|1.001323459|
|1x2-w32 / REF / stat|1.002296678|
|1x2-w32 / REF / cycles|1.003899806|
|1x2-w32 / ON / stat|1.003698310|
|1x2-w32 / ON / cycles|1.002753748|

All36 fresh diagnostic exports pass Gnark verification,9144SDK calls include
primes and9036timed records are retained. Capture precedes decoders, verifiers
and analysis, without owned CPU cojobs. Both readers agree on132receipts,264
stream hashes and the full proof inventory. Detailed MMAP/event parsers are
inherited shared components; the separate reader reconstructs samples/ownership/
unknown counts, PMU readings and observers. This is not an external audit.
No successful capture, verifier or analysis is repeated.

The first manual helper compile omitted the existing rustc_demangle extern
argument and failed E0433 before any SDK capture. Correcting that invocation
passes; no candidate arithmetic or successful run changes.
[Preparation note](artifacts/preparation-note.md).

Native188 and generic122 tests pass or are inherited by source/configuration
identity across233/234 and unchanged field qualification. Paired requested
capacities fit the original proof-workspace forecast. All234 peakRSS guards
pass10%+8MiB, with increases retained there; this is not minimumRAM or
physical-pressure/phone/concurrent-throughput qualification. No key table,
key copy, new pool, per-reduction OS query/lock or fixed memory cap.

The233-235 program totals1005freshGnarkexports and56544SDKcalls. Canonical
now62187; SHA-256 `38314450cd7201e88e527845db52c4de60479688cc2c7b8d0201c4d472596ff3`.

Preparation including mandatory234 wait 1520.85s (allowance1800s, exceeded=false);
capture span 89.32s and active SDK 89.29s (capture allowance600s).

All-eight CPU/Metal comparisons are in234; the full CPU10x goal remains
active and unmet,0/8. Currentb70/233C7a and fresh Aeglos results are unavailable.
Next: normal native selector integration and final source/resource review,
preserving these completed measurements and the no-minimum-gain rule.

[Protocol](artifacts/protocol.md), [summary](artifacts/summary.json),
[independent review](artifacts/independent-main.json),
[sources/plans](artifacts/source-and-plans.tar.gz), [checksums](SHA256SUMS).
Private perf data, proofs, full inventories, executable binaries, and raw
PID/TID/address streams are excluded. Publication awaits approval.
