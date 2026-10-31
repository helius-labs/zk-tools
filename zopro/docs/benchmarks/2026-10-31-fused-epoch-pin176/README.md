# Fused epoch pinning in complete MSM queries

**Final-driver compiler correction (31 October 2026):** These query drivers used an extern-only `rustc_flags` override, omitting the default optimization/LTO profile while their backend libraries remained native release builds. The [compiler-profile audit and correction](../2026-10-31-sdk-link-flags176/README.md) documents the limitation. Original data and failed gates remain; this correction promotes no candidate and changes no accepted proof timing.

**The query gate fails; no adoption.** The primary A/B2 aggregate at 32 workers improves 1.18% against fresh OFF: ratio 0.988185 [0.981163, 0.998546], 12/15 wins. Against the previously frozen original caller it is 1.013214 [0.988838, 1.021032], 6/15 wins. Five OFF/frozen calibration bounds fail, and one candidate/frozen nonregression bound fails. The positive OFF comparison is retained; it does not establish a gain against the original implementation.

Production source 176, accepted SDK, proof latency and verified-export ledger 27,353 remain unchanged. All eight CPU 10× targets remain unmet. This experiment measures complete A/B2 queries, without producing a new proof or AWS result.

## Change and linked code

The accepted single-job steal first queries `epoch::is_pinned()`, issues the required full fence when already pinned, then calls `epoch::pin()`. Both calls access the same thread-local handle. The actual accepted binary contains both accesses. A private Crossbeam-epoch 0.9.21 adds a safe `pin_fenced` operation that performs the same check/fence and original handle pin inside one existing `with_handle` callback. Private Crossbeam-deque 0.8.8 selects it only for `Stealer::steal`, through a compile-time flag. OFF retains the original operations.

The linked candidate has one steady-state TLS lookup and handle-state check instead of two. Its pin body is inlined into the single-job steal; the reentrant `SeqCst` fence, unpinned epoch CAS and release unpin store remain visible. Original front/back loads, buffer checks, job CAS, guard lifetime, victim order, retries, collection cadence, sleep policy and task graph retain their rules. This differs from the earlier guard-sharing, empty-queue checks, batch-transfer and idle-policy experiments. Those results remain closed.

| Linked query caller | Single-steal bytes | Static instructions | Steady TLS lookups |
|---|---:|---:|---:|
| Frozen original |328|80|2|
| Fresh OFF |328|80|2|
| Fused |726|159|1|

The candidate's function grows because pinning is inlined; the controls call a separate 457-byte pin helper. These are selected static function counts, not executed work or a whole-program size comparison. Other default pin callers remain unchanged. No key table, point copy, plan, worker pool or per-proof buffer is added. Slow TLS initialization/destruction can still register temporary handles. No minimum-RAM claim follows from the warm-path source change.

## Complete-query measurements

Ryzen 9 9950X: 16 physical cores/32 logical CPUs. Identical driver bytes and original A/B2 query APIs compare the previously frozen original caller, fresh OFF and fused candidate. Each custom Rayon worker is individually pinned, using 512 KiB stacks and CPU 2/0–15/0–31 for 1/16/32 workers. All four real point/scalar fixtures and their full independent Arkworks oracle receipts are reused by hash. The earlier sparse-coalescing candidate is not composed.

Timers include scalar classification, all point work, ordinary/top fills, component/part merges, weighting, normalization, allocation/drop and pool dispatch/join. B2 scalar reset is charged identically inside each timer. Loading, pool construction and independent result checks are outside. Every timed result is retained in preallocated storage and checked afterward; each process also checks three primes and one post. Inputs remain immutable, and all diagnostic route/count/payload fields stay zero as expected for the original query APIs.

Fifteen fixed rotated/reversed blocks cover 12 contexts and three variants, with 20 timed queries per process. All 540 latency processes finish before diagnostics. No owned build, hash, analysis, archive or other benchmark overlaps timing; exclusive host use is not claimed.

