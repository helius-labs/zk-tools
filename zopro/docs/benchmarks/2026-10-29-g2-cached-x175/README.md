# Packed X coordinate reuse in G2 prefix batches

The separately declared caller gate **fails**. Production remains unchanged. The native candidate saves already-packed input X coordinates across inversion, removing their second load/packing pass. This complete-batch experiment follows the latest profile's dominant MSM arithmetic, while retaining the [cached-hash SDK](../2026-10-29-cached-hash-sdk175/README.md) memory and 54→1 gains within their own failed speed gate.

The all-18 aggregate is **1.72% slower versus OFF** and 1.79% slower versus frozen, with zero winning aggregate blocks. The threaded-12 aggregate is 1.80% slower versus OFF. All calibration checks pass; seven nonregression and four aggregate-gain checks fail. Reducing input packing did not produce a universal caller improvement.

## Implementation and memory

The original prefix record holds two FqBatch vectors, prefix and norm. A native private cfg extends it to six, adding packed Ax.re/Ax.im/Bx.re/Bx.im. The reverse pass reads those values and packs only Y. All denominators, norms, slope and coordinate formulas, signs, order, scalar tails and complete-MSM exceptional handling remain unchanged. Public input points and field arithmetic are unchanged. The addition kernel retains its existing disjoint nonexceptional pair contract.

ON uses reserve_exact after clearing the prefix vector. A new test checks varying request/reuse geometry and six-vector record size against the old nine-vector generic G2 workspace allowance. Extra scratch depends on the current batch size, not key size; no persistent table, point-array copy, execution plan, pool, new unsafe code or field representation is added. Actual capacities can differ across variants and remain visible in the results. Each is checked against the original nine-vector-plus-tail budget; stable primed capacity and zero timed allocations remain mandatory. Typed bounds do not establish process RSS or minimum RAM.

Observed retained scratch payload per worker for full batches is:

|Pairs|Frozen bytes|OFF bytes|ON bytes|Original record allowance bytes|
|---|---:|---:|---:|---:|
|32|2560|2560|7680|11520|
|200|16000|16000|48000|72000|
|352|28160|28160|84480|126720|

The new cache triples these record payloads. Tail-buffer bytes are additionally reported and bounded in the preflights.

The private snapshot has 176 files, with 171 of 175 originals exact. Four reversible original edits cover msm/lib adapter integration and the two G2 kernels; one child holds caller adapters with correct record-byte introspection. All 17 field files are byte-exact and their 33-test qualification is inherited. Native unfiltered core suites pass 128 OFF/129 ON tests and prover suites pass eight each.

## Measurement method

Ryzen 9 9950X. Owned unsigned and borrowed signed G2 prefix batches use 32/200/352 pairs on one CPU, 16 physical cores and 32 logical CPUs. Fifteen fixed rotated/reversed blocks compare exact frozen accepted libraries, fresh OFF and ON in a common fat-LTO harness. All frozen source/flags/library identities are checked. Each worker performs three checked/reset 1,024-call primes and 8,192 timed complete batch calls. Ready/start/done/release barriers exclude fixture allocation and generator oracles from every worker's clock. Whole-wave timings include inversion, packing, arithmetic, output scatter and the barrier tail. These fixtures do not model proof witness distributions or concurrent proofs.

Ninety preflights include 199/351-pair tails for correctness; 810 latency processes cover the fixed 18 contexts. Every output agrees with independent generator scalar sums and cross-variant digests/reference bytes. Capacity bytes are intentionally variant-specific and bounded, rather than forced equal.

## Caller latency

Microseconds per complete wave, median of 15 process means. Ratios pair process results and need not equal ratios of medians. A negative percentage means faster.

|Caller|Pairs|Workers|Frozen µs|OFF µs|ON µs|ON/OFF paired change|Bounds|Wins / 15|
|---|---:|---:|---:|---:|---:|---:|---|---:|
|owned|32|1|1.881099|1.882225|2.055577|+9.288%|[1.084759514, 1.100537520]|0|
|owned|32|16|2.129473|2.134947|2.287468|+7.183%|[1.068809051, 1.083800152]|0|
|owned|32|32|3.179477|3.180096|3.409421|+7.093%|[1.063407152, 1.078526604]|0|
|owned|200|1|7.823716|7.830498|7.936928|+1.325%|[1.001797895, 1.026021117]|2|
|owned|200|16|9.070795|9.086713|9.305422|+1.616%|[1.012471688, 1.025973447]|1|
|owned|200|32|14.705868|14.669805|15.051255|+1.657%|[1.011826737, 1.028277610]|0|
|owned|352|1|13.209938|13.210556|13.314851|+0.897%|[0.998667541, 1.017131949]|4|
|owned|352|16|15.593495|15.561554|15.652135|+0.809%|[1.001540975, 1.030330505]|3|
|owned|352|32|25.466947|25.476323|25.745776|+1.391%|[1.008285887, 1.020065633]|1|
|borrowed signed|32|1|1.963262|1.962642|1.953177|-0.721%|[0.986592410, 0.995869793]|13|
|borrowed signed|32|16|2.185138|2.187255|2.161863|-0.632%|[0.984639826, 0.996386040]|12|
|borrowed signed|32|32|3.310459|3.292575|3.326664|+1.381%|[0.997082001, 1.019552125]|4|
|borrowed signed|200|1|8.466771|8.481082|8.415861|-0.757%|[0.981319992, 0.999742992]|12|
|borrowed signed|200|16|9.813306|9.844448|9.790373|-1.067%|[0.981592908, 0.997780809]|13|
|borrowed signed|200|32|15.932301|15.919883|16.022791|+0.595%|[0.998750700, 1.011750221]|5|
|borrowed signed|352|1|14.258499|14.281429|14.246242|-0.179%|[0.989377720, 1.006825165]|8|
|borrowed signed|352|16|16.848251|16.820996|16.837167|-0.095%|[0.995381853, 1.007886580]|8|
|borrowed signed|352|32|27.378848|27.506162|27.581302|+0.405%|[1.002731730, 1.013322788]|2|

