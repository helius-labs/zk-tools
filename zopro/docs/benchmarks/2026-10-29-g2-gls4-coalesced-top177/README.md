# Coalesced GLS4 top reduction: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **8.796095 ms ON versus 8.032045 ms accepted and 8.074154 ms fresh OFF**. Paired ON/accepted is 1.099716599, bounds [1.092476192, 1.103742258], 0/15 wins; ON/OFF is 1.089901813, bounds [1.083514492, 1.099249241], 0/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +9.972% and versus OFF +8.990%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

The candidate changes only the GLS4 highest window from per-bucket tiled trees to the existing group_affine reduction. Each tree level batches ordinary pairs across 256 adjacent sign-class buckets, amortizing inversion work. Lower five eleven-bit Horner windows, four partitions and sequential components, 1,024 magnitude bins, 2,048 sign classes, phi/compound/phi maps and merges remain the parent algorithm. group_affine, complete-curve exceptions, generic/prefix arithmetic, Window and all field code remain exact. This changes pair grouping, point copies and the task graph, with no width or flag sweep.

The [tiled-top parent](../2026-10-29-g2-gls4-indexed-top177/README.md) remains rejected at 10.325850 ms versus 8.036072 ms accepted in its own run. Its profiles motivated testing inversion amortization, without proving an exclusive top cost. Earlier full ordinary-window tree/prefix/gather trials also remain rejected; they explicitly preserved narrow-top handling. This experiment does not repeat their ordinary-window routing.

All seven GLS4 tests pass: six parent tests rerun, including signed 65-bit extrema and full MSMs in custom 1/4/16/32-thread pools. One added skew/complete-curve test covers empty/odd inputs, many equal/opposite/identity points and distant sign bins under 1/4/32-thread pools. Fresh OFF/ON pass all 124/131 core tests and eight prover tests each. Exact 17 field sources inherit 33 field tests. Historical 100,767 lattice integer cases remain inherited. The official generator check passes.

Optional admission conservatively retains the old six-output shape and four top parents' routing/output/sign/component arrays. It adds workers times the original nonforking leaf scratch, collective workspace_grouping(N,32), 32 local group stack arrays and the 40-byte-per-scalar vector. Across four parts and eight groups each, point ranges are disjoint, so retained per-init maximum point capacities sum to at most N. Existing typed grouping bounds cover pair capacity growth, generic/prefix records and allocator margins. Point limb copies are temporary, charged memory. Dynamic Reservation::analysis and scalar try_reserve_exact preserve original words and fall back on denial/failure. No retained key setup, lookup table, new runtime pool, field formula or synthetic global RAM cap.

Source 177 retains 172 of 175 original files exact, three cfg-only parent edits and two new modules; inverse edits recover 175. Relative to the frozen tiled-top parent only gls4.rs and its workspace estimate change. Lattice, field, solver, FFT, key and original MSM/Window methods remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [implementation](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs), [cost](artifacts/source/crates/zopro-core/src/msm_affine/workspace.rs).

## Linked route and activation

New GLS4 symbols occur only in ON. The primary ON preflight requires four COALESCED entry trace markers before the pinned Gnark check and timing; those are the standard three primes and one preflight request. Timing/profiles have no trace. Selected disassemblies establish linked structure, not dynamic operation counts or speed.

