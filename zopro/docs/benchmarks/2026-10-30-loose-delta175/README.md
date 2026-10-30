# Bounded loose delta in G2 coordinate multiplication

30 October 2026. The separately declared complete-caller gate **passes**. Production remains unchanged; these are actual batch-call timings, not proof timings.

The all-18 equal-log aggregate improves **0.405% versus fresh OFF**, ratio 0.995945397 [0.994375835, 0.996459571], with 14/15 wins. Against frozen control it improves **0.844%**, ratio 0.991561425 [0.990400977, 0.993886951], with 15/15 wins. The threaded-12 aggregate improves **0.466% versus OFF**, ratio 0.995342207 [0.994461179, 0.998481000], with 13/15 wins. All calibration and nonregression checks pass; no failed check is waived.

These aggregate gains select a separate full SDK experiment. Individual cells can still span parity: borrowed 200-pair calls at 32 workers have ON/OFF ratio 0.995330931 [0.993716932, 1.002293063]. This result does not establish a gain in every batch size or a faster proof.

## Arithmetic and bounds

The final G2 Y product `slope*(anchor-X)` uses a private normalized delta `D=2p+anchor-X`. Anchor coordinates are canonical (<p); slope and X are below 2p, so 0≤D≤3p including test endpoints. Positive fused real/imaginary products are bounded by 12p². Since 4p<R256, REDC(T)<4p, and the original conditional subtraction of 2p returns a value below 2p. R260 has enough headroom for the original uncorrected result below 2p. No loose operand escapes this operation.

The existing four raw products, two REDCs and 300 IFMA sites remain; two full modular subtractions become fixed-offset carry-only preparations. The product recurrence, norm/inversion prefixes, slope, square, X/final Y subtraction, scalar tails and exceptional handling remain. No heap allocation, buffer, persistent key table, point copy, plan or pool is added; scratch and forecasts are unchanged. This differs from previous G1 loose-export, lazy-quotient and complement-CIOS experiments, whose failed gates remain. [Prior work and papers](artifacts/research-audit.md), [exact design](artifacts/design.md).

The independent BigInt model passes 103,424 cases and 206,848 outputs over both primes and radices, including Cartesian boundary inputs, random values, chains, normalized-delta and exact unsigned CIOS/REDC checks. The first model compilation had a six-entry state initializer typo; it was corrected before model execution or native work, preserving the original source and compiler error. No successful work repeats. [Repair record](artifacts/pre-build-repairs.md).

Native ON passes all 35 field tests, including two new Arkworks field/range/chained-output tests; OFF/ON each pass 128 unfiltered core and eight prover tests. The original field-operation bodies remain unchanged; one field parent includes the new helper. Six original parents change reversibly, 169 of 175 originals stay exact, and 177 private files include two children. Original common caller adapters/fixtures and frozen accepted libraries are reused after source, flags, rlib and proc-macro verification. Two shared FIFO-control tests also pass. The private prototype targets the native ifma8 feature; its guards need that explicit feature condition before four-lane/portable integration. [Pre-capture review](artifacts/pre-capture-review.md).

## Complete callers

Ryzen9950X, CPU2solo/CPUs0–15physical16/CPUs0–31logical32. Owned andborrowedsignedG2prefix callers at32/200/352pairs make18contexts. Additional199/351tails receive correctnesspreflights. Fifteenfixedrotated/reversedblocks comparefrozen/OFF/ON,8192calls/worker after3checked/reset1024primes. Each complete wave includes inversion, packing, arithmetic, output/scatter and worker participation. No source-level arithmetic gain predicts latency. Every output agrees with an independent generator-coefficient oracle and canonical output digests agree acrossvariants; immutableinputs/capacitieschecked,zero timedallocations.

|Caller|Pairs|Workers|Frozen µs|OFF µs|ON µs|ON/OFF change|Bounds|Wins/15|
|---|---:|---:|---:|---:|---:|---:|---|---:|
|owned|32|1|1.920688|1.882294|1.876616|-0.317%|[0.988159378, 1.002994028]|10|
|owned|32|16|2.165938|2.131793|2.121151|-0.419%|[0.992400750, 0.997418890]|13|
|owned|32|32|3.219597|3.176310|3.165675|-0.355%|[0.990586534, 1.002749503]|9|
|owned|200|1|7.871063|7.843575|7.759736|-0.981%|[0.980008332, 1.003124056]|11|
|owned|200|16|9.146439|9.087566|9.037705|-0.630%|[0.984381532, 1.004503699]|11|
|owned|200|32|14.729085|14.658434|14.700406|+0.136%|[0.996705286, 1.010154626]|7|
|owned|352|1|13.244690|13.193660|13.108496|-0.818%|[0.984810726, 1.003899297]|11|
|owned|352|16|15.639531|15.505692|15.395823|-1.034%|[0.985998960, 0.992451594]|14|
|owned|352|32|25.366180|25.461676|25.249028|-0.051%|[0.991566751, 1.003388294]|9|
|borrowed signed|32|1|1.962090|1.954417|1.945561|-0.502%|[0.990531864, 0.997505889]|13|
|borrowed signed|32|16|2.187660|2.184486|2.171290|-0.654%|[0.984863105, 0.998817098]|13|
|borrowed signed|32|32|3.305646|3.324835|3.293139|-0.389%|[0.984072903, 1.004174572]|9|
|borrowed signed|200|1|8.472019|8.484225|8.439710|-0.526%|[0.988344374, 1.005606424]|10|
|borrowed signed|200|16|9.843801|9.811733|9.752194|-0.620%|[0.989678097, 0.997873656]|12|
|borrowed signed|200|32|15.915062|15.958807|15.870599|-0.467%|[0.993716932, 1.002293063]|10|
|borrowed signed|352|1|14.248686|14.323442|14.180191|-1.067%|[0.983942594, 1.003586598]|10|
|borrowed signed|352|16|16.798540|16.804264|16.705903|-0.400%|[0.993690524, 1.004852641]|10|
|borrowed signed|352|32|27.483806|27.374247|27.368629|-0.227%|[0.995763623, 1.003878060]|8|

