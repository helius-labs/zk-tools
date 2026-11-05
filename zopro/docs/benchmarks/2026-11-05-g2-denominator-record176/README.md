# Retained G2 denominator and reconstructed X sum

The complete-caller gate fails. The four-vector denominator record is not adopted and does not advance to a full-SDK trial. All six OFF/FROZEN calibration checks pass; five of twelve ON nonregression checks and all four aggregate gain checks fail. Original source176, acceptedSDK6d83, and the all-eight CPU10× objective remain unchanged.

- all6/on/frozen: **1.017899132 [1.012060405, 1.019607825]**, 1/15 wins.
- all6/on/off: **1.016550886 [1.013436279, 1.020154964]**, 0/15 wins.
- logical32_2/on/frozen: **1.030525573 [1.027369907, 1.034753948]**, 1/15 wins.
- logical32_2/on/off: **1.027569278 [1.023571514, 1.036134374]**, 0/15 wins.

## Change and qualification

The forward pass already computes D=Bx−Ax. The candidate retains its two components with prefix/norm and reconstructs Ax+Bx=2Ax+D in the reverse pass, avoiding that pass's Bx load/pack and denominator subtraction. It pays an extra field addition and extra record traffic. Only native FAST borrowed-G2 arithmetic selects this formula. Original OFF expressions remain verbatim under inverse cfg; FAST=false retains original arithmetic. Field source/representation, norms, inversion, slope, square, finalY, tails, signs, exceptional selection, points, queues, grouping, and scheduling are unchanged.

This differs from the [rejected six-vector X cache](../2026-10-29-g2-cached-x175/README.md): that stores both Ax and Bx and removes both repacks. Here Ax is repacked and four record vectors are used. Neither result supplies a positive speed claim for the other. The shared record type also pads owned-G2 prefix records with two zero slots under ON; that coupled layout cost is disclosed rather than described as unchanged owned storage. No new heap vector, key table, point-array copy, duplicate execution plan, pool, arbitrary RAM cap, or unsafe operation is added.

Fresh OFF and ON each pass **130unfiltered core tests** on their first execution. One new case checks retained denominators against scalar field differences, all changing signed generator outputs through zero/full/tail lengths and scratch reuse, and actual capacity against the existing nine-vector-per-group allowance. Existing small-stack threaded/sign-mask/tail tests also execute. All17field sources are byte-exact and inherit qualification; field suites are not rerun. The177-file snapshot has172exact original files and four reversible originals; all176original hashes recover. The caller adapter changes only two capacity-type expressions to account for actual record size.

Native6steps and direct optimized links2steps succeed once. Actual link argv includes-O/target-cpu=native/lto=fat/codegen-units=1 and coherent frozen dependency/procmacro sets. FROZEN is the independently pinned existing accepted caller adapter, not a fresh build relabeled as the original SDK.

## Fixed complete-caller measurements

Ryzen9 9950X,16physical/32logical CPUs. Solo usesCPU2, physical16 uses0–15, logical32 uses0–31. Workers pin individually; the coordinator stays unpinned. These synthetic ordinary curve-kernel workers are not a full MSM, proof schedule, or concurrent-proof throughput result.

Eighteen32-call preflights precede15rotated/reversed blocks of8192calls/worker after3checked1024-call primes. Native tests cover tails. The wave timer includes synchronization and the complete caller, including inversion, packing, arithmetic, and scatter. Complete generator/scalar checks occur after worker clocks. All cross-arm output/reference signatures match, timed allocations are zero, and capacities remain stable within each process. Capacities intentionally differ between arms and are checked against the original allowance.

| Pairs | Workers | FROZEN µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF [bounds] | Wins/15 |
|---|---:|---:|---:|---:|---|---:|
| 200 | 1 | 7.862840 | 7.864978 | 7.947534 | 1.013285 [1.004090, 1.018629] | 2 |
| 200 | 16 | 9.022653 | 9.082972 | 9.177892 | 1.009404 [1.003733, 1.014521] | 2 |
| 200 | 32 | 14.673835 | 14.726103 | 15.155474 | 1.029299 [1.021524, 1.035110] | 0 |
| 352 | 1 | 13.301990 | 13.283542 | 13.413294 | 1.009210 [1.002188, 1.018989] | 3 |
| 352 | 16 | 15.409075 | 15.427354 | 15.683301 | 1.013680 [1.009106, 1.020363] | 1 |
| 352 | 32 | 25.262770 | 25.267476 | 26.038419 | 1.030058 [1.023032, 1.034377] | 0 |

Latency columns are medians of15wave observations; paired ratios are not ratios of those medians. Calibration requires ordered-ratio intervals within[1/1.03,1.03]; ON nonregression upper bounds must be≤1.03. Equal-log all-six and logical32-two aggregates must each have upper<1 and≥10/15wins against both controls. Bounds use ranks4–12of15, nominal96.484375%marginal under independent/exchangeable-block assumptions without a familywise guarantee. No exclusion, sample extension, changed threshold, favorable subset, unchanged rerun, or size/layout sweep is used.

Failed checks, retained exactly:

```json
[
  "n200-logical32/on/frozen",
  "n200-logical32/on/off",
  "n352-physical16/on/frozen",
  "n352-logical32/on/frozen",
  "n352-logical32/on/off",
  "all6/on/frozen/gain",
  "all6/on/off/gain",
  "logical32_2/on/frozen/gain",
  "logical32_2/on/off/gain"
]
```

