# G2 prefix-chain endpoint products

30 October 2026. Real-operand complete-caller comparison, following the [production G2 replay census](../2026-10-30-real-g2-replay175/README.md). Production remains unchanged. This is a caller experiment, not a proof-latency result.

Declared caller gate: **false**. Failed checks: `["12x2-dominant-w32","1x2-dominant-w32","4x2-dominant-w32","merge-24x1-dominant-w32","original8/gain"]`. Dominant primary ON/OFF ratio **1.013109710**, bounds **[1.011559551, 1.016175286]**, **2/31 wins**, a **+1.311% latency change**. Lower is faster. No adoption follows from this stage.

The candidate is rejected before SDK timing. The primary replay is **1.31% slower** and the original-eight aggregate **1.45% slower**, with both intervals above parity. All eleven case intervals are above parity. Three other nonregression intervals exceed the declared 1.02 upper limit: 12×2, 4×2 and merge24. The remaining nonregression checks pass their tolerance; that does not mean those cases became faster. Every observation is retained.

## Implementation and scope

The first forward G2 norm-prefix group assigns its norm instead of multiplying by one. At reverse j=0, the factor is the current inverse and the final unused inverse update is omitted. Interior chains, both record slots, eight lanes, inversion, tails, signs, coordinate formulas and field arithmetic remain unchanged. The prefix identity proves equality of residues for nonzero norms; lazy representatives may differ within the existing field bounds. No heap buffer, key table, point-array copy, plan, pool, worker geometry, readiness dependency or forecast changes. Earlier failed G1 stored-numerator endpoint trials remain rejected. [Design](artifacts/design.md), [history](artifacts/history-audit.md), [accepted code review](artifacts/static-review.md).

The private source has 176 files; 171 of 175 originals are exact and reversal recovers all 175. All 17 field files remain byte-exact and inherit qualification. The common replay child contains codec/runner code and two existing tests; recording hooks, Mutex and file I/O are absent from timed kernels. OFF/ON each pass 126 unfiltered core and eight prover tests. All 152 immutable captured fixtures preflight in each variant, with outputs matching the inherited independent Arkworks oracles. No old capture or independent oracle is repeated.

Ryzen 9 9950X, 16 physical cores / 32 logical CPUs, shared host. Eleven cases use real dominant 200-pair operands from all nine shapes at 32 workers, plus primary owned-512 at 32 and primary dominant-200 at 16. Each dominant context has four captured fixtures. Replay compacts storage and includes a reset copy per call; it does not preserve original key/bucket locality or proof scheduling. These are independent pinned std-thread batch waves, not the proof task scheduler.

Thirty-one fixed blocks rotate/reverse case order and alternate OFF/ON order. Each worker executes three checked 256-round primes and 2,048 measured rounds across four fixtures: 8,192 timed batch calls per worker. Parsing, borrowed reference construction, capacity snapshots and final oracles remain outside every worker’s clock through barriers. All outputs and retained capacities match across variants; all timed global allocations are zero. Whole-process taskset also constrains the coordinator.

The comparison has two fresh arms. The earlier capture library cannot be a frozen trace-free control because it includes recorder locks. Source recovery and immutable coherent native library/feature/flag receipts qualify OFF; a later full SDK stage must still compare accepted/OFF/ON and require calibration.

|Case|OFF µs/batch wave|ON µs/batch wave|ON/OFF ratio|Bounds|Wins /31|Check|
|---|---:|---:|---:|---|---:|---|
|12x2-dominant-w32|15.332018|15.639292|1.016134760|[1.007693591, 1.023435331]|4|false|
|16x2-dominant-w32|15.381748|15.557685|1.015716574|[1.007964313, 1.018900464]|3|true|
|1x2-dominant-w16|10.038523|10.202711|1.015347421|[1.013003059, 1.016866650]|0|true|
|1x2-dominant-w32|16.355370|16.565850|1.013109710|[1.011559551, 1.016175286]|2|false|
|1x2-owned512-w32|39.347546|39.916940|1.014454078|[1.008789670, 1.018175401]|6|true|
|24x2-dominant-w32|15.417811|15.611760|1.009478751|[1.004056611, 1.019293615]|6|true|
|2x2-dominant-w32|16.313638|16.542131|1.016581106|[1.013313647, 1.018893006]|2|true|
|4x2-dominant-w32|15.386925|15.618772|1.017640615|[1.012421728, 1.022286458]|3|false|
|8x2-dominant-w32|15.369933|15.567842|1.013295005|[1.009400376, 1.018940272]|2|true|
|merge-24x1-dominant-w32|15.423087|15.609745|1.019139490|[1.008533829, 1.024636620]|6|false|
|merge-54x1-dominant-w32|15.478521|15.652346|1.011412839|[1.007045179, 1.017260707]|3|true|

Equal-log original-eight dominant-caller aggregate: **1.014543296 [1.011975064, 1.016832514], 0/31 wins**, check **false**. Ratios pair within blocks and need not equal ratios of displayed medians. The aggregate is not a frequency-weighted proof-latency prediction.

Primary dominant-200/32 and the original-eight aggregate require upper ratios below one and at least 21/31 wins independently. Every other case requires upper ratio ≤1.02. Sorted ranks 10–22 of 31 give nominal 97.05506265163422% individual median coverage under independent/exchangeable blocks, without familywise coverage. No gain floor, exclusions, extension, threshold change, favorable subset or unchanged reroll is used. A pass only permits distinct full-proof/resource/final-artifact qualification.

