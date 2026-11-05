# Complete-distribution blocked G2 coordinate codec

The fixed blocked-input loader is **25.67% slower across all six contexts** and **28.75% slower across the two 32-worker contexts** than the fresh control. Every control calibration passes. The candidate fails all twelve non-regression checks and all four aggregate gain checks. **Reject this loader; no caller, SDK, or key-format adoption follows.**

This measures coordinate packing with a common field norm-sum consumer, not full G2 queries or proofs. Each worker replays every full group from one complete captured distribution. Multiple workers duplicate that cycle against shared input to measure the codec under CPU contention; this is not the original query's partitioned scheduling or proof throughput.

## Matched results

Fifteen rotated matched blocks compare the original accepted field library, a fresh control, and the [qualified loader](../2026-11-05-g2-blocked-load176/README.md). All final-link argv receipts include `-O`, `target-cpu=native`, `lto=fat`, and `codegen-units=1`. Workers have 512 KiB stacks and affinity to CPU 2, CPUs 0–15, or CPUs 0–31. Every process primes three complete cycles, then measures eight cycles per worker. All dispatch classes, index lookup, radix conversion, and the common consumer remain inside timing; source decoding, validation, grouping, and in-place encoding occur beforehand.

| Actual root | Workers | Original ms/cycle wave | Fresh control | Blocked loader |
|---|---:|---:|---:|---:|
| 1×2, 30,089 points | 1 | 3.735088 | 3.730417 | 4.622633 |
| 1×2 | 16 | 4.308317 | 4.312301 | 5.379832 |
| 1×2 | 32 | 6.774411 | 6.782259 | 8.657978 |
| 2×2, 28,852 grouped points | 1 | 3.901354 | 3.900235 | 4.803780 |
| 2×2 | 16 | 4.495773 | 4.504851 | 5.574525 |
| 2×2 | 32 | 6.948603 | 6.959136 | 9.034534 |

These are medians of individual arms. Decisions use paired ratios within each block, including every observation. Bounds below are the fourth and twelfth sorted block ratios, not a claimed confidence interval. Aggregate ratios use the geometric mean across the declared contexts in each block. A gain requires the upper bound below one and at least ten of fifteen winning blocks; control calibration and individual non-regression use the predeclared 1.03 bound.

| Aggregate, ON/control | Median ratio | Lower / upper | Winning blocks |
|---|---:|---:|---:|
| All six / original | 1.255414 | 1.249734 / 1.257730 | 0/15 |
| All six / fresh | 1.256729 | 1.250908 / 1.258583 | 0/15 |
| Both 32-worker / original | 1.287047 | 1.282506 / 1.292001 | 0/15 |
| Both 32-worker / fresh | 1.287486 | 1.277787 / 1.296101 | 0/15 |

## Counters and profiles

All four PMU panels report 100% running coverage. Instructions increase approximately **44.48%** at both 16 and 32 workers. Cycles increase in both cases. The 16-worker branch-miss count rises from 2,281 to 5,411,708; at 32 workers it rises from 6,211 to 10,085,075. Reported generic cache-miss counts fall, but execution still slows. These events are not measurements of DRAM traffic, physical cache-line movement, or minimum RAM.

Static inspection of the common cycle finds 300 IFMA sites in every arm. Instruction-line counts increase from 815 to 1,649; this is a static count, separate from dynamic retired instructions. Four automatically captured user-cycle profiles contain **45,086 samples**, with zero recorded loss, all workers observed at their assigned CPUs, and unknown fractions below 0.064%. Almost all samples resolve to the common cycle. Together with the unchanged arithmetic source and increased instruction/branch counts, the evidence supports routing and representation overhead as the regression mechanism; it does not assign an exclusive cycle cost to each instruction.

## Actual inputs and correctness

The [preceding census](../2026-11-05-g2-pack-locality176/README.md) supplies all 64,541 primary full groups and all 66,884 grouped 2×2 full groups. No favorable subset is selected. Scalar tails are outside this codec-only workload and would remain required in a complete caller. X and Y are packed for each group; the common consumer sums their four coordinate squares using unchanged native field arithmetic, ensuring every returned lane and limb contributes to a checked result. It is not the original affine caller.

An append-only diagnostic bridge calls the accepted scalar grouping and `group_affine` implementations. The 2×2 root contains the actual 28,852 witness-dependent grouped points derived from 52,274 original inputs, preserving first-occurrence order. Two untimed complete MSM results match the existing pinned independent Arkworks oracles. A separate scalar norm-sum oracle binds each complete codec cycle; independent review reconstructs both oracles from the private raw roots and every recorded index. All 296 codec processes match their expected outputs across worker counts and arms, including priming checks.