| Variant | Selected GLS4 functions | Selected bytes |
|---|---:|---:|
| accepted | 0 | 0 |
| control | 0 | 0 |
| candidate | 24 | 40878 |

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 8.032045 | 181.532 | 130.25/132.75 |
| OFF | 8.074154 | 181.934 | 130.50/133.75 |
| ON | 8.796095 | 191.584 | 129.75/134.25 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.099716599 [1.092476192, 1.103742258] | 0 | -800.088 | FAIL |
| ON/OFF | 1.089901813 [1.083514492, 1.099249241] | 0 | -723.414 | FAIL |
| OFF/accepted | 1.006298280 [0.995982374, 1.020865794] | 6 | -50.535 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` are byte-identical executables at different paths. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 821.696 | 1418.868 | 2.760 | 1.185 |
| arith-1x2-w32-ON | 875.985 | 1508.276 | 3.324 | 1.272 |
| arith-1x2-w32-accepted | 830.132 | 1418.956 | 2.769 | 1.190 |

| Profile variant | Samples | Lost records |
|---|---:|---:|
| cycles-1x2-w32-OFF | 8983 | 0 |
| cycles-1x2-w32-ON | 9391 | 0 |
| cycles-1x2-w32-accepted | 9072 | 0 |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.

The grouped top also changes when the existing adaptive prefix path is selected: `reduce_layer` uses it when at least 32 ordinary pairs share a layer. Coalescing changes those layer sizes. This is a combined topology/kernel-dispatch experiment, not an isolation of inversion count.

| Variant | G2 projective add_affine sampled period | G2 reduce_layer sampled period | Fq inverse sampled period |
|---|---:|---:|---:|
| accepted | 0.824% | 5.444% | 8.037% |
| OFF | 0.386% | 6.542% | 6.545% |
| ON | 2.975% | 3.714% | 5.836% |

ON's lower affine-reduction and inverse shares accompany higher total work: 1,508.276 million instructions/request versus 1,418.868 million OFF, and 875.985 million cycles versus 821.696 million. These are separate stat observations. Median per-block aggregate CPU-time/loop-wall ratios are 22.353 accepted, 22.300 OFF and 21.564 ON; these are logical CPU-time ratios, not physical core counts. The accepted/OFF profile differences also show why individual sampled shares cannot establish a small speedup.

The coalesced variant approaches the accepted implementation more closely than the tiled-top parent in its separate historical run, but the current paired comparison still loses every block against both controls. Ordinary-top, tiled-top and this coalesced-top GLS4 implementations remain rejected. Further local layout, width or annotation changes are not justified by this result. A different future reduction study should start from the accepted two-component path and a new operation/task model, rather than promoting a GLS4 variant on an improved intermediate metric.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 8.032045 | 37 | 4.607× | 24 | 2.988× |
| OFF | 8.074154 | 37 | 4.583× | 24 | 2.972× |
| ON | 8.796095 | 37 | 4.206× | 24 | 2.728× |

The complete all-eight accepted source 175 comparison is inherited below; it is not a candidate all-eight performance qualification or a new measurement of the other cases. All-nine preflights establish correctness, not their latency.

| Shape | Accepted ms | M5 CPU ms | CPU/accepted | M5 Metal ms | Metal/accepted | CPU 10× required ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.022165 | 37 | 4.612× | 24 | 2.992× | 3.7 |
| 2x2 | 11.335969 | 59 | 5.205× | 30 | 2.646× | 5.9 |
| 4x2 | 15.945544 | 109 | 6.836× | 48 | 3.010× | 10.9 |
| 8x2 | 24.090485 | 189 | 7.845× | 74 | 3.072× | 18.9 |
| 12x2 | 33.844855 | 284 | 8.391× | 101 | 2.984× | 28.4 |
| 16x2 | 39.931557 | 335 | 8.389× | 124 | 3.105× | 33.5 |
| 24x2 | 66.233309 | 528 | 7.972× | 176 | 2.657× | 52.8 |
| merge-24x1 | 65.273224 | 507 | 7.767× | 175 | 2.681× | 50.7 |

The all-eight CPU 10× goal remains **active and unmet**. Accepted 175 reaches 0/8; this candidate has only one fresh latency row and no all-eight qualification. Primary requires 3.7 ms. Accepted 54→1 remains 162.250464 ms without a supplied M5 reference. The accepted physical 16 baseline-calibration exception remains recorded; this pilot does not rerun or relabel it.

## Verification and provenance

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,630 prior identities. The ledger is **17,708**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 177 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).

Publication review retains 11 whitespace findings: ten raw test/perf log lines and an inherited Cargo.toml blank line. None is newly authored source. The report interpretation was added after generation and hashes were regenerated; captured data and pinned implementation files remain exact. The top.rs and top-tests.rs fragments are inherited parent references; the selected implementation is the archived source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs.
