# Single-state four-component G2 Horner: primary SDK pilot

The declared pilot gate is **false**. Primary at 32 workers measures **12.826593 ms ON versus 8.024429 ms accepted and 8.009300 ms fresh OFF**. Paired ON/accepted is 1.599990678, bounds [1.595540879, 1.608077903], 0/15 wins; ON/OFF is 1.600559127, bounds [1.593940669, 1.604510738], 0/15 wins. Production remains source 175/SDK 20fde; no candidate is adopted.

Failed checks: `["1x2-w32/ON/OFF","1x2-w32/ON/accepted"]`. Paired change versus accepted is +59.999% and versus OFF +60.056%; positive values here mean slower. All observations remain. A passing pilot permits separate full-scope qualification; a failed pilot closes this exact mechanism without an unchanged rerun.

## Change and scope

The candidate reuses one accepted borrowed Window through four scalar-component passes: S3, phi, S2, phi²psi, S1, phi, S0. Since phi³=1 and the maps commute, the result is S0+phi*S1+psi*S2+phi*psi*S3. Both affine and deferred projective state are transformed after pending queues drain. The old four-component study used separate component fills and three component merges per partition/window; it remains rejected. This implementation removes those merges and separate component bucket states.

One prospective geometry uses six 11-bit windows and four point partitions, with 24 fill jobs and 1,024 buckets per state. Accepted phi2 uses eleven 12-bit windows with two partitions, and a narrower indexed top. The candidate has no narrow top window. Four input traversals, scalar decomposition, collision costs and scheduling can offset fewer bucket reductions; counts do not establish CPU gains.

The old local GLS4 lattice/quotient/digit bodies are reused with source and arithmetic-oracle provenance. Historical 100,767 integer cases are inherited, not newly executed. Four fresh candidate tests recheck full-width decomposition/digits, affine/projective map composition, complete MSM generator sums including identity/opposite/zero/one/full-width cases and tails, and denied admission/checked-cost/input preservation. Native OFF/ON pass all 124/128 core tests and eight prover tests each. The 17 field files are exact, so 33 native field tests are inherited. The unchanged official generator check passes.

The 40-byte-per-scalar temporary and the entire new typed workspace geometry are conservatively charged through resource-aware analysis admission, in addition to existing future-proof forecasts. Admission/allocation failure preserves input words and uses the original phi2 path. No transformed/copied key, point lookup table, duplicate plan, new field arithmetic, pool, synthetic universal memory cap or retained per-key state is added. The temporary scalar vector is additional memory; fewer bucket sets alone do not prove a process-RAM saving. Unsupported sizes/targets retain the original implementation.

Source has 177 files: 172 of 175 originals exact, three cfg-only parent edits and two new modules. Inverse edits recover all 175. Original Window methods, solver, FFT, key and field sources remain exact. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [historical lattice oracle](artifacts/historical-integer-oracle.json), [new implementation](artifacts/source/crates/zopro-core/src/msm_affine/g2_borrowed/gls4.rs).

## Linked route and activation

The new GLS4 namespace is present only in ON. The primary ON preflight enables a diagnostic trace; its verifier wrapper requires four entry markers before executing the pinned Gnark verifier. These are the same three primes plus one preflight request, not additional exports. Timing and profiler jobs have no trace. Selected linked disassemblies are archived; code size is not dynamic work or a memory bound.

| Variant | Selected GLS4 namespace functions | Selected bytes |
|---|---:|---:|
| accepted | 0 | 0 |
| control | 0 | 0 |
| candidate | 23 | 28216 |

## Complete verified requests

On this Ryzen 9950X, every latency process uses 32 workers on CPUs 0–31, measures 151 requests after 3 primes, and exports a fresh proof for independent verification. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and ON. All 9 shapes × 3 variants pass preflight Gnark checks before latency; all 45 latency captures precede their 45 Gnark checks. No owned build, helper compilation, hashing, archiving or second workload overlaps captures. The host is shared; exclusive access is not claimed.

| Variant | Median latency ms | CPU ms/request | Median/max process RSS MiB |
|---|---:|---:|---:|
| accepted | 8.024429 | 181.544 | 130.50/133.50 |
| OFF | 8.009300 | 181.344 | 129.75/133.75 |
| ON | 12.826593 | 199.757 | 129.75/131.25 |

Process high-water RSS includes loading/primes; it is not minimum RAM. Displayed medians are not paired-ratio estimates.

