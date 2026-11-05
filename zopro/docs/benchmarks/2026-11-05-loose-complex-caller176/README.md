# Deferred square correction in the complete G2 caller

The candidate does not improve the complete borrowed-G2 caller. Across six contexts, its paired latency ratio is **1.004742 [1.002919, 1.005709] against the frozen control** (3/15 wins) and **1.002952 [0.997156, 1.005756] against fresh OFF** (5/15 wins). All six control calibration checks and all twelve candidate nonregression checks pass. All four required aggregate gain checks fail. **No adoption or full-SDK trial follows.**

This closes the fixed caller implementation, while preserving the parent arithmetic qualification. It is a different algorithm from the previous signed/symmetric square candidate. No unchanged rerun, threshold change, or favorable worker subset is used to rescue either result. The all-eight CPU10× goal remains active and unmet, 0/8.

## Change and qualification

The [qualified loose-square helper](../2026-11-05-loose-complex-square176/README.md) computes the real component `a²−b²` through carry-only operands `S=a+b`, `D=2p+a−b`, postponing modular correction until after the original unsigned Montgomery product. The correlated bound `SD=(a+p)²−(b−p)²≤9p²` permits the original final correction. The imaginary product/doubling is unchanged. The ordinary field/Fq2 representation, forward norms, inversion, slope, final Y, tails, exceptions, points, queues, grouping, schedules, and scratch forecasts remain original.

Only the native FAST borrowed-G2 reverse-loop real square changes. OFF keeps the original expression verbatim under the inverse private cfg; unsupported and FAST=false paths retain their original arithmetic. The field child is byte-exact to the qualified parent. Its successful model and field tests are inherited, not rerun.

Both OFF and ON pass **129 unfiltered core tests**, including generator, sign-mask, tail, reuse, and thread-pool cases. All six native test/library/freeze steps succeed on their first execution. The snapshot contains178files:171 original files exact, five reversible parent changes, and two children. Removing the edits recovers all176 original hashes. Production and the accepted SDK remain unchanged.

Libraries are coherent and frozen separately. Both direct-link receipts include `-O`, `target-cpu=native`, `lto=fat`, and `codegen-units=1`. FROZEN is the existing original accepted caller adapter, whose source/library/qualification receipts are checked; it is not relabeled as a fresh build.

## Matched complete-caller measurements

Ryzen9 9950X: soloCPU2, physical16 CPUs0–15, logical32 CPUs0–31. Workers pin individually; the coordinator remains unpinned. These are independent synthetic curve-kernel workers, not a full MSM, proof schedule, or concurrent-proof throughput measurement.

Eighteen32-call preflights cover two pair counts, three topologies, and three arms. Native core tests cover tails. Fifteen rotated/reversed matched blocks measure8192calls per worker after three checked1024-call primes. The wave timer includes common synchronization and complete calls, including inversion/packing/scatter. Complete generator-scalar output checks occur after worker clocks; all output/capacity signatures agree and timed allocation counts are zero.

| Pairs | Workers | FROZEN µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF [bounds] | Wins/15 |
|---|---:|---:|---:|---:|---|---:|
| 200 | 1 | 7.877332 | 7.856437 | 7.909681 | 1.009123 [0.996180, 1.014253] | 4 |
| 200 | 16 | 9.029604 | 9.126222 | 9.077082 | 1.005098 [0.988785, 1.008828] | 5 |
| 200 | 32 | 14.691099 | 14.763569 | 14.681329 | 0.998074 [0.992656, 1.000214] | 11 |
| 352 | 1 | 13.294541 | 13.297468 | 13.389963 | 1.004736 [0.998654, 1.012586] | 4 |
| 352 | 16 | 15.582057 | 15.411098 | 15.479106 | 1.011783 [1.000346, 1.017495] | 3 |
| 352 | 32 | 25.156106 | 25.162320 | 25.149257 | 1.002619 [0.994774, 1.015307] | 7 |

Latencies are medians of15wave observations. Paired ratios are computed within blocks, not by dividing those medians.

| Equal-log aggregate | Paired ratio [bounds] | Wins/15 | Gain gate |
|---|---|---:|---|
| All six, ON/FROZEN | 1.004742 [1.002919, 1.005709] | 3 | fail |
| All six, ON/OFF | 1.002952 [0.997156, 1.005756] | 5 | fail |
| Logical32 two, ON/FROZEN | 1.001798 [1.000834, 1.005833] | 2 | fail |
| Logical32 two, ON/OFF | 1.001264 [0.994986, 1.003958] | 6 | fail |

