# Reduced Gauss products in native G2 callers

The candidate **fails the declared actual-caller screen and is not adopted**. G2 prefix batches of 200 at 32 logical CPUs have ON/OFF paired ratio **1.111847210**, bounds [1.104041023,1.120968058], 0/31 wins; ON/frozen is **1.112867630**, bounds [1.108363946,1.121356422], 0/31 wins. Production and the accepted SDK remain unchanged.

## Formula and scope

For a complex product over Fq[u]/(u²+1), compute P=(a0+a1)b0, Q=a1(b0+b1), R=a0(b1-b0), then return (P-Q,P+R). For a times conjugate(b), compute P=(a0+a1)b0, Q=a1(b0-b1), R=a0(b0+b1), then return (P-Q,P-R). Original shared real-factor scaling follows unchanged. All operations use the existing reduced field API and its lazy bounds.

This removes one raw bilinear product and adds one modular reduction: three ordinary products use 300 IFMA sites, the same as two fused expressions. Five reduced linear operations and different dependencies remain costs. The earlier shared/streamed/nonnegative raw Gauss methods used three raw products and two reductions, with wider signed or biased accumulators. Those rejected kernels are absent here. This tests a different arithmetic trade; fewer raw products do not establish fewer instructions or faster proving.

Only two native G2 formula bodies in msm.rs change. Their old bodies remain verbatim fallback under a private cfg. Removing the two adapter suffixes and reversing those body edits recovers all 172 originals; 170 original files remain byte-exact. Field, G1, FFT, solver, curve formats, keys and public APIs retain accepted source. No new unsafe arithmetic, heap buffer, key table, cache, execution plan or public runtime option is added.

## Complete caller measurements

Ryzen 9950X, 31 fixed rotated/reversed blocks, three callers at solo CPU2, physical16 CPUs0–15 and logical32 CPUs0–31, with independent frozen accepted-library, fresh OFF and selected ON executables. Three checked/reset 1024-call primes precede 8192 timed calls per worker. Ready/start/done/release barriers place every worker's checking and allocation outside every worker's clock. Main wave time includes the slowest worker and barrier tail; per-worker elapsed, CPU and throughput remain in raw results. These synthetic ordinary points at real stream sizes are not proof or complete-MSM timings.

| Caller/pairs | Layout | Frozen µs/wave | OFF µs/wave | ON µs/wave | Paired ON/OFF | Marginal interval | Wins |
|---|---|---:|---:|---:|---:|---|---:|
| g2-prefix/32 | solo | 2.223494 | 1.915917 | 2.489889 | 1.150422022 | [1.121059679,1.172775225] | 0/31 |
| g2-prefix/32 | physical16 | 2.427366 | 2.381231 | 2.674984 | 1.118030532 | [1.111212783,1.127872331] | 0/31 |
| g2-prefix/32 | logical32 | 4.396604 | 4.445553 | 4.948580 | 1.112808151 | [1.074514716,1.136514430] | 0/31 |
| g2-prefix/200 | solo | 9.189845 | 9.201998 | 10.290953 | 1.124247845 | [1.122296262,1.127380436] | 2/31 |
| g2-prefix/200 | physical16 | 10.279131 | 10.116140 | 11.528935 | 1.129356010 | [1.121390411,1.142272897] | 0/31 |
| g2-prefix/200 | logical32 | 18.712768 | 18.714051 | 20.868690 | 1.111847210 | [1.104041023,1.120968058] | 0/31 |
| g1-borrowed-signed/200 | solo | 4.434443 | 4.783358 | 4.780201 | 1.000177768 | [0.998144896,1.004309580] | 15/31 |
| g1-borrowed-signed/200 | physical16 | 5.236418 | 5.134845 | 5.189615 | 1.011553979 | [1.002040437,1.020238817] | 8/31 |
| g1-borrowed-signed/200 | logical32 | 9.430133 | 9.477657 | 9.510478 | 1.004319160 | [0.988400041,1.018319334] | 13/31 |