|Aggregate|Ratio|Bounds|Wins / 15|
|---|---:|---|---:|
|all18/on/frozen|0.991561425|[0.990400977, 0.993886951]|15|
|all18/on/off|0.995945397|[0.994375835, 0.996459571]|14|
|threaded12/on/frozen|0.992392319|[0.991404171, 0.995478723]|14|
|threaded12/on/off|0.995342207|[0.994461179, 0.998481000]|13|

Every OFF/frozen paired interval must lie inside [1/1.03,1.03], and every ON/control upper ratio must be ≤1.03. Both all-18 and threaded-12 equal-log aggregates require upper <1 and at least ten wins against both controls. Ranks 4–12/15 give nominal 96.484375% individual median coverage under independent/exchangeable blocks, not a familywise guarantee. There is no gain floor, subset omission, sample extension, threshold change, seed/layout selection or unchanged rerun.

Failed checks:

None.

## Profiling and verification

All latency processes precede six separate FIFO-gated 199 Hz cycles:u profiles: borrowed/200 at 16 and 32 workers, and owned/200 at 32 workers, OFF/ON. Each worker executes 262,144 profiled calls after primes/reset. CPU IDs and timestamps are explicit. All recordings finish before ownership transfer, owner-user decoding and lost-record audits; all helpers were compiled before capture and no owned compiler/helper/hash/archive or second CPU workload overlaps it. Selected linked code is inspected with read-only objdump.

|Profile|Samples|Lost records|
|---|---:|---:|
|profile-g2-borrowed-signed-200-logical32-off|26105|0|
|profile-g2-borrowed-signed-200-logical32-on|26091|0|
|profile-g2-borrowed-signed-200-physical16-off|8109|0|
|profile-g2-borrowed-signed-200-physical16-on|8117|0|
|profile-g2-prefix-200-logical32-off|24299|0|
|profile-g2-prefix-200-logical32-on|24321|0|

Every profile has one sampled process and zero lost records. All 930 capture/offline steps complete, covering 196020160 batch calls including primes and profiles. Source and receipt archives round-trip byte-exactly. Raw recordings and ELF binaries remain local with hashes. Flat samples are not exclusive wall-time or cache/allocator/clock attribution.

All samples are retained, including the unpinned coordinator; each worker sample is checked against its recorded TID/CPU pair. Absolute-period coverage remains unvalidated. A separate offline analyzer corrected the original process-wide affinity assumption without repeating captures. [Analysis repair](artifacts/offline-analysis-repair.md), [exact corrected analyzer](artifacts/analysis-fixed.rs).

## Decision and accepted prover comparisons

This result selects separate complete-SDK accepted/OFF/ON qualification across all nine circuits and subsequent resource/portable checks. It does not establish faster proofs or adopt the implementation.

No SDK requests, proof exports or Gnark verifications are added. The distinct-proof ledger remains **22,990**. The original all-eight CPU AVX single-proof 10× objective stays active and unmet, 0/8. Accepted local 1×2 remains 8.022165 ms with 134 MiB peak RSS; latest AWS remains Zopro 11.915591 ms at 32 workers versus pinned Aeglos Sorted 7.959184 ms at 96. The accepted physical-16 calibration engineering exception and separate final-artifact gate remain documented. [All nine local/AWS rows and original eight supplied CPU/Metal references](../2026-10-29-current-cpu-summary/README.md).

No AWS, phone/ARM/TEE, forester 250/4999, memory-pressure/swap, minimum-RAM or concurrent-throughput result follows. [Research scope and sources](artifacts/research-audit.md), [protocol](artifacts/design.md), [all observations and checks](artifacts/analysis.json), [source audit](artifacts/source-audit.json), [source](artifacts/source.json.gz), [receipts](artifacts/receipts.json.gz), [archive review](artifacts/publication-review.json).