| Shape/query | Workers | Frozen ms | OFF ms | Fused ms | Fused/OFF [bounds], wins | Fused/frozen [bounds], wins |
|---|---:|---:|---:|---:|---|---|
|1x2/A|1|14.663559|14.746116|14.654388|0.994826 [0.992628,0.996851], 14|0.999878 [0.998550,1.001505], 8|
|1x2/A|16|1.554688|1.555183|1.552528|0.996117 [0.993987,1.003394], 10|0.999856 [0.987280,1.004832], 8|
|1x2/A|32|1.201314|1.197994|1.204959|1.004733 [0.992246,1.014476], 6|1.002447 [0.994650,1.017205], 6|
|1x2/B|1|37.252254|37.493374|37.187660|0.992995 [0.991658,0.993965], 15|0.998556 [0.997393,1.000866], 11|
|1x2/B|16|3.471520|3.543877|3.496887|0.989953 [0.969038,0.998651], 13|1.009841 [0.996388,1.019444], 6|
|1x2/B|32|3.065282|3.145343|3.100463|0.982620 [0.961680,1.006159], 11|1.010421 [0.992427,1.026851], 4|
|2x2/A|1|14.527265|14.754228|14.643729|0.993831 [0.991191,0.994581], 15|1.008017 [1.005811,1.009608], 0|
|2x2/A|16|1.804923|2.171782|2.170926|0.995358 [0.992182,1.010088], 9|1.015640 [1.001612,1.211949], 3|
|2x2/A|32|1.781069|1.747231|1.769327|1.004424 [0.997344,1.013811], 5|0.989199 [0.981622,0.997138], 13|
|2x2/B|1|39.847325|39.741450|39.665094|0.999037 [0.997017,1.000416], 11|0.995452 [0.994402,0.996362], 15|
|2x2/B|16|4.021493|4.006652|4.000589|1.000131 [0.985248,1.010208], 7|0.995743 [0.990075,1.003582], 10|
|2x2/B|32|3.591492|3.606635|3.640668|1.016359 [0.977722,1.024383], 7|1.005071 [0.971956,1.025851], 7|

The prospective gate requires every OFF/frozen interval inside factor 1.03, primary equal-log A/B2 at 32 workers upper <1 and at least 10/15 wins against both controls, and every candidate context upper ≤1.03 against both. Candidate/OFF passes its aggregate and all 12 context bounds. Candidate/frozen fails the primary gain check and the 2×2 A-w16 context bound. Calibration failures are primary B-w16, primary B-w32, 2×2 A-w16, 2×2 A-w32 and 2×2 B-w32. The A-w32 calibration failure is its lower bound below 1/1.03; the others exceed the upper bound.

Ranks 4–12 of 15 give nominal 96.484375% marginal median intervals, without familywise coverage. Paired ratios need not equal ratios of marginal medians. The broad 2×2 A-w16 interval is retained, without attributing it to a unique cause. There is no gain floor, exclusion, extension, favorable subset, parameter sweep or unchanged rerun. Calibration uncertainty does not turn the fresh-OFF gain into a verified original-caller improvement.

## CPU, faults and process memory

Entries below follow frozen / OFF / fused. RSS is the maximum process high-water sampled at the timed-loop end, including loading and primes. Faults are median warm-loop minor faults per query. These do not measure complete prover RAM, minimum RAM, swap activity or causal memory stalls.