| Caller/pairs/layout | OFF/frozen | Marginal interval | ON/frozen | Marginal interval |
|---|---:|---|---:|---|
| g2-prefix/32/solo | 0.997687934 | [0.989146875,1.000334577] | 1.137333089 | [1.116787960,1.159604937] |
| g2-prefix/32/physical16 | 0.990664343 | [0.972368081,0.994154762] | 1.101931979 | [1.094553064,1.110934551] |
| g2-prefix/32/logical32 | 1.010714441 | [0.982386592,1.058361027] | 1.124241449 | [1.098086334,1.158124608] |
| g2-prefix/200/solo | 0.994257352 | [0.988707313,1.003436367] | 1.122433878 | [1.115354724,1.129160959] |
| g2-prefix/200/physical16 | 0.986750761 | [0.974175778,0.998083307] | 1.115641517 | [1.113642358,1.121406168] |
| g2-prefix/200/logical32 | 0.996676864 | [0.993260756,1.012411792] | 1.112867630 | [1.108363946,1.121356422] |
| g1-borrowed-signed/200/solo | 0.997954886 | [0.993681326,1.004498320] | 0.998434726 | [0.996027059,1.006380865] |
| g1-borrowed-signed/200/physical16 | 0.982529410 | [0.973213119,0.993975346] | 0.994369425 | [0.990643905,1.000577031] |
| g1-borrowed-signed/200/logical32 | 0.998827969 | [0.987513628,1.015494651] | 1.011603944 | [0.989678015,1.020592697] |

Calibration: **false**; nonregression: **false**; target G2/200 logical32 gain: **false**.

The candidate is slower than fresh OFF in all six G2 size/topology comparisons, with paired medians from 11.18% to 15.04% slower and every interval above parity. The primary G2/200 logical32 comparison loses all31 blocks. The frozen-control comparison independently shows the primary regression. The only calibration failure is the supplemental G2/32 logical32 OFF/frozen interval; it remains failed and does not rescue the negative G2 comparisons.

The frozen and fresh OFF executables are byte-identical after independent linking, and the mode argument only labels results. Thus their timing difference is not a different compiled arithmetic implementation. Process placement/address layout, barrier tails and host variation remain unresolved. The ON executable differs. This equality and all hashes are recorded in interpretation.json.

The primary gain check applies only to G2 prefix/200 at logical32, requiring upper ratio below parity and at least 21/31 wins against both controls. The legacy analysis field named logical32_aggregate now contains exactly this single caller, not a cross-caller average. All nine ON comparisons require upper<=1.02; all OFF/frozen calibration bounds must fit [1/1.03,1.03]. Bounds use ordered ranks10–22 of31, nominal97.055% under independent exchangeable blocks, without familywise guarantees. Every positive estimate, uncertainty and failed check is retained; there is no gain floor, exclusion, extension or unchanged reroll.

## Correctness and provenance

OFF and ON each pass the full 123-test native core suite: 119 accepted tests, three original generator-oracle point adapters, and one new scalar-Fq2 expression oracle. The new case checks 512 groups of eight lanes through eight changing steps, both ordinary complex multiplication and conjugate multiplication with shared real scaling: 65,536 Fq2 outputs per configuration. Boundary inputs and existing lazy add/sub paths are included. Repeating the same cases in both builds is not additional distinct input coverage. Existing G2 caller tests independently check generator scalar sums, tails and scratch reuse. Some optional external-fixture cases return when fixtures are absent; those scenarios are not claimed.

Each independent frozen/OFF/ON harness passes three tests, including concurrency, barriers, affinity, complete point outputs and inherited perf-helper checks. No fresh field suite is claimed; field source and its accepted qualification are unchanged. No SDK/Gnark proof is generated.

The first workflow stops in its first process at the reused harness's argument check, which permits only G2 length200 and rejects the prospectively planned length32. No fixture construction, priming or timed loop ran. The preserved repair extends only this main argument check to lengths32/200, then links three benchmark executables against the same frozen libraries. Test bodies and algorithm bytes are unchanged; successful native or harness tests and successful captures are not repeated. The original zero-completion workflow is retained under runs-initial-argument-failure with its original protocol and receipts. New captures use run-plan-v2/pins-v2. An offline analyzer changes only its pin-file path. Read-only G2 disassembly is regenerated for the actual relinked executables. All failures, initial sources/binaries and pin revisions remain available.

Every transitive rlib/proc-macro object is frozen immediately in separate target roots. Actual core/field flags/features are checked. Adapters and harness code are identical across variants. Static IFMA or instruction counts are not executed cost, dynamic spills or a causal timing decomposition. No successful test/library build/capture is repeated for bookkeeping.

All 845 kernel processes verify full generator results, capacity consistency and zero global timed allocations. There are **179,101,696 timed kernel calls**, not proofs. The new algebra adds no retained scratch; per-worker capacities remain in the analysis. Fixed temporaries and process RSS are not stack bounds or minimum RAM claims.

