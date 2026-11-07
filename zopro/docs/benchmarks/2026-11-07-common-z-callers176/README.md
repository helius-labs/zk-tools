# Segmented common-Z G1 reducer callers

**Not adopted.** The fixed integration fails four control-calibration checks and six nonregression checks. Calibrated small-frontier gains remain useful: sixteen-point reductions are about twice as fast, and 257–511-point callers gain roughly 6–9% at 32 workers. These are reducer waves; accepted full-proof latency is unchanged.

The declared equal-log aggregate across all ten cases at 32 workers gives these ON/control ratios:

|Control|Paired ratio [bounds], winning blocks|
|---|---|
|frozen|0.845401836 [0.840708168, 0.847345995], 15/15|
|fresh|0.843129881 [0.840116393, 0.844913335], 15/15|
|OFF|0.840504398 [0.839172271, 0.842439325], 15/15|

All three aggregate gain checks pass, with 15/15 winning blocks. That equal-case aggregate is not weighted by production proof work and does not override failed calibration/nonregression checks. All thirty same-binary ON/OFF nonregression checks pass. Six ON/original-or-fresh upper bounds exceed the declared 1.03 limit in three contexts; those contexts also have control-calibration failures. This fixed implementation is closed without an SDK trial, subset adoption, parameter or compiler rescue, extension or unchanged rerun.

## Algorithm and integration

The [parent mathematical screen](../2026-11-07-common-z-product-tree176/README.md) and [standalone IFMA prototype](../2026-11-07-common-z-small-native176/README.md) remain immutable. The new segmented implementation finishes all live groups together when their entire task frontier fits sixteen points and some group has at least three points. Those rules come from one eight-lane pair vector and at least two remaining reduction levels, before timing.