All317capture commands complete once:296caller processes,270latencies, four grouped counter panels, and four profiles, plus preflight/check/decode work. There are36,126,720measured latency kernel calls,62,088,384measured calls including preflight/diagnostics,15,040,512priming calls, and77,128,896total calls. These are not proofs. No owned compilation/helper/hash/archive/verifier/second workload overlaps capture.

## CPU and memory observations

All latency precedes counters and then fixed16Mcycles:u profiles. FIFO gates exclude primes and final validation while retaining synchronization/accounting edges. All four counter panels report100%running coverage. Values below divide by workers×8192:

| Workers/arm | Instructions/batch | Cycles/batch |
|---|---:|---:|
| stat-n200-physical16-off | 111169.528 | 41493.367 |
| stat-n200-physical16-on | 112673.487 | 41897.564 |
| stat-n200-logical32-off | 111169.529 | 60863.285 |
| stat-n200-logical32-on | 112673.503 | 62170.478 |

ON executes about1.35%more instructions in each panel and has more measured cycles. Single panels have no repeated-panel uncertainty estimate and do not isolate a causal cache, RAM, dependency, or port cost. Generic cache-event differences remain in the data without a cache-level attribution.

All three linked borrowed kernels have1450static IFMA sites. FROZEN/OFF have8587instruction sites and ON8659. Static sites are not dynamic spill counts, stack bounds, or a causal speed model.

| Profile | Samples | Unknown period | Sampled workers | Qualified |
|---|---:|---:|---:|---|
| cycles-n200-physical16-off | 10870 | 0.0460% | 16 | true |
| cycles-n200-physical16-on | 10961 | 0.0365% | 16 | true |
| cycles-n200-logical32-off | 31512 | 0.0381% | 32 | true |
| cycles-n200-logical32-on | 32222 | 0.0372% | 32 | true |

All85565samples pass executable/PID/FORK/workerTIDCPU/event/fixed-period/zeroLoss checks. Every worker is sampled; no nonworker sample occurs. Attribution thresholds are≥1000samples and≤5%unknown. Inlining, sampling skid, and unvalidated absolute period coverage prevent interpreting symbol shares as exclusive removable wall costs.

| Pairs/logical32 | OFF/ON scratch bytes per worker | FROZEN/OFF/ON maximum process RSS MiB |
|---|---:|---:|
| 200 | 16000 / 32000 | 4.25 / 4.00 / 5.50 |
| 352 | 28160 / 56320 | 11.25 / 11.25 / 11.75 |

Fixture reference payload adds3200/5632bytes per worker at200/352pairs. Prefix storage doubles from2to4vectors per group and remains within the declared original9-vector allowance. These payloads exclude other live arrays, stacks, and allocator state. RSS is a process-lifetime diagnostic that may undercount; it establishes neither minimum RAM nor a rigorous peak. No new memory-pressure, portable/device, or owned-prefix performance qualification is claimed.

## Review and decision

The independent Rust reviewer imports no campaign/parser code and passes on its first execution: **592pins,325successful workflow commands**, protocols/stream hashes/source recovery, exact field/adapter identities, optimized coherent links, all270integer latency observations and gates, per-arm workspace bounds, CPU/RSS/capacities, and separately parsed sample ownership/CPU/period/loss. No successful test, link, or capture is repeated for bookkeeping.

Close this implementation at the failed complete-caller gate. Arithmetic correctness and smaller record size than the earlier X cache do not establish a speed benefit. No new SDK request, proof export, Gnark verification, AWS/Aeglos comparison, phone/ARM/TEE, large250/4999circuit, pressure/swap, or concurrent-throughput result follows. Canonical ledger29953 remains unchanged. Read the [design and prior-evidence scope](artifacts/history.md) before proposing an unchanged repeat.

## Unchanged accepted full-proof comparison

Inherited accepted32-worker local results and supplied M5 references, not timings of this candidate. Source: [accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Comparing different machines does not establish equal hardware efficiency. Latest separate fullSDK evidence remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), retaining its primary gain and failed full gate.

| Shape | Accepted local ms | M5 CPU ms | M5 Metal ms | CPU/local | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1x2 | 7.928660 | 37 | 24 | 4.667× | 3.027× | 3.7 |
| 2x2 | 11.151932 | 59 | 30 | 5.291× | 2.690× | 5.9 |
| 4x2 | 16.086386 | 109 | 48 | 6.776× | 2.984× | 10.9 |
| 8x2 | 24.179453 | 189 | 74 | 7.817× | 3.060× | 18.9 |
| 12x2 | 33.884455 | 284 | 101 | 8.381× | 2.981× | 28.4 |
| 16x2 | 40.077381 | 335 | 124 | 8.359× | 3.094× | 33.5 |
| 24x2 | 65.850777 | 528 | 176 | 8.018× | 2.673× | 52.8 |
| merge-24x1 | 65.060937 | 507 | 175 | 7.793× | 2.690× | 50.7 |

Merge54 remains161.883899ms without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet,0/8.

## Artifacts

[Protocol](artifacts/capture-design.md), [analysis](artifacts/analysis.json), [independent review](artifacts/review.json), [source edits](artifacts/edits.json), [reproduction](artifacts/reproduce.md), [artifact index](artifact-index.json), [checksums](SHA256SUMS).

The gzip JSON receipt archive preserves relative paths, exact UTF-8 bytes, lengths and SHA256s with verified extraction. Sources, complete native/link/capture receipts, decoded samples/metadata/events/loss, counter CSVs, and linked assembly are included. Executables, libraries, raw perf.data, and private key/image/witness/input arrays are excluded.
