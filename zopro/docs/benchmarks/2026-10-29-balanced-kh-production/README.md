# Clean balanced K/H prover: final-artifact qualification

Primary at 32 workers measures **8.022165 ms candidate versus 8.127910 ms accepted**. The paired reduction is **1.131%**. ON/accepted is 0.988688677, bounds [0.983266114, 0.991334550], with 15/15 wins. The complete current performance gate is **false**; the separately declared engineering-assessment gate is **true**. Production remains unchanged and the candidate is not adopted by this report.

Current failed checks: `["1x2-w16/OFF/accepted"]`. The [preceding private-candidate qualification](../2026-10-28-balanced-kh-qualification175/README.md) remains **failed** at `1x2-w16/OFF/accepted`; this new artifact does not relabel that result. [Portable/resource checks](../2026-10-29-balanced-kh-compatibility175/README.md) also remain separate evidence.

## Source, builds and method

The clean candidate removes the private balanced cfg and the optional trace statement. Its child module is byte-exact to rustfmt of the tested child after removing that statement. The parent adds one private module declaration and a width-13 fused K/H dispatch after existing input assertions. Of 174 accepted files, 173 remain byte-exact; reversing the parent edits and removing the child recovers all 174. There are 175 candidate files. The balanced 12/13-bit windows retain existing field arithmetic, borrowed queues, complete exceptions and conservative allocation forecasts. No key table, duplicate plan, scheduler, pool, image or public API change is made. Production source is unchanged during assessment.

Fresh native control and candidate use the same 17 accepted flags, without the private balanced flag. Fresh candidate native core/prover suites pass 124/8 tests; portable x86-64/no-IFMA/no-ASM suites pass 101/8, including both balanced scalar/point oracles. Control 122/8 and field 33-native/11-portable evidence are inherited only where source is unchanged. All transitive libraries and shared proc macros are frozen before standard eight-library SDK links. This is qualification of the actual clean artifact, not a repeat of the previous private binary.

Ryzen 9950X: all nine shapes use 32 workers on CPUs 0–31; primary also uses 16 distinct physical cores on CPUs 0–15. Fifteen fixed rotated/reversed blocks compare accepted, fresh OFF and clean ON in separate processes. Primary processes measure 151 requests, others 31, after three primes. Witness solving, fresh randomized proving, SDK verification and serialization are timed; image loading is excluded. No exclusive-host claim is made.

Before latency, 13 portable processes cover all nine basis shapes and primary/54 raw/denied preparation, followed by 30 native preflights. Every preflight export verifies independently before latency. All 450 latency captures precede their 450 Gnark checks. Six FIFO-gated stat runs and six separate cycle profiles then precede their 12 verifications and decoding. No owned build, helper, hash, archive or second workload overlaps captures. There is no diagnostic trace in the clean prover; route evidence is the checked source/control flow.

## Complete verified requests

Times are medians of fifteen process medians, not paired-ratio estimates. CPU is aggregate measured-loop CPU per request; RSS includes loading/primes and is not minimum RAM.

| Case | Accepted ms | Fresh OFF ms | Clean ON ms | ON CPU ms | ON RSS median/max MiB |
|---|---:|---:|---:|---:|---:|
| 1x2-w32 | 8.127910 | 8.166508 | 8.022165 | 182.119 | 130.50/134.00 |
| 2x2-w32 | 11.342834 | 11.364746 | 11.335969 | 262.076 | 263.50/275.00 |
| 4x2-w32 | 16.036953 | 16.094512 | 15.945544 | 416.348 | 306.50/315.00 |
| 8x2-w32 | 24.198098 | 24.149981 | 24.090485 | 666.117 | 485.25/512.50 |
| 12x2-w32 | 33.741328 | 33.808636 | 33.844855 | 936.335 | 695.25/707.25 |
| 16x2-w32 | 40.137942 | 40.064335 | 39.931557 | 1120.456 | 793.25/826.25 |
| 24x2-w32 | 66.379476 | 66.475511 | 66.233309 | 1893.989 | 1246.39/1328.75 |
| merge-24x1-w32 | 65.444730 | 65.093360 | 65.273224 | 1800.729 | 1121.75/1175.50 |
| merge-54x1-w32 | 161.689371 | 161.679509 | 162.250464 | 4435.826 | 1733.00/1831.87 |
| 1x2-w16 | 9.883403 | 9.982890 | 9.793302 | 119.522 | 86.75/91.00 |