## Counters and profiles

All latency precedes six FIFO-gated grouped four-event counter panels and six fixed-period user-cycle profiles at primary 16/32 and merge54 owned-200/32, OFF/ON. Counter values below are per executed batch across all workers. Each panel is a single diagnostic observation, not a repeated-trial interval or isolated cause.

|Panel|Instructions/batch|Cycles/batch|Counter coverage qualified|
|---|---:|---:|---|
|stat-1x2-dominant-w16-OFF|123397.859|44553.482|true|
|stat-1x2-dominant-w16-ON|122688.868|45492.997|true|
|stat-1x2-dominant-w32-OFF|123397.884|63471.545|true|
|stat-1x2-dominant-w32-ON|122688.901|65240.165|true|
|stat-merge-54x1-dominant-w32-OFF|108903.151|59956.532|true|
|stat-merge-54x1-dominant-w32-ON|108940.107|61747.325|true|

Profiles use cycles:u, period 16 million, 262,144 calls per worker after primes. CPU/time/event/MMAP, worker TID/CPU correspondence, one-PID and zero-loss checks accompany attribution thresholds of at least 1,000 samples and at most 5% unknown period. The previously observed absolute-period accounting deficit remains unresolved; shares are qualitative, not exclusive costs, local CPI or removable proof wall time.

All six counter panels and six profile-attribution panels pass. Primary retired instructions fall about **0.575%** versus OFF at both worker counts, while counted cycles rise about **2.11% at 16 workers and 2.79% at 32** in these single panels. Removing mathematical products therefore did not produce a faster caller here. This comparison does not isolate branch, dependency, register-allocation or instruction-cache effects. The two arithmetic arms use the same trace-free replay harness and no proof task scheduler.

|Profile|Samples|Unknown period share|Attribution qualified|Lost|
|---|---:|---:|---|---:|
|cycles-1x2-dominant-w16-OFF|11668|0.021940|true|0|
|cycles-1x2-dominant-w16-ON|11918|0.023997|true|0|
|cycles-1x2-dominant-w32-OFF|33350|0.028636|true|0|
|cycles-1x2-dominant-w32-ON|34182|0.025920|true|0|
|cycles-merge-54x1-dominant-w32-OFF|31344|0.028235|true|0|
|cycles-merge-54x1-dominant-w32-ON|32550|0.031951|true|0|

## Records and limitations

All 742 workflow steps complete, including 18 all-fixture preflights, 682 latency processes, 12 diagnostic processes and 30 ownership/decode/audit steps. Total **278,889,664 actual kernel calls**, including primes/profiles. The original SDK consumers are linked but not run in this caller stage. No new SDK request, proof export or Gnark verification occurs; the canonical distinct-proof ledger remains **23,971**. Raw coordinates remain local and are not copied into this report.

The initial standard SDK linker omitted the replay driver’s explicit SHA-256 dependency and stopped before a driver ran. The four completed native steps, including OFF core/prover suites, were retained. A separate link helper passes all nine frozen libraries explicitly; a remaining-step workflow reuses completed work. Initial failed link command/stderr/exit and both helpers remain archived. No successful test or capture repeats. All original pins close; source, receipts and symbol archives round-trip byte-exactly. No independent owned compiler/helper/hash/archive or other workload overlaps capture.

[Full analysis](artifacts/analysis.json), [every observation](artifacts/rows.json), [source recovery](artifacts/source-audit.json), [complete private source](artifacts/source.json.gz), [command receipts](artifacts/receipts.json.gz), [archive review](artifacts/archive-review.json), [capture plan](artifacts/plan.json), [original build plan](artifacts/build-plan.json), [remaining build plan](artifacts/remaining-native-plan.json), [local-only raw hashes](artifacts/raw-local-pins.json).

Caller peak RSS and scratch capacities are retained in analysis; they are not prover RAM, minimum RAM or pressure qualification. No new AWS/Aeglos, ARM/mobile/TEE, forester250/4999, swap/pressure or concurrent-throughput qualification follows.

## CPU and Metal comparison

These accepted local measurements and supplied historical M5 references are inherited, not candidate proof results. The all-eight CPU 10× goal remains active and unmet, 0/8. Accepted local 1×2 remains 8.022165 ms / 134 MiB; AWS 1×2 remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. The accepted physical-16 calibration engineering exception is retained.

|Shape|Accepted local ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU 10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1x2|8.022165|37.0|4.612×|24.0|2.992×|3.7|
|2x2|11.335969|59.0|5.205×|30.0|2.646×|5.9|
|4x2|15.945544|109.0|6.836×|48.0|3.010×|10.9|
|8x2|24.090485|189.0|7.845×|74.0|3.072×|18.9|
|12x2|33.844855|284.0|8.391×|101.0|2.984×|28.4|
|16x2|39.931557|335.0|8.389×|124.0|3.105×|33.5|
|24x2|66.233309|528.0|7.972×|176.0|2.657×|52.8|
|merge-24x1|65.273224|507.0|7.767×|175.0|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms / 1831.87 MiB, without a supplied M5 reference. [Current summary](../2026-10-29-current-cpu-summary/README.md), [dated consolidated report](../2026-10-30-cpu-benchmark-report/README.md).

[Offline report notes](artifacts/offline-report-notes.json) document the postcapture interpretation. Original pinned helpers, captures, statistics and gate thresholds are unchanged.
