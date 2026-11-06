# Direct and TLS query-wrapper diagnostic

**The same-executable comparison does not support TLS as the explanation for the earlier frozen/fresh query gap.** DIRECT/TLS paired intervals fit within ±2% in both B32 contexts. The known fresh control remains faster than frozen and both entries in the new executable. This is a diagnostic result; no proving optimization is adopted and accepted full-proof timings stay unchanged.

The static audit extracts actual G2 borrowed-add and G2 layer-reduction functions from four pinned binaries: optimized frozen query, two fresh query controls and accepted SDK. Each has respectively 51,862/81,580 bytes, 8,591/13,453 instruction sites, 1,450/3,000 IFMA sites and 1,454/2,153 stack-reference sites. These are identical static counts, not proof of identical machine instructions, data addresses or runtime costs. The query API wrappers differ: original frozen directly returns the query and default metadata, while recent fresh APIs reset/read thread-local metadata.

## Complete query comparison

Ryzen 9950X, 16 physical cores / 32 logical CPUs. Four arms, all seven contexts, 15 fixed rotated/reversed blocks. Each caller has three checked primes, 17 timed queries and one final check. Times are medians of process medians; ratios are paired within blocks. Scalar resets, grouping, preparation, allocation/drop and normalization are inside; parsing and pool creation are outside. Inputs and every output match inherited complete Arkworks oracle hashes.

|Context|FROZEN ms|Known fresh ms|TLS ms|DIRECT ms|DIRECT/TLS [bounds]|Wins/15|
|---|---:|---:|---:|---:|---|---:|
|1x2-A-w32|1.181056|1.179563|1.191586|1.185495|0.993374 [0.982888, 1.006735]|10|
|1x2-B-w1|37.373230|37.092962|37.402456|37.363202|0.999273 [0.998642, 1.000406]|8|
|1x2-B-w16|3.655510|3.436147|3.668365|3.653257|1.006558 [0.995422, 1.010718]|5|
|1x2-B-w32|3.258432|3.020394|3.297667|3.270686|0.996412 [0.982218, 1.008455]|10|
|2x2-B-w1|40.005482|39.382428|40.057540|39.997207|0.998877 [0.997104, 1.000454]|11|
|2x2-B-w16|4.215967|3.927574|4.192283|4.201410|1.000505 [0.995580, 1.011441]|7|
|2x2-B-w32|3.839979|3.466724|3.868242|3.875726|1.001685 [0.992440, 1.014943]|7|

DIRECT and TLS use the exact same executable and arithmetic functions; the A negative-control labels invoke the same API. The new driver selects `D` for direct B and `B` for TLS B; original controls retain the byte-exact original driver. Consequently comparisons with original artifacts also contain a driver/call-layout difference. Same-artifact equivalence does not establish universal absence of TLS or layout effects.

The preregistered explanatory condition fails; the ±2% condition passes. At 32 workers, primary DIRECT/TLS is 0.996412 [0.982218, 1.008455]; grouped 2×2 is 1.001685 [0.992440, 1.014943]. Bounds use sorted ranks 4–12 of 15, with nominal 96.484375% marginal median coverage conditional on exchangeable blocks, not familywise. No observation is excluded, extended or rerun. Known fresh/FROZEN is 0.926947 [0.914657, 0.937000] for primary and 0.908046 [0.898062, 0.915495] for grouped 2×2. The persistent gap is not an accepted SDK gain and remains unexplained.

The immutable native design had a reversed explanatory ratio direction and an insufficient upper-bound requirement. Before any recording, the separately pinned protocol corrects this to TLS/DIRECT upper <1 with at least 10/15 wins, plus DIRECT/known-fresh lower >1 and median ≥1.03 in both B32 contexts. All sample counts, cases and diagnostics stay fixed. Native source/results do not change; no successful qualification or measurement is repeated for the clarification.

## CPU, memory and profiles

Separate warm PMU panels give the following primary32 values per query. All four events have qualified running coverage; these single panels are diagnostics, not confidence intervals or causal decomposition.

|Arm|Instructions M|User cycles M|Generic cache misses M|
|---|---:|---:|---:|
|FROZEN|568.190|323.231|1.208845|
|KNOWN_FRESH|568.191|316.592|1.044837|
|TLS|567.909|322.177|1.299300|
|DIRECT|568.008|321.192|1.177174|