| Case | ON/accepted paired [bounds] | Wins | ON/OFF paired [bounds] | Wins |
|---|---|---:|---|---:|
| 1x2-w32 | 0.988688677 [0.983266114, 0.991334550] | 15 | 0.985236314 [0.977220954, 0.989581578] | 15 |
| 2x2-w32 | 0.996400211 [0.990186738, 1.005370288] | 8 | 0.992028672 [0.986983947, 1.006999979] | 9 |
| 4x2-w32 | 0.995279991 [0.987874556, 1.014297584] | 11 | 0.990510134 [0.978638669, 1.008886403] | 10 |
| 8x2-w32 | 0.990882035 [0.984579971, 0.997204295] | 12 | 0.996211623 [0.990227070, 1.006365199] | 9 |
| 12x2-w32 | 1.008473809 [0.998019518, 1.023376054] | 5 | 1.001765125 [0.990560064, 1.014684493] | 7 |
| 16x2-w32 | 0.992090574 [0.987358173, 0.998856098] | 13 | 0.995712298 [0.989821247, 1.002499480] | 10 |
| 24x2-w32 | 0.996551748 [0.988666258, 1.000174707] | 11 | 0.996090220 [0.986587619, 0.999106325] | 13 |
| merge-24x1-w32 | 0.995049352 [0.990098635, 1.000699645] | 9 | 1.003622476 [0.994019937, 1.008061121] | 6 |
| merge-54x1-w32 | 1.009116977 [0.989926783, 1.013464681] | 5 | 1.004841322 [0.994181667, 1.010564194] | 6 |
| 1x2-w16 | 0.986211087 [0.968328815, 1.009731567] | 11 | 0.968367377 [0.938845440, 0.997358900] | 12 |

Primary32 requires upper candidate ratios below 1 and at least 10/15 wins against both controls. Physical16 requires both upper ratios at most 1.02; other32 cases at most 1.03. Every full-screen baseline calibration interval must fit [1/1.03, 1.03]. The separate engineering-assessment gate excludes only physical16 OFF/accepted calibration, as declared before capture; it does not turn a failed full gate into a pass. Passing it alone does not adopt the candidate or achieve the original goal.

| Case | OFF/accepted paired [bounds] | Calibration |
|---|---|---|
| 1x2-w32 | 1.005130932 [1.001771427, 1.007272360] | pass |
| 2x2-w32 | 1.001775203 [0.989972188, 1.007795731] | pass |
| 4x2-w32 | 1.002289649 [0.988509512, 1.011521132] | pass |
| 8x2-w32 | 0.994889602 [0.991373267, 1.003570553] | pass |
| 12x2-w32 | 1.007433540 [0.996888753, 1.016026398] | pass |
| 16x2-w32 | 0.999645256 [0.991619569, 1.006379575] | pass |
| 24x2-w32 | 1.000463339 [0.996636911, 1.008483208] | pass |
| merge-24x1-w32 | 0.990087812 [0.987557042, 1.001991003] | pass |
| merge-54x1-w32 | 1.001087224 [0.989784747, 1.006736778] | pass |
| 1x2-w16 | 1.007659425 [0.985180801, 1.037588560] | FAIL |

Accepted SHA256 is `81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20`; fresh OFF is `81033d7786e14821cb76494de7faa2ebcc9e40c3222708372b21861fdeb56c20`; they are byte-identical executables at separate paths. Independent process streams and all fifteen observations remain. Bounds are paired-ratio ranks 4–12 of 15, nominal 96.484375% coverage under independent exchangeable blocks, without a familywise guarantee. No minimum gain floor, exclusion, sample extension, threshold change, fastest-layout selection or successful-capture repeat is applied. Small positive and negative estimates remain in the tables without being rounded to zero.

## Memory and diagnostics

| Case | Accepted RSS median/max MiB | OFF RSS median/max MiB | ON RSS median/max MiB |
|---|---:|---:|---:|
| 1x2-w32 | 131.25/135.00 | 132.00/135.75 | 130.50/134.00 |
| 2x2-w32 | 257.50/271.25 | 259.75/271.50 | 263.50/275.00 |
| 4x2-w32 | 300.75/313.25 | 303.25/317.25 | 306.50/315.00 |
| 8x2-w32 | 487.50/506.50 | 494.00/504.50 | 485.25/512.50 |
| 12x2-w32 | 692.75/710.25 | 698.00/717.00 | 695.25/707.25 |
| 16x2-w32 | 790.09/835.25 | 793.50/823.75 | 793.25/826.25 |
| 24x2-w32 | 1251.09/1291.84 | 1234.64/1313.00 | 1246.39/1328.75 |
| merge-24x1-w32 | 1126.40/1156.36 | 1125.45/1160.25 | 1121.75/1175.50 |
| merge-54x1-w32 | 1758.85/1899.85 | 1737.62/1841.68 | 1733.00/1831.87 |
| 1x2-w16 | 85.50/90.25 | 86.75/89.50 | 86.75/91.00 |