## Counters and profiles

All 837 latency processes finish before four FIFO-gated grouped perf-stat processes, then four separate FIFO-gated 199Hz cycles:u profiles. Each uses logical32 and 262144 calls per worker after the same primes. All eight PMU captures finish before owner/decode/loss, hash or archive steps. No owned helper/build/hash/archive/second benchmark overlaps capture; host snapshots do not establish exclusive-host use.

| Counter case | Cycles/call | Instructions/call | IPC | Branch misses/call | Generic cache misses/call |
|---|---:|---:|---:|---:|---:|
| stat-g1-borrowed-signed-200-m1 | 29443.51136958599 | 52350.29877591133 | 1.7779910187610015 | 13.447998046875 | 18.89812982082367 |
| stat-g1-borrowed-signed-200-m2 | 29448.00713896751 | 52350.298027038574 | 1.7777195509357664 | 13.039828300476074 | 16.747873306274414 |
| stat-g2-prefix-200-m1 | 58883.89909636974 | 109428.03226315977 | 1.8583693325754325 | 14.68250846862793 | 22.36316967010498 |
| stat-g2-prefix-200-m2 | 66745.05092954636 | 123128.80652141573 | 1.844763091894049 | 13.17398965358734 | 20.099501252174377 |

The single grouped G2 observation records 12.52% more instructions and 13.35% more cycles per call; all four groups report100% running. The actual G2 prefix symbols have1450 IFMA sites in both algorithms. These observations retain the added work despite one fewer raw product; they do not isolate an exclusive microarchitectural cause.


Counter observations are descriptive, not confidence bounds. IPC, generic cache misses and sampled period do not identify a memory level, DRAM bandwidth, an execution-port cause or removable proof latency. SMT cycles sum logical-thread events, not disjoint physical-core cycles.

| Profile | Samples | Sampled worker TIDs | Lost |
|---|---:|---:|---:|
| cycles-g1-borrowed-signed-200-m1 | 11425 | 32 | 0 |
| cycles-g1-borrowed-signed-200-m2 | 11410 | 32 | 0 |
| cycles-g2-prefix-200-m1 | 23878 | 32 | 0 |
| cycles-g2-prefix-200-m2 | 26582 | 32 | 0 |

Total 73295 samples; one sampled process and zero lost records per profile. Worker coverage and full symbol weights remain in analysis.json. Raw perf.data and executables remain local with hashes. Four loss dumps are archived using deterministic gzip and verified decompression hashes.

This exact reduced-Gauss integration is closed without an SDK trial or adoption. Positive estimates remain reported within their measured scope. No unchanged formula, annotation, layout or scheduling reroll is selected.

## Inherited CPU and Metal comparison

No new full-proof timing or peak RSS is established. The accepted FFT adoption retains its engineering exception and false full-performance/qualified-scope gates. The original all-eight CPU10x objective remains active and unmet, 0/8. These accepted local timings and supplied M5 references are inherited.

| Shape | Accepted ms | M5 CPU ms | M5 Metal ms | CPU/accepted | Metal/accepted |
|---|---:|---:|---:|---:|---:|
| 1x2 | 8.324853 | 37.0 | 24.0 | 4.445× | 2.883× |
| 2x2 | 11.460675 | 59.0 | 30.0 | 5.148× | 2.618× |
| 4x2 | 15.854365 | 109.0 | 48.0 | 6.875× | 3.028× |
| 8x2 | 24.084626 | 189.0 | 74.0 | 7.847× | 3.072× |
| 12x2 | 33.693397 | 284.0 | 101.0 | 8.429× | 2.998× |
| 16x2 | 39.981636 | 335.0 | 124.0 | 8.379× | 3.101× |
| 24x2 | 66.010670 | 528.0 | 176.0 | 7.999× | 2.666× |
| merge-24x1 | 65.086624 | 507.0 | 175.0 | 7.790× | 2.689× |

Historical primary peak is134.50MiB; merge54 remains161.341835ms /1910.1875MiB, with no supplied M5 reference. No new portable/mobile/TEE, concurrent proof, pressure, giant-circuit or AWS/Aeglos result is claimed. Independent proof ledger remains13,181.

[Design](artifacts/design.md), [analysis](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [new oracle](artifacts/oracle.rs), [actual code](artifacts/codegen.json), [native suites](artifacts/native/result.json), [harness checks](artifacts/linked/result.json), [all captures](artifacts/runs/result.json).
