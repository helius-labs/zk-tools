# Nonnegative Gauss in the borrowed G2 caller

**Rejected at the actual-caller gate; no SDK trial or adoption.** Across the two G2 cases at 32 workers, the paired aggregate is 8.22% slower than fresh OFF [ratio 1.080324–1.092389], with 0/31 wins. It is 9.02% slower than frozen [1.085732–1.094945], with 0/31 wins. All control calibrations pass; gain and nonregression gates fail. Production remains unchanged.

## Change and scope

The exact method qualified on 26 October uses three raw products and two reductions: 250 IFMAs per complex product versus 300 in the accepted implementation. It replaces slope and final-y products in the current borrowed signed G2 reverse pass. Ordinary denominator conjugation is included. Norms, scaling, batch inversion, prefix records, tails, geometry and scratch remain original. There is no heap addition, per-key table or copied point array. The earlier reduced Gauss caller used three ordinary products and three reductions. [History audit](artifacts/history-audit.md).

The old component aggregate passed, but its affine expression was 2.81% slower. This caller result closes the missing integration question for this recipe: the isolated arithmetic gain did not transfer.

## Fixed caller comparison

Ryzen 9950X, 16 physical cores and 32 logical CPUs. Solo uses CPU 2; physical16 uses CPUs 0–15; logical32 uses CPUs 0–31. Three coherent private native builds compare frozen current source, fresh OFF and ON. Thirty-one fixed rotated/reversed blocks cover nine contexts; each worker runs 8,192 measured batches after three separately reset 1,024-batch primes. The statistic below is wall-clock microseconds per batch wave, including completion of all independent workers. Paired changes are medians of within-block ratios and need not equal ratios of displayed medians.

|Caller|Size|Workers|Frozen µs|Fresh OFF µs|Gauss µs|Paired change/OFF|Wins/OFF|
|---|---:|---:|---:|---:|---:|---:|---:|
|G2 borrowed signed|200|1|8.474|8.423|9.026|+7.107%|0/31|
|G2 borrowed signed|200|16|9.781|9.606|10.497|+8.373%|0/31|
|G2 borrowed signed|200|32|15.820|15.857|17.219|+8.516%|0/31|
|G2 borrowed signed|352|1|14.258|14.180|15.281|+7.816%|0/31|
|G2 borrowed signed|352|16|16.762|16.372|18.006|+8.600%|0/31|
|G2 borrowed signed|352|32|27.198|27.385|29.707|+8.592%|0/31|
|G1 unchanged control|200|1|4.368|4.353|4.339|-0.308%|22/31|
|G1 unchanged control|200|16|4.881|4.834|4.862|+0.314%|5/31|
|G1 unchanged control|200|32|7.035|7.073|7.006|-0.475%|22/31|

Both G2 sizes regress at every tested topology, with 0/31 wins against fresh OFF. The unchanged G1 control has small positive estimates at solo/logical32 and a small regression at physical16. Its arithmetic was unchanged; these are control observations rather than a demonstrated Gauss benefit.

The predeclared gate requires all nine fresh/frozen rank intervals within factor 1.03, all candidate upper nonregression ratios ≤1.03 versus both controls, and both logical32 G2 cases individually plus their equal-log-weight aggregate to have upper ratios <1 and at least 21/31 wins versus both controls. There is no minimum gain floor. Ranks 10–22 of 31 give nominal 97.055% individual coverage conditional on independent exchangeable blocks, without a familywise guarantee. All observations are retained. No exclusions, extensions, successful reruns, threshold changes or subset promotion occur. [Design](artifacts/design.md), [analysis](artifacts/analysis.json), [observations](artifacts/observations.json).

## Correctness and memory

All three configurations pass 128 unfiltered native core tests each. The new borrowed signed G2 test checks scalar-generator results at 13 lengths, including empty batches, tails and full groups, with 1/3/17 reuse counts. The exact old method and unchanged IFMA base allow inheritance of 731,136 Arkworks lane comparisons and 91,392 exact raw cases; these were not rerun or counted as new proofs. Current integration tests and all 868 capture processes check independent scalar-generator outputs. All mode output digests match.