| Shape/query | Workers | CPU ms/query | RSS MiB | Minor faults/query |
|---|---:|---:|---:|---:|
|1x2/A|1|14.663 / 14.746 / 14.654|10.25 / 10.25 / 10.25|0.10 / 0.10 / 0.10|
|1x2/A|16|17.013 / 17.172 / 17.017|18.75 / 18.75 / 19.00|24.15 / 23.95 / 23.70|
|1x2/A|32|25.189 / 25.021 / 24.774|23.25 / 23.75 / 24.75|28.85 / 29.20 / 29.60|
|1x2/B|1|37.251 / 37.493 / 37.187|15.75 / 15.75 / 15.75|0.10 / 0.10 / 0.10|
|1x2/B|16|45.017 / 45.345 / 44.873|27.75 / 27.75 / 27.50|36.85 / 33.55 / 35.70|
|1x2/B|32|69.998 / 70.651 / 70.361|32.50 / 34.50 / 33.00|43.25 / 46.95 / 44.35|
|2x2/A|1|14.527 / 14.754 / 14.643|13.75 / 13.50 / 13.50|0.10 / 0.10 / 0.10|
|2x2/A|16|17.760 / 17.987 / 17.918|22.25 / 22.25 / 22.75|26.00 / 27.90 / 26.40|
|2x2/A|32|26.940 / 26.852 / 26.895|29.75 / 29.50 / 29.00|53.10 / 54.50 / 53.75|
|2x2/B|1|39.846 / 39.741 / 39.665|27.75 / 27.75 / 27.75|0.10 / 0.10 / 0.10|
|2x2/B|16|48.215 / 48.080 / 48.144|43.00 / 43.50 / 43.25|26.80 / 27.95 / 24.80|
|2x2/B|32|76.332 / 76.356 / 76.023|69.25 / 61.00 / 60.75|65.35 / 64.95 / 122.55|

The small CPU changes and mixed RSS observations do not qualify a universal memory or latency benefit. Work placement, code generation and allocation lifetimes can differ even when no buffer is added. Raw context-switch and CPU observations remain in the machine-readable results.

## Integrated counters and profiles

After all latency: 12 grouped counter panels and 12 separate cycle profiles, primary A/B2 at 16/32 workers across all three variants. Each uses 200 timed queries after three excluded primes, controlled through the existing perf FIFO. All four grouped events report 100% coverage. Counters are descriptive panels, without paired confidence intervals.

| Query/workers/variant | Cycles M/query | Instructions M/query | Generic cache misses M/query | Branch misses M/query | Profile samples |
|---|---:|---:|---:|---:|---:|
|A/16/frozen|85.869|208.496|0.242036|0.137862|1066|
|A/16/control|86.437|208.426|0.221233|0.137922|1075|
|A/16/candidate|85.848|207.539|0.234232|0.138944|1064|
|A/32/frozen|114.904|214.168|0.316435|0.162500|1429|
|A/32/control|114.664|213.524|0.307821|0.165545|1431|
|A/32/candidate|114.262|211.798|0.305482|0.164168|1431|
|B/16/frozen|223.743|550.466|0.870378|0.269811|2782|
|B/16/control|226.042|551.060|0.748222|0.268528|2815|
|B/16/candidate|223.252|548.667|0.727013|0.262553|2789|
|B/32/frozen|318.768|568.789|1.267170|0.317275|3963|
|B/32/control|322.349|568.795|1.243779|0.323664|4007|
|B/32/candidate|319.602|563.089|1.049749|0.311678|4019|

At 32 workers, candidate instructions fall about 0.81%/1.00% for A/B2 against OFF; cycles fall about 0.35%/0.85%. Against frozen, B2 cycles instead rise about 0.26%. Single panels do not isolate an execution-port, cache-level, DRAM or causal wall-time fraction.

All 27,871 samples have the fixed 16-million-cycle period, correct executable PID, registered worker/caller TID, worker CPU, binary MMAP and worker FORK/task evidence. Every profile has zero lost records and at least 1,000 samples. Unknown shares stay below 1.32%. Pinning is now inlined into Stealer, which receives roughly 10.34%/9.18% of A/B2 candidate 32-worker samples. A lower epoch-symbol share would therefore not prove that epoch work disappeared. Stream/apply, reduction and inversion kernels remain dominant. Symbol shares are not removable wall time, and absolute period coverage is not validated.

## Qualification and retained preparation failures

Both dependency configurations pass their upstream unit, integration and doc tests: 165 OFF and 167 ON successful tests, with five pre-existing ignored doc tests per mode. The two new ON tests cover nested pin/drop/unwind behavior and pin/flush after TLS handle destruction. These are ordinary native tests; no new Loom result is claimed. Both modes pass all 125 native core tests and two raw-format prover tests. Field arithmetic and its existing qualification are inherited unchanged. No new full SDK, cubic-feature, portable, huge-circuit, pressure or concurrent-proof qualification is inferred.