| Comparison | Paired ratio [bounds] | Wins | Paired saving µs | Check |
|---|---|---:|---:|---|
| ON/accepted | 1.599990678 [1.595540879, 1.608077903] | 0 | -4814.186 | FAIL |
| ON/OFF | 1.600559127 [1.593940669, 1.604510738] | 0 | -4808.806 | FAIL |
| OFF/accepted | 1.000670508 [0.996065125, 1.002498748] | 6 | -5.380 | pass |

Accepted SHA256 `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` and fresh OFF `20fde19fcfc2f06bb5c640a0ed0fd313fa73af8d03ed46e5df7fdc377791e99f` are byte-identical executables at different paths. Pilot primary gains require upper paired ratios below 1 and at least 10/15 wins against both controls; OFF/accepted calibration bounds must lie within [1/1.03,1.03]. Sorted paired-ratio ranks 4–12 of 15 give nominal 96.484375% per-comparison coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, changed threshold or fastest-layout selection is used.

## CPU profiles

After latency verification, three FIFO-gated four-event stat runs (31 requests) and three separate 199 Hz cycles:u profiles (251 requests), after 3 primes each, precede their six Gnark checks and decoding. FIFO gates exclude loading/primes. Diagnostic times do not enter the latency gate.

| Stat variant | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w32-OFF | 825.200 | 1423.430 | 3.018 | 1.203 |
| arith-1x2-w32-ON | 929.638 | 1745.380 | 2.737 | 1.255 |
| arith-1x2-w32-accepted | 830.914 | 1418.576 | 2.800 | 1.191 |

| Profile variant | Samples | Lost records |
|---|---:|---:|
| cycles-1x2-w32-OFF | 9170 | 0 |
| cycles-1x2-w32-ON | 9354 | 0 |
| cycles-1x2-w32-accepted | 9008 | 0 |

All stat events run at least 99.9%; each profile contains one SDK PID and zero lost records. Symbol aggregates include surrounding work and inlining; cycle-period shares are not removable wall time or an isolated cost. No exclusive memory/cache/scheduler cause is established.

The candidate retires about 22.6% more instructions than OFF in the single diagnostic counter run. The scalar G2 `Projective::add_affine` symbol rises from 54 OFF samples (0.626% of period) to 932 ON samples (13.848%); accepted has 52 samples (0.826%). The median per-process ratio `(user_cpu_ms + system_cpu_ms) / loop_wall_ms` falls from 22.453 OFF to 15.427 ON, with accepted at 22.451. This ratio describes aggregate logical CPU time, including coordination; it is not physical-core occupancy or an exclusive query cost. More CPU work and less overlap accompany the regression, despite fewer component merges. These observations do not isolate a queue policy, top window, map, cache, or scheduling cause.

The exact six-ordinary-window implementation is closed. A distinct diagnostic question is which component/window generates deferred projective additions. This capture has no such counters, so neither a top-window attribution nor a proposed repair is claimed.

## CPU and Metal scope

The fresh candidate primary comparison follows; M5 values are supplied historical references, not new M5 captures.

| Primary variant | Measured ms | M5 CPU ms | CPU speedup | M5 Metal ms | Metal speedup |
|---|---:|---:|---:|---:|---:|
| accepted | 8.024429 | 37 | 4.611× | 24 | 2.991× |
| OFF | 8.009300 | 37 | 4.620× | 24 | 2.997× |
| ON | 12.826593 | 37 | 2.885× | 24 | 1.871× |

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

All 165 workflow steps complete. All **7,902 SDK requests** verify internally; all **78 fresh 164-byte exports** independently pass Gnark, exact image/public binding and freshness against 17,473 prior identities. The ledger is **17,551**, with 156 proof/public members representing 78 proofs. Native libraries, shared proc macros, helper/driver, source 177 and all image/cache pins close. Full source including non-Rust files, commands, observations, proofs and decoded profiles are archived. Three owned raw perf recordings and binary/rlib payloads stay local with hashes. No successful capture is repeated.

No new portable/ARM/mobile/TEE, physical 16 latency, giant 250/4999, concurrent-proof, swap, active-pressure, minimum-RAM or AWS/Aeglos result is claimed. [Accepted production](../2026-10-29-balanced-kh-adoption/README.md) remains unchanged. [Design](artifacts/design.md), [history audit](artifacts/history-audit.md), [analysis](artifacts/analysis.json), [build/tests](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json).

Publication review retains eleven whitespace findings in raw outputs and original source, preserving those bytes; authored files have none. The profile/concurrency interpretation was added after report generation from the archived analysis. Captured data and pinned inputs are unchanged. The final report and archive are covered by `SHA256SUMS`.