No new field/core/prover tests run here. The two focused loader tests and 35 filtered original field tests are inherited from the qualified exact child. The 177-file diagnostic snapshot has 173 byte-exact originals; the field module export and two append-only core bridges recover all 176 accepted hashes. Existing field arithmetic, generated IFMA code and generator, and original core function bodies remain exact. Production and the accepted SDK remain unchanged.

## Memory and limits

Each process holds one shared root allocation, with exact payload and capacity of **3,851,392 bytes** for primary or **3,693,056 bytes** for grouped 2×2. Candidate encoding preserves the pointer and capacity and needs no padding or retained duplicate root. Timed allocation counts are zero in all arms. This establishes the codec's measured allocation behavior, not full-proof RAM requirements.

The diagnostic also retains the complete shared replay-index vector: 4,130,624 / 4,280,576 logical bytes, with capacities 4,194,304 / 8,388,608 bytes. It is a benchmark input, not a proposed production cache. The setup process temporarily owns decoded original/grouped inputs, and threads reserve stacks. No RSS peak, minimum-RAM, swap/pressure, mobile/ARM, GPU, TEE, or huge-circuit qualification is claimed. Excluded setup costs would have to be charged in a full-query design; this negative screen does not proceed there.

## Review and decision

Independent review checks **397 source/library pins and 325 successful workflow commands**: two library/freeze commands, four optimized links, two actual-root preparations, and 317 capture/diagnostic commands. There are 296 codec processes, including 270 latency processes. Measured workload totals 3,973,383,579 full eight-entry groups including diagnostics, plus 964,510,416 priming groups; these are repeated codec operations, not that many independent inputs or proofs.

Review independently reconstructs scalar oracles, checks receipt/protocol/stream hashes and source recovery, recomputes all paired ratios and sixteen failed gates, verifies the six passing calibrations, and checks PMU/profile ownership and loss. Review succeeds on its first execution; no successful test, preparation, or capture is repeated. The initially pinned `prepare-root.rs` remains preserved; the actual preparer link uses `prepare-root-with-oracle.rs`, which adds the scalar checksum oracle before any root execution. There is no failed native attempt or timing rerun.

Close this fixed loader without a size, worker-count, class-subset, or layout rescue. The useful conclusion is that a high logical two-block hit rate does not establish cheaper execution. The original constructor remains faster on these complete observed distributions. No SDK/Gnark/AWS/Aeglos run is warranted for this candidate. The canonical proof ledger remains 29,953; there are no new proofs or adopted changes.

## Accepted CPU and Metal comparisons, inherited

These are accepted 32-worker local full-proof results and supplied historical M5 references, **not measurements of this codec**. [Accepted manifest](../2026-10-30-g2-vector-sign-adoption176/artifacts/production-source-manifest.json). Latest separate full-SDK evidence remains [finalization/drain composition](../2026-11-05-finalize-drain-bundle176/README.md), preserving its primary gain and failed full gate.

| Shape | Accepted local ms | M5 CPU ms | CPU/local | M5 Metal ms | Metal/local | CPU10× target ms |
|---|---:|---:|---:|---:|---:|---:|
| 1×2 | 7.928660 | 37 | 4.667× | 24 | 3.027× | 3.7 |
| 2×2 | 11.151932 | 59 | 5.291× | 30 | 2.690× | 5.9 |
| 4×2 | 16.086386 | 109 | 6.776× | 48 | 2.984× | 10.9 |
| 8×2 | 24.179453 | 189 | 7.817× | 74 | 3.060× | 18.9 |
| 12×2 | 33.884455 | 284 | 8.381× | 101 | 2.981× | 28.4 |
| 16×2 | 40.077381 | 335 | 8.359× | 124 | 3.094× | 33.5 |
| 24×2 | 65.850777 | 528 | 8.018× | 176 | 2.673× | 52.8 |
| merge24×1 | 65.060937 | 507 | 7.793× | 175 | 2.690× | 50.7 |

Merge54×1 remains 161.883899 ms without a supplied M5 reference. The all-eight CPU10× goal remains active and unmet, **0/8**.

## Artifacts

[Design](artifacts/design.md), [all observations](artifacts/analysis.json), [independent review](artifacts/review.json), [driver](artifacts/driver.rs), [root bridge](artifacts/root-bridge.rs), [capture plan](artifacts/capture-plan.json), [code inspection](artifacts/codegen.json), [reproduction](artifacts/reproduce.md), [source archive](artifacts/source.tar.gz), [exact receipts](artifacts/receipts.json.gz), [checksums](SHA256SUMS).

Source archive members and exact UTF-8 receipt contents are checked after round trip. Executables, libraries, private root/expected-value/ordinal/scalar/point/key/image/witness arrays, and raw perf data remain excluded. Published private paths and hashes identify locally retained evidence; independent reproduction needs authorized fixture inputs.