All 176 production originals remain pinned. The private copies change only the Cargo patch/lock source entries and one diagnostic export suffix in the original tree; all cryptographic bodies recover exactly. Runtime dependency versions and edges stay unchanged, with only two path-source replacements. Dependency test mirrors use the same modified source and Crossbeam-utils 0.8.23. Original licenses are retained.

An initial offline metadata request tried to fetch an unused GPU package unavailable in the local cache; no build or test ran. A no-dependency metadata check and the actual selected native builds then succeed without fetching it. The first candidate dependency compile fails because the added fence used the wrong module path. Its source and compiler receipt remain. Separate fixed snapshots change only that module path to the same primitive atomic module used by Crossbeam internally. The completed OFF tests/builds are retained, and only candidate steps execute after repair. No successful test, query or oracle is repeated.

The native phase contains five successful OFF steps, one preserved failed ON compile, then five successful repaired ON steps. Two links and the 660-step capture workflow complete once. That capture contains 36 preflights, 540 latency processes, 24 diagnostics, 12 planned ownership changes on only generated perf.data, and 48 read-only decoders. All 600 caller processes check 18,036 complete query outputs including primes/post. There is no witness solve, fresh proof or new Gnark export.

Independent review checks 673 raw command/exit/stream receipts including the failed compile, 2,979 pin entries, source/lock recovery, the exact one-module repair, unchanged fixture/oracle/binary bindings, every output receipt, all paired wall/CPU/RSS statistics and both control comparisons, all counter values and every profile sample/symbol bin. It also verifies exact registered worker CPU/TID, MMAP/FORK and loss evidence. It repeats no query, test or verifier.

## Research scope

The reviewed [mixed-window chiplet paper](https://eprint.iacr.org/2025/252) studies an ASIC memory/processing-element configuration; its gains do not establish a CPU result here. [Ozaki Scheme II on CPUs](https://arxiv.org/abs/2609.27831) concerns dense multiple-precision numerical GEMM using CRT with AMX or DGEMM. Inferring a faster exact BN254 field kernel from that result would require new bounds, representation/conversion work and native evidence. Neither paper supplied implementation code for this experiment. The selected change comes from the actual current linked runtime path.

## Accepted proof comparisons — inherited, unchanged

|Shape|Accepted ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge 24→1|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Accepted merge 54→1 is 161.883899 ms without a supplied M5 reference. Accepted primary process RSS is 132.75 MiB. The [adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its physical-16 calibration engineering exception and qualification scope. No new memory or platform result follows.

Latest AWS primary remains historical source 175 on c7a.24xlarge: Zopro 11.915591 ms at 32 workers versus Aeglos 7.959184 ms at 96, each campaign's best tested configuration. That does not compare current source 176. No AWS resources were created here.

The 10× objective remains active and unmet in all eight shapes. No source adoption or full-SDK trial follows this failed query gate. The removed duplicate access is real, but this exact lowering does not establish an original-caller latency gain. No unchanged timing, wrapper/inlining sweep or calibration reroll follows. A further candidate needs a distinct work or dependency reduction.

[Prospective design](artifacts/design.md), [capture protocol](artifacts/capture-design.md), [all observations](artifacts/analysis.json), [symbols](artifacts/symbols.json), [independent review](artifacts/review.json), [linked code](artifacts/codegen.json), [manual path audit](artifacts/codegen-review.json), [identities](artifacts/identities.json). Archives: [fixed private source and initial failed fence source](artifacts/source.tar.gz), [helpers/protocols/pins](artifacts/workflow.tar.gz), [raw command receipts and counter CSVs](artifacts/receipts.tar.gz), [selected disassembly](artifacts/codegen.tar.gz). [Checksums](SHA256SUMS).

Key/scalar/point arrays, executables, libraries and raw perf.data remain local with recorded hashes. No witness-derived output point is published. The [accepted adoption report](../2026-10-30-g2-vector-sign-adoption176/README.md) retains its qualification scope and original calibration exception.