Every timed interval allocates zero times. G2 scratch stays 16,000 bytes per worker at size 200 and 28,160 bytes at size 352; diagnostic borrowed reference vectors stay 3,200/5,632 bytes. G1 control scratch stays 40,000 bytes with 3,200 reference bytes. These are caller capacities, not full proof peak RAM or minimum RAM. No phone/ARM/TEE or memory-pressure qualification follows.

An initial setup audit tried UTF-8 decoding of an unchanged binary fixture; it was repaired before builds. The first ON compilation failed because the replacement removed array-length inference. One explicit local `&[FqBatch;2]` alias repaired inference without changing the formula or protocol. Failed sources/logs and the repair are retained. Successful frozen/OFF tests were inherited. [Source audit](artifacts/source-audit.json), [repair](artifacts/repair.json).

## Code and profiles

Linked borrowed G2 functions have 1,450 IFMA instruction sites in both controls and 1,350 in ON, consistent with replacing two complex products. The whole function also grows.

|Build|Instruction sites|IFMA sites|RSP memory operand lines|Literal stack reservation bytes|
|---|---:|---:|---:|---:|
|frozen|8672|1450|1514|6464|
|off|8672|1450|1514|6464|
|on|9142|1350|1779|10880|

These are static function counts, not executed work, measured spill events, exclusive stall time or minimum stack requirements. Fewer IFMAs coexist with additional preparation and larger generated code/stack use; these observations do not isolate a single cause of the regression. [Code audit](artifacts/static-code.json).

Four separate `cycles:u` recordings follow all latency measurements: 199 Hz, 262,144 batches per worker, G2 size 200 at solo/logical32, OFF/ON. FIFO enable follows fixture construction, primes and reset; disable precedes output oracles. All four recordings finish before decoding.

|Profile|Samples|Lost|
|---|---:|---:|
|profile-g2-200-solo-m1|431|0|
|profile-g2-200-solo-m2|465|0|
|profile-g2-200-logical32-m1|25673|0|
|profile-g2-200-logical32-m2|28401|0|

Each recording contains one SDK-free caller process; the actual borrowed G2 apply function leads sampled period. Samples are descriptive and do not establish an exclusive stall cause. Raw perf data and eight full decoded/loss stdout streams stay locally with hashes; analysis retains profile summaries. No owned build/helper/hash/archive/other benchmark overlaps measurement. Only the four privileged perf files' ownership is changed after all recordings; bytes are preserved.

All 27 preflights, 837 latency processes, four profiles and 12 postcapture steps finish (880 workflow steps). All 768 final pins close. Reversing private edits recovers all 175 production files; 170 private originals and 173 frozen originals are exact. The inherited field helper, source snapshots, repair, flags and coherent library receipts remain pinned. Archive review checks all 3,572 original receipt streams byte-for-byte; eight profile loss/decode stdout streams are hash-indexed locally to keep the repository compact. [Run receipts](artifacts/run-receipts.json).

Zero SDK proof requests, exports or new Gnark verifications occur. The independent proof ledger remains 17,811. There is no new AWS or Aeglos result. Accepted source 175 and SDK `20fde…` remain unchanged; the prior full-performance-gate failure and physical16 calibration engineering exception remain.

## Inherited accepted CPU and Metal comparison

Historical accepted local measurements versus supplied M5 references; not remeasured in this caller experiment. Peak RSS for 1×2 remains 134 MiB.

|Shape|Accepted ms|M5 CPU ms|CPU speedup|M5 Metal ms|Metal speedup|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|8.022165|37|4.612×|24|2.992×|3.7|
|2×2|11.335969|59|5.205×|30|2.646×|5.9|
|4×2|15.945544|109|6.836×|48|3.010×|10.9|
|8×2|24.090485|189|7.845×|74|3.072×|18.9|
|12×2|33.844855|284|8.391×|101|2.984×|28.4|
|16×2|39.931557|335|8.389×|124|3.105×|33.5|
|24×2|66.233309|528|7.972×|176|2.657×|52.8|
|merge24×1|65.273224|507|7.767×|175|2.681×|50.7|

Accepted 54→1 remains 162.250464 ms, without a supplied M5 reference. The CPU 10× goal remains active and unmet in all eight shapes. This Gauss caller integration is closed. Next work should use current critical-path profiles to select a distinct operation-removal or arithmetic representation change.
