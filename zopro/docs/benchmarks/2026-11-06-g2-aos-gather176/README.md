# Direct AoS gathers for G2 coordinate loading

The fixed loader is **45.73% slower across all six contexts** and **54.03% slower across both 32-worker contexts** than fresh control, with zero winning blocks in either aggregate. All six control calibrations pass. The candidate fails all twelve nonregression checks and four aggregate gain checks. **Not adopted; no caller, query or SDK integration follows.**

The experiment gathers coordinate words directly from the unchanged 128-byte point rows, then performs the accepted mixed-R256 radix split. It adds no encoded key, retained point buffer, lookup table, plan or pool. Unlike the [blocked codec](../2026-11-05-g2-blocked-codec176/README.md), it has no blocked format, one/two-block classification or encoding pass. Unlike [paired coordinate packing](../2026-10-31-paired-fq2-pack176/README.md) and [G1 reference packing](../2026-10-28-direct-ref-pack/README.md), it uses hardware word gathers rather than vector loads and transposes. Those earlier failures stay closed.

This measures coordinate loading with a common field norm-sum consumer, **not complete G2 additions, queries, proof latency or concurrent-proof throughput**. Every worker replays the complete saved distribution against shared immutable input; workers duplicate the cycle rather than reproducing the prover's partitioned query schedule.

## Measurements

Ryzen 9950X, 16 physical/32 logical CPUs, shared host. Fifteen fixed rotated/reversed blocks compare the original accepted field library, fresh original constructor and direct gather. Actual consumer links use O3, native CPU, fat LTO and one codegen unit. Each worker performs three checked full-cycle primes then eight measured cycles, with rotating start positions. Workers pin to CPU 2, CPUs 0–15 or CPUs 0–31 and reserve 512 KiB stacks. Dispatch, indices, loading, radix conversion and the common consumer are timed. Parsing, validation, thread creation and output checks are outside timing; validation would need charging in any complete integration.

Values below are medians of process cycle-wave times. Ratios pair observations within blocks and need not equal ratios of displayed medians. Every ON/OFF individual comparison loses all 15 blocks.

| Root / workers | Original ms | Fresh OFF ms | Gather ms | ON/OFF ratio [bounds] | Wins/15 |
|---|---:|---:|---:|---|---:|
|1x2 / 1|3.731363|3.730623|5.409122|1.452469 [1.439901, 1.462653]|0|
|1x2 / 16|4.313142|4.314885|6.204496|1.436668 [1.431881, 1.440145]|0|
|1x2 / 32|6.794723|6.729489|10.339525|1.534653 [1.525717, 1.538591]|0|
|2x2 / 1|3.906165|3.891603|5.491870|1.408337 [1.395696, 1.412037]|0|
|2x2 / 16|4.513280|4.505469|6.265835|1.391280 [1.386156, 1.392778]|0|
|2x2 / 32|6.916554|6.914651|10.597826|1.540301 [1.524191, 1.548596]|0|

The predeclared gain requires upper<1 and at least 10/15 wins against both controls for the all-six and two-size 32-worker aggregates. Every ON/control upper must be at most 1.03; every OFF/original interval must fit factor 1.03. Bounds are sorted ranks 4–12/15, nominal 96.484375% marginal coverage conditional on exchangeable blocks, without familywise coverage. No gain floor, exclusions, extensions, changed thresholds, favorable subset or unchanged retry.

| Aggregate / control | Median ratio | Lower / upper | Wins/15 |
|---|---:|---:|---:|
|all6 / frozen|1.455553|1.453117 / 1.457121|0|
|all6 / off|1.457262|1.455784 / 1.459731|0|
|logical32_2 / frozen|1.531381|1.526536 / 1.540971|0|
|logical32_2 / off|1.540279|1.525646 / 1.542897|0|

## Generated code and profiles

The linked common cycle has 815/815/775 instruction sites in original/OFF/ON, with 300 IFMA sites in every arm and 0/0/16 hardware-gather sites. Selected bytes are 4,932/4,932/4,845. These are static counts; fewer sites do not establish cheaper execution.

The separate counter panels show about 2.63% fewer retired instructions but more cycles. All-worker counts below are per complete cycle wave; they are single diagnostic observations, not paired latency bounds or a causal decomposition.

| Workers | OFF / ON instructions, million | OFF / ON cycles, million | Cycle change |
|---|---:|---:|---:|
|16|706.340 / 687.753|321.156 / 466.340|+45.21%|
|32|1412.682 / 1375.506|910.218 / 1487.559|+63.43%|

All eighteen preflights pass before 270 latency processes. Four warm PMU panels and four separate fixed 16M-user-cycle profiles follow all latency. No owned build, hash, archive, analyzer, verifier or second benchmark overlaps capture. Every counter group runs 100%; all four profiles qualify, with 50,774 samples, zero LOST records, all registered workers observed at assigned CPUs and unknown fractions below 0.079%. Generic cache events are not DRAM measurements. Flat samples do not identify exclusive critical-path costs or local CPI.