Instruction counts barely differ and cycle differences are much smaller than the wall-time gap. This motivates examining task completion, synchronization and memory behavior; it does not identify a cause. Generic cache events do not identify a cache level. The four primary32 process-lifetime RSS maxima are FROZEN 32.50 MiB, KNOWN_FRESH 32.25 MiB, TLS 34.50 MiB and DIRECT 32.00 MiB. RSS includes setup and primes, not an isolated query peak or minimum RAM.

All 420 latency callers precede eight grouped PMU panels and eight separate fixed-period `cycles:u` profiles. FIFO gates exclude primes. Profiles pass actual PID, worker TID/CPU mapping, exact 16-million-cycle period, executable MMAP and zero-loss checks; all exceed 1,000 samples and unknown fractions stay below 1.2%. Total: 27,155 samples. Flat sample shares do not establish exclusive critical paths or whole-interval absolute period coverage. No compiler, analyzer, verifier, archive or second owned workload overlaps recording.

## Qualification and scope

The full unfiltered native suite passes 126 tests, including a new direct/TLS generator-oracle equivalence test at 1/4/16/32 workers with 512 KiB stacks, repeated calls and input immutability. Before the first native build, source inspection replaces unsupported Fr iterator Sum with a fold in the test oracle; no native build or test fails or repeats. Unchanged field qualification is inherited. The 176-file snapshot has 175 originals exact; only a diagnostic lib.rs suffix changes and all 176 originals recover. Arithmetic, task geometry, pools, grouping, resource policy and key data remain unchanged. Coherent libraries and actual final `-O`, native CPU, fat LTO and one-codegen-unit flags are verified.

The first complete capture has 491 workflow steps, 450 new caller processes plus 14 exact reused preflight receipts, 464 qualified rows and 12,224 checked query invocations including 70 reused calls. These are fixed component fixtures, not serialized proofs or distinct witnesses. The independent reviewer passes 813 pin checks and 495 successful commands, reconstructs all ratios, RSS and both diagnostic conditions, and independently checks 27,155 profile samples for exact ownership/CPU/period/MMAP/loss. Raw PMU parsing and symbol qualification remain in the capture helper; no second independent PMU parser is claimed.

Existing accepted full-proof profiles remain relevant: in the retained 251-request primary32 panel, borrowed G1 affine addition has 2,477/12,209 flat samples (20.29%), borrowed G2 addition 1,911 (15.65%), field inversion 869 (7.12%), G2 reduction 723 (5.92%) and G1 reduction 653 (5.35%). This is inherited evidence from the [composed-PGO full-SDK study](../2026-11-05-bundle-proof-pgo-full176/README.md), not a fresh capture. An [older instrumented trace](../2026-10-27-arithmetic-leaves/README.md) often finished K/H last but has observer/calibration limitations; it does not prove the current uninstrumented critical path. These results support investigating actual full-proof service and completion.

Accepted SDK `6d83…` and the distinct-proof ledger of 30,708 remain unchanged. No new full SDK/Gnark, AWS/Aeglos, phone, pressure/swap, minimum-RAM or concurrent-throughput result is claimed. All-eight CPU10× remains active and unmet: 0/8. Supplied historical M5 references and accepted local full-proof times follow.

|Shape|Accepted ms|M5 CPU ms|M5 Metal ms|CPU10x target ms|
|---|---:|---:|---:|---:|
|1x2|7.928660|37|24|3.7|
|2x2|11.151932|59|30|5.9|
|4x2|16.086386|109|48|10.9|
|8x2|24.179453|189|74|18.9|
|12x2|33.884455|284|101|28.4|
|16x2|40.077381|335|124|33.5|
|24x2|65.850777|528|176|52.8|
|merge24|65.060937|507|175|50.7|

Merge54 remains 161.883899 ms without a supplied M5 reference.

[Analysis](artifacts/analysis.json), [review](artifacts/review.json), [protocol correction](artifacts/protocol.md), [design](artifacts/design.md), [codegen audit](artifacts/codegen-audit.json), [codegen sources/views](artifacts/codegen-audit.json.gz), [source](artifacts/source-snapshot.hex.json.gz), [receipts/helpers](artifacts/receipts.json.gz), [raw-profile inventory](artifacts/raw-profile-inventory.json), [checksums](SHA256SUMS). Archives round-trip exact bytes, lengths and hashes. Private fixtures, keys, witnesses, executables, libraries and raw perf data remain local with hashes. Reproduction requires the pinned toolchain/dependencies and private fixtures. Private roots: `/home/ubuntu/src/zopro-perf-work/query-wrapper176` and `/home/ubuntu/src/zopro-perf-work/query-codegen-gap176`.