The declared gain gates require upper<1 and at least10/15wins against both controls for both aggregates. Calibration intervals must fit[1/1.03,1.03]; candidate nonregression upper bounds must be≤1.03. Bounds are ordered ratios4–12of15, nominal96.484375%marginal under independent/exchangeable-block assumptions, without a familywise guarantee. All observations remain included. Against FROZEN the aggregate bounds lie above parity; against OFF they span parity. This is not evidence that every individual context is slower.

All317capture commands succeed once:296caller processes comprising18preflights,270latencies, four counter panels, and four profiles, plus checking/decoding commands. Measured latency covers36,126,720kernel calls; measured calls including diagnostics/preflights total62,088,384, plus15,040,512priming calls, for77,128,896total. These are not proofs. No owned build, helper, hashing, archive, verifier, or second workload overlaps capture.

## CPU and memory observations

All latency blocks precede grouped counters and then fixed16Mcycles:u profiles. FIFO gates exclude primes and final validation, while synchronization/accounting edges remain. Each counter panel has100%running coverage. Per200-pair batch, divided by workers×rounds:

| Workers | OFF instructions | ON instructions | OFF cycles | ON cycles |
|---|---:|---:|---:|---:|
| 16 | 111,169.518 | 109,968.504 | 41,503.867 | 41,917.072 |
| 32 | 111,169.522 | 109,968.512 | 60,983.938 | 61,240.784 |

The candidate executes about1.08%fewer instructions, yet neither diagnostic panel has fewer cycles. These single panels have no repeated-panel uncertainty estimate. Cache-miss counts differ sharply between panels and are retained in the raw data; they do not establish a cache-level, RAM, or execution-port explanation. Fewer instructions alone did not predict a latency gain.

Linked borrowed-kernel static counts are1450IFMA sites in all arms; FROZEN/OFF have8587instruction lines and ON8555. Static instruction sites are not executed counts, spill traffic, stack bounds, or a causal speed model.

| Profile | Samples | Unknown period | Sampled workers | Qualified |
|---|---:|---:|---:|---|
| 16 OFF | 10,869 | 0.0276% | 16 | yes |
| 16 ON | 10,944 | 0.0274% | 16 | yes |
| 32 OFF | 31,465 | 0.0445% | 32 | yes |
| 32 ON | 31,858 | 0.0408% | 32 | yes |

All85,136samples pass executable/PID/FORK, workerTID/CPU, fixed user event/period, and zero-loss checks. Every worker is sampled; no nonworker sample occurs. Attribution requires≥1000samples and≤5%unknown. Symbol attribution includes inlined work; absolute sampling coverage and exclusive removable wall costs are unvalidated.

Scratch capacity remains16,000bytes per worker at200pairs and28,160bytes at352, plus3,200/5,632fixture reference bytes. Maximum process RSS across the15logical32latency processes is FROZEN/OFF/ON4.25/4.00/4.00MiB at200pairs and11.25/11.25/11.25MiB at352. These counters include setup/fixture/allocator behavior, may undercount, and establish neither minimum RAM nor a rigorous lifetime memory peak. No new source-level heap/table/key-copy/plan/pool allocation is introduced. Device, stack, resource-pressure, and portable qualification remain unperformed.

## Review and decision

The independent Rust reviewer imports no campaign/parser code. It checks **596pins and325successful workflow commands**, protocol/default normalization, stream hashes, source recovery, exact qualified field child, original OFF expression, frozen libraries/actual optimized link argv, all270integer latency observations, every gate/aggregate, CPU/RSS/capacity accounting, and separately parsed profile ownership/CPU/period/loss. Review succeeds on its first execution. No successful test, link, or capture is repeated for bookkeeping.

Close this implementation at the failed caller gate. No new SDK call, proof export, Gnark verification, AWS/Aeglos comparison, or large-circuit/phone/ARM/TEE/pressure/swap/minimum-RAM result is claimed. The accepted SDK and29,953-entry proof ledger remain unchanged. Further research needs a distinct reduction in dominant useful work rather than an unchanged rerun of this square.

## Unchanged accepted full-proof comparison

Inherited accepted32-worker local results and supplied M5 references, not measurements of this candidate. Source: [accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). These different machines do not establish equal hardware efficiency. The latest separate full-SDK experiment remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), retaining its positive primary estimate and failed full gate.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54x1 remains161.883899ms without a supplied M5 reference. The all-eight CPU10× objective remains unmet,0/8.

## Artifacts

[Protocol](artifacts/capture-design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source edits](artifacts/edits.json), [reproduction](artifacts/reproduce.md), [artifact index](artifact-index.json), [checksums](SHA256SUMS).

The gzip JSON receipt archive preserves relative paths, exact UTF-8 bytes, lengths, and SHA256s; extraction is verified. Sources, native/link/capture receipts, decoded profile metadata/events/loss, counters, and linked assembly are included. Executables, libraries, raw perf.data, and private key/image/witness/input arrays are excluded.