For each pair, d=dx and e=dy. The shared denominator product D and all-other cofactors q produce u=e*q, X'=u²−(X1+X2)D², Y'=u(X1D²−X')−Y1D³, Z'=ZD. Odd carried and singleton-group points rescale with D²/D³. All final groups share one scalar inverse for normalization. Equal-x/identity cases at any level decline transactionally; the original reducer continues from its untouched frontier. No partial output escapes.

The hooks sit in group_affine_impl and indexed-tile unweighted loops, with the same hook in the diagnostic reducer. reduce_layer and weighted_sum retain their original one-level contract. G2 retains its original arithmetic. The runtime option is read once in Scratch::default. The kernel adds no heap scratch, field implementation, unsafe code, key table/copy, retained plan or production pool. Its fixed stack arrays and compiler spills are additional to the observed heap counters.

## Qualification and provenance

All 131 changed native core tests pass, including segmented compositions through sixteen points, odd/carry/zero/singleton/mixed groups, discontiguous slots, unchanged inputs, late equal/opposite/identity fallback, and complete production grouping/indexed outputs on 1/4/16/32-worker pools requesting 512 KiB stacks. Parent field and standalone suites are inherited rather than rerun. The unchanged field library is `1200e635…`.

The private snapshot has 180 files: 178 diagnostic originals recover exactly after reversing eight recorded edits, with two new child files. The earlier diagnostic snapshot retains all 176 accepted originals, including 174 byte-exact original files and two append-only diagnostic parents. Successful runtime libraries are built once. A failed test-only compile required replacing `assert_eq!` with `assert!` for private limbs lacking `Debug`; runtime source before `cfg(test)` remains exact and both successful libraries are reused. The failure and repair remain archived.

The reused original diagnostic library, a fresh cfg-disabled library, and one candidate library supply four arms: original/frozen, fresh, candidate OFF and the same candidate ON. Actual consumers use O3, native CPU, fat LTO and one codegen unit. The qualified TLS allocation/perf harness changes only the three added cases and fresh label. Seven older fixture output digests are inherited and matched; unchanged harness qualification is reused.

## Fixed measurements

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Workers pin individually to CPU 2, CPUs 0–15 or 0–31, with 512 KiB stacks. Each worker owns a fixture and reducer. Three checked eight-wave primes precede the timed loop. Timers include dispatch/wait, gathers/conversions, ordinary reduction, common-Z finish, final normalization and the case's normal allocation/drop. Pool/fixture construction and final output checks are outside timing. These are complete reducer caller waves, not SDK requests or concurrent-proof throughput.

Fifteen fixed rotated/reversed blocks compare all ten cases at all three worker counts. Calibration requires fresh/original and OFF/original intervals inside [1/1.03, 1.03]; ON/control nonregression requires an upper bound ≤1.03 in every context. Three aggregate gains at 32 workers require an upper bound <1 and at least 11 wins. Observed heap capacity must not grow more than 5% against any control. There is no gain floor, excluded block, extension, parameter sweep or rerun. Ranks 4–12 of 15 give nominal 96.484375% marginal coverage under independent exchangeable blocks, with no familywise guarantee.

Capture completes all 1,949 commands on their first executions: 120 preflights, one preflight review, 1,800 latency processes, four counter processes, four profile processes and twenty offline commands. Every one of 1,928 caller processes checks its complete output; intermediate timed iterations are not each separately oracled.

Times below are microseconds per wave. Paired ratios need not equal ratios of marginal medians.

|Context|Original / fresh / OFF / ON µs|ON/OFF ratio [bounds], wins|
|---|---:|---|
|tile257-w1|12.567845 / 12.500784 / 12.549615 / 10.866518|0.866502387 [0.858587247, 0.868531150], 15/15|
|tile257-w16|14.079154 / 14.073961 / 14.059553 / 12.275529|0.872747239 [0.869526766, 0.874514312], 15/15|
|tile257-w32|19.320534 / 19.336026 / 19.494953 / 17.934365|0.912127175 [0.906058358, 0.925222847], 15/15|
|tile384-w1|16.173228 / 16.300238 / 15.971391 / 14.014304|0.881303742 [0.874223026, 0.887168038], 15/15|
|tile384-w16|18.188629 / 18.236259 / 18.179124 / 16.066623|0.881172584 [0.879466956, 0.885687037], 15/15|
|tile384-w32|24.961714 / 25.137929 / 25.199808 / 23.049370|0.917488723 [0.900496782, 0.933563610], 15/15|
|tile511-w1|19.048154 / 18.956084 / 19.112680 / 17.377558|0.905460696 [0.902667420, 0.913921919], 15/15|
|tile511-w16|21.667401 / 21.652907 / 21.716995 / 19.916024|0.917839569 [0.914721304, 0.918852849], 15/15|
|tile511-w32|30.182614 / 30.059057 / 30.072736 / 28.381897|0.940220744 [0.928800468, 0.947101672], 15/15|
|tile2048-w1|56.651085 / 56.812867 / 55.381802 / 53.908179|0.972549286 [0.967655618, 0.982017871], 15/15|
|tile2048-w16|64.856046 / 64.959518 / 65.361950 / 63.545322|0.972495316 [0.970010655, 0.975423962], 15/15|
|tile2048-w32|93.164340 / 94.909026 / 98.081913 / 95.797256|0.975311806 [0.968890167, 0.980315916], 13/15|
|group35973-w1|929.947648 / 929.300117 / 923.213570 / 921.222258|0.998390871 [0.997392586, 1.001095446], 9/15|
|group35973-w16|6298.927148 / 6295.945125 / 6493.247922 / 6487.144234|0.999153622 [0.992718621, 1.013091357], 8/15|
|group35973-w32|24626.318156 / 24713.787805 / 24536.508492 / 24573.757187|1.003755021 [0.998798754, 1.004472783], 5/15|
|groups31x256-w1|198.376887 / 198.502426 / 197.602988 / 198.276955|1.002479784 [1.000304688, 1.007385446], 3/15|
|groups31x256-w16|231.882158 / 231.513373 / 230.864695 / 231.249867|1.000629666 [0.997173789, 1.006169803], 7/15|
|groups31x256-w32|392.444625 / 395.348023 / 395.342693 / 393.221656|0.994134184 [0.988753086, 1.005037979], 9/15|
|tails-w1|11.647548 / 11.639171 / 11.730054 / 11.630644|0.991627215 [0.989172278, 1.003938120], 9/15|
|tails-w16|13.099157 / 13.051184 / 13.057380 / 13.099627|1.002757274 [0.995626538, 1.003577270], 6/15|
|tails-w32|17.882725 / 17.949222 / 18.390658 / 18.781966|1.019141844 [0.996361654, 1.027197011], 4/15|
|small16-w1|3.379774 / 3.396164 / 3.328051 / 1.610030|0.484746797 [0.481014407, 0.491132875], 15/15|
|small16-w16|3.737687 / 3.742277 / 3.711433 / 1.818065|0.489911278 [0.488122547, 0.493157318], 15/15|
|small16-w32|5.391957 / 5.398372 / 5.318727 / 2.688791|0.505704368 [0.500968433, 0.507209281], 15/15|
|groups3x4-w1|2.055181 / 2.034341 / 2.022286 / 1.263095|0.627066812 [0.623562887, 0.629378800], 15/15|
|groups3x4-w16|2.254453 / 2.248654 / 2.219914 / 1.427315|0.640753386 [0.638862208, 0.642902694], 15/15|
|groups3x4-w32|3.306349 / 3.307918 / 3.267610 / 2.063081|0.631515352 [0.624208831, 0.636607488], 15/15|
|small-mixed-w1|1.905832 / 1.920952 / 1.874446 / 1.338631|0.710696766 [0.707753973, 0.717314838], 15/15|
|small-mixed-w16|2.104450 / 2.103459 / 2.079537 / 1.495176|0.719082896 [0.716567101, 0.723628971], 15/15|
|small-mixed-w32|3.079376 / 3.072872 / 3.013115 / 2.150791|0.705035086 [0.700291702, 0.715860003], 15/15|

### Failed checks

- `tile2048-w32/fresh/frozen/calibration`
- `tile2048-w32/OFF/frozen/calibration`
- `tile2048-w32/ON/frozen/nonregression`
- `tile2048-w32/ON/fresh/nonregression`
- `group35973-w16/OFF/frozen/calibration`
- `group35973-w16/ON/frozen/nonregression`
- `group35973-w16/ON/fresh/nonregression`
- `tails-w32/OFF/frozen/calibration`
- `tails-w32/ON/frozen/nonregression`
- `tails-w32/ON/fresh/nonregression`

The 32-worker tile2048 control interval is too wide for fresh/original and systematically above the declared band for OFF/original. OFF/original also fails at group35973/16 workers and tails/32 workers. The corresponding ON/original-or-fresh comparisons retain their actual unfavorable bounds; no universal nonregression or whole-proof gain is claimed.

## CPU profiles and memory

All four PMU sets run 100% of their measurement windows. All four warm-gated profiles pass: 115,598 samples, zero lost records, worker sample share 100%, maximum unknown share 1.7866%. They use `cycles:u` at a fixed period of 4,000,000, 512 pages, and FIFO enable after checked primes / disable before final oracles. Independent review reconstructs sample PID/TID/CPU/period headers. It does not independently reparse exact ELF/IP ownership or LOST records.

|Counter context, ON/original|Cycles|Instructions|Cache misses|
|---|---:|---:|---:|
|tile511-w32|-8.49%|-10.57%|+50.56%|
|groups3x4-w32|-41.19%|-42.67%|-16.16%|

The shorter small-group arithmetic and reduced inverse-bridge samples support the intended mechanism. Flat samples and generic cache counters do not establish removable wall fractions or a causal cache/DRAM explanation. Tile511 cache misses increase even while its measured cycles decrease.

Requested worker heap capacity is unchanged for the seven inherited cases. At small16 it falls from 3,640 to 1,976 bytes, groups3x4 falls from 1,752 to 1,720, and small-mixed falls from 1,888 to 1,856. `Scratch::default`'s diagnostic option read adds one worker allocation in fresh-buffer cases; that cost is timed. Retained-buffer cases have zero timed worker allocations. TLS counters exclude main/coordinator allocations.

At 32 workers the largest synthetic group has 355 MiB whole-process high-water RSS in both original and ON; tile511 has 3 MiB in both. These counters include fixture/setup lifetimes and do not establish a rigorous incremental peak, prover RAM, admission bound or minimum RAM. Small fixed stack arrays and spills are outside heap capacity. No pressure, portable/mobile, huge-circuit or concurrent-proof qualification is added.

## Review, decision and accepted comparison

Offline review independently reconstructs all 243 timing/capacity checks, the same ten failures, all 1,928 output/worker receipts, 1,949 command/stream receipts, source reversals and all 115,598 sample headers. An initial reader used exact floating equality on a JSON round-trip and stopped at a one-ULP difference; the corrected separate reader permits 1e-14 for reconstructed floating summaries, with unchanged thresholds and gate decisions. Both versions and failed receipts are retained. Neither analysis nor successful arithmetic, capture or decode is rerun.

The tested fixed integration is closed without adoption or an SDK trial. Calibrated small-frontier gains remain established component evidence. A future mathematical construction must justify new work rather than rescue this fixed gate through a worker/layout/compiler change.

The following latest accepted local 32-worker full-proof observations and supplied historical M5 references are inherited, not this experiment's timings. [Accepted source manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json), [full-proof evidence](../2026-11-06-window-batch-proof-capture176/README.md).

|Shape|Accepted local ms|M5 CPU ms|M5 Metal ms|CPU/local|Metal/local|CPU10× target ms|
|---|---:|---:|---:|---:|---:|---:|
|1×2|7.915095|37|24|4.675×|3.032×|3.7|
|2×2|11.169812|59|30|5.282×|2.686×|5.9|
|4×2|16.085882|109|48|6.776×|2.984×|10.9|
|8×2|24.517520|189|74|7.709×|3.018×|18.9|
|12×2|33.838626|284|101|8.393×|2.985×|28.4|
|16×2|40.083672|335|124|8.358×|3.094×|33.5|
|24×2|65.506198|528|176|8.060×|2.687×|52.8|
|merge24|65.200982|507|175|7.776×|2.684×|50.7|

Accepted merge54 is 161.718956 ms, without a supplied M5 reference. The all-eight CPU 10× goal remains active and unmet, 0/8. No new SDK request, proof, Gnark export or AWS/Aeglos result is produced; the canonical distinct-proof ledger stays at 31,387. Production source 176 and SDK `6d83…` remain unchanged.

[Design](artifacts/design.md), [complete results](artifacts/analysis.json), [independent review](artifacts/independent-review.json), [lossless observations](artifacts/observations.json.gz), [lossless command receipts](artifacts/receipts.json.gz), [exact raw streams including failed stages](artifacts/raw-streams.json.gz), [source audit and inverse edits](artifacts/source-audit-after-test-repair.json), [capture plan](artifacts/capture-plan.json), [capture pins](artifacts/capture-pins.json), [local binary/raw-profile identities](artifacts/local-artifacts.json), [checksums](SHA256SUMS). Public artifacts contain no actual key, witness, proof, native executable or library payload.