|Aggregate|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all18/on/frozen|1.017859445|[1.015400839, 1.018940953]|0|
|all18/on/off|1.017241273|[1.015096934, 1.019725284]|0|
|threaded12/on/frozen|1.018575126|[1.014903964, 1.020916122]|0|
|threaded12/on/off|1.017953293|[1.015005397, 1.020129986]|0|

Every OFF/frozen paired interval must lie inside [1/1.03,1.03], and every ON/control upper ratio must be ≤1.03. Both all-18 and threaded-12 equal-log aggregates require upper <1 and at least ten wins against both controls. Ranks 4–12/15 give nominal 96.484375% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. There is no gain floor, subset omission, sample extension, threshold change, seed/layout selection or unchanged rerun.

The positive borrowed-200 context improves 0.757% solo and 1.067% at 16 physical cores versus OFF, with 12/15 and 13/15 wins. At 32 logical workers its 0.595% slowdown estimate has bounds spanning parity. Owned 32-pair batches regress 9.29% solo and about 7.1% at either threaded topology. These retained subcases do not rescue the failed universal gate.

Failed checks:

- `all18/on/frozen/gain`
- `all18/on/off/gain`
- `g2-prefix-32-logical32/on/frozen`
- `g2-prefix-32-logical32/on/off`
- `g2-prefix-32-physical16/on/frozen`
- `g2-prefix-32-physical16/on/off`
- `g2-prefix-32-solo/on/frozen`
- `g2-prefix-32-solo/on/off`
- `g2-prefix-352-physical16/on/off`
- `threaded12/on/frozen/gain`
- `threaded12/on/off/gain`

## Profiling and verification

All latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles: borrowed/200 at 16 and 32 workers, and owned/200 at 32 workers, OFF/ON. Each worker executes 262,144 profiled calls after primes/reset. CPU IDs and timestamps are explicit. All recordings finish before ownership transfer, owner-user decoding and lost-record audits; all helpers were compiled before capture and no owned compiler/helper/hash/archive or second CPU workload overlaps it. Selected linked code is inspected with read-only objdump.

|Profile|Samples|Lost records|
|---|---:|---:|
|profile-g2-borrowed-signed-200-logical32-off|26323|0|
|profile-g2-borrowed-signed-200-logical32-on|26396|0|
|profile-g2-borrowed-signed-200-physical16-off|8112|0|
|profile-g2-borrowed-signed-200-physical16-on|8095|0|
|profile-g2-prefix-200-logical32-off|24329|0|
|profile-g2-prefix-200-logical32-on|24781|0|

Every profile has one sampled process and zero lost records. All 930 capture/offline steps complete, covering 196020160 batch calls including primes and profiles. Source and receipt archives round-trip byte-exactly. Raw recordings and ELF binaries remain local with hashes. Flat samples are not exclusive wall-time or cache/allocator/clock attribution.

The selected borrowed-G2 function retains 1,450 static IFMA sites in every executable. Its code size is 51,942 bytes in frozen/OFF and 51,764 in ON. Neither a smaller function nor fewer input-packing expressions proves faster execution. Additional record traffic and register lifetimes are plausible tradeoffs, but this experiment does not isolate their costs or identify a hardware stall cause. No SDK trial or unchanged cache/store-layout sweep follows this failed caller result.

## Decision and accepted prover comparisons

This exact caller does not advance under its declared gate. Every positive context and failed check remains in the record; production is unchanged and no favorable subset is promoted as passing.

No SDK requests, proof exports or Gnark verifications are added. The distinct-proof ledger remains **22,144**. The original all-eight CPU AVX single-proof 10× objective stays active and unmet, 0/8. Accepted local 1×2 remains 8.022165 ms with 134 MiB peak RSS; latest AWS remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. The accepted physical-16 calibration engineering exception and separate final-artifact gate remain documented. [All nine local/AWS rows and original eight supplied CPU/Metal references](../2026-10-29-current-cpu-summary/README.md).

No AWS, phone/ARM/TEE, forester 250/4999, memory-pressure/swap, minimum-RAM or concurrent-throughput result follows. [Research scope and sources](artifacts/research-audit.md), [protocol](artifacts/design.md), [all observations and checks](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [archive review](artifacts/publication-review.json).