No input-sized buffer or retained key table is added; that is not a claim of identical process peaks or a general memory reduction. The earlier eight configured memory-limit cases had zero high/max/OOM events and did not establish behavior under active throttling. Those cases are not new measurements of this clean binary.

Stat runs measure 31 requests and cycle profiles 251, after three primes. FIFO gates exclude loading and primes; diagnostic times do not enter latency decisions.

| Diagnostic | Cycles/request, million | Instructions/request, million | Generic cache misses/request, million | Branch misses/request, million |
|---|---:|---:|---:|---:|
| arith-1x2-w16-OFF | 595.296 | 1412.914 | 2.090 | 1.137 |
| arith-1x2-w16-ON | 567.526 | 1377.991 | 1.910 | 1.075 |
| arith-1x2-w16-accepted | 566.288 | 1407.680 | 1.955 | 1.124 |
| arith-1x2-w32-OFF | 854.200 | 1453.476 | 3.140 | 1.238 |
| arith-1x2-w32-ON | 829.385 | 1422.482 | 3.008 | 1.193 |
| arith-1x2-w32-accepted | 861.908 | 1455.091 | 3.157 | 1.275 |

All stat events run at least 99.9%. The six profiles contain 43953 samples, one SDK PID each and zero lost records. Complete per-profile PID/TID sets and symbol weights are retained. These observations do not isolate removable wall time or establish a cache, DRAM, scheduler or external-preemption cause.

## Fresh CPU and Metal comparisons

M5 values are supplied historical references, not new M5 captures. Ratios use the fresh 32-worker medians.

| Shape | Accepted ms | ON ms | M5 CPU ms | CPU/accepted | CPU/ON | M5 Metal ms | Metal/accepted | Metal/ON |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| 1x2 | 8.127910 | 8.022165 | 37 | 4.552× | 4.612× | 24 | 2.953× | 2.992× |
| 2x2 | 11.342834 | 11.335969 | 59 | 5.202× | 5.205× | 30 | 2.645× | 2.646× |
| 4x2 | 16.036953 | 15.945544 | 109 | 6.797× | 6.836× | 48 | 2.993× | 3.010× |
| 8x2 | 24.198098 | 24.090485 | 189 | 7.811× | 7.845× | 74 | 3.058× | 3.072× |
| 12x2 | 33.741328 | 33.844855 | 284 | 8.417× | 8.391× | 101 | 2.993× | 2.984× |
| 16x2 | 40.137942 | 39.931557 | 335 | 8.346× | 8.389× | 124 | 3.089× | 3.105× |
| 24x2 | 66.379476 | 66.233309 | 528 | 7.954× | 7.972× | 176 | 2.651× | 2.657× |
| merge-24x1 | 65.444730 | 65.273224 | 507 | 7.747× | 7.767× | 175 | 2.674× | 2.681× |

Candidate reaches 0/8 CPU10× thresholds; accepted reaches 0/8. The original all-eight goal remains **active and unmet**. Primary requires 3.7 ms. No M5 reference was supplied for 54→1, whose complete observations remain above. No current AWS/Aeglos comparison follows from local data.

## Verification and provenance

All 1,028 workflow steps complete. All **28,000 SDK requests** verify internally. All **505 fresh 164-byte exports** independently pass Gnark, exact public/key binding, member hashing and uniqueness against 16,500 previous identities. The ledger reaches **17,005**; 1,010 archived proof/public members represent 505 proofs. Raw portable images are verified with their corresponding Euler verifying image; both identities are retained. Every closing source, library, helper, executable, image and cache pin matches.

All source files, including non-Rust manifests/locks/licenses/shaders, fresh build/test/link receipts, command logs, observations, exported proofs and decoded profiles are retained. The preserved initial setup failures precede all successful builds/tests/captures; their fixes concern a field-receipt schema assertion and formatting of old test imports, with no runtime algorithm variant. Six raw perf files remain local with hashes; ownership changes only for those owned files after capture. No successful capture is repeated. No new phone/ARM/TEE, giant250/4999, concurrency, swap, forced-pressure or AWS result is claimed. Production is unchanged and any engineering adoption decision remains separate.

[Protocol](artifacts/final-protocol.md), [analysis](artifacts/analysis.json), [identities](artifacts/identities.json), [build completion](artifacts/native/result.json), [source audit](artifacts/source-audit.json), [proof inventory](artifacts/proof-inventory.json), [capture completion](artifacts/runs/result.json), [closing pins](artifacts/pins.json).