## Qualification and memory

Two focused native tests pass once, comparing 317,184 component lanes with the original constructor and independent scalar bit extraction. Coverage includes canonical Montgomery boundaries, word/radix boundaries, repeated/reversed/nonadjacent indices, short allocations, 8,192 deterministic index groups, invalid roots/indices and input preservation. The original 35 field tests are filtered and inherited. Existing field arithmetic/generated source, core, prover and all 176 production files remain unchanged. The private 177-file snapshot has 175 byte-exact originals, one append-only module export and one child; inverse edits recover every original.

The API borrows one immutable slice of sixteen-u64 rows, validates coordinate words below p and checks all eight indices before gathering. Every gather address remains within that allocation. No absolute-pointer/null-base gather or transmuted external SIMD representation is used. This does not establish that every operand in a complete borrowed prover call belongs to one slice; that integration was deliberately withheld after the negative screen.

Inputs reuse the exact two roots and all 64,541 primary / 66,884 grouped 2×2 index groups from the earlier qualified codec study. The roots contain 30,089 / 28,852 points; the second is the actual witness-dependent grouped root, not an original-key prefix. All 397 prior input pins and their existing complete-Arkworks/scalar-oracle bindings are checked. No witness, complete MSM oracle or root preparation is rerun. Every new process matches the expected scalar digest; the reviewer recomputes the small final digest from the existing scalar words and repeat count. This is not a new proof or complete-MSM execution.

Timed allocations are zero. The one shared root allocation retains its pointer/capacity and 3,851,392 / 3,693,056 payload bytes. The replay-index vector is benchmark input, not a proposed retained runtime cache. Its logical 4,130,624 / 4,280,576 bytes and capacities 4,194,304 / 8,388,608 bytes remain included in driver records. No process RSS, full-proof peak, minimum-RAM, allocator/stack bound, swap or pressure qualification is supplied.

## Review and decision

Independent review passes 782 unique pins / 1,909 pin occurrences, 323 successful workflow commands, 296 caller processes, all 22 paired distributions, all 16 failed checks, raw expected digests, source recovery and decoded profile ownership/loss. Workload totals 3,973,383,579 measured and 964,510,416 priming full groups, including diagnostics. These are repeated codec operations, not independent inputs or proofs.

The campaign helper's first compilation omitted the demangler dependency and stopped before execution; its repair adds only the missing extern. The first offline reviewer assumed separate PID/TID columns and stopped at profile counting after its other checks passed. Its parser was corrected for the actual PID/TID column. Initial source/logs and repair notes are retained. No successful native test, build, preparation, latency capture or profile is repeated. No arithmetic, observation or threshold changes after capture.

Close this loader without a format/class/core/width/layout/attribute rescue. Fewer static and retired instructions do not make the hardware gathers cheaper here. No source-level loading improvement, Aeglos comparison or twofold proof gain follows. The accepted implementation stays selected.

## Accepted CPU and Metal comparisons, unchanged

Inherited local 32-worker full-proof observations and supplied historical M5 references follow. None is remeasured by this codec experiment.

|Shape|Zopro ms|M5 CPU ms / speedup|M5 Metal ms / speedup|CPU10× limit ms|
|---|---:|---:|---:|---:|
|1×2|7.928660|37 / 4.667×|24 / 3.027×|3.7|
|2×2|11.151932|59 / 5.291×|30 / 2.690×|5.9|
|4×2|16.086386|109 / 6.776×|48 / 2.984×|10.9|
|8×2|24.179453|189 / 7.817×|74 / 3.060×|18.9|
|12×2|33.884455|284 / 8.381×|101 / 2.981×|28.4|
|16×2|40.077381|335 / 8.359×|124 / 3.094×|33.5|
|24×2|65.850777|528 / 8.018×|176 / 2.673×|52.8|
|merge24|65.060937|507 / 7.793×|175 / 2.690×|50.7|

Merge54 remains 161.883899 ms without an M5 reference. Primary RSS 132.75 MiB is inherited. Accepted source176/SDK6d83 and the 30,846-proof ledger remain unchanged. The all-eight CPU10× objective remains active/unmet, **0/8**. No new AWS/Aeglos/Gnark, phone, huge-circuit, pressure, swap, minimum-RAM or concurrent-throughput result is added.

[Full analysis](artifacts/analysis.json.gz), [independent review](artifacts/review.json), [loader](artifacts/aos_gather.rs), [driver](artifacts/driver.rs), [design](artifacts/design.md), [code counts](artifacts/codegen.json), [source recovery](artifacts/source-audit.json), [exact sources/protocols/command receipts/decoded profiles](artifacts/source-and-receipts.json.gz), [publication](artifacts/publication.json), [checksums](SHA256SUMS). Four lossless raw profile archives accompany the receipts. Executables, libraries, raw point/index roots, keys, witnesses and proof inputs stay private.
